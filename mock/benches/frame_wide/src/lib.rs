//! The harness side of `frame_wide`'s negative control: which routine a cell
//! resolves to, which workload program wraps a call, and the worker dispatch
//! the control needs when validation re-executes it.
//!
//! The timed runs go through the driver `mock bench run` generates, whose hooks
//! library takes the routine from `frame-wide-support` as this crate does. The
//! negative control is a test binary of its own, re-executed by validation as
//! its worker, so it answers `--worker` through `worker_main` here.
//!
//! The workload itself, the arms and the routine type are in
//! `frame-wide-support`; nothing here knows what a frame computes.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

pub use frame_wide_support::is_heavy;
use mockspace_bench_harness::{self as harness, BenchError, HarnessTuning, RoutineSpec, Workload};

/// The routine one cell resolves to, from `frame_wide_support::bridge_for`.
pub fn routine_for(n: usize, bench: &str) -> Option<RoutineSpec> {
    let bridge = frame_wide_support::bridge_for(n, is_heavy(bench)?)?;
    Some(RoutineSpec {
        name: bench.to_string(),
        bridge,
    })
}

/// One program of one item, the call itself. A frame over tens of thousands
/// of records or more is its own cache context; the harness's mix-in items
/// are sized for calls a thousand times smaller and would only add noise.
/// The orchestrator and every worker build it from here, so they agree.
pub fn workload_for(bench: &str) -> Workload {
    let mut w = Workload::new();
    w.program(bench, |b| {
        b.stage(vec![harness::algo_call()]);
    });
    w
}

/// Whether this process was started as a harness worker.
pub fn is_worker(args: &[String]) -> bool {
    args.iter().any(|a| a == "--worker")
}

/// Answer a worker invocation: time one variant, or validate it.
///
/// `--mode validate` goes to `run_worker_validate`. Without that arm a
/// validation worker prints no `VOUT` lines, every variant is skipped for
/// returning none of its outputs, and validation refuses the run for having
/// compared nothing. Every other mode goes to `run_worker`.
pub fn worker_main(args: &[String]) -> ExitCode {
    let get = |flag: &str| -> Option<String> {
        let pos = args.iter().position(|a| a == flag)?;
        args.get(pos + 1).cloned()
    };
    let Some(dylib_path) = get("--worker") else {
        eprintln!("worker: missing --worker <path>");
        return ExitCode::FAILURE;
    };
    let bench_name = get("--bench-name").unwrap_or_default();
    let mode = get("--mode").unwrap_or_else(|| "warm".into());
    let n: usize = get("--n").and_then(|s| s.parse().ok()).unwrap_or(0);
    // Passed for a section declaring `threaded = true`: the worker then leaves
    // its own thread unpinned, since a variant's spawned threads never inherit
    // the pin and pinning the coordinator alone skews a threaded workload.
    let threaded = args.iter().any(|a| a == "--threaded");

    let Some(routine) = routine_for(n, &bench_name) else {
        eprintln!("worker: bench `{bench_name}` has no routine at n={n}");
        return ExitCode::FAILURE;
    };

    if mode == "validate" {
        let seeds: Vec<u64> = match get("--seeds") {
            Some(s) => {
                match s.split(',').map(str::parse).collect() {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("worker: --seeds `{s}` is not a list of u64: {e}");
                        return ExitCode::FAILURE;
                    },
                }
            },
            None => {
                eprintln!("worker: --mode validate needs --seeds");
                return ExitCode::FAILURE;
            },
        };
        harness::run_worker_validate(&routine, &dylib_path, &seeds, n, threaded);
        return ExitCode::SUCCESS;
    }

    let seed: u64 = get("--seed").and_then(|s| s.parse().ok()).unwrap_or(0);
    let cooldown_ms: u64 = get("--cooldown").and_then(|s| s.parse().ok()).unwrap_or(0);
    let runs: usize = get("--runs").and_then(|s| s.parse().ok()).unwrap_or(0);
    let batch: usize = get("--batch").and_then(|s| s.parse().ok()).unwrap_or(1);
    let batch_k: usize = get("--batch-k").and_then(|s| s.parse().ok()).unwrap_or(1);
    let max_call_us: Option<u64> = get("--max-call-us")
        .and_then(|s| s.parse().ok())
        .filter(|&v| v > 0);
    harness::run_worker(
        &routine,
        &workload_for(&bench_name),
        &dylib_path,
        seed,
        cooldown_ms,
        &mode,
        runs,
        batch,
        n,
        batch_k,
        max_call_us,
        threaded,
    );
    ExitCode::SUCCESS
}

/// Where an arm's cdylib actually is, given the path the manifest resolved.
///
/// The harness resolves a short name to
/// `variants/<name>/target/release/lib<name>.so`, a per-variant target
/// directory. This member is one workspace building into whatever target
/// directory cargo is configured with, so every arm lands beside the driver
/// binary (and one level above a test binary, which cargo puts in `deps/`).
/// The file name the harness resolved is kept, prefix and extension included,
/// and only the directory is replaced.
// FIXME(placeholders_tagged_c9d2fc212037): drop once the harness resolves arms in the target
// directory cargo built them into.
pub fn locate_arm(resolved: &Path) -> Result<PathBuf, String> {
    let file = resolved
        .file_name()
        .ok_or_else(|| format!("arm path `{}` has no file name", resolved.display()))?;
    let exe = std::env::current_exe().map_err(|e| format!("cannot read current_exe: {e}"))?;
    let dir = exe.parent().map(Path::to_path_buf).unwrap_or_default();
    let candidates = [dir.join(file), dir.parent().map(|p| p.join(file)).unwrap_or_default()];
    candidates
        .iter()
        .find(|p| p.is_file())
        .cloned()
        .ok_or_else(|| {
            format!(
                "arm `{}` is not built beside `{}`. Build the member first, from \
             `mock/benches/frame_wide`: `CARGO_INCREMENTAL=0 cargo build --release --workspace`",
                file.to_string_lossy(),
                exe.display()
            )
        })
}

/// Validate every arm of one cell against the reference and against each
/// other, and refuse unless every arm survived.
///
/// `validate` drops an arm whose probe or worker crashed and goes on with the
/// rest, which is right for a sweep that may lose a slow arm and wrong here:
/// an arm missing from the comparison has not been shown to compute the
/// frame, and its timings would be reported beside the ones that have.
pub fn validate_every_arm(
    routine: &RoutineSpec,
    arms: &[String],
    n: usize,
    bench: &str,
    max_call_us: Option<u64>,
    tuning: &HarnessTuning,
) -> Result<(), BenchError> {
    let survivors = harness::validate(routine, arms, n, bench, max_call_us, Some(tuning))?;
    if survivors.len() != arms.len() {
        let dropped: Vec<&String> = arms.iter().filter(|a| !survivors.contains(a)).collect();
        return Err(BenchError::ValidationFailed {
            variant: bench.to_string(),
            reason:  format!(
                "{} of {} arms were dropped before comparison and none may be: {dropped:?}",
                dropped.len(),
                arms.len()
            ),
        });
    }
    Ok(())
}

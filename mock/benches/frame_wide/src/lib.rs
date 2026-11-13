//! The harness side of `frame_wide`: which routine a cell resolves to, which
//! workload program wraps a call, and the worker dispatch every binary that
//! the harness re-executes needs.
//!
//! Two binaries are re-executed as workers: the driver, by the orchestrator and
//! by validation, and the negative-control test, by validation. Both have to
//! answer `--worker` the same way, so the answer is here once and each `main`
//! is a call into `worker_main`.
//!
//! The workload itself, the arms and the routine type are in
//! `frame-wide-support`; nothing here knows what a frame computes.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use frame_wide_support::{FrameHeavy, FrameLight};
use mockspace_bench_core::routine_bridge;
use mockspace_bench_harness::{
    self as harness,
    BenchError,
    BenchResult,
    HarnessTuning,
    RoutineSpec,
    Workload,
};

/// The regime a bench name selects. A section's name is the sweep, and in a
/// composed tree the key is `<member>/<sweep>`, so the last segment decides.
pub fn is_heavy(bench: &str) -> Option<bool> {
    match bench.rsplit('/').next().unwrap_or(bench) {
        "heavy" => Some(true),
        "light" => Some(false),
        _ => None,
    }
}

/// The routine one cell resolves to: one monomorphisation per record count
/// and regime.
///
/// The record counts here are the counts a bench section may name. Each
/// variant's `#[bench_variant]` attribute lists the same literals, and a size
/// missing from one of them is refused at preflight by the harness's
/// output-size check rather than run.
pub fn routine_for(n: usize, bench: &str) -> Option<RoutineSpec> {
    macro_rules! pick {
        ($heavy:expr, $($n:literal),*) => {
            match n {
                $( $n => if $heavy {
                    routine_bridge!(FrameHeavy<$n>)
                } else {
                    routine_bridge!(FrameLight<$n>)
                }, )*
                _ => return None,
            }
        };
    }
    let heavy = is_heavy(bench)?;
    let bridge = pick!(heavy, 4096, 65536, 1048576, 4194304, 16777216);
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

/// The arm every delta in a generated report is taken against.
///
/// Not an engine arm. The harness frames each row as a delta from the
/// baseline and writes stock lines calling a large delta a reason to switch;
/// with `engine` as the baseline those lines read as a verdict on the engine,
/// which is the one reading op's words rule out. `hand` is the hand-written
/// fused path, which the bench matrix names as the bar the engine is measured
/// against, so a row against it says how far an arm is from what a person
/// writes for this workload.
const REPORT_BASELINE: &str = "hand";

/// What a reader opening a generated findings file alone has to see first.
///
/// The research note disclaims these numbers, and a findings file does not
/// travel with the note. Without this the file is a verdict sheet.
const FINDINGS_CAVEAT: &str = "\
## Before the numbers

Nothing in this file is a verdict on any arm, and no delta below is a reason to
use one arm over another.

The engine arms measure the engine without its headline features. Op, on 13
November: the engine arms measure something that \"doesn't yet include any of
the actual headline features like fusing and the adaptive trunk/fiber/morsel
crunching on the datadlow-ish schedule. It's bare bones, barely lowers to
anything interesting yet. So no calls should be made based on those yet, they
are a baseline to compare against then.\" An engine number here is a baseline
for a later comparison and decides nothing.

Every delta below is taken against `hand`, the hand-written fused path, rather
than against an engine arm. The bench matrix names `hand` as the bar the engine
is measured against, so a row against it says how far an arm is from what a
person writes for this workload. Where the report says the baseline is the
slowest variant and reads as picked by accident, it was not: it is set in this
member's driver, on purpose, and is meant to be the bar whether it wins or
loses. Do not re-baseline on whichever arm happens to be fastest.

The harness writes a `Why it matters` line under each highlight, calling a large
delta a reason to switch and a dominant winner a safe default pick. Those lines
are its stock phrasing for any dataset, they are judgements this file has no
standing to make, and they are stripped before it is written. What is left under
each highlight is the shape of the numbers.

A median here is wall-clock time and carries nothing about how many cores an
arm used to reach it. Two arms with the same median are not the same cost if
one of them ran on one core and the other on two. The occupancy of every
parallel arm is measured separately and stated beside these medians in
`mock/research/202611131400_frame-wide-first-run.md`. Read the two together or
neither.

The generated report follows.

";

/// Write a findings file: the caveat, then the harness's report with `hand` as
/// the baseline.
///
/// `harness::write_report_for_routine` is the one-call form and it takes
/// neither a baseline nor a preamble, so the dataset is built, pointed at
/// `REPORT_BASELINE` and rendered here instead. The baseline is checked rather
/// than assumed: `with_baseline` ignores a name it does not find, and silently
/// falling back to the first variant is how the report comes out framed on
/// `engine` again.
pub fn write_findings(
    result: &BenchResult,
    routine: &RoutineSpec,
    mode: &str,
    path: &str,
) -> Result<(), String> {
    let ds = result
        .dataset_for_routine(routine, mode)
        .with_baseline(REPORT_BASELINE);
    let chosen = ds.baseline().name.clone();
    if chosen != REPORT_BASELINE {
        return Err(format!(
            "the report baseline is `{chosen}`, not `{REPORT_BASELINE}`: no arm of this cell is \
             named `{REPORT_BASELINE}`, so every delta would be framed on an arm nobody chose"
        ));
    }
    let md = format!(
        "{FINDINGS_CAVEAT}{}",
        plain(&harness::generate_report(&ds, &result.title))
    );
    std::fs::write(path, md).map_err(|e| format!("writing {path}: {e}"))
}

/// Take the harness's editorialising and its emphasis out of a report.
///
/// Two things go. The `_Why it matters:_` lines are the harness telling a
/// reader which way to decide, which is exactly what this member's numbers do
/// not support, and disclaiming them in the preamble while leaving them in the
/// body reads worse than either. Bold spans go because the workspace writes no
/// inline styling in prose a human reads, and a generated body committed to the
/// repo is prose a human reads.
///
/// Everything else is left alone, including the figures and the highlight
/// headings the bold was wrapped around.
fn plain(report: &str) -> String {
    let mut out = String::with_capacity(report.len());
    for line in report.lines() {
        if line.trim_start().starts_with("_Why it matters:_") {
            continue;
        }
        out.push_str(&line.replace("**", ""));
        out.push('\n');
    }
    out
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
// FIXME(c9d2fc212037): drop once the harness resolves arms in the target
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

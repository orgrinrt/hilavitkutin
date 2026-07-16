//! Orchestrator for the six-variant resource-storage-model comparison
//! (round 202606210600), run through the mockspace bench harness.
//!
//! Six variant cdylibs (V0 blob-live, V1 blob-snapshot, V2 decomposed, V3
//! shape-bound, V4 erased, V5 handle-table) each build their storage layout from
//! the seeded byte input and run the morsel loop in the timed region. The harness
//! isolates each variant in a subprocess, times the run block with the hardware
//! counter, validates the 8-byte checksum output byte-exact across all six, and
//! writes per-bench CSV + findings.md with bootstrap CI, sign test, and multi-N
//! scaling.
//!
//! The payload size (bytes, = column-record data) is the size sweep. The mix-in
//! intensity is the workload program selected per bench: `clean` (algo only),
//! `light_thrash` (a little surrounding work), `heavy_thrash` (L1-evicting
//! heavy-memory passes between calls, the regime that turns V0's L1-hot member
//! reload into a cache miss so the snapshot/live difference can surface). Three
//! bench sections in bench.toml sweep the same six variants under the three
//! intensities.
//!
//! Run: `cargo run --release` from this dir (after building the six variant
//! cdylibs). Or via `mock bench run`.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use mockspace_bench_core::{routine_bridge, ByteRoutine};
use mockspace_bench_harness::{self as harness, BenchManifest, RoutineSpec, Workload};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--worker") {
        return run_worker(&args);
    }
    let report_only = args.iter().any(|a| a == "--report-only");

    let manifest = match BenchManifest::load(Path::new("bench.toml")) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };
    let dir = std::env::current_dir()
        .expect("readable cwd for variant path resolution")
        .canonicalize()
        .expect("canonicalize cwd");

    for (bench_name, section) in &manifest.bench {
        for (size_idx, _size) in section.sizes.iter().enumerate() {
            let mut config = match manifest.for_size(bench_name, size_idx, &dir) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("error: {e}");
                    return ExitCode::FAILURE;
                }
            };
            config.variant_paths =
                config.variant_paths.into_iter().map(shape_variant_path).collect();
            let routine = match routine_for(config.n, &section.workload) {
                Some(r) => r,
                None => {
                    eprintln!("error: bench `{bench_name}` unsupported n={}", config.n);
                    return ExitCode::FAILURE;
                }
            };
            // Outputs land in the benches root (../), alongside every other
            // bench's csv / json / findings, not in this orchestrator subdir.
            let csv = format!("../{}_n{}.csv", bench_name, config.n);
            let report = format!("../{}_n{}_findings.md", bench_name, config.n);

            if report_only {
                let samples = match harness::load_samples_csv(Path::new(&csv)) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("error: report-only `{csv}`: {e}");
                        return ExitCode::FAILURE;
                    }
                };
                let result = mockspace_bench_harness::BenchResult {
                    title: section.title.clone(),
                    env: mockspace_bench_harness::EnvMeta::default(),
                    samples,
                    cache_path: csv.clone(),
                    report_path: report.clone(),
                };
                if let Err(e) = harness::write_report_for_routine(&result, &routine, "warm", &report)
                {
                    eprintln!("error: report: {e}");
                    return ExitCode::FAILURE;
                }
                eprintln!("  regenerated {report}");
            } else {
                let result = match harness::run(&config, &routine, &workload_for(bench_name)) {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("error: bench `{bench_name}` n={}: {e}", config.n);
                        return ExitCode::FAILURE;
                    }
                };
                if let Err(e) = harness::write_csv(&result, &csv) {
                    eprintln!("error: csv: {e}");
                    return ExitCode::FAILURE;
                }
                if let Err(e) = harness::write_report_for_routine(&result, &routine, "warm", &report)
                {
                    eprintln!("error: report: {e}");
                    return ExitCode::FAILURE;
                }
                eprintln!("  wrote {csv} + {report}");
            }
        }
    }
    ExitCode::SUCCESS
}

/// Build a SINGLE-program workload for one bench section. The harness picks the
/// program by `seed % programs.len()`, so registering more than one program per
/// run would randomly rotate the mix-in intensity across samples and confound
/// the comparison. One program per section keeps every sample at the labelled
/// intensity. The orchestrator and the worker subprocess both call THIS one
/// helper so they build a byte-identical program for a given section (a mismatch
/// would silently measure under a different cache context than the label says).
///
/// The mix-in intensity is the bench-section axis: `clean` (algo only),
/// `light_thrash` (a little surrounding work), `heavy_thrash` (L1-evicting
/// heavy-memory passes between calls, the regime that turns V0's L1-hot member
/// reload into a real cache miss).
fn workload_for(section: &str) -> Workload {
    let mut w = Workload::new();
    if section.contains("heavy") {
        w.program(section, |b| {
            b.stage(vec![
                harness::algo_call(),
                harness::scalar_work(48),
                harness::graph_work(32),
                harness::heavy_memory(2048), // evict L1 between morsel-loop calls
                harness::branch_work(24),
                harness::light_scalar(),
            ]);
        });
    } else if section.contains("light") {
        w.program(section, |b| {
            b.stage(vec![
                harness::algo_call(),
                harness::scalar_work(32),
                harness::branch_work(16),
                harness::light_scalar(),
            ]);
        });
    } else {
        w.program(section, |b| {
            b.stage(vec![harness::algo_call()]);
        });
    }
    w
}

/// Routine bridge per (bench, payload size). The seqd bench uses the seed-driven
/// `SeqAlgo<N>` Routine (tiny u64 input, N-element heap payload built in setup,
/// so the payload size has no stack-array ceiling); every other bench uses the
/// flat `ByteRoutine<N>` (N bytes of column-record data). MAY_DIFFER=false so the
/// harness validates the 8-byte checksum byte-exact across variants.
fn routine_for(n: usize, bench: &str) -> Option<RoutineSpec> {
    use rsb_kernels::seqd::SeqAlgo;
    let bridge = if bench.contains("seqd") {
        match n {
            65536 => routine_bridge!(SeqAlgo<65536>),
            1048576 => routine_bridge!(SeqAlgo<1048576>),
            4194304 => routine_bridge!(SeqAlgo<4194304>),
            16777216 => routine_bridge!(SeqAlgo<16777216>),
            67108864 => routine_bridge!(SeqAlgo<67108864>),
            268435456 => routine_bridge!(SeqAlgo<268435456>),
            _ => return None,
        }
    } else {
        match n {
            256 => routine_bridge!(ByteRoutine<256, 8, false>),
            1024 => routine_bridge!(ByteRoutine<1024, 8, false>),
            4096 => routine_bridge!(ByteRoutine<4096, 8, false>),
            16384 => routine_bridge!(ByteRoutine<16384, 8, false>),
            65536 => routine_bridge!(ByteRoutine<65536, 8, false>),
            262144 => routine_bridge!(ByteRoutine<262144, 8, false>),
            1048576 => routine_bridge!(ByteRoutine<1048576, 8, false>),
            4194304 => routine_bridge!(ByteRoutine<4194304, 8, false>),
            _ => return None,
        }
    };
    Some(RoutineSpec { name: bench.to_string(), bridge })
}

fn shape_variant_path(p: PathBuf) -> PathBuf {
    let parent = p.parent().map(Path::to_path_buf).unwrap_or_default();
    let stem = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
    parent.join(format!(
        "{}{}{}",
        std::env::consts::DLL_PREFIX,
        stem,
        std::env::consts::DLL_SUFFIX
    ))
}

fn run_worker(args: &[String]) -> ExitCode {
    let get = |flag: &str| -> Option<String> {
        let pos = args.iter().position(|a| a == flag)?;
        args.get(pos + 1).cloned()
    };
    let dylib_path = match get("--worker") {
        Some(p) => p,
        None => {
            eprintln!("worker: missing --worker <path>");
            return ExitCode::FAILURE;
        }
    };
    let bench_name = get("--bench-name").unwrap_or_default();
    let seed: u64 = get("--seed").and_then(|s| s.parse().ok()).unwrap_or(0);
    let cooldown_ms: u64 = get("--cooldown").and_then(|s| s.parse().ok()).unwrap_or(0);
    let mode = get("--mode").unwrap_or_else(|| "warm".into());
    let runs: usize = get("--runs").and_then(|s| s.parse().ok()).unwrap_or(0);
    let batch: usize = get("--batch").and_then(|s| s.parse().ok()).unwrap_or(1);
    let n: usize = get("--n").and_then(|s| s.parse().ok()).unwrap_or(1);
    let batch_k: usize = get("--batch-k").and_then(|s| s.parse().ok()).unwrap_or(1);
    let max_call_us: Option<u64> =
        get("--max-call-us").and_then(|s| s.parse().ok()).filter(|&v| v > 0);

    // The worker uses the bench-name (= the workload program selected for the
    // section) to pick the same routine the orchestrator built.
    let routine = match routine_for(n, &bench_name) {
        Some(r) => r,
        None => {
            eprintln!("worker: unsupported n={n}");
            return ExitCode::FAILURE;
        }
    };
    let workload = workload_for(&bench_name);
    harness::run_worker(
        &routine, &workload, &dylib_path, seed, cooldown_ms, &mode, runs, batch, n, batch_k,
        max_call_us,
    );
    ExitCode::SUCCESS
}

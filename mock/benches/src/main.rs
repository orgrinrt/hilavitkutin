//! Bench binary for the hilavitkutin runtime megaround (202605101036).
//!
//! Uses `ByteRoutine<N, 8, MAY_DIFFER>` from `mockspace-bench-core` to bridge
//! byte input / 8-byte hash output across cdylib variants. Each variant
//! implements the same algorithm at multiple sizes via the `#[bench_variant]`
//! macro's N dispatch.
//!
//! The workload program surrounds the AlgoCall with realistic context:
//! scalar dep chains, pointer-chase graph work, heavy-memory passes
//! (L1 eviction), branchy code (predictor pressure). That replicates
//! the real-runtime calling context where the algorithm under test is
//! one item in a fiber's morsel loop alongside other column reads.
//!
//! Three benches:
//!
//! - `ema_axis_h`: NEON vs scalar vs autovec EMA. All variants compute
//!   identical math; MAY_DIFFER=false enforces byte-exact validation.
//!
//! - `dispatch_static`: direct vs sealed-trait vs const fn pointer.
//!   All variants compute the same FNV1a; MAY_DIFFER=false.
//!
//! - `dispatch_dynamic`: direct vs opaque fn pointer vs data-dependent
//!   table. The table variant picks one of four different mixers per
//!   chunk; output diverges from direct. MAY_DIFFER=true.

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

    let manifest_path = Path::new("bench.toml");
    let manifest = match BenchManifest::load(manifest_path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };

    let mock_benches_dir = std::env::current_dir()
        .expect("bench requires readable current_dir for variant path resolution")
        .canonicalize()
        .expect("could not canonicalize cwd for variant path resolution");

    let workload = build_workload();

    for (bench_name, section) in &manifest.bench {
        for (size_idx, _size) in section.sizes.iter().enumerate() {
            let mut config = match manifest.for_size(bench_name, size_idx, &mock_benches_dir) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("error: {e}");
                    return ExitCode::FAILURE;
                }
            };
            config.variant_paths = config
                .variant_paths
                .into_iter()
                .map(shape_variant_path)
                .collect();
            let routine = match routine_for_bench(bench_name, &section.workload, config.n) {
                Some(r) => r,
                None => {
                    eprintln!(
                        "error: bench `{bench_name}` declares unsupported size n={}",
                        config.n
                    );
                    return ExitCode::FAILURE;
                }
            };

            let csv_path = format!("{}_n{}.csv", bench_name, config.n);
            let report_path = format!("{}_n{}_findings.md", bench_name, config.n);

            if report_only {
                let samples = match harness::load_samples_csv(Path::new(&csv_path)) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("error: report-only could not load `{csv_path}`: {e}");
                        return ExitCode::FAILURE;
                    }
                };
                if samples.is_empty() {
                    eprintln!("error: report-only: no samples in `{csv_path}`");
                    return ExitCode::FAILURE;
                }
                let result = mockspace_bench_harness::BenchResult {
                    title: section.title.clone(),
                    env: mockspace_bench_harness::EnvMeta::default(),
                    samples,
                    cache_path: csv_path.clone(),
                    report_path: report_path.clone(),
                };
                if let Err(e) =
                    harness::write_report_for_routine(&result, &routine, "warm", &report_path)
                {
                    eprintln!("error: writing report: {e}");
                    return ExitCode::FAILURE;
                }
                eprintln!("  regenerated {report_path}");
            } else {
                let result = match harness::run(&config, &routine, &workload) {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("error: bench `{bench_name}` n={}: {e}", config.n);
                        return ExitCode::FAILURE;
                    }
                };
                if let Err(e) = harness::write_csv(&result, &csv_path) {
                    eprintln!("error: writing csv: {e}");
                    return ExitCode::FAILURE;
                }
                if let Err(e) =
                    harness::write_report_for_routine(&result, &routine, "warm", &report_path)
                {
                    eprintln!("error: writing report: {e}");
                    return ExitCode::FAILURE;
                }
                eprintln!("  wrote {csv_path} + {report_path}");
            }
        }
    }

    ExitCode::SUCCESS
}

/// Build the workload program. Surrounds the AlgoCall with realistic
/// context: scalar dep chain, graph pointer-chase (L1 latency), heavy
/// memory pass (L1 eviction), branchy code (predictor pressure),
/// light scalar (cheap filler). This mimics a fiber's morsel loop
/// where the algorithm under test is one of several operations
/// contending for register pressure, cache lines, and branch state.
fn build_workload() -> Workload {
    let mut workload = Workload::new();
    workload.program("realistic", |b| {
        b.stage(vec![
            harness::algo_call(),
            harness::scalar_work(48),
            harness::graph_work(32),
            harness::heavy_memory(384),
            harness::branch_work(24),
            harness::light_scalar(),
        ]);
    });
    workload
}

/// Map (bench, workload, N) -> ByteRoutine bridge. MAY_DIFFER routes by bench
/// name: `dispatch_dynamic` allows divergent output because its table variant
/// picks one of four different mixers per chunk; every other bench enforces
/// byte-exact cross-variant validation.
fn routine_for_bench(bench_name: &str, workload: &str, n: usize) -> Option<RoutineSpec> {
    let may_differ = matches!(
        bench_name,
        "dispatch_dynamic" | "hash_algos" | "fxpmul_strategy" | "cache_layout"
    );
    let bridge = if may_differ {
        match n {
            8 => routine_bridge!(ByteRoutine<8, 8, true>),
            16 => routine_bridge!(ByteRoutine<16, 8, true>),
            32 => routine_bridge!(ByteRoutine<32, 8, true>),
            64 => routine_bridge!(ByteRoutine<64, 8, true>),
            128 => routine_bridge!(ByteRoutine<128, 8, true>),
            256 => routine_bridge!(ByteRoutine<256, 8, true>),
            1024 => routine_bridge!(ByteRoutine<1024, 8, true>),
            4096 => routine_bridge!(ByteRoutine<4096, 8, true>),
            16384 => routine_bridge!(ByteRoutine<16384, 8, true>),
            _ => return None,
        }
    } else {
        match n {
            8 => routine_bridge!(ByteRoutine<8, 8, false>),
            16 => routine_bridge!(ByteRoutine<16, 8, false>),
            32 => routine_bridge!(ByteRoutine<32, 8, false>),
            64 => routine_bridge!(ByteRoutine<64, 8, false>),
            128 => routine_bridge!(ByteRoutine<128, 8, false>),
            256 => routine_bridge!(ByteRoutine<256, 8, false>),
            1024 => routine_bridge!(ByteRoutine<1024, 8, false>),
            4096 => routine_bridge!(ByteRoutine<4096, 8, false>),
            16384 => routine_bridge!(ByteRoutine<16384, 8, false>),
            _ => return None,
        }
    };
    Some(RoutineSpec {
        name: workload.to_string(),
        bridge,
    })
}

/// Turn a variant bare-stem path into the platform dylib path.
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
    let max_call_us: Option<u64> = get("--max-call-us")
        .and_then(|s| s.parse().ok())
        .filter(|&v| v > 0);

    let routine = match routine_for_bench(&bench_name, &bench_name, n) {
        Some(r) => r,
        None => {
            eprintln!("worker: unsupported n={n} for bench `{bench_name}`");
            return ExitCode::FAILURE;
        }
    };

    let workload = build_workload();

    harness::run_worker(
        &routine,
        &workload,
        &dylib_path,
        seed,
        cooldown_ms,
        &mode,
        runs,
        batch,
        n,
        batch_k,
        max_call_us,
    );
    ExitCode::SUCCESS
}

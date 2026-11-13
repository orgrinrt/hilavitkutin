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

use benches::disasm_5check;
use mockspace_bench_core::{ByteDispatch, byte_routine_dispatch};
use mockspace_bench_harness::{self as harness, BenchConfig, BenchManifest, RoutineSpec, Workload};

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
        },
    };

    let mock_benches_dir = std::env::current_dir()
        .expect("bench requires readable current_dir for variant path resolution")
        .canonicalize()
        .expect("could not canonicalize cwd for variant path resolution");

    let workload = build_workload();

    // 5-check axis-A invariant: any FAIL on a dispatch_static_n*
    // bench drives the bench binary's exit code so `cargo mock bench`
    // surfaces the regression. dispatch_dynamic_n* FAILs are
    // expected (counter-example) and recorded without poisoning the
    // exit code.
    let mut axis_a_static_fail = false;

    for (bench_name, section) in &manifest.bench {
        for (size_idx, _size) in section.sizes.iter().enumerate() {
            let mut config = match manifest.for_size(bench_name, size_idx, &mock_benches_dir) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("error: {e}");
                    return ExitCode::FAILURE;
                },
            };
            config.variant_paths = variant_paths_for(&config);
            let routine = match routine_for_bench(bench_name, &section.workload, config.n) {
                Some(r) => r,
                None => {
                    eprintln!(
                        "error: bench `{bench_name}` declares unsupported size n={}; this \
                         driver dispatches {:?}",
                        config.n,
                        byte_dispatch().sizes
                    );
                    return ExitCode::FAILURE;
                },
            };

            let csv_path = format!("{}_n{}.csv", bench_name, config.n);
            let report_path = format!("{}_n{}_findings.md", bench_name, config.n);

            if report_only {
                let samples = match harness::load_samples_csv(Path::new(&csv_path)) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("error: report-only could not load `{csv_path}`: {e}");
                        return ExitCode::FAILURE;
                    },
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
                    },
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

            // 5-check axis-A pass: only applies to dispatch_* benches
            // (the Topic 4 axis A shapes). Other benches measure
            // orthogonal axes (EMA, hash, modulo, etc.) where the 5
            // checks are not the relevant invariant.
            if bench_name.starts_with("dispatch_") {
                let morsel_immediates = [config.n as u64];
                let report_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
                match disasm_5check::run_and_write(
                    bench_name,
                    config.n,
                    &config.variant_paths,
                    &morsel_immediates,
                    &report_dir,
                ) {
                    Ok(br) => {
                        let path =
                            report_dir.join(format!("{}_n{}_5check.md", bench_name, config.n));
                        if br.any_fail() && bench_name.starts_with("dispatch_static") {
                            eprintln!(
                                "  5-check FAIL (axis-A regression on dispatch_static): {}",
                                path.display()
                            );
                            axis_a_static_fail = true;
                        } else if br.any_fail() {
                            eprintln!(
                                "  5-check report (counter-example FAILs expected): {}",
                                path.display()
                            );
                        } else {
                            eprintln!("  5-check PASS: {}", path.display());
                        }
                    },
                    Err(e) => {
                        eprintln!("  5-check skipped: {e}");
                    },
                }
            }
        }
    }

    if axis_a_static_fail {
        eprintln!(
            "error: at least one dispatch_static 5-check failed; the axis-A LLVM-transparency invariant regressed"
        );
        return ExitCode::FAILURE;
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

/// The variant paths this driver hands the harness for one cell.
///
/// `BenchManifest::for_size` has already resolved every bare stem to a
/// full platform dylib path, so there is nothing left for the driver to
/// add, and adding nothing is the whole content of this function. It is a
/// function rather than an absence because the driver used to shape these
/// paths a second time at this point, turning every entry into
/// `liblibema_neon.so.so`, and a test cannot catch that coming back
/// unless there is a seam to call. `variant-path` holds the check and the
/// old shaping it has to refuse.
fn variant_paths_for(config: &BenchConfig) -> Vec<PathBuf> {
    config.variant_paths.clone()
}

/// The record counts this driver can monomorphise a `ByteRoutine` for.
///
/// Declared once through the harness macro rather than as a match arm per
/// size in each `MAY_DIFFER` branch, which is what this driver carried
/// before the macro existed. The list is also what the error names when a
/// manifest asks for a size outside it, so the gap is reported against this
/// declaration instead of passing as a bare `None`.
fn byte_dispatch() -> ByteDispatch {
    byte_routine_dispatch!(
        out = 8,
        sizes = [8, 16, 32, 64, 128, 256, 1024, 4096, 16384]
    )
}

/// Map (bench, workload, N) -> ByteRoutine bridge. MAY_DIFFER routes by bench
/// name: `dispatch_dynamic` allows divergent output because its table variant
/// picks one of four different mixers per chunk; every other bench enforces
/// byte-exact cross-variant validation.
fn routine_for_bench(bench_name: &str, workload: &str, n: usize) -> Option<RoutineSpec> {
    let may_differ = matches!(
        bench_name,
        "dispatch_dynamic"
            | "hash_algos"
            | "fxpmul_strategy"
            | "cache_layout"
            | "iter_patterns"
            | "ema_formulation"
            | "modulo_strategy"
            | "branch_predictability"
            | "fold_strategy"
            | "rotate_strategy"
            | "shift_amount_origin"
            | "match_arm_count"
            | "float_width"
    );
    let bridge = (byte_dispatch().dispatch)(n, may_differ)?;
    Some(RoutineSpec {
        name: workload.to_string(),
        bridge,
    })
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
        },
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
    // The orchestrator passes `--threaded` for a bench section that declares
    // `threaded = true`, and the worker must not pin its own thread then: a
    // threaded variant's spawned threads never inherit the pin, so pinning the
    // coordinator alone skews the workload.
    let threaded = args.iter().any(|a| a == "--threaded");

    let routine = match routine_for_bench(&bench_name, &bench_name, n) {
        Some(r) => r,
        None => {
            eprintln!(
                "worker: unsupported n={n} for bench `{bench_name}`; this driver dispatches {:?}",
                byte_dispatch().sizes
            );
            return ExitCode::FAILURE;
        },
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
        threaded,
    );
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use variant_path::{shaped_as_the_drivers_once_did, shaped_once};

    use super::*;

    /// Resolve every cell the repo's own manifest declares, and hand back
    /// what the driver would give the harness for each.
    fn driver_variant_paths() -> Vec<(String, usize, Vec<PathBuf>)> {
        let dir = std::env::current_dir().expect("readable cwd");
        let manifest = BenchManifest::load(&dir.join("bench.toml")).expect("the repo's bench.toml");

        let mut out = Vec::new();
        for (bench_name, section) in &manifest.bench {
            for (size_idx, _) in section.sizes.iter().enumerate() {
                let config = manifest
                    .for_size(bench_name, size_idx, &dir)
                    .unwrap_or_else(|e| panic!("bench `{bench_name}` size {size_idx}: {e}"));
                out.push((bench_name.clone(), size_idx, variant_paths_for(&config)));
            }
        }
        out
    }

    /// Every path the driver hands the harness carries the platform
    /// dylib prefix and suffix exactly once.
    ///
    /// This goes through `variant_paths_for`, which is the point at which
    /// the driver used to shape an already-resolved path a second time.
    /// Put that shaping back inside it and this test goes red, which is
    /// the only reason the function exists.
    #[test]
    fn the_paths_the_driver_hands_over_are_shaped_once() {
        let cells = driver_variant_paths();
        let mut checked = 0usize;
        for (bench_name, size_idx, paths) in &cells {
            for path in paths {
                let name = path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or_else(|| {
                        panic!("bench `{bench_name}`: variant path has no file name")
                    });
                if let Err(e) = shaped_once(name) {
                    panic!("bench `{bench_name}` size {size_idx}: {e}");
                }
                checked += 1;
            }
        }
        assert!(
            checked > 0,
            "the manifest resolved no variant paths, so this test asserted nothing"
        );
    }

    /// Put the old shaping back over the same paths and the check above
    /// refuses every one of them.
    ///
    /// Without this, a check that accepted everything would leave the
    /// test above green and say nothing. The shaping here is the
    /// driver's own, as it stood until df8a9419, applied to the real
    /// resolved paths rather than to a name the test invented.
    #[test]
    fn the_shaping_the_driver_once_did_is_refused_on_every_cell() {
        let cells = driver_variant_paths();
        let mut refused = 0usize;
        for (bench_name, size_idx, paths) in &cells {
            for path in paths {
                let doubled = shaped_as_the_drivers_once_did(path);
                let name = doubled
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or_else(|| {
                        panic!("bench `{bench_name}`: doubled path has no file name")
                    });
                match shaped_once(name) {
                    Ok(()) => {
                        panic!(
                            "bench `{bench_name}` size {size_idx}: the check accepted `{name}`, so \
                         the test above proves nothing"
                        )
                    },
                    Err(e) => {
                        assert!(
                            e.contains("twice"),
                            "bench `{bench_name}` size {size_idx}: the refusal should say an affix \
                         doubled, got `{e}`"
                        )
                    },
                }
                refused += 1;
            }
        }
        assert!(
            refused > 0,
            "no cell was checked, so this control asserted nothing"
        );
    }
}

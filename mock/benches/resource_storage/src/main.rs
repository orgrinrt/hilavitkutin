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

use mockspace_bench_core::{ByteDispatch, byte_routine_dispatch, routine_bridge};
use mockspace_bench_harness::{self as harness, BenchConfig, BenchManifest, RoutineSpec, Workload};

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
        },
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
                },
            };
            config.variant_paths = variant_paths_for(&config);
            let routine = match routine_for(config.n, &section.workload) {
                Some(r) => r,
                None => {
                    eprintln!("error: bench `{bench_name}` unsupported n={}", config.n);
                    return ExitCode::FAILURE;
                },
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
                    },
                };
                let result = mockspace_bench_harness::BenchResult {
                    title: section.title.clone(),
                    env: mockspace_bench_harness::EnvMeta::default(),
                    samples,
                    cache_path: csv.clone(),
                    report_path: report.clone(),
                };
                if let Err(e) =
                    harness::write_report_for_routine(&result, &routine, "warm", &report)
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
                    },
                };
                if let Err(e) = harness::write_csv(&result, &csv) {
                    eprintln!("error: csv: {e}");
                    return ExitCode::FAILURE;
                }
                if let Err(e) =
                    harness::write_report_for_routine(&result, &routine, "warm", &report)
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

/// The variant paths this driver hands the harness for one cell.
///
/// `BenchManifest::for_size` has already resolved every bare stem to a
/// full platform dylib path, so there is nothing left for the driver to
/// add, and adding nothing is the whole content of this function. It is a
/// function rather than an absence because this driver used to shape
/// these paths a second time at this point, turning every entry into
/// `liblibrsb_soa.so.so`, and a test cannot catch that coming back unless
/// there is a seam to call.
fn variant_paths_for(config: &BenchConfig) -> Vec<PathBuf> {
    config.variant_paths.clone()
}

/// The record counts the flat `ByteRoutine` path can be monomorphised for.
///
/// Declared once through the harness macro rather than as a match arm per
/// size, which is what this driver carried before the macro existed. The
/// seqd path keeps its own match because it monomorphises `SeqAlgo<N>`, a
/// different routine with a different size list, and the macro only builds
/// `ByteRoutine` dispatches.
fn byte_dispatch() -> ByteDispatch {
    byte_routine_dispatch!(
        out = 8,
        sizes = [256, 1024, 4096, 16384, 65536, 262144, 1048576, 4194304]
    )
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
        (byte_dispatch().dispatch)(n, false)?
    };
    Some(RoutineSpec {
        name: bench.to_string(),
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
    // Passed by the orchestrator for a section declaring `threaded = true`, and
    // the worker then leaves its own thread unpinned: a variant's spawned
    // threads never inherit the pin, so pinning the coordinator alone skews it.
    let threaded = args.iter().any(|a| a == "--threaded");

    // The worker uses the bench-name (= the workload program selected for the
    // section) to pick the same routine the orchestrator built.
    let routine = match routine_for(n, &bench_name) {
        Some(r) => r,
        None => {
            eprintln!("worker: unsupported n={n}");
            return ExitCode::FAILURE;
        },
    };
    let workload = workload_for(&bench_name);
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

    /// Resolve every cell this manifest declares, and hand back what the
    /// driver would give the harness for each.
    ///
    /// The driver loads `bench.toml` relative to the working directory,
    /// so this does the same and the test runs from the member root.
    fn driver_variant_paths() -> Vec<(String, usize, Vec<PathBuf>)> {
        let dir = std::env::current_dir().expect("readable cwd");
        let manifest = BenchManifest::load(Path::new("bench.toml")).expect("the repo's bench.toml");

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
    /// Put that shaping back inside it and this test goes red.
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
    /// test above green and say nothing.
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

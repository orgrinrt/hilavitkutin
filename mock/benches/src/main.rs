//! Consumer bench binary: registrations plus the axis-A disasm gate.
//!
//! The generic loop (manifest iteration, filtering, report-only,
//! preflight, seed replay, validation, history, summary, findings
//! index) lives in `mockspace_bench_harness::driver::drive`. This
//! binary contributes the workload program, the declared byte-size
//! dispatch, and the repo-specific 5-check disassembly gate that runs
//! after a successful pass over any `dispatch_*` bench (the Topic 4
//! axis-A LLVM-transparency invariant: a `dispatch_static` 5-check
//! failure fails the run; `dispatch_dynamic` failures are the
//! expected counter-example and only get recorded).

use std::path::PathBuf;
use std::process::ExitCode;

use benches::disasm_5check;
use mockspace_bench_core::byte_routine_dispatch;
use mockspace_bench_harness::driver::{DriverRegistry, drive};
use mockspace_bench_harness::{self as harness, BenchConfig, BenchManifest, RoutineSpec, Workload};

/// The realistic workload program: the measured call embedded in
/// scalar dependency chains, pointer-chase graph work, cache
/// pressure, and branchy context, replicating the real-runtime
/// calling environment.
fn build_workload(_name: &str, _n: usize) -> Workload {
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

/// Every bench here is byte-shaped; the dispatch below serves all of
/// them (`may_differ` comes from the manifest, not a name list).
fn routine_for(_config: &BenchConfig) -> Option<RoutineSpec> {
    None
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let code = drive(&DriverRegistry {
        build_workload,
        routine_for,
        // Every size is its own monomorphisation: this list is the
        // strictly controlled input set. A manifest size outside it
        // is a targeted error naming this line.
        byte_dispatch: byte_routine_dispatch!(
            out = 8,
            sizes = [8, 16, 32, 64, 128, 256, 1024, 2048, 4096, 8192, 16384, 65536, 262144]
        ),
    });

    // Workers and report-only invocations skip the disasm gate; a
    // failed drive already carries its own exit code.
    let passive = args.iter().any(|a| a == "--worker" || a == "--report-only");
    if passive || code != ExitCode::SUCCESS {
        return code;
    }
    axis_a_gate(&args)
}

/// Post-run 5-check disassembly gate over the `dispatch_*` benches in
/// this run's selection.
fn axis_a_gate(args: &[String]) -> ExitCode {
    let manifest = match BenchManifest::load(std::path::Path::new("bench.toml")) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error: 5-check gate could not load bench.toml: {e}");
            return ExitCode::FAILURE;
        },
    };
    // Mirror the driver's selection grammar: positional names plus
    // --only values; an empty selection means every bench.
    let mut only: Vec<String> = Vec::new();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--only" => {
                if let Some(v) = args.get(i + 1) {
                    only.push(v.clone());
                    i += 1;
                }
            },
            "--seed" => i += 1,
            a if !a.starts_with("--") => only.push(a.to_string()),
            _ => {},
        }
        i += 1;
    }
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut axis_a_static_fail = false;

    for bench_name in manifest.bench_names() {
        if !bench_name.starts_with("dispatch_") {
            continue;
        }
        if !only.is_empty() && !only.contains(&bench_name) {
            continue;
        }
        let section = &manifest.bench[&bench_name];
        for size_idx in 0 .. section.sizes.len() {
            let Ok(config) = manifest.for_size(&bench_name, size_idx, &cwd) else {
                continue;
            };
            let morsel_immediates = [config.n as u64];
            match disasm_5check::run_and_write(
                &bench_name,
                config.n,
                &config.variant_paths,
                &morsel_immediates,
                &cwd,
            ) {
                Ok(br) => {
                    let path = cwd.join(format!("{}_n{}_5check.md", bench_name, config.n));
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
                Err(e) => eprintln!("  5-check skipped: {e}"),
            }
        }
    }

    if axis_a_static_fail {
        eprintln!(
            "error: at least one dispatch_static 5-check failed; the axis-A \
             LLVM-transparency invariant regressed"
        );
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use variant_path::{shaped_as_the_drivers_once_did, shaped_once};

    use super::*;

    /// Resolve every cell the repo's own manifest declares, and hand back
    /// the variant paths `drive` resolves for each.
    fn resolved_variant_paths() -> Vec<(String, usize, Vec<PathBuf>)> {
        let dir = std::env::current_dir().expect("readable cwd");
        let manifest = BenchManifest::load(&dir.join("bench.toml")).expect("the repo's bench.toml");

        let mut out = Vec::new();
        for (bench_name, section) in &manifest.bench {
            for (size_idx, _) in section.sizes.iter().enumerate() {
                let config = manifest
                    .for_size(bench_name, size_idx, &dir)
                    .unwrap_or_else(|e| panic!("bench `{bench_name}` size {size_idx}: {e}"));
                out.push((bench_name.clone(), size_idx, config.variant_paths.clone()));
            }
        }
        out
    }

    /// Every variant path the manifest resolves carries the platform
    /// dylib prefix and suffix exactly once.
    ///
    /// The driver itself no longer touches these paths: `drive` resolves
    /// them and loads them. It once shaped an already-resolved path a second
    /// time, turning every arm into `liblibema_neon.so.so`, so the property
    /// is held here over what `for_size` resolves for every cell the manifest
    /// declares. A second shaping inside `drive`, after that point, is the
    /// harness's to catch and would not show here.
    #[test]
    fn the_resolved_variant_paths_are_shaped_once() {
        let cells = resolved_variant_paths();
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
        let cells = resolved_variant_paths();
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

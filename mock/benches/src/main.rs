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

use mockspace_bench_core::byte_routine_dispatch;
use mockspace_bench_harness::driver::{drive, DriverRegistry};
use mockspace_bench_harness::{self as harness, BenchConfig, BenchManifest, RoutineSpec, Workload};

use benches::disasm_5check;

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
            sizes = [8, 16, 32, 64, 128, 256, 1024, 2048, 4096, 8192, 16384]
        ),
    });

    // Workers and report-only invocations skip the disasm gate; a
    // failed drive already carries its own exit code.
    let passive = args
        .iter()
        .any(|a| a == "--worker" || a == "--report-only");
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
        }
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
            }
            "--seed" => i += 1,
            a if !a.starts_with("--") => only.push(a.to_string()),
            _ => {}
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
        for size_idx in 0..section.sizes.len() {
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
                }
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

//! The `frame_wide` driver: validate every arm of a cell, then time it.
//!
//! The harness has two entry points and only `driver::drive_spec` validates.
//! This member keeps its own loop, like `resource_storage`, and so calls
//! `validation::validate` itself before `harness::run`, which the harness's
//! own crate docs name as the way to get the checks on that path. A cell
//! whose arms do not all agree with the reference and with each other is not
//! timed, and the run fails.
//!
//! Flags: `--n <N>` runs only that record count, `--only <sweep>` only that
//! sweep (repeatable), `--report-only` regenerates the findings from the CSVs
//! already written. The harness re-executes this binary with `--worker`, which
//! `frame_wide::worker_main` answers.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use frame_wide::{
    is_worker,
    locate_arm,
    routine_for,
    validate_every_arm,
    worker_main,
    workload_for,
    write_findings,
};
use mockspace_bench_harness::{self as harness, BenchManifest, BenchResult, EnvMeta};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if is_worker(&args) {
        return worker_main(&args);
    }
    match drive(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        },
    }
}

/// The values every occurrence of `flag` takes.
fn flag_values(args: &[String], flag: &str) -> Vec<String> {
    args.windows(2)
        .filter(|w| w[0] == flag)
        .map(|w| w[1].clone())
        .collect()
}

fn drive(args: &[String]) -> Result<(), String> {
    let report_only = args.iter().any(|a| a == "--report-only");
    let only = flag_values(args, "--only");
    let only_n: Option<usize> = match flag_values(args, "--n").last() {
        Some(s) => Some(s.parse().map_err(|e| format!("--n `{s}`: {e}"))?),
        None => None,
    };

    let manifest = BenchManifest::load(Path::new("bench.toml")).map_err(|e| e.to_string())?;
    let member_dir = std::env::current_dir()
        .and_then(|d| d.canonicalize())
        .map_err(|e| format!("cannot resolve the member directory: {e}"))?;

    let mut cells = 0usize;
    for bench in manifest.bench_names() {
        if !only.is_empty() && !only.contains(&bench) {
            continue;
        }
        let section = &manifest.bench[&bench];
        for (idx, size) in section.sizes.iter().enumerate() {
            if only_n.is_some_and(|n| n != size.n) {
                continue;
            }
            cells += 1;
            run_cell(&manifest, &bench, idx, &member_dir, report_only)?;
        }
    }
    if cells == 0 {
        return Err("no cell matched `--only` and `--n`".to_string());
    }
    Ok(())
}

fn run_cell(
    manifest: &BenchManifest,
    bench: &str,
    idx: usize,
    member_dir: &Path,
    report_only: bool,
) -> Result<(), String> {
    let mut config = manifest
        .for_size(bench, idx, member_dir)
        .map_err(|e| e.to_string())?;
    let n = config.n;
    let routine =
        routine_for(n, bench).ok_or_else(|| format!("bench `{bench}` has no routine at n={n}"))?;

    // The manifest and the routine each say whether arms must agree. They
    // have to say the same thing, or the section reads as checked while the
    // routine compares nothing.
    if config.may_differ != routine.bridge.outputs_may_differ || config.may_differ {
        return Err(format!(
            "bench `{bench}`: every arm must compute the same frame, so `may_differ` must be \
             false in the manifest and in the routine"
        ));
    }
    if !config.required {
        return Err(format!(
            "bench `{bench}`: `required` must be true, so a validation failure fails the run"
        ));
    }

    let csv = format!("{bench}_n{n}.csv");
    let findings = format!("{bench}_n{n}_findings.md");

    if report_only {
        let samples = harness::load_samples_csv(Path::new(&csv)).map_err(|e| e.to_string())?;
        let result = BenchResult {
            title: config.title.clone(),
            env: EnvMeta::default(),
            samples,
            cache_path: csv.clone(),
            report_path: findings.clone(),
        };
        write_findings(&result, &routine, "warm", &findings)?;
        eprintln!("  regenerated {findings}");
        return Ok(());
    }

    let arms: Vec<PathBuf> = config
        .variant_paths
        .iter()
        .map(|p| locate_arm(p))
        .collect::<Result<_, _>>()?;
    config.variant_paths = arms;
    // `for_size` leaves this unset; without it a worker is killed at 300
    // seconds and the preflight cannot tell a slow arm from a hung one.
    config.max_call_us = (routine.bridge.max_call_us)(n);

    let arm_strings: Vec<String> = config
        .variant_paths
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect();
    eprintln!("[{bench} n={n}] validating {} arms", arm_strings.len());
    validate_every_arm(
        &routine,
        &arm_strings,
        n,
        bench,
        config.max_call_us,
        &config.tuning,
    )
    .map_err(|e| format!("bench `{bench}` n={n}: {e}"))?;

    eprintln!("[{bench} n={n}] timing");
    let result = harness::run(&config, &routine, &workload_for(bench))
        .map_err(|e| format!("bench `{bench}` n={n}: {e}"))?;
    harness::write_csv(&result, &csv).map_err(|e| e.to_string())?;
    write_findings(&result, &routine, "warm", &findings)?;
    eprintln!("  wrote {csv} and {findings}");
    Ok(())
}

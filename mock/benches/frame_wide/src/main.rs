//! The `frame_wide` driver: validate every arm of a cell, then time it.
//!
//! The harness has two entry points and only `driver::drive_spec` validates.
//! This member keeps its own loop, like `resource_storage`, and so calls
//! `validation::validate` itself before `harness::run`, which the harness's
//! own crate docs name as the way to get the checks on that path. A cell
//! whose arms do not all agree with the reference and with each other is not
//! timed, and the run fails.
//!
//! Before any cell is timed the driver runs `./run-checks.sh`, which is the
//! build, the grouping test, the checks gate's own test, the support crate's
//! unit tests and the two negative controls. Nothing invoked that script until
//! now, so its checks passed only when somebody remembered to run them.
//!
//! A timing run starts from a stage (`mock/benches/stage.sh .`, then
//! `<stage>/release/frame_wide`), so the arms it loads are the ones staged
//! beside it and not whatever another worktree last built into the shared
//! target directory. The stage is verified before the first cell and after the
//! last, and its build record is written beside every CSV as
//! `<bench>_n<N>.build.txt`.
//!
//! Flags: `--n <N>` runs only that record count, `--only <sweep>` only that
//! sweep (repeatable), `--report-only` regenerates the findings from the CSVs
//! already written, `--skip-checks` runs without the checks and says so. The
//! harness re-executes this binary with `--worker`, which
//! `frame_wide::worker_main` answers.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use bench_stage::Stage;
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

/// Run `run-checks.sh` before anything is timed, or say that it was skipped.
///
/// The script's own header calls itself everything that has to pass before a
/// timing from this member counts, and until now nothing invoked it. This
/// member declares its own `[workspace]`, as every bench member here does, so
/// `cargo mock test` never reaches it and its tests passed only because
/// somebody ran them by hand. Timing a cell is minutes of work; the checks are
/// a couple of minutes of it, nearly all counter calibration, so paying them
/// is cheaper than a number nobody can vouch for.
///
/// `--skip-checks` is for iterating on the driver itself. It prints, because a
/// run that skipped them should not look like a run that passed them.
fn run_checks(skip: bool) -> Result<(), String> {
    if skip {
        eprintln!("  checks skipped (--skip-checks): these timings are not vouched for");
        return Ok(());
    }
    eprintln!("  running ./run-checks.sh");
    let status = std::process::Command::new("./run-checks.sh")
        .status()
        .map_err(|e| format!("could not run ./run-checks.sh: {e}"))?;
    if !status.success() {
        return Err("./run-checks.sh failed, so nothing was timed".to_string());
    }
    Ok(())
}

fn drive(args: &[String]) -> Result<(), String> {
    let report_only = args.iter().any(|a| a == "--report-only");
    let only = flag_values(args, "--only");
    let only_n: Option<usize> = match flag_values(args, "--n").last() {
        Some(s) => Some(s.parse().map_err(|e| format!("--n `{s}`: {e}"))?),
        None => None,
    };

    let stage = if report_only {
        None
    } else {
        let stage = Stage::of_current_exe()?;
        stage.verify()?;
        eprintln!("  staged at {}", stage.dir().display());
        run_checks(args.iter().any(|a| a == "--skip-checks"))?;
        Some(stage)
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
            run_cell(&manifest, &bench, idx, &member_dir, stage.as_ref())?;
        }
    }
    if cells == 0 {
        return Err("no cell matched `--only` and `--n`".to_string());
    }
    if let Some(stage) = &stage {
        stage
            .verify()
            .map_err(|e| format!("{e}\nthe timings above are not of the staged build"))?;
    }
    Ok(())
}

fn run_cell(
    manifest: &BenchManifest,
    bench: &str,
    idx: usize,
    member_dir: &Path,
    stage: Option<&Stage>,
) -> Result<(), String> {
    let report_only = stage.is_none();
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
        // A configured size with no CSV has not been run (FIXME(5f526d071953)
        // covers the three large ones). Regenerating is a pass over whatever
        // results exist, so an absent cell is skipped rather than ending the
        // pass and leaving the later cells stale.
        if !Path::new(&csv).exists() {
            eprintln!("  skipped {csv}, not run");
            return Ok(());
        }
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
    let build = format!("{bench}_n{n}.build.txt");
    if let Some(stage) = stage {
        std::fs::write(&build, stage.build_record()?)
            .map_err(|e| format!("cannot write {build}: {e}"))?;
    }
    eprintln!("  wrote {csv}, {findings} and {build}");
    Ok(())
}

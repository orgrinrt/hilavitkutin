//! The negative control: validation has to refuse an arm that computes the
//! wrong frame, and only that arm.
//!
//! Two wrong arms are run, one per place the frame's topology is written down.
//! `drop_stage` is the hand path with the second stage of the first pipeline
//! missing, which is the `frame_record` switch stating it. `engine_drop_stage`
//! is the engine's builder call sequence with the same `Map` omitted, which is
//! the other statement of the same twelve-unit shape and the one a reader can
//! least check by eye. A control over the first says nothing about the second,
//! so each gets its own run against a good arm of its own kind.
//!
//! Each run goes through the same `validate_every_arm` the driver uses, so
//! through the preflight probes, the per-arm check against the reference and
//! the byte-exact comparison across arms. The refusal must name the wrong arm,
//! and the mismatch count must be exactly two per seed: one against the
//! reference and one against the good arm it ran beside. A check refusing
//! everything would also count the good arms, and a check that did nothing
//! would count zero, so the one number carries both the negative and the
//! positive half.
//!
//! It builds the member's arms first, with the profile and into the target
//! directory it was built with itself, so the arms it loads are the ones of
//! this tree and are found beside the build. The harness re-executes this
//! binary as its validation worker, which is why `main` answers `--worker`.

use std::path::Path;
use std::process::{Command, ExitCode};

use frame_wide::{is_worker, locate_arm, routine_for, validate_every_arm, worker_main};
use mockspace_bench_harness::{BenchError, HarnessTuning};

/// Small enough that the whole control runs in a couple of minutes, most of
/// it the harness's per-worker counter calibration.
const N: usize = 4096;
const SEEDS: usize = 3;

/// Build every arm of the member with the profile this binary was built with,
/// into the target directory it was built into, where `locate_arm` finds them.
fn build_arms() -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| format!("cannot read current_exe: {e}"))?;
    // A test binary sits in `<target>/<profile>/deps/`.
    let profile = exe
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| format!("`{}` sits in no profile directory", exe.display()))?;
    let target = profile
        .parent()
        .ok_or_else(|| format!("`{}` sits in no target directory", profile.display()))?;
    let mut build = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    build
        .args(["build", "--workspace", "--lib", "--manifest-path"])
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .arg("--target-dir")
        .arg(target);
    if profile.file_name().is_some_and(|n| n == "release") {
        build.arg("--release");
    }
    let status = build
        .status()
        .map_err(|e| format!("could not run cargo: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("building the arms failed ({status})"))
    }
}

fn arm(stem: &str) -> String {
    let file = format!(
        "{}{stem}{}",
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_SUFFIX
    );
    match locate_arm(Path::new(&file)) {
        Ok(p) => p.to_string_lossy().into_owned(),
        Err(e) => panic!("{e}"),
    }
}

/// Run `wrong` beside `good`, and require the refusal to name `wrong` with
/// exactly two mismatches per seed.
///
/// `good` is the arm of the same kind the wrong one is a copy of, so a refusal
/// that named it instead would say the check cannot tell the two apart.
fn refuses(bench: &str, good: &str, wrong: &str, wrong_name: &str) -> Result<(), String> {
    let routine = routine_for(N, bench).ok_or_else(|| format!("no routine for `{bench}`"))?;
    let tuning = HarnessTuning {
        validation_seeds: SEEDS,
        determinism_check_seeds: SEEDS,
        ..HarnessTuning::default()
    };
    let limit = (routine.bridge.max_call_us)(N);
    let arms = [arm(good), arm(wrong)];

    match validate_every_arm(&routine, &arms, N, bench, limit, &tuning) {
        Ok(()) => Err(format!("{bench}: `{wrong_name}` passed validation")),
        Err(BenchError::ValidationFailed {
            variant,
            reason,
        }) => {
            if variant != wrong_name {
                return Err(format!("{bench}: the refusal named `{variant}`: {reason}"));
            }
            let expected = format!("{} mismatches across {SEEDS} seeds", 2 * SEEDS);
            if !reason.starts_with(&expected) {
                return Err(format!("{bench}: expected `{expected}`, got `{reason}`"));
            }
            eprintln!("  {bench}: {wrong_name} refused, {good} accepted: {reason}");
            Ok(())
        },
        Err(other) => Err(format!("{bench}: refused for the wrong reason: {other}")),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if is_worker(&args) {
        return worker_main(&args);
    }
    if let Err(e) = build_arms() {
        eprintln!("FAIL {e}");
        return ExitCode::FAILURE;
    }
    let mut failed = false;
    for bench in ["frame_wide/heavy", "frame_wide/light"] {
        // The fused statement of the frame, and the engine's own.
        let runs = [
            ("fw_hand", "fw_drop_stage", "drop_stage"),
            ("fw_engine", "fw_engine_drop_stage", "engine_drop_stage"),
        ];
        for (good, wrong, name) in runs {
            if let Err(e) = refuses(bench, good, wrong, name) {
                eprintln!("FAIL {e}");
                failed = true;
            }
        }
    }
    if failed {
        ExitCode::FAILURE
    } else {
        eprintln!("negative control: ok");
        ExitCode::SUCCESS
    }
}

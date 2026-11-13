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
//! Run with `cargo test --release` after `cargo build --release --workspace`:
//! the arms are found beside the build, and the harness re-executes this
//! binary as its validation worker, which is why `main` answers `--worker`.
//! `./run-checks.sh` from the member directory does both and the unit tests.

use std::path::Path;
use std::process::ExitCode;

use frame_wide::{is_worker, locate_arm, routine_for, validate_every_arm, worker_main};
use mockspace_bench_harness::{BenchError, HarnessTuning};

/// Small enough that the whole control runs in a couple of minutes, most of
/// it the harness's per-worker counter calibration.
const N: usize = 4096;
const SEEDS: usize = 3;

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

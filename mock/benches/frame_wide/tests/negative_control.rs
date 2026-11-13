//! The negative control: validation has to refuse an arm that computes the
//! wrong frame, and only that arm.
//!
//! `drop_stage` is the hand path with the second stage of the first pipeline
//! missing. It runs beside `hand` and `engine` through the same
//! `validate_every_arm` the driver uses, so through the preflight probes, the
//! per-arm check against the reference and the byte-exact comparison across
//! arms. The refusal must name `drop_stage`, and the mismatch count must be
//! exactly two per seed: one for `drop_stage` against the reference and one
//! for `drop_stage` against `hand`. A check refusing everything would also
//! count `engine`, and a check that did nothing would count zero, so the one
//! number carries both the negative and the positive half.
//!
//! Run with `cargo test --release` after `cargo build --release --workspace`:
//! the arms are found beside the build, and the harness re-executes this
//! binary as its validation worker, which is why `main` answers `--worker`.

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

fn check(bench: &str) -> Result<(), String> {
    let routine = routine_for(N, bench).ok_or_else(|| format!("no routine for `{bench}`"))?;
    let tuning = HarnessTuning {
        validation_seeds: SEEDS,
        determinism_check_seeds: SEEDS,
        ..HarnessTuning::default()
    };
    let limit = (routine.bridge.max_call_us)(N);
    let arms = [arm("fw_hand"), arm("fw_engine"), arm("fw_drop_stage")];

    match validate_every_arm(&routine, &arms, N, bench, limit, &tuning) {
        Ok(()) => Err(format!("{bench}: `drop_stage` passed validation")),
        Err(BenchError::ValidationFailed {
            variant,
            reason,
        }) => {
            if variant != "drop_stage" {
                return Err(format!("{bench}: the refusal named `{variant}`: {reason}"));
            }
            let expected = format!("{} mismatches across {SEEDS} seeds", 2 * SEEDS);
            if !reason.starts_with(&expected) {
                return Err(format!("{bench}: expected `{expected}`, got `{reason}`"));
            }
            eprintln!("  {bench}: drop_stage refused, hand and engine accepted: {reason}");
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
        if let Err(e) = check(bench) {
            eprintln!("FAIL {e}");
            failed = true;
        }
    }
    if failed {
        ExitCode::FAILURE
    } else {
        eprintln!("negative control: ok");
        ExitCode::SUCCESS
    }
}

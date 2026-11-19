//! The routine's `validate_output` refuses what it exists to refuse. A run's
//! validation counts only because each case below is shown failing it.
//!
//! The outputs come from the frame the timed arm runs, then one field is
//! changed, so every refusal is one step away from an output that passes.

use accum_group_support::arms::{GroupedArm, PerCoreArm, run_once};
use accum_group_support::{Acc, AccInput, AccOutput};
use mockspace_bench_core::Routine;

/// Not a multiple of any worker count or group width the bench declares.
const N: usize = 1001;

const SEED: u64 = 0x5EED_0000_0000_0001;

fn input(workers: u32, group: u32) -> AccInput {
    AccInput {
        seed: SEED,
        workers,
        group,
    }
}

fn validate(input: &AccInput, output: &AccOutput) -> Result<(), &'static str> {
    <Acc<N> as Routine>::validate_output(input, output)
}

#[test]
fn an_output_as_the_frame_gave_it_passes() {
    for w in [1, 2, 3, 4] {
        let at = input(w, 8);
        assert_eq!(validate(&at, &run_once::<GroupedArm>(&at, N)), Ok(()));
        assert_eq!(validate(&at, &run_once::<PerCoreArm>(&at, N)), Ok(()));
    }
}

#[test]
fn a_flipped_bit_in_the_digest_is_refused() {
    let at = input(3, 8);
    let mut out = run_once::<GroupedArm>(&at, N);
    out.digest ^= 1;
    assert!(validate(&at, &out).is_err());
}

#[test]
fn a_changed_combined_value_is_refused() {
    let at = input(3, 8);
    let mut out = run_once::<GroupedArm>(&at, N);
    out.combined = out.combined.wrapping_add(1);
    assert!(validate(&at, &out).is_err());
}

#[test]
fn a_dropped_partial_is_refused() {
    let at = input(3, 8);
    let mut out = run_once::<GroupedArm>(&at, N);
    out.parts -= 1;
    assert!(validate(&at, &out).is_err());
}

#[test]
fn an_output_at_another_walker_count_is_refused() {
    let at = input(3, 8);
    let mut out = run_once::<GroupedArm>(&at, N);
    out.walkers = 2;
    assert!(validate(&at, &out).is_err());
}

#[test]
fn an_unknown_scheme_id_is_refused() {
    let at = input(3, 8);
    let mut out = run_once::<GroupedArm>(&at, N);
    out.scheme = 99;
    assert!(validate(&at, &out).is_err());
}

/// The partials of one partial per core at two workers, relabelled as run
/// at three: the walker count matches, the bytes are the wrong count's.
#[test]
fn per_core_partials_of_two_workers_are_refused_at_three() {
    let mut out = run_once::<PerCoreArm>(&input(2, 8), N);
    out.walkers = 3;
    assert!(validate(&input(3, 8), &out).is_err());
}

/// The grouped partials at two workers, relabelled as run at three, pass:
/// that the grouped model names no worker count is what B1a claims.
#[test]
fn grouped_partials_of_two_workers_pass_at_three() {
    let mut out = run_once::<GroupedArm>(&input(2, 8), N);
    out.walkers = 3;
    assert_eq!(validate(&input(3, 8), &out), Ok(()));
}

/// The walker count is read from the crew: one walker over every record
/// reports one walker at a section of four, and passes there.
#[test]
fn the_walker_count_is_the_crews_not_the_sections() {
    use accum_group_support::arms::SequentialArm;
    let at = input(4, 8);
    let out = run_once::<SequentialArm>(&at, N);
    assert_eq!(out.walkers, 1);
    assert_eq!(validate(&at, &out), Ok(()));
}

//! The case that must fail: a 1-bit packed column split at a boundary that is a
//! multiple of 4 records but not of 64 loses writes, because two cores write
//! one word. The aligned split is the control and must hold.
//!
//! Both splits are what the trunk can produce: 68 is a legal window under
//! `& !3` with MIN_MORSEL set to 4 or a computed window of 68, and
//! ceil(136 / 2) = 68 is the trunk's per-core region for 136 records on two
//! cores. Nondeterministic by nature; the counts are in test-output.txt.

use c2_packed_word_race::frames_with_lost_writes;

const TRIALS: usize = 200_000;

#[test]
fn a_four_aligned_split_of_a_packed_column_loses_no_writes() {
    let lost = frames_with_lost_writes(136, 68, TRIALS);
    println!("split at 68 of 136, {TRIALS} frames: {lost} frames lost a write");
    assert_eq!(
        lost, 0,
        "{lost} of {TRIALS} frames lost a write at a split that is 4-aligned and not word-aligned"
    );
}

#[test]
fn a_word_aligned_split_of_a_packed_column_loses_no_writes() {
    let lost = frames_with_lost_writes(128, 64, TRIALS);
    println!("split at 64 of 128, {TRIALS} frames: {lost} frames lost a write");
    assert_eq!(lost, 0);
}

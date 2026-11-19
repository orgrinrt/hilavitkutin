//! B1a: the accumulator grouped by the data gives the same bytes at 1, 2, 3,
//! 4 and 256 workers under signed saturating addition.
//!
//! 256 is the most workers the engine admits (`MAX_CORES`), and the crew is
//! clamped to it, so it is the maximum worker count here too. Every run goes
//! through the frame the timed arm runs, on real worker threads.
//!
//! An equality counts only beside the case that must fail: the one partial
//! per core gives different bytes across the same counts, and so does a
//! grouping from each slice's start on the unrounded slice, the engine as
//! shipped, wherever a slice boundary falls inside a group.

use accum_group_support::arms::{
    GroupedArm,
    GroupedOnCeilArm,
    PerCoreArm,
    SequentialArm,
    run_once,
};
use accum_group_support::scheme::{ID_GROUPED, ID_GROUPED_ON_CEIL, ID_PER_CORE};
use accum_group_support::{AccInput, AccOutput, GROUPS, data, expected};

/// The worker counts of B1a.
const WORKERS: [u32; 5] = [1, 2, 3, 4, 256];

/// Record counts below a group, at a group, one past it, odd, and many
/// groups that the worker counts do not divide.
const COUNTS: [usize; 9] = [1, 3, 8, 9, 255, 1001, 2049, 6144, 65_537];

const SEED: u64 = 0x5EED_0000_0000_0001;

fn input(workers: u32, group: u32) -> AccInput {
    AccInput {
        seed: SEED,
        workers,
        group,
    }
}

/// The outputs at every worker count, with the worker echo cleared so they
/// compare on the result alone.
fn across<A: accum_group_support::arms::Arm>(n: usize, group: u32) -> Vec<(u32, AccOutput)> {
    WORKERS
        .iter()
        .map(|&w| {
            let mut out = run_once::<A>(&input(w, group), n);
            assert_eq!(out.workers, w);
            out.workers = 0;
            (w, out)
        })
        .collect()
}

fn distinct(outs: &[(u32, AccOutput)]) -> usize {
    let mut seen: Vec<AccOutput> = Vec::new();
    for (_, o) in outs {
        if !seen.contains(o) {
            seen.push(*o);
        }
    }
    seen.len()
}

#[test]
fn grouped_by_the_data_gives_the_same_bytes_at_every_worker_count() {
    for &group in &GROUPS {
        for &n in &COUNTS {
            let outs = across::<GroupedArm>(n, group);
            let (_, first) = outs[0];
            for (w, out) in &outs {
                assert_eq!(
                    *out, first,
                    "n = {n}, G = {group}: {w} workers differ from 1"
                );
            }
            let data = data::column(SEED, n);
            let mut model = expected(ID_GROUPED, &data, 1, group).unwrap();
            model.workers = 0;
            assert_eq!(
                first, model,
                "n = {n}, G = {group}: differs from the hand model"
            );
            assert_eq!(first.parts, n.div_ceil(group as usize) as u64);
        }
    }
}

#[test]
fn one_partial_per_core_gives_different_bytes_across_worker_counts() {
    // The control: the grouping by worker must be shown to move before an
    // equality of the grouping by the data may count.
    for &n in &[1001usize, 6144, 65_537] {
        let outs = across::<PerCoreArm>(n, GROUPS[0]);
        assert!(
            distinct(&outs) > 1,
            "n = {n}: per-core partials gave one result at every count"
        );
        for (w, out) in &outs {
            let mut model =
                expected(ID_PER_CORE, &data::column(SEED, n), *w as usize, GROUPS[0]).unwrap();
            model.workers = 0;
            assert_eq!(
                *out, model,
                "n = {n}, {w} workers: differs from the hand model"
            );
        }
    }
}

#[test]
fn grouping_from_each_unrounded_slice_start_fails_where_a_slice_cuts_a_group() {
    // The engine as shipped with a unit that groups by itself: at 1001
    // records the ceil slice at 2 workers is 501, which cuts a group of 8.
    let outs = across::<GroupedOnCeilArm>(1001, 8);
    assert!(
        distinct(&outs) > 1,
        "the unrounded slice gave the data's grouping at every count"
    );
    for (w, out) in &outs {
        let mut model = expected(
            ID_GROUPED_ON_CEIL,
            &data::column(SEED, 1001),
            *w as usize,
            8,
        )
        .unwrap();
        model.workers = 0;
        assert_eq!(*out, model, "{w} workers: differs from the hand model");
    }
    // Where every slice is whole groups, the two agree: 6144 records over 1,
    // 2, 3, 4 workers are slices of 6144, 3072, 2048 and 1536, all multiples
    // of 8; at 256 workers the slice is 24.
    let whole = across::<GroupedOnCeilArm>(6144, 8);
    let grouped = across::<GroupedArm>(6144, 8);
    assert_eq!(distinct(&whole), 1);
    let result = |o: AccOutput| (o.digest, o.parts, o.combined);
    assert_eq!(result(whole[0].1), result(grouped[0].1));
}

#[test]
fn the_grouped_result_is_not_the_left_fold_of_the_records() {
    // Option 3 fixes the answer across worker counts by changing what the
    // answer is, at one worker too.
    let n = 1001;
    let seq = run_once::<SequentialArm>(&input(1, 8), n);
    let grouped = run_once::<GroupedArm>(&input(1, 8), n);
    assert_ne!(seq.combined, grouped.combined);
    assert_eq!(seq.parts, 1);
}

#[test]
fn a_record_count_of_zero_keeps_no_partial_in_any_scheme() {
    let empty = |o: AccOutput| o.parts == 0 && o.combined == 0;
    assert!(empty(run_once::<SequentialArm>(&input(4, 8), 0)));
    assert!(empty(run_once::<PerCoreArm>(&input(4, 8), 0)));
    assert!(empty(run_once::<GroupedArm>(&input(4, 8), 0)));
}

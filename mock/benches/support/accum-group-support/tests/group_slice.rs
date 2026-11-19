//! `group_slice`, the split option 3 rests on: at every walker count from 1
//! to 256, the slices are in sequence, disjoint, cover the records, and
//! every slice that has records starts on a group boundary.

use accum_group_support::scheme::group_slice;

const COUNTS: [usize; 10] = [0, 1, 7, 8, 9, 255, 1001, 2049, 6144, 65_537];
const WIDTHS: [usize; 4] = [1, 8, 64, 1024];

#[test]
fn the_slices_tile_the_records_on_group_boundaries() {
    for n in COUNTS {
        for g in WIDTHS {
            for walkers in 1 ..= 256 {
                let mut next = 0;
                for core in 0 .. walkers {
                    let s = group_slice(core, walkers, n, g);
                    assert!(
                        s.start <= s.end,
                        "n {n} g {g} w {walkers} core {core}: {s:?}"
                    );
                    if s.is_empty() {
                        continue;
                    }
                    assert_eq!(
                        s.start, next,
                        "n {n} g {g} w {walkers} core {core}: a gap or overlap"
                    );
                    assert_eq!(
                        s.start % g,
                        0,
                        "n {n} g {g} w {walkers} core {core}: starts inside a group"
                    );
                    next = s.end;
                }
                assert_eq!(
                    next, n,
                    "n {n} g {g} w {walkers}: the slices do not cover the records"
                );
            }
        }
    }
}

/// The control: the unrounded ceil slice, which the same check refuses at
/// a count where a slice boundary falls inside a group.
#[test]
fn the_ceil_slice_starts_inside_a_group_at_1001_records() {
    use stc_support::split::ceil_slice;
    let starts_inside = (0 .. 2).any(|core| {
        let s = ceil_slice(core, 2, 1001);
        !s.is_empty() && s.start % 8 != 0
    });
    assert!(starts_inside);
}

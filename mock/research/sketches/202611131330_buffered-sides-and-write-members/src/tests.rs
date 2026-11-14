//! What the sketch is held to. Each test names the sentence of the design
//! it stands under.

extern crate std;

use core::cell::Cell;
use std::vec::Vec;

use super::*;

/// A step that reads both neighbours: the sum of the record and the
/// records beside it.
fn spread(w: Window<'_, u32>) -> u32 {
    w.before(1).unwrap_or(0) + w.here() + w.after(1).unwrap_or(0)
}

fn seed() -> [u32; 8] {
    [1, 0, 0, 5, 0, 0, 0, 2]
}

fn run_spread<M: BufferMode>(reverse: bool, frames: usize) -> Vec<u32> {
    let mut a = seed();
    let mut b = seed();
    let mut s = Sides::<u32, M>::new(&mut a, &mut b);
    for _ in 0 .. frames {
        if reverse {
            s.step((0 .. 8).rev(), Rule::Differs, spread);
        } else {
            s.step(0 .. 8, Rule::Differs, spread);
        }
        s.close();
    }
    s.frozen().to_vec()
}

/// "Every read of it sees the frozen side": a unit that reads the store it
/// writes, with a reach, gets one result whatever sequence the records are
/// visited in. `mutant-read-live` turns this red.
#[test]
fn a_step_over_its_own_store_does_not_depend_on_the_sequence_of_records() {
    for frames in 1 .. 4 {
        assert_eq!(
            run_spread::<Snapshot>(false, frames),
            run_spread::<Snapshot>(true, frames)
        );
        assert_eq!(
            run_spread::<Exchange>(false, frames),
            run_spread::<Exchange>(true, frames)
        );
    }
    // And it is the value worked out by hand from the frozen side alone.
    assert_eq!(run_spread::<Exchange>(false, 1), [1, 1, 5, 5, 5, 0, 2, 2]);
}

/// The two modes are two ways to the same values when the writer writes
/// every record.
#[test]
fn both_modes_give_the_same_values_under_a_total_write() {
    for frames in 1 .. 5 {
        assert_eq!(
            run_spread::<Snapshot>(false, frames),
            run_spread::<Exchange>(false, frames)
        );
    }
}

/// "A store no unit wrote costs nothing there". `mutant-swap-when-unwritten`
/// turns the exchange half red.
#[test]
fn a_store_nobody_wrote_keeps_its_frozen_side() {
    let mut a = seed();
    let mut b = [9; 8];
    let mut s = Sides::<u32, Exchange>::new(&mut a, &mut b);
    let before = s.frozen().as_ptr();
    assert!(!s.close());
    assert_eq!(s.frozen().as_ptr(), before);
    assert_eq!(s.frozen(), seed());
}

/// Under `Exchange` the sides trade places and neither region moves or is
/// copied. `mutant-exchange-copies` turns this red.
#[test]
fn exchange_trades_the_sides_and_moves_no_base() {
    let mut a = seed();
    let mut b = [0; 8];
    let (pa, pb) = (a.as_ptr(), b.as_ptr());
    let mut s = Sides::<u32, Exchange>::new(&mut a, &mut b);
    assert_eq!(s.frozen().as_ptr(), pa);
    s.step(0 .. 8, Rule::Written, |w| w.here() + 1);
    assert!(s.close());
    assert_eq!(s.frozen().as_ptr(), pb);
    s.step(0 .. 8, Rule::Written, |w| w.here() + 1);
    assert!(s.close());
    assert_eq!(s.frozen().as_ptr(), pa);
    assert_eq!(s.frozen()[0], 3);
}

/// Under `Snapshot` the frozen side is the same region in every frame and
/// takes the live side's values at the boundary.
#[test]
fn snapshot_copies_into_the_one_frozen_region() {
    let mut a = seed();
    let mut b = seed();
    let pa = a.as_ptr();
    let mut s = Sides::<u32, Snapshot>::new(&mut a, &mut b);
    s.step(0 .. 8, Rule::Written, |w| w.here() + 1);
    assert!(s.close());
    assert_eq!(s.frozen().as_ptr(), pa);
    assert_eq!(s.frozen()[0], 2);
}

/// Why `Exchange` asks a total write. A writer that writes some records
/// only reads back, a frame later, the value of two frames before at the
/// records it left. Under `Snapshot` the same writer is right.
#[test]
fn a_partial_write_is_right_under_snapshot_and_stale_under_exchange() {
    fn two_frames<M: BufferMode>() -> Vec<u32> {
        let mut a = [0u32; 4];
        let mut b = [0u32; 4];
        let mut s = Sides::<u32, M>::new(&mut a, &mut b);
        // Frame one writes every record to 7.
        s.step(0 .. 4, Rule::Written, |_| 7);
        s.close();
        // Frame two writes record 0 only.
        s.step(0 .. 1, Rule::Written, |_| 8);
        s.close();
        s.frozen().to_vec()
    }
    assert_eq!(two_frames::<Snapshot>(), [8, 7, 7, 7]);
    assert_eq!(two_frames::<Exchange>(), [8, 0, 0, 0]);
}

/// "Changed only where the value differs from the value the record held
/// when the frame began": a step that returns what it read changes nothing
/// and the boundary leaves the store alone; `Written` swaps all the same.
#[test]
fn a_step_that_returns_what_it_read_swaps_only_under_written() {
    let mut a = seed();
    let mut b = seed();
    let mut s = Sides::<u32, Exchange>::new(&mut a, &mut b);
    s.step(0 .. 8, Rule::Differs, |w| w.here());
    assert!(!s.close());
    s.step(0 .. 8, Rule::Written, |w| w.here());
    assert!(s.close());
    assert_eq!(s.frozen(), seed());
}

/// Two writers of one buffered store in one frame, in the sequence the
/// schedule gives them: the later one's values are what the boundary
/// brings in, and both read the same frozen side.
#[test]
fn a_second_writer_reads_the_frozen_side_too() {
    let mut a = seed();
    let mut b = seed();
    let mut s = Sides::<u32, Exchange>::new(&mut a, &mut b);
    s.step(0 .. 8, Rule::Written, |w| w.here() + 100);
    s.step(0 .. 8, Rule::Written, |w| w.here() + 1);
    s.close();
    assert_eq!(s.frozen()[0], 2);
}

// ----- the positional write follows the member -----

#[derive(Copy, Clone, PartialEq, Debug)]
struct Heat(u32);
#[derive(Copy, Clone, PartialEq, Debug)]
struct Wet(u32);
#[derive(Copy, Clone, PartialEq, Debug)]
struct Lit(u32);

type Three = Cons<Column<Heat>, Cons<Written<Column<Wet>>, Cons<Differs<Column<Lit>>, Empty>>>;
// The bundle in another sequence than the list, as a runner may have it.
type ThreePtrs = PtrCons<Lit, PtrCons<Heat, PtrCons<Wet, PtrNil>>>;

/// One `write::<T, _>` with the two generic arguments it has today reaches
/// a column under each of the three members, and the mark follows the
/// member. The list's index and the bundle's are inferred together.
#[test]
fn the_positional_write_follows_the_member() {
    let mut heat = [Heat(1); 4];
    let mut wet = [Wet(1); 4];
    let mut lit = [Lit(1); 4];
    let marks = [Cell::new(false), Cell::new(false), Cell::new(false)];
    // SAFETY: each base is the column its pointer in the bundle is for,
    // live for the writer's life, and every position below is under four.
    let w: Writer<'_, Three, ThreePtrs, 3> = unsafe {
        Writer::new(
            [lit.as_mut_ptr().cast(), heat.as_mut_ptr().cast(), wet.as_mut_ptr().cast()],
            &marks,
        )
    };

    // The same value as the record holds, to each.
    // SAFETY: position 2 is in range.
    unsafe {
        w.write::<Heat, _>(2, Heat(1));
        w.write::<Wet, _>(2, Wet(1));
        w.write::<Lit, _>(2, Lit(1));
    }
    assert!(
        marks[1].get(),
        "a bare member counts when written, in this sketch"
    );
    assert!(marks[2].get(), "a Written member counts when written");
    assert!(
        !marks[0].get(),
        "a Differs member does not count an equal value"
    );

    // SAFETY: position 3 is in range.
    unsafe { w.write::<Lit, _>(3, Lit(9)) };
    assert!(
        marks[0].get(),
        "a Differs member counts a value that differs"
    );
    assert_eq!(lit[3], Lit(9));
    assert_eq!(heat[2], Heat(1));
    assert_eq!(wet[2], Wet(1));
}

/// A value type with no comparison is writable under `Written` and under a
/// bare member. Under `Differs` it is refused, which the feature
/// `refused-differs-without-eq` shows.
#[test]
fn a_value_with_no_comparison_is_written_under_written() {
    struct NoEq(u32);
    let mut col = [NoEq(0), NoEq(0)];
    let marks = [Cell::new(false)];
    // SAFETY: one column of two records, positions below two.
    let w: Writer<'_, Cons<Written<Column<NoEq>>, Empty>, PtrCons<NoEq, PtrNil>, 1> =
        unsafe { Writer::new([col.as_mut_ptr().cast()], &marks) };
    // SAFETY: position 1 is in range.
    unsafe { w.write::<NoEq, _>(1, NoEq(5)) };
    assert!(marks[0].get());
    assert_eq!(col[1].0, 5);
}

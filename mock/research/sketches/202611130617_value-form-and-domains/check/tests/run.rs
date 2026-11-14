//! The units of `author`, unchanged, under the engine and under the oracle.

use core::cell::Cell;

use author::{ALARM, CROSSED, Crossed, Diffuse, Drift, HERE, Heat, POS, Pos};
use check::{Stores, Tally, seeded};
use plumb::World;

fn heat(world: &World<Stores>) -> Vec<i64> {
    world.lane(HERE).front.iter().map(|h| h.0).collect()
}

fn pos(world: &World<Stores>) -> Vec<i64> {
    world.lane(POS).front.iter().map(|p| p.0).collect()
}

/// The appends, sorted: the sequence of appends is not part of the contract.
fn crossed(world: &World<Stores>) -> Vec<Crossed> {
    let mut all: Vec<Crossed> = world
        .lane(CROSSED)
        .front
        .iter()
        .map(|c| c.unwrap())
        .collect();
    all.sort();
    all
}

/// A known answer, worked by hand, so that two runners wrong the same way do
/// not pass: a 2 by 2 plane, key order north first.
#[test]
fn one_pass_of_diffuse_matches_the_hand_worked_plane() {
    let mut world = seeded(2, 2, 1);
    // seeded heat by index: (i * 7) % 11 - 3 = [-3, 4, 0, 7]
    // index = north * 2 + east: (n0,e0) -3, (n0,e1) 4, (n1,e0) 0, (n1,e1) 7
    assert_eq!(heat(&world), [-3, 4, 0, 7]);
    let tally = Cell::new(0);
    engine::pass(&(Diffuse, ()), &mut world, &tally);
    // here + east + north - west - south + 2 * north_east + rate, outside is 0, rate is 2
    // (n0,e0): -3 + 4 + 0 - 0 - 0 + 2 * 7 + 2 = 17
    // (n0,e1):  4 + 0 + 7 - (-3) - 0 + 0 + 2 = 16
    // (n1,e0):  0 + 7 + 0 - 0 - (-3) + 0 + 2 = 12
    // (n1,e1):  7 + 0 + 0 - 0 - 4 + 0 + 2 = 5
    assert_eq!(heat(&world), [17, 16, 12, 5]);
}

/// The same units under both runners, several passes, two domains of different
/// counts in one world.
#[test]
fn both_runners_leave_the_same_stores() {
    let mut a = seeded(3, 4, 7);
    let mut b = seeded(3, 4, 7);
    let tally = Cell::new(0);
    for _ in 0 .. 5 {
        engine::pass(&(Diffuse, (Drift, (Tally, ()))), &mut a, &tally);
        oracle::pass(&(Diffuse, (Drift, ())), &mut b);
    }
    assert_eq!(heat(&a).len(), 12);
    assert_eq!(pos(&a).len(), 7);
    assert_eq!(heat(&a), heat(&b));
    assert_eq!(pos(&a), pos(&b));
    assert_eq!(crossed(&a), crossed(&b));
    assert_eq!(a.lane(ALARM).front, b.lane(ALARM).front);
    // The block-form unit ran beside the value-form ones, once a pass.
    assert_eq!(tally.get(), 5);
    // The stores moved at all: a pass that did nothing would also agree.
    assert_ne!(heat(&a), heat(&seeded(3, 4, 7)));
    assert_eq!(pos(&a)[3], 90 + 3 + 5 * 3);
    assert!(!crossed(&a).is_empty());
    assert_eq!(a.lane(ALARM).front, [true]);
}

/// A write to a frame-buffered store is not seen until the close.
#[test]
fn a_reader_after_the_writer_still_reads_the_committed_frame() {
    let mut world = seeded(2, 2, 1);
    let tally = Cell::new(0);
    // Two Diffuse units in one pass: the second reads what the first read.
    // There is one writer of a frame-buffered store in a real plan; the sketch
    // engine does not refuse two, and here that lets the committed side be seen
    // twice in one pass.
    engine::pass(&(Diffuse, (Diffuse, ())), &mut world, &tally);
    assert_eq!(heat(&world), [17, 16, 12, 5]);
    let _ = (Heat(0), Pos(0));
}

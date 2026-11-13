//! The position mask at list lengths 8 and 32, at the default recursion limit.

mod common;

use common::*;

type Lo8 = list!(L0 L1 L2 L3);
type Hi8 = list!(L4 L5 L6 L7);
type R8 = list!(L0 L1 L2 L3 L4 L5 L6 L7);

type Lo32 = list!(
    L0 L1 L2 L3 L4 L5 L6 L7 L8 L9 L10 L11 L12 L13 L14 L15
);
type Hi32 = list!(
    L16 L17 L18 L19 L20 L21 L22 L23 L24 L25 L26 L27 L28 L29 L30 L31
);
type R32 = list!(
    L0 L1 L2 L3 L4 L5 L6 L7 L8 L9 L10 L11 L12 L13 L14 L15
    L16 L17 L18 L19 L20 L21 L22 L23 L24 L25 L26 L27 L28 L29 L30 L31
);

writes_in!(R8: R8, Lo8, Hi8);
writes_in!(R32: R32, Lo32, Hi32);

at_length!(the_mask_evaluates_at_length_8, R8, Lo8, Hi8, 8);
at_length!(the_mask_evaluates_at_length_32, R32, Lo32, Hi32, 32);

// Write lists out of registry order, with gaps, and interleaved with each
// other: the halves above are in order and contiguous, which a mask folded
// from list position rather than registry position would also pass.
type Scattered = list!(L30 L3 L17);
type Evens = list!(L6 L0 L4 L2);
type Odds = list!(L1 L7 L3 L5);

writes_in!(R32: Scattered);
writes_in!(R8: Evens, Odds);

#[test]
fn an_out_of_order_list_with_gaps_masks_its_registry_places() {
    let scattered = engine::Block(Writes::<Scattered>::new());
    assert_eq!(
        mask_of::<R32, _, _>(&scattered),
        ((1 << 30) | (1 << 3) | (1 << 17), 3)
    );
    let mut frame: engine::Frame<R32, 1> = engine::Frame::new();
    frame.step(scattered);
}

#[test]
fn two_interleaved_out_of_order_lists_are_disjoint_and_cover_the_registry() {
    let evens = engine::Block(Writes::<Evens>::new());
    let odds = engine::Block(Writes::<Odds>::new());
    assert_eq!(mask_of::<R8, _, _>(&evens), (0b0101_0101, 4));
    assert_eq!(mask_of::<R8, _, _>(&odds), (0b1010_1010, 4));
    let mut frame: engine::Frame<R8, 1> = engine::Frame::new();
    frame.step2(evens, odds);
}

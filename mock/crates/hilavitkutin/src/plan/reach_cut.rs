//! Where a per-core walk of the carrier is cut for a read with a reach.
//!
//! A read with a reach behind a writer needs the writer's whole range landed
//! on every core before any core starts the reader. The phases carry that
//! boundary for the phase loop. A path that hands each core a slice of the
//! records and walks the carrier over it has no phases, so it walks the
//! consumer units in segments instead, cut ahead of each such reader, with a
//! barrier across the cores between two segments.

use arvo::strategy::{Additive, Identity};
use arvo::{Bool, USize};
use arvo_bitmask::BitAccess;
use arvo_tensor::{Capacity, ConstCapacity};
use hilavitkutin_api::meta::RANK_CONSUMER;

use super::bundle_masks::BundleMasks;
use super::edge::reach_behind_a_writer;
use super::unit_masks::{masks_of, reaches_of};

/// Per-carrier-position mask of the consumer units that read a plain column
/// with a reach behind a writer of it.
///
/// Bit `p` is set when the unit at position `p` is of the consumer rank and
/// `reach_behind_a_writer` holds for it. A unit of another rank is left out:
/// the bands run on one thread over the whole record range. The second part
/// says whether any bit is set, so a carrier with no such reader can keep the
/// walk it had.
pub const fn reach_cuts<
    Wus,
    Stores,
    Witnesses,
    CU: Capacity + [const] ConstCapacity,
    CS: Capacity,
    Adj,
>() -> (Adj, Bool)
where
    Wus: [const] BundleMasks<Stores, Witnesses, CS>,
    Adj: [const] BitAccess + Identity<Additive>,
{
    let (_reads, writes, ranks, n) = masks_of::<Wus, Stores, Witnesses, CU, CS>();
    let reaches = reaches_of::<Wus, Stores, Witnesses, CU, CS>();
    let count = n.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: loop bound; tracked: #121
    let mut cuts = <Adj as Identity<Additive>>::IDENTITY;
    let mut any = Bool::FALSE;
    let mut p = 0; // lint:allow(no-bare-numeric) reason: unit index; tracked: #121
    while p < count {
        if <CU as ConstCapacity>::get(&ranks, USize(p)).0 == RANK_CONSUMER.0
            && reach_behind_a_writer(
                <CU as ConstCapacity>::slice(&reaches),
                <CU as ConstCapacity>::slice(&writes),
                <CU as ConstCapacity>::slice(&ranks),
                USize(p),
            )
            .0
        {
            cuts = cuts.with_bit_set(USize(p));
            any = Bool::TRUE;
        }
        p += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
    (cuts, any)
}

/// Walk the `consumers` among the first `units` carrier positions in segments
/// cut ahead of each position of `cuts`.
///
/// `each` is handed every segment's member mask in carrier sequence, with
/// whether it is the last. A cut with no consumer unit since the cut before
/// it opens no segment, so no segment but the last is ever empty, and the
/// last is empty only when `consumers` is. The segments depend on the masks
/// alone, so every core that walks the same masks is handed the same number
/// of them, which is what lets the cores meet at a barrier after each.
#[inline(always)]
pub fn cut_segments<Adj>(cuts: Adj, consumers: Adj, units: USize, mut each: impl FnMut(Adj, Bool))
where
    Adj: BitAccess + Identity<Additive>,
{
    let mut segment = <Adj as Identity<Additive>>::IDENTITY;
    let mut open = Bool::FALSE;
    let mut u = 0; // lint:allow(no-bare-numeric) reason: unit index; tracked: #121
    while u < units.0 {
        if cuts.bit(USize(u)).0 && open.0 {
            each(segment, Bool::FALSE);
            segment = <Adj as Identity<Additive>>::IDENTITY;
            open = Bool::FALSE;
        }
        if consumers.bit(USize(u)).0 {
            segment = segment.with_bit_set(USize(u));
            open = Bool::TRUE;
        }
        u += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
    each(segment, Bool::TRUE);
}

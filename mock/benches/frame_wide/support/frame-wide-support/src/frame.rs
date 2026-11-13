//! The frame itself, stated once, and the negative control beside it.
//!
//! This is the oracle. Every arm is checked against `record` rather than only
//! against its neighbours, so a defect every arm shared would still be caught.
//! `record_without_a2` is the smallest wrong frame, one stage of twelve
//! missing, and it exists to be rejected: the cross-arm validation is worth
//! nothing until it has been seen refusing something.

use crate::*;

/// The fan-in: one record of the join column from the three pipeline tails.
#[inline(always)]
pub fn join<KN: Kernel>(a: u32, b: u32, c: u32) -> u32 {
    KN::apply(a ^ b.rotate_left(7) ^ c.rotate_left(19), K_J)
}

/// The whole frame for one record, fused: the three pipelines, the join and
/// the two transforms, returning `(join, t1, t2)`.
///
/// This is the reference the hand-written arms run and the thing every other
/// arm has to agree with. Nothing else states what the frame computes.
#[inline(always)]
pub fn record<KN: Kernel>(v: u32) -> (u32, u32, u32) {
    frame_record::<KN, false>(v)
}

/// The negative control: `record` with the second stage of the first pipeline
/// dropped, so `a3` is computed from `a1` directly.
///
/// It exists to be rejected. The cross-arm validation is only worth something
/// once it has been seen to refuse an arm that computes the wrong frame, and
/// this is the smallest wrong frame: one stage of twelve missing, every other
/// stage and every output column in place.
#[inline(always)]
pub fn record_without_a2<KN: Kernel>(v: u32) -> (u32, u32, u32) {
    frame_record::<KN, true>(v)
}

/// The one statement of the per-record frame, with the negative control's
/// dropped stage as a compile-time switch so the two cannot drift apart.
#[inline(always)]
fn frame_record<KN: Kernel, const DROP_A2: bool>(v: u32) -> (u32, u32, u32) {
    let a1 = KN::apply(v, K_A1);
    let a2 = if DROP_A2 { a1 } else { KN::apply(a1, K_A2) };
    let a3 = KN::apply(a2, K_A3);
    let b3 = KN::apply(KN::apply(KN::apply(v, K_B1), K_B2), K_B3);
    let c3 = KN::apply(KN::apply(KN::apply(v, K_C1), K_C2), K_C3);
    (
        join::<KN>(a3, b3, c3),
        KN::apply(v, K_T1),
        KN::apply(v, K_T2),
    )
}

/// The digest the frame must produce for these input records, computed from
/// `record` alone, one record at a time, with no column materialised.
///
/// This is what `validate_output` checks every arm against, so an arm is held
/// to the reference on its own and not only to its neighbours: a defect every
/// arm shared would pass the cross-arm comparison and still fail here.
pub fn reference_digest<KN: Kernel>(records: &[u32]) -> FrameOut {
    let (mut j, mut t1, mut t2) = (Fnv1a::new(), Fnv1a::new(), Fnv1a::new());
    for &v in records {
        let (rj, r1, r2) = record::<KN>(v);
        j.push(rj);
        t1.push(r1);
        t2.push(r2);
    }
    [j.finish(), t1.finish(), t2.finish()]
}

/// How many columns the frame reserves: the input, nine pipeline columns, the
/// join column and the two transform columns.
pub const COLUMNS: usize = 13;

/// How many per-record stage applications one frame performs, counting the
/// join as one. Feeds the harness throughput table.
pub const STAGES_PER_RECORD: u64 = 12;

//! The position mask of a write list, and the staged handles cut for it.

use notko_hlist::{Cons, Empty, Position};

use crate::lane::{FrameLane, FrameList};
use crate::registry::{Pick, RawLanes, Registry};
use crate::sealed::Token;
use crate::slot::Slot;
use crate::view::StagedLane;

/// A write list placed in the registry `Reg`, at the positions `Ps` (one per
/// member, inferred).
///
/// `MASK` folds each member's place in the registry to one bit, `LEN` counts
/// the members. A list naming a lane twice folds two members to one bit, so
/// `MASK.count_ones() == LEN` holds exactly when every member is distinct.
/// Nothing here asserts it: the runner does, in an inline `const` block, where
/// the kernel is instantiated (`kernel.rs`).
///
/// The mask is a bare `u128` because the sketch may depend on `notko` and
/// `notko-hlist` only; the production form takes arvo's bit set, and a
/// registry past 128 lanes needs it, since the shift has no 129th bit.
///
/// Sealed (`sealed.rs`): the runner's unsafe cut trusts `MASK`, so only the
/// engine's own impls below exist.
pub trait Stage<Reg: Registry, Ps>: FrameList {
    const MASK: u128;
    const LEN: u32;

    /// The seal: an impl outside the engine cannot name `Token`.
    #[doc(hidden)]
    fn seal(_: Token);

    /// Cuts one staged handle per member out of the raw lanes.
    ///
    /// # Safety
    ///
    /// No other live reference reaches any lane in `MASK`, and the list names
    /// each lane once (`MASK.count_ones() == LEN`). Both are what the runner's
    /// `const` block establishes before it calls this.
    unsafe fn parts<'s>(raw: &RawLanes<'s, Reg>) -> Self::Parts<'s>;
}

impl<Reg: Registry> Stage<Reg, Empty> for Empty {
    const LEN: u32 = 0;
    const MASK: u128 = 0;

    fn seal(_: Token) {}

    unsafe fn parts<'s>(_raw: &RawLanes<'s, Reg>) {}
}

impl<Reg, H, T, P, Ps> Stage<Reg, Cons<P, Ps>> for Cons<H, T>
where
    Reg: Registry,
    H: FrameLane,
    T: Stage<Reg, Ps>,
    P: Position<Slot>,
    Reg::Raw: Pick<H, P, V = *mut [H::Item]>,
{
    const LEN: u32 = T::LEN + 1;
    const MASK: u128 = (1u128 << <P as Position<Slot>>::INDEX.0) | T::MASK;

    fn seal(_: Token) {}

    unsafe fn parts<'s>(raw: &RawLanes<'s, Reg>) -> Self::Parts<'s> {
        let lane: *mut [H::Item] = *raw.lanes.pick();
        // SAFETY: the pointer came from the staging buffers borrowed for 's
        // (`RawLanes::new`), and the caller's contract says nothing else
        // reaches this lane while the handle lives.
        let head = StagedLane::new(unsafe { &mut *lane });
        // SAFETY: the same contract, for the rest of the list.
        (head, unsafe { T::parts(raw) })
    }
}

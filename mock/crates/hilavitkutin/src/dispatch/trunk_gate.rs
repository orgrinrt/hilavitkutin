//! GATE-2 const-gated per-trunk walk (round 2a, domain 17).
//!
//! `RunGatedTrunk` walks the value carrier `WuCons` / `WuNil` and runs only the
//! units belonging to one `TRUNK`: each cell gates its head on the round-1
//! compile-time grouping over the full bundle (`Member::<..>::IS`), running the
//! member through the shipped `RunFiber::run_head` and recursing the tail. Every
//! non-member position's body is statically false, so dead-code elimination
//! collapses each `TRUNK` monomorphisation to its members only: one member-only
//! program per trunk, devirt-clean (the per-unit body is the same `RunFiber`
//! projection `run_gated` already proves devirts; the gate is a
//! compile-time-known branch, strictly more optimisable than `run_gated`'s
//! runtime dirty bit). Proven shapes: sketches 071230 (const-gated DCE) + 070950
//! (real `MaskProject` grouping) + 080200 (outer dispatcher over trunk-only
//! keying).
//!
//! Trunk-only keyed: a trunk lies wholly within one phase (the grouping computes
//! trunks per phase), so the trunk id alone identifies a member; the redundant
//! phase const is gone. Phase order is the outer dispatcher's runtime phase loop
//! (`trunk_dispatch`), not a second gate here.
//!
//! The carrier position is a type-level Peano witness `Pos` (`Here` /
//! `There<..>`, `Pos::INDEX` is its usize), NOT a `const POS: usize`: the
//! recursion threads `There<Pos>` (a type, no const arithmetic), which avoids
//! the `{POS + 1}` generic constant the trait solver overflows normalising
//! through the recursion (the engine already represents positions as Peano
//! witnesses: `Locate` / `WitnessIndex`). `TRUNK` stays a const generic, fixed
//! through the recursion.
//!
//! `Full` is the whole carrier type, fixed through the recursion so the gate
//! references the entire bundle's grouping, not the shrinking tail. `GW` is the
//! grouping witness list (the `(RI, WI)` MaskProject pairs); `Witnesses` is the
//! per-unit projection-index list `RunFiber` consumes. The outer all-trunks
//! dispatcher that runs every trunk in phase order, and the `Scheduler::run`
//! re-point, live in `dispatch::trunk_dispatch` (round 2b).

use core::marker::PhantomData;

use arvo::USize;
use arvo::strategy::Identity;
use arvo_bitmask::BitAccess;
use arvo_tensor::{Capacity, ConstCapacity};
use hilavitkutin_api::HasSchedule;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::work_unit_values::{WuCons, WuNil};

use crate::dispatch::engine_ctx::{GateWith, There};
use crate::dispatch::fiber_run::RunFiber;
use crate::dispatch::morsel::MorselRange;
use crate::meta::MetaBlock;
use crate::plan::grouping::{BundleMasks, trunk_of};
use crate::plan::project::WitnessIndex;

/// Carries the compile-time membership of carrier position `Pos` in `TRUNK` as
/// an associated const, so the gate reads `Member::<..>::IS` (a const usable in
/// `if`, which DCE collapses) rather than an anonymous `const { trunk_of::<..>()
/// }` block, which the generic-constant complexity limit rejects when the
/// grouping carries this many parameters.
///
/// Trunk-only keyed: a trunk lies wholly within one phase, so the trunk id alone
/// identifies membership. Phase order is the outer dispatcher's runtime phase
/// loop, not a second const gate here.
struct Member<Full, Stores, GW, CU, CS, Adj, Pos, const TRUNK: usize>(
    // lint:allow(no-bare-numeric) reason: const-generic trunk carrier; tracked: #121
    PhantomData<(Full, Stores, GW, CU, CS, Adj, Pos)>,
);

impl<
    Full,
    Stores,
    GW,
    CU: Capacity + const ConstCapacity,
    CS: Capacity,
    Adj: const BitAccess + Identity,
    Pos: WitnessIndex,
    const TRUNK: usize,
> Member<Full, Stores, GW, CU, CS, Adj, Pos, TRUNK>
// lint:allow(no-bare-numeric) reason: const-generic trunk carrier; tracked: #121
where
    Full: const BundleMasks<Stores, GW, CS>,
{
    const IS: bool = trunk_of::<Full, Stores, GW, CU, CS, Adj>(Pos::INDEX).0 == TRUNK; // lint:allow(no-bare-numeric) reason: membership-gate const bool; tracked: #121
}

/// Walk the carrier running only the members of phase `PHASE`, trunk `TRUNK`.
///
/// `Full` is the whole carrier type (fixed through the recursion); `A` the
/// scheduler bindings; `Witnesses` the per-unit `RunFiber` projection list; `GW`
/// the grouping witness list; `Stores` the global store set; `CU` / `CS` the
/// unit / store capacities; `Pos` the head's carrier position (Peano witness).
pub trait RunGatedTrunk<
    Full,
    A,
    Witnesses,
    GW,
    Stores,
    CU: Capacity,
    CS: Capacity,
    Adj,
    const TRUNK: usize,
    Pos,
>
{
    // lint:allow(no-bare-numeric) reason: const-generic trunk carrier; tracked: #121
    /// Run this carrier's members of `TRUNK` over `morsel`, head-first.
    ///
    /// `dirty` carries one bit per carrier position (the incremental-skip mask);
    /// a member whose bit is clear is skipped, exactly as `RunFiber::run_gated`
    /// does. Pass an all-ones mask for the no-skip path. The dirty test is a
    /// predicated branch around the member's `RunFiber` step, so the per-trunk
    /// mono still devirtualises.
    ///
    /// `epoch` is the current pass epoch: an `On<V>` member runs only when its
    /// `Virtual<V>` stamp equals `epoch` (the firer set it this pass). The
    /// per-unit gate index (`GI`) is the sixth element of each unit's `RunFiber`
    /// witness tuple; `Always` members const-fold their gate to true.
    fn run_trunk<M: BitAccess>(
        &self,
        bindings: &A,
        meta_block: &MetaBlock,
        morsel: MorselRange,
        dirty: M,
        epoch: USize,
    );
}

impl<Full, A, GW, Stores, CU: Capacity, CS: Capacity, Adj, const TRUNK: usize, Pos>
    RunGatedTrunk<Full, A, Empty, GW, Stores, CU, CS, Adj, TRUNK, Pos> for WuNil
{
    // lint:allow(no-bare-numeric) reason: const-generic trunk carrier; tracked: #121
    #[inline]
    fn run_trunk<M: BitAccess>(
        &self,
        _bindings: &A,
        _meta_block: &MetaBlock,
        _morsel: MorselRange,
        _dirty: M,
        _epoch: USize,
    ) {
    }
}

impl<
    Full,
    A,
    W,
    Tail,
    RI,
    RCI,
    WCI,
    WAI,
    WVI,
    GI,
    WTail,
    GW,
    Stores,
    CU: Capacity + const ConstCapacity,
    CS: Capacity,
    Adj: const BitAccess + Identity,
    Pos: WitnessIndex,
    const TRUNK: usize,
>
    RunGatedTrunk<
        Full,
        A,
        Cons<(RI, RCI, WCI, WAI, WVI, GI), WTail>,
        GW,
        Stores,
        CU,
        CS,
        Adj,
        TRUNK,
        Pos,
    > for WuCons<W, Tail>
// lint:allow(no-bare-numeric) reason: const-generic trunk carrier; tracked: #121
where
    WuCons<W, Tail>: RunFiber<A, Cons<(RI, RCI, WCI, WAI, WVI, GI), WTail>>,
    W: HasSchedule,
    <W as HasSchedule>::Sched: GateWith<A, GI>,
    Full: const BundleMasks<Stores, GW, CS>,
    Tail: RunGatedTrunk<Full, A, WTail, GW, Stores, CU, CS, Adj, TRUNK, There<Pos>>,
{
    #[inline]
    fn run_trunk<M: BitAccess>(
        &self,
        bindings: &A,
        meta_block: &MetaBlock,
        morsel: MorselRange,
        dirty: M,
        epoch: USize,
    ) {
        // Member of this trunk (compile-time) AND not skipped this frame
        // (runtime incremental-skip bit) AND its schedule opens this pass (const
        // true for Always, a stamp == epoch test for On<V>). Non-members fold
        // away at DCE; clean Always members cost only the dirty bit test.
        if Member::<Full, Stores, GW, CU, CS, Adj, Pos, TRUNK>::IS
            && dirty.bit(Pos::INDEX).0
            && <<W as HasSchedule>::Sched as GateWith<A, GI>>::open(bindings, epoch).0
        {
            self.run_head(bindings, meta_block, morsel, epoch);
        }
        self.tail
            .run_trunk(bindings, meta_block, morsel, dirty, epoch);
    }
}

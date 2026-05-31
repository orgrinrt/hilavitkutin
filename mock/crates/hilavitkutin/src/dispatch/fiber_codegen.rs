//! Type-erased per-fiber dispatch slots (domain 17, HILA-RUNTIME C2 slice 3).
//!
//! The sibling of `RunFiber`. Where `RunFiber` projects each unit's
//! `EngineCtx` and runs `execute` inline, in cons-list (registration) order,
//! `CollectFiber` records one type-erased dispatch slot per unit at its
//! registration index and leaves the dispatch order to the caller. The
//! scheduler walks the plan's topological permutation over the collected
//! slots, so a unit's data dependencies run before it regardless of where it
//! sits in the registration list.
//!
//! A slot is an erased instance pointer plus a per-unit shim function. The
//! shim casts the pointer back to the unit's concrete type, projects its
//! `EngineCtx` exactly as `RunFiber::run` does, and runs `execute` via the
//! `invoke_wu_in_fiber` shim. The walk stays resource-only: the projection
//! bounds mirror `RunFiber`, so a unit that reads or writes a column has a
//! non-empty column projection and cannot enter the collection.

use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::work_unit_values::{WuCons, WuNil};
use hilavitkutin_api::WorkUnit;

use crate::dispatch::engine_ctx::{ColProject, ColPtrNil, EngineCtx, Project};
use crate::dispatch::morsel::MorselRange;
use crate::dispatch::wu_fn::invoke_wu_in_fiber;

/// One type-erased fiber dispatch slot.
///
/// The first element is the erased instance pointer (`*const W` of a retained
/// unit, cast to `*const ()`); the second is the per-unit shim that casts it
/// back, projects the unit's `EngineCtx`, and runs `execute`. `Copy`, so the
/// slot array materialises trivially. `A` is the arena the shim projects from.
pub type FiberSlot<A> = (*const (), fn(*const (), &A, MorselRange));

/// Placeholder shim for slots past the live unit count.
///
/// Never dispatched: `run` walks only the live prefix of the topological
/// permutation. Exists so the slot array can be filled before `CollectFiber`
/// overwrites the live entries.
#[inline]
pub fn noop_fiber_shim<A>(_ptr: *const (), _arena: &A, _morsel: MorselRange) {}

/// Per-unit dispatch shim, monomorphised per `(W, A, RIdx)`.
///
/// Casts the erased pointer back to `&W`, projects the unit's `EngineCtx` from
/// the arena (resource-only, identical to `RunFiber::run`), and runs `execute`
/// through the `invoke_wu_in_fiber` shim.
#[inline]
fn fiber_shim<W, A, RIdx>(ptr: *const (), arena: &A, morsel: MorselRange)
where
    W: WorkUnit,
    A: Project<<W as WorkUnit>::Read, RIdx>,
    ColPtrNil: ColProject<<W as WorkUnit>::Read, Empty, Out = ColPtrNil>,
    ColPtrNil: ColProject<<W as WorkUnit>::Write, Empty, Out = ColPtrNil>,
    for<'f> W: WorkUnit<
        Ctx<'f> = EngineCtx<
            'f,
            <W as WorkUnit>::Read,
            <W as WorkUnit>::Write,
            <A as Project<<W as WorkUnit>::Read, RIdx>>::Out,
            ColPtrNil,
            ColPtrNil,
        >,
    >,
{
    // SAFETY: `ptr` is the address of a retained `W` instance, recorded by
    // `CollectFiber::collect` from `&self.head` while walking the scheduler's
    // `wu_values` list. The scheduler owns `wu_values` and neither mutates nor
    // moves it across a `run` call, so the instance lives for the duration of
    // this dispatch and the cast-and-borrow is valid.
    let unit: &W = unsafe { &*(ptr as *const W) };
    let ctx: <W as WorkUnit>::Ctx<'_> =
        EngineCtx::project::<A, ColPtrNil, RIdx, Empty, Empty>(arena, &ColPtrNil, morsel);
    invoke_wu_in_fiber(unit, &ctx);
}

/// Record one `FiberSlot` per retained unit, at its registration index.
///
/// `Witnesses` is the parallel per-unit resource-projection index list,
/// inferred at the call site exactly as `RunFiber`'s `Witnesses`. The walk
/// writes the head unit's slot into the first element of `slots` and recurses
/// on the tail, so a unit's slot index equals its cons-list position. That
/// position equals the unit's `unit_meta` id index, because the builder
/// prepends the unit bundle and the value list in lockstep; the scheduler
/// relies on that equality to index slots by the plan's permutation.
pub trait CollectFiber<A, Witnesses> {
    /// Write one slot per unit into `slots`, head-first.
    fn collect(&self, slots: &mut [FiberSlot<A>]);
}

impl<A> CollectFiber<A, Empty> for WuNil {
    #[inline]
    fn collect(&self, _slots: &mut [FiberSlot<A>]) {}
}

impl<A, W, Tail, RIdx, WTail> CollectFiber<A, Cons<RIdx, WTail>> for WuCons<W, Tail>
where
    W: WorkUnit,
    A: Project<<W as WorkUnit>::Read, RIdx>,
    ColPtrNil: ColProject<<W as WorkUnit>::Read, Empty, Out = ColPtrNil>,
    ColPtrNil: ColProject<<W as WorkUnit>::Write, Empty, Out = ColPtrNil>,
    // Tie each unit's Ctx GAT to the projection of its Read set over the
    // shared arena, with empty column projections, for all frame lifetimes:
    // the same resource-only boundary `RunFiber` enforces. A unit reading or
    // writing a column has a non-empty column projection and fails this bound.
    for<'f> W: WorkUnit<
        Ctx<'f> = EngineCtx<
            'f,
            <W as WorkUnit>::Read,
            <W as WorkUnit>::Write,
            <A as Project<<W as WorkUnit>::Read, RIdx>>::Out,
            ColPtrNil,
            ColPtrNil,
        >,
    >,
    Tail: CollectFiber<A, WTail>,
{
    #[inline]
    fn collect(&self, slots: &mut [FiberSlot<A>]) {
        // `split_first_mut` writes the head slot and hands the tail the rest,
        // avoiding a bare index literal. The slot array is capacity-sized
        // (the unit count never exceeds it), so the `Some` arm always taken
        // for a non-empty unit list; the `None` arm is the empty-slice guard.
        if let Some((first, rest)) = slots.split_first_mut() {
            *first = (
                &self.head as *const W as *const (),
                fiber_shim::<W, A, RIdx>,
            );
            self.tail.collect(rest);
        }
    }
}

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
//! `EngineCtx` from the scheduler bindings (resources via `Project`, columns
//! via `ColProject` over the Read and Write sets, Shape A), and runs `execute`
//! via the `invoke_wu_in_fiber` shim. A column-bearing unit projects its real
//! column pointers; a resource-only unit projects empty column bundles and
//! dispatches exactly as before.

use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::work_unit_values::{WuCons, WuNil};
use hilavitkutin_api::WorkUnit;

use crate::dispatch::engine_ctx::{ColProject, EngineCtx, Project};
use crate::dispatch::morsel::MorselRange;
use crate::dispatch::wu_fn::invoke_wu_in_fiber;

/// One type-erased fiber dispatch slot.
///
/// The first element is the erased instance pointer (`*const W` of a retained
/// unit, cast to `*const ()`); the second is the per-unit shim that casts it
/// back, projects the unit's `EngineCtx`, and runs `execute`. `Copy`, so the
/// slot array materialises trivially. `A` is the bindings the shim projects from.
pub type FiberSlot<A> = (*const (), fn(*const (), &A, MorselRange));

/// Placeholder shim for slots past the live unit count.
///
/// Never dispatched: `run` walks only the live prefix of the topological
/// permutation. Exists so the slot array can be filled before `CollectFiber`
/// overwrites the live entries.
#[inline]
pub fn noop_fiber_shim<A>(_ptr: *const (), _bindings: &A, _morsel: MorselRange) {}

/// Per-unit dispatch shim, monomorphised per `(W, A, RIdx, RCIdx, WCIdx)`.
///
/// Casts the erased pointer back to `&W`, projects the unit's `EngineCtx` from
/// the bindings (resources via `Project`, columns via `ColProject` over the
/// Read and Write sets, fully-qualified as `EngineCtx::project` does it; the
/// bindings serve as both sources, Shape A), and runs `execute` through the
/// `invoke_wu_in_fiber` shim.
#[inline]
fn fiber_shim<W, A, RIdx, RCIdx, WCIdx>(ptr: *const (), bindings: &A, morsel: MorselRange)
where
    W: WorkUnit,
    A: Project<<W as WorkUnit>::Read, RIdx>,
    A: ColProject<<W as WorkUnit>::Read, RCIdx>,
    A: ColProject<<W as WorkUnit>::Write, WCIdx>,
    for<'f> W: WorkUnit<
        Ctx<'f> = EngineCtx<
            'f,
            <W as WorkUnit>::Read,
            <W as WorkUnit>::Write,
            <A as Project<<W as WorkUnit>::Read, RIdx>>::Out,
            <A as ColProject<<W as WorkUnit>::Read, RCIdx>>::Out,
            <A as ColProject<<W as WorkUnit>::Write, WCIdx>>::Out,
        >,
    >,
{
    // SAFETY: `ptr` is the address of a retained `W` instance, recorded by
    // `CollectFiber::collect` from `&self.head` while walking the scheduler's
    // `wu_values` list. The scheduler owns `wu_values` and neither mutates nor
    // moves it across a `run` call, so the instance lives for the duration of
    // this dispatch and the cast-and-borrow is valid.
    let unit: &W = unsafe { &*(ptr as *const W) };
    // The bindings are both the resource source and the column source; the
    // column pointers are read out (Copy) at projection time, so the second
    // borrow needs only to outlive this call.
    let ctx: <W as WorkUnit>::Ctx<'_> =
        EngineCtx::project::<A, A, RIdx, RCIdx, WCIdx>(bindings, bindings, morsel);
    invoke_wu_in_fiber(unit, &ctx);
}

/// Record one `FiberSlot` per retained unit, at its registration index.
///
/// `Witnesses` is the parallel per-unit projection-index list: each element is
/// the triple `(RIdx, RCIdx, WCIdx)` (the resource-projection index for the
/// Read set, the column-projection index for the Read set, and for the Write
/// set), all inferred at the call site, no caller turbofish. The walk
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

impl<A, W, Tail, RIdx, RCIdx, WCIdx, WTail>
    CollectFiber<A, Cons<(RIdx, RCIdx, WCIdx), WTail>> for WuCons<W, Tail>
where
    W: WorkUnit,
    A: Project<<W as WorkUnit>::Read, RIdx>,
    A: ColProject<<W as WorkUnit>::Read, RCIdx>,
    A: ColProject<<W as WorkUnit>::Write, WCIdx>,
    // Tie each unit's Ctx GAT to the projection of its Read set (resources)
    // and its Read / Write sets (columns) over the shared bindings, for all
    // frame lifetimes. A resource-only unit projects empty column bundles
    // (`ColProject` over a column-free set is `ColPtrNil`), so it dispatches
    // exactly as before; a column-bearing unit projects its real column
    // pointers.
    for<'f> W: WorkUnit<
        Ctx<'f> = EngineCtx<
            'f,
            <W as WorkUnit>::Read,
            <W as WorkUnit>::Write,
            <A as Project<<W as WorkUnit>::Read, RIdx>>::Out,
            <A as ColProject<<W as WorkUnit>::Read, RCIdx>>::Out,
            <A as ColProject<<W as WorkUnit>::Write, WCIdx>>::Out,
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
                fiber_shim::<W, A, RIdx, RCIdx, WCIdx>,
            );
            self.tail.collect(rest);
        }
    }
}

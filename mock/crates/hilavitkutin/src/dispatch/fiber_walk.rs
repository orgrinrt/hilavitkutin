//! Sequential WorkUnit walk over a fiber's unit sequence (domain 17,
//! HILA-RUNTIME C2 slice 1).
//!
//! A fiber is a sequential dispatch unit: an ordered sequence of
//! WorkUnits that run over one morsel window. To run a fiber the engine
//! constructs each unit's `Context` and calls `execute`. The single-unit
//! construct-and-execute is proven in `tests/engine_ctx.rs`; this module
//! adds the heterogeneous walk over a sequence, where each unit has its
//! own `Read` set, its own projection index, its own resource bundle, and
//! therefore its own `Ctx<'frame>` GAT instantiation.
//!
//! The walk is pure Rust generics, no codegen and no LLVM, matching the
//! C2 framing that the basic per-fiber dispatch loop is the generics core
//! and the LLVM follow-up is only the inlining and devirtualisation. It
//! reuses the shipped `engine_ctx` projection (`Project`,
//! `EngineCtx::project`) and the `wu_fn::invoke_wu_in_fiber` execute shim;
//! only the walk trait, the two value cells, and the entry function are
//! new here.
//!
//! Resource-only this slice. The per-unit `EngineCtx` carries `ColPtrNil`
//! for both column sides, so a unit that reads or writes a column does not
//! satisfy the walk's bound and cannot enter the sequence. Per-frame
//! column buffers depend on real column-buffer allocation from the
//! `MemoryProvider` (a later slice); resource pointers in the arena are
//! real and stable today. Feasibility was validated by the sketch at
//! `mock/research/sketches/202605300823_run-fiber-wutuple-walk`.

use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::WorkUnit;

use crate::dispatch::engine_ctx::{ColProject, ColPtrNil, EngineCtx, Project};
use crate::dispatch::morsel::MorselRange;
use crate::dispatch::wu_fn::invoke_wu_in_fiber;

/// Terminator for a fiber's value-carrying WorkUnit sequence.
pub struct WuNil;

/// Cons cell: a WorkUnit instance at this position plus the tail.
///
/// Value-carrying analogue of the type-level access-set cons, following
/// the `PtrCons` / `PtrNil` cell convention in `engine_ctx`.
pub struct WuCons<W, Tail> {
    /// The unit at this position in the sequence.
    pub head: W,
    /// The remaining units.
    pub tail: Tail,
}

/// Drive a fiber's WorkUnit sequence over a shared arena.
///
/// `Witnesses` is the parallel per-unit resource-projection index list
/// (one index list per unit), inferred at the entry call exactly as
/// `Project<R, Indices>` infers its selector indices. Carrying it as a
/// trait parameter constrains each per-unit index, dodging the
/// unconstrained-parameter error, the same way `plan::project::BundleProject`
/// carries its parallel witness list.
pub trait RunFiber<A, Witnesses> {
    /// Construct each unit's `EngineCtx` from the arena and run `execute`,
    /// in sequence order.
    fn run(&self, arena: &A, morsel: MorselRange);
}

impl<A> RunFiber<A, Empty> for WuNil {
    #[inline]
    fn run(&self, _arena: &A, _morsel: MorselRange) {}
}

impl<A, W, Tail, RIdx, WTail> RunFiber<A, Cons<RIdx, WTail>> for WuCons<W, Tail>
where
    W: WorkUnit,
    A: Project<<W as WorkUnit>::Read, RIdx>,
    ColPtrNil: ColProject<<W as WorkUnit>::Read, Empty, Out = ColPtrNil>,
    ColPtrNil: ColProject<<W as WorkUnit>::Write, Empty, Out = ColPtrNil>,
    // Tie each unit's Ctx GAT to the projection of its Read set over the
    // shared arena, for all frame lifetimes. The right side is a projected
    // associated type, lifetime-independent, so the equality resolves
    // against the unit's own `type Ctx<'frame> = EngineCtx<'frame, ...>`.
    // The `ColPtrNil` column projections express the resource-only
    // boundary: a unit reading or writing a column has a non-empty column
    // projection and does not satisfy this bound.
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
    Tail: RunFiber<A, WTail>,
{
    #[inline]
    fn run(&self, arena: &A, morsel: MorselRange) {
        // Project this unit's own Context from the shared arena. The
        // turbofish pins the per-unit resource index (`RIdx`) and the empty
        // column witnesses; `EngineCtx::project` takes `arena: &'frame A`,
        // so the projected pointers stay tied to the arena borrow.
        let ctx: <W as WorkUnit>::Ctx<'_> =
            EngineCtx::project::<A, ColPtrNil, RIdx, Empty, Empty>(arena, &ColPtrNil, morsel);
        invoke_wu_in_fiber(&self.head, &ctx);
        self.tail.run(arena, morsel);
    }
}

/// Drive a fiber's WorkUnit sequence over a shared arena and morsel.
///
/// The per-unit witness list infers at the call site, so the caller writes
/// no turbofish.
///
/// The walk is resource-only this slice. A unit that reads or writes a
/// column has a non-`ColPtrNil` column projection in its `Ctx`, so it does
/// not satisfy the walk's bound and cannot enter the sequence. The
/// following does not compile, because the column-writing unit's `Ctx`
/// carries `ColPtrCons` for its write columns where the walk requires
/// `ColPtrNil`:
///
/// ```compile_fail
/// use arvo::strategy::Identity;
/// use arvo::USize;
/// use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, PtrNil};
/// use hilavitkutin::dispatch::fiber_walk::{run_fiber_walk, WuCons, WuNil};
/// use hilavitkutin::dispatch::morsel::MorselRange;
/// use hilavitkutin_api::access::{Cons, Empty};
/// use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
/// use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
/// use hilavitkutin_api::store::Column;
/// use hilavitkutin_api::work_unit::{Always, WorkUnit};
///
/// type WriteCol = Cons<Column<u32>, Empty>;
///
/// struct ColWu;
/// impl BuilderInput for ColWu {
///     type Init = Self;
///     type Dispatch = UnitDispatch<Self>;
/// }
/// impl WorkUnit<Always> for ColWu {
///     type Read = Empty;
///     type Write = WriteCol;
///     type Hint = (Immediate, Atomic, Normal);
///     type Ctx<'frame> =
///         EngineCtx<'frame, Empty, WriteCol, PtrNil, ColPtrNil, ColPtrCons<u32, ColPtrNil>>;
///     fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {}
/// }
///
/// let fiber = WuCons { head: ColWu, tail: WuNil };
/// run_fiber_walk(&fiber, &PtrNil, MorselRange::new(USize::ZERO, USize::ZERO));
/// ```
#[inline]
pub fn run_fiber_walk<F, A, Witnesses>(fiber: &F, arena: &A, morsel: MorselRange)
where
    F: RunFiber<A, Witnesses>,
{
    fiber.run(arena, morsel);
}

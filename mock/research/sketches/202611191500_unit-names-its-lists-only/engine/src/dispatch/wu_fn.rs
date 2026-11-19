//! Monomorphised WU function-pointer shape + per-WU shim emission
//! (domain 17, Topic 4 axis C).
//!
//! Two layers:
//!
//! 1. **`WuFn<Ctx>`** function-pointer alias matching
//!    `WorkUnit::execute(&self, &Ctx)` with the `&self` receiver
//!    closed over at monomorphisation time. Stored in dispatch
//!    records so the codegen can hand a uniform shape to the
//!    morsel loop.
//!
//! 2. **`invoke_wu_in_fiber<W>`** the `#[inline(always)]` shim
//!    helper. Per Topic 4 axis C, codegen emits one
//!    `invoke_<wu>_in_fiber_<n>(ctx) { wu.execute(ctx) }` per WU
//!    per fiber occurrence. LLVM elides every shim, so consumer
//!    WU authors carry no inlining obligation in their `execute`
//!    impls. This generic helper is the substrate the codegen
//!    layer instantiates per occurrence; the per-occurrence
//!    monomorphisation flows out of LLVM.
//!
//! The full per-fiber walk + morsel loop + arena progress wiring
//! lands in `fiber_dispatch::run_fiber` (CHANGE 3) and the morsel
//! / sync CHANGE blocks below.

use hilavitkutin_api::context::ResolveMap;
use hilavitkutin_api::{HasSchedule, Positional, ReadSet, RecordOp, WorkUnit, WriteSet};

/// Function-pointer shape used by dispatch records.
pub type WuFn<Ctx> = fn(&Ctx);

/// Per-WU shim. Codegen emits one specialised copy per fiber
/// occurrence; LLVM elides each shim so consumer `WorkUnit::execute`
/// impls do not need their own `#[inline]` discipline.
///
/// Generic over the WU's recovered schedule (`Always` or `On<V>`), so a mixed
/// carrier invokes either kind through the same shim (E4 slice 1).
///
/// The one place a unit body runs, under every entry and on every thread, so
/// it is where an unwind out of the body is stopped: `execute` is called
/// through [`execute_without_unwinding`], and a panic in it aborts the process.
#[inline(always)]
pub fn invoke_wu_in_fiber<'frame, W>(
    wu: &W,
    ctx: &<W as Positional<<W as HasSchedule>::Sched>>::Ctx<'frame>,
) where
    W: HasSchedule + Positional<<W as HasSchedule>::Sched>,
{
    execute_without_unwinding::<W>(wu, ctx);
}

/// Sketch edit (202611191500): the shim for a unit whose body is its value
/// function. The unit names no context; the engine's own context, built from
/// the unit's lists, applies the function at each record of the morsel.
///
/// `C` is the engine's context for the unit's lists and nothing else: the one
/// caller is the value arm of `RunFiber`, which builds it by `project`.
#[inline(always)]
pub fn invoke_value_in_fiber<W, C>(wu: &W, ctx: &C)
where
    W: HasSchedule + RecordOp<<W as HasSchedule>::Sched>,
    <W as WorkUnit<<W as HasSchedule>::Sched>>::Read: ReadSet,
    <W as WorkUnit<<W as HasSchedule>::Sched>>::Write: WriteSet,
    C: ResolveMap<
            <W as WorkUnit<<W as HasSchedule>::Sched>>::Read,
            <W as WorkUnit<<W as HasSchedule>::Sched>>::Write,
        >,
{
    apply_without_unwinding::<W, C>(wu, ctx);
}

/// The value function of a unit behind the boundary `execute_without_unwinding`
/// keeps for a positional body.
#[inline(always)]
extern "C" fn apply_without_unwinding<W, C>(wu: &W, ctx: &C)
where
    W: HasSchedule + RecordOp<<W as HasSchedule>::Sched>,
    <W as WorkUnit<<W as HasSchedule>::Sched>>::Read: ReadSet,
    <W as WorkUnit<<W as HasSchedule>::Sched>>::Write: WriteSet,
    C: ResolveMap<
            <W as WorkUnit<<W as HasSchedule>::Sched>>::Read,
            <W as WorkUnit<<W as HasSchedule>::Sched>>::Write,
        >,
{
    ctx.resolve_map(|r| wu.apply(r));
}

/// Run a unit body behind a boundary an unwind cannot cross.
///
/// An unwind reaching an `extern "C"` function aborts, whatever the build's
/// panic strategy. The engine owns no thread and cannot catch, so a body that
/// unwound on a `run_parallel` worker would end the executor's thread before
/// the engine's worker reached the frame's done barrier, and the frame would
/// never return.
#[inline(always)]
extern "C" fn execute_without_unwinding<'frame, W>(
    wu: &W,
    ctx: &<W as Positional<<W as HasSchedule>::Sched>>::Ctx<'frame>,
) where
    W: HasSchedule + Positional<<W as HasSchedule>::Sched>,
{
    wu.execute(ctx);
}

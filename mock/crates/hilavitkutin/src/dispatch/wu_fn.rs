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

use hilavitkutin_api::{HasSchedule, WorkUnit};

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
    ctx: &<W as WorkUnit<<W as HasSchedule>::Sched>>::Ctx<'frame>,
) where
    W: HasSchedule + WorkUnit<<W as HasSchedule>::Sched>,
{
    execute_without_unwinding::<W>(wu, ctx);
}

/// Run a unit body behind a boundary an unwind cannot cross.
///
/// An unwind reaching an `extern "C"` function aborts, whatever the build's
/// panic strategy. The engine owns no thread and cannot catch, so a body that
/// unwound on a `run_parallel` worker would end the executor's thread before
/// the engine's worker reached the frame's done barrier, and the frame would
/// never return. Under `panic = "abort"` the boundary costs nothing; under
/// unwinding it is one cold landing pad per inlined call.
#[inline(always)]
extern "C" fn execute_without_unwinding<'frame, W>(
    wu: &W,
    ctx: &<W as WorkUnit<<W as HasSchedule>::Sched>>::Ctx<'frame>,
) where
    W: HasSchedule + WorkUnit<<W as HasSchedule>::Sched>,
{
    wu.execute(ctx);
}

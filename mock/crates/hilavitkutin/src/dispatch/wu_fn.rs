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

use hilavitkutin_api::WorkUnit;

/// Function-pointer shape used by dispatch records.
pub type WuFn<Ctx> = fn(&Ctx);

/// Per-WU shim. Codegen emits one specialised copy per fiber
/// occurrence; LLVM elides each shim so consumer `WorkUnit::execute`
/// impls do not need their own `#[inline]` discipline.
#[inline(always)]
pub fn invoke_wu_in_fiber<'frame, W>(wu: &W, ctx: &W::Ctx<'frame>)
where
    W: WorkUnit,
{
    wu.execute(ctx);
}

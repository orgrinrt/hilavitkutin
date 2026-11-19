//! Each unit's body, instantiated over the engine's context.
//!
//! One unmangled, never-inlined function per body, so the release build's
//! assembly holds each as a symbol of its own. `value_*` is a unit of `unit`
//! through `invoke_value_in_fiber`, the call the engine's walk makes for a
//! unit whose body is its value function. `native_*` is the same unit with a
//! positional body, through `Positional::execute`. `control_indirect` calls
//! through a function pointer, so the search for indirect calls has a line it
//! must find in the same file.

#![no_std]

use hilavitkutin::dispatch::engine_ctx::CtxFor;
use hilavitkutin::dispatch::wu_fn::invoke_value_in_fiber;
use hilavitkutin_api::work_unit::{Positional, WorkUnit};
use native::{NDrift, NWarm};
use unit::{Drift, Warm};

type Cx<'f, U> = CtxFor<'f, <U as WorkUnit>::Read, <U as WorkUnit>::Write>;

#[unsafe(no_mangle)]
#[inline(never)]
pub fn probe_value_drift(ctx: &Cx<'_, Drift>) {
    invoke_value_in_fiber(&Drift, ctx);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn probe_value_warm(ctx: &Cx<'_, Warm>) {
    invoke_value_in_fiber(&Warm, ctx);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn probe_native_drift(ctx: &<NDrift as Positional>::Ctx<'_>) {
    NDrift.execute(ctx);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn probe_native_warm(ctx: &<NWarm as Positional>::Ctx<'_>) {
    NWarm.execute(ctx);
}

/// The control: a call the search must find.
#[unsafe(no_mangle)]
#[inline(never)]
pub fn probe_control_indirect(f: fn(i64) -> i64, x: i64) -> i64 {
    f(x)
}

//! Each unit's body, instantiated over the engine's context.
//!
//! One unmangled, never-inlined function per body, so the release build's
//! assembly holds each as a symbol of its own. `block_*` is the block-form
//! body handed the engine's context directly. `blk_*` is the same body
//! through `Blk::execute`, which is the call the scheduler makes. `native_*`
//! is the control: the same unit as the engine takes it today.

#![no_std]

use hilavitkutin::dispatch::engine_ctx::{Blk, CtxFor};
use hilavitkutin_api::block::Block;
use hilavitkutin_api::work_unit::WorkUnit;
use native::{NDrift, NStamp, NWarm};
use unit::{Drift, Stamp, Warm};

type Cx<'f, U> = CtxFor<'f, <U as Block>::Read, <U as Block>::Write>;

#[unsafe(no_mangle)]
#[inline(never)]
pub fn probe_block_stamp(ctx: &Cx<'_, Stamp>) {
    Stamp.run(ctx);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn probe_block_drift(ctx: &Cx<'_, Drift>) {
    Drift.run(ctx);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn probe_block_warm(ctx: &Cx<'_, Warm>) {
    Warm.run(ctx);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn probe_blk_stamp(ctx: &<Blk<Stamp> as WorkUnit>::Ctx<'_>) {
    Blk(Stamp).execute(ctx);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn probe_blk_drift(ctx: &<Blk<Drift> as WorkUnit>::Ctx<'_>) {
    Blk(Drift).execute(ctx);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn probe_blk_warm(ctx: &<Blk<Warm> as WorkUnit>::Ctx<'_>) {
    Blk(Warm).execute(ctx);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn probe_native_stamp(ctx: &<NStamp as WorkUnit>::Ctx<'_>) {
    NStamp.execute(ctx);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn probe_native_drift(ctx: &<NDrift as WorkUnit>::Ctx<'_>) {
    NDrift.execute(ctx);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn probe_native_warm(ctx: &<NWarm as WorkUnit>::Ctx<'_>) {
    NWarm.execute(ctx);
}

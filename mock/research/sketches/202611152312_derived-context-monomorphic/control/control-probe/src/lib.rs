//! The first sketch's bodies, instantiated as that sketch runs them.
//!
//! `control_body_*` is the unit's body handed the first sketch's derived
//! context `Cx`, which is the one type that body is ever instantiated over.
//! `control_blk_*` is the same body through `bridge::Blk::execute`, the call
//! the scheduler makes, where the thunk tables are built on the frame.

#![no_std]

use bridge::Blk;
use derived::{Block, Cx};
use hilavitkutin::dispatch::engine_ctx::CtxFor;
use hilavitkutin_api::work_unit::WorkUnit;
use unit::{Drift, Stamp, Warm};

type DCx<'c, U> = Cx<'c, <U as Block>::Read, <U as Block>::Write>;
type ECx<'f, U> = CtxFor<'f, <U as Block>::Read, <U as Block>::Write>;

#[unsafe(no_mangle)]
#[inline(never)]
pub fn control_body_stamp(cx: &DCx<'_, Stamp>) {
    Stamp.run(cx);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn control_body_drift(cx: &DCx<'_, Drift>) {
    Drift.run(cx);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn control_body_warm(cx: &DCx<'_, Warm>) {
    Warm.run(cx);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn control_blk_stamp(ctx: &ECx<'_, Stamp>) {
    WorkUnit::execute(&Blk::<Stamp, _, _>::new(Stamp), ctx);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn control_blk_drift(ctx: &ECx<'_, Drift>) {
    WorkUnit::execute(&Blk::<Drift, _, _>::new(Drift), ctx);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn control_blk_warm(ctx: &ECx<'_, Warm>) {
    WorkUnit::execute(&Blk::<Warm, _, _>::new(Warm), ctx);
}

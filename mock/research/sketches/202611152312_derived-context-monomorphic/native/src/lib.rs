//! The controls: the three units of `unit` as the engine takes them today.
//!
//! Each is a `WorkUnit` naming `CtxFor`, its body on the engine's keyed
//! accessors, over the same columns as its block-form twin. The bodies are
//! those of the first sketch's run test.

#![no_std]

use hilavitkutin::dispatch::engine_ctx::CtxFor;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi,
    ColumnWriterApi,
    EachApi,
    HasColumnReader,
    HasColumnWriter,
    HasEach,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::member::Written;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::WorkUnit;
use unit::{Glow, Pos, Seen, Vel};

type Hint = (Immediate, Atomic, Normal);

/// `unit::Stamp`, natively.
#[derive(Copy, Clone)]
pub struct NStamp;
impl BuilderInput for NStamp {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit for NStamp {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;
    type Hint = Hint;
    type Read = Empty;
    type Write = Cons<Column<Seen>, Empty>;

    #[inline]
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each()
            .run(|i| unsafe { ctx.writer().write::<Seen, _>(i, Seen(i.0 as i64)) });
    }
}

/// `unit::Drift`, natively.
#[derive(Copy, Clone)]
pub struct NDrift;
impl BuilderInput for NDrift {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit for NDrift {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;
    type Hint = Hint;
    type Read = Cons<Column<Pos>, Cons<Column<Vel>, Empty>>;
    type Write = Cons<Column<Pos>, Empty>;

    #[inline]
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| unsafe {
            let pos = ctx.reader().read::<Pos, _>(i);
            let vel = ctx.reader().read::<Vel, _>(i);
            ctx.writer().write::<Pos, _>(i, Pos(pos.0 + vel.0));
        });
    }
}

/// `unit::Warm`, natively.
#[derive(Copy, Clone)]
pub struct NWarm;
impl BuilderInput for NWarm {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit for NWarm {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;
    type Hint = Hint;
    type Read = Cons<Column<Pos>, Empty>;
    type Write = Cons<Written<Column<Glow>>, Empty>;

    #[inline]
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| unsafe {
            let pos = ctx.reader().read::<Pos, _>(i);
            ctx.writer().write::<Glow, _>(i, Glow(pos.0 + pos.0));
        });
    }
}

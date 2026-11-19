//! The controls: units with a positional body, naming the engine's context.
//!
//! `NDrift` and `NWarm` are `unit::Drift` and `unit::Warm` over the same
//! columns, each a declaration (`WorkUnit`) and a body handed the context
//! (`Positional`, naming `CtxFor`), on the engine's keyed accessors. `NStamp`
//! has no twin in `unit`: it writes each record's index inside its window,
//! which a value function is not handed. It is registered beside the units of
//! either kind, so the run can tell that a frame was cut into more than one
//! window.

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
use hilavitkutin_api::work_unit::{Positional, WorkUnit};
use unit::{Glow, Pos, Seen, Vel};

type Hint = (Immediate, Atomic, Normal);

/// Writes each record's index inside its window.
#[derive(Copy, Clone)]
pub struct NStamp;
impl BuilderInput for NStamp {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit for NStamp {
    type Hint = Hint;
    type Read = Empty;
    type Write = Cons<Column<Seen>, Empty>;
}
impl Positional for NStamp {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;

    #[inline]
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each()
            .run(|i| unsafe { ctx.writer().write::<Seen, _>(i, Seen(i.0 as i64)) });
    }
}

/// `unit::Drift`, with a positional body.
#[derive(Copy, Clone)]
pub struct NDrift;
impl BuilderInput for NDrift {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit for NDrift {
    type Hint = Hint;
    type Read = Cons<Column<Pos>, Cons<Column<Vel>, Empty>>;
    type Write = Cons<Column<Pos>, Empty>;
}
impl Positional for NDrift {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;

    #[inline]
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| unsafe {
            let pos = ctx.reader().read::<Pos, _>(i);
            let vel = ctx.reader().read::<Vel, _>(i);
            ctx.writer().write::<Pos, _>(i, Pos(pos.0 + vel.0));
        });
    }
}

/// `unit::Warm`, with a positional body.
#[derive(Copy, Clone)]
pub struct NWarm;
impl BuilderInput for NWarm {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit for NWarm {
    type Hint = Hint;
    type Read = Cons<Column<Pos>, Empty>;
    type Write = Cons<Written<Column<Glow>>, Empty>;
}
impl Positional for NWarm {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;

    #[inline]
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| unsafe {
            let pos = ctx.reader().read::<Pos, _>(i);
            ctx.writer().write::<Glow, _>(i, Glow(pos.0 + pos.0));
        });
    }
}

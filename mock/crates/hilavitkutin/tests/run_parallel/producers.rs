//! The two column-disjoint producers the fan-in, three-trunk and participation
//! fixtures put in phase 0: `ProducerA` writes `Av = In * 10`, `ProducerB`
//! writes `Bv = In * 100`.

use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
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
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};

use super::arena::{HintT, Inv, OneIn};

#[derive(Copy, Clone, Default)]
pub struct Av(pub u32);
#[derive(Copy, Clone, Default)]
pub struct Bv(pub u32);

type ColA = Cons<Column<Av>, Empty>;
type ColB = Cons<Column<Bv>, Empty>;

pub struct ProducerA;
impl BuilderInput for ProducerA {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for ProducerA {
    type Ctx<'frame> = EngineCtx<
        'frame,
        OneIn,
        ColA,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<Av, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = OneIn;
    type Write = ColA;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated for every reserved record; Av reserved and
            // exclusive; the reader and writer window the morsel to absolute.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Av, _>(i, Av(inp.0 * 10)) };
        });
    }
}

pub struct ProducerB;
impl BuilderInput for ProducerB {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for ProducerB {
    type Ctx<'frame> = EngineCtx<
        'frame,
        OneIn,
        ColB,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<Bv, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = OneIn;
    type Write = ColB;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: as ProducerA, for Bv.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Bv, _>(i, Bv(inp.0 * 100)) };
        });
    }
}

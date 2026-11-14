//! A third column-disjoint producer, `ProducerC`, writing `Cv = In * 1000`. The
//! three-trunk and participation fixtures need more trunks in phase 0 than the
//! fan-in does.

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
pub struct Cv(pub u32);

type ColC = Cons<Column<Cv>, Empty>;

pub struct ProducerC;
impl BuilderInput for ProducerC {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for ProducerC {
    type Ctx<'frame> = EngineCtx<
        'frame,
        OneIn,
        ColC,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<Cv, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = OneIn;
    type Write = ColC;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated; Cv reserved and exclusive; windowed.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Cv, _>(i, Cv(inp.0 * 1000)) };
        });
    }
}

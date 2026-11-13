// A unit declares `Read = Column<Ca>` and `Write = Column<Cb>` but its body
// reads `Column<Cb>` through the reader. The context refuses a read of a store
// the access set does not declare as read. The twin `undeclared_read_pass.rs`
// differs only in the column named in the `read::<..>` call (`Ca`).

use arvo::USize;
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

#[derive(Copy, Clone)]
struct Ca(u32);
#[derive(Copy, Clone)]
struct Cb(u32);

type ReadCa = Cons<Column<Ca>, Empty>;
type WriteCb = Cons<Column<Cb>, Empty>;

struct Copies;

impl BuilderInput for Copies {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for Copies {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadCa,
        WriteCb,
        SnapNil,
        ColPtrCons<Ca, ColPtrNil>,
        ColPtrCons<Cb, ColPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ReadCa;
    type Write = WriteCb;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i: USize| {
            let v = unsafe { ctx.reader().read::<Cb, _>(i) };
            unsafe { ctx.writer().write::<Cb, _>(i, Cb(v.0)) };
        });
    }
}

fn main() {
    let _ = Copies;
}

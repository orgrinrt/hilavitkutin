// Attempt 1: the body generic over the context, bounded on the accessor
// traits the api ships (`HasColumnReader`, `HasColumnWriter`, `HasEach`), so
// the unit names no context type. Names the api only.
//
// The keyed read `read::<T, I>` asks `Self: ResolveColumnRead<T, I>` of the
// provider, with the witness `I` inferred where the provider is a concrete
// bundle. Inside a body generic over `C` it is not concrete, and nothing in
// the bound says it resolves `Pos`.

use hilavitkutin_api::access::{AccessSet, Cons, Empty};
use hilavitkutin_api::context::{
    ColumnReaderApi,
    ColumnWriterApi,
    EachApi,
    HasColumnReader,
    HasColumnWriter,
    HasEach,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal, SchedulingHint};
use hilavitkutin_api::store::Column;
use unit::{Pos, Vel};

pub trait GenericBlock {
    type Read: AccessSet;
    type Write: AccessSet;
    type Hint: SchedulingHint;

    fn run<C>(&self, ctx: &C)
    where
        C: HasColumnReader<Self::Read>
            + HasColumnWriter<Self::Write>
            + HasEach<Self::Read, Self::Write>;
}

pub struct Drift;

impl GenericBlock for Drift {
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Pos>, Cons<Column<Vel>, Empty>>;
    type Write = Cons<Column<Pos>, Empty>;

    fn run<C>(&self, ctx: &C)
    where
        C: HasColumnReader<Self::Read>
            + HasColumnWriter<Self::Write>
            + HasEach<Self::Read, Self::Write>,
    {
        ctx.each().run(|i| unsafe {
            let pos = ctx.reader().read::<Pos, _>(i);
            let vel = ctx.reader().read::<Vel, _>(i);
            ctx.writer().write::<Pos, _>(i, Pos(pos.0 + vel.0));
        });
    }
}

fn main() {}

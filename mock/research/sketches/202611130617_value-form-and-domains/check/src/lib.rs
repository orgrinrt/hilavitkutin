//! The composing crate: the store list, a seeded world, and one block-form unit.
//!
//! The block-form unit lives here and not with the author's units, because it
//! names the engine's context and the author's crate does not depend on the
//! engine.

use api::{
    Accum,
    BuilderInput,
    Column,
    Cons,
    Empty,
    Extent,
    PCons,
    PNil,
    Records,
    UnitDispatch,
    Virtual,
    WorkUnit,
};
use author::{
    ALARM,
    Alarm,
    CROSSED,
    Crossed,
    HERE,
    Heat,
    HeatAt,
    POS,
    Plane,
    Pos,
    RATE,
    Rate,
    VEL,
    Vel,
};
use engine::EngineCtx;
use plumb::World;

/// Every store of the test world, two domains among them.
pub type Stores = Cons<
    Extent<Plane>,
    Cons<
        Extent<Records>,
        Cons<
            HeatAt,
            Cons<
                Rate,
                Cons<
                    Column<Pos>,
                    Cons<Column<Vel>, Cons<Accum<Crossed>, Cons<Virtual<Alarm>, Empty>>>,
                >,
            >,
        >,
    >,
>;

/// A world of a `north` by `east` plane and `records` records of the default
/// domain, filled from the record index.
pub fn seeded(north: usize, east: usize, records: usize) -> World<Stores> {
    let mut world = World::<Stores>::new();
    world.size::<Plane, _>(PCons {
        head: north,
        tail: PCons {
            head: east,
            tail: PNil,
        },
    });
    world.size::<Records, _>(PCons {
        head: records,
        tail: PNil,
    });
    world.fill::<_, Plane, _, _>(HERE, |i| Heat((i as i64 * 7) % 11 - 3));
    world.put(RATE, 2);
    world.fill::<_, Records, _, _>(POS, |i| Pos(90 + i as i64));
    world.fill::<_, Records, _, _>(VEL, |i| Vel(i as i64 % 4));
    let _ = (CROSSED, ALARM);
    world
}

/// A block-form unit: counts the morsels it is handed. It names `EngineCtx`.
pub struct Tally;

impl BuilderInput for Tally {
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit for Tally {
    type Ctx<'frame> = EngineCtx<'frame>;
    type Read = Empty;
    type Write = Empty;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.tally.set(ctx.tally.get() + 1);
    }
}

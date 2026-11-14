//! Stand-in for the scheduler: walks a list of units of either form, in the
//! order they were given, one record at a time from the first.
//!
//! `Runs` is this crate's own trait, implemented for the api's two routers. A
//! unit says which form it is in by the router it names, so the two impls never
//! meet and no unit names anything from here unless it is in the block form.

use core::cell::Cell;

use api::{
    BuilderInput,
    DomainOf,
    Extent,
    Has,
    Place,
    Project,
    RecordDispatch,
    RecordOp,
    UnitDispatch,
    Vals,
    WorkUnit,
};
use plumb::{Close, Gather, Scatter, World};

/// The context a block-form unit names. The sketch hands it a tally only: what
/// a block body reaches is ground the sketches before this one cover.
pub struct EngineCtx<'frame> {
    pub tally: &'frame Cell<u64>,
}

/// How this runner runs a unit routed by `Self`.
pub trait Runs<W, S: Project, Ps> {
    fn run(unit: &W, world: &mut World<S>, tally: &Cell<u64>);
}

impl<U, S, Pr, Pw, Pe> Runs<U, S, (Pr, Pw, Pe)> for RecordDispatch<U>
where
    U: RecordOp,
    S: Project + Has<Extent<DomainOf<U>>, Pe>,
    Pe: Place,
    U::Read: Gather<S, Pr>,
    U::Write: Scatter<S, Pw>,
{
    fn run(unit: &U, world: &mut World<S>, _: &Cell<u64>) {
        let count = world.count::<DomainOf<U>, Pe>();
        for i in 0 .. count {
            let read = Vals(<U::Read as Gather<S, Pr>>::gather(&world.lanes, i));
            let write = unit.apply(&read);
            <U::Write as Scatter<S, Pw>>::scatter(&mut world.lanes, i, &write.0);
        }
    }
}

impl<W, S: Project> Runs<W, S, ()> for UnitDispatch<W>
where
    W: for<'frame> WorkUnit<Ctx<'frame> = EngineCtx<'frame>>,
{
    fn run(unit: &W, _: &mut World<S>, tally: &Cell<u64>) {
        unit.execute(&EngineCtx {
            tally,
        });
    }
}

/// A list of unit values, each run by the router it names.
pub trait Walk<S: Project, Ps> {
    fn walk(&self, world: &mut World<S>, tally: &Cell<u64>);
}

impl<S: Project> Walk<S, ()> for () {
    fn walk(&self, _: &mut World<S>, _: &Cell<u64>) {}
}

impl<W, Tl, S, P, Ps> Walk<S, (P, Ps)> for (W, Tl)
where
    S: Project,
    W: BuilderInput,
    W::Dispatch: Runs<W, S, P>,
    Tl: Walk<S, Ps>,
{
    fn walk(&self, world: &mut World<S>, tally: &Cell<u64>) {
        <W::Dispatch as Runs<W, S, P>>::run(&self.0, world, tally);
        self.1.walk(world, tally);
    }
}

/// One pass: every unit, then the close.
pub fn pass<S, Us, Ps>(units: &Us, world: &mut World<S>, tally: &Cell<u64>)
where
    S: Close,
    Us: Walk<S, Ps>,
{
    units.walk(world, tally);
    S::close(&mut world.lanes);
}

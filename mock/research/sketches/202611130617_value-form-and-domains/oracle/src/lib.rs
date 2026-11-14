//! A second runner, written apart from the engine: it takes every record's
//! reads before it writes any, and goes through the records from the last.
//!
//! It runs value-form units only. A block-form unit names the engine's context
//! and so runs under the engine alone, which is why the value form is the one
//! the frozen authors are written in.

use api::{BuilderInput, DomainOf, Extent, Has, Place, Project, RecordDispatch, RecordOp, Vals};
use plumb::{Close, Gather, Scatter, World};

/// How this runner runs a unit routed by `Self`.
pub trait Checks<W, S: Project, Ps> {
    fn check(unit: &W, world: &mut World<S>);
}

impl<U, S, Pr, Pw, Pe> Checks<U, S, (Pr, Pw, Pe)> for RecordDispatch<U>
where
    U: RecordOp,
    S: Project + Has<Extent<DomainOf<U>>, Pe>,
    Pe: Place,
    U::Read: Gather<S, Pr>,
    U::Write: Scatter<S, Pw>,
{
    fn check(unit: &U, world: &mut World<S>) {
        let count = world.count::<DomainOf<U>, Pe>();
        let mut writes = Vec::with_capacity(count);
        for i in (0 .. count).rev() {
            let read = Vals(<U::Read as Gather<S, Pr>>::gather(&world.lanes, i));
            writes.push((i, unit.apply(&read)));
        }
        for (i, write) in &writes {
            <U::Write as Scatter<S, Pw>>::scatter(&mut world.lanes, *i, &write.0);
        }
    }
}

/// A list of value-form units.
pub trait Walk<S: Project, Ps> {
    fn walk(&self, world: &mut World<S>);
}

impl<S: Project> Walk<S, ()> for () {
    fn walk(&self, _: &mut World<S>) {}
}

impl<W, Tl, S, P, Ps> Walk<S, (P, Ps)> for (W, Tl)
where
    S: Project,
    W: BuilderInput,
    W::Dispatch: Checks<W, S, P>,
    Tl: Walk<S, Ps>,
{
    fn walk(&self, world: &mut World<S>) {
        <W::Dispatch as Checks<W, S, P>>::check(&self.0, world);
        self.1.walk(world);
    }
}

/// One pass: every unit, then the close.
pub fn pass<S, Us, Ps>(units: &Us, world: &mut World<S>)
where
    S: Close,
    Us: Walk<S, Ps>,
{
    units.walk(world);
    S::close(&mut world.lanes);
}

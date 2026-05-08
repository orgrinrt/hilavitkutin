//! WorkUnit trait plus the two built-in schedule markers.
//!
//! `WorkUnit<Schedule>` is the consumer's unit of work. Composition
//! is static: the engine takes a tuple of WU types at compile time.
//! Identity is the type itself; no `NAME` const, no registry.

use core::marker::PhantomData;

use arvo::Bool;

use crate::access::AccessSet;
use crate::context::{
    HasBatch, HasColumnReader, HasColumnWriter, HasEach, HasReduce, HasResourceProvider,
    HasVirtualFirer,
};
use crate::hint::SchedulingHint;

/// Schedule marker: the WU runs every pass.
#[derive(Copy, Clone, Default, Debug)]
pub struct Always;

/// Schedule marker: the WU runs when virtual `V` fires.
#[derive(Copy, Clone, Default, Debug)]
pub struct On<V>(PhantomData<V>);

/// A unit of work.
///
/// Declares its read/write access sets at type level, its
/// scheduling hint, and the provider-tuple shape it expects. The
/// engine composes WUs into fused per-core programs that LLVM
/// devirtualises into straight-line code.
///
/// `Schedule` picks the firing condition: `Always` runs every pass,
/// `On<V>` runs when virtual `V` fires.
pub trait WorkUnit<Schedule = Always>: Send + Sync + 'static {
    /// Columns / resources this WU reads.
    type Read: AccessSet;
    /// Columns / virtuals this WU writes.
    type Write: AccessSet;
    /// Scheduling hint triple. Consumer provides; no default because
    /// the implementing tuple is marker-specific.
    type Hint: SchedulingHint;
    /// Provider-tuple shape this WU's body consumes.
    ///
    /// Monomorphisation resolves `HasX<...>` bounds to the concrete
    /// provider the engine wires up at plan time.
    type Ctx: HasColumnReader<Self::Read>
        + HasColumnWriter<Self::Write>
        + HasResourceProvider<Self::Read>
        + HasVirtualFirer<Self::Write>
        + HasEach<Self::Read, Self::Write>
        + HasBatch<Self::Read, Self::Write>
        + HasReduce<Self::Read, Self::Write>;

    /// True if the WU's writes commute across record order.
    ///
    /// Enables the scheduler to emit a reduce-style pattern instead
    /// of serialising. Default `Bool::FALSE`: consumer opts in.
    const COMMUTATIVE: Bool = Bool::FALSE;

    /// Run one pass of this WU against the provided context.
    fn execute(&self, ctx: &Self::Ctx);
}

// ---------------------------------------------------------------------
// Round 4 substrate: WorkUnitBundle.
//
// Cons-list bundle of WorkUnit types with accumulated Read / Write
// access sets. AccumRead is the Concat-projected union of every WU's
// Read over the bundle; AccumWrite is the symmetric projection over
// Write. Used by Kit's 'type Units' bound; the engine reads
// Wus::AccumRead and Wus::AccumWrite at compile time.
// ---------------------------------------------------------------------

use crate::access::{Concat, Cons, Empty};

/// Cons-list bundle of WorkUnit types with accumulated Read / Write
/// access sets.
pub trait WorkUnitBundle {
    type AccumRead: AccessSet;
    type AccumWrite: AccessSet;
}

impl WorkUnitBundle for Empty {
    type AccumRead = Empty;
    type AccumWrite = Empty;
}

impl<W, T> WorkUnitBundle for Cons<W, T>
where
    W: WorkUnit,
    T: WorkUnitBundle,
    W::Read: Concat<T::AccumRead>,
    W::Write: Concat<T::AccumWrite>,
    <W::Read as Concat<T::AccumRead>>::Out: AccessSet,
    <W::Write as Concat<T::AccumWrite>>::Out: AccessSet,
{
    type AccumRead = <W::Read as Concat<T::AccumRead>>::Out;
    type AccumWrite = <W::Write as Concat<T::AccumWrite>>::Out;
}


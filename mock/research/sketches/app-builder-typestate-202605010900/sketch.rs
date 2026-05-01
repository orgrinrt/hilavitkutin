//! Sketch: SchedulerBuilder phantom-tuple type-state plus Kit preset.
//!
//! Validates the mechanism for round 202605010900 (#255).
//! Standalone, no_std-compatible (uses only core).
//!
//! Build with:
//!   rustc --crate-type=lib --edition=2024 sketch.rs --emit=metadata
//!
//! Notes:
//!   1. Single unified `Stores` tuple. Holds Resource<T>, Column<T>,
//!      Virtual<T> markers together (matches how WorkUnit::Read is
//!      shaped in hilavitkutin-api).
//!   2. .build() requires Wus: Buildable<Stores>; Buildable per-arity
//!      blanket impls reduce to Stores: WuSatisfied<Wn::Read> +
//!      WuSatisfied<Wn::Write> for every Wn in Wus.
//!   3. WuSatisfied<A> per-arity reduces to Self: Contains<T> for every
//!      T in A (via overlapping #[marker] impls on Contains).
//!   4. Kit composes via install method; the Kit's signature ties the
//!      input builder type to the output builder type.

#![no_std]
#![feature(marker_trait_attr)]
#![allow(dead_code, unused)]

use core::marker::PhantomData;

// -----------------------------------------------------------------------
// Mock substrate.
// -----------------------------------------------------------------------

type USize = usize;

// -----------------------------------------------------------------------
// AccessSet + Contains, mirroring hilavitkutin-api/src/access.rs.
// Sealed; arities 0..=4 for the sketch.
// -----------------------------------------------------------------------

mod sealed {
    pub trait Sealed {}
}

#[allow(private_bounds)]
pub trait AccessSet: sealed::Sealed + 'static {
    const LEN: USize;
}

#[marker]
pub trait Contains<S>: AccessSet {}

impl sealed::Sealed for () {}
impl AccessSet for () {
    const LEN: USize = 0;
}

impl<T0: 'static> sealed::Sealed for (T0,) {}
impl<T0: 'static> AccessSet for (T0,) {
    const LEN: USize = 1;
}
impl<T0: 'static> Contains<T0> for (T0,) {}

impl<T0: 'static, T1: 'static> sealed::Sealed for (T0, T1) {}
impl<T0: 'static, T1: 'static> AccessSet for (T0, T1) {
    const LEN: USize = 2;
}
impl<T0: 'static, T1: 'static> Contains<T0> for (T0, T1) {}
impl<T0: 'static, T1: 'static> Contains<T1> for (T0, T1) {}

impl<T0: 'static, T1: 'static, T2: 'static> sealed::Sealed for (T0, T1, T2) {}
impl<T0: 'static, T1: 'static, T2: 'static> AccessSet for (T0, T1, T2) {
    const LEN: USize = 3;
}
impl<T0: 'static, T1: 'static, T2: 'static> Contains<T0> for (T0, T1, T2) {}
impl<T0: 'static, T1: 'static, T2: 'static> Contains<T1> for (T0, T1, T2) {}
impl<T0: 'static, T1: 'static, T2: 'static> Contains<T2> for (T0, T1, T2) {}

impl<T0: 'static, T1: 'static, T2: 'static, T3: 'static> sealed::Sealed for (T0, T1, T2, T3) {}
impl<T0: 'static, T1: 'static, T2: 'static, T3: 'static> AccessSet for (T0, T1, T2, T3) {
    const LEN: USize = 4;
}
impl<T0: 'static, T1: 'static, T2: 'static, T3: 'static> Contains<T0> for (T0, T1, T2, T3) {}
impl<T0: 'static, T1: 'static, T2: 'static, T3: 'static> Contains<T1> for (T0, T1, T2, T3) {}
impl<T0: 'static, T1: 'static, T2: 'static, T3: 'static> Contains<T2> for (T0, T1, T2, T3) {}
impl<T0: 'static, T1: 'static, T2: 'static, T3: 'static> Contains<T3> for (T0, T1, T2, T3) {}

// Cons-list recursion: the builder accumulates Stores as (Head, Rest)
// where Rest is itself a cons-list. The arity-2 Contains<T0> impl above
// covers head matches. This recursive impl propagates membership down
// the tail. #[marker] allows the overlap with the arity-2 impl.
impl<H: 'static, R: 'static, T: 'static> Contains<T> for (H, R) where R: Contains<T> {}

// -----------------------------------------------------------------------
// Store markers, mirroring hilavitkutin-api/src/store.rs.
// -----------------------------------------------------------------------

#[repr(transparent)]
pub struct Resource<T>(PhantomData<T>);
impl<T> Copy for Resource<T> {}
impl<T> Clone for Resource<T> {
    fn clone(&self) -> Self { *self }
}

#[repr(transparent)]
pub struct Column<T>(PhantomData<T>);
impl<T> Copy for Column<T> {}
impl<T> Clone for Column<T> {
    fn clone(&self) -> Self { *self }
}

#[repr(transparent)]
pub struct Virtual<T>(PhantomData<T>);
impl<T> Copy for Virtual<T> {}
impl<T> Clone for Virtual<T> {
    fn clone(&self) -> Self { *self }
}

// -----------------------------------------------------------------------
// WorkUnit trait, simplified: only Read/Write AccessSets (no Ctx).
// The Ctx bound conjunction in real api reduces to per-store membership
// at the same level we sketch here.
// -----------------------------------------------------------------------

pub trait WorkUnit: 'static {
    type Read: AccessSet;
    type Write: AccessSet;
}

// -----------------------------------------------------------------------
// SchedulerBuilder: phantom-tuple type-state with two slots.
//   Wus    = tuple of registered WU types
//   Stores = tuple of registered store markers (Resource<T>, Column<T>,
//            Virtual<T> mixed)
// -----------------------------------------------------------------------

pub struct SchedulerBuilder<Wus, Stores> {
    _phantom: PhantomData<(Wus, Stores)>,
}

impl SchedulerBuilder<(), ()> {
    pub const fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl<Wus, Stores> SchedulerBuilder<Wus, Stores>
where
    Wus: AccessSet,
    Stores: AccessSet,
{
    pub fn add<W: WorkUnit>(self) -> SchedulerBuilder<(W, Wus), Stores>
    where
        (W, Wus): AccessSet,
    {
        SchedulerBuilder { _phantom: PhantomData }
    }

    pub fn resource<T: 'static>(self, _init: T) -> SchedulerBuilder<Wus, (Resource<T>, Stores)>
    where
        (Resource<T>, Stores): AccessSet,
    {
        SchedulerBuilder { _phantom: PhantomData }
    }

    pub fn column<T: 'static>(self) -> SchedulerBuilder<Wus, (Column<T>, Stores)>
    where
        (Column<T>, Stores): AccessSet,
    {
        SchedulerBuilder { _phantom: PhantomData }
    }

    pub fn virtual_<T: 'static>(self) -> SchedulerBuilder<Wus, (Virtual<T>, Stores)>
    where
        (Virtual<T>, Stores): AccessSet,
    {
        SchedulerBuilder { _phantom: PhantomData }
    }

    pub fn add_kit<K: Kit<Self>>(self, k: K) -> K::Output {
        k.install(self)
    }
}

// -----------------------------------------------------------------------
// .build() carries: Wus: Buildable<Stores>. Buildable per-arity reduces
// to Stores: WuSatisfied<Wn::Read> + WuSatisfied<Wn::Write>.
// -----------------------------------------------------------------------

mod build_sealed {
    pub trait Sealed {}
}

#[allow(private_bounds)]
pub trait Buildable<Stores: AccessSet>: build_sealed::Sealed {}

// (): trivially buildable.
impl build_sealed::Sealed for () {}
impl<Stores: AccessSet> Buildable<Stores> for () {}

// (W0, ()): require Stores satisfies W0's Read and Write.
impl<W0: WorkUnit> build_sealed::Sealed for (W0, ()) {}
impl<W0, Stores> Buildable<Stores> for (W0, ())
where
    W0: WorkUnit,
    Stores: AccessSet + WuSatisfied<W0::Read> + WuSatisfied<W0::Write>,
{
}

// (W0, (W1, ())): both.
impl<W0: WorkUnit, W1: WorkUnit> build_sealed::Sealed for (W0, (W1, ())) {}
impl<W0, W1, Stores> Buildable<Stores> for (W0, (W1, ()))
where
    W0: WorkUnit,
    W1: WorkUnit,
    Stores: AccessSet
        + WuSatisfied<W0::Read>
        + WuSatisfied<W0::Write>
        + WuSatisfied<W1::Read>
        + WuSatisfied<W1::Write>,
{
}

// (W0, (W1, (W2, ()))): three.
impl<W0: WorkUnit, W1: WorkUnit, W2: WorkUnit> build_sealed::Sealed for (W0, (W1, (W2, ()))) {}
impl<W0, W1, W2, Stores> Buildable<Stores> for (W0, (W1, (W2, ())))
where
    W0: WorkUnit,
    W1: WorkUnit,
    W2: WorkUnit,
    Stores: AccessSet
        + WuSatisfied<W0::Read>
        + WuSatisfied<W0::Write>
        + WuSatisfied<W1::Read>
        + WuSatisfied<W1::Write>
        + WuSatisfied<W2::Read>
        + WuSatisfied<W2::Write>,
{
}

// -----------------------------------------------------------------------
// WuSatisfied<A>: "All members of A are present in Self." Sealed.
//
// A = ()         : trivially satisfied.
// A = (T0,)      : Self: Contains<T0>.
// A = (T0, T1)   : Self: Contains<T0> + Contains<T1>.
// ...
// -----------------------------------------------------------------------

mod wu_sealed {
    pub trait Sealed<A> {}
}

#[allow(private_bounds)]
pub trait WuSatisfied<A: AccessSet>: wu_sealed::Sealed<A> {}

impl<S: AccessSet> wu_sealed::Sealed<()> for S {}
impl<S: AccessSet> WuSatisfied<()> for S {}

impl<S, T0: 'static> wu_sealed::Sealed<(T0,)> for S where S: Contains<T0> {}
impl<S, T0: 'static> WuSatisfied<(T0,)> for S where S: Contains<T0> + AccessSet {}

impl<S, T0: 'static, T1: 'static> wu_sealed::Sealed<(T0, T1)> for S
where
    S: Contains<T0> + Contains<T1>,
{
}
impl<S, T0: 'static, T1: 'static> WuSatisfied<(T0, T1)> for S
where
    S: Contains<T0> + Contains<T1> + AccessSet,
{
}

impl<S, T0: 'static, T1: 'static, T2: 'static> wu_sealed::Sealed<(T0, T1, T2)> for S
where
    S: Contains<T0> + Contains<T1> + Contains<T2>,
{
}
impl<S, T0: 'static, T1: 'static, T2: 'static> WuSatisfied<(T0, T1, T2)> for S
where
    S: Contains<T0> + Contains<T1> + Contains<T2> + AccessSet,
{
}

// -----------------------------------------------------------------------
// Scheduler: marker type returned by build().
// -----------------------------------------------------------------------

pub struct Scheduler;

impl<Wus, Stores> SchedulerBuilder<Wus, Stores>
where
    Wus: Buildable<Stores>,
    Stores: AccessSet,
{
    pub fn build(self) -> Scheduler {
        Scheduler
    }
}

// -----------------------------------------------------------------------
// Kit trait: method-only Bevy-style.
// -----------------------------------------------------------------------

pub trait Kit<B> {
    type Output;
    fn install(self, builder: B) -> Self::Output;
}

// -----------------------------------------------------------------------
// Concrete examples to stress the mechanism.
// -----------------------------------------------------------------------

pub struct Interner;
pub struct Workspace;
pub struct FileInfo;
pub struct Diagnostic;

pub struct InternerKit;

// InternerKit registers Resource<Interner>. Bound shape: input
// SchedulerBuilder<Wus, Stores>; output extends Stores with Resource<Interner>.
impl<Wus, Stores> Kit<SchedulerBuilder<Wus, Stores>> for InternerKit
where
    Wus: AccessSet,
    Stores: AccessSet,
    (Resource<Interner>, Stores): AccessSet,
{
    type Output = SchedulerBuilder<Wus, (Resource<Interner>, Stores)>;
    fn install(self, builder: SchedulerBuilder<Wus, Stores>) -> Self::Output {
        builder.resource(Interner)
    }
}

// A Kit that registers two stores at once: Resource<Workspace> + Column<FileInfo>.
pub struct WorkspaceKit;

impl<Wus, Stores> Kit<SchedulerBuilder<Wus, Stores>> for WorkspaceKit
where
    Wus: AccessSet,
    Stores: AccessSet,
    (Resource<Workspace>, Stores): AccessSet,
    (Column<FileInfo>, (Resource<Workspace>, Stores)): AccessSet,
{
    type Output = SchedulerBuilder<Wus, (Column<FileInfo>, (Resource<Workspace>, Stores))>;
    fn install(self, builder: SchedulerBuilder<Wus, Stores>) -> Self::Output {
        builder.resource(Workspace).column::<FileInfo>()
    }
}

// -----------------------------------------------------------------------
// WUs that exercise different shapes.
// -----------------------------------------------------------------------

pub struct ReadInterner;
impl WorkUnit for ReadInterner {
    type Read = (Resource<Interner>,);
    type Write = ();
}

pub struct DiscoverFiles;
impl WorkUnit for DiscoverFiles {
    type Read = (Resource<Workspace>,);
    type Write = (Column<FileInfo>,);
}

pub struct EmitDiagnostics;
impl WorkUnit for EmitDiagnostics {
    type Read = (Column<Diagnostic>, Resource<Interner>);
    type Write = ();
}

// -----------------------------------------------------------------------
// Smoke tests: positive cases must compile.
// -----------------------------------------------------------------------

pub fn smoke_kit_only() -> Scheduler {
    SchedulerBuilder::new()
        .add_kit(InternerKit)
        .add::<ReadInterner>()
        .build()
}

pub fn smoke_two_kits_chained() -> Scheduler {
    SchedulerBuilder::new()
        .add_kit(InternerKit)
        .add_kit(WorkspaceKit)
        .add::<ReadInterner>()
        .add::<DiscoverFiles>()
        .build()
}

pub fn smoke_mixed_kit_and_raw() -> Scheduler {
    SchedulerBuilder::new()
        .add_kit(WorkspaceKit) // adds Workspace + FileInfo
        .resource(Interner) // raw resource, no Kit
        .column::<Diagnostic>() // raw column
        .add::<DiscoverFiles>()
        .add::<EmitDiagnostics>()
        .build()
}

// -----------------------------------------------------------------------
// Negative case: the WU declares Read = (Resource<Interner>,) but no
// Resource<Interner> was registered. Uncomment to confirm compile error.
// -----------------------------------------------------------------------

// pub fn smoke_fail_missing_interner() -> Scheduler {
//     SchedulerBuilder::new()
//         .add::<ReadInterner>()
//         .build()
// }
//
// Verified compile-fail (uncomment to reproduce):
//   error[E0599]: the method `build` exists for struct
//     `SchedulerBuilder<(ReadInterner, ()), ()>`, but its trait bounds
//     were not satisfied
//   note: trait bound `(): Contains<Resource<Interner>>` was not satisfied
//
// The error points the consumer directly at the missing registration.

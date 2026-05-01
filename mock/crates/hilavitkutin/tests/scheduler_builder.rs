//! SchedulerBuilder type-state tests.
//!
//! Two layers:
//!
//! 1. Smoke tests with `Wus = ()` exercising store accumulation
//!    (raw resource / column / virtual registration, Kit install,
//!    Kit chaining, mixed Kit + raw). `.build()` reduces via the
//!    arity-0 Buildable impl.
//!
//! 2. WU-bearing tests with a `Stub + TestCtx` shim providing the
//!    seven HasX accessor traits. These exercise the load-bearing
//!    `Buildable<Stores>` -> `WuSatisfied<W::Read/Write>` ->
//!    `Contains<Tᵢ>` reduction with non-empty Wus.
//!
//! The negative case (`.add::<ReadInterner>().build()` without a
//! matching resource) is verified manually as a compile-fail; a
//! trybuild fixture is tracked in #296.

use arvo::USize;
use hilavitkutin::scheduler::{Scheduler, SchedulerBuilder};
use hilavitkutin_api::{
    AccessSet, Always, BatchApi, Column, ColumnReaderApi, ColumnValue, ColumnWriterApi, Contains,
    EachApi, HasBatch, HasColumnReader, HasColumnWriter, HasEach, HasReduce, HasResourceProvider,
    HasVirtualFirer, Atomic, Immediate, Normal, ReduceApi, Resource, ResourceProviderApi,
    Virtual, VirtualFirerApi, WorkUnit,
};
use hilavitkutin_kit::Kit;

// ---------------------------------------------------------------------
// Fake stores.
// ---------------------------------------------------------------------

pub struct Interner;
pub struct Workspace;
pub struct FileInfo;

// ---------------------------------------------------------------------
// Kits.
// ---------------------------------------------------------------------

pub struct InternerKit;

impl<
    const MAX_UNITS: usize,
    const MAX_STORES: usize,
    const MAX_LANES: usize,
    Wus,
    Stores,
> Kit<SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES, Wus, Stores>> for InternerKit
where
    Wus: AccessSet,
    Stores: AccessSet,
    (Resource<Interner>, Stores): AccessSet,
{
    type Output =
        SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES, Wus, (Resource<Interner>, Stores)>;

    fn install(
        self,
        builder: SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES, Wus, Stores>,
    ) -> Self::Output {
        builder.resource(Interner)
    }
}

pub struct WorkspaceKit;

impl<
    const MAX_UNITS: usize,
    const MAX_STORES: usize,
    const MAX_LANES: usize,
    Wus,
    Stores,
> Kit<SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES, Wus, Stores>> for WorkspaceKit
where
    Wus: AccessSet,
    Stores: AccessSet,
    (Resource<Workspace>, Stores): AccessSet,
    (Column<FileInfo>, (Resource<Workspace>, Stores)): AccessSet,
{
    type Output = SchedulerBuilder<
        MAX_UNITS,
        MAX_STORES,
        MAX_LANES,
        Wus,
        (Column<FileInfo>, (Resource<Workspace>, Stores)),
    >;

    fn install(
        self,
        builder: SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES, Wus, Stores>,
    ) -> Self::Output {
        builder.resource(Workspace).column::<FileInfo>()
    }
}

// ---------------------------------------------------------------------
// Positive smoke tests.
//
// All build with `Wus = ()` so `Buildable<Stores>` reduces
// trivially via the arity-0 impl. The Stores-accumulation path is
// exercised independently from WU declarations.
// ---------------------------------------------------------------------

#[test]
fn empty_build() {
    let _ = Scheduler::<8, 16, 4>::builder().build();
}

#[test]
fn raw_resource_registration_builds() {
    let _ = Scheduler::<8, 16, 4>::builder()
        .resource(Interner)
        .build();
}

#[test]
fn raw_column_registration_builds() {
    let _ = Scheduler::<8, 16, 4>::builder().column::<FileInfo>().build();
}

#[test]
fn kit_only_builds() {
    let _ = Scheduler::<8, 16, 4>::builder().add_kit(InternerKit).build();
}

#[test]
fn two_kits_chained_build() {
    let _ = Scheduler::<8, 16, 4>::builder()
        .add_kit(InternerKit)
        .add_kit(WorkspaceKit)
        .build();
}

#[test]
fn mixed_kit_and_raw_build() {
    let _ = Scheduler::<8, 16, 4>::builder()
        .add_kit(WorkspaceKit)
        .resource(Interner)
        .column::<FileInfo>()
        .build();
}

#[test]
fn default_scheduler_constructs() {
    let _: Scheduler<4, 8, 2> = Scheduler::default();
}

// ---------------------------------------------------------------------
// WU-bearing tests.
//
// These tests exercise the load-bearing piece of the round: the
// Buildable<Stores> → WuSatisfied<W::Read/Write> → Contains<Tᵢ>
// reduction with non-empty Wus. The Stub + TestCtx pattern below
// is the same shape used in hilavitkutin-api/tests/work_unit.rs.
// ---------------------------------------------------------------------

struct Stub;

impl<R: AccessSet> ColumnReaderApi<R> for Stub {
    unsafe fn read<T: ColumnValue>(&self, _i: USize) -> T
    where
        R: Contains<Column<T>>,
    {
        unsafe { core::mem::zeroed() }
    }
}

impl<W: AccessSet> ColumnWriterApi<W> for Stub {
    unsafe fn write<T: ColumnValue>(&self, _i: USize, _v: T)
    where
        W: Contains<Column<T>>,
    {
    }
}

impl<R: AccessSet> ResourceProviderApi<R> for Stub {
    fn resource<T: 'static>(&self) -> &T
    where
        R: Contains<Resource<T>>,
    {
        unsafe { &*(self as *const _ as *const T) }
    }
}

impl<W: AccessSet> VirtualFirerApi<W> for Stub {
    fn fire<V: 'static>(&self)
    where
        W: Contains<Virtual<V>>,
    {
    }
}

impl<R: AccessSet, W: AccessSet> EachApi<R, W> for Stub {
    fn run<F>(&self, _f: F)
    where
        F: FnMut(USize),
    {
    }
}

impl<R: AccessSet, W: AccessSet> BatchApi<R, W> for Stub {
    fn run<F>(&self, _f: F)
    where
        F: FnMut(USize, USize),
    {
    }
}

impl<R: AccessSet, W: AccessSet> ReduceApi<R, W> for Stub {
    fn run<A, F>(&self, init: A, _f: F) -> A
    where
        A: 'static,
        F: FnMut(A, USize) -> A,
    {
        init
    }
}

struct TestCtx {
    p: Stub,
}

impl<R: AccessSet> HasColumnReader<R> for TestCtx {
    type Provider = Stub;
    fn reader(&self) -> &Stub {
        &self.p
    }
}

impl<W: AccessSet> HasColumnWriter<W> for TestCtx {
    type Provider = Stub;
    fn writer(&self) -> &Stub {
        &self.p
    }
}

impl<R: AccessSet> HasResourceProvider<R> for TestCtx {
    type Provider = Stub;
    fn resources(&self) -> &Stub {
        &self.p
    }
}

impl<W: AccessSet> HasVirtualFirer<W> for TestCtx {
    type Provider = Stub;
    fn virtuals(&self) -> &Stub {
        &self.p
    }
}

impl<R: AccessSet, W: AccessSet> HasEach<R, W> for TestCtx {
    type Provider = Stub;
    fn each(&self) -> &Stub {
        &self.p
    }
}

impl<R: AccessSet, W: AccessSet> HasBatch<R, W> for TestCtx {
    type Provider = Stub;
    fn batch(&self) -> &Stub {
        &self.p
    }
}

impl<R: AccessSet, W: AccessSet> HasReduce<R, W> for TestCtx {
    type Provider = Stub;
    fn reduce(&self) -> &Stub {
        &self.p
    }
}

// A WU that reads the Interner resource. Read mentions
// `Resource<Interner>`; .build() must prove `Stores:
// Contains<Resource<Interner>>` after registration.
struct ReadInterner;

impl WorkUnit<Always> for ReadInterner {
    type Read = (Resource<Interner>,);
    type Write = ();
    type Hint = (Immediate, Atomic, Normal);
    type Ctx = TestCtx;
    fn execute(&self, _ctx: &TestCtx) {}
}

// A WU with write set: writes Column<FileInfo>. Confirms .build()
// proof handles non-empty Write tuples too.
struct DiscoverFiles;

impl WorkUnit<Always> for DiscoverFiles {
    type Read = (Resource<Workspace>,);
    type Write = (Column<FileInfo>,);
    type Hint = (Immediate, Atomic, Normal);
    type Ctx = TestCtx;
    fn execute(&self, _ctx: &TestCtx) {}
}

#[test]
fn wu_with_raw_resource_builds() {
    let _ = Scheduler::<8, 16, 4>::builder()
        .resource(Interner)
        .add::<ReadInterner>()
        .build();
}

#[test]
fn wu_with_kit_builds() {
    let _ = Scheduler::<8, 16, 4>::builder()
        .add_kit(InternerKit)
        .add::<ReadInterner>()
        .build();
}

#[test]
fn two_wus_with_two_kits_build() {
    let _ = Scheduler::<8, 16, 4>::builder()
        .add_kit(InternerKit)
        .add_kit(WorkspaceKit)
        .add::<ReadInterner>()
        .add::<DiscoverFiles>()
        .build();
}

// ---------------------------------------------------------------------
// Type-state shape verification.
//
// Asserts the Kit's `Output` type matches the documented contract:
// `InternerKit::install(builder)` returns a builder with
// `Resource<Interner>` prepended onto the previous `Stores`.
// ---------------------------------------------------------------------

#[test]
fn kit_extends_stores_type() {
    fn _type_check_only<const M: usize, const N: usize, const L: usize, W, S>(
        b: SchedulerBuilder<M, N, L, W, S>,
    ) -> SchedulerBuilder<M, N, L, W, (Resource<Interner>, S)>
    where
        W: AccessSet,
        S: AccessSet,
        (Resource<Interner>, S): AccessSet,
    {
        b.add_kit(InternerKit)
    }
}

// Negative-case verification: ReadInterner declares Read =
// (Resource<Interner>,) but no .resource(Interner) and no
// InternerKit registered. Uncommenting the body must produce a
// compile error of the form:
//
//   trait bound `(): Contains<Resource<Interner>>` was not satisfied
//   which is required by `(ReadInterner, ()): Buildable<()>`
//
// The error names the missing store directly. Verified manually
// 2026-05-01 against this commit. Captured as compile_fail
// trybuild fixture in #296 follow-up.
//
// fn _negative_compile_fail() {
//     let _ = Scheduler::<8, 16, 4>::builder()
//         .add::<ReadInterner>()
//         .build();
// }

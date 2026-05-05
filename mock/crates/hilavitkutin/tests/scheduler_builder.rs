//! SchedulerBuilder type-state tests.
//!
//! Round 4 reshape: Kit becomes declarative
//! (`type Units; type Owned`), `add_kit` is type-level only,
//! SchedulerBuilder loses cap const generics, `.build()` proves
//! `Stores: ContainsAll<Wus::AccumRead> + ContainsAll<Wus::AccumWrite>`.
//!
//! Two layers of tests:
//!
//! 1. Smoke tests with `Wus = Empty` exercising store accumulation.
//! 2. WU-bearing tests with a Stub + TestCtx shim. These exercise
//!    the load-bearing ContainsAll proof reduction.
//!
//! The negative case (`.add::<ReadInterner>().build()` without a
//! matching resource) is verified manually as a compile-fail; a
//! trybuild fixture is tracked in #296.

use arvo::USize;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::{
    AccessSet, Always, BatchApi, Column, ColumnReaderApi, ColumnValue,
    ColumnWriterApi, Cons, Contains, Depth, EachApi, Empty, HasBatch, HasColumnReader,
    HasColumnWriter, HasEach, HasReduce, HasResourceProvider, HasVirtualFirer, Atomic,
    Immediate, Normal, ReduceApi, Resource, ResourceProviderApi, Virtual,
    VirtualFirerApi, WorkUnit, read, write,
};
use hilavitkutin_kit::Kit;

// ---------------------------------------------------------------------
// Fake stores.
// ---------------------------------------------------------------------

pub struct Interner;
pub struct Workspace;
pub struct FileInfo;

// ---------------------------------------------------------------------
// Kits (declarative shape).
// ---------------------------------------------------------------------

pub struct InternerKit;

impl Kit for InternerKit {
    type Units = Empty;
    type Owned = Cons<Resource<Interner>, Empty>;
}

pub struct WorkspaceKit;

impl Kit for WorkspaceKit {
    type Units = Empty;
    type Owned = Cons<Column<FileInfo>, Cons<Resource<Workspace>, Empty>>;
}

// ---------------------------------------------------------------------
// Positive smoke tests.
// ---------------------------------------------------------------------

#[test]
fn empty_build() {
    let _ = Scheduler::builder().build();
}

#[test]
fn raw_resource_registration_builds() {
    let _ = Scheduler::builder()
        .resource(Interner)
        .build();
}

#[test]
fn raw_column_registration_builds() {
    let _ = Scheduler::builder().column::<FileInfo>().build();
}

#[test]
fn kit_only_builds() {
    let _ = Scheduler::builder().add_kit::<InternerKit>().build();
}

#[test]
fn two_kits_chained_build() {
    let _ = Scheduler::builder()
        .add_kit::<InternerKit>()
        .add_kit::<WorkspaceKit>()
        .build();
}

#[test]
fn mixed_kit_and_raw_build() {
    let _ = Scheduler::builder()
        .add_kit::<WorkspaceKit>()
        .resource(Interner)
        .column::<FileInfo>()
        .build();
}

#[test]
fn default_scheduler_constructs() {
    let _: Scheduler = Scheduler::default();
}

// ---------------------------------------------------------------------
// WU-bearing tests.
// ---------------------------------------------------------------------

struct Stub;

impl<R: AccessSet> ColumnReaderApi<R> for Stub {
    unsafe fn read<T: ColumnValue>(&self, _i: USize) -> T
    where
        R: Contains<Column<T>>,
    {
        unimplemented!()
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
        unimplemented!()
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

// A WU that reads the Interner resource.
struct ReadInterner;

impl WorkUnit<Always> for ReadInterner {
    type Read = read![Resource<Interner>];
    type Write = read![];
    type Hint = (Immediate, Atomic, Normal);
    type Ctx = TestCtx;
    fn execute(&self, _ctx: &TestCtx) {}
}

// A WU with write set: writes Column<FileInfo>.
struct DiscoverFiles;

impl WorkUnit<Always> for DiscoverFiles {
    type Read = read![Resource<Workspace>];
    type Write = write![Column<FileInfo>];
    type Hint = (Immediate, Atomic, Normal);
    type Ctx = TestCtx;
    fn execute(&self, _ctx: &TestCtx) {}
}

#[test]
fn wu_with_raw_resource_builds() {
    let _ = Scheduler::builder()
        .resource(Interner)
        .add::<ReadInterner>()
        .build();
}

#[test]
fn wu_with_kit_builds() {
    let _ = Scheduler::builder()
        .add_kit::<InternerKit>()
        .add::<ReadInterner>()
        .build();
}

#[test]
fn two_wus_with_two_kits_build() {
    let _ = Scheduler::builder()
        .add_kit::<InternerKit>()
        .add_kit::<WorkspaceKit>()
        .add::<ReadInterner>()
        .add::<DiscoverFiles>()
        .build();
}

// ---------------------------------------------------------------------
// Type-state shape verification (declarative Kit).
// ---------------------------------------------------------------------

#[test]
fn kit_declarative_shape_typechecks() {
    fn _type_check_only<K: Kit>() {}
    _type_check_only::<InternerKit>();
    _type_check_only::<WorkspaceKit>();
}

// ---------------------------------------------------------------------
// Wus uncap stress: 50 WUs in one builder. Validates the recursive
// HList accumulator handles realistic depth.
// ---------------------------------------------------------------------

struct NoStores;
impl WorkUnit<Always> for NoStores {
    type Read = read![];
    type Write = write![];
    type Hint = (Immediate, Atomic, Normal);
    type Ctx = TestCtx;
    fn execute(&self, _ctx: &TestCtx) {}
}

#[test]
fn smoke_fifty_wus() {
    let _ = Scheduler::builder()
        .add::<NoStores>().add::<NoStores>().add::<NoStores>().add::<NoStores>().add::<NoStores>()
        .add::<NoStores>().add::<NoStores>().add::<NoStores>().add::<NoStores>().add::<NoStores>()
        .add::<NoStores>().add::<NoStores>().add::<NoStores>().add::<NoStores>().add::<NoStores>()
        .add::<NoStores>().add::<NoStores>().add::<NoStores>().add::<NoStores>().add::<NoStores>()
        .add::<NoStores>().add::<NoStores>().add::<NoStores>().add::<NoStores>().add::<NoStores>()
        .add::<NoStores>().add::<NoStores>().add::<NoStores>().add::<NoStores>().add::<NoStores>()
        .add::<NoStores>().add::<NoStores>().add::<NoStores>().add::<NoStores>().add::<NoStores>()
        .add::<NoStores>().add::<NoStores>().add::<NoStores>().add::<NoStores>().add::<NoStores>()
        .add::<NoStores>().add::<NoStores>().add::<NoStores>().add::<NoStores>().add::<NoStores>()
        .add::<NoStores>().add::<NoStores>().add::<NoStores>().add::<NoStores>().add::<NoStores>()
        .build();
}

// ---------------------------------------------------------------------
// WuSatisfied uncap stress: a single WU with 16 stores in its Read
// set. Validates the recursive ContainsAll proof handles realistic
// store-count depth via the read! macro emitting Cons cells.
// ---------------------------------------------------------------------

struct S0;
struct S1;
struct S2;
struct S3;
struct S4;
struct S5;
struct S6;
struct S7;
struct S8;
struct S9;
struct S10;
struct S11;
struct S12;
struct S13;
struct S14;
struct S15;

struct SixteenStores;
impl WorkUnit<Always> for SixteenStores {
    type Read = read![
        Resource<S0>, Resource<S1>, Resource<S2>, Resource<S3>,
        Resource<S4>, Resource<S5>, Resource<S6>, Resource<S7>,
        Resource<S8>, Resource<S9>, Resource<S10>, Resource<S11>,
        Resource<S12>, Resource<S13>, Resource<S14>, Resource<S15>,
    ];
    type Write = write![];
    type Hint = (Immediate, Atomic, Normal);
    type Ctx = TestCtx;
    fn execute(&self, _ctx: &TestCtx) {}
}

#[test]
fn smoke_wu_with_sixteen_stores() {
    let _ = Scheduler::builder()
        .resource(S0).resource(S1).resource(S2).resource(S3)
        .resource(S4).resource(S5).resource(S6).resource(S7)
        .resource(S8).resource(S9).resource(S10).resource(S11)
        .resource(S12).resource(S13).resource(S14).resource(S15)
        .add::<SixteenStores>()
        .build();
}

// ---------------------------------------------------------------------
// Depth compile-time assertion using the Cons<H, R> impl.
// ---------------------------------------------------------------------

type Cons1<T> = Cons<NoStores, T>;
type Cons5<T> = Cons1<Cons1<Cons1<Cons1<Cons1<T> > > > >;
type Cons10<T> = Cons5<Cons5<T> >;
type Cons50 = Cons10<Cons10<Cons10<Cons10<Cons10<Empty> > > > >;

const _: () = assert!(<Cons50 as Depth>::D.0 == 50);

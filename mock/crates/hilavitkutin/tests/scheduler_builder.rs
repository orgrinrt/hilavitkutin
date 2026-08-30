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
//! The negative case (`.add_unit::<ReadInterner>().build()` without a
//! matching resource) is verified manually as a compile-fail; a
//! trybuild fixture is tracked in #296.

use arvo::{Bool, USize};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::{
    AccessSet, Always, Atomic, BatchApi, Column, ColumnReaderApi, ColumnValue,
    ColumnWriterApi, Cons, Contains, Depth, EachApi, Empty, HasBatch, HasColumnReader,
    HasColumnWriter, HasEach, HasReduce, HasResourceProvider, HasVirtualFirer,
    Immediate, Normal, BuilderInput, ReduceApi, ResolveColumnRead, ResolveColumnWrite,
    ResolveResource, ResolveVirtualFire, Resource, ResourceProviderApi,
    UnitDispatch, Virtual, VirtualFirerApi, WorkUnit, read, write,
};
use hilavitkutin_kit::{Kit, KitDispatch};
use hilavitkutin_providers::ArenaColumnStorage;

// ---------------------------------------------------------------------
// Stack-backed test memory provider (a fixed bump allocator).
// ---------------------------------------------------------------------

struct TestProvider<const N: usize> {
    buf: core::cell::UnsafeCell<[core::mem::MaybeUninit<u8>; N]>,
    used: core::cell::Cell<usize>,
}

impl<const N: usize> TestProvider<N> {
    fn new() -> Self {
        Self {
            buf: core::cell::UnsafeCell::new([const { core::mem::MaybeUninit::uninit() }; N]),
            used: core::cell::Cell::new(0),
        }
    }
}

unsafe impl<const N: usize> Send for TestProvider<N> {}
unsafe impl<const N: usize> Sync for TestProvider<N> {}

impl<const N: usize> MemoryProviderApi for TestProvider<N> {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let base = self.buf.get() as *mut u8;
        let used = self.used.get();
        let align = align.0.max(1);
        let aligned = (used + align - 1) / align * align;
        if aligned + len.0 > N {
            return core::ptr::null_mut();
        }
        self.used.set(aligned + len.0);
        // SAFETY: in bounds of the owned buffer.
        unsafe { base.add(aligned) }
    }
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize, _align: USize) {}
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

fn provider() -> TestProvider<4096> {
    TestProvider::<4096>::new()
}

/// The default-capacity arena store over the test provider. The return
/// type omits `D`, applying the `Dim<256>` default so inference is
/// anchored at the `build` call sites.
fn store() -> ArenaColumnStorage<TestProvider<4096>> {
    ArenaColumnStorage::new(provider())
}

// ---------------------------------------------------------------------
// Fake stores.
// ---------------------------------------------------------------------

// `Interner` is registered as a `Resource` value (`.with(Resource::new(
// Interner))`), so it must be `ColumnValue` (`Copy + 'static`).
// `Workspace` and `FileInfo` are only Kit-owned / column markers (never
// drained as resource values), so they need no derive.
#[derive(Copy, Clone)]
pub struct Interner;
pub struct Workspace;
#[derive(Copy, Clone)]
pub struct FileInfo;

// ---------------------------------------------------------------------
// Kits (declarative shape).
// ---------------------------------------------------------------------

pub struct InternerKit;

impl BuilderInput for InternerKit {
    type Init = Self;
    type Dispatch = KitDispatch<Self>;
}

impl Kit for InternerKit {
    type Units = Empty;
    type Owned = Cons<Resource<Interner>, Empty>;
}

pub struct WorkspaceKit;

impl BuilderInput for WorkspaceKit {
    type Init = Self;
    type Dispatch = KitDispatch<Self>;
}

impl Kit for WorkspaceKit {
    type Units = Empty;
    type Owned = Cons<Column<FileInfo>, Cons<Resource<Workspace>, Empty>>;
}

// ---------------------------------------------------------------------
// Positive smoke tests.
// ---------------------------------------------------------------------

#[test]
fn empty_build() {
    let _ = Scheduler::builder().build(store(), USize(0)).ok();
}

#[test]
fn raw_resource_registration_builds() {
    let _ = Scheduler::builder()
        .with(Resource::new(Interner))
        .build(store(), USize(0))
        .ok();
}

#[test]
fn raw_column_registration_builds() {
    let _ = Scheduler::builder()
        .with(Column::<FileInfo>::new())
        .build(store(), USize(0))
        .ok();
}

#[test]
fn kit_only_builds() {
    let _ = Scheduler::builder().with(InternerKit).build(store(), USize(0)).ok();
}

#[test]
fn two_kits_chained_build() {
    let _ = Scheduler::builder()
        .with(InternerKit)
        .with(WorkspaceKit)
        .build(store(), USize(0))
        .ok();
}

#[test]
fn mixed_kit_and_raw_build() {
    let _ = Scheduler::builder()
        .with(WorkspaceKit)
        .with(Resource::new(Interner))
        .with(Column::<FileInfo>::new())
        .build(store(), USize(0))
        .ok();
}

#[test]
fn scheduler_constructs_via_build() {
    // A scheduler now requires a memory provider; constructing through
    // an empty builder with a provider is the no-store base case.
    let _ = Scheduler::builder().build(store(), USize(0)).ok();
}

// ---------------------------------------------------------------------
// WU-bearing tests.
// ---------------------------------------------------------------------

struct Stub;

impl<R: AccessSet> ColumnReaderApi<R> for Stub {
    unsafe fn read<T: ColumnValue, I>(&self, _i: USize) -> T
    where
        R: Contains<Column<T>>,
        Self: ResolveColumnRead<T, I>,
    {
        unimplemented!()
    }
}

impl<T: ColumnValue, I> ResolveColumnRead<T, I> for Stub {
    unsafe fn resolve_read(&self, _i: USize) -> T {
        unimplemented!()
    }
}

impl<W: AccessSet> ColumnWriterApi<W> for Stub {
    unsafe fn write<T: ColumnValue, I>(&self, _i: USize, _v: T)
    where
        W: Contains<Column<T>>,
        Self: ResolveColumnWrite<T, I>,
    {
    }
}

impl<T: ColumnValue, I> ResolveColumnWrite<T, I> for Stub {
    unsafe fn resolve_write(&self, _i: USize, _v: T) {}
}

impl<R: AccessSet> ResourceProviderApi<R> for Stub {
    fn resource<T: 'static, I>(&self) -> &T
    where
        R: Contains<Resource<T>>,
        Self: ResolveResource<T, I>,
    {
        unimplemented!()
    }
}

impl<T: 'static, I> ResolveResource<T, I> for Stub {
    fn resolve_resource(&self) -> &T {
        unimplemented!()
    }
}

impl<W: AccessSet> VirtualFirerApi<W> for Stub {
    fn fire<V: 'static, I>(&self)
    where
        W: Contains<Virtual<V>>,
        Self: ResolveVirtualFire<V, I>,
    {
    }
}

impl<V, I> ResolveVirtualFire<V, I> for Stub {
    fn resolve_fire(&self) {}
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

impl BuilderInput for ReadInterner {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for ReadInterner {
    type Read = read![Resource<Interner>];
    type Write = read![];
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> = TestCtx;
    fn execute<'frame>(&self, _ctx: &TestCtx) {}
}

// A WU with write set: writes Column<FileInfo>.
struct DiscoverFiles;

impl BuilderInput for DiscoverFiles {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for DiscoverFiles {
    type Read = read![Resource<Workspace>];
    type Write = write![Column<FileInfo>];
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> = TestCtx;
    fn execute<'frame>(&self, _ctx: &TestCtx) {}
}

#[test]
fn wu_with_raw_resource_builds() {
    let _ = Scheduler::builder()
        .with(Resource::new(Interner))
        .with(ReadInterner)
        .build(store(), USize(0))
        .ok();
}

#[test]
fn wu_with_kit_builds() {
    let _ = Scheduler::builder()
        .with(InternerKit)
        .with(ReadInterner)
        .build(store(), USize(0))
        .ok();
}

#[test]
fn two_wus_with_two_kits_build() {
    let _ = Scheduler::builder()
        .with(InternerKit)
        .with(WorkspaceKit)
        .with(ReadInterner)
        .with(DiscoverFiles)
        .build(store(), USize(0))
        .ok();
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

#[derive(Copy, Clone)]
struct NoStores;

impl BuilderInput for NoStores {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for NoStores {
    type Read = read![];
    type Write = write![];
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> = TestCtx;
    fn execute<'frame>(&self, _ctx: &TestCtx) {}
}

#[test]
fn smoke_fifty_wus() {
    let _ = Scheduler::builder()
        .with(NoStores).with(NoStores).with(NoStores).with(NoStores).with(NoStores)
        .with(NoStores).with(NoStores).with(NoStores).with(NoStores).with(NoStores)
        .with(NoStores).with(NoStores).with(NoStores).with(NoStores).with(NoStores)
        .with(NoStores).with(NoStores).with(NoStores).with(NoStores).with(NoStores)
        .with(NoStores).with(NoStores).with(NoStores).with(NoStores).with(NoStores)
        .with(NoStores).with(NoStores).with(NoStores).with(NoStores).with(NoStores)
        .with(NoStores).with(NoStores).with(NoStores).with(NoStores).with(NoStores)
        .with(NoStores).with(NoStores).with(NoStores).with(NoStores).with(NoStores)
        .with(NoStores).with(NoStores).with(NoStores).with(NoStores).with(NoStores)
        .with(NoStores).with(NoStores).with(NoStores).with(NoStores).with(NoStores)
        .build(store(), USize(0))
        .ok();
}

// ---------------------------------------------------------------------
// WuSatisfied uncap stress: a single WU with 16 stores in its Read
// set. Validates the recursive ContainsAll proof handles realistic
// store-count depth via the read! macro emitting Cons cells.
// ---------------------------------------------------------------------

// Each `Sn` is registered as a `Resource` value below, so each must be
// `ColumnValue` (`Copy + 'static`).
#[derive(Copy, Clone)]
struct S0;
#[derive(Copy, Clone)]
struct S1;
#[derive(Copy, Clone)]
struct S2;
#[derive(Copy, Clone)]
struct S3;
#[derive(Copy, Clone)]
struct S4;
#[derive(Copy, Clone)]
struct S5;
#[derive(Copy, Clone)]
struct S6;
#[derive(Copy, Clone)]
struct S7;
#[derive(Copy, Clone)]
struct S8;
#[derive(Copy, Clone)]
struct S9;
#[derive(Copy, Clone)]
struct S10;
#[derive(Copy, Clone)]
struct S11;
#[derive(Copy, Clone)]
struct S12;
#[derive(Copy, Clone)]
struct S13;
#[derive(Copy, Clone)]
struct S14;
#[derive(Copy, Clone)]
struct S15;

struct SixteenStores;

impl BuilderInput for SixteenStores {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for SixteenStores {
    type Read = read![
        Resource<S0>, Resource<S1>, Resource<S2>, Resource<S3>,
        Resource<S4>, Resource<S5>, Resource<S6>, Resource<S7>,
        Resource<S8>, Resource<S9>, Resource<S10>, Resource<S11>,
        Resource<S12>, Resource<S13>, Resource<S14>, Resource<S15>,
    ];
    type Write = write![];
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> = TestCtx;
    fn execute<'frame>(&self, _ctx: &TestCtx) {}
}

#[test]
fn smoke_wu_with_sixteen_stores() {
    let _ = Scheduler::builder()
        .with(Resource::new(S0)).with(Resource::new(S1)).with(Resource::new(S2)).with(Resource::new(S3))
        .with(Resource::new(S4)).with(Resource::new(S5)).with(Resource::new(S6)).with(Resource::new(S7))
        .with(Resource::new(S8)).with(Resource::new(S9)).with(Resource::new(S10)).with(Resource::new(S11))
        .with(Resource::new(S12)).with(Resource::new(S13)).with(Resource::new(S14)).with(Resource::new(S15))
        .with(SixteenStores)
        .build(store(), USize(0))
        .ok();
}

// ---------------------------------------------------------------------
// Depth compile-time assertion using the Cons<H, R> impl.
// ---------------------------------------------------------------------

type Cons1<T> = Cons<NoStores, T>;
type Cons5<T> = Cons1<Cons1<Cons1<Cons1<Cons1<T> > > > >;
type Cons10<T> = Cons5<Cons5<T> >;
type Cons50 = Cons10<Cons10<Cons10<Cons10<Cons10<Empty> > > > >;

const _: () = assert!(<Cons50 as Depth>::D.0 == 50);

// A3b: test-local resource values are bare scalars/markers with no Seq/Map
// collection members, so their L1 morsel footprint is zero.
impl hilavitkutin_api::footprint::ResourceFootprint for Interner {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for S0 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for S1 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for S10 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for S11 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for S12 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for S13 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for S14 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for S15 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for S2 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for S3 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for S4 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for S5 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for S6 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for S7 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for S8 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for S9 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for Workspace {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}

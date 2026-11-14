//! SchedulerBuilder type-state tests.
//!
//! Round 4 reshape: Kit becomes declarative
//! (`type Units; type Owned`), `add_kit` is type-level only,
//! SchedulerBuilder loses cap const generics, `.build()` proves
//! `Stores: HoldsAll<Wus::AccumRead> + HoldsAll<Wus::AccumWrite>`.
//!
//! Two layers of tests:
//!
//! 1. Smoke tests with `Wus = Empty` exercising store accumulation.
//! 2. WU-bearing tests with a Stub + TestCtx shim. These exercise
//!    the load-bearing HoldsAll proof reduction.
//!
//! The negative case (a unit registered without the resource it reads) is
//! the compile-fail pair `tests/ui/unregistered_store_{fail,pass}.rs`, run
//! by `tests/ui.rs`.

use arvo::{Bool, USize};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::{
    AccessSet,
    Always,
    Atomic,
    BatchApi,
    BuilderInput,
    Column,
    ColumnReaderApi,
    ColumnValue,
    ColumnWriterApi,
    Cons,
    Depth,
    EachApi,
    Empty,
    HasBatch,
    HasColumnReader,
    HasColumnWriter,
    HasEach,
    HasReduce,
    HasResourceProvider,
    HasVirtualFirer,
    Holds,
    Immediate,
    Normal,
    ReduceApi,
    ResolveColumnRead,
    ResolveColumnWrite,
    ResolveResource,
    ResolveVirtualFire,
    Resource,
    ResourceProviderApi,
    StartsFromDefault,
    UnitDispatch,
    Virtual,
    VirtualFirerApi,
    WorkUnit,
    read,
    write,
};
use hilavitkutin_kit::{Kit, KitDispatch};
use hilavitkutin_providers::ArenaColumnStorage;

// ---------------------------------------------------------------------
// Stack-backed test memory provider (a fixed bump allocator).
// ---------------------------------------------------------------------

struct TestProvider<const N: usize> {
    buf:  core::cell::UnsafeCell<[core::mem::MaybeUninit<u8>; N]>,
    used: core::cell::Cell<usize>,
}

impl<const N: usize> TestProvider<N> {
    fn new() -> Self {
        Self {
            buf:  core::cell::UnsafeCell::new([const { core::mem::MaybeUninit::uninit() }; N]),
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

// The fifty-unit registration needs more plan and meta storage than the
// default 4096 bytes: with the small arena `build` returns `AllocationFailed`.
fn store_for_fifty() -> ArenaColumnStorage<TestProvider<65536>> {
    ArenaColumnStorage::new(TestProvider::<65536>::new())
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
#[derive(Copy, Clone, Default)]
pub struct FileInfo;
// A column no kit owns, for the raw registration beside a kit.
#[derive(Copy, Clone, Default)]
pub struct Notes;
// A second one, registered after a kit.
#[derive(Copy, Clone, Default)]
pub struct Marks;

// ---------------------------------------------------------------------
// Kits (declarative shape).
// ---------------------------------------------------------------------

pub struct InternerKit;

impl BuilderInput for InternerKit {
    type Dispatch = KitDispatch<Self>;
    type Init = Self;
}

impl Kit for InternerKit {
    type Owned = Cons<Resource<Interner>, Empty>;
    type Units = Empty;
}

pub struct WorkspaceKit;

impl BuilderInput for WorkspaceKit {
    type Dispatch = KitDispatch<Self>;
    type Init = Self;
}

impl Kit for WorkspaceKit {
    type Owned = Cons<Column<FileInfo>, Cons<Resource<Workspace>, Empty>>;
    type Units = Empty;
}

// ---------------------------------------------------------------------
// Positive smoke tests.
// ---------------------------------------------------------------------

// An empty builder with a provider is the no-store base case: a scheduler
// requires a memory provider, and nothing else.
#[test]
fn empty_build() {
    let built = Scheduler::builder().build(store(), USize(0));
    assert!(built.is_ok(), "the builder must accept this registration");
}

#[test]
fn raw_resource_registration_builds() {
    let built = Scheduler::builder()
        .with(Resource::new(Interner))
        .build(store(), USize(0));
    assert!(built.is_ok(), "the builder must accept this registration");
}

#[test]
fn raw_column_registration_builds() {
    let built = Scheduler::builder()
        .with(Column::<FileInfo>::start())
        .build(store(), USize(0));
    assert!(built.is_ok(), "the builder must accept this registration");
}

#[test]
fn kit_only_builds() {
    let built = Scheduler::builder()
        .with(InternerKit)
        .build(store(), USize(0));
    assert!(built.is_ok(), "the builder must accept this registration");
}

#[test]
fn two_kits_chained_build() {
    let built = Scheduler::builder()
        .with(InternerKit)
        .with(WorkspaceKit)
        .build(store(), USize(0));
    assert!(built.is_ok(), "the builder must accept this registration");
}

// The raw column is one the kit does not own. A column registered twice, once
// by a kit and once raw, has two positions in the store list, so `build`
// cannot give its binding one and the registration is refused for want of a
// type annotation.
#[test]
fn mixed_kit_and_raw_build() {
    let built = Scheduler::builder()
        .with(WorkspaceKit)
        .with(Resource::new(Interner))
        .with(Column::<Notes>::start())
        .build(store(), USize(0));
    assert!(built.is_ok(), "the builder must accept this registration");
}

// A kit's stores have a place in the store list and no value of their own,
// so they have no node along the bindings. A column registered ahead of the
// kit is the second node and the fourth store, and its position is the
// store's: the bit the access masks know it by.
#[test]
fn a_column_is_placed_by_the_store_list_and_not_by_its_node() {
    let built = Scheduler::builder()
        .with(Column::<Notes>::start())
        .with(WorkspaceKit)
        .with(Column::<Marks>::start())
        .build(store(), USize(0));
    let notko::Outcome::Ok(scheduler) = built else {
        panic!("the builder must accept this registration");
    };
    let marks = scheduler.__bindings();
    let notes = marks.__tail();
    assert_eq!(
        marks.__pos(),
        USize(0),
        "the last store registered leads the list"
    );
    assert_eq!(notes.__pos(), USize(3), "the kit's two stores sit between");
}

// ---------------------------------------------------------------------
// WU-bearing tests.
// ---------------------------------------------------------------------

struct Stub;

impl<R: AccessSet> ColumnReaderApi<R> for Stub {
    unsafe fn read<T: ColumnValue, I>(&self, _i: USize) -> T
    where
        R: Holds<Column<T>>,
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

impl<W: AccessSet> ColumnWriterApi<W> for Stub {}

impl<T: ColumnValue, I> ResolveColumnWrite<T, I> for Stub {
    unsafe fn resolve_write(&self, _i: USize, _v: T) {}

    unsafe fn resolve_held(&self, _i: USize) -> T {
        unimplemented!()
    }
}

impl<R: AccessSet> ResourceProviderApi<R> for Stub {
    fn resource<T: 'static, I>(&self) -> &T
    where
        R: Holds<Resource<T>>,
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
        W: Holds<Virtual<V>>,
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
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for ReadInterner {
    type Ctx<'frame> = TestCtx;
    type Hint = (Immediate, Atomic, Normal);
    type Read = read![Resource<Interner>];
    type Write = read![];

    fn execute<'frame>(&self, _ctx: &TestCtx) {}
}

// A WU with write set: writes Column<FileInfo>.
struct DiscoverFiles;

impl BuilderInput for DiscoverFiles {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for DiscoverFiles {
    type Ctx<'frame> = TestCtx;
    type Hint = (Immediate, Atomic, Normal);
    type Read = read![Resource<Workspace>];
    type Write = write![Column<FileInfo>];

    fn execute<'frame>(&self, _ctx: &TestCtx) {}
}

#[test]
fn wu_with_raw_resource_builds() {
    let built = Scheduler::builder()
        .with(Resource::new(Interner))
        .with(ReadInterner)
        .build(store(), USize(0));
    assert!(built.is_ok(), "the builder must accept this registration");
}

#[test]
fn wu_with_kit_builds() {
    let built = Scheduler::builder()
        .with(InternerKit)
        .with(ReadInterner)
        .build(store(), USize(0));
    assert!(built.is_ok(), "the builder must accept this registration");
}

#[test]
fn two_wus_with_two_kits_build() {
    let built = Scheduler::builder()
        .with(InternerKit)
        .with(WorkspaceKit)
        .with(ReadInterner)
        .with(DiscoverFiles)
        .build(store(), USize(0));
    assert!(built.is_ok(), "the builder must accept this registration");
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
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for NoStores {
    type Ctx<'frame> = TestCtx;
    type Hint = (Immediate, Atomic, Normal);
    type Read = read![];
    type Write = write![];

    fn execute<'frame>(&self, _ctx: &TestCtx) {}
}

#[test]
fn smoke_fifty_wus() {
    let built = Scheduler::builder()
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .build(store_for_fifty(), USize(0));
    if let notko::Outcome::Err(e) = built {
        panic!("fifty-unit registration: {e:?}");
    }
}

// ---------------------------------------------------------------------
// WuSatisfied uncap stress: a single WU with 16 stores in its Read
// set. Validates the recursive HoldsAll proof handles realistic
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
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for SixteenStores {
    type Ctx<'frame> = TestCtx;
    type Hint = (Immediate, Atomic, Normal);
    type Read = read![
        Resource<S0>,
        Resource<S1>,
        Resource<S2>,
        Resource<S3>,
        Resource<S4>,
        Resource<S5>,
        Resource<S6>,
        Resource<S7>,
        Resource<S8>,
        Resource<S9>,
        Resource<S10>,
        Resource<S11>,
        Resource<S12>,
        Resource<S13>,
        Resource<S14>,
        Resource<S15>,
    ];
    type Write = write![];

    fn execute<'frame>(&self, _ctx: &TestCtx) {}
}

#[test]
fn smoke_wu_with_sixteen_stores() {
    let built = Scheduler::builder()
        .with(Resource::new(S0))
        .with(Resource::new(S1))
        .with(Resource::new(S2))
        .with(Resource::new(S3))
        .with(Resource::new(S4))
        .with(Resource::new(S5))
        .with(Resource::new(S6))
        .with(Resource::new(S7))
        .with(Resource::new(S8))
        .with(Resource::new(S9))
        .with(Resource::new(S10))
        .with(Resource::new(S11))
        .with(Resource::new(S12))
        .with(Resource::new(S13))
        .with(Resource::new(S14))
        .with(Resource::new(S15))
        .with(SixteenStores)
        .build(store(), USize(0));
    assert!(built.is_ok(), "the builder must accept this registration");
}

// ---------------------------------------------------------------------
// Depth compile-time assertion using the Cons<H, R> impl.
// ---------------------------------------------------------------------

type Cons1<T> = Cons<NoStores, T>;
type Cons5<T> = Cons1<Cons1<Cons1<Cons1<Cons1<T>>>>>;
type Cons10<T> = Cons5<Cons5<T>>;
type Cons50 = Cons10<Cons10<Cons10<Cons10<Cons10<Empty>>>>>;

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

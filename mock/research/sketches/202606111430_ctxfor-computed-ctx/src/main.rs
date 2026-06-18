//! Feasibility sketch: compute the full `EngineCtx` type from (R, W, Sched).
//!
//! HYPOTHESIS (full statement in FINDINGS.md): the six derived `EngineCtx`
//! parameters (RBundle, RCols, WCols, WAccum, WVirt, MP) are pure type
//! functions of a WorkUnit's Read / Write access sets and its schedule. An
//! engine-side type-level map (disjoint impls per access-set head kind, the
//! same kind dispatch the shipped `Project` / `ColProject` / `AccumProject` /
//! `VirtualProject` traits already use; no specialization anywhere) can
//! therefore compute the whole nine-parameter type, so a consumer writes
//! `type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write, Sched>`
//! instead of hand-spelling the bundles. The dispatch-side `RunFiber`
//! projection-equality bound must unify with the computed form (both sides
//! normalize to the same concrete cons chains), proven by real
//! `Scheduler::run` passes over column, virtual-firing, and meta-bridge DAGs.
//!
//! Three probe layers:
//! 1. compile-time type-identity assertions: `CtxFor` output == the
//!    hand-spelled `EngineCtx` aliases, across all four store kinds, set
//!    interleavings, and all three schedule kinds (MP keying);
//! 2. self-referential consumer spelling: `Self::Read` / `Self::Write` and
//!    `<Self as HasSchedule>::Sched` (including the blanket-impl-derived
//!    form, the potential normalization cycle) inside real WorkUnit impls;
//! 3. real dispatch: three `Scheduler::run` scenarios where every WU declares
//!    its Ctx via `CtxFor`, exercising resource + column projection (A),
//!    write-virtual bundle + On<V> gating (B), and accumulator bundle +
//!    OnMeta MetaRef bridging (C).

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use std::cell::RefCell;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons, AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, MetaNil, MetaPtrFor, MetaRef, PtrCons,
    PtrNil, VirtCons, VirtNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{AccessSet, Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    AccumWriterApi, ColumnReaderApi, ColumnWriterApi, EachApi, HasAccumWriter, HasColumnReader,
    HasColumnWriter, HasEach, HasResourceProvider, HasVirtualFirer, ResourceProviderApi,
    VirtualFirerApi,
};
use hilavitkutin_api::meta::SchedulerMetrics;
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::run_cfg::ScheduleEnd;
use hilavitkutin_api::store::{Accum, Column, Resource, Virtual};
use hilavitkutin_api::work_unit::{Always, HasSchedule, On, OnMeta, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;

// ---------------------------------------------------------------------
// Part 1: the type function. Engine-side in the real change (it names the
// engine's bundle types, so it cannot live in hilavitkutin-api; the sketch
// crate sits in the same position as the engine: it sees both the api sets
// and the engine bundles, so a local definition is representative).
//
// Each trait is a pure fold over the access set, keyed on the head's store
// kind exactly the way the shipped projection traits key their impls:
// `Cons<Resource<T>, Tail>` / `Cons<Column<T>, Tail>` / `Cons<Accum<T>,
// Tail>` / `Cons<Virtual<T>, Tail>` are four distinct concrete type
// constructors, so the four impls are disjoint and no specialization is
// involved. The contributing kind conses its bundle node; the other three
// kinds pass the tail through. Output order is therefore the subsequence of
// set order, which is exactly the order `Project` and friends build the
// runtime bundles in, so the computed type and the projected value type
// agree node for node.
// ---------------------------------------------------------------------

/// The projected resource bundle type for an access set: `PtrCons` chain over
/// the `Resource<T>` members in set order, `PtrNil` leaf.
pub trait ResourceBundleOf {
    type Out;
}
impl ResourceBundleOf for Empty {
    type Out = PtrNil;
}
impl<T, Tail: ResourceBundleOf> ResourceBundleOf for Cons<Resource<T>, Tail> {
    type Out = PtrCons<T, Tail::Out>;
}
impl<T, Tail: ResourceBundleOf> ResourceBundleOf for Cons<Column<T>, Tail> {
    type Out = Tail::Out;
}
impl<T, Tail: ResourceBundleOf> ResourceBundleOf for Cons<Accum<T>, Tail> {
    type Out = Tail::Out;
}
impl<T, Tail: ResourceBundleOf> ResourceBundleOf for Cons<Virtual<T>, Tail> {
    type Out = Tail::Out;
}

/// The projected column bundle type for an access set: `ColPtrCons` chain over
/// the `Column<T>` members in set order, `ColPtrNil` leaf.
pub trait ColBundleOf {
    type Out;
}
impl ColBundleOf for Empty {
    type Out = ColPtrNil;
}
impl<T, Tail: ColBundleOf> ColBundleOf for Cons<Column<T>, Tail> {
    type Out = ColPtrCons<T, Tail::Out>;
}
impl<T, Tail: ColBundleOf> ColBundleOf for Cons<Resource<T>, Tail> {
    type Out = Tail::Out;
}
impl<T, Tail: ColBundleOf> ColBundleOf for Cons<Accum<T>, Tail> {
    type Out = Tail::Out;
}
impl<T, Tail: ColBundleOf> ColBundleOf for Cons<Virtual<T>, Tail> {
    type Out = Tail::Out;
}

/// The projected accumulator bundle type for a write set: `AccPtrCons<'frame>`
/// chain over the `Accum<T>` members, `AccPtrNil` leaf. Lifetime-bearing (the
/// runtime bundle borrows the bindings' live-length cells), so the trait
/// carries `'frame` the same way `AccumProject<'s, ...>` does.
pub trait AccumBundleOf<'frame> {
    type Out;
}
impl<'frame> AccumBundleOf<'frame> for Empty {
    type Out = AccPtrNil;
}
impl<'frame, T, Tail: AccumBundleOf<'frame>> AccumBundleOf<'frame> for Cons<Accum<T>, Tail> {
    type Out = AccPtrCons<'frame, T, Tail::Out>;
}
impl<'frame, T, Tail: AccumBundleOf<'frame>> AccumBundleOf<'frame> for Cons<Resource<T>, Tail> {
    type Out = Tail::Out;
}
impl<'frame, T, Tail: AccumBundleOf<'frame>> AccumBundleOf<'frame> for Cons<Column<T>, Tail> {
    type Out = Tail::Out;
}
impl<'frame, T, Tail: AccumBundleOf<'frame>> AccumBundleOf<'frame> for Cons<Virtual<T>, Tail> {
    type Out = Tail::Out;
}

/// The projected write-virtual bundle type for a write set: `VirtCons<'frame>`
/// chain over the `Virtual<V>` members, `VirtNil` leaf. Lifetime-bearing (the
/// runtime bundle borrows the bindings' stamp cells).
pub trait VirtBundleOf<'frame> {
    type Out;
}
impl<'frame> VirtBundleOf<'frame> for Empty {
    type Out = VirtNil;
}
impl<'frame, V, Tail: VirtBundleOf<'frame>> VirtBundleOf<'frame> for Cons<Virtual<V>, Tail> {
    type Out = VirtCons<'frame, V, Tail::Out>;
}
impl<'frame, T, Tail: VirtBundleOf<'frame>> VirtBundleOf<'frame> for Cons<Resource<T>, Tail> {
    type Out = Tail::Out;
}
impl<'frame, T, Tail: VirtBundleOf<'frame>> VirtBundleOf<'frame> for Cons<Column<T>, Tail> {
    type Out = Tail::Out;
}
impl<'frame, T, Tail: VirtBundleOf<'frame>> VirtBundleOf<'frame> for Cons<Accum<T>, Tail> {
    type Out = Tail::Out;
}

/// The full computed per-WU Context type: every derived `EngineCtx` parameter
/// is a projection over (R, W, S). `S` defaults `Always`, mirroring the
/// `WorkUnit<Schedule = Always>` default, so the common consumer spelling is
/// `CtxFor<'frame, Self::Read, Self::Write>`. MP keys off the schedule via
/// the shipped `MetaPtrFor` (engine-side already): `MetaNil` for `Always` /
/// `On<V>`, `MetaRef<'frame>` for `OnMeta<V>`.
pub type CtxFor<'frame, R, W, S = Always> = EngineCtx<
    'frame,
    R,
    W,
    <R as ResourceBundleOf>::Out,
    <R as ColBundleOf>::Out,
    <W as ColBundleOf>::Out,
    <W as AccumBundleOf<'frame>>::Out,
    <W as VirtBundleOf<'frame>>::Out,
    <S as MetaPtrFor<'frame>>::Ptr,
>;

// ---------------------------------------------------------------------
// Part 2: compile-time type-identity assertions. `assert_same::<A, B>()`
// compiles only when A and B are literally the same type; each assertion is
// generic over 'f so the identity holds for every frame lifetime.
// ---------------------------------------------------------------------

trait SameAs<T> {}
impl<T> SameAs<T> for T {}
fn assert_same<A, B: SameAs<A>>() {}

struct Cfg;
struct In1;
struct Out1;
#[derive(Copy, Clone)]
struct Mk(u32);
struct Tk;

// A mixed read set (resource + column) and a mixed write set covering the
// remaining kinds (column + accum + virtual).
type RMix = Cons<Resource<Cfg>, Cons<Column<In1>, Empty>>;
type WMix = Cons<Column<Out1>, Cons<Accum<Mk>, Cons<Virtual<Tk>, Empty>>>;

type HandMix<'f> = EngineCtx<
    'f,
    RMix,
    WMix,
    PtrCons<Cfg, PtrNil>,
    ColPtrCons<In1, ColPtrNil>,
    ColPtrCons<Out1, ColPtrNil>,
    AccPtrCons<'f, Mk, AccPtrNil>,
    VirtCons<'f, Tk, VirtNil>,
    MetaNil,
>;

// The interleaved set (column before resource, virtual before accum): the
// computed bundles must be the kind-filtered subsequence in set order, the
// same order `Project` / `AccumProject` build the runtime values in.
type RFlip = Cons<Column<In1>, Cons<Resource<Cfg>, Empty>>;
type WFlip = Cons<Virtual<Tk>, Cons<Column<Out1>, Cons<Accum<Mk>, Empty>>>;

type HandFlip<'f> = EngineCtx<
    'f,
    RFlip,
    WFlip,
    PtrCons<Cfg, PtrNil>,
    ColPtrCons<In1, ColPtrNil>,
    ColPtrCons<Out1, ColPtrNil>,
    AccPtrCons<'f, Mk, AccPtrNil>,
    VirtCons<'f, Tk, VirtNil>,
    MetaNil,
>;

// OnMeta swaps only the 9th parameter to MetaRef<'f>.
type HandMeta<'f> = EngineCtx<
    'f,
    RMix,
    WMix,
    PtrCons<Cfg, PtrNil>,
    ColPtrCons<In1, ColPtrNil>,
    ColPtrCons<Out1, ColPtrNil>,
    AccPtrCons<'f, Mk, AccPtrNil>,
    VirtCons<'f, Tk, VirtNil>,
    MetaRef<'f>,
>;

fn type_identity_probe<'f>() {
    // All four store kinds, default schedule (S = Always defaults in the alias).
    assert_same::<HandMix<'f>, CtxFor<'f, RMix, WMix>>();
    // Interleaved kind order: computed bundles preserve set-order subsequences.
    assert_same::<HandFlip<'f>, CtxFor<'f, RFlip, WFlip, Always>>();
    // On<V> keys MP to MetaNil, identical to Always.
    assert_same::<HandMix<'f>, CtxFor<'f, RMix, WMix, On<Tk>>>();
    // OnMeta<V> keys MP to MetaRef<'f>.
    assert_same::<HandMeta<'f>, CtxFor<'f, RMix, WMix, OnMeta<ScheduleEnd>>>();
    // Empty sets: every bundle parameter collapses to its nil leaf.
    assert_same::<
        EngineCtx<'f, Empty, Empty, PtrNil, ColPtrNil, ColPtrNil, AccPtrNil, VirtNil, MetaNil>,
        CtxFor<'f, Empty, Empty>,
    >();
}

// ---------------------------------------------------------------------
// Shared test scaffolding (mirrors the engine integration tests).
// ---------------------------------------------------------------------

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

struct BumpProvider<const N: usize> {
    buf: UnsafeCell<[MaybeUninit<u8>; N]>,
    used: Cell<usize>,
}
impl<const N: usize> BumpProvider<N> {
    fn new() -> Self {
        Self { buf: UnsafeCell::new([const { MaybeUninit::uninit() }; N]), used: Cell::new(0) }
    }
}
unsafe impl<const N: usize> Send for BumpProvider<N> {}
unsafe impl<const N: usize> Sync for BumpProvider<N> {}
impl<const N: usize> MemoryProviderApi for BumpProvider<N> {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let base = self.buf.get() as *mut u8;
        let used = self.used.get();
        let align = align.0.max(1);
        let aligned = (used + align - 1) / align * align;
        if aligned + len.0 > N {
            return core::ptr::null_mut();
        }
        self.used.set(aligned + len.0);
        // SAFETY: aligned + len <= N, in bounds of the owned buffer.
        unsafe { base.add(aligned) }
    }
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

type Hints = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);

// ---------------------------------------------------------------------
// Scenario A: resource + column DAG through Scheduler::run, every Ctx
// computed. Producer reads Resource<InA>, writes Column<Ca>; Consumer reads
// Column<Ca>, writes Column<Cb> and records what it read. The producer
// spells the schedule explicitly; the consumer spells it as
// `<Self as HasSchedule>::Sched`, which resolves through the BLANKET
// `impl<W: WorkUnit<Always>> HasSchedule for W`, i.e. through the very
// WorkUnit impl whose Ctx is being defined: the potential normalization
// cycle this sketch probes.
// ---------------------------------------------------------------------

const RECORDS: usize = 4;

#[derive(Copy, Clone)]
struct InA(u32);
#[derive(Copy, Clone)]
struct Ca(u32);
#[derive(Copy, Clone)]
struct Cb(u32);

type ReadA = Cons<Resource<InA>, Empty>;
type WriteCa = Cons<Column<Ca>, Empty>;
type ReadCa = Cons<Column<Ca>, Empty>;
type WriteCb = Cons<Column<Cb>, Empty>;

thread_local! {
    static OBSERVED: RefCell<Vec<u32>> = const { RefCell::new(Vec::new()) };
}

struct ProducerWu;
impl BuilderInput for ProducerWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for ProducerWu {
    type Read = ReadA;
    type Write = WriteCa;
    type Hint = Hints;
    // The computed spelling under probe: no hand-spelled bundles.
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write, Always>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        let seed: &InA = ctx.resources().resource();
        let base = seed.0;
        ctx.each().run(|i| {
            // SAFETY: Ca reserved for the record count; exclusive writer.
            unsafe { ctx.writer().write::<Ca, _>(i, Ca(base + i.0 as u32)) };
        });
    }
}

struct ReaderWu;
impl BuilderInput for ReaderWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for ReaderWu {
    type Read = ReadCa;
    type Write = WriteCb;
    type Hint = Hints;
    // Blanket-HasSchedule cycle probe: Sched resolves through the blanket
    // impl gated on `Self: WorkUnit<Always>`, the impl being defined here.
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write, <Self as HasSchedule>::Sched>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: producer (RAW edge on Ca) wrote every covered record.
            let v: Ca = unsafe { ctx.reader().read::<Ca, _>(i) };
            OBSERVED.with(|o| o.borrow_mut().push(v.0));
            // SAFETY: Cb reserved for the record count; exclusive writer.
            unsafe { ctx.writer().write::<Cb, _>(i, Cb(v.0 + 1)) };
        });
    }
}

fn scenario_a_resource_column() {
    OBSERVED.with(|o| o.borrow_mut().clear());
    let provider = BumpProvider::<8192>::new();
    let mut scheduler = Scheduler::builder()
        .with(Resource::new(InA(100)))
        .with(Column::<Ca>::new())
        .with(Column::<Cb>::new())
        .with(ProducerWu)
        .with(ReaderWu)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("scenario A build should succeed"));
    let result = scheduler.run();
    assert!(matches!(result, Outcome::Ok(())));
    OBSERVED.with(|o| {
        assert_eq!(
            o.borrow().as_slice(),
            &[100u32, 101, 102, 103],
            "scenario A: reader read back the producer's resource-seeded column",
        );
    });
    println!("scenario A (resource + column, computed Ctx): PASS");
}

// ---------------------------------------------------------------------
// Scenario B: write-virtual bundle + On<V> gating, every Ctx computed. The
// firer (Always) writes Column<Gate> + Virtual<Tick> and fires; an On<Tick>
// consumer runs (sentinel 1), an On<Never> consumer is gate-skipped
// (sentinel stays 0). Mirrors tests/gate2_virtual_firing.rs.
// ---------------------------------------------------------------------

struct Tick;
struct Never;
#[derive(Copy, Clone)]
struct Gate(u32);
#[derive(Copy, Clone)]
struct RanA(u32);
#[derive(Copy, Clone)]
struct RanN(u32);

type WTick = Cons<Virtual<Tick>, Cons<Column<Gate>, Empty>>;
type ColGate = Cons<Column<Gate>, Empty>;

struct FirerWu;
impl BuilderInput for FirerWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for FirerWu {
    type Read = Empty;
    type Write = WTick;
    type Hint = Hints;
    // The write set carries a virtual, so the computed WVirt is
    // VirtCons<'frame, Tick, VirtNil>, derived rather than hand-spelled.
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write, Always>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: Gate reserved + exclusive; morsel covers this slice.
            unsafe { ctx.writer().write::<Gate, _>(i, Gate(1)) };
        });
        ctx.virtuals().fire::<Tick, _>();
    }
}

macro_rules! on_consumer {
    ($name:ident, $sched:ty, $ran:ident) => {
        struct $name;
        impl BuilderInput for $name {
            type Init = Self;
            type Dispatch = UnitDispatch<Self>;
        }
        impl HasSchedule for $name {
            type Sched = $sched;
        }
        impl WorkUnit<$sched> for $name {
            type Read = ColGate;
            type Write = Cons<Column<$ran>, Empty>;
            type Hint = Hints;
            // Computed, with the schedule recovered through the explicit
            // HasSchedule impl above.
            type Ctx<'frame> =
                CtxFor<'frame, Self::Read, Self::Write, <Self as HasSchedule>::Sched>;
            fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
                ctx.each().run(|i| {
                    // SAFETY: Gate produced this frame; Ran reserved + exclusive.
                    let _g = unsafe { ctx.reader().read::<Gate, _>(i) };
                    unsafe { ctx.writer().write::<$ran, _>(i, $ran(1)) };
                });
            }
        }
    };
}

on_consumer!(ConA, On<Tick>, RanA);
on_consumer!(ConN, On<Never>, RanN);

fn scenario_b_virtual_gating() {
    let provider = BumpProvider::<16384>::new();
    // Sentinels registered last: bindings head-chain is RanA (head), RanN.
    let mut scheduler = Scheduler::builder()
        .with(Virtual::<Tick>::new())
        .with(Virtual::<Never>::new())
        .with(Column::<Gate>::new())
        .with(Column::<RanN>::new())
        .with(Column::<RanA>::new())
        .with(FirerWu)
        .with(ConN)
        .with(ConA)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("scenario B build should succeed"));

    let a_base = scheduler.__bindings().__ptr().as_ptr() as *mut u32;
    let n_base = scheduler.__bindings().__tail().__ptr().as_ptr() as *mut u32;
    for i in 0..RECORDS {
        // SAFETY: both Ran columns reserved for RECORDS records; poison to 0.
        unsafe {
            *a_base.add(i) = 0;
            *n_base.add(i) = 0;
        }
    }

    let result = scheduler.run();
    assert!(matches!(result, Outcome::Ok(())));

    let a = scheduler.__bindings().__ptr().as_ptr() as *const u32;
    let n = scheduler.__bindings().__tail().__ptr().as_ptr() as *const u32;
    for i in 0..RECORDS {
        // SAFETY: Ran columns hold RECORDS reserved records; scheduler alive.
        let (va, vn) = unsafe { (*a.add(i), *n.add(i)) };
        assert_eq!(va, 1, "rec {i}: On<Tick> consumer ran (Tick fired)");
        assert_eq!(vn, 0, "rec {i}: On<Never> consumer skipped");
    }
    println!("scenario B (virtual fire + On gating, computed Ctx): PASS");
}

// ---------------------------------------------------------------------
// Scenario C: accumulator bundle + OnMeta meta bridge, every Ctx computed.
// Mirrors tests/gate2_meta_metrics.rs: an Always consumer appends a sentinel
// to the shared Accum<Mark>; an OnMeta<ScheduleEnd> hook reads engine-owned
// pass_count through the MetaRef-gated accessor and appends it. The computed
// EndWu Ctx must come out with MP = MetaRef<'frame> (keyed by the schedule)
// or `ctx.meta::<SchedulerMetrics>()` does not exist on the type.
// ---------------------------------------------------------------------

const CAP: usize = 8;

#[derive(Copy, Clone)]
struct Mark(u32);

type AccW = Cons<Accum<Mark>, Empty>;

struct AppenderWu;
impl BuilderInput for AppenderWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for AppenderWu {
    type Read = Empty;
    type Write = AccW;
    type Hint = Hints;
    // Computed: WAccum derives to AccPtrCons<'frame, Mark, AccPtrNil>, MP to
    // MetaNil. The consumer alias gains no meta accessor.
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // SAFETY: Mark reserved (CAP); this frame's appends stay in capacity.
        unsafe { ctx.accums().append::<Mark, _>(Mark(9)) };
    }
}

struct EndWu;
impl BuilderInput for EndWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl HasSchedule for EndWu {
    type Sched = OnMeta<ScheduleEnd>;
}
impl WorkUnit<OnMeta<ScheduleEnd>> for EndWu {
    type Read = Empty;
    type Write = AccW;
    type Hint = Hints;
    // Computed: the OnMeta schedule keys MP = MetaRef<'frame> through
    // MetaPtrFor, so the `meta` accessor exists on this Ctx and on no
    // consumer Ctx.
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write, <Self as HasSchedule>::Sched>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        let pc = ctx.meta::<SchedulerMetrics>().pass_count.get();
        // SAFETY: Mark reserved (CAP); this frame's appends stay in capacity.
        unsafe { ctx.accums().append::<Mark, _>(Mark(pc.0 as u32)) };
    }
}

fn scenario_c_accum_meta() {
    let provider = BumpProvider::<8192>::new();
    let mut scheduler = Scheduler::builder()
        .with(Accum::<Mark>::new())
        .with(AppenderWu)
        .with(EndWu)
        .build(store(provider), USize(CAP))
        .unwrap_or_else(|_| panic!("scenario C build should succeed"));

    let r1 = scheduler.run();
    assert!(matches!(r1, Outcome::Ok(())));
    let len1 = scheduler.__bindings().__len_cell().get().0;
    let base1 = scheduler.__bindings().__ptr().as_ptr();
    assert_eq!(len1, 2, "frame 1: appender and end hook both ran");
    // SAFETY: two records appended this frame into a reset buffer.
    let buf1 = unsafe { [core::ptr::read(base1.add(0)).0, core::ptr::read(base1.add(1)).0] };
    assert_eq!(buf1, [9, 1], "frame 1: appender(9) -> end hook reads pass_count = 1");

    let r2 = scheduler.run();
    assert!(matches!(r2, Outcome::Ok(())));
    let len2 = scheduler.__bindings().__len_cell().get().0;
    let base2 = scheduler.__bindings().__ptr().as_ptr();
    assert_eq!(len2, 2, "frame 2: appender and end hook both ran");
    // SAFETY: two records appended this frame into a reset buffer.
    let buf2 = unsafe { [core::ptr::read(base2.add(0)).0, core::ptr::read(base2.add(1)).0] };
    assert_eq!(buf2, [9, 2], "frame 2: appender(9) -> end hook reads pass_count = 2");
    println!("scenario C (accumulator + OnMeta meta bridge, computed Ctx): PASS");
}

fn main() {
    type_identity_probe();
    println!("type-identity probe (CtxFor == hand-spelled, all kinds + scheds): PASS");
    scenario_a_resource_column();
    scenario_b_virtual_gating();
    scenario_c_accum_meta();
    println!("all probes green");
}

// Keep the AccessSet import live for the alias bound context (EngineCtx's
// struct bounds check at every CtxFor use site).
#[allow(dead_code)]
fn _accessset_bound_witness<R: AccessSet, W: AccessSet>() {}

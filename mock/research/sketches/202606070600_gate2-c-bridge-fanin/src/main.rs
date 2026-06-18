//! GATE-2 Sketch C: bridge fan-in (the only cross-trunk data path).
//!
//! Roadmap r3 (`202606070200`) step G2-Nc. A bridge is a fan-in fiber that reads
//! from MULTIPLE parent trunks' write columns and runs after those parents reach
//! the required record range (spec `:745-746`, `:85`). Trunks otherwise share no
//! columns and never synchronise (Sketch B); the bridge is the explicit join.
//!
//! Sketches A and B had single-column reads within a trunk. This proves a bridge
//! WU whose Read AccessSet spans TWO parent trunks' columns (AX from trunk X, AY
//! from trunk Y) projects + dispatches correctly through the shipped RunFiber,
//! produces the correct fan-in combination CZ[i] = combine(AX[i], AY[i]), and
//! devirts (zero blr). Structure: phase 0 = trunk X (InX -> AX) + trunk Y
//! (InY -> AY); waist; phase 1 = the bridge trunk (reads AX + AY -> writes CZ).
//!
//! Single-threaded here: the fan-in READ is the feasibility question, not the
//! concurrency (Sketch B's domain). At full range after the waist the "required
//! record range" is the whole column; the head/tail progress-gated partial-range
//! bridge is the N-core refinement (rides on E4 progress counters). Outcome below.

#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, PtrNil};
use hilavitkutin::dispatch::fiber_run::RunFiber;
use hilavitkutin::dispatch::morsel::MorselRange;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::work_unit_values::{WuCons, WuNil};
use hilavitkutin_providers::ArenaColumnStorage;

struct FiberCons<F, Rest> {
    fiber: F,
    rest: Rest,
}
struct FiberNil;
trait RunTrunk<A, WL> {
    fn run(&self, bindings: &A, morsel: MorselRange);
}
impl<A> RunTrunk<A, Empty> for FiberNil {
    #[inline]
    fn run(&self, _b: &A, _m: MorselRange) {}
}
impl<A, F, Rest, FW, RestWL> RunTrunk<A, Cons<FW, RestWL>> for FiberCons<F, Rest>
where
    F: RunFiber<A, FW>,
    Rest: RunTrunk<A, RestWL>,
{
    #[inline]
    fn run(&self, bindings: &A, morsel: MorselRange) {
        self.fiber.run(bindings, morsel);
        self.rest.run(bindings, morsel);
    }
}
#[inline(never)]
fn run_one_trunk<A, T, WL>(bindings: &A, trunk: &T, morsel: MorselRange)
where
    T: RunTrunk<A, WL>,
{
    trunk.run(bindings, morsel);
}

const M1: u32 = 2654435761;
const M2: u32 = 2246822519;
#[inline(always)]
fn fx(i: u32) -> u32 {
    i.wrapping_mul(M1)
}
#[inline(always)]
fn fy(i: u32) -> u32 {
    i.wrapping_mul(M2).wrapping_add(1)
}
// The fan-in combine: depends on BOTH parents (the point of a bridge).
#[inline(always)]
fn combine(ax: u32, ay: u32) -> u32 {
    (ax ^ ay.rotate_left(7)).wrapping_add(ax.wrapping_mul(ay))
}

#[derive(Copy, Clone)]
struct InX(u32);
#[derive(Copy, Clone)]
struct AX(u32);
#[derive(Copy, Clone)]
struct InY(u32);
#[derive(Copy, Clone)]
struct AY(u32);
#[derive(Copy, Clone)]
struct CZ(u32);
type One<T> = Cons<Column<T>, Empty>;
// The bridge's read set: TWO columns, one from each parent trunk.
type TwoRead = Cons<Column<AX>, Cons<Column<AY>, Empty>>;

// Trunk X parent: InX -> AX.
#[derive(Copy, Clone)]
struct SX;
impl BuilderInput for SX {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for SX {
    type Read = One<InX>;
    type Write = One<AX>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'f> =
        EngineCtx<'f, One<InX>, One<AX>, PtrNil, ColPtrCons<InX, ColPtrNil>, ColPtrCons<AX, ColPtrNil>>;
    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        ctx.each().run(|i| {
            // SAFETY: InX host-populated; AX reserved + exclusively written; morsel-bounded.
            let v = unsafe { ctx.reader().read::<InX, _>(i) };
            unsafe { ctx.writer().write::<AX, _>(i, AX(fx(v.0))) };
        });
    }
}

// Trunk Y parent: InY -> AY.
#[derive(Copy, Clone)]
struct SY;
impl BuilderInput for SY {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for SY {
    type Read = One<InY>;
    type Write = One<AY>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'f> =
        EngineCtx<'f, One<InY>, One<AY>, PtrNil, ColPtrCons<InY, ColPtrNil>, ColPtrCons<AY, ColPtrNil>>;
    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        ctx.each().run(|i| {
            let v = unsafe { ctx.reader().read::<InY, _>(i) };
            unsafe { ctx.writer().write::<AY, _>(i, AY(fy(v.0))) };
        });
    }
}

// THE BRIDGE: fan-in reading BOTH AX (trunk X) and AY (trunk Y), writing CZ.
// Its Read AccessSet is a 2-element cons-list spanning two parent trunks; the
// read-column projection ColPtrCons<AX, ColPtrCons<AY, ColPtrNil>> carries both.
#[derive(Copy, Clone)]
struct BridgeZ;
impl BuilderInput for BridgeZ {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for BridgeZ {
    type Read = TwoRead;
    type Write = One<CZ>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'f> = EngineCtx<
        'f,
        TwoRead,
        One<CZ>,
        PtrNil,
        ColPtrCons<AX, ColPtrCons<AY, ColPtrNil>>,
        ColPtrCons<CZ, ColPtrNil>,
    >;
    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        ctx.each().run(|i| {
            // SAFETY: AX, AY both written by the parent trunks (complete before this
            // bridge via the waist); CZ reserved + exclusive; morsel-bounded. Reading
            // two columns from one ctx is the fan-in.
            let ax = unsafe { ctx.reader().read::<AX, _>(i) };
            let ay = unsafe { ctx.reader().read::<AY, _>(i) };
            unsafe { ctx.writer().write::<CZ, _>(i, CZ(combine(ax.0, ay.0))) };
        });
    }
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
        unsafe { base.add(aligned) }
    }
    unsafe fn deallocate(&self, _p: *mut u8, _l: USize) {}
    unsafe fn protect(&self, _p: *mut u8, _l: USize, _r: Bool, _w: Bool) {}
}
fn store<M: MemoryProviderApi>(p: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(p)
}

const N: usize = 256;

fn main() {
    let provider = BumpProvider::<262144>::new();
    // Register columns CZ, AY, InY, AX, InX (prepend: InX head). Units SX, SY,
    // BridgeZ: BridgeZ reads AX (SX writes) + AY (SY writes), so it follows both
    // in topological registration order, which build validates.
    let sched = Scheduler::builder()
        .with(Column::<CZ>::new())
        .with(Column::<AY>::new())
        .with(Column::<InY>::new())
        .with(Column::<AX>::new())
        .with(Column::<InX>::new())
        .with(SX)
        .with(SY)
        .with(BridgeZ)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("engine build should succeed"));

    // bindings prepend-reverse: InX -> AX -> InY -> AY -> CZ.
    let inx = sched.__bindings().__ptr().as_ptr() as *mut InX;
    let iny = sched.__bindings().__tail().__tail().__ptr().as_ptr() as *mut InY;
    for i in 0..N {
        // SAFETY: InX, InY each reserved for N records; storage alive; one write each.
        unsafe { *inx.add(i) = InX(i as u32) };
        unsafe { *iny.add(i) = InY(i as u32) };
    }

    let bindings = sched.__bindings();
    let morsel = MorselRange::new(USize(0), USize(N));

    // Phase 0: the two parent trunks (sequential here; concurrency is Sketch B).
    let trunk_x = FiberCons { fiber: WuCons { head: SX, tail: WuNil }, rest: FiberNil };
    let trunk_y = FiberCons { fiber: WuCons { head: SY, tail: WuNil }, rest: FiberNil };
    run_one_trunk(bindings, &trunk_x, morsel);
    run_one_trunk(bindings, &trunk_y, morsel);
    // Waist: both parents complete (here sequential ordering is the waist).
    // Phase 1: the bridge trunk, fanning in AX + AY.
    let bridge = FiberCons { fiber: WuCons { head: BridgeZ, tail: WuNil }, rest: FiberNil };
    run_one_trunk(bindings, &bridge, morsel);

    // Verify the fan-in: CZ[i] = combine(fx(i), fy(i)), depending on BOTH parents.
    let cz_base =
        sched.__bindings().__tail().__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
    // SAFETY: CZ reserved for N records; storage alive; the bridge wrote every record.
    let cz = unsafe { core::slice::from_raw_parts(cz_base, N) };
    for i in 0..N {
        assert_eq!(
            cz[i],
            combine(fx(i as u32), fy(i as u32)),
            "CZ[{i}] (bridge fan-in of AX trunk X + AY trunk Y)"
        );
    }

    println!(
        "WORKS: bridge fiber fanned in two parent trunks' columns (AX from trunk X, AY from \
         trunk Y) over {N} records, CZ[i] = combine(AX[i], AY[i]) correct. The bridge's 2-column \
         Read AccessSet projects + dispatches through the shipped RunFiber. objdump run_one_trunk \
         (bridge mono) for zero blr."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28, release fat LTO cgu=1).
//
// The bridge fiber fanned in two parent trunks' columns (AX from trunk X, AY from
// trunk Y) over 256 records; CZ[i] = combine(AX[i], AY[i]) correct for all i, a
// value that depends on BOTH parents (so a missed parent or wrong projection would
// fail the assertion). The bridge's 2-column Read AccessSet
// (Cons<Column<AX>, Cons<Column<AY>, Empty>>) projects through the shipped
// RunFiber via ColPtrCons<AX, ColPtrCons<AY, ColPtrNil>>, and both reads resolve
// from the one EngineCtx. All three run_one_trunk monos (trunk X, trunk Y, the
// bridge) objdump to zero blr; the bridge's multi-column read does not introduce
// any indirect call.
//
// SETTLES (roadmap r3 G2-Nc): the bridge, the only cross-trunk data path, composes
// with the trunk model. A fan-in fiber reading multiple parent trunks' columns
// dispatches devirt-clean through the same RunFiber walk. The real build runs the
// bridge after its parent trunks reach the required record range; at full range
// after the waist (here, sequential ordering) that is "after the parents complete".
// The N-core refinement (the bridge gated on a progress counter so it starts when
// parents reach a partial record range, :746) rides on E4 progress counters and is
// not this feasibility question. Together with A (the nest), B (concurrent
// zero-sync trunks), and B2 (the ~3x speedup), the GATE-2 trunk-parallelism model
// is sketch-proven end to end.
// ---------------------------------------------------------------------

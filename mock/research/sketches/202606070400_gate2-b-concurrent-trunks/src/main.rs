//! GATE-2 Sketch B (THE keystone): concurrent column-disjoint trunks, zero sync.
//!
//! Roadmap r3 (`202606070200`) step G2-Nb. The canonical parallelism is isolated
//! column-disjoint trunks, one per core, with ZERO synchronisation between trunks
//! (spec `:741-742`, `:769`): sibling trunks share no write column, so nothing
//! coordinates them during a phase. The only cross-trunk join is the waist barrier
//! between phases (`:772`, `:1619-1633`) and the bridge fan-in (`:745`, Sketch C).
//!
//! This proves that integration end to end:
//!   - Phase 0 has two column-disjoint trunks: trunk X (InX -> AX) and trunk Y
//!     (InY -> AY). AX and AY are disjoint write columns.
//!   - The two trunks run CONCURRENTLY on two real threads, each walking the real
//!     `RunTrunk` -> shipped `RunFiber` dispatch over the shared scheduler bindings,
//!     touching NO shared atomic during the phase (the disjoint write columns are
//!     the license; the only thing shared is the immutable binding structure).
//!   - They synchronise only at the WAIST via the SHIPPED `phase_barrier_arrive` /
//!     `phase_barrier_observe` over a real `PoolFrame` (expected = 2 arrivers).
//!   - Phase 1 (trunk Z: AX -> CZ) runs after the waist, reading phase 0's output.
//!   - Output is bit-identical to the single-core run (Sketch A, the E6 oracle),
//!     partitioned here by TRUNK (the real parallel unit), not by record range.
//!   - Each trunk's dispatch walk objdumps to zero blr.
//!
//! std threads stand in for the pre-allocated pool's workers (the shipped pool
//! spawns once and parks between frames; the CONCURRENCY shape, not the spawn
//! mechanism, is what this proves). One waist episode: the multi-episode barrier
//! reset / generation bit is E3 (deferred), so this does not reset the counter.
//! Outcome at bottom.

#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize};

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, PtrNil};
use hilavitkutin::dispatch::fiber_run::RunFiber;
use hilavitkutin::dispatch::morsel::MorselRange;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin::thread::{phase_barrier_arrive, phase_barrier_observe};
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::platform::{MemoryProviderApi, PoolFrame};
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::work_unit_values::{WuCons, WuNil};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Maybe;

// ---- trunk = list of fibers (Sketch A's RunTrunk over the shipped RunFiber) ----
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

// Isolated per-trunk dispatch symbol (objdump target): zero blr is the bar.
#[inline(never)]
fn run_one_trunk<A, T, WL>(bindings: &A, trunk: &T, morsel: MorselRange)
where
    T: RunTrunk<A, WL>,
{
    trunk.run(bindings, morsel);
}

// Share the immutable scheduler bindings across worker threads. SAFETY: the two
// trunks write DISJOINT columns (AX vs AY) and read disjoint inputs (InX vs InY),
// so no column cell is aliased; the binding structure itself is read-only shared
// (the ColumnPtr values are never mutated during dispatch). This is the spec's
// zero-sync-between-trunks guarantee (:742) made concrete.
struct SyncBind<'a, A>(&'a A);
// Manual Copy/Clone: derive would add a spurious `A: Clone` bound (A is the
// binding cons-list, not Clone), which would defeat Copy and force a move.
impl<'a, A> Clone for SyncBind<'a, A> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, A> Copy for SyncBind<'a, A> {}
unsafe impl<'a, A> Send for SyncBind<'a, A> {}
unsafe impl<'a, A> Sync for SyncBind<'a, A> {}

// Waist: arrive at the shipped barrier, then spin until all `expected` arrivers
// have arrived (single episode; no reset, which is E3). Uses the SHIPPED
// phase_barrier_arrive + phase_barrier_observe over a real PoolFrame.
fn waist<const C: usize, const P: usize>(pool: &PoolFrame<C, P>, expected: USize) {
    let _ = phase_barrier_arrive(pool, expected);
    loop {
        match phase_barrier_observe(pool) {
            Maybe::Is(v) if v.0 >= expected.0 => break,
            _ => core::hint::spin_loop(),
        }
    }
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
#[inline(always)]
fn fz(a: u32) -> u32 {
    (a >> 13) ^ a
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
            // SAFETY: InX host-populated; AX reserved + exclusively written by this
            // trunk (no other trunk writes AX); morsel-bounded.
            let v = unsafe { ctx.reader().read::<InX, _>(i) };
            unsafe { ctx.writer().write::<AX, _>(i, AX(fx(v.0))) };
        });
    }
}

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
            // SAFETY: InY host-populated; AY reserved + exclusively written by this
            // trunk (disjoint from trunk X's AX); morsel-bounded.
            let v = unsafe { ctx.reader().read::<InY, _>(i) };
            unsafe { ctx.writer().write::<AY, _>(i, AY(fy(v.0))) };
        });
    }
}

#[derive(Copy, Clone)]
struct SZ;
impl BuilderInput for SZ {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for SZ {
    type Read = One<AX>;
    type Write = One<CZ>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'f> =
        EngineCtx<'f, One<AX>, One<CZ>, PtrNil, ColPtrCons<AX, ColPtrNil>, ColPtrCons<CZ, ColPtrNil>>;
    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        ctx.each().run(|i| {
            // SAFETY: AX written by phase 0 (complete before phase 1 via the waist);
            // CZ reserved + exclusive; morsel-bounded.
            let a = unsafe { ctx.reader().read::<AX, _>(i) };
            unsafe { ctx.writer().write::<CZ, _>(i, CZ(fz(a.0))) };
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

const N: usize = 4096;

fn main() {
    let provider = BumpProvider::<1048576>::new();
    let sched = Scheduler::builder()
        .with(Column::<CZ>::new())
        .with(Column::<AY>::new())
        .with(Column::<AX>::new())
        .with(Column::<InY>::new())
        .with(Column::<InX>::new())
        .with(SX)
        .with(SY)
        .with(SZ)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("engine build should succeed"));

    // bindings prepend-reverse: InX -> InY -> AX -> AY -> CZ.
    let inx_base = sched.__bindings().__ptr().as_ptr() as *mut InX;
    let iny_base = sched.__bindings().__tail().__ptr().as_ptr() as *mut InY;
    for i in 0..N {
        // SAFETY: InX, InY each reserved for N records; storage alive; one write each.
        unsafe { *inx_base.add(i) = InX(i as u32) };
        unsafe { *iny_base.add(i) = InY(i as u32) };
    }

    let bind = SyncBind(sched.__bindings());
    let morsel = MorselRange::new(USize(0), USize(N));

    // Real PoolFrame for the shipped waist barrier. 2 cores, 2 phases.
    let slots: [AtomicUsize; 4] = core::array::from_fn(|_| AtomicUsize::new(0));
    let pool: PoolFrame<2, 2> = PoolFrame {
        shutdown: AtomicBool::new(false),
        phase_arrived: AtomicU32::new(0),
        predicted_wait_ns: core::array::from_fn(|_| AtomicU32::new(0)),
        idle_accumulator: core::array::from_fn(|_| AtomicU64::new(0)),
        park_count: core::array::from_fn(|_| AtomicU64::new(0)),
        progress_slots: NonNull::new(slots.as_ptr() as *mut AtomicUsize).unwrap(),
        progress_slot_count: USize(4),
        _arena: PhantomData,
    };

    // Phase 0: two column-disjoint trunks run CONCURRENTLY on two threads, then
    // both arrive at the shipped waist barrier. No atomic is touched between the
    // trunks during the walk; the disjoint write columns make it race-free.
    std::thread::scope(|s| {
        let bind_y = bind;
        let pool_ref = &pool;
        s.spawn(move || {
            // Trunk Y on a worker thread: InY -> AY.
            let trunk_y =
                FiberCons { fiber: WuCons { head: SY, tail: WuNil }, rest: FiberNil };
            run_one_trunk(bind_y.0, &trunk_y, morsel);
            waist(pool_ref, USize(2));
        });
        // Trunk X on the main thread: InX -> AX. Concurrent with trunk Y, zero sync.
        let trunk_x = FiberCons { fiber: WuCons { head: SX, tail: WuNil }, rest: FiberNil };
        run_one_trunk(bind.0, &trunk_x, morsel);
        waist(&pool, USize(2));
    });

    // Waist passed: phase 0 complete on both trunks. Phase 1: trunk Z (AX -> CZ).
    let trunk_z = FiberCons { fiber: WuCons { head: SZ, tail: WuNil }, rest: FiberNil };
    run_one_trunk(bind.0, &trunk_z, morsel);

    // Verify N-core output == 1-core oracle (Sketch A's values), trunk-partitioned.
    let ax_base = sched.__bindings().__tail().__tail().__ptr().as_ptr() as *const u32;
    let ay_base = sched.__bindings().__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
    let cz_base =
        sched.__bindings().__tail().__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
    // SAFETY: AX, AY, CZ each reserved for N records; storage alive; each written once.
    let ax = unsafe { core::slice::from_raw_parts(ax_base, N) };
    let ay = unsafe { core::slice::from_raw_parts(ay_base, N) };
    let cz = unsafe { core::slice::from_raw_parts(cz_base, N) };
    for i in 0..N {
        assert_eq!(ax[i], fx(i as u32), "AX[{i}] (trunk X, worker-concurrent)");
        assert_eq!(ay[i], fy(i as u32), "AY[{i}] (trunk Y, worker-concurrent)");
        assert_eq!(cz[i], fz(fx(i as u32)), "CZ[{i}] (trunk Z, phase 1, reads phase-0 AX)");
    }

    println!(
        "WORKS: two column-disjoint trunks (X: InX->AX, Y: InY->AY) ran CONCURRENTLY on two \
         threads with zero sync between them over {N} records, synchronised only at the shipped \
         phase_barrier_arrive waist (expected=2), then phase 1 trunk Z (AX->CZ) ran. Output \
         bit-identical to the 1-core oracle = N-core == 1-core, trunk-partitioned. objdump \
         run_one_trunk for zero blr."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28, release, fat LTO, cgu=1).
//
// Two column-disjoint trunks (X: InX->AX on the main thread, Y: InY->AY on a
// spawned worker) ran CONCURRENTLY over 4096 records with ZERO synchronisation
// between them during the walk (no shared atomic touched; the disjoint write
// columns AX / AY make it race-free, the spec :742 guarantee). They synchronised
// only at the waist via the SHIPPED phase_barrier_arrive + phase_barrier_observe
// over a real PoolFrame<2, 2> (expected = 2 arrivers). After the waist, phase 1's
// trunk Z (AX -> CZ) ran. All output bit-identical to the single-core oracle
// (Sketch A): AX[i]=fx(i), AY[i]=fy(i), CZ[i]=fz(fx(i)). N-core == 1-core,
// partitioned by TRUNK (the real parallel unit), not by record range.
//
// Devirt: all three `run_one_trunk` monos (trunk X, Y, Z) objdump to ZERO blr.
// The per-trunk RunTrunk -> shipped RunFiber walk stays fully devirtualised under
// concurrency; running it on a worker thread changes nothing about the codegen.
//
// Race check: 20/20 stress runs passed (the assertion is exact equality to the
// oracle, so a partition overlap or a missed write would fail it).
//
// SETTLES (roadmap r3 G2-Nb, THE keystone): the canonical parallelism works as
// specified. Column-disjoint trunks run concurrently, one per thread/core, with
// zero sync between them; the only cross-trunk join is the shipped waist barrier
// between phases. This replaces the earlier (wrong) record-range-partition
// keystone. The real build: assign_cores pins each trunk's sub-carrier to a core
// (re-point synthesise_core_programs off fibers/RecordRange::Full onto trunks),
// the pool's spawned-once workers run run_one_trunk per pinned trunk, and the
// waist barrier sits between phases. Bridge fan-in (a trunk Z reading TWO parent
// trunks' columns after both reach a record range) is Sketch C. Multi-episode
// barrier reset / generation bit (many phases, many frames) is E3; this proved one
// waist episode, which is all the keystone needs.
// ---------------------------------------------------------------------

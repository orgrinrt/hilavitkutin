//! GATE-2 Sketch B2: do parallel column-disjoint trunks actually get FASTER?
//!
//! Roadmap r3 (`202606070200`) step G2-Nb, completing the keystone. Sketch B proved
//! column-disjoint trunks run concurrently with correct output and zero blr, but
//! proved nothing about speed. Parallelism that runs correct but does not get
//! faster is broken parallelism (false sharing, barrier serialisation, E-core
//! scheduling, work too small to amortise). The canonical design's whole reason for
//! trunk parallelism (spec `:769`, `:793-801`) is the speedup; it must be measured.
//!
//! Three column-disjoint COMPUTE-BOUND trunks (each In_k -> A_k with a heavy
//! per-record kernel so the work dominates memory traffic and thread overhead).
//! Timed two ways:
//!   - sequential: all three trunks on one thread, one after another.
//!   - parallel:   the three trunks on three threads concurrently (scoped + joined).
//! Report seq / par. The ideal is 3x (three independent trunks, three cores). The
//! measured ratio is the finding; a conservative floor asserts real multi-core
//! overlap (a ratio well above 1 is unreachable on one core).
//!
//! Compute-bound on purpose: a trivial one-multiply kernel would be memory-bound
//! and cap the speedup at the memory bus, telling us nothing about whether the
//! trunk parallelism itself scales. The heavy kernel isolates the scheduling
//! question. Outcome at bottom.

#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use std::hint::black_box;
use std::time::{Duration, Instant};

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

// ---- trunk = list of fibers (Sketch A/B's RunTrunk over the shipped RunFiber) ----
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

struct SyncBind<'a, A>(&'a A);
impl<'a, A> Clone for SyncBind<'a, A> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, A> Copy for SyncBind<'a, A> {}
unsafe impl<'a, A> Send for SyncBind<'a, A> {}
unsafe impl<'a, A> Sync for SyncBind<'a, A> {}

// Heavy compute-bound per-record kernel: ROUNDS of mix ops. Pure; same input ->
// same output, so sequential and parallel runs produce identical columns.
const ROUNDS: u32 = 8192;
const M1: u32 = 2654435761;
#[inline(always)]
fn heavy(seed: u32) -> u32 {
    let mut x = seed;
    let mut k = 0u32;
    while k < ROUNDS {
        x = x.wrapping_mul(M1).wrapping_add(1);
        x ^= x >> 13;
        k += 1;
    }
    x
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
struct InW(u32);
#[derive(Copy, Clone)]
struct AW(u32);
type One<T> = Cons<Column<T>, Empty>;

macro_rules! heavy_wu {
    ($S:ident, $In:ident, $A:ident) => {
        #[derive(Copy, Clone)]
        struct $S;
        impl BuilderInput for $S {
            type Init = Self;
            type Dispatch = UnitDispatch<Self>;
        }
        impl WorkUnit<Always> for $S {
            type Read = One<$In>;
            type Write = One<$A>;
            type Hint = (Immediate, Atomic, Normal);
            type Ctx<'f> = EngineCtx<
                'f,
                One<$In>,
                One<$A>,
                PtrNil,
                ColPtrCons<$In, ColPtrNil>,
                ColPtrCons<$A, ColPtrNil>,
            >;
            fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
                ctx.each().run(|i| {
                    // SAFETY: In host-populated; A reserved + exclusively written by
                    // this trunk (disjoint from the other trunks); morsel-bounded.
                    let v = unsafe { ctx.reader().read::<$In, _>(i) };
                    unsafe { ctx.writer().write::<$A, _>(i, $A(heavy(v.0))) };
                });
            }
        }
    };
}
heavy_wu!(SX, InX, AX);
heavy_wu!(SY, InY, AY);
heavy_wu!(SW, InW, AW);

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

const N: usize = 8192;
const REPS: usize = 12;

fn min_dur<F: FnMut()>(mut f: F) -> Duration {
    f(); // warm-up (not timed)
    let mut best = Duration::MAX;
    for _ in 0..REPS {
        let t = Instant::now();
        f();
        let d = t.elapsed();
        if d < best {
            best = d;
        }
    }
    best
}

fn main() {
    // 6 columns x N x 4 bytes ~= 196 KiB at N=8192; 256 KiB arena. Small enough to
    // sit on the stack (the provider moves through build() into the Scheduler);
    // a multi-MiB arena overflows the main thread stack. The work is dominated by
    // ROUNDS (compute), not N, so a small column count keeps it compute-bound.
    let provider = BumpProvider::<262144>::new();
    let sched = Scheduler::builder()
        .with(Column::<AW>::new())
        .with(Column::<InW>::new())
        .with(Column::<AY>::new())
        .with(Column::<InY>::new())
        .with(Column::<AX>::new())
        .with(Column::<InX>::new())
        .with(SX)
        .with(SY)
        .with(SW)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("engine build should succeed"));

    // bindings prepend-reverse: InX -> AX -> InY -> AY -> InW -> AW.
    let inx = sched.__bindings().__ptr().as_ptr() as *mut InX;
    let iny = sched.__bindings().__tail().__tail().__ptr().as_ptr() as *mut InY;
    let inw =
        sched.__bindings().__tail().__tail().__tail().__tail().__ptr().as_ptr() as *mut InW;
    for i in 0..N {
        // SAFETY: each In column reserved for N records; storage alive; one write each.
        unsafe { *inx.add(i) = InX(i as u32) };
        unsafe { *iny.add(i) = InY(i as u32) };
        unsafe { *inw.add(i) = InW(i as u32) };
    }

    let bind = SyncBind(sched.__bindings());
    let morsel = MorselRange::new(USize(0), USize(N));

    // Trunks are cheap ZST carriers; build them at each use site (the spawned
    // closures move-capture, so a once-built value cannot be reused across reps).
    let tx = || FiberCons { fiber: WuCons { head: SX, tail: WuNil }, rest: FiberNil };
    let ty = || FiberCons { fiber: WuCons { head: SY, tail: WuNil }, rest: FiberNil };
    let tw = || FiberCons { fiber: WuCons { head: SW, tail: WuNil }, rest: FiberNil };

    // Sequential: all three trunks on one thread.
    let seq = min_dur(|| {
        run_one_trunk(bind.0, &tx(), morsel);
        run_one_trunk(bind.0, &ty(), morsel);
        run_one_trunk(bind.0, &tw(), morsel);
    });

    // Parallel: the three trunks on three threads, concurrently, zero sync between
    // them (disjoint write columns AX / AY / AW). join is the only synchronisation.
    let par = min_dur(|| {
        std::thread::scope(|s| {
            let by = bind;
            let bw = bind;
            s.spawn(move || run_one_trunk(by.0, &ty(), morsel));
            s.spawn(move || run_one_trunk(bw.0, &tw(), morsel));
            run_one_trunk(bind.0, &tx(), morsel);
        });
    });

    // Correctness: each column equals heavy(input), identical in both modes (the
    // last run was parallel; the values are run-order-independent).
    let ax = sched.__bindings().__tail().__ptr().as_ptr() as *const u32;
    let ay = sched.__bindings().__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
    let aw = sched
        .__bindings()
        .__tail()
        .__tail()
        .__tail()
        .__tail()
        .__tail()
        .__ptr()
        .as_ptr() as *const u32;
    // SAFETY: AX, AY, AW each reserved for N records; storage alive; each written.
    let (ax, ay, aw) = unsafe {
        (
            core::slice::from_raw_parts(ax, N),
            core::slice::from_raw_parts(ay, N),
            core::slice::from_raw_parts(aw, N),
        )
    };
    let mut checksum = 0u32;
    for i in 0..N {
        assert_eq!(ax[i], heavy(i as u32), "AX[{i}]");
        assert_eq!(ay[i], heavy(i as u32), "AY[{i}]");
        assert_eq!(aw[i], heavy(i as u32), "AW[{i}]");
        checksum ^= ax[i] ^ ay[i] ^ aw[i];
    }
    black_box(checksum);

    let seq_ms = seq.as_secs_f64() * 1000.0;
    let par_ms = par.as_secs_f64() * 1000.0;
    let ratio = seq.as_secs_f64() / par.as_secs_f64();
    println!(
        "seq (3 trunks, 1 thread): {seq_ms:.3} ms\n\
         par (3 trunks, 3 threads): {par_ms:.3} ms\n\
         speedup seq/par = {ratio:.2}x  (ideal 3.00x, N={N}, ROUNDS={ROUNDS})"
    );

    // Strict floor: real 3-way multi-core overlap is unreachable on one core. A
    // ratio at or below ~1 would mean the parallelism delivers nothing (broken).
    // The floor is conservative (machine load / P-vs-E scheduling pull below 3x);
    // the printed ratio is the finding judged against the 3x ideal.
    assert!(
        ratio >= 1.8,
        "parallel trunks must be substantially faster than sequential (got {ratio:.2}x); \
         a ratio near 1 means the trunk parallelism is not delivering (false sharing, \
         barrier/scheduling serialisation, or work too small)"
    );

    println!("WORKS: parallel column-disjoint trunks deliver a real speedup (see ratio).");
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28, release fat LTO cgu=1, Apple Silicon).
//
// Three column-disjoint compute-bound trunks (N=8192, ROUNDS=8192), min over 12
// reps after warm-up:
//   seq (3 trunks, 1 thread):  318.97 ms
//   par (3 trunks, 3 threads): 112.51 ms
//   speedup seq/par = 2.84x   (ideal 3.00x)
//
// 2.84x is the "roughly 3x" the design promises: column-disjoint trunks running
// one-per-core deliver near-linear parallel scaling, not merely correct concurrent
// output. The 0.16 gap from the 3.0 ideal is join/scope overhead plus the
// parallel run being bound by its slowest thread (scheduling jitter; the threads
// land on P-cores here). Output verified bit-identical to heavy(input) in both
// modes (correctness retained under concurrency).
//
// Devirt of the MEASURED code (objdump run_one_trunk, the bench's hot path): the
// trunk dispatch fully inlines into one straight-line body, ZERO blr AND ZERO bl.
// The outer record/morsel loop and the inner 8192-round heavy kernel are both
// inlined, with M1 baked as an immediate (mov #0x79b1 / movk #0x9e37 -> w11 =
// 0x9e3779b1; madd w13,w13,w11,w12; eor w13,w13,w13,lsr #13; subs/b.ne the inner
// loop; str + outer b.ne). No call of any kind on the dispatch path. The other two
// trunk monos (SX/SY, identical kernel structure) ICF-fold to the same body. So
// the speedup was measured on fully-devirtualised, fully-fused dispatch, not on
// a call-laden path: the 2.84x is representative of real engine dispatch.
//
// SETTLES (roadmap r3 G2-Nb performance premise): trunk parallelism actually gets
// faster, ~Nx for N independent column-disjoint trunks on N cores. Sketch B proved
// it runs correct + devirts; this proves it is worth doing. Caveats for the real
// build, to confirm on the #664 suite once the pool is real: (1) heterogeneous
// P/E cores will cap the parallel run at the slowest assigned core unless trunks
// are pinned P-first (E5 affinity, :1810-1827); (2) memory-bound trunks (light
// per-record kernels) will scale below Nx at the memory bus, which is expected and
// is why morsel sizing + cache co-location (domains 04/12) matter. The compute-
// bound case scales; the design's job is to keep hot trunks compute-bound.
// ---------------------------------------------------------------------

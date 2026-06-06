//! Sketch (HILA-RUNTIME C2 / #340, Phase D): the Approach-E schedule-mega
//! single-core body. Two open questions past the `202606052130` body probe
//! (which proved the column-capable inline walk devirtualizes for a branching
//! DAG, order-agnostic, in one whole-range pass):
//!
//!   1. MORSEL-OUTER. Wrap the inline walk in a morsel loop, calling the same
//!      walk per morsel sub-range. `EngineCtx::project` already takes the
//!      morsel and `each()` iterates exactly it, so morsel-outer is "call the
//!      walk with a sub-range". Does it still devirtualize when the walk is
//!      inside a runtime morsel loop, not run once over the whole range?
//!   2. MULTI-PHASE. A single-core phase boundary is a sequence point: all
//!      records of phase P complete before phase P+1 starts. Expressed in-body
//!      that is two sequenced per-phase morsel loops in one fn. Does a two-phase
//!      body devirtualize end to end, no indirect dispatch at any boundary?
//!
//! Approach E (consolidation spec L1551-1615, "schedule mega, all trunks in one
//! fn", preferred for >10K records) is the chosen single-core shape: one
//! monomorphised body walks the schedule, fiber/phase boundaries and morsel
//! sizes are in-body control flow plus compile-time constants ("compiled
//! per-core dispatch", L1596-1613). No type-level fiber partition on one core.
//!
//! Workload: phase 0 = the diamond (BranchX: In->Xv, BranchY: In->Yv, JoinZ:
//! {Xv,Yv}->Zv); phase 1 = NormW (Zv->Wv). Both run morsel-outer inside one
//! `#[inline(never)]` `run_schedule_mega` whose only bounds are the two
//! `RunFiberCol` bounds (witnesses stay free generic params, inferred at the
//! call site, which is the fix for the placeholder-witness problem the body
//! probe hit when it tried a named helper). The morsel chunk is a const generic
//! so it bakes as an immediate (spec "morsel constants").
//!
//! Phase 1 here is element-wise on Zv; step-8 grouping would fuse an
//! element-wise dependent into phase 0. It sits in a separate phase only to
//! exercise the multi-phase body structure (two sequenced morsel loops), the
//! codegen shape step-8 emits for a GENUINE barrier (reduction / scan). The
//! devirt result is independent of whether this phase 1 needs the barrier. A
//! genuine cross-record phase 1 needs a cross-morsel read (awkward through the
//! morsel-relative each()/reader) or an accumulator (the non-nil AccumProject
//! tie, the deferred SRC-time residual), so it is out of scope here.
//!
//! Bench: at scale, does the within-level RCM order (phase 0 BranchX-first vs
//! BranchY-first, both topo-valid) move single-core runtime? The branches touch
//! disjoint columns, so the order changes only which disjoint column is written
//! first per morsel. The answer steers whether the SRC slice realises the RCM
//! within-level reorder or accepts validated registration order on one core.
//!
//! Outcome recorded at the bottom of this file.

#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use std::time::Instant;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrNil, AccumProject, ColPtrCons, ColPtrNil, ColProject, EngineCtx, Project, PtrNil,
};
use hilavitkutin::dispatch::fiber_walk::{WuCons, WuNil};
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
use hilavitkutin_providers::ArenaColumnStorage;

// ---------------------------------------------------------------------
// The column-capable inline-recursive fiber walk (verbatim from the
// 202606051601 / 202606052130 sketches). Constructs each unit's EngineCtx
// INLINE rather than erasing to a fiber_shim fn pointer, so the walk
// monomorphises into one straight-line body with no stored fn pointer.
// ---------------------------------------------------------------------
trait RunFiberCol<A, Witnesses> {
    fn run(&self, bindings: &A, morsel: MorselRange);
}

impl<A> RunFiberCol<A, Empty> for WuNil {
    #[inline]
    fn run(&self, _bindings: &A, _morsel: MorselRange) {}
}

impl<A, W, Tail, RIdx, RCIdx, WCIdx, WAIdx, WTail>
    RunFiberCol<A, Cons<(RIdx, RCIdx, WCIdx, WAIdx), WTail>> for WuCons<W, Tail>
where
    W: WorkUnit,
    A: Project<<W as WorkUnit>::Read, RIdx>,
    A: ColProject<<W as WorkUnit>::Read, RCIdx>,
    A: ColProject<<W as WorkUnit>::Write, WCIdx>,
    for<'f> A: AccumProject<'f, <W as WorkUnit>::Write, WAIdx, Out = AccPtrNil>,
    for<'f> W: WorkUnit<
        Ctx<'f> = EngineCtx<
            'f,
            <W as WorkUnit>::Read,
            <W as WorkUnit>::Write,
            <A as Project<<W as WorkUnit>::Read, RIdx>>::Out,
            <A as ColProject<<W as WorkUnit>::Read, RCIdx>>::Out,
            <A as ColProject<<W as WorkUnit>::Write, WCIdx>>::Out,
            AccPtrNil,
        >,
    >,
    Tail: RunFiberCol<A, WTail>,
{
    #[inline]
    fn run(&self, bindings: &A, morsel: MorselRange) {
        let ctx: <W as WorkUnit>::Ctx<'_> =
            EngineCtx::project::<A, A, RIdx, RCIdx, WCIdx, WAIdx>(bindings, bindings, morsel);
        self.head.execute(&ctx);
        self.tail.run(bindings, morsel);
    }
}

// ---------------------------------------------------------------------
// The schedule-mega body: two phases, each morsel-outer, in ONE fn. The
// phase boundary is the sequence point between the two morsel loops. The
// morsel chunk is a const generic, so it bakes as an immediate. The only
// bounds are the two RunFiberCol bounds; the witnesses are free generic
// params inferred at the call site. #[inline(never)] gives a clean disasm
// target.
// ---------------------------------------------------------------------
#[inline(never)]
fn run_schedule_mega<const MORSEL: usize, A, P0, W0, P1, W1>(
    phase0: &P0,
    phase1: &P1,
    bindings: &A,
    n: USize,
) where
    P0: RunFiberCol<A, W0>,
    P1: RunFiberCol<A, W1>,
{
    // phase 0: every morsel completes before phase 1 begins (the barrier).
    let mut s = 0usize;
    while s < n.0 {
        let len = if s + MORSEL <= n.0 { MORSEL } else { n.0 - s };
        phase0.run(bindings, MorselRange::new(USize(s), USize(len)));
        s += MORSEL;
    }
    // phase boundary (sequence point): all of Zv is materialised here.
    // phase 1: reads Zv, writes Wv, morsel-outer.
    let mut s = 0usize;
    while s < n.0 {
        let len = if s + MORSEL <= n.0 { MORSEL } else { n.0 - s };
        phase1.run(bindings, MorselRange::new(USize(s), USize(len)));
        s += MORSEL;
    }
}

// ---------------------------------------------------------------------
// Workload transforms (mirror branching.rs / the body probe).
// ---------------------------------------------------------------------
const M1: u32 = 2654435761;
const M2: u32 = 2246822519;
const M4: u32 = 668265263;
const SH: u32 = 13;

#[inline(always)]
fn stage1(i: u32) -> u32 {
    i.wrapping_mul(M1)
}
#[inline(always)]
fn stage2(a: u32) -> u32 {
    a.wrapping_mul(M2).wrapping_add(1)
}
#[inline(always)]
fn stage3(b: u32) -> u32 {
    (b >> SH) ^ b
}
#[inline(always)]
fn stage4(c: u32) -> u32 {
    c.wrapping_mul(M4)
}

#[inline(always)]
fn branch_x(seed: u32) -> u32 {
    stage1(seed)
}
#[inline(always)]
fn branch_y(seed: u32) -> u32 {
    stage3(stage2(seed))
}
#[inline(always)]
fn join_fn(x: u32, y: u32) -> u32 {
    stage4(x ^ y)
}
#[inline(always)]
fn norm_fn(z: u32) -> u32 {
    stage3(stage2(z))
}

#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Xv(u32);
#[derive(Copy, Clone)]
struct Yv(u32);
#[derive(Copy, Clone)]
struct Zv(u32);
#[derive(Copy, Clone)]
struct Wv(u32);

type One<T> = Cons<Column<T>, Empty>;
type Two<A, B> = Cons<Column<A>, Cons<Column<B>, Empty>>;

struct BranchX;
impl BuilderInput for BranchX {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for BranchX {
    type Read = One<Inv>;
    type Write = One<Xv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Inv>, One<Xv>, PtrNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Xv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Xv, _>(i, Xv(branch_x(inp.0))) };
        });
    }
}

struct BranchY;
impl BuilderInput for BranchY {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for BranchY {
    type Read = One<Inv>;
    type Write = One<Yv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Inv>, One<Yv>, PtrNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Yv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Yv, _>(i, Yv(branch_y(inp.0))) };
        });
    }
}

// The join: reads TWO columns (Xv, Yv), writes Zv. Element-wise, fuses with
// the branches into phase 0.
struct JoinZ;
impl BuilderInput for JoinZ {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for JoinZ {
    type Read = Two<Xv, Yv>;
    type Write = One<Zv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> = EngineCtx<
        'frame,
        Two<Xv, Yv>,
        One<Zv>,
        PtrNil,
        ColPtrCons<Xv, ColPtrCons<Yv, ColPtrNil>>,
        ColPtrCons<Zv, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let x = unsafe { ctx.reader().read::<Xv, _>(i) };
            let y = unsafe { ctx.reader().read::<Yv, _>(i) };
            unsafe { ctx.writer().write::<Zv, _>(i, Zv(join_fn(x.0, y.0))) };
        });
    }
}

// Phase 1: reads Zv, writes Wv. The phase-1 occupant.
struct NormW;
impl BuilderInput for NormW {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for NormW {
    type Read = One<Zv>;
    type Write = One<Wv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Zv>, One<Wv>, PtrNil, ColPtrCons<Zv, ColPtrNil>, ColPtrCons<Wv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let z = unsafe { ctx.reader().read::<Zv, _>(i) };
            unsafe { ctx.writer().write::<Wv, _>(i, Wv(norm_fn(z.0))) };
        });
    }
}

// Heap-backed bump provider (the inline-buffer BumpProvider would overflow the
// stack at bench scale). The Box content is heap, the base pointer is stable
// because the buffer is never grown.
struct HeapBump {
    buf: UnsafeCell<Box<[MaybeUninit<u8>]>>,
    cap: usize,
    used: Cell<usize>,
}
impl HeapBump {
    fn new(bytes: usize) -> Self {
        let v: Box<[MaybeUninit<u8>]> = (0..bytes).map(|_| MaybeUninit::uninit()).collect();
        Self { buf: UnsafeCell::new(v), cap: bytes, used: Cell::new(0) }
    }
}
unsafe impl Send for HeapBump {}
unsafe impl Sync for HeapBump {}
impl MemoryProviderApi for HeapBump {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let base = unsafe { (*self.buf.get()).as_mut_ptr() as *mut u8 };
        let used = self.used.get();
        let align = align.0.max(1);
        let aligned = (used + align - 1) / align * align;
        if aligned + len.0 > self.cap {
            return core::ptr::null_mut();
        }
        self.used.set(aligned + len.0);
        unsafe { base.add(aligned) }
    }
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

// The morsel chunk (compile-time constant, bakes as an immediate). 1024
// records of u32 = 4 KiB per column, cache-friendly.
const MORSEL: usize = 1024;

fn bench_min<F: FnMut()>(warmup: usize, iters: usize, mut f: F) -> u128 {
    for _ in 0..warmup {
        f();
    }
    let mut best = u128::MAX;
    for _ in 0..iters {
        let t = Instant::now();
        f();
        let ns = t.elapsed().as_nanos();
        if ns < best {
            best = ns;
        }
    }
    best
}

fn main() {
    const N: usize = 1 << 17; // 131072 records, > 10K (Approach-E territory)

    // 5 columns of u32. Generous arena (8 MiB) covers 5 * N * 4 = ~2.6 MiB
    // plus per-column alignment padding.
    let provider = HeapBump::new(8 * 1024 * 1024);
    let mut sched = Scheduler::builder()
        .with(Column::<Inv>::new())
        .with(Column::<Xv>::new())
        .with(Column::<Yv>::new())
        .with(Column::<Zv>::new())
        .with(Column::<Wv>::new())
        .with(BranchX)
        .with(BranchY)
        .with(JoinZ)
        .with(NormW)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("engine build should succeed"));

    // Bindings are the columns, prepended: head Wv -> Zv -> Yv -> Xv -> Inv.
    // SAFETY: In's buffer reserved for N records of Inv (repr u32); the
    // scheduler (hence arena) is alive; each reserved slot written once.
    let in_base = sched
        .__bindings()
        .__tail()
        .__tail()
        .__tail()
        .__tail()
        .__ptr()
        .as_ptr() as *mut Inv;
    for i in 0..N {
        unsafe { *in_base.add(i) = Inv(i as u32) };
    }

    let bindings = sched.__bindings();

    // Registration-order phase 0: BranchX, BranchY, JoinZ. Phase 1: NormW.
    let p0_reg = WuCons {
        head: BranchX,
        tail: WuCons { head: BranchY, tail: WuCons { head: JoinZ, tail: WuNil } },
    };
    // RCM-within-level order phase 0: BranchY first (the plan picking the Y
    // branch ahead of X among the two equal-depth independent units), then
    // BranchX, then JoinZ. Both are topo-valid (JoinZ last either way).
    let p0_rcm = WuCons {
        head: BranchY,
        tail: WuCons { head: BranchX, tail: WuCons { head: JoinZ, tail: WuNil } },
    };
    let p1 = WuCons { head: NormW, tail: WuNil };

    // ----- correctness (registration order) -----
    run_schedule_mega::<MORSEL, _, _, _, _, _>(&p0_reg, &p1, bindings, USize(N));
    {
        // Wv = bindings head; Zv = one tail down.
        let wv_base = sched.__bindings().__ptr().as_ptr() as *const u32;
        let zv_base = sched.__bindings().__tail().__ptr().as_ptr() as *const u32;
        // SAFETY: both columns hold N reserved records; the scheduler is alive;
        // the schedule wrote every record.
        let wv = unsafe { core::slice::from_raw_parts(wv_base, N) };
        let zv = unsafe { core::slice::from_raw_parts(zv_base, N) };
        for i in 0..N {
            let z = join_fn(branch_x(i as u32), branch_y(i as u32));
            assert_eq!(zv[i], z, "Zv[{i}] mismatch (phase 0)");
            assert_eq!(wv[i], norm_fn(z), "Wv[{i}] mismatch (phase 1)");
        }
    }

    // ----- correctness (RCM order) produces the same result -----
    run_schedule_mega::<MORSEL, _, _, _, _, _>(&p0_rcm, &p1, bindings, USize(N));
    {
        let wv_base = sched.__bindings().__ptr().as_ptr() as *const u32;
        let wv = unsafe { core::slice::from_raw_parts(wv_base, N) };
        for i in 0..N {
            let z = join_fn(branch_x(i as u32), branch_y(i as u32));
            assert_eq!(wv[i], norm_fn(z), "Wv[{i}] mismatch (RCM order)");
        }
    }

    // ----- bench: RCM within-level order vs registration order -----
    let warmup = 50usize;
    let iters = 500usize;
    let reg_ns = bench_min(warmup, iters, || {
        run_schedule_mega::<MORSEL, _, _, _, _, _>(&p0_reg, &p1, bindings, USize(N));
        core::hint::black_box(&p0_reg);
    });
    let rcm_ns = bench_min(warmup, iters, || {
        run_schedule_mega::<MORSEL, _, _, _, _, _>(&p0_rcm, &p1, bindings, USize(N));
        core::hint::black_box(&p0_rcm);
    });

    let reg_per = reg_ns as f64 / N as f64;
    let rcm_per = rcm_ns as f64 / N as f64;
    let ratio = rcm_ns as f64 / reg_ns as f64;
    println!(
        "WORKS: two-phase morsel-outer schedule-mega ran {N} records (morsel={MORSEL}), \
         Zv+Wv correct in both orders"
    );
    println!(
        "bench (min of {iters}, warmup {warmup}): \
         registration-order phase0 = {reg_ns} ns ({reg_per:.3} ns/rec), \
         RCM-order phase0 = {rcm_ns} ns ({rcm_per:.3} ns/rec), \
         rcm/reg ratio = {ratio:.4}"
    );
}

// OUTCOME: WORKS (nightly-2026-05-28, release fat-LTO cgu=1).
//
// Type-check + run: the two-phase morsel-outer schedule-mega body compiled with
// only the two RunFiberCol bounds (witnesses inferred at the call site, the fix
// for the placeholder-witness problem the body probe hit on a named helper).
// Both phases ran correctly for 131072 records at morsel 1024: Zv[i] ==
// join(branch_x(i), branch_y(i)) and Wv[i] == norm(Zv[i]) for all records, in
// both the registration-order and the RCM-within-level-order phase 0.
//
// Devirt (objdump of `run_schedule_mega`, 193 instructions):
//   - check 1 (zero indirect call): PASS. 0 `blr`.
//   - check 5 (no helper calls): PASS. 0 `bl` of any kind; the whole two-phase
//     body, both morsel loops, and the inline walk are one straight-line body.
//   - check 4 (immediate morsel size): PASS. the const-generic MORSEL=1024 bakes
//     as `#0x400`, 4 occurrences; it is also visible in the mangled symbol as
//     `Kj400_`.
//   - check 2 (indexed addressing): PASS in substance. column loads/stores are
//     `ldr w21, [x11, x20]` / `str w21, [x10, x20]` (base + pre-scaled index
//     register). The disasm_5check check-2 text pattern looks for the `lsl
//     #scale` form, which 4-byte w-register loads do not emit; the addressing is
//     still fully indexed. Check-text gap, not a devirt failure.
//   - check 3 (no stack in inner loop): PASS. the only `[sp` accesses are the
//     prologue (0x870-0x874, stp x19-x22) and epilogue (0xb68-0xb6c, ldp); the
//     SIMD body (0x8d0-0xb54) has zero stack accesses, so no inner-loop spills.
//   - bonus: the per-record body auto-vectorized (48 vector ops: dup.4s,
//     eor.16b, ...), so morsel-outer dispatch did not block vectorization.
//   - 0 surviving run_fiber_col / RunFiberCol / fiber_shim / CollectFiber
//     symbols anywhere in the binary.
//
// Bench (min of 500, warmup 50, N=131072, morsel=1024), reproducible across
// runs: registration-order phase0 ~45.8us (~0.349 ns/rec), RCM-order phase0
// ~44.9us (~0.343 ns/rec), rcm/reg ratio ~0.98. So the within-level order of
// the two independent equal-depth branches has a small (~2%) but REPRODUCIBLE
// effect on single core (BranchY-first, the heavier branch, consistently a hair
// faster). Not perf-neutral, not large. The magnitude is workload-dependent
// (this is a tiny per-record kernel); the definitive per-workload number belongs
// in the #664 perf-gate suite once the real plan output drives the order.
//
// WHAT THIS SETTLES: the Approach-E schedule-mega single-core body is feasible.
// A multi-phase body (phase boundary = sequence point between two per-phase
// morsel loops) and morsel-outer dispatch (morsel loop wrapping the inline walk)
// both devirtualize end to end, the morsel constant bakes as an immediate, and
// the per-record body still vectorizes. No type-level fiber partition is needed
// on one core; fiber/phase boundaries and morsel sizes are in-body control flow
// plus compile-time constants. The single-core assembly problem reduces to
// "build the flat cons-list(s) in the plan's RCM-reordered topo order", and the
// within-level RCM order is a small benched perf refinement, not a structural
// blocker.
//
// WHAT THIS DOES NOT SETTLE: the genuine cross-record phase boundary (a
// reduction / scan) needs the non-nil AccumProject tie (deferred SRC-time
// residual, MEMORY LATEST-55), modeled here only structurally. The GATE-2
// parallel path needs the type-level per-fiber partition (trunks -> cores),
// out of scope for single core.

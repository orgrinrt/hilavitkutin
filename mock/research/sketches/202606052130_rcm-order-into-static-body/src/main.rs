//! Sketch (HILA-RUNTIME C2 / #340, Phase D): does the column-capable inline
//! `RunFiber` walk handle a BRANCHING diamond, a multi-column-read join, and a
//! walk order that differs from registration order, all while still
//! devirtualizing into one straight-line body?
//!
//! Context (design-oracle correction, MEMORY LATEST-56, rule
//! `design-is-the-oracle.md`): the corrected single-core dispatch is per-fiber
//! statically-derivable bodies (the column-capable walk, proven for a linear
//! chain in `202606051601_column-capable-runfiber-walk`) walking each fiber's
//! units in RCM EXECUTION order (the consolidation spec L1331-1339, L1403:
//! RCM's row reorder IS the WU execution order, not arena-layout-only), composed
//! into the schedule by inlining. "Codegen" means MIR to ASM; the mechanism
//! that gets each fiber's body statically derivable in RCM order is an
//! implementation/bench choice, not a pre-committed design point.
//!
//! The linear-chain sketch settled the per-unit column projection (one read
//! column, one write column) and the devirt of an inline three-deep walk. It
//! did NOT cover:
//!   1. a unit reading TWO columns (`JoinZ: Read = Two<Xv, Yv>`), so the read
//!      ColProject witness is a two-deep `ColPtrCons<Xv, ColPtrCons<Yv, ..>>`;
//!   2. a branching diamond (two independent branches feeding a join), the
//!      shape where the fiber partition is non-trivial and RCM picks the
//!      within-depth order among the equal-depth branches;
//!   3. a walk order that DIFFERS from the WU registration order, the property
//!      a statically-derivable body needs to express an RCM order that is a
//!      runtime-computed permutation of registration order.
//!
//! This probe builds the diamond from `mock/benches/engine_vs_std/src/branching.rs`
//! (BranchX: In->Xv, BranchY: In->Yv, JoinZ: {Xv,Yv}->Zv) against the real
//! engine crates, registers the WUs in the order BranchX, BranchY, JoinZ, and
//! drives the inline walk over a hand-built cons-list in the DIFFERENT order
//! BranchY, BranchX, JoinZ (simulating RCM picking the Y branch first). If it
//! type-checks, runs correctly (Zv matches the fused reference), and the release
//! disassembly shows no surviving dispatch symbol, then the static body half of
//! the corrected design is feasible for branching DAGs and is order-agnostic:
//! any statically-known order (including an RCM permutation) devirtualizes. The
//! remaining open bit (how the per-fiber sub-sequences are ASSEMBLED in RCM
//! order from the registration-order flat list) is then a builder/codegen
//! concern, not a walk concern, and the candidate mechanisms can be judged
//! against that fact.
//!
//! Outcome recorded at the bottom of this file.

#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

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
// 202606051601 sketch; it is what a shipping slice would generalize the
// resource-only RunFiber into). Constructs each unit's EngineCtx INLINE rather
// than erasing to a fiber_shim fn pointer, so the walk monomorphises into one
// straight-line body with no stored fn pointer. The crux this sketch adds:
// the SAME trait, unchanged, must handle a two-column read set (JoinZ) and a
// cons-list built in non-registration order.
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

#[inline]
fn run_fiber_col<F, A, Witnesses>(fiber: &F, bindings: &A, morsel: MorselRange)
where
    F: RunFiberCol<A, Witnesses>,
{
    fiber.run(bindings, morsel);
}

// ---------------------------------------------------------------------
// Workload: the branching diamond (mirrors branching.rs).
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

#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Xv(u32);
#[derive(Copy, Clone)]
struct Yv(u32);
#[derive(Copy, Clone)]
struct Zv(u32);

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

// The join: reads TWO columns (Xv, Yv), writes Zv. This is the new projection
// dimension over the linear-chain sketch: a two-deep read ColPtrCons.
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

// Stack-backed bump provider (mirrors the linear-chain sketch).
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
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

const N: usize = 64;

fn main() {
    let provider = BumpProvider::<65536>::new();
    // Register columns Inv, Xv, Yv, Zv then WUs BranchX, BranchY, JoinZ. Prepend
    // makes Zv the bindings head; Inv is three tails down (Zv -> Yv -> Xv -> In).
    let mut sched = Scheduler::builder()
        .with(Column::<Inv>::new())
        .with(Column::<Xv>::new())
        .with(Column::<Yv>::new())
        .with(Column::<Zv>::new())
        .with(BranchX)
        .with(BranchY)
        .with(JoinZ)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("engine build should succeed"));

    // Host-populate In[i] = i (In is three tails down from the Zv head).
    // SAFETY: In's buffer reserved for N records of Inv (repr u32); the
    // scheduler (hence arena) is alive; each reserved slot written once.
    let in_base =
        sched.__bindings().__tail().__tail().__tail().__ptr().as_ptr() as *mut Inv;
    for i in 0..N {
        unsafe { *in_base.add(i) = Inv(i as u32) };
    }

    // Drive the diamond through the inline walk in BranchY -> BranchX -> JoinZ
    // order. The per-unit four-witness list infers at the call site. The point:
    // a static order that differs from registration order, with a two-column
    // join, dispatches through the inline body.
    let bindings = sched.__bindings();
    let fiber = WuCons {
        head: BranchY,
        tail: WuCons { head: BranchX, tail: WuCons { head: JoinZ, tail: WuNil } },
    };
    run_fiber_col(&fiber, bindings, MorselRange::new(USize(0), USize(N)));

    // Read back the Zv column (the bindings head) and verify the diamond.
    let zv_base = sched.__bindings().__ptr().as_ptr();
    // SAFETY: Zv holds N reserved records; the scheduler (hence storage) is
    // alive; the walk wrote every record this morsel covered.
    let zv = unsafe { core::slice::from_raw_parts(zv_base as *const u32, N) };
    for i in 0..N {
        let expected = join_fn(branch_x(i as u32), branch_y(i as u32));
        assert_eq!(zv[i], expected, "Zv[{i}] mismatch: walk did not run the diamond correctly");
    }
    println!(
        "WORKS: column-capable inline walk ran the diamond (BranchY->BranchX->JoinZ, \
         two-column join) for {N} records, Zv correct"
    );
}

// OUTCOME: WORKS (nightly-2026-05-28, debug + release fat-LTO cgu=1).
//
// Debug run: Zv[i] == join(branch_x(i), branch_y(i)) for all 64 records, so the
// diamond dispatched correctly through the inline walk built in BranchY ->
// BranchX -> JoinZ order while the WUs registered in BranchX, BranchY, JoinZ
// order. The two-column read set (JoinZ: Read = Two<Xv, Yv>) resolved its read
// ColProject witness (a two-deep ColPtrCons<Xv, ColPtrCons<Yv, ColPtrNil>>) with
// no extra bound beyond what the linear-chain sketch carried; arg-inference at
// the call site found the four-witness-per-unit list, no turbofish.
//
// Release devirt: `nm` shows NO surviving run_fiber_col / RunFiberCol / fiber_shim
// symbol (fully inlined). Disassembling `sketch::main`, the per-record loop
// auto-vectorized to `eor.16b` SIMD (last vector op at 0x...6d18); the only two
// `blr` in the function are AFTER it, in the final println! path (the
// OnceLock<LineWriter<Stdout>> lazy-init and stdout-lock acquisition, with
// panic_fmt adjacent). Zero indirect calls in the dispatch region. A stored
// fn-pointer dispatch would have been an optimization barrier at each unit
// boundary, blocking both the devirt and the across-records vectorization; the
// inline walk has neither barrier.
//
// WHAT THIS SETTLES: the static-body half of the corrected single-core dispatch
// (MEMORY LATEST-56) is feasible for a BRANCHING DAG with a multi-column join,
// and it is ORDER-AGNOSTIC. Any statically-known order (including an RCM
// permutation of registration order) devirtualizes identically, because the
// walk dispatches whatever static cons-list it is handed. The hypothesis's
// "compile-time-nested WuCons cannot be sliced/reordered at a runtime boundary"
// wall is real but the walk never needs to slice: it walks the list as built.
//
// WHAT THIS DOES NOT SETTLE (the next decision, see FINDINGS.md): how the
// per-fiber cons-list is ASSEMBLED in RCM-reordered topo order. The current
// engine computes the order at RUNTIME (derive_phase_dispatch_order produces a
// topo_order permutation of registration indices; CollectFiber + run() dispatch
// slots[topo_order[step]] through a stored fn pointer, the 12.6x path). The
// consolidation spec puts the topology at BUILD time (L2437; the flattener emits
// a monomorphised function per fiber, L1566). Honoring "RCM is the execution
// order" (not arena-only) in a devirtualized body therefore requires the order
// to be a compile-time fact, which the runtime plan-chain is not. That gap is
// the mechanism fork recorded in FINDINGS.md; this sketch does not pick it.

//! Sketch (E1/E7 integration, roadmap r2 section 7): per-WU dirty-gated RunFiber walk.
//!
//! The E7 sketch (202606062600) proved the per-WU dirty propagation pass
//! (`predecessors[N] & dirty_mask`, spec Step 9 :1418-1429) marks exactly the
//! transitive cone of changed inputs, modeled with u64 bitsets over a plain index
//! loop. The E4 sketch (202606062100) proved the self-hosting frame loop fires the
//! meta markers and reuses the schedule across frames. Neither touched the REAL
//! type-level RunFiber cons-walk (dispatch/fiber_run.rs), which is the load-bearing
//! GATE-1 dispatch core: it monomorphises into one straight-line body that
//! devirtualises (zero `blr`) under fat LTO.
//!
//! Hypothesis: a dirty-gated variant of the real RunFiber walk, threading a runtime
//! per-WU dirty mask + the carrier position and gating the per-WU project+invoke on
//! the WU's dirty bit, (a) compiles against the real engine types, (b) STILL
//! devirtualises (the gate is a data-dependent branch around a DIRECT call to a
//! statically-known concrete `execute`, so the call target stays type-known and no
//! indirect `blr` is introduced), and (c) correctly skips clean WUs and runs dirty
//! ones (the skipped WU's output column is untouched; the run WU's is written).
//!
//! Asserted against a 3-WU DAG with one independent cone: P0 (In->A), P1 (A->B,
//! depends on P0), P2 (In->C, independent). Dirty masks are passed pre-computed
//! (the propagation is E7's job, already proven); this sketch proves the CONSUMPTION
//! in the real walk. Leeway (r2 section 7): SOME-SHAPE. The dirty mask is a u64 here
//! (the engine threads the arvo_bitmask BitSequence/BitAccess family for >64 WUs,
//! per #668's AdjRow row word; the branch-on-a-bit + direct-call shape, the thing
//! devirt depends on, is identical regardless of the bit-storage type). The carrier
//! position is a plain counter threaded through the recursion. Outcome at the bottom.

#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrNil, AccumProject, ColProject, ColPtrCons, ColPtrNil, EngineCtx, Project, PtrNil,
};
use hilavitkutin::dispatch::morsel::MorselRange;
use hilavitkutin::dispatch::wu_fn::invoke_wu_in_fiber;
use hilavitkutin::dispatch::{WuCons, WuNil};
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

// The dirty-gated variant of the real RunFiber walk (dispatch/fiber_run.rs). Same
// bound block as the shipping RunFiber impl, with two added run-time parameters:
// `dirty` (the per-WU dirty mask, bit p = WU at carrier position p is dirty) and
// `pos` (this cell's carrier position). The per-WU project+invoke is gated on the
// WU's dirty bit; the recursion threads `pos + 1`. A clean WU is skipped entirely
// (spec Step 9: "skips execution entirely"), including its ctx projection.
pub trait RunFiberDirty<A, Witnesses> {
    fn run_dirty(&self, bindings: &A, morsel: MorselRange, dirty: u64, pos: u32);
}

impl<A> RunFiberDirty<A, Empty> for WuNil {
    #[inline]
    fn run_dirty(&self, _bindings: &A, _morsel: MorselRange, _dirty: u64, _pos: u32) {}
}

impl<A, W, Tail, RIdx, RCIdx, WCIdx, WAIdx, WTail>
    RunFiberDirty<A, Cons<(RIdx, RCIdx, WCIdx, WAIdx), WTail>> for WuCons<W, Tail>
where
    W: WorkUnit,
    A: Project<<W as WorkUnit>::Read, RIdx>,
    A: ColProject<<W as WorkUnit>::Read, RCIdx>,
    A: ColProject<<W as WorkUnit>::Write, WCIdx>,
    // Column-only WUs (no accumulator): the write-set accumulator projection is
    // AccPtrNil, pinned so the 7th Ctx param resolves concretely (matching the
    // proven E4 RunFiberCol shape; the real general RunFiber leaves Out open and
    // resolves it at the concrete `run()` call site). The dirty gate below is the
    // same branch regardless of this pin, so the devirt + skip proof is
    // representative; accumulator WUs run unit-outer in the engine anyway.
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
    Tail: RunFiberDirty<A, WTail>,
{
    #[inline]
    fn run_dirty(&self, bindings: &A, morsel: MorselRange, dirty: u64, pos: u32) {
        // The dirty gate: a clean WU (its dirty bit unset) is skipped entirely,
        // projection included, exactly as spec Step 9 prescribes. The branch is
        // data-dependent on `dirty`, but the call inside is a direct call to the
        // statically-known concrete `W::execute` via the inline shim, so the call
        // target stays type-known and no indirect dispatch is introduced.
        if dirty & (1u64 << pos) != 0 {
            let ctx: <W as WorkUnit>::Ctx<'_> =
                EngineCtx::project::<A, A, RIdx, RCIdx, WCIdx, WAIdx>(bindings, bindings, morsel);
            invoke_wu_in_fiber(&self.head, &ctx);
        }
        self.tail.run_dirty(bindings, morsel, dirty, pos + 1);
    }
}

// Three columns and a small DAG. P1 depends on P0 (A is P0's output, P1's input);
// P2 is independent (reads In, writes C). Sentinels distinguish "ran" from "skipped".
#[derive(Copy, Clone)]
struct In(u32);
#[derive(Copy, Clone)]
struct A(u32);
#[derive(Copy, Clone)]
struct B(u32);
#[derive(Copy, Clone)]
struct C(u32);
type One<T> = Cons<Column<T>, Empty>;

const SENTINEL: u32 = 0xDEAD_BEEF;

struct P0; // In -> A
impl BuilderInput for P0 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for P0 {
    type Read = One<In>;
    type Write = One<A>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<In>, One<A>, PtrNil, ColPtrCons<In, ColPtrNil>, ColPtrCons<A, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let v = unsafe { ctx.reader().read::<In, _>(i) };
            unsafe { ctx.writer().write::<A, _>(i, A(v.0 + 1)) };
        });
    }
}

struct P1; // A -> B (depends on P0)
impl BuilderInput for P1 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for P1 {
    type Read = One<A>;
    type Write = One<B>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<A>, One<B>, PtrNil, ColPtrCons<A, ColPtrNil>, ColPtrCons<B, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let a = unsafe { ctx.reader().read::<A, _>(i) };
            unsafe { ctx.writer().write::<B, _>(i, B(a.0 * 10)) };
        });
    }
}

struct P2; // In -> C (independent cone)
impl BuilderInput for P2 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for P2 {
    type Read = One<In>;
    type Write = One<C>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<In>, One<C>, PtrNil, ColPtrCons<In, ColPtrNil>, ColPtrCons<C, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let v = unsafe { ctx.reader().read::<In, _>(i) };
            unsafe { ctx.writer().write::<C, _>(i, C(v.0 + 1000)) };
        });
    }
}

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

const N: usize = 1 << 12;
const MORSEL: usize = 512;

// The dirty-gated morsel loop. #[inline(never)] so objdump has a real symbol to
// scan for `blr`. This is the E1 frame body shape (window the record range into
// morsels) with the E7 dirty gate threaded into the walk.
#[inline(never)]
fn gated_run<Carrier, Bindings, W>(carrier: &Carrier, bindings: &Bindings, total: usize, dirty: u64)
where
    Carrier: RunFiberDirty<Bindings, W>,
{
    let m = MORSEL.max(1);
    let mut start = 0;
    while start < total {
        let len = m.min(total - start);
        carrier.run_dirty(bindings, MorselRange::new(USize(start), USize(len)), dirty, 0);
        start += m;
    }
}

fn seed_inputs(in_base: *mut In) {
    for i in 0..N {
        unsafe { *in_base.add(i) = In(i as u32) };
    }
}

fn main() {
    let provider = HeapBump::new(8 * 1024 * 1024);
    let sched = Scheduler::builder()
        .with(Column::<In>::new())
        .with(Column::<A>::new())
        .with(Column::<B>::new())
        .with(Column::<C>::new())
        .with(P0)
        .with(P1)
        .with(P2)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build"));

    // Reach the four column bases. Registration order is In, A, B, C; the bindings
    // cons-list is built in reverse, so the head is the last-registered (C).
    let c_base = sched.__bindings().__ptr().as_ptr() as *mut C;
    let b_base = sched.__bindings().__tail().__ptr().as_ptr() as *mut B;
    let a_base = sched.__bindings().__tail().__tail().__ptr().as_ptr() as *mut A;
    let in_base = sched.__bindings().__tail().__tail().__tail().__ptr().as_ptr() as *mut In;

    let carrier = WuCons { head: P0, tail: WuCons { head: P1, tail: WuCons { head: P2, tail: WuNil } } };

    let reset = |a_base: *mut A, b_base: *mut B, c_base: *mut C| {
        for i in 0..N {
            unsafe {
                *a_base.add(i) = A(SENTINEL);
                *b_base.add(i) = B(SENTINEL);
                *c_base.add(i) = C(SENTINEL);
            }
        }
    };

    seed_inputs(in_base);

    // Carrier positions: P0=0, P1=1, P2=2.

    // Case 1: all dirty (full run). A, B, C all written.
    reset(a_base, b_base, c_base);
    gated_run(&carrier, sched.__bindings(), N, 0b111);
    {
        let a = unsafe { core::slice::from_raw_parts(a_base, N) };
        let b = unsafe { core::slice::from_raw_parts(b_base, N) };
        let c = unsafe { core::slice::from_raw_parts(c_base, N) };
        for i in 0..N {
            assert_eq!(a[i].0, i as u32 + 1, "all-dirty: P0 ran, A[{i}]");
            assert_eq!(b[i].0, (i as u32 + 1) * 10, "all-dirty: P1 ran, B[{i}]");
            assert_eq!(c[i].0, i as u32 + 1000, "all-dirty: P2 ran, C[{i}]");
        }
    }

    // Case 2: only P2 dirty (its independent cone). P0, P1 skipped: A, B stay
    // sentinel. P2 ran: C written.
    reset(a_base, b_base, c_base);
    gated_run(&carrier, sched.__bindings(), N, 0b100);
    {
        let a = unsafe { core::slice::from_raw_parts(a_base, N) };
        let b = unsafe { core::slice::from_raw_parts(b_base, N) };
        let c = unsafe { core::slice::from_raw_parts(c_base, N) };
        for i in 0..N {
            assert_eq!(a[i].0, SENTINEL, "P2-only: P0 skipped, A[{i}] untouched");
            assert_eq!(b[i].0, SENTINEL, "P2-only: P1 skipped, B[{i}] untouched");
            assert_eq!(c[i].0, i as u32 + 1000, "P2-only: P2 ran, C[{i}]");
        }
    }

    // Case 3: P0 + P1 dirty (the In->A->B cone), P2 clean. A, B written; C stays.
    reset(a_base, b_base, c_base);
    gated_run(&carrier, sched.__bindings(), N, 0b011);
    {
        let a = unsafe { core::slice::from_raw_parts(a_base, N) };
        let b = unsafe { core::slice::from_raw_parts(b_base, N) };
        let c = unsafe { core::slice::from_raw_parts(c_base, N) };
        for i in 0..N {
            assert_eq!(a[i].0, i as u32 + 1, "AB-cone: P0 ran, A[{i}]");
            assert_eq!(b[i].0, (i as u32 + 1) * 10, "AB-cone: P1 ran, B[{i}]");
            assert_eq!(c[i].0, SENTINEL, "AB-cone: P2 skipped, C[{i}] untouched");
        }
    }

    // Case 4: nothing dirty (clean frame). Incremental processor runs NOTHING.
    reset(a_base, b_base, c_base);
    gated_run(&carrier, sched.__bindings(), N, 0b000);
    {
        let a = unsafe { core::slice::from_raw_parts(a_base, N) };
        let b = unsafe { core::slice::from_raw_parts(b_base, N) };
        let c = unsafe { core::slice::from_raw_parts(c_base, N) };
        for i in 0..N {
            assert_eq!(a[i].0, SENTINEL, "clean-frame: A[{i}] untouched");
            assert_eq!(b[i].0, SENTINEL, "clean-frame: B[{i}] untouched");
            assert_eq!(c[i].0, SENTINEL, "clean-frame: C[{i}] untouched");
        }
    }

    println!(
        "WORKS: dirty-gated RunFiber walk. The real type-level cons-walk, gated per-WU by a \
         runtime dirty mask threaded by carrier position, ran the right WUs each frame: \
         all-dirty -> A,B,C written; P2-only -> only C (P0,P1 skipped, A,B untouched); \
         AB-cone -> A,B written, C skipped; clean-frame -> nothing ran. Skip == output column \
         untouched, exactly spec Step 9's 'skips execution entirely'. Run `objdump` on \
         gated_run to confirm zero `blr` (devirt preserved)."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28, release fat-LTO, codegen-units=1).
//
// (a) COMPILES against the real engine types. The dirty-gated walk reuses the
//     real RunFiber bound block (Project / ColProject / AccumProject witnesses,
//     the EngineCtx GAT tie) verbatim, pinning the column-only accumulator
//     projection to AccPtrNil (the proven E4 RunFiberCol shape; the general
//     RunFiber leaves Out open and resolves it at the concrete run() call site).
//     The added run-time params (dirty: u64, pos: u32) thread cleanly through the
//     type-level WuCons recursion.
//
// (b) DEVIRT PRESERVED, the load-bearing GATE-1 property. objdump of the single
//     `gated_run` mono (the 3-WU carrier WuCons<P0,WuCons<P1,WuCons<P2,WuNil>>>):
//     2840 instructions, ZERO `blr` AND ZERO `bl`. The whole walk (per-WU project
//     + execute for all three WUs, across the morsel loop) inlined into
//     straight-line code; the dirty gate survives only as a predicated
//     compare+branch, introducing NO call of any kind, direct or indirect.
//     Threading the runtime `pos` counter through the type-level recursion did not
//     force any indirect path: the recursion stays fully unrolled and
//     monomorphised. A per-WU dirty gate does not degrade the dispatch core: a
//     clean WU costs one branch, dirty WUs run exactly as today.
//
// (c) SKIP CORRECT. All four pre-computed dirty-mask cases pass: all-dirty (0b111)
//     -> A,B,C written; P2-only (0b100) -> only C written, A,B untouched (P0,P1
//     skipped); AB-cone (0b011) -> A,B written, C untouched (P2 skipped);
//     clean-frame (0b000) -> nothing ran. Skip == output column stays at SENTINEL,
//     exactly spec Step 9's "skips execution entirely" (:1418-1429).
//
// WHAT THIS SETTLES (E1/E7 integration): the per-WU dirty gate (E7's consumption,
// spec Step 9) integrates into the real type-level RunFiber dispatch walk with
// devirt fully preserved, gated by carrier position threaded through the
// recursion. Combined with the E4 sketch (frame loop + meta markers + reuse) and
// the E7 sketch (dirty propagation over predecessor masks), the GATE-1-completing
// E1+E7 round is de-risked end to end. The real round adds the gate to the
// EXISTING RunFiber::run body (a runtime if around the same project+invoke), so
// the bound block is untouched and the E0271 seen while authoring this separate
// RunFiberDirty trait does not arise there.
//
// WHAT THIS DOES NOT SETTLE: the per-frame dirty-mask COMPUTATION (run() reading
// the plan_dirty seed + propagating over per-WU predecessor masks in carrier
// order) and the meta-WU On<Marker> firing order are the E4/E7 sketches' domain
// and the real round's wiring; this proves the dispatch-walk CONSUMPTION of an
// already-computed mask. Leeway taken (r2 section 7, SOME-SHAPE): the mask is a
// u64 here; the engine threads the arvo_bitmask BitSequence/BitAccess family for
// >64 WUs (#668's AdjRow row word). The branch-on-a-bit + direct-call shape that
// devirt depends on is identical regardless of the bit-storage type.
// ---------------------------------------------------------------------

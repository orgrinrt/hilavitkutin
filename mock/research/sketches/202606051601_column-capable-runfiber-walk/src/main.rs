//! Sketch (HILA-RUNTIME C2 / #340, Phase D): a COLUMN-CAPABLE inline-recursive
//! fiber walk.
//!
//! Hypothesis: the resource-only `RunFiber` walk (proven in the
//! `202605300823_run-fiber-wutuple-walk` sketch and shipped in
//! `dispatch/fiber_walk.rs`) can be extended to column-reading and
//! column-writing WorkUnits WITHOUT erasing to a function pointer, so a
//! fiber's unit sequence monomorphises into one inlined body. That body is the
//! devirtualization half of Phase D Shape B: no stored fn pointer means LLVM
//! sees one straight-line per-fiber function (the risk-R2 fix), and it is the
//! precondition the fusion half (scratch-backed internal columns) needs for
//! dead-store elimination across unit boundaries.
//!
//! Why this is NOT already answered by `CollectFiber` (`dispatch/fiber_codegen.rs`):
//! `CollectFiber` carries the same four-witness bound shape
//! `(RIdx, RCIdx, WCIdx, WAIdx)`, but its `collect` writes a
//! `fiber_shim::<W, A, ...>` FUNCTION POINTER into a slot array and defers the
//! `EngineCtx` construction into that shim. So the 7-param `EngineCtx` GAT
//! equality, including the LIFETIME-DEPENDENT accumulator bundle
//! (`<A as AccumProject<'f, W::Write, WAIdx>>::Out`, the 7th param), is resolved
//! inside the concrete-per-W shim, never in the recursive trait impl. The
//! INLINE walk constructs the Context inside the recursive impl body, which
//! forces that GAT equality and the `for<'f> AccumProject` HRTB to normalize
//! WITHIN the recursive trait impl at each depth. Whether rustc resolves that
//! for a multi-deep heterogeneous cons-list without overflow or normalization
//! failure under nightly-2026-05-28 is the crux this sketch settles. The
//! resource-only sketch's bound was lifetime-INDEPENDENT; this one is not.
//!
//! Faithfulness: this uses the REAL `EngineCtx`, `Project`, `ColProject`,
//! `AccumProject`, `WuCons`/`WuNil`, and `Scheduler`. Only the
//! `RunFiberCol` walk trait is new (it is what a shipping slice would add). A
//! WORKS result therefore transfers directly. The fiber is a three-stage
//! column chain (In -> A -> B -> C); the column-no-accumulator case resolves
//! the lifetime-dependent accumulator HRTB to the empty bundle, exercising the
//! normalization without a non-empty accumulator value. A non-empty
//! accumulator inline is a follow-on probe.
//!
//! Outcome recorded at the bottom of this file.

#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrNil, AccumProject, ColPtrCons, ColPtrNil, ColProject, EngineCtx, Project, PtrNil,
};
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
// The value-carrying fiber list (re-exported from the engine next to the walk).
use hilavitkutin::dispatch::fiber_walk::{WuCons, WuNil};
use hilavitkutin_providers::ArenaColumnStorage;

// ---------------------------------------------------------------------
// THE CRUX: the column-capable inline-recursive fiber walk.
//
// Mirrors `CollectFiber`'s four-witness bound shape (RIdx resources, RCIdx read
// columns, WCIdx write columns, WAIdx accumulators) and the resource-only
// `RunFiber`'s inline body (project the Context, call execute, recurse), but
// constructs the `EngineCtx` INLINE rather than erasing to a `fiber_shim` fn
// pointer. The column source is the bindings itself (`EngineCtx::project::<A,
// A, ...>`), matching `fiber_shim`.
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
    // Constrain the accumulator projection's output to the empty bundle via an
    // associated-type-equality bound rather than restating it as an unresolved
    // projection in the GAT tie. The WUs here write no accumulator, so this is
    // faithful, and it breaks the inference circularity: the 7th Ctx param is
    // the WU's concrete `AccPtrNil` (its default), and `Out = AccPtrNil` here
    // pins the projection to match without the solver having to normalize a
    // projection over an unresolved `A`. A non-empty accumulator inline (the
    // projection restated, as `CollectFiber` carries it) is a follow-on probe;
    // the column dimension (read/write column projections) is what this sketch
    // settles, and those stay restated as projections below.
    for<'f> A: AccumProject<'f, <W as WorkUnit>::Write, WAIdx, Out = AccPtrNil>,
    // Tie each unit's Ctx GAT to the projection of its resources and its read
    // and write columns over the shared bindings, for all frame lifetimes. The
    // accumulator bundle (7th param) is the concrete `AccPtrNil` the WU declares
    // by default.
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

/// Entry point: drive a fiber's column-bearing WU sequence over the bindings.
/// The four-witness-per-unit list infers at the call site, no turbofish.
#[inline]
fn run_fiber_col<F, A, Witnesses>(fiber: &F, bindings: &A, morsel: MorselRange)
where
    F: RunFiberCol<A, Witnesses>,
{
    fiber.run(bindings, morsel);
}

// ---------------------------------------------------------------------
// Workload: a three-stage column chain In -> A -> B -> C (the perf-gate
// element-wise shape, truncated to three units). Each WU reads one column and
// writes one column; no accumulators, so the lifetime-dependent accumulator
// projection resolves to the empty bundle.
// ---------------------------------------------------------------------
const M1: u32 = 2654435761;
const M2: u32 = 2246822519;
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

#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Av(u32);
#[derive(Copy, Clone)]
struct Bv(u32);
#[derive(Copy, Clone)]
struct Cv(u32);

type One<T> = Cons<Column<T>, Empty>;

struct S1;
impl BuilderInput for S1 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for S1 {
    type Read = One<Inv>;
    type Write = One<Av>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Inv>, One<Av>, PtrNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Av, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated for the record count; Av reserved and
            // exclusively written here; the morsel covers only reserved records.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Av, _>(i, Av(stage1(inp.0))) };
        });
    }
}

struct S2;
impl BuilderInput for S2 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for S2 {
    type Read = One<Av>;
    type Write = One<Bv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Av>, One<Bv>, PtrNil, ColPtrCons<Av, ColPtrNil>, ColPtrCons<Bv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let a = unsafe { ctx.reader().read::<Av, _>(i) };
            unsafe { ctx.writer().write::<Bv, _>(i, Bv(stage2(a.0))) };
        });
    }
}

struct S3;
impl BuilderInput for S3 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for S3 {
    type Read = One<Bv>;
    type Write = One<Cv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Bv>, One<Cv>, PtrNil, ColPtrCons<Bv, ColPtrNil>, ColPtrCons<Cv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let b = unsafe { ctx.reader().read::<Bv, _>(i) };
            unsafe { ctx.writer().write::<Cv, _>(i, Cv(stage3(b.0))) };
        });
    }
}

// Stack-backed bump provider (mirrors tests/accum_dispatch.rs).
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
    let provider = BumpProvider::<32768>::new();
    // Register Cv, Bv, Av, Inv: prepend makes the bindings head Inv (last
    // registered), with Cv the deepest tail (Inv -> Av -> Bv -> Cv).
    let mut sched = Scheduler::builder()
        .with(Column::<Cv>::new())
        .with(Column::<Bv>::new())
        .with(Column::<Av>::new())
        .with(Column::<Inv>::new())
        .with(S1)
        .with(S2)
        .with(S3)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("engine build should succeed"));

    // Host-populate In[i] = i (In is the bindings head).
    // SAFETY: In's buffer was reserved for N records of Inv (repr u32); the
    // scheduler (hence the arena) is alive; each reserved slot is written once.
    let in_base = sched.__bindings().__ptr().as_ptr() as *mut Inv;
    for i in 0..N {
        unsafe { *in_base.add(i) = Inv(i as u32) };
    }

    // Drive the THREE-unit column chain through the column-capable inline walk
    // over one morsel covering the whole range. The per-unit four-witness list
    // infers; no turbofish. This is the call whose monomorphisation the crux is
    // about: a heterogeneous WuCons<S1, WuCons<S2, WuCons<S3, WuNil>>> with
    // distinct column Read/Write sets, each projecting its own Context inline.
    let fiber = WuCons {
        head: S1,
        tail: WuCons { head: S2, tail: WuCons { head: S3, tail: WuNil } },
    };
    // Pin A (the bindings type) before the RunFiberCol bound is solved, so the
    // accumulator projection in the GAT tie normalizes to AccPtrNil. The engine
    // gets this for free: Scheduler::run<Witnesses> derives A from Self.
    let bindings = sched.__bindings();
    run_fiber_col(&fiber, bindings, MorselRange::new(USize(0), USize(N)));

    // Read back the Cv column (deepest tail) and verify the chain.
    let cv_base = sched.__bindings().__tail().__tail().__tail().__ptr().as_ptr();
    // SAFETY: Cv holds N reserved records; the scheduler (hence storage) is
    // alive; the walk wrote every record this morsel covered.
    let cv = unsafe { core::slice::from_raw_parts(cv_base as *const u32, N) };
    for i in 0..N {
        let expected = stage3(stage2(stage1(i as u32)));
        assert_eq!(cv[i], expected, "Cv[{i}] mismatch: walk did not run the chain in order");
    }
    println!("WORKS: column-capable inline fiber walk ran In->A->B->C for {N} records, Cv correct");
}

// OUTCOME: WORKS (nightly-2026-05-28). Compiled and ran; Cv == stage3(stage2(
// stage1(i))) for all 64 records, so the three-unit column chain dispatched in
// order through the inline walk with no function pointer.
//
// What it settles (the architect's named crux): a column-capable inline
// recursive fiber walk that constructs each unit's EngineCtx INLINE (not erased
// to a fiber_shim fn pointer) type-checks for a heterogeneous three-deep
// WuCons with distinct column Read/Write sets. No overflow, no recursion-limit,
// no normalization failure. So Phase D Shape B's devirtualization half (one
// monomorphised straight-line per-fiber body, the risk-R2 fix) is feasible.
//
// The precise constraint discovered: the GAT-equality tie can restate a Ctx
// param as an unresolved projection only where that param genuinely varies and
// the solver can pin `A` independently. Where the WU declares a param
// CONCRETELY (the 7th, the accumulator bundle, defaults to `AccPtrNil`),
// restating it as `<A as AccumProject<..>>::Out` deadlocks inference at a free
// entry call: normalizing the projection needs the witness, and the witness is
// being inferred from this bound. Two fixes, both available to the engine: (a)
// pin the concrete param with an `Out = AccPtrNil` associated-type-equality
// bound (used here, faithful because these WUs write no accumulator), keeping
// only the varying column projections restated; or (b) drive the walk from a
// context where `A` is pinned by Self, which is exactly `Scheduler::run<
// Witnesses>` where the shipped `CollectFiber` already resolves the same tie
// (including a non-empty accumulator bundle). The column projections (read and
// write columns, the genuinely-new dimension over the resource-only sketch)
// stayed restated as projections and normalized fine with `A` arg-inferred.
//
// Follow-on probe NOT done here: a non-empty accumulator bundle restated inline
// (fix path b). The bench already runs accumulator WUs through CollectFiber +
// run<Witnesses>, so that context is proven; the inline-with-non-nil-accum
// variant is a small delta to confirm when the engine slice wires the walk into
// run<Witnesses>.

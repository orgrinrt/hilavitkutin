//! Sketch (HILA-RUNTIME C2 / #340, Phase D): RunFiberCol with a NON-EMPTY
//! accumulator bundle restated inline, driven from an `A`-pinned context.
//!
//! This is the one follow-on delta sketch 202606051601 named and did not run.
//! That sketch settled the column dimension (read/write column projections
//! restated inline) but pinned the 7th EngineCtx GAT param (the accumulator
//! bundle) to the concrete `AccPtrNil` via an `Out = AccPtrNil` bound, because
//! its workload wrote no accumulator and restating the projection at a FREE
//! entry call deadlocks witness inference. It recorded fix path (b): drive the
//! walk from a context where `A` is pinned by `Self`, which is exactly
//! `Scheduler::run<Witnesses>`, where the shipped `CollectFiber` already
//! resolves the identical tie (including a non-empty accumulator bundle).
//!
//! Hypothesis: a `RunFiberCol` whose bound restates the accumulator projection
//! (`for<'f> A: AccumProject<'f, W::Write, WAIdx>` plus the 7th GAT param as
//! `<A as AccumProject<'f, W::Write, WAIdx>>::Out`, byte-for-byte `CollectFiber`'s
//! accumulator bound) type-checks and runs INLINE for a heterogeneous cons-list
//! that mixes a column-writing unit and an accumulator-appending unit, when `A`
//! is pinned before `Witnesses` is inferred. The pin is provided by a small
//! `Harness<A>` whose method fixes `A` from `Self`, mirroring the
//! `Scheduler::run<Witnesses>` shape (`A = <Vals as BindingsFor>::Bindings`,
//! fixed by `Self` before `Witnesses`).
//!
//! Why this matters: the SRC slice replaces the `CollectFiber` + `fiber_shim`
//! fn-pointer slot dispatch in `Scheduler::run` with this inline `RunFiberCol`
//! walk. `CollectFiber` carries this exact accumulator bound and resolves it in
//! `run<Witnesses>` today; the only change is moving the EngineCtx construction
//! from the per-W `fiber_shim` fn pointer to the inline recursive body. If the
//! inline body resolves the same tie in the same `A`-pinned context, the slot
//! path can be deleted with no loss. This sketch confirms it does.
//!
//! Faithfulness: real `EngineCtx`, `Project`, `ColProject`, `AccumProject`,
//! `WuCons`/`WuNil`, `Scheduler`, `Accum`. Only the `RunFiberCol` trait and the
//! `Harness` driver are new; both are what the shipping slice adds (the walk
//! into `dispatch/`, the `A`-pin already present as `run<Witnesses>`'s `Self`).
//! Outcome recorded at the bottom.

#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons, AccPtrNil, AccumProject, ColPtrCons, ColPtrNil, ColProject, EngineCtx, Project,
    PtrNil,
};
use hilavitkutin::dispatch::fiber_walk::{WuCons, WuNil};
use hilavitkutin::dispatch::morsel::MorselRange;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    AccumWriterApi, ColumnReaderApi, ColumnWriterApi, EachApi, HasAccumWriter, HasColumnReader,
    HasColumnWriter, HasEach,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Accum, Column};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;

// ---------------------------------------------------------------------
// THE CRUX: RunFiberCol with the accumulator projection RESTATED inline.
//
// Bound is byte-for-byte `CollectFiber`'s (four witnesses, the column read/write
// projections restated, AND the lifetime-dependent accumulator projection
// restated as the 7th GAT param). Body is `RunFiber`'s inline recursion. The
// difference from sketch 202606051601: that one pinned the 7th param to
// `AccPtrNil`; this one restates it as `<A as AccumProject<'f, W::Write,
// WAIdx>>::Out`, which is the genuine non-empty-accumulator shape.
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
    // The accumulator projection restated (NOT pinned to AccPtrNil). This is the
    // delta over sketch 202606051601 and the exact bound `CollectFiber` carries.
    for<'f> A: AccumProject<'f, <W as WorkUnit>::Write, WAIdx>,
    for<'f> W: WorkUnit<
        Ctx<'f> = EngineCtx<
            'f,
            <W as WorkUnit>::Read,
            <W as WorkUnit>::Write,
            <A as Project<<W as WorkUnit>::Read, RIdx>>::Out,
            <A as ColProject<<W as WorkUnit>::Read, RCIdx>>::Out,
            <A as ColProject<<W as WorkUnit>::Write, WCIdx>>::Out,
            <A as AccumProject<'f, <W as WorkUnit>::Write, WAIdx>>::Out,
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
// The `A`-pin: `Harness<'b, A>` fixes `A` from `Self` before `Witnesses` is
// inferred at the `.drive` call, mirroring `Scheduler::run<Witnesses>` where
// `A = <Vals as BindingsFor>::Bindings` is fixed by `Self`. This is fix path (b)
// from sketch 202606051601: the witness-inference deadlock that a FREE entry
// call hit does not arise here because `A` is already pinned.
// ---------------------------------------------------------------------
struct Harness<'b, A> {
    bindings: &'b A,
}

impl<'b, A> Harness<'b, A> {
    #[inline]
    fn drive<F, Witnesses>(&self, fiber: &F, morsel: MorselRange)
    where
        F: RunFiberCol<A, Witnesses>,
    {
        fiber.run(self.bindings, morsel);
    }
}

// ---------------------------------------------------------------------
// Workload: a two-unit fiber mixing a column writer and an accumulator
// appender. S1 reads In, writes Av (column). Tally reads Av, appends one value
// per record to Accum<Sum>. The cons-list is heterogeneous: S1's 7th GAT param
// projects to AccPtrNil, Tally's projects to AccPtrCons<Sum, AccPtrNil>, both
// restated through the single RunFiberCol bound.
// ---------------------------------------------------------------------
const M1: u32 = 2654435761;

#[inline(always)]
fn stage1(i: u32) -> u32 {
    i.wrapping_mul(M1)
}

#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Av(u32);
#[derive(Copy, Clone)]
struct Sum(u32);

type One<T> = Cons<Column<T>, Empty>;
type AccW = Cons<Accum<Sum>, Empty>;

struct S1;
impl BuilderInput for S1 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for S1 {
    type Read = One<Inv>;
    type Write = One<Av>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<Inv>,
        One<Av>,
        PtrNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<Av, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated for the record count; Av reserved and
            // exclusively written here; the morsel covers only reserved records.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Av, _>(i, Av(stage1(inp.0))) };
        });
    }
}

struct Tally;
impl BuilderInput for Tally {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for Tally {
    type Read = One<Av>;
    type Write = AccW;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<Av>,
        AccW,
        PtrNil,
        ColPtrCons<Av, ColPtrNil>,
        ColPtrNil,
        AccPtrCons<'frame, Sum, AccPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let a = unsafe { ctx.reader().read::<Av, _>(i) };
            // SAFETY: build reserved the Accum<Sum> buffer for the record count;
            // the plan proved this unit the exclusive appender; one append per
            // record stays within the reserved capacity.
            unsafe { ctx.accums().append::<Sum, _>(Sum(a.0)) };
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
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

const N: usize = 64;

fn main() {
    let provider = BumpProvider::<32768>::new();
    // Register Sum (accum), Av (column), Inv (column), then S1, Tally. Prepend
    // makes Inv the bindings head; Sum the deepest tail.
    let sched = Scheduler::builder()
        .with(Accum::<Sum>::new())
        .with(Column::<Av>::new())
        .with(Column::<Inv>::new())
        .with(S1)
        .with(Tally)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("engine build should succeed"));

    // Host-populate In[i] = i (In is the bindings head after prepend).
    // SAFETY: In's buffer reserved for N records of Inv (repr u32); the
    // scheduler (hence arena) is alive; each reserved slot written once.
    let in_base = sched.__bindings().__ptr().as_ptr() as *mut Inv;
    for i in 0..N {
        unsafe { *in_base.add(i) = Inv(i as u32) };
    }

    // Drive the heterogeneous column+accumulator cons-list through the inline
    // RunFiberCol walk, A-pinned by the Harness (the run<Witnesses> shape). The
    // four-witness-per-unit list infers; no turbofish.
    let fiber = WuCons { head: S1, tail: WuCons { head: Tally, tail: WuNil } };
    let harness = Harness { bindings: sched.__bindings() };
    harness.drive(&fiber, MorselRange::new(USize(0), USize(N)));

    // Verify: S1 wrote Av = stage1(i) (column path), Tally appended one Sum per
    // record (accumulator path). The accumulator live length must be N and each
    // appended Sum must equal stage1(i) in append (record) order.
    let av_base = sched.__bindings().__tail().__ptr().as_ptr() as *const u32;
    // SAFETY: Av holds N reserved records; storage alive; S1 wrote every record.
    let av = unsafe { core::slice::from_raw_parts(av_base, N) };
    for i in 0..N {
        assert_eq!(av[i], stage1(i as u32), "Av[{i}] mismatch: column write path");
    }

    // Accumulator buffer is the deepest tail (Sum, registered first). Read its
    // live length and contents back through the binding accessors (same shape as
    // tests/accum_dispatch.rs: `__len_cell().get()` and `__ptr()`).
    let sum_binding = sched.__bindings().__tail().__tail();
    let sum_len = sum_binding.__len_cell().get().0;
    assert_eq!(sum_len, N, "accum live length should be N after one append per record");
    let sum_base = sum_binding.__ptr().as_ptr() as *const u32;
    // SAFETY: Sum reserved for N records; storage alive; Tally appended N values.
    let sums = unsafe { core::slice::from_raw_parts(sum_base, N) };
    for i in 0..N {
        assert_eq!(sums[i], stage1(i as u32), "Sum[{i}] mismatch: accumulator append path");
    }

    println!(
        "WORKS: inline RunFiberCol with non-empty accumulator ran S1(col)+Tally(accum) for {N} \
         records; Av column correct, {N} Sum appends correct"
    );
}

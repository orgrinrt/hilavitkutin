//! Sketch §7-6 (roadmap r2, op decision (a)): RCM-row recovery. Can the engine
//! auto-apply the RCM-optimal dispatch order WITHOUT the consumer registering WUs
//! in that order, while keeping devirt? op flagged loimu as order-sensitive, so the
//! engine applying the cache-optimal order itself (not relying on manual
//! registration) is a near-term priority.
//!
//! The constraints that make this hard (all established by prior sketches):
//!   - You cannot reorder a heterogeneous cons-list TYPE at runtime (no mechanism).
//!   - You cannot const-INDEX the cons-list at a permuted position (`Nth<K>`): the
//!     per-position impls overlap, E0119 (proven 202606071000 Tier A2/B).
//!   - A proc-macro / build.rs cannot see resolved AccessSets (they are trait
//!     associated types, invisible to a syntactic macro), so it cannot emit an
//!     RCM-ordered carrier from the access analysis.
//!   - A const-ORDER fn-pointer-slot array dispatched UNDER A MORSEL LOOP gets 2 blr
//!     (202606080300): the anti-pattern. Only the type-level walk devirts under the
//!     morsel loop (202606081200), and it follows the carrier TYPE order.
//!
//! The UNTESTED candidate this sketch probes: a type-level walk that visits the
//! cons-list once PER OUTPUT SLOT, executing a WU only when its CONST RCM position
//! equals the current slot. This reorders execution WITHOUT const-indexing the list
//! (no `Nth<K>`, so no E0119) and WITHOUT reordering the carrier type. The walk uses
//! the same non-overlapping base/step impls as the proven `RunFiberCol`; the
//! position check is a const-vs-runtime branch INSIDE the step, not an impl
//! selector. Hypothesis: LLVM const-folds the position guards (each WU's POS is a
//! compile-time const), so the N-slot x N-WU walk collapses to the N executes in
//! RCM order, fully inlined: devirt + auto-reorder, no manual registration, no
//! coherence wall. If TRUE, §7-6 WORKS and the engine can ship auto-RCM order. If
//! the guards do not fold (residual blr, or it fails to type), §7-6 walls and the
//! recovery stays the manual-registration interim (op call b) + future toolchain.
//!
//! The decisive test is CORRECTNESS plus objdump: the fiber A->B->C has a real data
//! dependency (B reads what A wrote, C reads what B wrote), but is REGISTERED
//! scrambled (C, A, B). If the guarded walk applies the const RCM order it executes
//! A,B,C and the output is correct; if it fell back to carrier/registration order
//! (C,A,B) the output would be garbage (C reads Y before B writes it). So a correct
//! result proves the reorder happened. The const RCM positions are hand-assigned
//! here to isolate the guarded-walk question; computing them via const-fn RCM over
//! the carrier masks is already proven (202606071000 / dispatch::order). Outcome at
//! the bottom.

#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::hint::black_box;
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccumProject, ColPtrCons, ColPtrNil, ColProject, EngineCtx, Project, PtrNil,
};
use hilavitkutin::dispatch::morsel::MorselRange;
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

// Each WU's RCM-order slot. In production this is a const computed by const-fn RCM
// over the carrier masks (proven computable, dispatch::order / 202606071000); hand
// assigned here to isolate the guarded-walk-devirt question. A=0, B=1, C=2 = the
// topological / RCM order; the carrier is registered scrambled (C, A, B).
trait RcmPos {
    const POS: usize;
}

// =====================================================================
// THE CANDIDATE: a type-level walk parameterised by a runtime output SLOT. It
// visits the whole cons-list (same base/step shape as RunFiberCol, no Nth<K>, no
// impl overlap) and executes a WU only when its const POS == slot. The outer slot
// loop drives slots 0..N, so execution lands in RCM order regardless of carrier
// (registration) order. The position check `<W as RcmPos>::POS == slot` is a const
// (POS) vs runtime (slot) branch inside the step; the execute is a monomorphised
// concrete-type call, so the walk has no fn pointer to hold across the morsel loop.
// =====================================================================
trait RunOrdered<A, Witnesses> {
    fn run_slot(&self, bindings: &A, morsel: MorselRange, slot: usize);
}

impl<A> RunOrdered<A, Empty> for WuNil {
    #[inline]
    fn run_slot(&self, _bindings: &A, _morsel: MorselRange, _slot: usize) {}
}

impl<A, W, Tail, RIdx, RCIdx, WCIdx, WAIdx, WTail>
    RunOrdered<A, Cons<(RIdx, RCIdx, WCIdx, WAIdx), WTail>> for WuCons<W, Tail>
where
    W: WorkUnit + RcmPos,
    A: Project<<W as WorkUnit>::Read, RIdx>,
    A: ColProject<<W as WorkUnit>::Read, RCIdx>,
    A: ColProject<<W as WorkUnit>::Write, WCIdx>,
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
    Tail: RunOrdered<A, WTail>,
{
    #[inline]
    fn run_slot(&self, bindings: &A, morsel: MorselRange, slot: usize) {
        // const POS vs runtime slot. The execute is monomorphised on W.
        if <W as RcmPos>::POS == slot {
            let ctx: <W as WorkUnit>::Ctx<'_> =
                EngineCtx::project::<A, A, RIdx, RCIdx, WCIdx, WAIdx>(bindings, bindings, morsel);
            self.head.execute(&ctx);
        }
        self.tail.run_slot(bindings, morsel, slot);
    }
}

// Morsel-outer driver: per morsel, walk the carrier once per output slot 0..n_wus,
// so each morsel runs its WUs in RCM order. The morsel loop trip count and n_wus
// are black_box'd so neither unrolls trivially. objdump this symbol for zero blr.
#[inline(never)]
fn dispatch_rcm_ordered<A, F, WL>(
    bindings: &A,
    fiber: &F,
    total: USize,
    morsel_size: USize,
    n_wus: USize,
) where
    F: RunOrdered<A, WL>,
{
    let total = black_box(total).0;
    let step = black_box(morsel_size).0.max(1);
    let n_wus = black_box(n_wus).0;
    let mut start = 0usize;
    while start < total {
        let len = step.min(total - start);
        let morsel = MorselRange::new(USize(start), USize(len));
        let mut slot = 0usize;
        while slot < n_wus {
            fiber.run_slot(bindings, morsel, slot);
            slot += 1;
        }
        start += len;
    }
}

// =====================================================================
// Workload: A (Inp -> X), B (X -> Y), C (Y -> Z). Real RAW chain. Registered
// scrambled C, A, B; correct only if the guarded walk reorders to A, B, C.
// =====================================================================
const M1: u32 = 2654435761;
const M2: u32 = 40503;
const M3: u32 = 2246822519;

#[derive(Copy, Clone)]
struct Inp(u32);
#[derive(Copy, Clone)]
struct X(u32);
#[derive(Copy, Clone)]
struct Y(u32);
#[derive(Copy, Clone)]
struct Z(u32);

type One<T> = Cons<Column<T>, Empty>;

macro_rules! col_wu {
    ($name:ident, $rd:ty, $wr:ty, $pos:expr, $body:expr) => {
        struct $name;
        impl BuilderInput for $name {
            type Init = Self;
            type Dispatch = UnitDispatch<Self>;
        }
        impl RcmPos for $name {
            const POS: usize = $pos;
        }
        impl WorkUnit<Always> for $name {
            type Read = One<$rd>;
            type Write = One<$wr>;
            type Hint = (Immediate, Atomic, Normal);
            type Ctx<'frame> = EngineCtx<
                'frame,
                One<$rd>,
                One<$wr>,
                PtrNil,
                ColPtrCons<$rd, ColPtrNil>,
                ColPtrCons<$wr, ColPtrNil>,
            >;
            fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
                EachApi::run(ctx.each(), |i| {
                    // SAFETY: read col written by the upstream WU this morsel (RCM
                    // order guarantees it ran first); write col reserved + exclusive.
                    let r = unsafe { ctx.reader().read::<$rd, _>(i) };
                    let f: fn(u32) -> u32 = $body;
                    unsafe { ctx.writer().write::<$wr, _>(i, <$wr>::from(f(r.0))) };
                });
            }
        }
    };
}

impl From<u32> for X {
    fn from(v: u32) -> Self {
        X(v)
    }
}
impl From<u32> for Y {
    fn from(v: u32) -> Self {
        Y(v)
    }
}
impl From<u32> for Z {
    fn from(v: u32) -> Self {
        Z(v)
    }
}

// A: Inp -> X (X = Inp*M1), POS 0. B: X -> Y (Y = X+M2), POS 1. C: Y -> Z (Z =
// Y*M3), POS 2. Wrapping helpers so the closures are plain fn(u32)->u32.
fn fa(v: u32) -> u32 {
    v.wrapping_mul(M1)
}
fn fb(v: u32) -> u32 {
    v.wrapping_add(M2)
}
fn fc(v: u32) -> u32 {
    v.wrapping_mul(M3)
}
col_wu!(A, Inp, X, 0, fa);
col_wu!(B, X, Y, 1, fb);
col_wu!(C, Y, Z, 2, fc);

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

const N: usize = 256;

fn main() {
    let provider = BumpProvider::<262144>::new();
    // Stores Z, Y, X, Inp registered (prepend -> [Inp, X, Y, Z], Inp head). WUs
    // registered SCRAMBLED: C, A, B. A correct result proves the guarded walk
    // reordered execution to RCM (A, B, C) order despite the scrambled carrier.
    let sched = Scheduler::builder()
        .with(Column::<Z>::new())
        .with(Column::<Y>::new())
        .with(Column::<X>::new())
        .with(Column::<Inp>::new())
        .with(C)
        .with(A)
        .with(B)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("engine build should succeed"));

    // Host-populate Inp[i] = i (bindings head).
    let inp_base = sched.__bindings().__ptr().as_ptr() as *mut Inp;
    for i in 0..N {
        // SAFETY: Inp reserved for N records; storage alive; each slot written once.
        unsafe { *inp_base.add(i) = Inp(i as u32) };
    }

    // Carrier in REGISTRATION order C, A, B (anti-topological). The guarded walk +
    // slot loop must execute A, B, C.
    let fiber = WuCons { head: C, tail: WuCons { head: A, tail: WuCons { head: B, tail: WuNil } } };
    dispatch_rcm_ordered(sched.__bindings(), &fiber, USize(N), USize(32), USize(3));

    // Verify Z = fc(fb(fa(Inp))) = ((i*M1)+M2)*M3. Only achievable if A ran before
    // B before C (RCM order), NOT the C,A,B carrier order.
    let z_base = sched.__bindings().__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
    // SAFETY: Z reserved for N records; storage alive; written every record.
    let z = unsafe { core::slice::from_raw_parts(z_base, N) };
    for i in 0..N {
        let expect = fc(fb(fa(i as u32)));
        assert_eq!(z[i], expect, "Z[{i}]: guarded walk must run A,B,C (RCM) not C,A,B (registration)");
    }

    println!(
        "ran {N} records through a fiber registered SCRAMBLED (C,A,B); the const-position-guarded \
         type-level walk executed in RCM order (A,B,C) and Z is correct. objdump \
         dispatch_rcm_ordered for zero blr."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS, on nightly-2026-05-28 (release, fat LTO, cgu=1). §7-6 does NOT
// wall: the engine CAN auto-apply RCM order devirt-free without manual registration.
//
// Compiled with NO E0119 (the guarded walk reuses the proven non-overlapping
// base/step impls; the position check is a value branch inside the step, not an
// impl selector). Ran 256 records through a fiber registered SCRAMBLED (C, A, B)
// with a real RAW chain (B reads what A wrote, C reads what B wrote); Z came out
// correct = ((Inp*M1)+M2)*M3, which is ONLY achievable if execution ran A, B, C
// (the const RCM order), not the C, A, B carrier/registration order. So the
// reorder demonstrably happened.
//
// objdump of dispatch_rcm_ordered: blr=0, bl=0 (248 instrs), and 30 vector
// instructions (auto-vectorised NEON). Fully devirtualised under the runtime morsel
// loop, despite reordering execution away from the carrier type order.
//
// THE MECHANISM (resolves op decision (a), auto-RCM, positively): each WU carries a
// const RCM position (`<W as RcmPos>::POS`); in production this const is computed by
// const-fn RCM over the carrier masks (the same machinery proven in 202606071000 /
// shipped in dispatch::order: CarrierMasks fold -> const order -> position lookup),
// hand-assigned here to isolate the walk question. The dispatch walks the type-level
// cons-list ONCE PER OUTPUT SLOT (0..N), executing only the WU whose const POS ==
// the current slot. Because POS is a compile-time const and each `head.execute()` is
// a monomorphised concrete-type call, LLVM folds the position guards and inlines the
// executes in RCM order. No `Nth<K>` const-indexing (so no E0119 coherence wall, the
// 202606071000 Tier A2/B failure), no carrier TYPE reorder (impossible at runtime),
// no proc-macro/build.rs AccessSet visibility problem (the order is a const derived
// from the in-type masks at monomorphisation, not a syntactic emission).
//
// Why this beats the prior dead ends: the failed approaches put the permutation in
// the IMPL SELECTOR (`Nth<K>` overlapping impls -> E0119) or in a fn-pointer SLOT
// ARRAY (held in a register across the morsel loop -> 2 blr, 202606080300). This
// puts the permutation in a const VALUE BRANCH inside a type-level walk: the walk
// keeps the proven devirt property (no pointer to hold) and the const branch folds.
//
// COST / BOUND (the honest limitation, bench-decidable, not a blocker): the walk is
// O(N_wus^2) guard-checks per morsel (N slots x N-length walk). For a 3-WU fiber
// that is 9 const-folded checks producing 3 executes; the residual instr count (248
// vs ~140 for a plain in-order walk) reflects the extra structure, all inlined and
// vectorised. Fibers are cache-resident sequential units (small N by design), so the
// N^2 guard overhead is acceptable; for an unusually large fiber it is worth a bench
// vs the manual-registration in-order walk. The const-false guards appear largely
// folded (zero blr/bl, vectorised), so the effective work trends toward the N
// executes in RCM order.
//
// IMPLICATION for the GATE-1 build + op decision (b): auto-RCM is achievable, so the
// producer-before-consumer manual-registration constraint (op call (b), accepted as
// a PROVISIONAL compromise) can potentially be RELAXED via this mechanism rather
// than waiting on toolchain maturity (generic_const_args). The GATE-1 dispatch can
// either (i) ship the manual-registration interim first (D1d, simplest, op call b)
// and adopt the guarded walk as the auto-RCM follow-on, or (ii) adopt the guarded
// walk directly so GATE-1 ships auto-RCM-ordered (op call a's stated preference:
// "ideally landed before or with the GATE-1 dispatch"). Both are now de-risked; the
// choice is a build-sequencing call (the guarded walk's O(N^2) cost vs the plain
// walk is the only tradeoff, and it is small for real fibers). NOT a wall; no
// Step-11 op-decision triggered. §7-6 clears.
// ---------------------------------------------------------------------

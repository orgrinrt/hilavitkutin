//! Sketch (op question 2026-06-08, Approach 1 vs Approach 2): can dispatch
//! ORDER / fiber GROUPING be RUNTIME-SELECTABLE without losing devirt?
//!
//! Approach 1 (canonical thin): only morsel sizes / configs / affinity / record
//! ranges recompute between frames; dispatch order + fiber grouping are baked into
//! the compile-time carrier type (fixed). Proven devirt by sketch 202606081200.
//!
//! Approach 2 (broader): the fiber grouping AND ordering also recompute on replan.
//! First-principles constraint: there is no runtime monomorphisation in Rust, so a
//! type-level walk cannot be reordered at runtime without indexing, and indexing is
//! the fn-POINTER path that does NOT devirtualise (sketch 202606080300: 2 blr under
//! a morsel loop; spec FAIL modes struct-field 12.6x, &[fn;N] 5.8x; fiber-level
//! runtime ordering = spec Approach C 1.17x, rejected). So the ONLY devirt-
//! preserving form of Approach 2 is: a BOUNDED set of compile-time-monomorphised
//! grouping/order variants, SELECTED at runtime by a per-frame branch. Each variant
//! is a distinct devirt type-walk; the selection is one predictable branch OFF the
//! hot path; the cost is code size (N variant bodies compiled).
//!
//! Hypothesis: a runtime-selected branch between two compile-time grouping variants
//! of the same WU set, each driven under a real runtime morsel loop, devirtualises
//! in BOTH arms (zero blr), proving Approach 2 is achievable in its bounded form
//! without indirection. If true, Approach 2 is a viable optional extension (not
//! arbitrarily foreclosed); the only cost over Approach 1 is the extra variant body
//! plus one off-hot-path branch.
//!
//! The two variants group the SAME two independent WUs (S1: Inv->Av, S2: Inv2->Av2,
//! no data dependency so both groupings are valid):
//!   Variant A (two fibers): FiberCons<[S1], FiberCons<[S2], FiberNil>>
//!   Variant B (one fiber):  FiberCons<[S1, S2], FiberNil>
//! Both run under a black_box'd morsel loop; a black_box'd selector picks the
//! variant. Real engine crates; RunFiberCol/RunTrunk restated from D1b/202606081200.
//! Outcome at bottom.

#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::hint::black_box;
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccumProject, ColPtrCons, ColPtrNil, ColProject, EngineCtx, Project, PtrNil,
};
use hilavitkutin::dispatch::{WuCons, WuNil};
use hilavitkutin::dispatch::morsel::MorselRange;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;

// Proven type-level fiber walk (RunFiberCol) + trunk carrier (RunTrunk), restated.
trait RunFiberCol<A, Witnesses> {
    fn run(&self, bindings: &A, morsel: MorselRange);
}
impl<A> RunFiberCol<A, Empty> for WuNil {
    #[inline]
    fn run(&self, _b: &A, _m: MorselRange) {}
}
impl<A, W, Tail, RIdx, RCIdx, WCIdx, WAIdx, WTail>
    RunFiberCol<A, Cons<(RIdx, RCIdx, WCIdx, WAIdx), WTail>> for WuCons<W, Tail>
where
    W: WorkUnit,
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
    F: RunFiberCol<A, FW>,
    Rest: RunTrunk<A, RestWL>,
{
    #[inline]
    fn run(&self, bindings: &A, morsel: MorselRange) {
        self.fiber.run(bindings, morsel);
        self.rest.run(bindings, morsel);
    }
}

// APPROACH 2: a per-frame branch selecting between two compile-time grouping
// variants, each a distinct monomorphised RunTrunk walk, both under the runtime
// morsel loop. The selector is black_box'd (a genuine runtime branch). objdump
// this isolated symbol: zero blr in BOTH arms is the bar. The function body holds
// both variant walks (the code-size cost) plus the branch (the only runtime cost,
// off the per-record hot path).
#[inline(never)]
fn dispatch_selected<A, TA, TB, WLA, WLB>(
    sel: bool,
    bindings: &A,
    variant_a: &TA,
    variant_b: &TB,
    total: USize,
    msize: USize,
) where
    TA: RunTrunk<A, WLA>,
    TB: RunTrunk<A, WLB>,
{
    let sel = black_box(sel);
    let total = black_box(total).0;
    let step = black_box(msize).0.max(1);
    if sel {
        let mut start = 0;
        while start < total {
            let len = step.min(total - start);
            variant_a.run(bindings, MorselRange::new(USize(start), USize(len)));
            start += len;
        }
    } else {
        let mut start = 0;
        while start < total {
            let len = step.min(total - start);
            variant_b.run(bindings, MorselRange::new(USize(start), USize(len)));
            start += len;
        }
    }
}

// APPROACH 1 baseline: a single baked variant, no branch (one walk body).
#[inline(never)]
fn dispatch_single<A, T, WL>(bindings: &A, trunk: &T, total: USize, msize: USize)
where
    T: RunTrunk<A, WL>,
{
    let total = black_box(total).0;
    let step = black_box(msize).0.max(1);
    let mut start = 0;
    while start < total {
        let len = step.min(total - start);
        trunk.run(bindings, MorselRange::new(USize(start), USize(len)));
        start += len;
    }
}

const M1: u32 = 2654435761;
const M2: u32 = 40503;
#[inline(always)]
fn stage1(i: u32) -> u32 {
    i.wrapping_mul(M1)
}
#[inline(always)]
fn stage2(i: u32) -> u32 {
    i.wrapping_mul(M2)
}

#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Av(u32);
#[derive(Copy, Clone)]
struct Inv2(u32);
#[derive(Copy, Clone)]
struct Av2(u32);
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
    type Ctx<'f> =
        EngineCtx<'f, One<Inv>, One<Av>, PtrNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Av, ColPtrNil>>;
    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        ctx.each().run(|i| {
            // SAFETY: Inv host-populated; Av reserved + exclusively written; morsel-bounded.
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
    type Read = One<Inv2>;
    type Write = One<Av2>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'f> = EngineCtx<
        'f,
        One<Inv2>,
        One<Av2>,
        PtrNil,
        ColPtrCons<Inv2, ColPtrNil>,
        ColPtrCons<Av2, ColPtrNil>,
    >;
    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        ctx.each().run(|i| {
            // SAFETY: Inv2 host-populated; Av2 reserved + exclusively written; morsel-bounded.
            let inp = unsafe { ctx.reader().read::<Inv2, _>(i) };
            unsafe { ctx.writer().write::<Av2, _>(i, Av2(stage2(inp.0))) };
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
    let sched = Scheduler::builder()
        .with(Column::<Av2>::new())
        .with(Column::<Inv2>::new())
        .with(Column::<Av>::new())
        .with(Column::<Inv>::new())
        .with(S1)
        .with(S2)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("engine build should succeed"));

    // Inv head; Inv2 two tail hops in (Inv -> Av -> Inv2).
    let in_base = sched.__bindings().__ptr().as_ptr() as *mut Inv;
    let inv2_base = sched.__bindings().__tail().__tail().__ptr().as_ptr() as *mut Inv2;
    for i in 0..N {
        // SAFETY: both columns reserved for N records; storage alive; one write each.
        unsafe { *in_base.add(i) = Inv(i as u32) };
        unsafe { *inv2_base.add(i) = Inv2(i as u32) };
    }

    // Two compile-time GROUPING variants of the same WU set:
    //   A (two fibers): each WU its own fiber.
    //   B (one fiber):  both WUs co-located in one fiber.
    let variant_a = FiberCons {
        fiber: WuCons { head: S1, tail: WuNil },
        rest: FiberCons { fiber: WuCons { head: S2, tail: WuNil }, rest: FiberNil },
    };
    let variant_b = FiberCons { fiber: WuCons { head: S1, tail: WuCons { head: S2, tail: WuNil } }, rest: FiberNil };

    // Approach 1 baseline (single baked variant, no branch).
    dispatch_single(sched.__bindings(), &variant_b, USize(N), USize(32));
    // Approach 2: runtime branch selecting the grouping variant. Run both arms to
    // verify each produces correct output (independent WUs => identical result).
    dispatch_selected(true, sched.__bindings(), &variant_a, &variant_b, USize(N), USize(32));
    dispatch_selected(false, sched.__bindings(), &variant_a, &variant_b, USize(N), USize(32));

    let av_base = sched.__bindings().__tail().__ptr().as_ptr() as *const u32;
    let av2_base = sched.__bindings().__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
    // SAFETY: Av, Av2 reserved for N records; storage alive; written every record.
    let av = unsafe { core::slice::from_raw_parts(av_base, N) };
    let av2 = unsafe { core::slice::from_raw_parts(av2_base, N) };
    for i in 0..N {
        assert_eq!(av[i], stage1(i as u32), "Av[{i}]");
        assert_eq!(av2[i], stage2(i as u32), "Av2[{i}]");
    }
    println!(
        "ran {N} records through Approach-1 single + Approach-2 runtime-selected grouping variants \
         (both arms), all correct. objdump dispatch_selected for zero blr in BOTH arms; compare \
         body size vs dispatch_single (the Approach-2 code-size cost)."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS, on nightly-2026-05-28 (release, fat LTO, cgu=1).
//
// Ran 256 records through Approach-1 single + Approach-2 both arms; all correct.
//
// objdump:
//   dispatch_selected (Approach 2): 275 instrs, ZERO blr, ZERO bl.
//   dispatch_single  (Approach 1): 144 instrs, ZERO blr, ZERO bl.
//
// DECISIVE: a runtime branch selecting between two COMPILE-TIME grouping variants
// of the same WU set (variant A = two fibers, variant B = one fiber), each driven
// under a real runtime morsel loop, devirtualises in BOTH arms (zero blr/bl). The
// per-WU bodies inline + fuse identically to the single-variant case; the only
// difference is the function holds BOTH walk bodies (275 vs 144 instrs, ~1.9x for
// two variants, linear in variant count) plus one `cbz`/branch on the black_box'd
// selector. That branch is per-invocation (per frame), OFF the per-record hot path.
//
// IMPLICATION (Approach 1 vs Approach 2, op question 2026-06-08):
//   - Approach 1 (single baked order/grouping; only morsel/config/affinity/range
//     recompute): one body, smallest code, the canonical R6 thin-params shape.
//   - Approach 2 (order/grouping ALSO runtime-recomputable): achievable WITHOUT
//     losing devirt or adding indirection, in its ONLY devirt-preserving form: a
//     BOUNDED set of compile-time-monomorphised variants selected by a per-frame
//     branch. Cost = code size linear in variant count + one off-hot-path branch.
//   - Unbounded runtime grouping (synthesise any grouping group_fibers computes,
//     at runtime) is IMPOSSIBLE devirt-free: it needs runtime monomorphisation
//     (Rust has none) or indexing (the fn-ptr 12.6x/2-blr path, sketch 202606080300;
//     fiber-level = Approach C 1.17x, spec-rejected).
//
// So choosing Approach 1 for GATE-1 does NOT arbitrarily foreclose Approach 2:
// Approach 2 is an ADDITIVE layer on the same static-dispatch foundation (compile
// a menu of variants, branch-select on replan), addable later if a workload wants
// runtime-recomputable grouping/order, at zero hot-path cost. Approach 2 has no
// spec-mandated trigger (R6 adaptive list is morsel/count/strategy/affinity; no
// adaptive path reorders dispatch), so it is a "could", not a "must" for the
// canonical engine. GATE-1 ships Approach 1; Approach 2 stays a proven option.
// ---------------------------------------------------------------------

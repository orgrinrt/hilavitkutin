//! Sketch (D1b / #340, Phase D KEYSTONE): type-level per-fiber partition.
//!
//! Roadmap `202606061100_engine-completion-roadmap-draft.md` section 4 (D1b) and
//! section 9 name this the highest-risk, write-first sketch. The canonical
//! per-core program (consolidation domain 17, `:1596-1613`) carries, per physical
//! core, its phases, record ranges, and per-fiber devirtualised LOCAL `&[WuFn]`
//! slices. The keystone question (synthesis `202606060900` section 2.4, the
//! open "full type-level vs hybrid" fork): can the per-fiber partition be
//! expressed as a TYPE-LEVEL structure over `AccessSet` cons-lists, walked and
//! devirtualised, WITHOUT `generic_const_exprs`-extreme machinery on the pinned
//! nightly-2026-05-28?
//!
//! Hypothesis (section 9 premise + leeway SOME-SHAPE-IN-FAMILY): a type-level
//! encoding that CARRIES the per-fiber WU sequence (a cons-list of fibers, each a
//! cons-list of WUs) compiles, infers its witnesses, and devirtualises; AND the
//! data-dependency relation that defines a fiber boundary is type-expressible
//! from registered `AccessSet`s alone. If the full GROUPING fold (deriving the
//! partition purely from types) needs negative trait reasoning or GCE-extreme,
//! that resolves the hybrid-vs-full fork and, if it forces a macro-flattener,
//! triggers section-6.5 drift (Step 11 to op).
//!
//! Three tiers, each an independent finding:
//!   Tier 1 - the multi-fiber CARRIER. A `Trunk` = cons-list of fibers, each a
//!            `WuCons` list, walked by `RunTrunk` delegating to the proven
//!            `RunFiberCol` per fiber. Proves a type-level structure carrying N
//!            distinct per-fiber WU sequences devirtualises end to end.
//!   Tier 2 - the dependency PREDICATE. `SharesStore<Other>`: an `AccessSet`
//!            shares a member with another. WU B depends on WU A iff B::Read
//!            shares a store with A::Write. Proves the fiber-boundary relation is
//!            derivable from types alone, no GCE.
//!   Tier 3 - the full GROUPING fold (cons-list of WUs -> cons-list of fibers by
//!            data dependency). The genuinely novel derivation. Attempt + record
//!            the exact wall.
//!
//! Faithfulness: real `EngineCtx`/`Project`/`ColProject`/`AccumProject`,
//! `WuCons`/`WuNil`, `Cons`/`Empty`/`Contains`, `Scheduler`, `Accum`/`Column`.
//! New: `FiberCons`/`FiberNil` + `RunTrunk` (the outer carrier the D1 slice adds
//! to dispatch/), `SharesStore` (the boundary predicate). The inner `RunFiberCol`
//! is the proven shape from sketch 202606060730, restated here. Outcome at bottom.

#![allow(dead_code)]
#![feature(marker_trait_attr)]

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
use hilavitkutin_api::access::{Cons, Contains, Empty};
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

// =====================================================================
// Inner fiber walk: the proven RunFiberCol (sketch 202606060730), restated.
// Walks one fiber's WU cons-list, projecting each WU's EngineCtx (4 witnesses
// per WU: resource / read-col / write-col / accum), A-pinned by the caller.
// =====================================================================
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

// =====================================================================
// TIER 1: the multi-fiber CARRIER.
//
// A `Trunk` is a cons-list of fibers: `FiberCons<F, Rest>` / `FiberNil`. The
// per-core program is exactly this shape, one Trunk per core, each Trunk a
// type-level list of fibers, each fiber a type-level list of WUs. `RunTrunk`
// walks the outer list, delegating each fiber to the proven `RunFiberCol`. The
// witness parameter is itself a cons-list: head = this fiber's per-WU witnesses,
// tail = the rest. This is the structure that "carries the per-fiber WU
// sequence" (section 9 leeway) for an arbitrary number of fibers.
// =====================================================================
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
    fn run(&self, _bindings: &A, _morsel: MorselRange) {}
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

// The `A`-pin (the `Scheduler::run<Witnesses>` shape): `A` fixed by `Self`
// before the nested witness cons-list is inferred at the `.drive_trunk` call.
struct Harness<'b, A> {
    bindings: &'b A,
}

impl<'b, A> Harness<'b, A> {
    #[inline]
    fn drive_trunk<T, WL>(&self, trunk: &T, morsel: MorselRange)
    where
        T: RunTrunk<A, WL>,
    {
        trunk.run(self.bindings, morsel);
    }
}

// Isolated dispatch symbol for the asm-checklist (bench-acceptance method, as in
// sketch 202606060500). ONLY the type-level Trunk walk, no build/setup/print.
// `#[inline(never)]` keeps it a standalone symbol; the inner `#[inline]`
// RunTrunk/RunFiberCol/execute calls still fold INTO it. objdump the monomorphised
// `d1b_dispatch_trunk` symbol: zero `blr` (indirect call) is the devirt bar.
#[inline(never)]
fn d1b_dispatch_trunk<A, T, WL>(bindings: &A, trunk: &T, morsel: MorselRange)
where
    T: RunTrunk<A, WL>,
{
    let harness = Harness { bindings };
    harness.drive_trunk(trunk, morsel);
}

// =====================================================================
// TIER 2: the dependency PREDICATE.
//
// `SharesStore<Other>`: `Self` (an AccessSet cons-list) shares at least one
// member type with `Other` (another AccessSet). The fiber-boundary relation:
// WU B is data-dependent on WU A iff `B::Read: SharesStore<A::Write>` (B reads a
// store A writes). Expressed as a `#[marker]` trait (same mechanism `Contains`
// uses) so the head-match and tail-recurse impls coexist without coherence
// conflict. No GCE, no const generics, no negative reasoning for the POSITIVE
// direction.
// =====================================================================
#[marker]
trait SharesStore<Other> {}

// Head shared: Self = Cons<H, T>, and Other contains H.
impl<H: 'static, T: 'static, Other> SharesStore<Other> for Cons<H, T> where Other: Contains<H> {}
// Tail shares: Self = Cons<H, T>, and T shares with Other.
impl<H: 'static, T: 'static, Other> SharesStore<Other> for Cons<H, T> where T: SharesStore<Other> {}
// `Empty` shares nothing: no impl. A non-dependent pair fails to resolve the
// bound (verified by the assert_dep call sites below: only sharing pairs compile).

/// Resolves only when `RB` (a Read set) shares a store with `WA` (a Write set),
/// i.e. only when the dependency holds. Calling it on a non-sharing pair fails
/// to compile (the negative case; see the documented block below).
#[inline]
fn assert_dep<RB, WA>()
where
    RB: SharesStore<WA>,
{
}

// =====================================================================
// Workload. Two INDEPENDENT fibers proving the partition is non-trivial:
//   Fiber 1: S1 (read Inv,  write Av ) -> Tally (read Av, append Accum<Sum>).
//            Tally depends on S1 (Av shared). One fiber.
//   Fiber 2: S2 (read Inv2, write Av2). Shares no store with fiber 1. Separate.
// The Trunk = FiberCons<Fiber1, FiberCons<Fiber2, FiberNil>>.
// =====================================================================
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
struct Sum(u32);
#[derive(Copy, Clone)]
struct Inv2(u32);
#[derive(Copy, Clone)]
struct Av2(u32);

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
            // SAFETY: Inv host-populated for the record count; Av reserved and
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
            // SAFETY: build reserved Accum<Sum> for the record count; the plan
            // proved this unit the exclusive appender; one append per record.
            unsafe { ctx.accums().append::<Sum, _>(Sum(a.0)) };
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
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<Inv2>,
        One<Av2>,
        PtrNil,
        ColPtrCons<Inv2, ColPtrNil>,
        ColPtrCons<Av2, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: Inv2 host-populated; Av2 reserved + exclusively written.
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
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

const N: usize = 64;

fn main() {
    // -----------------------------------------------------------------
    // TIER 2 first (compile-time): the dependency predicate resolves for the
    // real WU AccessSets exactly where the data dependency holds.
    // -----------------------------------------------------------------
    // POSITIVE: Tally::Read (One<Av>) shares a store with S1::Write (One<Av>).
    // Tally depends on S1 -> same fiber. Resolves.
    assert_dep::<<Tally as WorkUnit>::Read, <S1 as WorkUnit>::Write>();
    // POSITIVE: S1::Read (One<Inv>) does NOT share with S1::Write, but DOES with
    // a host-input set; demonstrate sharing across a multi-store set.
    assert_dep::<Cons<Column<Av>, One<Inv>>, One<Inv>>();
    // NEGATIVE (documented, cannot assert non-impl in-crate without a separate
    // trybuild target): S2::Read (One<Inv2>) shares NO store with S1::Write
    // (One<Av>). The line below fails to compile (no `SharesStore` impl chains
    // Inv2 to Av), which is the fiber-boundary signal that S2 starts a new fiber:
    //     assert_dep::<<S2 as WorkUnit>::Read, <S1 as WorkUnit>::Write>();
    // Verified by uncommenting locally: E0277 `Cons<Column<Inv2>, Empty>: SharesStore<...>`
    // is not satisfied. Recorded as the boundary detector's negative arm.

    // -----------------------------------------------------------------
    // TIER 1 (compile + run + devirt): build the two-fiber Trunk and drive it.
    // -----------------------------------------------------------------
    let provider = BumpProvider::<65536>::new();
    // Register all five stores, then the three units. Prepend order makes Inv the
    // bindings head. Registration order is independent of fiber grouping (the
    // Trunk carrier names the per-fiber sequence explicitly).
    let sched = Scheduler::builder()
        .with(Accum::<Sum>::new())
        .with(Column::<Av2>::new())
        .with(Column::<Inv2>::new())
        .with(Column::<Av>::new())
        .with(Column::<Inv>::new())
        .with(S1)
        .with(Tally)
        .with(S2)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("engine build should succeed"));

    // Host-populate Inv[i] = i (bindings head) and Inv2[i] = i (its tail slot).
    // SAFETY: both columns reserved for N records of u32-repr; storage alive;
    // each reserved slot written once. Inv is the head; Inv2 is two tail hops in
    // (after Inv, Av come Inv2). Walk the binding tail chain for Inv2's base.
    let in_base = sched.__bindings().__ptr().as_ptr() as *mut Inv;
    for i in 0..N {
        unsafe { *in_base.add(i) = Inv(i as u32) };
    }
    // bindings: Inv -> Av -> Inv2 -> Av2 -> Sum (prepend order reversed).
    let inv2_base = sched.__bindings().__tail().__tail().__ptr().as_ptr() as *mut Inv2;
    for i in 0..N {
        unsafe { *inv2_base.add(i) = Inv2(i as u32) };
    }

    // The Trunk: two fibers carried at the type level.
    //   Fiber 1: S1 -> Tally -> nil.
    //   Fiber 2: S2 -> nil.
    let fiber1 = WuCons { head: S1, tail: WuCons { head: Tally, tail: WuNil } };
    let fiber2 = WuCons { head: S2, tail: WuNil };
    let trunk = FiberCons { fiber: fiber1, rest: FiberCons { fiber: fiber2, rest: FiberNil } };

    // Drive through the isolated dispatch symbol (asm-checklist target). Witness
    // cons-list inferred with no turbofish (the run<Witnesses> shape).
    d1b_dispatch_trunk(sched.__bindings(), &trunk, MorselRange::new(USize(0), USize(N)));

    // Verify fiber 1: S1 wrote Av = stage1(i); Tally appended one Sum per record.
    let av_base = sched.__bindings().__tail().__ptr().as_ptr() as *const u32;
    // SAFETY: Av reserved for N records; storage alive; S1 wrote every record.
    let av = unsafe { core::slice::from_raw_parts(av_base, N) };
    for i in 0..N {
        assert_eq!(av[i], stage1(i as u32), "Av[{i}] mismatch (fiber 1, S1)");
    }
    // Verify fiber 2: S2 wrote Av2 = stage2(i).
    let av2_base =
        sched.__bindings().__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
    // SAFETY: Av2 reserved for N records; storage alive; S2 wrote every record.
    let av2 = unsafe { core::slice::from_raw_parts(av2_base, N) };
    for i in 0..N {
        assert_eq!(av2[i], stage2(i as u32), "Av2[{i}] mismatch (fiber 2, S2)");
    }
    // Accumulator (deepest tail: Sum, registered first).
    let sum_binding = sched.__bindings().__tail().__tail().__tail().__tail();
    let sum_len = sum_binding.__len_cell().get().0;
    assert_eq!(sum_len, N, "accum live length should be N (fiber 1, Tally)");
    let sum_base = sum_binding.__ptr().as_ptr() as *const u32;
    // SAFETY: Sum reserved for N records; storage alive; Tally appended N values.
    let sums = unsafe { core::slice::from_raw_parts(sum_base, N) };
    for i in 0..N {
        assert_eq!(sums[i], stage1(i as u32), "Sum[{i}] mismatch (fiber 1, Tally)");
    }

    println!(
        "WORKS: Tier 1 - two-fiber type-level Trunk (S1->Tally | S2) drove {N} records, all \
         columns + accumulator correct. Tier 2 - SharesStore dependency predicate resolves the \
         positive (Tally::Read shares S1::Write) and rejects the negative (S2::Read vs S1::Write) \
         from AccessSets alone, no GCE."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (Tiers 1 + 2), on nightly-2026-05-28.
//
// Tier 1 (CARRIER): the two-fiber type-level Trunk (FiberCons<S1->Tally,
// FiberCons<S2, FiberNil>>) compiled, inferred its nested witness cons-list with
// no turbofish, ran 64 records correct (Av, Av2 columns + Sum accumulator), and
// the isolated `d1b_dispatch_trunk` symbol objdumps to ZERO `blr` (indirect
// calls): the inner RunTrunk/RunFiberCol/execute all fold in, leaving a flat
// `ldr/mul/str` body with the M1 constant (0x9e3779b1) baked. This is the
// canonical per-core-program shape (domain 17). A type-level structure CARRYING N
// distinct per-fiber WU sequences devirtualises end to end. No GCE.
//
// Tier 2 (PREDICATE): `SharesStore<Other>` resolves the positive (Tally::Read
// shares S1::Write -> same fiber) and the negative fails to compile (E0277:
// `Cons<Column<Inv2>, Empty>: SharesStore<Cons<Column<Av>, Empty>>` is not
// satisfied -> S2 starts a new fiber). The fiber-boundary relation is derivable
// from registered AccessSets alone, no GCE, via the same `#[marker]` mechanism
// `Contains` uses.
//
// Tier 3 (full GROUPING DERIVATION): see tier3.rs. FAILS WITH E0119 (needs
// `specialization`, forbidden). Fork resolves toward HYBRID: plan-computed
// grouping + type-carried per-core program. The hybrid-vs-6.5-drift call is the
// domain-expert consensus item (op asleep). The KEYSTONE devirt premise stands.
// ---------------------------------------------------------------------

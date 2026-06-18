//! GATE-2 carrier-mechanism sketch: const-gated DCE trunk isolation.
//!
//! op mandate 2026-06-07 (memory gate2-carrier-mechanism-mandate; fork doc
//! mock/research/202606071200_gate2-carrier-mechanism-fork.md). Building a nested
//! `PhaseCons<TrunkCons<FiberCons<WuCons>>>` carrier TYPE from flat registration
//! walls: the grouping is global graph connectivity (not a local type fold) and
//! the partition boundary needs forbidden specialization. op chose the codegen
//! flattener direction but wants the mechanism to stay in rustc/Rust.
//!
//! Hypothesis: express the partition as CONST DATA (a `GROUPING` array, in the
//! real engine produced by a const fn over collected access masks) and gate each
//! carrier position by `const { GROUPING[POS] == THIS_TRUNK }`. Because the gate
//! is const, DCE removes every non-member position's dispatch, so each per-TRUNK
//! monomorphisation contains ONLY its member WUs' code. That is the codegen
//! flattener's EFFECT (isolated per-trunk programs) via const-eval + DCE, in pure
//! Rust. This sketch hardcodes `GROUPING` (the const-fn computation is a separate,
//! feasible step) and tests the load-bearing unknown:
//!   (a) each per-TRUNK mono DCEs to MEMBER-ONLY (trunk 0's mono has no trunk-1
//!       WU code, and vice versa), proving real isolation, not a runtime branch;
//!   (b) zero blr in each per-trunk mono (devirt preserved through the const gate);
//!   (c) running every trunk (sequentially, single core) is output-equivalent to
//!       the flat whole-program walk.
//!
//! Workload (mirrors Sketch A): carrier [SX, SY, SZ]. SX: InX->AX, SZ: AX->CZ
//! (SZ depends on SX, same trunk). SY: InY->AY, column-disjoint (its own trunk).
//! GROUPING = [0, 1, 0]: SX and SZ in trunk 0, SY in trunk 1. Single-core runs
//! trunk 0 then trunk 1; at N cores each trunk is one core's program (Sketch B
//! shape), zero sync (disjoint write columns).
//!
//! POS is threaded as a const generic through the walk (the real mechanism indexes
//! the const grouping by carrier position); `{ POS + 1 }` in the recursive bound
//! needs `generic_const_exprs` (WATCH-allowed, unstable-features.md). Outcome at
//! the bottom.

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, PtrNil};
use hilavitkutin::dispatch::fiber_run::RunFiber; // the SHIPPED per-fiber walk
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

// =====================================================================
// The const-gated per-trunk walk. Walks the FLAT carrier; each position's
// dispatch is gated by `const { GROUPING[POS] == TRUNK }`. The gate is const, so
// the non-member arm is dead and DCE removes it: each TRUNK mono carries only its
// member WUs' code. POS threads through as a const generic; each head dispatches
// as a single-WU `WuCons<H, WuNil>` through the shipped RunFiber.
// =====================================================================

// Hardcoded grouping (the real engine computes this with a const fn over the
// collected per-WU access masks; here it stands in to test the gate+DCE). Carrier
// position -> trunk id. SX=0 (pos 0), SY=1 (pos 1), SZ=0 (pos 2).
const GROUPING: [u64; 3] = [0, 1, 0];

// Indexing is not allowed directly in a generic `const {}` block (rustc:
// "indexing is not supported in generic constants"), so the lookup lives in a
// const fn. This is also where the real engine's const-fn grouping computation
// would live: `trunk_of` becomes "run the grouping over the collected access
// masks, return position POS's trunk id".
const fn trunk_of(pos: usize) -> u64 {
    GROUPING[pos]
}

trait RunTrunkSel<A, WL, const POS: usize, const TRUNK: u64> {
    fn run(&self, bindings: &A, morsel: MorselRange);
}

impl<A, const POS: usize, const TRUNK: u64> RunTrunkSel<A, Empty, POS, TRUNK> for WuNil {
    #[inline]
    fn run(&self, _b: &A, _m: MorselRange) {}
}

impl<A, H, T, HFib, TW, const POS: usize, const TRUNK: u64>
    RunTrunkSel<A, Cons<HFib, TW>, POS, TRUNK> for WuCons<H, T>
where
    H: Copy,
    WuCons<H, WuNil>: RunFiber<A, HFib>,
    T: RunTrunkSel<A, TW, { POS + 1 }, TRUNK>,
{
    #[inline]
    fn run(&self, bindings: &A, morsel: MorselRange) {
        if const { trunk_of(POS) == TRUNK } {
            // This position belongs to TRUNK: dispatch it as a single-WU fiber
            // through the shipped RunFiber. For a non-member position this whole
            // block is dead (const-false gate) and DCE removes it.
            let single = WuCons { head: self.head, tail: WuNil };
            RunFiber::run(&single, bindings, morsel);
        }
        self.tail.run(bindings, morsel);
    }
}

// A-pinned harness mirroring Scheduler::run<Witnesses>: `A` is fixed by the
// bindings ref, the witness list + POS=0 + TRUNK are inferred / supplied at the
// call. `#[inline(never)]` so each (carrier, TRUNK) mono is an isolated symbol to
// objdump.
#[inline(never)]
fn run_one_trunk<A, C, WL, const TRUNK: u64>(bindings: &A, carrier: &C, morsel: MorselRange)
where
    C: RunTrunkSel<A, WL, 0, TRUNK>,
{
    carrier.run(bindings, morsel);
}

// =====================================================================
// Workload WUs (mirror Sketch A). Three pure per-record maps, column-only.
// =====================================================================
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
            // SAFETY: InX host-populated; AX reserved + exclusively written; morsel-bounded.
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

const N: usize = 256;

fn main() {
    let provider = BumpProvider::<262144>::new();
    // Register columns CZ, AY, AX, InY, InX (prepend order: InX is bindings head).
    // Then units SX, SY, SZ: the flat carrier [SX, SY, SZ] is topological.
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

    // Host-populate InX[i] = i (bindings head) and InY[i] = i (its tail, after AX).
    // bindings prepend-reverse order: InX -> InY -> AX -> AY -> CZ.
    let inx_base = sched.__bindings().__ptr().as_ptr() as *mut InX;
    for i in 0..N {
        // SAFETY: InX reserved for N records; storage alive; one write each.
        unsafe { *inx_base.add(i) = InX(i as u32) };
    }
    let iny_base = sched.__bindings().__tail().__ptr().as_ptr() as *mut InY;
    for i in 0..N {
        // SAFETY: InY reserved for N records; storage alive; one write each.
        unsafe { *iny_base.add(i) = InY(i as u32) };
    }

    // The flat carrier value: WuCons<SX, WuCons<SY, WuCons<SZ, WuNil>>>.
    let carrier = WuCons { head: SX, tail: WuCons { head: SY, tail: WuCons { head: SZ, tail: WuNil } } };
    let morsel = MorselRange::new(USize(0), USize(N));

    // Run every trunk (single core, sequential). At N cores each call is one
    // core's isolated program. TRUNK 0 = {SX, SZ}, TRUNK 1 = {SY}. The walk visits
    // all positions but the const gate keeps only its trunk's dispatch in each mono.
    run_one_trunk::<_, _, _, 0>(sched.__bindings(), &carrier, morsel);
    run_one_trunk::<_, _, _, 1>(sched.__bindings(), &carrier, morsel);

    // Output-equivalence: AX = fx(InX), AY = fy(InY), CZ = fz(AX).
    let ax_base = sched.__bindings().__tail().__tail().__ptr().as_ptr() as *const u32;
    let ay_base = sched.__bindings().__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
    let cz_base =
        sched.__bindings().__tail().__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
    // SAFETY: AX, AY, CZ each reserved for N records; storage alive; written every record.
    let ax = unsafe { core::slice::from_raw_parts(ax_base, N) };
    let ay = unsafe { core::slice::from_raw_parts(ay_base, N) };
    let cz = unsafe { core::slice::from_raw_parts(cz_base, N) };
    for i in 0..N {
        assert_eq!(ax[i], fx(i as u32), "AX[{i}] (trunk 0, SX)");
        assert_eq!(ay[i], fy(i as u32), "AY[{i}] (trunk 1, SY)");
        assert_eq!(cz[i], fz(fx(i as u32)), "CZ[{i}] (trunk 0, SZ reads SX's AX)");
    }

    println!(
        "WORKS: const-gated flat walk ran trunk 0 {{SX, SZ}} then trunk 1 {{SY}}, output \
         equal to the flat topological walk (AX=fx, AY=fy, CZ=fz(AX)). Each per-TRUNK mono \
         is gated by `const {{ GROUPING[POS] == TRUNK }}`. objdump run_one_trunk::<..,0> and \
         <..,1>: expect MEMBER-ONLY (trunk 0 has SX+SZ work via fx/fz, no SY/fy; trunk 1 has \
         SY/fy, no SX/SZ) and zero blr in each."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28, release, fat LTO, cgu=1).
//
// Ran 256 records. Output bit-equal to the flat topological walk: AX[i]=fx(i),
// AY[i]=fy(i), CZ[i]=fz(fx(i)). The const-gated flat walk ran trunk 0 {SX, SZ}
// then trunk 1 {SY}, single core.
//
// DCE ISOLATION CONFIRMED (the load-bearing result). objdump of the two isolated
// `run_one_trunk` monos:
//   TRUNK 0 (members SX + SZ): ~354 instrs, M1/fx constant (0x9e37) PRESENT,
//     M2/fy constant (0x85eb) ABSENT. blr=0, br=0, bl=0.
//   TRUNK 1 (member SY):       ~130 instrs, M2/fy constant PRESENT,
//     M1/fx constant ABSENT.   blr=0, br=0, bl=0.
// Each per-TRUNK mono carries ONLY its member WUs' machine code: trunk 0 has the
// fx/fz path and no trace of SY's fy; trunk 1 the reverse. The const gate
// `const { trunk_of(POS) == TRUNK }` makes the non-member arm dead, and DCE
// removes it. This is REAL isolation (separate per-trunk programs), not a runtime
// branch (which would keep every body behind a predicate, the rejected option B).
// The instruction counts track member count (2 WUs vs 1). Zero blr = devirt
// preserved; bl=0 = full straight-line fold under fat LTO.
//
// SETTLES (op mandate 2026-06-07, the in-Rust mechanism question): isolated,
// devirt-clean, member-only per-trunk dispatch programs ARE producible from a
// FLAT registration in PURE RUST, with no proc-macro, no build.rs, no LLVM pass.
// The mechanism: a `POS`-threaded walk (generic_const_exprs for {POS+1}) gating
// each carrier position by `const { trunk_of(POS) == TRUNK }`, where `trunk_of`
// is a const fn. N trunks = N monomorphisations of `run_one_trunk::<.., TRUNK>`,
// each DCE'd to its members; run one per core, zero sync (disjoint write columns,
// Sketch B's concurrency shape). This realises op's "express the partitions
// without the typestate, enough for codegen to materialise them" via const-eval +
// DCE.
//
// WHAT THIS DOES NOT YET SETTLE (next sketch): `trunk_of` / GROUPING is hardcoded
// here. The real engine must compute it at COMPILE TIME from the registered WUs'
// access masks: a type-level COLLECTION walk (no partition, like RunFiber) gathers
// each WU's const READ/WRITE mask into a const array, then a const fn runs the
// graph grouping (column-disjoint connectivity) over that array to produce the
// per-position trunk id. The collection-to-const + const-fn-grouping half is the
// remaining unknown; it is const-fn engineering (feasible per analysis) but must
// be proven by its own sketch before the roadmap rewrite.
// ---------------------------------------------------------------------

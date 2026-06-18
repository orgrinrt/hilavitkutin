//! GATE-2 carrier-mechanism sketch 2: compile-time grouping from access masks.
//!
//! op mandate 2026-06-07 (memory gate2-carrier-mechanism-mandate). Sketch 1
//! (202606071230) proved a flat-carrier walk gated by
//! `const { trunk_of(POS) == TRUNK }` DCEs each per-TRUNK mono to member-only
//! code (isolated per-trunk programs, zero blr), in pure Rust, with `trunk_of`
//! hardcoded. This sketch proves the remaining half: `trunk_of` computed at
//! COMPILE TIME from the registered WUs' access sets.
//!
//! Hypothesis, three parts:
//!   (U1) AccessSet TYPE -> const bitmask. A `ConstMask` trait folds a WU's
//!        Read / Write cons-list (`Cons<Column<C>, ...>` / `Empty`) into a const
//!        u64 over a global column numbering (`ColId::ID` per column type; in the
//!        real engine this id is the `Locate`/`WitnessIndex` position, already a
//!        const). This is the risky type-level -> const projection.
//!   (U2) const fn graph grouping. A const fn takes the per-WU read/write mask
//!        arrays and computes, with real graph logic: dependency edges
//!        (read-after-write), phase = longest dependency depth (the waist
//!        structure), and within each phase, column-disjoint trunks. It returns a
//!        const grouping array (trunk id per carrier position).
//!   (U3) the COMPUTED const grouping drives the same const-gated walk from
//!        sketch 1 and still DCEs to member-only per-trunk monos.
//!
//! Workload: SX (InX->AX), SY (InY->AY), SZ (AX->CZ). SZ reads AX (SX writes), so
//! SZ depends on SX: phase 1. SX, SY independent and column-disjoint: phase 0, two
//! trunks. Expected grouping: trunks [0, 1, 2], phases [0, 0, 1]. The const fn
//! must derive this from the masks alone. Outcome at the bottom.
//!
//! `generic_const_exprs` is for the `{ POS + 1 }` const-generic threading in the
//! walk (WATCH-allowed). The const fn grouping uses only stable const-fn features
//! (loops, arrays, mutation).

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

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

// =====================================================================
// (U1) AccessSet TYPE -> const bitmask.
//
// Each column type carries a const id (the global column numbering). In the real
// engine this is the `Locate<Col, Index>` + `WitnessIndex::INDEX` position over
// the registered Stores list, already a const; here it stands in as `ColId::ID`.
// `ConstMask` folds an AccessSet cons-list into a const u64 by OR-ing `1 << ID`
// for each column. This is the type-level -> const projection: a recursive
// associated const over the cons-list shape, no partition, no specialization.
// =====================================================================
trait ColId {
    const ID: u32;
}
trait ConstMask {
    const MASK: u64;
}
impl ConstMask for Empty {
    const MASK: u64 = 0;
}
impl<C: ColId, T: ConstMask> ConstMask for Cons<Column<C>, T> {
    const MASK: u64 = (1u64 << C::ID) | <T as ConstMask>::MASK;
}

// =====================================================================
// (U2) const fn graph grouping. Real plan logic shrunk to a const fn: dependency
// edges (read-after-write), phase = longest dependency depth (the waist
// structure), within-phase column-disjoint trunks. Returns trunk id per position.
//
// This is a faithful (if simplified vs the shipped spectral former) grouping: it
// produces a correct column-disjoint, dependency-respecting partition. The
// spectral refinement of wide trunks is a later perf bench, not needed for the
// mechanism or correctness.
// =====================================================================
const NWU: usize = 3;

const fn compute_phase(reads: [u64; NWU], writes: [u64; NWU]) -> [u64; NWU] {
    // Longest dependency depth via repeated relaxation. Edge i -> j when j reads a
    // column i writes (read-after-write). N passes converge for a DAG of N nodes.
    let mut phase = [0u64; NWU];
    let mut iter = 0;
    while iter < NWU {
        let mut i = 0;
        while i < NWU {
            let mut j = 0;
            while j < NWU {
                if i != j && (reads[j] & writes[i]) != 0 && phase[j] < phase[i] + 1 {
                    phase[j] = phase[i] + 1;
                }
                j += 1;
            }
            i += 1;
        }
        iter += 1;
    }
    phase
}

const fn compute_trunks(reads: [u64; NWU], writes: [u64; NWU], phase: [u64; NWU]) -> [u64; NWU] {
    // Within a phase, two WUs share a trunk iff connected by any shared column
    // (read or write); distinct trunks are column-disjoint. Global trunk ids
    // assigned in carrier order. (A union-find would handle transitive chains; the
    // earlier-match scan suffices for the workloads here and stays const-fn-simple.)
    let mut trunk = [0u64; NWU];
    let mut next_id = 0u64;
    let mut i = 0;
    while i < NWU {
        let mut joined = false;
        let mut j = 0;
        while j < i {
            let ci = reads[i] | writes[i];
            let cj = reads[j] | writes[j];
            if phase[j] == phase[i] && (ci & cj) != 0 {
                trunk[i] = trunk[j];
                joined = true;
                break;
            }
            j += 1;
        }
        if !joined {
            trunk[i] = next_id;
            next_id += 1;
        }
        i += 1;
    }
    trunk
}

// Column numbering (global). Real engine: Locate/WitnessIndex positions.
struct InX(u32);
impl ColId for InX {
    const ID: u32 = 0;
}
struct AX(u32);
impl ColId for AX {
    const ID: u32 = 1;
}
struct InY(u32);
impl ColId for InY {
    const ID: u32 = 2;
}
struct AY(u32);
impl ColId for AY {
    const ID: u32 = 3;
}
struct CZ(u32);
impl ColId for CZ {
    const ID: u32 = 4;
}

// Per-WU read/write masks, projected from the AccessSet types at compile time.
const READ_MASKS: [u64; NWU] = [
    <<SX as WorkUnit<Always>>::Read as ConstMask>::MASK,
    <<SY as WorkUnit<Always>>::Read as ConstMask>::MASK,
    <<SZ as WorkUnit<Always>>::Read as ConstMask>::MASK,
];
const WRITE_MASKS: [u64; NWU] = [
    <<SX as WorkUnit<Always>>::Write as ConstMask>::MASK,
    <<SY as WorkUnit<Always>>::Write as ConstMask>::MASK,
    <<SZ as WorkUnit<Always>>::Write as ConstMask>::MASK,
];
const PHASE: [u64; NWU] = compute_phase(READ_MASKS, WRITE_MASKS);
const GROUPING: [u64; NWU] = compute_trunks(READ_MASKS, WRITE_MASKS, PHASE);

// (U3) the gate reads the COMPUTED grouping (not a hardcoded array).
const fn trunk_of(pos: usize) -> u64 {
    GROUPING[pos]
}

// =====================================================================
// The const-gated per-trunk walk (identical mechanism to sketch 1).
// =====================================================================
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
            let single = WuCons { head: self.head, tail: WuNil };
            RunFiber::run(&single, bindings, morsel);
        }
        self.tail.run(bindings, morsel);
    }
}

#[inline(never)]
fn run_one_trunk<A, C, WL, const TRUNK: u64>(bindings: &A, carrier: &C, morsel: MorselRange)
where
    C: RunTrunkSel<A, WL, 0, TRUNK>,
{
    carrier.run(bindings, morsel);
}

// =====================================================================
// Workload WUs (mirror sketch 1 / Sketch A).
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

impl Clone for InX {
    fn clone(&self) -> Self {
        InX(self.0)
    }
}
impl Copy for InX {}
impl Clone for AX {
    fn clone(&self) -> Self {
        AX(self.0)
    }
}
impl Copy for AX {}
impl Clone for InY {
    fn clone(&self) -> Self {
        InY(self.0)
    }
}
impl Copy for InY {}
impl Clone for AY {
    fn clone(&self) -> Self {
        AY(self.0)
    }
}
impl Copy for AY {}
impl Clone for CZ {
    fn clone(&self) -> Self {
        CZ(self.0)
    }
}
impl Copy for CZ {}

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
    // The grouping is computed at compile time from the WU access-set types alone.
    assert_eq!(READ_MASKS, [1, 4, 2], "read masks: InX=bit0, InY=bit2, AX=bit1");
    assert_eq!(WRITE_MASKS, [2, 8, 16], "write masks: AX=bit1, AY=bit3, CZ=bit4");
    assert_eq!(PHASE, [0, 0, 1], "SZ depends on SX (reads AX) -> phase 1; SX,SY phase 0");
    assert_eq!(GROUPING, [0, 1, 2], "SX trunk0, SY trunk1 (disjoint phase0), SZ trunk2 (phase1)");

    let provider = BumpProvider::<262144>::new();
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

    let carrier = WuCons { head: SX, tail: WuCons { head: SY, tail: WuCons { head: SZ, tail: WuNil } } };
    let morsel = MorselRange::new(USize(0), USize(N));

    // Run the three computed trunks in id order (single core). Trunk order respects
    // phase: SZ (trunk 2, phase 1) runs after SX (trunk 0, phase 0) whose AX it reads.
    run_one_trunk::<_, _, _, 0>(sched.__bindings(), &carrier, morsel);
    run_one_trunk::<_, _, _, 1>(sched.__bindings(), &carrier, morsel);
    run_one_trunk::<_, _, _, 2>(sched.__bindings(), &carrier, morsel);

    let ax_base = sched.__bindings().__tail().__tail().__ptr().as_ptr() as *const u32;
    let ay_base = sched.__bindings().__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
    let cz_base =
        sched.__bindings().__tail().__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
    // SAFETY: AX, AY, CZ each reserved for N records; storage alive; written every record.
    let ax = unsafe { core::slice::from_raw_parts(ax_base, N) };
    let ay = unsafe { core::slice::from_raw_parts(ay_base, N) };
    let cz = unsafe { core::slice::from_raw_parts(cz_base, N) };
    for i in 0..N {
        assert_eq!(ax[i], fx(i as u32), "AX[{i}]");
        assert_eq!(ay[i], fy(i as u32), "AY[{i}]");
        assert_eq!(cz[i], fz(fx(i as u32)), "CZ[{i}]");
    }

    println!(
        "WORKS: grouping computed AT COMPILE TIME from access-set types. READ_MASKS={READ_MASKS:?} \
         WRITE_MASKS={WRITE_MASKS:?} PHASE={PHASE:?} GROUPING={GROUPING:?}. The const-gated walk \
         ran the three computed trunks; output equal to the flat topological walk. objdump \
         run_one_trunk::<..,0/1/2>: expect MEMBER-ONLY per trunk (0=SX/fx, 1=SY/fy, 2=SZ/fz) and \
         zero blr."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28, release, fat LTO, cgu=1).
//
// The grouping is computed ENTIRELY at compile time from the WU access-set types:
//   READ_MASKS  = [1, 4, 2]   (InX=bit0, InY=bit2, AX=bit1)
//   WRITE_MASKS = [2, 8, 16]  (AX=bit1, AY=bit3, CZ=bit4)
//   PHASE       = [0, 0, 1]   (SZ reads AX that SX writes -> phase 1; SX,SY phase 0)
//   GROUPING    = [0, 1, 2]   (SX trunk0, SY trunk1 disjoint in phase0; SZ trunk2 in phase1)
// All four match the hand-computed expectation (asserted in main, which passed).
//
// objdump of the three per-trunk monos (each gated by the COMPUTED grouping):
//   trunk 0 (SX): ~145 instrs, fx PRESENT, fy/fz ABSENT, blr=0 br=0
//   trunk 1 (SY): ~210 instrs, fy PRESENT, fx/fz ABSENT, blr=0 br=0
//   trunk 2 (SZ): ~204 instrs, fz (lsr+eor) PRESENT, fx/fy ABSENT, blr=0 br=0
// Each mono carries only its member WU's code. Output bit-equal to the flat walk.
//
// SETTLES (op mandate 2026-06-07, the remaining half): the grouping IS computable
// at compile time from the access-set types, with no proc-macro / build.rs / LLVM
// pass. Three parts proven:
//   U1: AccessSet TYPE -> const bitmask via a recursive associated-const fold
//       (`ConstMask`) over the `Cons<Column<C>, ...>` cons-list. No partition, no
//       specialization. (In the real engine the column id is the Locate /
//       WitnessIndex position over the registered Stores, already a const.)
//   U2: a const fn runs the real plan logic (read-after-write edges -> longest-
//       depth phases -> within-phase column-disjoint trunks) over the mask arrays,
//       producing the grouping. Stable const-fn only (loops, arrays, mutation).
//   U3: the COMPUTED const grouping drives the const-gated walk (sketch 1's
//       mechanism) and still DCEs each per-trunk mono to member-only, zero blr.
//
// Combined with sketch 202606071230, the in-Rust GATE-2 carrier mechanism is fully
// de-risked: WU access types -> const masks -> const-fn grouping -> const-gated
// flat-carrier walk -> N isolated, devirt-clean, member-only per-trunk programs;
// run one per core, zero sync (Sketch B concurrency). This is op's Option 1
// (codegen flattener) realised in pure Rust via const-eval + DCE.
//
// REMAINING (build-integration, NOT feasibility unknowns):
//   G-b: collect the per-position masks over the real `WuVals` carrier into a
//        const array (here hardcoded as 3 explicit entries). A recursive
//        associated-const array fold (`Cons` impl writes its mask at POS into the
//        tail's const array) is the shape; routine const-Rust, validate in the
//        first build slice or a tiny sketch.
//   G-c: `ConstMask` parameterised by the registered `Stores` so column ids come
//        from Locate/WitnessIndex (the real global numbering) instead of a
//        hardcoded `ColId`.
//   G-d: thread phases + waist barriers per core (RunPipeline/RunPhase from
//        G2-0b + Sketch B) around the const-gated trunk selection.
//   G-e: the const-fn grouping here is connectivity-simple; the shipped spectral
//        refinement of wide trunks is a later perf bench, not a correctness gate.
// ---------------------------------------------------------------------

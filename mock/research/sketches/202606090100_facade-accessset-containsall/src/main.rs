//! Sketch §7-4a (roadmap r2): the one genuine remaining plugin-phase feasibility
//! unknown. Does a FACADE WorkUnit register on the real builder, pass build()'s
//! `Stores: ContainsAll<Wus::AccumRead> + ContainsAll<Wus::AccumWrite>` bound,
//! AND group correctly under the const grouping (the BundleMasks fold behind
//! `phase_of` / `trunk_of`), producing a correct conservative plan, WITHOUT
//! polluting the host's type-level access analysis or requiring every
//! plugin-accessible store pre-registered?
//!
//! Background. A runtime-loaded cdylib extension's transform never enters the
//! engine's monomorphised dispatch (R6: no runtime WU registration). It integrates
//! via a statically-registered FACADE WU that calls the extension through the
//! capability ABI (hilavitkutin-extensions). The plugin's real data access is
//! unknown at host compile time. The roadmap flags the feared wall: "if the facade
//! must carry an over-approximating concrete AccessSet, ContainsAll may reject it."
//!
//! Hypothesis: the wall does NOT materialise, because the SOUND facade declares
//! only its BRIDGE stores (the host I/O it marshals across the ABI: the input it
//! reads, the output it writes back), and the plugin's "unknown access" is
//! NON-HOST data (the plugin operates on the morsel slice handed across the ABI
//! plus its own private memory), which is correctly absent from every host
//! AccessSet. So no synthetic/opaque AccessSet is needed: a facade is an ordinary
//! bridge WU as far as the host is concerned. ContainsAll only requires the named
//! stores be registered (the bridge stores are), so it never rejects; and the
//! facade contributes exactly its bridge edges to the DAG, never an
//! over-approximation. A plugin reaching host data it was not handed is the
//! global-reach-in anti-pattern (hilavitkutin-workunit-mental-model), not a facade
//! requirement. This reframes §7-4a from "find a synthetic AccessSet shape that
//! ContainsAll accepts" to "show the bridge-store facade is the sound shape and the
//! feared over-approximation is never needed."
//!
//! Two facade shapes, both run through the REAL build() AND the real engine
//! dispatch (`Scheduler::run`, so the const grouping plus plan ordering execute,
//! not a bespoke local walk):
//!   A. Bridge facade (the canonical sound shape): Read=Column<In>, Write=Column<Out>,
//!      execute() reads each record, calls an OPAQUE plugin symbol (a black_box'd
//!      fn pointer = the real cdylib indirect-call ABI boundary), writes the result.
//!      The plugin keeps PRIVATE state (an invocation counter) that is NOT a host
//!      store. Downstream Consumer reads Out + writes Accum<Sum>, giving (1) a real
//!      RAW DAG edge onto the facade and (2) a non-trivial AccumWrite portion of the
//!      ContainsAll build proof.
//!   B. Opaque facade (maximally opaque, maps the boundary): Read=Empty, Write=Empty,
//!      execute() does a pure side-effecting opaque call (plugin mutates its own
//!      private memory). Tests whether a no-host-access facade registers + builds +
//!      floats in the DAG with no edges.
//!
//! Three probes:
//!   1. build() + run(): the bundle with both facades compiles (= the ContainsAll
//!      where-clause passed) and the real dispatch produces correct data.
//!   2. const grouping introspection: `group_n` / `phase_of` / `trunk_of` on the
//!      same unit list (the blanket `UnitAccess` covers real WUs) record exactly
//!      what the BundleMasks fold computes for a facade-bearing bundle.
//!   3. anti-topo counter-probe: registering Consumer BEFORE FacadeBridge must
//!      fail build() with `NonTopologicalRegistration`, proving the facade's
//!      bridge AccessSet genuinely enters the dependency analysis (the RAW edge
//!      is seen, not silently dropped).
//!
//! If all three hold, the leeway ("find ONE sound facade AccessSet shape, or
//! record the wall") is satisfied and §7-4a is WORKS. Outcome at the bottom and
//! in FINDINGS.md.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::hint::black_box;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU32, Ordering};

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons, AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, Here, PtrNil, There,
};
use hilavitkutin::plan::grouping::{group_n, phase_of, trunk_of};
use hilavitkutin::plan::{DefaultPlanDims, PlanDims};
use hilavitkutin::scheduler::{BuildError, Scheduler};
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
use notko::Outcome;

// =====================================================================
// The "plugin": a resolved cdylib capability symbol. In production this is a
// fn-ptr in a Resource<CapabilityVtable> singleton (the hilavitkutin-extensions
// ProviderId/CapabilityId ABI); here we model the resolved symbol directly. The
// plugin keeps PRIVATE state (these atomics) that is NOT a host store and never
// touches the scheduler's data plane except through the value the facade hands it.
// black_box on the fn pointer at the call site makes it a genuine indirect call,
// keeping the transform opaque to the host's compile-time analysis.
// =====================================================================
static PLUGIN_TRANSFORM_CALLS: AtomicU32 = AtomicU32::new(0);
static PLUGIN_SIDE_EFFECTS: AtomicU32 = AtomicU32::new(0);

const PM: u32 = 2654435761;

// Capability A: a transform symbol. Input value in, output value out. Models a
// pure per-record cdylib capability. Its private call counter is plugin-owned.
fn plugin_transform(x: u32) -> u32 {
    PLUGIN_TRANSFORM_CALLS.fetch_add(1, Ordering::Relaxed);
    x.wrapping_mul(PM).wrapping_add(1)
}

// Capability B: a pure side-effect symbol. Touches only plugin-private memory.
fn plugin_side_effect() {
    PLUGIN_SIDE_EFFECTS.fetch_add(1, Ordering::Relaxed);
}

#[inline(always)]
fn expected_transform(x: u32) -> u32 {
    x.wrapping_mul(PM).wrapping_add(1)
}

// =====================================================================
// Stores. Input + Output are host columns the facade BRIDGES. Sum is the
// downstream accumulator. There is NO "plugin region" / "plugin scratch" store:
// the plugin's memory is plugin-owned, never registered. That absence is the
// load-bearing point of the sketch.
// =====================================================================
#[derive(Copy, Clone)]
struct In(u32);
#[derive(Copy, Clone)]
struct Out(u32);
#[derive(Copy, Clone)]
struct Sum(u32);

type One<T> = Cons<Column<T>, Empty>;
type AccW = Cons<Accum<Sum>, Empty>;

// =====================================================================
// Shape A: the BRIDGE facade. Declares only its bridge stores (Read=Column<In>,
// Write=Column<Out>); the opaque plugin call lives inside execute(). The host's
// access analysis sees exactly In (read) and Out (write); the plugin's internal
// work is invisible because it is not host data.
// =====================================================================
struct FacadeBridge;
impl BuilderInput for FacadeBridge {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for FacadeBridge {
    type Read = One<In>;
    type Write = One<Out>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<In>,
        One<Out>,
        PtrNil,
        ColPtrCons<In, ColPtrNil>,
        ColPtrCons<Out, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // The resolved cdylib symbol, as an opaque indirect call.
        let cap: fn(u32) -> u32 = black_box(plugin_transform);
        ctx.each().run(|i| {
            // SAFETY: In host-populated; Out reserved + exclusively written; morsel
            // covers only reserved records.
            let inp = unsafe { ctx.reader().read::<In, _>(i) };
            let out = cap(inp.0);
            unsafe { ctx.writer().write::<Out, _>(i, Out(out)) };
        });
    }
}

// Downstream consumer: reads the facade's Out column, accumulates Sum. Gives the
// facade a real RAW dependency edge AND exercises the AccumWrite portion of the
// build()'s ContainsAll proof (the load-bearing part of the bound).
struct Consumer;
impl BuilderInput for Consumer {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for Consumer {
    type Read = One<Out>;
    type Write = AccW;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<Out>,
        AccW,
        PtrNil,
        ColPtrCons<Out, ColPtrNil>,
        ColPtrNil,
        AccPtrCons<'frame, Sum, AccPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let v = unsafe { ctx.reader().read::<Out, _>(i) };
            // SAFETY: Accum<Sum> reserved for the record count; exclusive appender.
            unsafe { ctx.accums().append::<Sum, _>(Sum(v.0)) };
        });
    }
}

// =====================================================================
// Shape B: the OPAQUE facade. Read=Empty, Write=Empty. The plugin does everything
// behind the ABI on its own memory; the host models it as touching nothing. Tests
// whether a no-host-access facade registers + builds + floats in the DAG.
// =====================================================================
struct FacadeOpaque;
impl BuilderInput for FacadeOpaque {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for FacadeOpaque {
    type Read = Empty;
    type Write = Empty;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> = EngineCtx<'frame, Empty, Empty, PtrNil, ColPtrNil, ColPtrNil>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // A pure side-effecting capability call. black_box keeps it opaque.
        let cap: fn() = black_box(plugin_side_effect);
        ctx.each().run(|_i| {
            cap();
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

const N: usize = 256;

// =====================================================================
// Probe 2 fixture: the const grouping over the SAME unit types (the blanket
// `UnitAccess` covers real WorkUnits). Stores named in a fixed order so the
// witness positions are explicit: In@0, Out@1, Accum<Sum>@2.
// =====================================================================
type GStores = Cons<Column<In>, Cons<Column<Out>, Cons<Accum<Sum>, Empty>>>;
type GUnits = Cons<FacadeBridge, Cons<Consumer, Cons<FacadeOpaque, Empty>>>;
type CS = <DefaultPlanDims as PlanDims>::Stores;
type CU = <DefaultPlanDims as PlanDims>::Units;
type Adj = <DefaultPlanDims as PlanDims>::AdjRow;
type P0 = Here;
type P1 = There<Here>;
type P2 = There<There<Here>>;
// Per-unit (ReadIdx, WriteIdx) witness lists, parallel to each unit's sets.
type WFacade = (Cons<P0, Empty>, Cons<P1, Empty>); // R{In@0} W{Out@1}
type WConsumer = (Cons<P1, Empty>, Cons<P2, Empty>); // R{Out@1} W{Sum@2}
type WOpaque = (Empty, Empty); // R{} W{}
type GWit = Cons<WFacade, Cons<WConsumer, Cons<WOpaque, Empty>>>;

fn main() {
    // ---- Probe 3: anti-topo counter-probe. Consumer registered BEFORE the
    // facade that produces its input must be REJECTED, because the facade's
    // bridge AccessSet contributes a real RAW edge the analysis sees. If this
    // build succeeded, the facade's access would be invisible to the plan
    // (an over-approximation in the other direction: under-approximation).
    let provider_bad = BumpProvider::<262144>::new();
    let bad = Scheduler::builder()
        .with(Accum::<Sum>::new())
        .with(Column::<Out>::new())
        .with(Column::<In>::new())
        .with(Consumer)
        .with(FacadeBridge)
        .with(FacadeOpaque)
        .build(store(provider_bad), USize(N));
    assert!(
        matches!(bad, Outcome::Err(BuildError::NonTopologicalRegistration { .. })),
        "anti-topo registration (Consumer before FacadeBridge) must be rejected: \
         the facade's bridge AccessSet contributes the RAW edge the analysis checks"
    );

    // ---- Probe 1: the real build() + run(). Stores: Accum<Sum>, Column<Out>,
    // Column<In>. NO plugin store. Then the three WUs, INCLUDING both facade
    // shapes, in one bundle, in topo-valid order. If build() compiles, the
    // `Stores: ContainsAll<Wus::AccumRead> + ContainsAll<Wus::AccumWrite>` bound
    // PASSED with two facades present: that is the §7-4a build proof.
    let provider = BumpProvider::<262144>::new();
    let mut sched = Scheduler::builder()
        .with(Accum::<Sum>::new())
        .with(Column::<Out>::new())
        .with(Column::<In>::new())
        .with(FacadeBridge)
        .with(Consumer)
        .with(FacadeOpaque)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("engine build should succeed with facade WUs in the bundle"));

    // Host-populate In[i] = i (bindings head, last store registered).
    let in_base = sched.__bindings().__ptr().as_ptr() as *mut In;
    for i in 0..N {
        // SAFETY: In reserved for N records; storage alive; each slot written once.
        unsafe { *in_base.add(i) = In(i as u32) };
    }

    // The REAL engine dispatch: const grouping -> phases/trunks -> per-trunk
    // monos -> morsel walk. The plan (not this sketch) orders FacadeBridge
    // before Consumer via the RAW edge on Column<Out>.
    let result = sched.run();
    assert!(matches!(result, Outcome::Ok(())), "scheduler.run() should succeed");

    // Verify the bridge facade transformed In -> Out via the opaque plugin symbol.
    let out_base = sched.__bindings().__tail().__ptr().as_ptr() as *const u32;
    // SAFETY: Out reserved for N records; storage alive; written every record.
    let out = unsafe { core::slice::from_raw_parts(out_base, N) };
    for i in 0..N {
        assert_eq!(out[i], expected_transform(i as u32), "Out[{i}] (FacadeBridge via opaque plugin)");
    }

    // Verify the consumer accumulated Sum from Out (the facade's output),
    // proving the plan ran the facade FIRST (a correct conservative ordering).
    let sum_binding = sched.__bindings().__tail().__tail();
    let sum_len = sum_binding.__len_cell().get().0;
    assert_eq!(sum_len, N, "accum live length should be N (Consumer over facade Out)");
    let sum_base = sum_binding.__ptr().as_ptr() as *const u32;
    // SAFETY: Sum reserved for N records; storage alive; Consumer appended N values.
    let sums = unsafe { core::slice::from_raw_parts(sum_base, N) };
    for i in 0..N {
        assert_eq!(sums[i], expected_transform(i as u32), "Sum[{i}] (Consumer)");
    }

    // The plugin's PRIVATE state advanced: opaque work happened on non-host memory.
    let transform_calls = PLUGIN_TRANSFORM_CALLS.load(Ordering::Relaxed);
    let side_effects = PLUGIN_SIDE_EFFECTS.load(Ordering::Relaxed);
    assert_eq!(transform_calls, N as u32, "plugin transform cap called once per record");
    assert_eq!(side_effects, N as u32, "plugin side-effect cap called once per record (opaque facade)");

    // ---- Probe 2: what the const grouping (BundleMasks fold) computes for the
    // facade-bearing bundle. Recorded, then the load-bearing properties asserted.
    let n_units = group_n::<GUnits, GStores, GWit, CU, CS>().0;
    let ph = [
        phase_of::<GUnits, GStores, GWit, CU, CS, Adj>(USize(0)).0,
        phase_of::<GUnits, GStores, GWit, CU, CS, Adj>(USize(1)).0,
        phase_of::<GUnits, GStores, GWit, CU, CS, Adj>(USize(2)).0,
    ];
    let tr = [
        trunk_of::<GUnits, GStores, GWit, CU, CS, Adj>(USize(0)).0,
        trunk_of::<GUnits, GStores, GWit, CU, CS, Adj>(USize(1)).0,
        trunk_of::<GUnits, GStores, GWit, CU, CS, Adj>(USize(2)).0,
    ];
    println!(
        "grouping over [FacadeBridge, Consumer, FacadeOpaque]: n={n_units} \
         phase={ph:?} trunk={tr:?}"
    );
    assert_eq!(n_units, 3, "all three units grouped, facades included");
    // The facade must never be ordered after its consumer.
    assert!(
        ph[0] <= ph[1],
        "FacadeBridge phase must not exceed Consumer phase (RAW edge respected)"
    );
    // Within a shared phase the RAW conflict joins facade + consumer into one
    // trunk; the no-access opaque facade must never be pulled into that trunk
    // (it would mean the empty AccessSet over-approximated into a serialiser).
    if ph[0] == ph[1] {
        assert_eq!(tr[0], tr[1], "facade + consumer share a trunk via the Out conflict");
    }
    if ph[2] == ph[0] {
        assert_ne!(
            tr[2], tr[0],
            "the opaque Empty/Empty facade floats in its own trunk, no synthetic edges"
        );
    }

    println!(
        "build() succeeded with TWO facade WUs in the bundle (bridge + opaque): the \
         ContainsAll<AccumRead/AccumWrite> bound passed. Real scheduler.run(): FacadeBridge \
         transformed {N} records via an opaque plugin symbol (In -> Out), Consumer accumulated \
         Sum from Out, FacadeOpaque fired {side_effects} side-effects on plugin-private memory. \
         Anti-topo registration correctly rejected. No plugin store was registered; the facades \
         carry only their bridge AccessSets ({} transform calls).",
        transform_calls
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS, on nightly-2026-05-28 (release, fat LTO, cgu=1), re-validated
// 2026-06-11 against the post-E4 engine (MetaBlock / WVirt / MP EngineCtx shape)
// through the REAL `Scheduler::run` dispatch and the REAL const grouping, not a
// local walk. The feared wall DISSOLVED rather than materialised. Full findings
// in FINDINGS.md alongside this file; the short form:
//
//   1. build() compiled and succeeded with TWO facade WUs in the bundle (the
//      bridge facade Read=Column<In>/Write=Column<Out>, and the opaque facade
//      Read=Empty/Write=Empty), alongside a real accumulator Consumer. The
//      ContainsAll bound is a where-clause: compiling IS the proof it passed.
//   2. The const grouping (BundleMasks fold) groups the facade by exactly its
//      bridge edges: facade + consumer joined by the RAW conflict on Out, the
//      opaque facade floating with no synthetic edges. `scheduler.run()`
//      dispatched the plan and produced correct data end to end.
//   3. The anti-topo counter-probe (Consumer registered before FacadeBridge)
//      fails build() with NonTopologicalRegistration: the facade's AccessSet
//      genuinely participates in the dependency analysis.
//
// The decisive finding: the SOUND facade never needs an over-approximating or
// synthetic AccessSet. It declares only its BRIDGE stores; the plugin's
// "unknown access" is non-host data (the values handed across the ABI plus
// plugin-private memory), correctly absent from every host AccessSet.
// ContainsAll is a registration check, not a coverage check; it never rejects
// for under-/over-approximation, only for an unregistered named store, which a
// bridge facade does not have. NOT a wall; no Step-11 op-decision triggered.
// §7-4a clears. The per-morsel ABI-hop cost question is §7-4b (sibling sketch
// 202606090200).
// ---------------------------------------------------------------------

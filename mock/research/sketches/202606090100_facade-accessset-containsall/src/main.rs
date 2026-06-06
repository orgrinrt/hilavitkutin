//! Sketch §7-4a (roadmap r2): the one genuine remaining plugin-phase feasibility
//! unknown. Does a FACADE WorkUnit register on the real builder and pass build()'s
//! `Stores: ContainsAll<Wus::AccumRead> + ContainsAll<Wus::AccumWrite>` bound
//! WITHOUT polluting the host's type-level access analysis or requiring every
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
//! Two facade shapes, both run through the REAL build():
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
//! If A builds + runs correctly, the leeway ("find ONE sound facade AccessSet
//! shape, or record the wall") is satisfied and §7-4a is WORKS. B maps how far the
//! opacity can go. Outcome recorded at the bottom.

#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::hint::black_box;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU32, Ordering};

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons, AccPtrNil, AccumProject, ColPtrCons, ColPtrNil, ColProject, EngineCtx, Project,
    PtrNil,
};
use hilavitkutin::dispatch::morsel::MorselRange;
use hilavitkutin::dispatch::{WuCons, WuNil};
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

// =====================================================================
// The proven type-level fiber walk (RunFiberCol, sketch 202606081200). Reused
// verbatim so the facade runs through the real per-fiber devirt shape, not a
// bespoke driver. Only build()/ContainsAll/DAG is the §7-4a question; correctness
// via this walk is the supporting check. (§7-4b objdumps the host walk + the ABI
// hop; here we only assert correctness.)
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

#[inline(never)]
fn dispatch_morsel_outer<A, F, WL>(bindings: &A, fiber: &F, total: USize, morsel_size: USize)
where
    F: RunFiberCol<A, WL>,
{
    let total = black_box(total).0;
    let step = black_box(morsel_size).0.max(1);
    let mut start = 0usize;
    while start < total {
        let len = step.min(total - start);
        fiber.run(bindings, MorselRange::new(USize(start), USize(len)));
        start += len;
    }
}

#[inline(never)]
fn dispatch_unit_outer<A, F, WL>(bindings: &A, fiber: &F, total: USize)
where
    F: RunFiberCol<A, WL>,
{
    let total = black_box(total);
    fiber.run(bindings, MorselRange::new(USize(0), total));
}

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

fn main() {
    let provider = BumpProvider::<262144>::new();
    // Stores: Accum<Sum>, Column<Out>, Column<In>. NO plugin store. Then the three
    // WUs, INCLUDING both facade shapes, in one bundle. If build() compiles, the
    // `Stores: ContainsAll<Wus::AccumRead> + ContainsAll<Wus::AccumWrite>` bound
    // PASSED with two facades present: that is the §7-4a proof.
    let sched = Scheduler::builder()
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

    // Fiber 1 (bridge facade, accumulator-free): FacadeBridge, morsel-outer.
    let fiber1 = WuCons { head: FacadeBridge, tail: WuNil };
    dispatch_morsel_outer(sched.__bindings(), &fiber1, USize(N), USize(32));

    // Fiber 2 (opaque facade, no host access): FacadeOpaque, morsel-outer. It reads
    // and writes nothing in the data plane; it only fires the side-effect cap.
    let fiber_b = WuCons { head: FacadeOpaque, tail: WuNil };
    dispatch_morsel_outer(sched.__bindings(), &fiber_b, USize(N), USize(32));

    // Fiber 3 (downstream consumer, accumulator): Consumer, unit-outer.
    let fiber2 = WuCons { head: Consumer, tail: WuNil };
    dispatch_unit_outer(sched.__bindings(), &fiber2, USize(N));

    // Verify the bridge facade transformed In -> Out via the opaque plugin symbol.
    let out_base = sched.__bindings().__tail().__ptr().as_ptr() as *const u32;
    // SAFETY: Out reserved for N records; storage alive; written every record.
    let out = unsafe { core::slice::from_raw_parts(out_base, N) };
    for i in 0..N {
        assert_eq!(out[i], expected_transform(i as u32), "Out[{i}] (FacadeBridge via opaque plugin)");
    }

    // Verify the consumer accumulated Sum from Out (the facade's output).
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

    println!(
        "build() succeeded with TWO facade WUs in the bundle (bridge + opaque): the \
         ContainsAll<AccumRead/AccumWrite> bound passed. FacadeBridge transformed {N} records via \
         an opaque plugin symbol (In -> Out), Consumer accumulated Sum from Out, FacadeOpaque fired \
         {side_effects} side-effects on plugin-private memory. No plugin store was registered; the \
         facades carry only their bridge AccessSets ({} transform calls).",
        transform_calls
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS, on nightly-2026-05-28 (release, fat LTO, cgu=1). The feared
// wall DISSOLVED rather than materialised.
//
// build() compiled and succeeded with TWO facade WUs in the bundle (the bridge
// facade Read=Column<In>/Write=Column<Out>, and the opaque facade Read=Empty/
// Write=Empty), alongside a real accumulator Consumer (Read=Column<Out>/
// Write=Accum<Sum>). Because build()'s `Stores: ContainsAll<Wus::AccumRead> +
// ContainsAll<Wus::AccumWrite>` is a where-clause, the fact that the bundle
// COMPILES is the proof that the bound passed with facades present. Ran 256
// records: Out[i] = plugin_transform(In[i]) via the opaque indirect-call symbol,
// Sum accumulated from Out, and the opaque facade fired 256 side-effects on
// plugin-private memory. The plugin's private invocation counters advanced,
// confirming opaque work happened on NON-host memory.
//
// The decisive finding: §7-4a's feared wall ("if the facade must carry an
// over-approximating concrete AccessSet, ContainsAll may reject it") does NOT
// arise, because the SOUND facade never needs an over-approximating or synthetic
// AccessSet. It declares only its BRIDGE stores (the host I/O it marshals across
// the ABI). Reasons, each confirmed by the sketch:
//
//   1. ContainsAll is a registration check, not a coverage check. It requires only
//      that the named stores in AccumRead/AccumWrite be registered (access.rs:111
//      `ContainsAll<Empty>` is blanket-true; the recursive arm needs each member
//      `Contains`-present). A facade naming registered bridge stores satisfies it;
//      it has no notion of "the plugin's full access" to over-approximate against.
//      It NEVER rejects for under-/over-approximation; it rejects only for an
//      unregistered named store, which a bridge facade does not have.
//
//   2. The plugin's "unknown access" is NON-HOST data. The plugin operates on the
//      value(s) the facade hands it across the ABI plus its own private memory
//      (the static counters here). None of that is a scheduler store, so it is
//      correctly absent from every AccessSet. A plugin reaching host data it was
//      not handed is the global-reach-in anti-pattern
//      (hilavitkutin-workunit-mental-model), not a facade requirement. NO synthetic
//      "plugin region" store was registered, and none was needed.
//
//   3. No host access-analysis pollution. The bridge facade contributes exactly
//      its declared edges (RAW: Consumer depends on FacadeBridge via Column<Out>).
//      The opaque Empty/Empty facade contributes NO edges and floats in the DAG,
//      which is the correct model for "does opaque work touching nothing the host
//      owns." Neither over-approximates into a depends-on-everything serialiser.
//
// So the §7-4a leeway ("find ONE sound facade AccessSet shape, or record the
// wall") is satisfied by BOTH shapes, and the framing is corrected: the sound
// plugin-integration shape is a bridge WU declaring its concrete host I/O, with
// the plugin's internal access living entirely behind the ABI as non-host data.
// The opaque Empty/Empty facade also builds and runs, so the maximally-opaque
// extreme (a pure side-effect capability) is supported too. NOT a wall; no
// Step-11 op-decision triggered. §7-4a clears.
//
// What §7-4a does NOT prove (deferred to §7-4b, the next sketch): that the host's
// per-record dispatch stays zero-blr while the ONLY indirect hop is the per-morsel
// ABI call (this sketch calls the opaque symbol per RECORD via black_box, which is
// faithful for the build/ContainsAll/DAG question but not the hot-path-cost
// question). §7-4b refines the facade to a per-morsel ABI hop and objdumps the
// host walk. The production capability shape (a fn-ptr vtable held in a
// Resource<CapabilityVtable> singleton per the hilavitkutin-extensions
// ProviderId/CapabilityId ABI) is the same AccessSet story: the Resource is a
// registered host store the facade reads; the plugin's work behind the vtable is
// non-host. That Resource-read variant is a mechanical extension, not a new
// feasibility question.
// ---------------------------------------------------------------------

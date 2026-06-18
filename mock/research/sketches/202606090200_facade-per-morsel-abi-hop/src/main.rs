//! Sketch §7-4b (roadmap r2): the facade's plugin ABI hop is amortised PER MORSEL,
//! never per record, and the host's own dispatch walk stays zero-blr with a facade
//! in the pipeline.
//!
//! §7-4a proved a facade registers + passes build()'s ContainsAll bound + groups
//! correctly (no wall; the sound facade declares only its bridge stores, the
//! plugin's access is non-host data). It called the opaque plugin symbol PER
//! RECORD, which is faithful for the build/DAG question but is the hot-path cost
//! §7-4b must rule out. The whole point of the plugin facade is that the cdylib
//! boundary (an indirect call the host optimiser cannot devirtualise) is crossed
//! once per MORSEL BATCH, not once per record, so the per-record hot path stays
//! branch-and-pointer-free.
//!
//! Two things to prove, both against the engine's SHIPPED `RunFiber` walk (the
//! canonical type-level dispatch, not a local copy):
//!   1. HOST WALK ZERO BLR. A normal host fiber (Producer writes Column<Cv>,
//!      Consumer reads Cv + accumulates Sum) driven by `RunFiber` objdumps zero
//!      `blr` EVEN WITH a facade WU in the same built pipeline. The facade's
//!      presence must not pull indirection into the host's monomorphised dispatch.
//!   2. FACADE HOP IS PER-MORSEL. The facade uses the engine's own BatchApi
//!      (`ctx.batch().run(|start, len| ...)`, one call per morsel) to invoke the
//!      opaque plugin capability ONCE per morsel, handing it the morsel range. The
//!      plugin (a black_box'd fn pointer = the cdylib ABI seam) does its work on
//!      plugin-owned memory (the per-morsel-capability / sub-engine shape). Proven
//!      two ways: objdump (the single `blr` sits in the morsel loop body, with NO
//!      inner per-record loop around it) AND a runtime call count equal to the
//!      number of MORSELS, not the number of records.
//!
//! The wire shape across the seam: `fn(usize, usize)`, the morsel-relative range
//! (start, len), extern-"C"-compatible scalars. The plugin owns its absolute
//! cursor (sub-engine shape), so the morsel LENGTH is the only host-supplied
//! state; no host pointer crosses the seam in this shape. No alloc, no dyn, no
//! std machinery anywhere near the seam.
//!
//! Leeway (roadmap): prove ONE shape. This proves the per-morsel-capability shape
//! (facade -> one opaque ABI call per morsel, plugin owns its data), which is op's
//! preferred "new data behind the ABI" model and the simplest decisive proof. The
//! host-data-bridge variant (plugin reads/writes host column slices per morsel)
//! needs a morsel-absolute slice accessor that the per-WU Context does not yet
//! expose; that is a mechanical API addition, noted in the outcome, not a
//! feasibility question. Outcome at the bottom and in FINDINGS.md.

#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::hint::black_box;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons, AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, PtrNil,
};
use hilavitkutin::dispatch::morsel::MorselRange;
use hilavitkutin::dispatch::{RunFiber, WuCons, WuNil};
use hilavitkutin::meta::MetaBlock;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    AccumWriterApi, BatchApi, ColumnReaderApi, ColumnWriterApi, EachApi, HasAccumWriter, HasBatch,
    HasColumnReader, HasColumnWriter, HasEach,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Accum, Column};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;

// Host dispatch drivers over the engine's SHIPPED RunFiber walk (fiber_run.rs).
// Monomorphised per fiber type, so objdumping the drivers gives one symbol per
// fiber: the HOST fiber's symbols must be zero blr; the FACADE fiber's symbol
// must show exactly the per-morsel ABI blr. The epoch is irrelevant here (no
// virtual fires); a fixed value is threaded to satisfy the walk's signature.
#[inline(never)]
fn dispatch_morsel_outer<A, F, WL>(
    bindings: &A,
    meta: &MetaBlock,
    fiber: &F,
    total: USize,
    morsel_size: USize,
) where
    F: RunFiber<A, WL>,
{
    let total = black_box(total).0;
    let step = black_box(morsel_size).0.max(1);
    let mut start = 0usize;
    while start < total {
        let len = step.min(total - start);
        fiber.run(bindings, meta, MorselRange::new(USize(start), USize(len)), USize(1));
        start += len;
    }
}

#[inline(never)]
fn dispatch_unit_outer<A, F, WL>(bindings: &A, meta: &MetaBlock, fiber: &F, total: USize)
where
    F: RunFiber<A, WL>,
{
    let total = black_box(total);
    fiber.run(bindings, meta, MorselRange::new(USize(0), total), USize(1));
}

// =====================================================================
// The plugin capability symbol = the cdylib ABI seam. The host cannot inline
// across it (black_box on the fn pointer makes the call genuinely indirect). It is
// invoked ONCE PER MORSEL with the morsel range and does its work on PLUGIN-OWNED
// memory (PLUGIN_BUF): the per-morsel-capability / sub-engine shape. Its call
// counter (PLUGIN_BATCH_CALLS) is the runtime proof of per-morsel amortisation.
// =====================================================================
const PM: u32 = 2654435761;
static PLUGIN_BATCH_CALLS: AtomicU32 = AtomicU32::new(0);
// The plugin owns its position cursor (the sub-engine shape): the facade hands it
// the morsel LENGTH and the plugin advances its own absolute cursor by it. This is
// why BatchApi giving a morsel-RELATIVE range (start always 0) is sufficient for
// this shape: the plugin tracks its own absolute index, the host need only say
// "here is the next batch of `len` records." (A facade that instead bridges a
// host column SLICE by absolute index would need a morsel-absolute accessor the
// per-WU Context does not yet expose. See the outcome note.)
static PLUGIN_CURSOR: AtomicUsize = AtomicUsize::new(0);
const CAP: usize = 256;
static PLUGIN_BUF: [AtomicU32; CAP] = {
    // const init of an atomic array
    const Z: AtomicU32 = AtomicU32::new(0);
    [Z; CAP]
};

// One capability call covers a whole morsel of `len` records. In a real cdylib this
// is the trampoline; here it advances the plugin's own cursor and fills its own
// buffer, modelling a plugin sub-engine that processes morsels in order.
fn plugin_batch(_relative_start: usize, len: usize) {
    PLUGIN_BATCH_CALLS.fetch_add(1, Ordering::Relaxed);
    let base = PLUGIN_CURSOR.fetch_add(len, Ordering::Relaxed);
    let mut k = 0usize;
    while k < len {
        let abs = base + k;
        PLUGIN_BUF[abs].store((abs as u32).wrapping_mul(PM), Ordering::Relaxed);
        k += 1;
    }
}

// =====================================================================
// Host WUs: a normal producer -> consumer fiber. These MUST objdump zero blr.
// =====================================================================
#[derive(Copy, Clone)]
struct Inp(u32);
#[derive(Copy, Clone)]
struct Cv(u32);
#[derive(Copy, Clone)]
struct Sum(u32);

type One<T> = Cons<Column<T>, Empty>;
type AccW = Cons<Accum<Sum>, Empty>;

// Producer reads a host-seeded Inp (absolute-indexed via the morsel offset) and
// writes Cv = Inp * PM. Reading rather than deriving from the morsel-relative
// index is the proven shape (reference 202606081200 S1): the value must come from
// an absolute-indexed source, since the EachApi closure index is morsel-relative.
struct Producer;
impl BuilderInput for Producer {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for Producer {
    type Read = One<Inp>;
    type Write = One<Cv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<Inp>,
        One<Cv>,
        PtrNil,
        ColPtrCons<Inp, ColPtrNil>,
        ColPtrCons<Cv, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // Fully-qualified: EngineCtx's provider impls EachApi + BatchApi + ReduceApi,
        // all with a `run`, so a bare `.run` is ambiguous (E0034). Name the trait.
        EachApi::run(ctx.each(), |i| {
            // SAFETY: Inp host-populated; Cv reserved + exclusively written; morsel
            // covers only reserved records.
            let inp = unsafe { ctx.reader().read::<Inp, _>(i) };
            unsafe { ctx.writer().write::<Cv, _>(i, Cv(inp.0.wrapping_mul(PM))) };
        });
    }
}

struct ConsumerAccum;
impl BuilderInput for ConsumerAccum {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for ConsumerAccum {
    type Read = One<Cv>;
    type Write = AccW;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<Cv>,
        AccW,
        PtrNil,
        ColPtrCons<Cv, ColPtrNil>,
        ColPtrNil,
        AccPtrCons<'frame, Sum, AccPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        EachApi::run(ctx.each(), |i| {
            let v = unsafe { ctx.reader().read::<Cv, _>(i) };
            // SAFETY: Accum<Sum> reserved for the record count; exclusive appender.
            unsafe { ctx.accums().append::<Sum, _>(Sum(v.0)) };
        });
    }
}

// =====================================================================
// The facade. Read=Empty, Write=Empty (the plugin owns its data behind the ABI).
// execute() uses the engine's BatchApi: ONE closure call per morsel, inside which
// the opaque capability symbol is invoked ONCE with the morsel range. So per
// morsel = one indirect ABI hop, never per record.
// =====================================================================
struct FacadePerMorsel;
impl BuilderInput for FacadePerMorsel {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for FacadePerMorsel {
    type Read = Empty;
    type Write = Empty;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> = EngineCtx<'frame, Empty, Empty, PtrNil, ColPtrNil, ColPtrNil>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // The resolved cdylib capability, opaque indirect call.
        let cap: fn(usize, usize) = black_box(plugin_batch);
        // BatchApi: one call per morsel with the morsel-relative [0, len) range.
        // The plugin owns its data indexed by its own absolute cursor, so the
        // morsel-relative range is sufficient; the call FREQUENCY (per morsel)
        // is the claim.
        BatchApi::run(ctx.batch(), |start, len| {
            cap(start.0, len.0);
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
const MORSEL: usize = 32;

fn main() {
    let provider = BumpProvider::<262144>::new();
    let sched = Scheduler::builder()
        .with(Accum::<Sum>::new())
        .with(Column::<Cv>::new())
        .with(Column::<Inp>::new())
        .with(Producer)
        .with(ConsumerAccum)
        .with(FacadePerMorsel)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("engine build should succeed with a facade in the bundle"));

    // Host-populate Inp[i] = i (bindings head, last store registered).
    let inp_base = sched.__bindings().__ptr().as_ptr() as *mut Inp;
    for i in 0..N {
        // SAFETY: Inp reserved for N records; storage alive; each slot written once.
        unsafe { *inp_base.add(i) = Inp(i as u32) };
    }

    let meta = MetaBlock::default();

    // Host fiber (Producer -> ConsumerAccum). Producer is accumulator-free
    // (morsel-outer); ConsumerAccum is the accumulator (unit-outer). The HOST
    // dispatch must objdump zero blr.
    let host_producer = WuCons { head: Producer, tail: WuNil };
    dispatch_morsel_outer(sched.__bindings(), &meta, &host_producer, USize(N), USize(MORSEL));
    let host_consumer = WuCons { head: ConsumerAccum, tail: WuNil };
    dispatch_unit_outer(sched.__bindings(), &meta, &host_consumer, USize(N));

    // Facade fiber: one ABI hop per morsel.
    let facade = WuCons { head: FacadePerMorsel, tail: WuNil };
    dispatch_morsel_outer(sched.__bindings(), &meta, &facade, USize(N), USize(MORSEL));

    // Host correctness: Sum[i] = Cv[i] = i*PM. Stores registered Accum<Sum>, then
    // Column<Cv>, then Column<Inp>; the builder prepends, so bindings = [Inp, Cv,
    // Accum<Sum>]: Inp head, Cv next, Accum<Sum> tail.
    let cv_base = sched.__bindings().__tail().__ptr().as_ptr() as *const u32;
    // SAFETY: Cv reserved for N records; storage alive; written every record.
    let cv = unsafe { core::slice::from_raw_parts(cv_base, N) };
    for i in 0..N {
        assert_eq!(cv[i], (i as u32).wrapping_mul(PM), "Cv[{i}] (host Producer)");
    }
    let sum_binding = sched.__bindings().__tail().__tail();
    assert_eq!(sum_binding.__len_cell().get().0, N, "accum live length N");
    let sum_base = sum_binding.__ptr().as_ptr() as *const u32;
    // SAFETY: Sum reserved for N; ConsumerAccum appended N values.
    let sums = unsafe { core::slice::from_raw_parts(sum_base, N) };
    for i in 0..N {
        assert_eq!(sums[i], (i as u32).wrapping_mul(PM), "Sum[{i}] (host Consumer)");
    }

    // THE PER-MORSEL PROOF: the plugin capability was called once per MORSEL, not
    // once per record. ceil(N / MORSEL) morsels = 256 / 32 = 8.
    let expected_morsels = (N + MORSEL - 1) / MORSEL;
    let calls = PLUGIN_BATCH_CALLS.load(Ordering::Relaxed);
    assert_eq!(
        calls as usize, expected_morsels,
        "plugin capability called once per MORSEL ({expected_morsels}), not per record ({N})"
    );
    // Plugin's own buffer was filled across the morsels (work happened behind the ABI).
    for i in 0..N {
        assert_eq!(
            PLUGIN_BUF[i].load(Ordering::Relaxed),
            (i as u32).wrapping_mul(PM),
            "PLUGIN_BUF[{i}] (plugin-owned, filled per morsel)"
        );
    }

    println!(
        "host fiber (Producer -> ConsumerAccum) ran {N} records correct; facade fired the opaque \
         plugin capability {calls} times = once per morsel (MORSEL={MORSEL}, {expected_morsels} \
         morsels), NOT {N} times. objdump dispatch_morsel_outer host symbol for zero blr; facade \
         symbol for exactly the per-morsel ABI blr."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS, on nightly-2026-05-28 (release, fat LTO, cgu=1), re-validated
// 2026-06-11 against the post-E4 engine, this time over the engine's SHIPPED
// `RunFiber` walk (fiber_run.rs, MetaBlock + epoch threaded), not a local copy.
// Full findings with the measured objdump numbers in FINDINGS.md alongside this
// file; the short form:
//
//   1. Runtime: host fiber ran 256 records correct (Cv[i] = Sum[i] = i*PM); the
//      facade fired the opaque plugin capability exactly ceil(256/32) = 8 times,
//      once per morsel, not 256 times; the plugin filled its own buffer via its
//      private cursor.
//   2. objdump (arm64): the host producer + consumer dispatch symbols contain
//      zero indirect calls; the facade dispatch symbol contains exactly ONE
//      `blr` (the per-morsel ABI hop), inside the morsel loop with no inner
//      per-record loop around it.
//
// So §7-4b's premise holds: the host's per-record dispatch stays zero-blr while a
// facade is present, and the facade's only indirection is the per-morsel
// capability hop (the cdylib ABI seam), amortised across the morsel's records.
// The minimal wire shape proven: `fn(usize, usize)` morsel range, extern-"C"
// compatible scalars, no host pointer crossing in the sub-engine shape, no
// alloc/dyn/std at the seam.
//
// FINDING for the build phase (mechanical, not a wall): the per-WU Context's
// BatchApi (and EachApi) hand the body a morsel-RELATIVE range; the engine adds
// `morsel.start` internally inside reader()/writer(). A facade that hands an
// EXTERNAL plugin a host column SLICE by ABSOLUTE index therefore needs a
// morsel-absolute accessor the Context does not yet expose (e.g.
// `ctx.morsel_range()` returning the absolute [start, start+len), or
// `ctx.read_slice::<T>()` / `write_slice::<T>()` returning the morsel's `&[T]` /
// `&mut [T]`). The per-morsel-capability shape proven here needs only the morsel
// LENGTH (the plugin owns its absolute cursor), so it works with today's API; the
// host-column-bridge shape needs that small accessor addition. Note it for the
// plugin-facade build; it is an additive API, not a feasibility blocker. The
// production capability shape (a fn-ptr vtable in a Resource<CapabilityVtable>
// singleton per the hilavitkutin-extensions ProviderId/CapabilityId ABI) is the
// same story: the Resource is a registered host store the facade reads (§7-4a),
// and the per-morsel hop through its fn-ptr is this sketch's `blr`.
// ---------------------------------------------------------------------

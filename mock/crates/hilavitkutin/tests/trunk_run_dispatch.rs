//! Trunk-nesting dispatch test (GATE-2 G2-0a, roadmap r3 stage G2-0a).
//!
//! `RunTrunk<A, WL>` is the dispatch level directly above the shipped
//! `RunFiber`: a trunk is a value-carrying list of fibers (`FiberCons` /
//! `FiberNil`), and `RunTrunk` runs each fiber through the unchanged
//! `RunFiber`, recursing on the tail. On a single core the fibers run
//! sequentially, the same order a flat concatenation of their unit lists
//! would produce. This is the inner level of the
//! `PhaseCons<TrunkCons<FiberCons<WuCons>>>` nest proven by sketch
//! `202606070300_gate2-a-phase-trunk-fiber-nest`; the phase level + waist
//! barrier (G2-0b) and the plan-driven carrier construction (G2-0c) build on
//! it.
//!
//! The output-equivalence contract: a `RunTrunk` walk over a two-fiber trunk
//! produces column state bit-identical to the shipped flat `Scheduler::run`
//! over the same units. The fixture is a RAW column chain `SX` (`InX -> AX`)
//! then `SZ` (`AX -> CZ`); the plan orders `SX` before `SZ`. One scheduler runs
//! the flat walk; a second drives `RunTrunk` over a hand-built
//! `FiberCons{ WuCons{SX}, FiberCons{ WuCons{SZ}, FiberNil } }` trunk through an
//! `#[inline(never)]` harness, inferring the 2-deep witness cons-list with no
//! turbofish (061400 proved 2-deep infers). Both paths must compute
//! `AX[i] = fx(i)` and `CZ[i] = fz(fx(i))` and agree bit-for-bit.
//!
//! Red first: `RunTrunk` / `FiberCons` / `FiberNil` do not exist until this
//! round lands them, so the file does not compile. Once the carrier + walk
//! land, both dispatch paths agree.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::dispatch::morsel::MorselRange;
use hilavitkutin::dispatch::trunk_run::RunTrunk;
use hilavitkutin::dispatch::{FiberCons, FiberNil, WuCons, WuNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
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
        // SAFETY: `aligned + len <= N`, in bounds of the owned buffer.
        unsafe { base.add(aligned) }
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize, _align: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

const N: usize = 256;

// Two pure per-record maps so the chain has a real RAW edge.
const M1: u32 = 2654435761;
#[inline(always)]
fn fx(i: u32) -> u32 {
    i.wrapping_mul(M1)
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
struct CZ(u32);
type One<T> = Cons<Column<T>, Empty>;

// Fiber 1: InX -> AX.
#[derive(Copy, Clone)]
struct SX;
impl BuilderInput for SX {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for SX {
    type Read = One<InX>;
    type Write = One<AX>;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'f> =
        EngineCtx<'f, One<InX>, One<AX>, SnapNil, ColPtrCons<InX, ColPtrNil>, ColPtrCons<AX, ColPtrNil>>;
    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        ctx.each().run(|i| {
            // SAFETY: InX host-populated for N records; AX reserved + exclusively
            // written; morsel covers only reserved records.
            let v = unsafe { ctx.reader().read::<InX, _>(i) };
            unsafe { ctx.writer().write::<AX, _>(i, AX(fx(v.0))) };
        });
    }
}

// Fiber 2: AX -> CZ. Reads fiber 1's output (RAW edge, plan orders SX first).
#[derive(Copy, Clone)]
struct SZ;
impl BuilderInput for SZ {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for SZ {
    type Read = One<AX>;
    type Write = One<CZ>;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'f> =
        EngineCtx<'f, One<AX>, One<CZ>, SnapNil, ColPtrCons<AX, ColPtrNil>, ColPtrCons<CZ, ColPtrNil>>;
    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        ctx.each().run(|i| {
            // SAFETY: SX (ordered before SZ by the RAW edge) wrote every AX the
            // morsel covers; CZ reserved + exclusively written.
            let a = unsafe { ctx.reader().read::<AX, _>(i) };
            unsafe { ctx.writer().write::<CZ, _>(i, CZ(fz(a.0))) };
        });
    }
}

// Trunk-level harness mirroring the sketch-A `nest_dispatch`, at the trunk
// level: `A` is fixed by the bindings ref before the witness cons-list `WL` is
// inferred at the call (no turbofish).
#[inline(never)]
fn trunk_dispatch<A, T, WL>(bindings: &A, trunk: &T, morsel: MorselRange)
where
    T: RunTrunk<A, WL>,
{
    // Epoch 1: this trunk-equivalence harness uses Always WUs only, whose gate
    // const-folds true regardless of epoch (E4 slice 1).
    let meta = hilavitkutin::meta::MetaBlock::default();
    trunk.run(bindings, &meta, morsel, arvo::USize(1)); // lint:allow(no-bare-numeric) reason: test epoch literal; tracked: #121
}

#[test]
fn runtrunk_two_fiber_trunk_matches_flat_walk() {
    // Path 1: shipped flat walk over the registered [SX, SZ] carrier. Columns
    // registered CZ, AX, InX, so the builder's prepend makes InX the bindings
    // head, AX its tail, CZ the tail's tail (the sketch-A bindings-walk order).
    let mut flat = Scheduler::builder()
        .with(Column::<CZ>::new())
        .with(Column::<AX>::new())
        .with(Column::<InX>::new())
        .with(SX)
        .with(SZ)
        .build(store(BumpProvider::<32768>::new()), USize(N))
        .unwrap_or_else(|_| panic!("flat engine build should succeed"));
    let flat_inx = flat.__bindings().__ptr().as_ptr() as *mut InX;
    for i in 0..N {
        // SAFETY: InX reserved for N records; storage alive; one write each.
        unsafe { *flat_inx.add(i) = InX(i as u32) };
    }
    assert!(matches!(flat.run(), notko::Outcome::Ok(())));
    let flat_ax = flat.__bindings().__tail().__ptr().as_ptr() as *const u32;
    let flat_cz = flat.__bindings().__tail().__tail().__ptr().as_ptr() as *const u32;
    // SAFETY: AX, CZ each reserved for N records; storage alive; written every record.
    let flat_ax = unsafe { core::slice::from_raw_parts(flat_ax, N) };
    let flat_cz = unsafe { core::slice::from_raw_parts(flat_cz, N) };

    // Path 2: RunTrunk over a hand-built two-fiber trunk
    // FiberCons{ WuCons{SX}, FiberCons{ WuCons{SZ}, FiberNil } }, driven through
    // the isolated trunk harness; the 2-deep witness cons-list infers with no
    // turbofish.
    let trunk = Scheduler::builder()
        .with(Column::<CZ>::new())
        .with(Column::<AX>::new())
        .with(Column::<InX>::new())
        .with(SX)
        .with(SZ)
        .build(store(BumpProvider::<32768>::new()), USize(N))
        .unwrap_or_else(|_| panic!("trunk engine build should succeed"));
    let trunk_inx = trunk.__bindings().__ptr().as_ptr() as *mut InX;
    for i in 0..N {
        // SAFETY: InX reserved for N records; storage alive; one write each.
        unsafe { *trunk_inx.add(i) = InX(i as u32) };
    }
    let trunk_carrier = FiberCons {
        fiber: WuCons { head: SX, tail: WuNil },
        rest: FiberCons { fiber: WuCons { head: SZ, tail: WuNil }, rest: FiberNil },
    };
    trunk_dispatch(trunk.__bindings(), &trunk_carrier, MorselRange::new(USize(0), USize(N)));
    let trunk_ax = trunk.__bindings().__tail().__ptr().as_ptr() as *const u32;
    let trunk_cz = trunk.__bindings().__tail().__tail().__ptr().as_ptr() as *const u32;
    // SAFETY: AX, CZ each reserved for N records; storage alive; written every record.
    let trunk_ax = unsafe { core::slice::from_raw_parts(trunk_ax, N) };
    let trunk_cz = unsafe { core::slice::from_raw_parts(trunk_cz, N) };

    for i in 0..N {
        assert_eq!(flat_ax[i], fx(i as u32), "flat AX[{i}]");
        assert_eq!(flat_cz[i], fz(fx(i as u32)), "flat CZ[{i}]");
        assert_eq!(
            trunk_ax[i], flat_ax[i],
            "RunTrunk AX[{i}] must match the flat walk (output-equivalence)"
        );
        assert_eq!(
            trunk_cz[i], flat_cz[i],
            "RunTrunk CZ[{i}] must match the flat walk (output-equivalence)"
        );
    }
}

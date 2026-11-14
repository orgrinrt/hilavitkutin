//! Contract: per-fiber morsel WINDOW formula (spec domain 12).
//!
//! `plan.morsel_windows[f]` (read through the `FiberDispatch` descriptor) is
//! the per-fiber L1 window
//! `(L1_usable / sum of write sizes).clamp(MIN_MORSEL, MAX_MORSEL) & !3`, where
//! the write sum walks the union of the fiber's units' write masks against the
//! per-store size fold: columns at type-native stride, resource values at
//! their `Seq`/`Map` collection footprint, accumulators and virtuals zero. A
//! fiber with no write bytes takes the `MAX_MORSEL` window, and a fiber whose
//! record count exceeds its window covers the range in multiple morsels (the
//! window is NOT the record-count partition the placeholder used).
//!
//! Budget values are `DefaultRunCfg`'s consts: `L1_USABLE = 24_576`,
//! `MIN_MORSEL = 64`, `MAX_MORSEL = 8192`.
//!
//! Lives under `tests/` so the bare-numeric fixtures do not trip the src-tree
//! primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Cap, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{ColumnWriterApi, EachApi, HasColumnWriter, HasEach};
use hilavitkutin_api::footprint::{CollectionBytes, ResourceFootprint};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Column, Resource, Seq, StartsFromDefault};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

struct BumpProvider<const N: usize> {
    buf:  UnsafeCell<[MaybeUninit<u8>; N]>,
    used: Cell<usize>,
}
impl<const N: usize> BumpProvider<N> {
    fn new() -> Self {
        Self {
            buf:  UnsafeCell::new([const { MaybeUninit::uninit() }; N]),
            used: Cell::new(0),
        }
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
        // SAFETY: aligned + len <= N, in bounds of the owned buffer.
        unsafe { base.add(aligned) }
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize, _align: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: arvo::Bool, _write: arvo::Bool) {}
}

#[derive(Copy, Clone, Default)]
struct Outv(u32);

// A resource value with one Seq collection member of 2 u32 elements: 8 bytes
// of L1 write budget per R5 (Field members would add 0). Hand impl, the same
// sum the derive emits.
const SEQ_N: Cap = arvo_tensor::cap(2);

#[derive(Copy, Clone)]
struct FxRes;
impl ResourceFootprint for FxRes {
    const L1_BYTES: USize = USize(<Seq<u32, SEQ_N> as CollectionBytes>::BYTES.0);
}

type WriteBoth = Cons<Resource<FxRes>, Cons<Column<Outv>, Empty>>;

// Writes a 4-byte column and (declares) an 8-byte-footprint resource:
// sum = 12 bytes, window = (24_576 / 12).clamp(64, 8192) & !3 = 2048.
struct Producer;
impl BuilderInput for Producer {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Producer {
    type Ctx<'frame> =
        EngineCtx<'frame, Empty, WriteBoth, SnapNil, ColPtrNil, ColPtrCons<Outv, ColPtrNil>>;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = Empty;
    type Write = WriteBoth;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: Outv reserved + exclusive; windowed by the morsel.
            unsafe { ctx.writer().write::<Outv, _>(i, Outv(i.0 as u32)) };
        });
    }
}

// Declares no writes at all: nothing constrains its L1 write budget, so its
// window is the MAX_MORSEL clamp.
struct Idler;
impl BuilderInput for Idler {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Idler {
    type Ctx<'frame> = EngineCtx<'frame, Empty, Empty, SnapNil, ColPtrNil, ColPtrNil>;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = Empty;
    type Write = Empty;

    fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {}
}

#[test]
fn morsel_window_matches_l1_formula() {
    // 24_576 / (4 column + 8 resource collection) = 2048, inside the clamps,
    // already 4-aligned.
    let provider = BumpProvider::<16384>::new();
    let s = Scheduler::builder()
        .with(Resource::new(FxRes))
        .with(Column::<Outv>::start())
        .with(Producer)
        .build(store(provider), USize(16))
        .unwrap_or_else(|_| panic!("build should succeed"));
    assert_eq!(
        s.__fiber_morsel_size(USize(0)).0,
        2048,
        "window = (L1_USABLE / (column stride + resource collection footprint)).clamp & !3"
    );
}

#[test]
fn morsel_window_is_not_the_record_count_partition() {
    // 4096 records with the same 12-byte write sum: the window stays 2048,
    // so the fiber covers the range in ceil(4096 / 2048) = 2 morsels. The
    // placeholder partition would have reported 4096 here.
    let provider = BumpProvider::<65536>::new();
    let s = Scheduler::builder()
        .with(Resource::new(FxRes))
        .with(Column::<Outv>::start())
        .with(Producer)
        .build(store(provider), USize(4096))
        .unwrap_or_else(|_| panic!("build should succeed"));
    let window = s.__fiber_morsel_size(USize(0)).0;
    assert_eq!(
        window, 2048,
        "the window is budget-derived, independent of record count"
    );
    assert!(
        window < 4096,
        "a fiber larger than its window runs multiple morsels"
    );
}

#[test]
fn no_write_bytes_takes_the_max_window() {
    // A fiber with an empty write sum has nothing constraining its L1 write
    // budget: window = MAX_MORSEL (8192, already 4-aligned).
    let provider = BumpProvider::<16384>::new();
    let s = Scheduler::builder()
        .with(Idler)
        .build(store(provider), USize(16))
        .unwrap_or_else(|_| panic!("build should succeed"));
    assert_eq!(
        s.__fiber_morsel_size(USize(0)).0,
        8192,
        "zero write bytes takes the MAX_MORSEL window"
    );
}

#[test]
#[ignore = "catalogue: a MIN_MORSEL that is not a multiple of 4 lets the post-clamp `& !3` land the window below the configured floor (or at 0 for MIN < 4); the intended resolution aligns the floor before clamping so the window never undercuts MIN_MORSEL; tracked #hilavitkutin_catalogue_a_min_morsel_that_is_not_a_multiple_o"]
fn non_aligned_min_morsel_keeps_the_floor() {
    // `RunCfg` is sealed, so the budget goes to the plan step directly. One
    // fiber whose unit writes one 512-byte store: 1024 / 512 = 2, far under
    // the floor, so the window is the floor. With a floor of 66 the post-clamp
    // `& !3` gives 64, and with a floor of 3 it gives 0, a window that covers
    // no record. Either undercuts what the run config asked for.
    use hilavitkutin::DefaultPlanDims;
    use hilavitkutin::plan::{
        AccessMask,
        FiberGrouping,
        MorselBudget,
        PlanDims,
        PlanInputs,
        steps,
    };
    use hilavitkutin_api::FiberId;

    type Dims = DefaultPlanDims;
    for floor in [66usize, 3] {
        let mut inputs: PlanInputs<<Dims as PlanDims>::Units, <Dims as PlanDims>::Stores> =
            PlanInputs::new();
        inputs.unit_count = USize(1);
        inputs.record_count = USize(10_000);
        inputs.writes.as_mut()[0] = AccessMask::empty().set(USize(0));
        inputs.store_sizes.as_mut()[0] = USize(512);
        inputs.morsel_budget = MorselBudget {
            l1_usable:  USize(1024),
            min_morsel: USize(floor),
            max_morsel: USize(512),
        };
        let mut fibers: FiberGrouping<Dims> = FiberGrouping::new();
        fibers.fiber_count = USize(1);
        fibers.assignment.as_mut()[0] = FiberId::from_index(USize(0));

        let window = steps::compute_fiber_morsel_windows::<Dims>(&inputs, &fibers).as_ref()[0];
        assert!(
            window.0 >= floor,
            "MIN_MORSEL = {floor}: the window {} undercuts the configured floor",
            window.0
        );
        assert_eq!(
            window.0 % 4,
            0,
            "MIN_MORSEL = {floor}: the window {} is not 4-aligned",
            window.0
        );
    }
}

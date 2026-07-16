//! Per-fiber morsel slice A1: `FiberDispatch.morsel_size` is populated from the
//! plan's per-fiber `morsel_windows`.
//!
//! Slice A1 threads `plan.morsel_windows[f]` onto the dispatch descriptor (with
//! the plan CSR index recorded in `fiber_plan_idx`). The dispatch loop does not
//! yet consume it (slice A2 inverts the loop). This test asserts the field
//! carries the plan's per-fiber value. Since A3b, `morsel_windows[f]` is the
//! L1 window formula value `(L1_USABLE / sum of write sizes).clamp & !3`; the
//! single-fiber carrier here writes one 4-byte column, so the window is
//! `24_576 / 4 = 6144`. The formula itself is pinned by
//! `r6_morsel_window_formula`; this test pins the descriptor threading.
//!
//! Lives under `tests/` so the bare-numeric record count does not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach};
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
        // SAFETY: aligned + len <= N, in bounds of the owned buffer.
        unsafe { base.add(aligned) }
    }
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: arvo::Bool, _write: arvo::Bool) {}
}

#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Outv(u32);

type OneIn = Cons<Column<Inv>, Empty>;
type ColOut = Cons<Column<Outv>, Empty>;

struct Copyer;
impl BuilderInput for Copyer {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for Copyer {
    type Read = OneIn;
    type Write = ColOut;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> =
        EngineCtx<'frame, OneIn, ColOut, SnapNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Outv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated; Outv reserved + exclusive; windowed.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Outv, _>(i, Outv(inp.0)) };
        });
    }
}

#[test]
fn single_fiber_morsel_size_is_plan_per_fiber_value() {
    let provider = BumpProvider::<16384>::new();
    let s = Scheduler::builder()
        .with(Column::<Inv>::new())
        .with(Column::<Outv>::new())
        .with(Copyer)
        .build(store(provider), USize(4))
        .unwrap_or_else(|_| panic!("build should succeed"));
    // One fiber writing one 4-byte column: the A3b window formula gives
    // 24_576 / 4 = 6144 (inside the clamps, 4-aligned). The descriptor must
    // carry exactly plan.morsel_windows[0].
    assert_eq!(
        s.__fiber_morsel_size(USize(0)).0,
        6144,
        "FiberDispatch.morsel_size carries plan.morsel_windows[0] (the A3b L1 window)"
    );
}

// Catalogue: the whole reason `fiber_plan_idx` exists is that the dispatch-order
// index (`fd`) and the plan CSR fiber index (`f`) diverge for multi-trunk plans;
// the single-fiber fixture above collapses them to 0, so the mapping ships
// unasserted. Left ignored because the mapping is not consumed until slice A2
// (loop inversion), and a multi-trunk fixture that forces `fd != f` is cleanest
// to build alongside A2's machinery (plus a `__fiber_plan_idx` accessor). The
// ignored test with the real assertion preserves the case per edge-cases-as-tests.
#[test]
#[ignore = "catalogue: fiber_plan_idx records CSR index when dispatch order reorders fibers (fd != f); needs multi-trunk fixture + __fiber_plan_idx accessor, lands with A2; tracked #341"]
fn fiber_plan_idx_records_csr_index_under_dispatch_reorder() {
    unimplemented!(
        "contract: for a multi-trunk plan whose phase-dispatch order places a fiber \
         at dispatch index fd != its CSR index f, FiberDispatch.fiber_plan_idx == f, \
         and morsel_size == plan.morsel_windows[f] (read at the CSR index, not fd). \
         Build a 2-trunk disjoint-column carrier forcing fd != f; add __fiber_plan_idx. \
         Lands with slice A2. tracked #341"
    );
}

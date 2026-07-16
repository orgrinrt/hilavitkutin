//! Morsel-loop windowing integration test (HILA-RUNTIME-C5, #343).
//!
//! `Scheduler::run` windows the record range `[0, record_count)` into morsels
//! of `RunCfg::MORSEL_SIZE` and dispatches each unit once per morsel
//! (unit-outer). This test discriminates that the windowing happens with the
//! right boundaries and morsel-relative indexing.
//!
//! A self-transform (read + increment) would not discriminate windowing: it
//! yields the same per-record result whether the range is one morsel or many.
//! The discriminating probe writes the morsel-RELATIVE index it receives from
//! `each`. `write(i)` lands at the absolute index `morsel.start + i`, so under
//! windowing `col[k] == k % MORSEL_SIZE` (each morsel restarts the relative
//! index at zero); the pre-windowing single full-range morsel instead writes
//! `col[k] == k`. The record count (600) spans three morsels under the 256
//! default: `[0, 256)`, `[256, 512)`, `[512, 600)`.
//!
//! Red first: against the single full-range morsel, `col[256]` is 256, not 0.
//! The sentinel pre-fill also catches a skipped morsel (the sentinel survives a
//! morsel that never ran), and the `% MORSEL_SIZE` assertion catches a wrong
//! morsel boundary or a miscomputed `start`.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{ColumnWriterApi, EachApi, HasColumnWriter, HasEach};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::run_cfg::{DefaultRunCfg, RunCfg};
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;

/// Wrap a provider in the default-capacity arena store (`D = Dim<256>`).
fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

// Stack-backed test memory provider (mirrors tests/column_dispatch.rs).
struct BumpProvider<const N: usize> {
    buf: UnsafeCell<[MaybeUninit<u8>; N]>,
    used: Cell<usize>,
}

impl<const N: usize> BumpProvider<N> {
    fn new() -> Self {
        Self {
            buf: UnsafeCell::new([const { MaybeUninit::uninit() }; N]),
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
        // SAFETY: `aligned + len <= N`, in bounds of the owned buffer.
        unsafe { base.add(aligned) }
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

// Three morsels under the 256-record default `MORSEL_SIZE`: [0,256), [256,512),
// [512,600).
const RECORDS: usize = 600;

// A pre-fill sentinel distinct from every `k % MORSEL_SIZE` (which is < 256), so
// a record a skipped morsel never wrote keeps the sentinel and fails the check.
const SENTINEL: u32 = u32::MAX;

// Column value: a Copy newtype over u32, so the blanket `ColumnValue` applies.
#[derive(Copy, Clone)]
struct Tv(u32);

type Col = Cons<Column<Tv>, Empty>;

// Probe: writes the morsel-relative index it receives from `each` into the
// column. Under windowing the relative index restarts at zero per morsel, so
// the absolute record `k` holds `k % MORSEL_SIZE`.
struct RelIndexWu;

impl BuilderInput for RelIndexWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for RelIndexWu {
    type Read = Empty;
    type Write = Col;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> = EngineCtx<'frame, Empty, Col, SnapNil, ColPtrNil, ColPtrCons<Tv, ColPtrNil>>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: `build` reserved the column for the record count and the
            // plan proved this unit the exclusive writer; the morsel covers
            // only reserved records. `i` is morsel-relative; `write` lands it at
            // the absolute index `morsel.start + i`.
            unsafe { ctx.writer().write::<Tv, _>(i, Tv(i.0 as u32)) };
        });
    }
}

#[test]
fn morsel_windowing_writes_relative_index_per_morsel() {
    let msize = <DefaultRunCfg as RunCfg>::MORSEL_SIZE.0;
    // The record count must exceed the morsel size for the windowing to split.
    assert!(RECORDS > msize, "test record count must exceed the morsel size");

    let provider = BumpProvider::<32768>::new();
    let mut scheduler = Scheduler::builder()
        .with(Column::<Tv>::new())
        .with(RelIndexWu)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Pre-fill every record with the sentinel through the binding pointer, so a
    // morsel that never runs leaves its records detectably untouched.
    let base = scheduler.__bindings().__ptr().as_ptr();
    for k in 0..RECORDS {
        // SAFETY: the drain reserved `RECORDS` records; `k` is in bounds.
        unsafe { core::ptr::write(base.add(k), Tv(SENTINEL)) };
    }

    let result = scheduler.run();
    assert!(matches!(result, Outcome::Ok(())));

    let base = scheduler.__bindings().__ptr().as_ptr();
    for k in 0..RECORDS {
        // SAFETY: every record was written by the probe (or, on failure, holds
        // the sentinel); `k` is in bounds of the reserved column.
        let v = unsafe { core::ptr::read(base.add(k)) };
        assert_eq!(
            v.0,
            (k % msize) as u32,
            "record {k} holds the morsel-relative index (windowed into MORSEL_SIZE morsels)"
        );
    }
}

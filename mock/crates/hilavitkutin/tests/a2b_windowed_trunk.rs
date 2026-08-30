//! Per-fiber morsel A2b-1: `run_one_trunk_windowed` dispatches one trunk
//! fiber-outer/morsel-inner.
//!
//! A one-WU carrier is one trunk. `run_one_trunk_windowed::<_,_,0>(window)` should
//! call the trunk's monomorphised program once per morsel window over
//! `[0, record_count)`, so the WU's `execute` runs `ceil(total/window)` times, each
//! over a distinct sub-range. `ctx.each()` yields morsel-LOCAL indices (0-based
//! within the morsel), so the observable that distinguishes windowing is the
//! per-call iteration COUNT (the morsel length), not an absolute start index (which
//! would be `[0, 0, 0]`). With `total = 10` and `window = 4` the recorded lengths
//! must be `[4, 4, 2]`. That distinguishes windowed dispatch from the single
//! whole-range call `run_one_trunk` makes (which would record `[10]`).
//!
//! Lives under `tests/` so the bare-numeric fixture values do not trip the src-tree
//! primitive lints.

use core::cell::{Cell, RefCell, UnsafeCell};
use core::mem::MaybeUninit;
use std::vec::Vec;

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

thread_local! {
    // The number of records iterated in each `execute` call = that morsel's length.
    // `ctx.each()` yields morsel-LOCAL indices (0-based within the morsel), so the
    // observable that distinguishes windowing is the per-call iteration count.
    static MORSEL_LENS: RefCell<Vec<usize>> = RefCell::new(Vec::new());
}

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
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize, _align: USize) {}
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: arvo::Bool, _write: arvo::Bool) {}
}

#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Outv(u32);

type OneIn = Cons<Column<Inv>, Empty>;
type ColOut = Cons<Column<Outv>, Empty>;

struct Windowed;
impl BuilderInput for Windowed {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for Windowed {
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
        let count = Cell::new(0usize);
        ctx.each().run(|i| {
            count.set(count.get() + 1);
            // SAFETY: Inv reserved by build; Outv reserved + exclusive; windowed.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Outv, _>(i, Outv(inp.0)) };
        });
        MORSEL_LENS.with(|s| s.borrow_mut().push(count.get()));
    }
}

#[test]
fn windowed_trunk_dispatches_one_morsel_per_window() {
    MORSEL_LENS.with(|s| s.borrow_mut().clear());
    let provider = BumpProvider::<16384>::new();
    let mut s = Scheduler::builder()
        .with(Column::<Inv>::new())
        .with(Column::<Outv>::new())
        .with(Windowed)
        .build(store(provider), USize(10))
        .unwrap_or_else(|_| panic!("build should succeed"));
    // Window 4 over 10 records: morsels [0,4), [4,8), [8,10); lengths [4, 4, 2].
    s.run_one_trunk_windowed::<_, _, 0>(USize(4));
    MORSEL_LENS.with(|lens| {
        assert_eq!(
            *lens.borrow(),
            std::vec![4usize, 4, 2],
            "run_one_trunk_windowed windows the trunk fiber-outer/morsel-inner: one execute per morsel, lengths [4,4,2] over 10 records"
        );
    });
}

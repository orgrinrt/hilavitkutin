//! Per-frame accumulator reset across reused frames (Gate-1, task #665).
//!
//! The schedule-once-reuse model builds one scheduler and runs it many frames.
//! An `Accum<T>` binding holds a `Cell<USize>` live-length the append accessor
//! advances; the drain zeroes it once at build time. Without a per-frame reset
//! in `run`, the second frame starts at the live offset the first frame left,
//! so appends land past the prior data and the live-length grows frame over
//! frame (saturating at capacity). `run` must reset every accumulator
//! live-length to zero at frame start so each frame appends into a fresh buffer.
//!
//! Fail-first: before the reset lands, the two-frame run leaves the live-length
//! at `2 * K` (both frames' appends summed) and records `[0, K)` still hold
//! frame one's values. After the reset, the live-length is `K` and `[0, K)`
//! hold frame two's values. The appending WorkUnit appends unconditionally (it
//! does not assert its own start length), so the contract is observed at the
//! binding level, not through an internal panic.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{AccPtrCons, AccPtrNil, ColPtrNil, EngineCtx, PtrNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{AccumWriterApi, HasAccumWriter};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Accum;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

// Stack-backed test memory provider (mirrors tests/accum_dispatch.rs).
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

// Reserved capacity per accumulator. Wide enough that two frames of K appends
// each (2 + 2 = 4) would NOT saturate, so the bug shows as a grown live-length
// (4) rather than a saturated one; the reset is what brings it back to K.
const CAP: usize = 4;
// Appends per frame.
const K: usize = 2;

#[derive(Copy, Clone)]
struct Av(u32);

type AccW = Cons<Accum<Av>, Empty>;

// Appends K values per frame. The values are frame-independent constants so the
// post-run buffer prefix is directly assertable. It does NOT assert its own
// start length, so the per-frame-reset contract is observed at the binding
// level after the run, not via an in-execute panic.
struct AppendKWu;

impl BuilderInput for AppendKWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for AppendKWu {
    type Read = Empty;
    type Write = AccW;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> =
        EngineCtx<'frame, Empty, AccW, PtrNil, ColPtrNil, ColPtrNil, AccPtrCons<'frame, Av, AccPtrNil>>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // SAFETY: `build` reserved `CAP` (>= K) records for `Av` and the plan
        // proved this unit the exclusive appender; K appends stay within the
        // reserved capacity.
        unsafe {
            ctx.accums().append::<Av, _>(Av(700));
            ctx.accums().append::<Av, _>(Av(701));
        }
    }
}

#[test]
fn accum_resets_live_length_each_frame() {
    let provider = BumpProvider::<8192>::new();
    let mut scheduler = Scheduler::builder()
        .with(Accum::<Av>::new())
        .with(AppendKWu)
        .build(store(provider), USize(CAP))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Frame one.
    let r1 = scheduler.run();
    assert!(matches!(r1, Outcome::Ok(())));
    assert_eq!(
        scheduler.__bindings().__len_cell().get(),
        USize(K),
        "frame one appended K values",
    );

    // Frame two: with the per-frame reset, the live-length starts fresh and
    // ends at K again. Without it, the second frame appends from offset K and
    // the live-length ends at 2*K.
    let r2 = scheduler.run();
    assert!(matches!(r2, Outcome::Ok(())));

    let bindings = scheduler.__bindings();
    assert_eq!(
        bindings.__len_cell().get(),
        USize(K),
        "frame two reset the live-length and appended K fresh, not continued to 2*K",
    );

    let base = bindings.__ptr().as_ptr();
    // SAFETY: K records were appended this frame into a reset buffer; records
    // `[0, K)` are initialised.
    let v0 = unsafe { core::ptr::read(base.add(0)) };
    let v1 = unsafe { core::ptr::read(base.add(1)) };
    assert_eq!(
        [v0.0, v1.0],
        [700u32, 701u32],
        "frame two's appends landed at [0, K) in append order, into the reset buffer",
    );
}

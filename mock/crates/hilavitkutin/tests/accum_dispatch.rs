//! Accumulator append round-trip integration test (column data plane, round 2).
//!
//! A WorkUnit declares `Accum<Av>` in its Write set and appends three values
//! during `execute`. The scheduler builds with a record count covering the
//! appends, runs, and the test reads the accumulator buffer back through the
//! binding's hidden accessors to assert the three values landed at `[0, 3)` in
//! append order and the live-length advanced to 3. This pins the round-2
//! contract: an `Accum<T>` reserves a real buffer (sized by the build-time
//! record count), the append accessor writes at the live offset and advances a
//! `Cell` live-length under `&self`, and the accumulator bundle dispatches
//! through `run`.
//!
//! Red first: before the `Accum<T>` marker, the `AccumBinding` drain arm, the
//! `AccumSelector` / `AccumProject` projection, the `AccumWriterApi` accessor,
//! and the lifted `RunFiber` bound exist, an appending WorkUnit cannot
//! satisfy the dispatch bound and the file does not compile; once the bundle is
//! projected but before the drain reserves a real buffer, the append writes
//! through a dangling placeholder. Both precede the green round-trip.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{AccPtrCons, AccPtrNil, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{AccumWriterApi, HasAccumWriter};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Accum;
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

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize, _align: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

// The capacity the accumulator reserves (>= the three appends). The live
// length grows from zero as the unit appends, never reaching the capacity, so
// the unwritten tail records stay reserved-but-unread.
const CAP: usize = 4;

// Accumulator value: a Copy newtype over u32, so the blanket `ColumnValue`
// applies.
#[derive(Copy, Clone)]
struct Av(u32);

type AccW = Cons<Accum<Av>, Empty>;

// Appender: writes nothing, appends three values to `Accum<Av>`. The appends
// are fixed (not driven by the morsel) so the live length and append order are
// directly assertable.
struct AppendWu;

impl BuilderInput for AppendWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for AppendWu {
    type Read = Empty;
    type Write = AccW;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> =
        EngineCtx<'frame, Empty, AccW, SnapNil, ColPtrNil, ColPtrNil, AccPtrCons<'frame, Av, AccPtrNil>>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // The drain initialised the live length to zero before the frame.
        assert_eq!(ctx.accums().len::<Av, _>().0, 0, "live length starts at zero");
        // SAFETY: `build` reserved the `Accum<Av>` buffer for the record count
        // (>= 3) and the plan proved this unit the exclusive appender; the
        // three appends stay within the reserved capacity. Each append advances
        // the live length, read back through the `len` accessor.
        unsafe { ctx.accums().append::<Av, _>(Av(100)) };
        assert_eq!(ctx.accums().len::<Av, _>().0, 1, "len advances per append");
        unsafe { ctx.accums().append::<Av, _>(Av(101)) };
        unsafe { ctx.accums().append::<Av, _>(Av(102)) };
        assert_eq!(ctx.accums().len::<Av, _>().0, 3, "len reflects all three appends");
    }
}

// The small capacity the over-capacity test reserves. The appending unit issues
// more appends than this, so the append at index `CAP_SMALL` violates the
// reserved-capacity contract.
const CAP_SMALL: usize = 2;

// Over-capacity appender: appends five values to an `Accum<Av>` reserved to
// `CAP_SMALL`. Appending past the reserved capacity is a contract violation; the
// append path asserts the bound before writing, so the append at index
// `CAP_SMALL` panics ahead of any out-of-bounds write.
struct SaturateWu;

impl BuilderInput for SaturateWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for SaturateWu {
    type Read = Empty;
    type Write = AccW;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> =
        EngineCtx<'frame, Empty, AccW, SnapNil, ColPtrNil, ColPtrNil, AccPtrCons<'frame, Av, AccPtrNil>>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // SAFETY: `build` reserved `CAP_SMALL` records; the appender holds the
        // exclusive slot. Appending past the reserved capacity is a contract
        // violation: the append path asserts the bound before writing, so the
        // append at index `CAP_SMALL` panics ahead of any out-of-bounds write.
        unsafe {
            ctx.accums().append::<Av, _>(Av(200));
            ctx.accums().append::<Av, _>(Av(201));
            ctx.accums().append::<Av, _>(Av(202));
            ctx.accums().append::<Av, _>(Av(203));
            ctx.accums().append::<Av, _>(Av(204));
        }
    }
}

// Appending past the reserved capacity is a contract violation, not a silent
// saturating drop: the append path asserts the live length is below the reserved
// capacity and panics before the write, so a misconfigured capacity fails loudly
// instead of losing records. `SaturateWu` appends `CAP_SMALL + 3` times into a
// `CAP_SMALL` accumulator; the append at index `CAP_SMALL` panics.
#[test]
#[should_panic(expected = "reserved capacity")]
fn accum_append_panics_past_reserved_capacity() {
    let provider = BumpProvider::<8192>::new();
    let mut scheduler = Scheduler::builder()
        .with(Accum::<Av>::new())
        .with(SaturateWu)
        .build(store(provider), USize(CAP_SMALL))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let _ = scheduler.run();
}

#[test]
fn accum_appends_land_in_order_with_live_length_through_run() {
    let provider = BumpProvider::<8192>::new();
    let mut scheduler = Scheduler::builder()
        .with(Accum::<Av>::new())
        .with(AppendWu)
        .build(store(provider), USize(CAP))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let result = scheduler.run();
    assert!(matches!(result, Outcome::Ok(())));

    // The single registered store value is the accumulator, so the bindings
    // head is the `AccumBinding<Av, _>`. Read its live length and buffer prefix
    // back through the hidden accessors.
    let bindings = scheduler.__bindings();
    assert_eq!(
        bindings.__len_cell().get(),
        USize(3),
        "three appends advanced the live length to 3"
    );

    let base = bindings.__ptr().as_ptr();
    // SAFETY: the drain reserved `CAP` records for `Av` and the unit appended
    // three; records `[0, 3)` are initialised.
    let v0 = unsafe { core::ptr::read(base.add(0)) };
    let v1 = unsafe { core::ptr::read(base.add(1)) };
    let v2 = unsafe { core::ptr::read(base.add(2)) };
    assert_eq!(
        [v0.0, v1.0, v2.0],
        [100u32, 101u32, 102u32],
        "the three appended values landed at indices [0, 3) in append order"
    );
}

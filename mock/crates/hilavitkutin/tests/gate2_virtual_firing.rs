//! E4 slice 1: virtual firing + `On<V>` dispatch gating through `Scheduler::run`.
//!
//! A producer (`Always`, writes `Virtual<Tick>` + `Column<Gate>`) fires `Tick`.
//! Three consumers read `Gate` (a real read-after-write edge that orders them
//! after the producer in the dispatch, so the producer's fire precedes their
//! gate check in the same pass): two are `On<Tick>`, one is `On<Never>`. After
//! one `run`, the two `On<Tick>` consumers ran (their sentinel column is 1) and
//! the `On<Never>` consumer did not (its sentinel stayed 0).
//!
//! This brackets the gate's whole comparison `stamp == current_epoch` in one
//! deterministic pass: the `On<Tick>` path proves the true branch (stamp set to
//! the live epoch by the fire), the `On<Never>` path proves the false branch
//! (stamp never set, 0 != epoch). The cross-pass epoch decay is the same
//! comparison with `stamp` holding a prior epoch; a dedicated multi-pass test
//! needs dirty-control to defeat incremental-skip and lands with slice 1b's
//! clear-on-dispatch round. The all-`Always` regression (a fired carrier stays
//! bit-identical to the pre-E4 behaviour) is the existing engine suite, green.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    ColPtrCons, ColPtrNil, EngineCtx, SnapNil, VirtCons, VirtNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach,
    HasVirtualFirer, VirtualFirerApi,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Column, Virtual};
use hilavitkutin_api::work_unit::{Always, HasSchedule, On, WorkUnit};
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
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

const N: usize = 16;

// Two virtual markers: Tick is fired by the producer; Never is registered but
// never fired, so an `On<Never>` consumer's gate stays shut.
struct Tick;
struct Never;

#[derive(Copy, Clone)]
struct Gate(u32);
// Distinct sentinel types per consumer so the type-keyed projection resolves
// each uniquely (three same-typed columns would be ambiguous).
#[derive(Copy, Clone)]
struct RanA(u32);
#[derive(Copy, Clone)]
struct RanB(u32);
#[derive(Copy, Clone)]
struct RanN(u32);

type WTick = Cons<Virtual<Tick>, Cons<Column<Gate>, Empty>>;
type ColGate = Cons<Column<Gate>, Empty>;

// Producer: writes Gate (the ordering edge) and fires Virtual<Tick>. Read-empty
// source; its write set carries both the virtual and the column.
struct Producer;
impl BuilderInput for Producer {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for Producer {
    type Read = Empty;
    type Write = WTick;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    // Write-virtual bundle is non-nil here: `VirtCons<Tick, VirtNil>` projects
    // the `Virtual<Tick>` stamp cell so `fire::<Tick>()` can stamp it.
    type Ctx<'frame> = EngineCtx<
        'frame,
        Empty,
        WTick,
        SnapNil,
        ColPtrNil,
        ColPtrCons<Gate, ColPtrNil>,
        hilavitkutin::dispatch::engine_ctx::AccPtrNil,
        VirtCons<'frame, Tick, VirtNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: Gate reserved + exclusive; morsel covers this slice.
            unsafe { ctx.writer().write::<Gate, _>(i, Gate(1)) };
        });
        // Fire once per morsel: stamps the Tick cell with the current epoch.
        ctx.virtuals().fire::<Tick, _>();
    }
}

// A macro-free On<Tick> consumer body: reads Gate (ordering), writes Ran = 1.
// Three near-identical consumers differ only in their schedule and sentinel.
macro_rules! consumer {
    ($name:ident, $sched:ty, $ran:ident) => {
        struct $name;
        impl BuilderInput for $name {
            type Init = Self;
            type Dispatch = UnitDispatch<Self>;
        }
        impl HasSchedule for $name {
            type Sched = $sched;
        }
        impl WorkUnit<$sched> for $name {
            type Read = ColGate;
            type Write = Cons<Column<$ran>, Empty>;
            type Hint = (
                hilavitkutin_api::hint::Immediate,
                hilavitkutin_api::hint::Atomic,
                hilavitkutin_api::hint::Normal,
            );
            type Ctx<'frame> = EngineCtx<
                'frame,
                ColGate,
                Cons<Column<$ran>, Empty>,
                SnapNil,
                ColPtrCons<Gate, ColPtrNil>,
                ColPtrCons<$ran, ColPtrNil>,
            >;
            fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
                ctx.each().run(|i| {
                    // SAFETY: Gate produced this frame; Ran reserved + exclusive.
                    let _g = unsafe { ctx.reader().read::<Gate, _>(i) };
                    unsafe { ctx.writer().write::<$ran, _>(i, $ran(1)) };
                });
            }
        }
    };
}

consumer!(ConA, On<Tick>, RanA);
consumer!(ConB, On<Tick>, RanB);
consumer!(ConN, On<Never>, RanN);

#[test]
fn on_consumer_runs_iff_its_virtual_fired() {
    let provider = BumpProvider::<16384>::new();
    // Register sentinels (RanA, RanB, RanN) LAST so the bindings head-chain is
    // three `ColumnBinding<Ran>` nodes, read back head / tail / tail.tail.
    let mut scheduler = Scheduler::builder()
        .with(Virtual::<Tick>::new())
        .with(Virtual::<Never>::new())
        .with(Column::<Gate>::new())
        .with(Column::<RanN>::new()) // ConN sentinel
        .with(Column::<RanB>::new()) // ConB sentinel
        .with(Column::<RanA>::new()) // ConA sentinel (head of bindings chain)
        .with(Producer)
        .with(ConN)
        .with(ConB)
        .with(ConA)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Head-chain: RanA (head, last registered), RanB, RanN. Poison all to 0.
    let a_base = scheduler.__bindings().__ptr().as_ptr() as *mut u32;
    let b_base = scheduler.__bindings().__tail().__ptr().as_ptr() as *mut u32;
    let n_base = scheduler.__bindings().__tail().__tail().__ptr().as_ptr() as *mut u32;
    for i in 0..N {
        // SAFETY: three Ran columns reserved for N records; scheduler alive.
        unsafe {
            *a_base.add(i) = 0;
            *b_base.add(i) = 0;
            *n_base.add(i) = 0;
        }
    }

    let _ = scheduler.run::<_, _>();

    let a = scheduler.__bindings().__ptr().as_ptr() as *const u32;
    let b = scheduler.__bindings().__tail().__ptr().as_ptr() as *const u32;
    let n = scheduler.__bindings().__tail().__tail().__ptr().as_ptr() as *const u32;
    for i in 0..N {
        // SAFETY: Ran columns hold N reserved records; scheduler alive.
        let (va, vb, vn) = unsafe { (*a.add(i), *b.add(i), *n.add(i)) };
        assert_eq!(va, 1, "rec {i}: On<Tick> consumer A ran (Tick fired)");
        assert_eq!(vb, 1, "rec {i}: On<Tick> consumer B ran (same fire, multi-consumer)");
        assert_eq!(vn, 0, "rec {i}: On<Never> consumer skipped (Never never fired)");
    }
}

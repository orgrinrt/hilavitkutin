//! GATE-2 G-e: full per-trunk dispatch through `Scheduler::run` (round 2b).
//!
//! A producer (reads `In`, writes `Mid`) and a consumer (reads `Mid`, writes
//! `Out`) form a read-after-write chain, so the const grouping puts them in
//! distinct phases (the producer is the waist). The re-pointed single-core
//! `run` drives the outer per-trunk dispatcher: it must run the producer's phase
//! before the consumer's, or the consumer would read the poison left in `Mid`.
//! `Out[i] == (In[i] + 1) * 10` confirms the consumer saw the producer's output,
//! i.e. the dispatcher's phase loop ordered the trunks correctly and each WU's
//! members ran over the whole range.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, PtrNil};
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
        // SAFETY: aligned + len <= N, in bounds of the owned buffer.
        unsafe { base.add(aligned) }
    }
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

const N: usize = 64;

#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Mid(u32);
#[derive(Copy, Clone)]
struct Out(u32);

type ColIn = Cons<Column<Inv>, Empty>;
type ColMid = Cons<Column<Mid>, Empty>;
type ColOut = Cons<Column<Out>, Empty>;

// Producer: Mid = In + 1. Reads In, writes Mid. Phase 0 (the waist).
struct Producer;
impl BuilderInput for Producer {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for Producer {
    type Read = ColIn;
    type Write = ColMid;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> =
        EngineCtx<'frame, ColIn, ColMid, PtrNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Mid, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated; Mid reserved + exclusive; morsel covers
            // this slice; reader/writer map relative -> absolute.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Mid, _>(i, Mid(inp.0 + 1)) };
        });
    }
}

// Consumer: Out = Mid * 10. Reads Mid, writes Out. Phase 1 (after the waist).
struct Consumer;
impl BuilderInput for Consumer {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for Consumer {
    type Read = ColMid;
    type Write = ColOut;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> =
        EngineCtx<'frame, ColMid, ColOut, PtrNil, ColPtrCons<Mid, ColPtrNil>, ColPtrCons<Out, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: Mid produced this frame by the producer phase; Out reserved
            // + exclusive; morsel covers this slice.
            let mid = unsafe { ctx.reader().read::<Mid, _>(i) };
            unsafe { ctx.writer().write::<Out, _>(i, Out(mid.0 * 10)) };
        });
    }
}

#[test]
fn run_drives_two_phase_chain_in_order() {
    let provider = BumpProvider::<16384>::new();
    let mut scheduler = Scheduler::builder()
        .with(Column::<Inv>::new())
        .with(Column::<Mid>::new())
        .with(Column::<Out>::new())
        .with(Producer)
        .with(Consumer)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Columns: head = Out (last registered), then Mid, then In. Host-populate
    // In[i] = i, poison Mid and Out so a wrong phase order (consumer before
    // producer) or a range gap surfaces as garbage.
    // SAFETY: three columns reserved for N records of u32; scheduler alive; each
    // slot written once here before the run.
    let out_base = scheduler.__bindings().__ptr().as_ptr() as *mut u32;
    let mid_base = scheduler.__bindings().__tail().__ptr().as_ptr() as *mut u32;
    let in_base = scheduler.__bindings().__tail().__tail().__ptr().as_ptr() as *mut u32;
    for i in 0..N {
        unsafe {
            *in_base.add(i) = i as u32;
            *mid_base.add(i) = u32::MAX;
            *out_base.add(i) = u32::MAX;
        }
    }

    let _ = scheduler.run::<_, _>();

    // Out[i] = (In[i] + 1) * 10. If the consumer ran before the producer, it
    // would have read the poison in Mid and Out would not match.
    let base = scheduler.__bindings().__ptr().as_ptr() as *const u32;
    for i in 0..N {
        // SAFETY: Out holds N reserved records; scheduler alive.
        let v = unsafe { *base.add(i) };
        assert_eq!(
            v,
            (i as u32 + 1) * 10,
            "rec {i}: consumer (phase 1) saw producer (phase 0) output; dispatcher ordered phases"
        );
    }
}

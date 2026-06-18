//! GATE-2 §8 slice 1: head+tail convergence for single-trunk phases.
//!
//! One producer writes `Column<Fv>` for N records, reading nothing: a single WU
//! is a single trunk in a single phase. Under `run_parallel` on a multi-core
//! pool, head+tail convergence splits that one trunk's record range across all
//! cores (each walks a ceil-sized slice; the union covers `[0,N)` with no gap or
//! overlap). The column is poisoned before the run, so a record left unwritten
//! by a range gap, or corrupted by an overlap, fails the per-record assert.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin::OsThreadPool;
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
use notko::Outcome;

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

const N: usize = 256;

#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Fv(u32);

type ColIn = Cons<Column<Inv>, Empty>;
type ColF = Cons<Column<Fv>, Empty>;

// Mapper: reads In, writes Fv = In*7. One WU => one trunk, one phase. The value
// derives from the (windowed) input, never from the morsel-relative each() index
// (which addresses within the core's slice, not the absolute record): a WU body
// reads its inputs through the reader, which maps relative -> absolute.
struct Filler;
impl BuilderInput for Filler {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for Filler {
    type Read = ColIn;
    type Write = ColF;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> = EngineCtx<
        'frame,
        ColIn,
        ColF,
        PtrNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<Fv, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated for N records; Fv reserved + exclusive;
            // the morsel covers only this core's reserved record slice, and the
            // reader/writer map the relative index to the absolute column index.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Fv, _>(i, Fv(inp.0 * 7)) };
        });
    }
}

#[test]
fn run_parallel_single_trunk_phase_converges() {
    let provider = BumpProvider::<8192>::new();
    let scheduler = Scheduler::builder()
        .with(Column::<Inv>::new())
        .with(Column::<Fv>::new())
        .with(Filler)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Host-populate In[i] = i (absolute) and poison Fv so an unwritten record (a
    // range gap) is caught as garbage. Fv is the head column (last registered);
    // In sits one tail down.
    // SAFETY: both columns reserved for N records of u32; the scheduler (hence
    // arena) is alive; each slot is written once here before the run.
    let fv_base = scheduler.__bindings().__ptr().as_ptr() as *mut u32;
    let in_base = scheduler.__bindings().__tail().__ptr().as_ptr() as *mut u32;
    for i in 0..N {
        unsafe {
            *in_base.add(i) = i as u32;
            *fv_base.add(i) = u32::MAX;
        }
    }

    let pool = OsThreadPool::new();
    let mut scheduler = core::pin::pin!(scheduler);
    let result = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(result, Outcome::Ok(())));

    // Every record written exactly once with Fv = i*7: all cores converged on
    // the single trunk over disjoint slices covering [0,N). A gap leaves the
    // poison value; an overlap or wrong slice writes the wrong i.
    let base = scheduler.as_ref().__bindings().__ptr().as_ptr() as *const u32;
    for i in 0..N {
        // SAFETY: Fv holds N reserved records; the scheduler is alive.
        let v = unsafe { *base.add(i) };
        assert_eq!(
            v,
            (i as u32) * 7,
            "rec {i}: single-trunk phase converged across cores, each slice written once"
        );
    }
}

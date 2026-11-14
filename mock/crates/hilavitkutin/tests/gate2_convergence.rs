//! GATE-2 §8 slice 1: a single-trunk phase covers every record exactly once.
//!
//! One producer writes `Column<Fv>` for N records, reading nothing: a single WU
//! is a single trunk in a single phase. Under `run_parallel` on a multi-core
//! pool the one trunk has one owning core, which walks the whole record range;
//! the other cores own nothing in the phase. Head+tail convergence, which
//! would let a second core take the tail of that range, is designed and not
//! built, and the N-way record split that once stood in for it is deleted.
//! The column starts at u32::MAX, so a record left unwritten by a
//! range gap, or written by more than one core, fails the per-record assert.
//! The assertion is the one convergence has to keep when it lands.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use std::sync::Mutex;
use std::thread::ThreadId;

use arvo::{Bool, USize};
mod common;
mod fixed_workers;
use common::TestExecutor;
use fixed_workers::FixedWorkers;
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi,
    ColumnWriterApi,
    EachApi,
    HasColumnReader,
    HasColumnWriter,
    HasEach,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Column, StartsFromDefault};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;

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

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

const N: usize = 256;

#[derive(Copy, Clone, Default)]
struct Inv(u32);
#[derive(Copy, Clone, Default)]
struct Fv(u32);

type ColIn = Cons<Column<Inv>, Empty>;
type ColF = Cons<Column<Fv>, Empty>;

// Mapper: reads In, writes Fv = In*7. One WU => one trunk, one phase. The value
// derives from the (windowed) input, never from the morsel-relative each() index
// (which addresses within the core's slice, not the absolute record): a WU body
// reads its inputs through the reader, which maps relative -> absolute.
struct Filler(Option<&'static Mutex<Vec<ThreadId>>>);
impl BuilderInput for Filler {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Filler {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ColIn,
        ColF,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<Fv, ColPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ColIn;
    type Write = ColF;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        if let Some(seen) = self.0 {
            // One mark per call, taken once so the record loop stays plain.
            seen.lock()
                .unwrap_or_else(|e| e.into_inner())
                .push(std::thread::current().id());
        }
        ctx.each().run(|i| {
            // SAFETY: In host-populated for N records; Fv reserved + exclusive;
            // the morsel covers a window of the owning core's walk, and the
            // reader/writer map the relative index to the absolute column index.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Fv, _>(i, Fv(inp.0 * 7)) };
        });
    }
}

#[test]
fn run_parallel_single_trunk_phase_covers_every_record_once() {
    let provider = BumpProvider::<8192>::new();
    let scheduler = Scheduler::builder()
        .with(Column::<Inv>::start())
        .with(Column::<Fv>::start_from(Fv(u32::MAX)))
        .with(Filler(None))
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Host-populate In[i] = i (absolute). Fv starts at u32::MAX so an unwritten record (a
    // range gap) is caught as garbage. Fv is the head column (last registered);
    // In sits one tail down.
    // SAFETY: both columns reserved for N records of u32; the scheduler (hence
    // arena) is alive; each slot is written once here before the run.
    let in_base = scheduler.__bindings().__tail().__ptr().as_ptr();
    for i in 0 .. N {
        unsafe {
            *in_base.add(i) = Inv(i as u32);
        }
    }

    let pool = TestExecutor::new();
    let mut scheduler = core::pin::pin!(scheduler);
    let result = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(result, Outcome::Ok(())));

    // Every record written exactly once with Fv = i*7: the owning core walked
    // the single trunk over windows covering [0,N). A gap leaves the
    // start value; a wrong window writes the wrong i.
    let base = scheduler.as_ref().__bindings().__ptr().as_ptr() as *const Fv;
    for i in 0 .. N {
        // SAFETY: Fv holds N reserved records; the scheduler is alive.
        let v = unsafe { (*base.add(i)).0 };
        assert_eq!(
            v,
            (i as u32) * 7,
            "rec {i}: single-trunk phase covers every record, each written once"
        );
    }
}

/// Head+tail convergence, as the design asks for it: with two workers, the one
/// trunk of a single-trunk phase is walked from both ends, so two threads
/// execute its unit. Today one owning core walks the whole range, so one thread
/// is seen and this fails.
#[test]
#[ignore = "catalogue: head+tail convergence is designed and not built, so a single-trunk phase under run_parallel is walked by its one owning core and a second worker takes none of it; tracked #hilavitkutin_catalogue_a_single_trunk_phase_is_walked_by_one"]
fn a_single_trunk_phase_is_walked_from_head_and_tail() {
    static SEEN: Mutex<Vec<ThreadId>> = Mutex::new(Vec::new());

    let provider = BumpProvider::<8192>::new();
    let scheduler = Scheduler::builder()
        .with(Column::<Inv>::start())
        .with(Column::<Fv>::start())
        .with(Filler(Some(&SEEN)))
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let in_base = scheduler.__bindings().__tail().__ptr().as_ptr();
    for i in 0 .. N {
        // SAFETY: In reserved for N records of Inv; the scheduler is alive.
        unsafe { *in_base.add(i) = Inv(i as u32) };
    }

    let pool = FixedWorkers::<2>;
    let mut scheduler = core::pin::pin!(scheduler);
    assert!(matches!(
        scheduler.as_mut().run_parallel(&pool),
        Outcome::Ok(())
    ));

    let seen = SEEN.lock().unwrap_or_else(|e| e.into_inner());
    let mut distinct: Vec<ThreadId> = Vec::new();
    for id in seen.iter() {
        if !distinct.contains(id) {
            distinct.push(*id);
        }
    }
    assert_eq!(
        distinct.len(),
        2,
        "two workers walk the single trunk, one from the head and one from the tail"
    );
}

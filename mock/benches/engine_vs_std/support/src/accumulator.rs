//! Workload 3: a transform feeding the accumulator append surface.
//!
//! One WorkUnit reads In[i], computes the full chain, and appends the result to
//! `Accum<Av>`. The engine dispatches the accumulator fiber unit-outer (it
//! writes an accumulator, so it is not morsel-local) and the append accessor
//! advances a live-length cell per record. The std arm fills a pre-sized buffer
//! in order: the optimal append-in-order shape. The gap isolates the
//! accumulator dispatch path and the per-record append accounting from the
//! fused-loop baseline.
//!
//! `run` resets every accumulator's live-length to zero at frame start (the
//! schedule-once-reuse frame lifecycle, task #665), so each timed iteration
//! appends N real records into a fresh buffer rather than saturating on a full
//! one. The bench does no hand reset.

use core::hint::black_box;
use core::pin::Pin;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons,
    AccPtrNil,
    ColPtrCons,
    ColPtrNil,
    EngineCtx,
    SnapNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    AccumWriterApi,
    ColumnReaderApi,
    EachApi,
    HasAccumWriter,
    HasColumnReader,
    HasEach,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::{Accum, Column, StartsFromDefault};
use hilavitkutin_api::work_unit::{Always, WorkUnit};

use crate::executor::BenchExecutor;
use crate::{
    Arm,
    ArmKind,
    Crew,
    Fnv1a,
    HeapBump,
    StateArm,
    arena_bytes,
    chain,
    fnv1a_u32,
    hash_column,
    startup_arm,
    std_arm,
    std_threads,
    store,
    write_column,
};

pub const NAME: &str = "accumulator";

#[derive(Copy, Clone, Default)]
#[repr(transparent)]
struct Inv(u32);
#[derive(Copy, Clone, Default)]
#[repr(transparent)]
#[allow(dead_code)] // appended by the WU, read back post-run as raw u32 for the hash
struct Av(u32);

type One<T> = Cons<Column<T>, Empty>;
type AccW = Cons<Accum<Av>, Empty>;

// Appender: reads In, appends chain(In) to Accum<Av>. Reads a column and writes
// an accumulator, so its Ctx carries both a read column-ptr and an accum-ptr.
struct AppendChain;
impl BuilderInput for AppendChain {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for AppendChain {
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<Inv>,
        AccW,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrNil,
        AccPtrCons<'frame, Av, AccPtrNil>,
    >;
    type Hint = (Immediate, Atomic, Normal);
    type Read = One<Inv>;
    type Write = AccW;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated for the record count; read adds the
            // morsel start. The append advances the live-length under the
            // reserved capacity (= record count); the appends across the frame
            // total the record count, within the reservation.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.accums().append::<Av, _>(Av(chain(inp.0))) };
        });
    }
}

/// The scheduler of this workload over `n` records; a macro for the reason
/// `element_wise` gives. Two stores, the `Inv` column and the `Av`
/// accumulator, each reserved for `n` records.
macro_rules! build {
    ($n:expr) => {{
        let n: usize = $n;
        Scheduler::builder()
            .with(Column::<Inv>::start())
            .with(Accum::<Av>::new())
            .with(AppendChain)
            .build(store(HeapBump::new(arena_bytes(2, n))), USize(n))
            .unwrap_or_else(|_| panic!("engine_vs_std: the {NAME} scheduler build failed at n={n}"))
    }};
}

/// Put the records into `In`, one tail below the `Av` head.
macro_rules! load {
    ($bindings:expr, $records:expr, $n:expr) => {{
        let records: &[u32] = $records;
        assert_eq!(
            records.len(),
            $n,
            "engine_vs_std: the input carries a different record count"
        );
        let base = $bindings.__tail().__ptr().as_ptr() as *mut Inv;
        // SAFETY: In was reserved for n records and no frame is running.
        unsafe { write_column(base, records, Inv) };
    }};
}

/// Hash the records the frame appended to `Av`, after checking it appended
/// all of them.
///
/// `run` resets the live length at frame start (#665), so a frame appends
/// `n` records into a fresh buffer. A short count means the append saturated,
/// and is named here rather than left to read as a digest mismatch.
macro_rules! digest {
    ($bindings:expr, $n:expr) => {{
        let b = $bindings;
        let len = b.__len_cell().get().0;
        assert_eq!(
            len, $n,
            "engine_vs_std: the accumulator appended {len} of {} records",
            $n
        );
        // SAFETY: the frame appended `len` records at [0, len) of Av.
        unsafe { hash_column(b.__ptr().as_ptr(), len) }
    }};
}

/// The optimal std shape: an in-order fill of a pre-sized buffer.
fn fill(input: &[u32], out: &mut [u32]) {
    for (o, &v) in out.iter_mut().zip(input.iter()) {
        *o = chain(v);
    }
}

/// The digest every arm must produce: FNV-1a over the appended column.
pub fn reference(records: &[u32]) -> u64 {
    let mut h = Fnv1a::new();
    for &v in records {
        h.push(chain(v));
    }
    h.finish()
}

/// One arm of this workload over `n` records.
pub fn arm(kind: ArmKind, n: usize) -> Box<dyn Arm<u64>> {
    match kind {
        ArmKind::Engine => {
            Box::new(StateArm::new(
                (build!(n), n),
                |s, records| load!(s.0.__bindings(), records, s.1),
                |s| {
                    let failed = s.0.run().is_err();
                    assert!(!failed, "engine_vs_std: the {NAME} frame failed");
                },
                |s| digest!(s.0.__bindings(), s.1),
            ))
        },
        // The accumulator carrier is one unit-outer trunk; `run_parallel`
        // splits its record range across cores, each core appending into its
        // own region of the reserved buffer, merged after the frame.
        ArmKind::EnginePar => {
            Box::new(StateArm::new(
                (Box::pin(build!(n)), BenchExecutor::new(), n),
                |s, records| load!(Pin::get_ref(s.0.as_ref()).__bindings(), records, s.2),
                |s| {
                    let failed = s.0.as_mut().run_parallel(&s.1).is_err();
                    assert!(!failed, "engine_vs_std: the threaded {NAME} frame failed");
                },
                |s| digest!(Pin::get_ref(s.0.as_ref()).__bindings(), s.2),
            ))
        },
        ArmKind::EngineStartup => {
            startup_arm(
                move || build!(n),
                move |s, records| {
                    load!(s.__bindings(), records, n);
                    let failed = s.run().is_err();
                    assert!(!failed, "engine_vs_std: the {NAME} frame failed");
                    digest!(s.__bindings(), n)
                },
            )
        },
        ArmKind::Std => {
            std_arm(
                (Vec::new(), vec![0u32; n]),
                |b| &mut b.0,
                |b| {
                    fill(&b.0, &mut b.1);
                    black_box(&b.1);
                },
                |b| fnv1a_u32(&b.1),
            )
        },
        // The same fill split into disjoint chunks across `std_threads()`
        // workers, parked between frames as the engine's are. Every record
        // appends, so the append is a full in-order map and the split is
        // byte-identical to `fill`; it mirrors the engine's per-core
        // record-range split (deviation 9), so the threaded arms compare equal
        // core budgets.
        ArmKind::StdPar => {
            let mut crew = Crew::new(std_threads());
            std_arm(
                (Vec::new(), vec![0u32; n]),
                |b| &mut b.0,
                move |b| {
                    crew.for_chunks(&b.0, [&mut b.1[..]], &|i, [o]| fill(i, o));
                    black_box(&b.1);
                },
                |b| fnv1a_u32(&b.1),
            )
        },
        ArmKind::StdStartup => {
            startup_arm(
                move || (vec![0u32; n], vec![0u32; n]),
                |b, records| {
                    b.0.copy_from_slice(records);
                    fill(&b.0, &mut b.1);
                    fnv1a_u32(&b.1)
                },
            )
        },
    }
}

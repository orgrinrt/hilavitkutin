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

use arvo::USize;
use hilavitkutin::OsThreadPool;
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons, AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, PtrNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    AccumWriterApi, ColumnReaderApi, EachApi, HasAccumWriter, HasColumnReader, HasEach,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::{Accum, Column};
use hilavitkutin_api::work_unit::{Always, WorkUnit};

use crate::{HeapBump, WorkloadMeasure, arena_bytes, bench, chain, fnv1a_u32_slice, store};

pub const NAME: &str = "accumulator";

#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
#[allow(dead_code)] // appended by the WU, read back post-run as raw u32 for the hash
struct Av(u32);

type One<T> = Cons<Column<T>, Empty>;
type AccW = Cons<Accum<Av>, Empty>;

// Appender: reads In, appends chain(In) to Accum<Av>. Reads a column and writes
// an accumulator, so its Ctx carries both a read column-ptr and an accum-ptr.
struct AppendChain;
impl BuilderInput for AppendChain {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for AppendChain {
    type Read = One<Inv>;
    type Write = AccW;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<Inv>,
        AccW,
        PtrNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrNil,
        AccPtrCons<'frame, Av, AccPtrNil>,
    >;
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

pub fn measure(n: usize, warmup: usize, iters: usize) -> WorkloadMeasure {
    // Two stores (Inv column + Av accumulator), each reserved for n records.
    let eng_startup = bench(warmup, iters, || {
        let provider = HeapBump::new(arena_bytes(2, n));
        let scheduler = Scheduler::builder()
            .with(Column::<Inv>::new())
            .with(Accum::<Av>::new())
            .with(AppendChain)
            .build(store(provider), USize(n))
            .unwrap_or_else(|_| panic!("engine build should succeed"));
        black_box(&scheduler);
    });
    let std_startup = bench(warmup, iters, || {
        let in_buf: Vec<u32> = vec![0u32; n];
        let out: Vec<u32> = vec![0u32; n];
        black_box(&in_buf);
        black_box(&out);
    });

    // ----- runtime engine -----
    let provider = HeapBump::new(arena_bytes(2, n));
    let mut sched = Scheduler::builder()
        .with(Column::<Inv>::new())
        .with(Accum::<Av>::new())
        .with(AppendChain)
        .build(store(provider), USize(n))
        .unwrap_or_else(|_| panic!("engine build should succeed"));
    // Av accumulator is the head (last-registered); Inv column is one tail down.
    // SAFETY: In's buffer was reserved for N records of Inv (repr u32); the
    // scheduler is alive; each reserved slot is written once.
    let in_base = sched.__bindings().__tail().__ptr().as_ptr() as *mut Inv;
    for i in 0..n {
        unsafe { *in_base.add(i) = Inv(i as u32) };
    }
    let eng_runtime = bench(warmup, iters, || {
        // `run` resets the accumulator live-length at frame start (#665), so
        // each iteration appends n real records into a fresh buffer.
        let r = sched.run();
        black_box(&r);
    });
    // One more run so the read-back sees a full, correct frame.
    let _ = sched.run();
    let len = sched.__bindings().__len_cell().get().0;
    assert_eq!(
        len, n,
        "accumulator appended {len} of {n} records; a short count means the append saturated \
         (capacity is the record count), which would compare a partial engine result against the \
         full std result. Names the cause directly rather than via a checksum mismatch."
    );
    let av_base = sched.__bindings().__ptr().as_ptr();
    let eng_hash = {
        // SAFETY: the appender wrote `len` records at [0, len) into the Av
        // buffer this frame; the scheduler (hence storage) is alive.
        let slice = unsafe { core::slice::from_raw_parts(av_base as *const u32, len) };
        fnv1a_u32_slice(slice)
    };

    // ----- runtime engine, multi-threaded (deviation 9 threaded accumulator) -----
    // A fresh scheduler so the pinned threaded run does not alias the single-core
    // one. The accumulator carrier is one unit-outer trunk; run_parallel splits
    // its record range head+tail across cores, each appending into its own region
    // of the reserved buffer, merged after the frame (byte-identical to run()).
    let provider_par = HeapBump::new(arena_bytes(2, n));
    let sched_par = Scheduler::builder()
        .with(Column::<Inv>::new())
        .with(Accum::<Av>::new())
        .with(AppendChain)
        .build(store(provider_par), USize(n))
        .unwrap_or_else(|_| panic!("engine build should succeed"));
    // SAFETY: In's buffer (one tail down from the Av head) reserves N records of
    // Inv (repr u32); the scheduler is alive; each slot written once.
    let pin_base = sched_par.__bindings().__tail().__ptr().as_ptr() as *mut Inv;
    for i in 0..n {
        unsafe { *pin_base.add(i) = Inv(i as u32) };
    }
    let pool = OsThreadPool::new();
    let mut sched_par = core::pin::pin!(sched_par);
    let eng_runtime_par = bench(warmup, iters, || {
        let r = sched_par.as_mut().run_parallel(&pool);
        black_box(&r);
    });
    // One more threaded run so the read-back sees a full, correct frame.
    let _ = sched_par.as_mut().run_parallel(&pool);
    let eng_par_hash = {
        let sref = sched_par.as_ref();
        let b = sref.__bindings();
        let plen = b.__len_cell().get().0;
        let pav = b.__ptr().as_ptr() as *const u32;
        // SAFETY: the merge wrote `plen` records at [0, plen); scheduler alive.
        let slice = unsafe { core::slice::from_raw_parts(pav, plen) };
        fnv1a_u32_slice(slice)
    };

    // ----- runtime std: optimal append-in-order fill of a pre-sized buffer -----
    let in_buf: Vec<u32> = (0..n as u32).collect();
    let mut out: Vec<u32> = vec![0u32; n];
    let std_runtime = bench(warmup, iters, || {
        for (o, &inv) in out.iter_mut().zip(in_buf.iter()) {
            *o = chain(inv);
        }
        black_box(&out);
    });
    let std_hash = fnv1a_u32_slice(&out);

    // ----- runtime std: optimal multi-threaded fill (the FAIR parallel bar) -----
    // The append is a full in-order map (every record appends, asserted above), so
    // optimal std parallelism splits the record range across `std_threads()`
    // threads, each filling its disjoint output chunk. This is byte-identical to
    // the serial fill and mirrors the engine's deviation-9 per-core record-range
    // split, so the parallel accumulator arm is judged engine-N-core vs std-N-core.
    let nthreads = crate::std_threads();
    let chunk = ((n + nthreads - 1) / nthreads).max(1);
    let mut out_par: Vec<u32> = vec![0u32; n];
    let std_runtime_par = bench(warmup, iters, || {
        std::thread::scope(|sc| {
            for (t, oc) in out_par.chunks_mut(chunk).enumerate() {
                let base = t * chunk;
                let ib = &in_buf;
                sc.spawn(move || {
                    for (j, o) in oc.iter_mut().enumerate() {
                        *o = chain(ib[base + j]);
                    }
                });
            }
        });
        black_box(&out_par);
    });
    let std_par_hash = fnv1a_u32_slice(&out_par);

    WorkloadMeasure {
        name: NAME,
        n,
        eng_startup,
        std_startup,
        eng_runtime,
        std_runtime,
        eng_runtime_par: Some(eng_runtime_par),
        std_runtime_par: Some(std_runtime_par),
        checksum_ok: eng_hash == std_hash
            && eng_par_hash == std_hash
            && std_par_hash == std_hash,
        eng_hash,
        std_hash,
    }
}

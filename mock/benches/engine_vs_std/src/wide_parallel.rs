//! Workload 4: a wide, embarrassingly-parallel multi-trunk DAG (the win path).
//!
//! `K` independent heavy transforms, each reading its own input column and
//! writing its own output column, all column-disjoint and all in phase 0. Block
//! diagonalisation places each in its own trunk, so `run_parallel` spreads the
//! trunks across cores with no inter-trunk synchronisation (one phase, no
//! waist). Each transform does real per-record work (`heavy`, the four-stage
//! chain iterated `HEAVY_ROUNDS` times), so per-record compute dominates
//! dispatch overhead.
//!
//! This is the workload the engine is expected to WIN, increasingly with N and
//! cores: the engine runs the `K` trunks across the machine's cores while the
//! optimal std arm runs the same `K` chains on one thread. At small N the
//! fixed per-frame cost (spawn-once already amortised, but barrier publish and
//! morsel setup) keeps it close or slightly behind; as N grows the parallel
//! speedup approaches `min(K, cores)`. The single-core engine arm is reported
//! too, as the no-parallelism baseline (near parity: each trunk is one heavy
//! WU, little dispatch overhead relative to the work).
//!
//! Checksum equality (engine vs std, both arms, all K outputs) is validated
//! outside the timed region, so a measured ratio always compares two arms
//! proven to compute the identical result.

use core::hint::black_box;

use arvo::USize;
use hilavitkutin::OsThreadPool;
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, PtrNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};

use crate::{HeapBump, WorkloadMeasure, arena_bytes, bench, fnv1a_u32_slice, heavy, store};

pub const NAME: &str = "wide_parallel";

/// Number of independent trunks. Four disjoint heavy chains: enough to spread
/// across cores and show the parallel win without blowing the arena up at the
/// large sizes (4 in + 4 out columns).
pub const K: usize = 4;

type One<T> = Cons<Column<T>, Empty>;

// Each chain k: a distinct input column `In{k}` and output column `Out{k}`,
// with a heavy single-WU transform `H{k}` reading In{k}, writing Out{k}. The
// macro generates the four disjoint sets so each is its own type identity (and
// thus its own trunk after block-diagonalisation).
macro_rules! def_chain {
    ($inv:ident, $outv:ident, $wu:ident) => {
        #[derive(Copy, Clone)]
        struct $inv(u32);
        #[derive(Copy, Clone)]
        #[allow(dead_code)] // written by the WU, read back post-run as raw u32 for the hash
        struct $outv(u32);

        struct $wu;
        impl BuilderInput for $wu {
            type Init = Self;
            type Dispatch = UnitDispatch<Self>;
        }
        impl WorkUnit<Always> for $wu {
            type Read = One<$inv>;
            type Write = One<$outv>;
            type Hint = (Immediate, Atomic, Normal);
            type Ctx<'frame> = EngineCtx<
                'frame,
                One<$inv>,
                One<$outv>,
                PtrNil,
                ColPtrCons<$inv, ColPtrNil>,
                ColPtrCons<$outv, ColPtrNil>,
            >;
            fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
                ctx.each().run(|i| {
                    // SAFETY: In{k} host-populated for the record count; Out{k}
                    // reserved and exclusively written here; the morsel covers
                    // only reserved records.
                    let inp = unsafe { ctx.reader().read::<$inv, _>(i) };
                    unsafe { ctx.writer().write::<$outv, _>(i, $outv(heavy(inp.0))) };
                });
            }
        }
    };
}

def_chain!(In0, Out0, H0);
def_chain!(In1, Out1, H1);
def_chain!(In2, Out2, H2);
def_chain!(In3, Out3, H3);

/// Build the scheduler with the K input columns, K output columns, and K WUs.
/// Columns are registered inputs-first then outputs, so the bindings head (last
/// registered) is `Out3`; outputs sit at depths 0..K from the head and inputs
/// at depths K..2K. Returns the scheduler ready to run.
macro_rules! build_sched {
    ($provider:expr, $n:expr) => {
        Scheduler::builder()
            .with(Column::<In0>::new())
            .with(Column::<In1>::new())
            .with(Column::<In2>::new())
            .with(Column::<In3>::new())
            .with(Column::<Out0>::new())
            .with(Column::<Out1>::new())
            .with(Column::<Out2>::new())
            .with(Column::<Out3>::new())
            .with(H0)
            .with(H1)
            .with(H2)
            .with(H3)
            .build(store($provider), USize($n))
            .unwrap_or_else(|_| panic!("engine build should succeed"))
    };
}

pub fn measure(n: usize, warmup: usize, iters: usize) -> WorkloadMeasure {
    // ----- startup -----
    let eng_startup = bench(warmup, iters, || {
        let provider = HeapBump::new(arena_bytes(2 * K, n));
        let scheduler = build_sched!(provider, n);
        black_box(&scheduler);
    });
    let std_startup = bench(warmup, iters, || {
        let bufs: Vec<Vec<u32>> = (0..2 * K).map(|_| vec![0u32; n]).collect();
        black_box(&bufs);
    });

    // ----- runtime engine, single-core -----
    let provider = HeapBump::new(arena_bytes(2 * K, n));
    let mut sched = build_sched!(provider, n);
    // Host-populate the K input columns. Head = Out3 (last registered); inputs
    // sit below the K outputs, so In{k} is at depth (2K-1-k) from the head.
    // SAFETY: each In{k} buffer was reserved for N records of u32; the
    // scheduler (hence arena) is alive; each reserved slot is written once.
    let b = sched.__bindings();
    let in0 = b.__tail().__tail().__tail().__tail().__tail().__tail().__tail().__ptr().as_ptr()
        as *mut In0;
    let in1 = b.__tail().__tail().__tail().__tail().__tail().__tail().__ptr().as_ptr() as *mut In1;
    let in2 = b.__tail().__tail().__tail().__tail().__tail().__ptr().as_ptr() as *mut In2;
    let in3 = b.__tail().__tail().__tail().__tail().__ptr().as_ptr() as *mut In3;
    for i in 0..n {
        unsafe {
            *in0.add(i) = In0(i as u32);
            *in1.add(i) = In1(i as u32 ^ 0x1111_1111);
            *in2.add(i) = In2(i as u32 ^ 0x2222_2222);
            *in3.add(i) = In3(i as u32 ^ 0x3333_3333);
        }
    }
    let eng_runtime = bench(warmup, iters, || {
        sched.mark_dirty::<Column<In0>, _>();
        sched.mark_dirty::<Column<In1>, _>();
        sched.mark_dirty::<Column<In2>, _>();
        sched.mark_dirty::<Column<In3>, _>();
        let r = sched.run();
        black_box(&r);
    });

    // Read back the K outputs (single-core run) for the checksum. Out{k} is at
    // depth (K-1-k) from the head.
    sched.mark_dirty::<Column<In0>, _>();
    sched.mark_dirty::<Column<In1>, _>();
    sched.mark_dirty::<Column<In2>, _>();
    sched.mark_dirty::<Column<In3>, _>();
    let _ = sched.run();
    let eng_hash = {
        let b = sched.__bindings();
        let out3 = b.__ptr().as_ptr() as *const u32;
        let out2 = b.__tail().__ptr().as_ptr() as *const u32;
        let out1 = b.__tail().__tail().__ptr().as_ptr() as *const u32;
        let out0 = b.__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
        // SAFETY: each Out{k} holds N reserved records; the scheduler is alive.
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for &base in &[out0, out1, out2, out3] {
            let slice = unsafe { core::slice::from_raw_parts(base, n) };
            h ^= fnv1a_u32_slice(slice);
        }
        h
    };

    // ----- runtime engine, multi-threaded (the win path) -----
    // A fresh scheduler so the pinned threaded run does not alias the single-
    // core one. Same inputs, same K trunks; run_parallel spreads them.
    let provider_par = HeapBump::new(arena_bytes(2 * K, n));
    let sched_par = build_sched!(provider_par, n);
    let bp = sched_par.__bindings();
    let pin0 = bp.__tail().__tail().__tail().__tail().__tail().__tail().__tail().__ptr().as_ptr()
        as *mut In0;
    let pin1 = bp.__tail().__tail().__tail().__tail().__tail().__tail().__ptr().as_ptr() as *mut In1;
    let pin2 = bp.__tail().__tail().__tail().__tail().__tail().__ptr().as_ptr() as *mut In2;
    let pin3 = bp.__tail().__tail().__tail().__tail().__ptr().as_ptr() as *mut In3;
    for i in 0..n {
        unsafe {
            *pin0.add(i) = In0(i as u32);
            *pin1.add(i) = In1(i as u32 ^ 0x1111_1111);
            *pin2.add(i) = In2(i as u32 ^ 0x2222_2222);
            *pin3.add(i) = In3(i as u32 ^ 0x3333_3333);
        }
    }
    let pool = OsThreadPool::new();
    let mut sched_par = core::pin::pin!(sched_par);
    // mark_dirty needs `&mut Self`; the scheduler is `!Unpin`, so reach it via
    // `get_unchecked_mut` (sound: mark_dirty only flips a flag, never moves the
    // scheduler). The `&mut` borrow ends before `run_parallel` reborrows the Pin.
    let eng_runtime_par = bench(warmup, iters, || {
        {
            let s = unsafe { sched_par.as_mut().get_unchecked_mut() };
            s.mark_dirty::<Column<In0>, _>();
            s.mark_dirty::<Column<In1>, _>();
            s.mark_dirty::<Column<In2>, _>();
            s.mark_dirty::<Column<In3>, _>();
        }
        let r = sched_par.as_mut().run_parallel(&pool);
        black_box(&r);
    });
    // Threaded checksum.
    {
        let s = unsafe { sched_par.as_mut().get_unchecked_mut() };
        s.mark_dirty::<Column<In0>, _>();
        s.mark_dirty::<Column<In1>, _>();
        s.mark_dirty::<Column<In2>, _>();
        s.mark_dirty::<Column<In3>, _>();
    }
    let _ = sched_par.as_mut().run_parallel(&pool);
    let eng_par_hash = {
        let sref = sched_par.as_ref();
        let b = sref.__bindings();
        let out3 = b.__ptr().as_ptr() as *const u32;
        let out2 = b.__tail().__ptr().as_ptr() as *const u32;
        let out1 = b.__tail().__tail().__ptr().as_ptr() as *const u32;
        let out0 = b.__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for &base in &[out0, out1, out2, out3] {
            let slice = unsafe { core::slice::from_raw_parts(base, n) };
            h ^= fnv1a_u32_slice(slice);
        }
        h
    };

    // ----- runtime std: the K chains on one thread (optimal naive) -----
    let in_bufs: [Vec<u32>; K] = [
        (0..n as u32).collect(),
        (0..n as u32).map(|i| i ^ 0x1111_1111).collect(),
        (0..n as u32).map(|i| i ^ 0x2222_2222).collect(),
        (0..n as u32).map(|i| i ^ 0x3333_3333).collect(),
    ];
    let mut out_bufs: [Vec<u32>; K] =
        [vec![0u32; n], vec![0u32; n], vec![0u32; n], vec![0u32; n]];
    let std_runtime = bench(warmup, iters, || {
        for k in 0..K {
            for (o, &iv) in out_bufs[k].iter_mut().zip(in_bufs[k].iter()) {
                *o = heavy(iv);
            }
        }
        black_box(&out_bufs);
    });
    let std_hash = {
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for k in 0..K {
            h ^= fnv1a_u32_slice(&out_bufs[k]);
        }
        h
    };

    // ----- runtime std: optimal multi-threaded (the FAIR parallel bar) -----
    // The K chains are independent, so optimal std parallelism runs each chain on
    // its own thread, matching the engine's K-trunk-across-cores spread (the
    // engine has exactly K trunks here, so K threads is the equal-width bar; more
    // threads would split a chain's records, which the engine does not do for
    // this workload). `std::thread::scope` is idiomatic optimal std; at the
    // win-path sizes (N >= 1M) the heavy per-record work dwarfs the per-frame
    // spawn, so the comparison isolates parallel dispatch quality, not spawn cost.
    let mut out_bufs_par: [Vec<u32>; K] =
        [vec![0u32; n], vec![0u32; n], vec![0u32; n], vec![0u32; n]];
    let std_runtime_par = bench(warmup, iters, || {
        std::thread::scope(|sc| {
            for (k, ob) in out_bufs_par.iter_mut().enumerate() {
                let ib = &in_bufs[k];
                sc.spawn(move || {
                    for (o, &iv) in ob.iter_mut().zip(ib.iter()) {
                        *o = heavy(iv);
                    }
                });
            }
        });
        black_box(&out_bufs_par);
    });
    let std_par_hash = {
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for k in 0..K {
            h ^= fnv1a_u32_slice(&out_bufs_par[k]);
        }
        h
    };

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

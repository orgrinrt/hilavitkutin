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
use core::pin::Pin;

use arvo::USize;
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
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::{Column, StartsFromDefault};
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
    fnv1a_u32,
    hash_column,
    heavy,
    startup_arm,
    store,
    write_column,
};

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
        #[derive(Copy, Clone, Default)]
        #[repr(transparent)]
        struct $inv(u32);
        #[derive(Copy, Clone, Default)]
        #[repr(transparent)]
        struct $outv(u32);

        struct $wu;
        impl BuilderInput for $wu {
            type Dispatch = UnitDispatch<Self>;
            type Init = Self;
        }
        impl WorkUnit<Always> for $wu {
            type Ctx<'frame> = EngineCtx<
                'frame,
                One<$inv>,
                One<$outv>,
                SnapNil,
                ColPtrCons<$inv, ColPtrNil>,
                ColPtrCons<$outv, ColPtrNil>,
            >;
            type Hint = (Immediate, Atomic, Normal);
            type Read = One<$inv>;
            type Write = One<$outv>;

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
            .with(Column::<In0>::start())
            .with(Column::<In1>::start())
            .with(Column::<In2>::start())
            .with(Column::<In3>::start())
            .with(Column::<Out0>::start())
            .with(Column::<Out1>::start())
            .with(Column::<Out2>::start())
            .with(Column::<Out3>::start())
            .with(H0)
            .with(H1)
            .with(H2)
            .with(H3)
            .build(store($provider), USize($n))
            .unwrap_or_else(|_| panic!("engine build should succeed"))
    };
}

/// What chain `k` reads: the record xored with `SALT[k]`, so the four chains
/// compute four different columns from one input.
pub const SALT: [u32; K] = [0, 0x1111_1111, 0x2222_2222, 0x3333_3333];

/// The scheduler over `n` records, ready to run.
macro_rules! build {
    ($n:expr) => {{
        let n: usize = $n;
        build_sched!(HeapBump::new(arena_bytes(2 * K, n)), n)
    }};
}

/// Put chain `k`'s input into `In{k}`. Columns are registered inputs first,
/// so the head is `Out3`, the outputs sit at depths 0 to 3 and `In{k}` at
/// depth `7 - k`.
macro_rules! load {
    ($bindings:expr, $records:expr, $n:expr) => {{
        let records: &[u32] = $records;
        assert_eq!(
            records.len(),
            $n,
            "engine_vs_std: the input carries a different record count"
        );
        let b = $bindings;
        let in3 = b.__tail().__tail().__tail().__tail();
        let in2 = in3.__tail();
        let in1 = in2.__tail();
        let in0 = in1.__tail();
        // SAFETY: each In{k} was reserved for n records and no frame is running.
        unsafe {
            write_column(in0.__ptr().as_ptr() as *mut In0, records, |v| {
                In0(v ^ SALT[0])
            });
            write_column(in1.__ptr().as_ptr() as *mut In1, records, |v| {
                In1(v ^ SALT[1])
            });
            write_column(in2.__ptr().as_ptr() as *mut In2, records, |v| {
                In2(v ^ SALT[2])
            });
            write_column(in3.__ptr().as_ptr() as *mut In3, records, |v| {
                In3(v ^ SALT[3])
            });
        }
    }};
}

/// Mark the four inputs dirty, so the incremental skip re-runs every trunk.
macro_rules! mark {
    ($s:expr) => {{
        let s = $s;
        s.mark_dirty::<Column<In0>, _>();
        s.mark_dirty::<Column<In1>, _>();
        s.mark_dirty::<Column<In2>, _>();
        s.mark_dirty::<Column<In3>, _>();
    }};
}

/// The four output hashes folded by xor, `Out0` (depth 3) to `Out3` (the
/// head).
macro_rules! digest {
    ($bindings:expr, $n:expr) => {{
        let out3 = $bindings;
        let out2 = out3.__tail();
        let out1 = out2.__tail();
        let out0 = out1.__tail();
        // SAFETY: each Out{k} holds n records, written by the frame that just ran.
        unsafe {
            fold([
                hash_column(out0.__ptr().as_ptr(), $n, |v| v.0),
                hash_column(out1.__ptr().as_ptr(), $n, |v| v.0),
                hash_column(out2.__ptr().as_ptr(), $n, |v| v.0),
                hash_column(out3.__ptr().as_ptr(), $n, |v| v.0),
            ])
        }
    }};
}

/// The four per-chain hashes as one digest: xor into the FNV offset basis.
fn fold(hashes: [u64; K]) -> u64 {
    hashes.iter().fold(Fnv1a::new().finish(), |h, x| h ^ x)
}

/// Chains through `heavy`, chain after chain: all K on one thread for the std
/// arm, one per worker for the threaded one.
fn chains(inputs: &[Vec<u32>], outputs: &mut [Vec<u32>]) {
    for (ob, ib) in outputs.iter_mut().zip(inputs.iter()) {
        for (o, &v) in ob.iter_mut().zip(ib.iter()) {
            *o = heavy(v);
        }
    }
}

/// The std arms' buffers: the K salted inputs and the K outputs.
struct Buffers {
    inputs:  [Vec<u32>; K],
    outputs: [Vec<u32>; K],
}

impl Buffers {
    fn new(n: usize) -> Self {
        Self {
            inputs:  core::array::from_fn(|_| vec![0u32; n]),
            outputs: core::array::from_fn(|_| vec![0u32; n]),
        }
    }

    fn load(&mut self, records: &[u32]) {
        for (k, ib) in self.inputs.iter_mut().enumerate() {
            ib.clear();
            ib.extend(records.iter().map(|v| v ^ SALT[k]));
        }
    }

    fn digest(&self) -> u64 {
        fold(core::array::from_fn(|k| fnv1a_u32(&self.outputs[k])))
    }
}

/// The std arm over `Buffers`, running `body` as the measured loop.
fn std_buffers(
    n: usize,
    mut body: impl FnMut(&[Vec<u32>; K], &mut [Vec<u32>; K]) + 'static,
) -> Box<dyn Arm<u64>> {
    Box::new(StateArm::new(
        Buffers::new(n),
        |b: &mut Buffers, records: &[u32]| b.load(records),
        move |b: &mut Buffers| {
            body(&b.inputs, &mut b.outputs);
            black_box(&b.outputs);
        },
        |b: &mut Buffers| b.digest(),
    ))
}

/// The digest every arm must produce.
pub fn reference(records: &[u32]) -> u64 {
    fold(core::array::from_fn(|k| {
        let mut h = Fnv1a::new();
        for &v in records {
            h.push(heavy(v ^ SALT[k]));
        }
        h.finish()
    }))
}

/// One arm of this workload over `n` records.
pub fn arm(kind: ArmKind, n: usize) -> Box<dyn Arm<u64>> {
    match kind {
        ArmKind::Engine => {
            Box::new(StateArm::new(
                (build!(n), n),
                |s, records| load!(s.0.__bindings(), records, s.1),
                |s| {
                    mark!(&mut s.0);
                    let failed = s.0.run().is_err();
                    assert!(!failed, "engine_vs_std: the {NAME} frame failed");
                },
                |s| digest!(s.0.__bindings(), s.1),
            ))
        },
        ArmKind::EnginePar => {
            Box::new(StateArm::new(
                (Box::pin(build!(n)), BenchExecutor::new(), n),
                |s, records| load!(Pin::get_ref(s.0.as_ref()).__bindings(), records, s.2),
                |s| {
                    // SAFETY: `mark_dirty` takes `&mut self` and only flips a flag;
                    // it never moves the scheduler out of its pin.
                    mark!(unsafe { s.0.as_mut().get_unchecked_mut() });
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
                    mark!(&mut *s);
                    let failed = s.run().is_err();
                    assert!(!failed, "engine_vs_std: the {NAME} frame failed");
                    digest!(s.__bindings(), n)
                },
            )
        },
        ArmKind::Std => std_buffers(n, |i, o| chains(i, o)),
        // Each chain on its own worker, matching the engine's K trunks across
        // cores, and parked between frames as the engine's are. K workers is
        // the equal-width bar: more would split a chain's records, which the
        // engine does not do for this workload.
        ArmKind::StdPar => {
            let mut crew = Crew::new(K);
            std_buffers(n, move |i, o| {
                crew.for_chunks(&i[..], [&mut o[..]], &|ic, [oc]| chains(ic, oc))
            })
        },
        ArmKind::StdStartup => {
            startup_arm(
                move || Buffers::new(n),
                |b, records| {
                    b.load(records);
                    chains(&b.inputs, &mut b.outputs);
                    b.digest()
                },
            )
        },
    }
}

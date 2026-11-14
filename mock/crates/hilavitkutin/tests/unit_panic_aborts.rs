//! A unit that panics on a `run_parallel` worker aborts the process.
//!
//! The executor's worker thread is not the engine's, so an unwind out of a
//! unit body would end that thread before the engine's worker arrives at the
//! frame's done barrier, and the thread that called `run_parallel` would park
//! in `frame_await_done` for good. The body runs behind a boundary an unwind
//! cannot cross, so the panic aborts instead, whatever the build's panic
//! strategy (the abort row
//! `execute_returns_nothing_and_a_failing_unit_aborts_because_abort_gives_better_llvm_codegen`,
//! reached through the provisionally stamped question ticket
//! `a_unit_panic_aborts_behind_a_no_unwind_boundary_in_hilavitku`).
//!
//! The frame is two phases on two workers: two column-disjoint producers in
//! the first, each its own trunk, and a combiner reading both in the second.
//! Trunks are owned round-robin by their rank within the phase, so the second
//! producer, the one that panics, is core 1's. The test re-runs this binary as
//! a child that runs the frame, and holds the child to dying by SIGABRT inside
//! a bound. Without the boundary the child hangs and the bound trips.

#![cfg(unix)]

use std::os::unix::process::ExitStatusExt;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

mod common;
mod stable_provider;
use arvo::USize;
use common::FixedWorkers;
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
use stable_provider::stable_provider;

/// Set in the child's environment; the child test runs the frame only then.
const CHILD: &str = "HILAVITKUTIN_UNIT_PANIC_CHILD";
/// Far past a four-record frame, so only a hang trips it.
const BOUND: Duration = Duration::from_secs(30);
/// SIGABRT, the signal `abort` raises.
const SIGABRT: i32 = 6;
const N: usize = 4;

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

#[derive(Copy, Clone, Default)]
struct Inv(u32);
#[derive(Copy, Clone, Default)]
struct Av(u32);
#[derive(Copy, Clone, Default)]
struct Bv(u32);
#[derive(Copy, Clone, Default)]
// The combiner writes it and nothing reads it back: the frame never returns.
struct Zv(#[allow(dead_code)] u32);

type OneIn = Cons<Column<Inv>, Empty>;
type ColA = Cons<Column<Av>, Empty>;
type ColB = Cons<Column<Bv>, Empty>;
type ColZ = Cons<Column<Zv>, Empty>;
type ReadAB = Cons<Column<Av>, Cons<Column<Bv>, Empty>>;
type HintT = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);

/// The first phase's first trunk, core 0's: copies the input into `Av`.
struct ProducerA;
impl BuilderInput for ProducerA {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for ProducerA {
    type Ctx<'frame> = EngineCtx<
        'frame,
        OneIn,
        ColA,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<Av, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = OneIn;
    type Write = ColA;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In and Av are reserved for N records; Av is exclusive here.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Av, _>(i, Av(inp.0)) };
        });
    }
}

/// The first phase's second trunk, core 1's: panics.
struct ProducerB;
impl BuilderInput for ProducerB {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for ProducerB {
    type Ctx<'frame> = EngineCtx<
        'frame,
        OneIn,
        ColB,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<Bv, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = OneIn;
    type Write = ColB;

    fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {
        panic!("a unit body panicked on a worker");
    }
}

/// The second phase, after the waist: reads both producers.
struct Combiner;
impl BuilderInput for Combiner {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Combiner {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadAB,
        ColZ,
        SnapNil,
        ColPtrCons<Av, ColPtrCons<Bv, ColPtrNil>>,
        ColPtrCons<Zv, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = ReadAB;
    type Write = ColZ;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: both producers ran in the earlier phase; Zv is exclusive here.
            let a: Av = unsafe { ctx.reader().read::<Av, _>(i) };
            let b: Bv = unsafe { ctx.reader().read::<Bv, _>(i) };
            unsafe { ctx.writer().write::<Zv, _>(i, Zv(a.0 + b.0)) };
        });
    }
}

/// In the child, the frame through the entry the parent names; in the
/// parent's own run, nothing.
#[test]
fn child_runs_the_panicking_frame() {
    let Some(entry) = std::env::var_os(CHILD) else {
        return;
    };
    let scheduler = Scheduler::builder()
        .with(Column::<Inv>::start())
        .with(Column::<Av>::start())
        .with(Column::<Bv>::start())
        .with(Column::<Zv>::start())
        .with(ProducerA)
        .with(ProducerB)
        .with(Combiner)
        .build(store(stable_provider(16384, 0)), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));
    if entry == "run_parallel" {
        let mut scheduler = core::pin::pin!(scheduler);
        let _ = scheduler.as_mut().run_parallel(&FixedWorkers::<2>);
    } else {
        let mut scheduler = scheduler;
        let _ = scheduler.run();
    }
    // Reached only if the frame returned over the panicked unit.
    std::process::exit(0);
}

/// Run the child through `entry`, and how it ended, killing it past the bound.
fn run_child(entry: &str) -> std::process::ExitStatus {
    let mut child = Command::new(std::env::current_exe().expect("the test binary's path"))
        .args(["--exact", "child_runs_the_panicking_frame", "--nocapture", "--test-threads=1"])
        .env(CHILD, entry)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("the child starts");
    let started = Instant::now();
    loop {
        if let Some(status) = child.try_wait().expect("the child can be waited on") {
            return status;
        }
        if started.elapsed() > BOUND {
            let _ = child.kill();
            let _ = child.wait();
            panic!("the child still ran after {BOUND:?}: the frame hung on the dead worker");
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

#[test]
fn a_unit_panic_on_a_worker_aborts_within_the_bound() {
    let status = run_child("run_parallel");
    assert_eq!(
        (status.code(), status.signal()),
        (None, Some(SIGABRT)),
        "the child ended by {status:?}, not by abort"
    );
}

/// The same boundary holds on the caller's thread: under `run` the body runs
/// there, and a panic aborts rather than unwinding into the caller.
#[test]
fn a_unit_panic_under_run_aborts() {
    let status = run_child("run");
    assert_eq!(
        (status.code(), status.signal()),
        (None, Some(SIGABRT)),
        "the child ended by {status:?}, not by abort"
    );
}

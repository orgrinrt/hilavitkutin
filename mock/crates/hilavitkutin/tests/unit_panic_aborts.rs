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
//! producer, the one that panics, is core 1's. Each test re-runs this binary on
//! itself alone as a child that runs the frame, and holds the child to dying by
//! SIGABRT inside a bound, having printed the unit's panic and then the
//! runtime's refusal to unwind past the boundary. Without the boundary the
//! child hangs and the bound trips.

#![cfg(unix)]

use std::io::Read;
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

/// Set in the child's environment to the entry the child runs the frame through.
const CHILD: &str = "HILAVITKUTIN_UNIT_PANIC_CHILD";
/// Far past a four-record frame, so only a hang trips it.
const BOUND: Duration = Duration::from_secs(30);
/// SIGABRT, the signal `abort` raises.
const SIGABRT: i32 = 6;
/// What the panicking unit says.
const PANIC_MESSAGE: &str = "a unit body panicked on a worker";
/// What the runtime says when a panic reaches a function that cannot unwind.
const CANNOT_UNWIND: &str = "panic in a function that cannot unwind";
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
        panic!("{PANIC_MESSAGE}");
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

/// The frame through `entry`, in the child. Returns only if the frame returned
/// over the panicked unit, and then exits with success, which the parent reads
/// as the boundary missing.
fn run_the_panicking_frame(entry: &std::ffi::OsStr) -> ! {
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
    std::process::exit(0);
}

/// How the frame through `entry` ends, run in a child that is this binary
/// running the test named `test` alone, with its standard error.
///
/// In that child the variable naming the entry is set, and this runs the frame
/// and does not return. In the parent it starts the child, kills it past the
/// bound, and returns its status and what it printed to standard error.
fn frame_in_a_child(test: &str, entry: &str) -> (std::process::ExitStatus, String) {
    if let Some(entry) = std::env::var_os(CHILD) {
        run_the_panicking_frame(&entry);
    }
    let mut child = Command::new(std::env::current_exe().expect("the test binary's path"))
        .args(["--exact", test, "--nocapture", "--test-threads=1"])
        .env(CHILD, entry)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the child starts");
    // Read while the child runs: a backtrace can be larger than the pipe holds,
    // and a child blocked on a full pipe would look like a hung frame.
    let mut pipe = child
        .stderr
        .take()
        .expect("the child's standard error is piped");
    let reader = std::thread::spawn(move || {
        let mut stderr = String::new();
        let _ = pipe.read_to_string(&mut stderr);
        stderr
    });
    let started = Instant::now();
    loop {
        if let Some(status) = child.try_wait().expect("the child can be waited on") {
            let stderr = reader.join().expect("the reader of standard error ends");
            return (status, stderr);
        }
        if started.elapsed() > BOUND {
            let _ = child.kill();
            let _ = child.wait();
            panic!("the child still ran after {BOUND:?}: the frame hung on the dead worker");
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

/// The child died by abort, and the abort was the unit's panic stopped at the
/// boundary: its message, then the runtime's refusal to unwind past it.
fn assert_aborted_at_the_boundary(status: std::process::ExitStatus, stderr: &str) {
    assert_eq!(
        (status.code(), status.signal()),
        (None, Some(SIGABRT)),
        "the child ended by {status:?}, not by abort; it printed:\n{stderr}"
    );
    let panicked = stderr.find(PANIC_MESSAGE);
    let refused = stderr.find(CANNOT_UNWIND);
    assert!(
        matches!((panicked, refused), (Some(p), Some(r)) if p < r),
        "the child printed the unit's panic and then the refusal to unwind; it printed:\n{stderr}"
    );
}

#[test]
fn a_unit_panic_on_a_worker_aborts_within_the_bound() {
    let (status, stderr) = frame_in_a_child(
        "a_unit_panic_on_a_worker_aborts_within_the_bound",
        "run_parallel",
    );
    assert_aborted_at_the_boundary(status, &stderr);
}

/// The same boundary holds on the caller's thread: under `run` the body runs
/// there, and a panic aborts rather than unwinding into the caller.
#[test]
fn a_unit_panic_under_run_aborts() {
    let (status, stderr) = frame_in_a_child("a_unit_panic_under_run_aborts", "run");
    assert_aborted_at_the_boundary(status, &stderr);
}

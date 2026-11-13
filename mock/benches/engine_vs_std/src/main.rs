//! Orchestrator for the engine-vs-std bench.
//!
//! The harness's own driver (`driver::drive`) runs everything: selection,
//! validation of every arm against the reference before timing, one worker
//! subprocess per arm and mode, the statistics, `results/`, history. This
//! binary registers what is this bench's own: the routine of each section,
//! picked by the section's `workload` key, and the program around the call.
//!
//! A run starts from a stage: `../stage.sh .` builds the member and prints a
//! directory, and `<that>/release/engine_vs_std` is run from this directory.
//! `bench.toml` names the arms under `target/`, which this binary points at
//! its own stage, so the arms timed are the ones staged beside it. The stage is
//! verified before the run and after it.
//!
//! `--verdict [<results dir>]` runs nothing. It reads the medians a run
//! promoted into `results/` and holds each cell to the design's ceiling in
//! `evs_support::expected` (see `verdict`). These were the ignored
//! `tests/perf_gate.rs` oracles; a timing assertion means something only over
//! a release build timed by the harness, so that is where they are applied.

use std::path::Path;
use std::process::ExitCode;

use bench_stage::Stage;
use evs_support::{Evs, Workload};
use mockspace_bench_core::{byte_routine_dispatch, routine_bridge};
use mockspace_bench_harness::driver::{DriverRegistry, drive};
use mockspace_bench_harness::{self as harness, BenchConfig, RoutineSpec, Workload as Program};

mod verdict;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    if args.next().as_deref() == Some("--verdict") {
        let dir = args.next().unwrap_or_else(|| "results".to_string());
        return match verdict::run(Path::new("bench.toml"), Path::new(&dir)) {
            Ok(true) => ExitCode::SUCCESS,
            Ok(false) => ExitCode::FAILURE,
            Err(e) => {
                eprintln!("error: {e}");
                ExitCode::from(2)
            },
        };
    }
    let stage =
        match Stage::of_current_exe().and_then(|s| s.link_into(Path::new("target")).map(|()| s)) {
            Ok(stage) => stage,
            Err(e) => {
                eprintln!("error: {e}");
                return ExitCode::FAILURE;
            },
        };
    let code = drive(&DriverRegistry {
        build_workload,
        routine_for,
        // Every section resolves through `routine_for`; the byte fallback is
        // never reached.
        byte_dispatch: byte_routine_dispatch!(out = 16, sizes = [16]),
    });
    if let Err(e) = stage.verify() {
        eprintln!("error: {e}\nthe timings of this run are not of the staged build");
        return ExitCode::FAILURE;
    }
    code
}

/// One program of one item, the call itself. Every call here is a whole
/// workload over thousands to millions of records and is its own cache
/// context; the harness's mix-in items are sized for calls a thousand times
/// smaller and would only add noise.
fn build_workload(name: &str, _n: usize) -> Program {
    let mut w = Program::new();
    w.program(name, |b| {
        b.stage(vec![harness::algo_call()]);
    });
    w
}

/// The routine of one section: `Evs<N, W>` for the section's `workload` and
/// record count.
fn routine_for(config: &BenchConfig) -> Option<RoutineSpec> {
    let workload = Workload::from_name(&config.workload)?;
    macro_rules! pick {
        ($($n:literal),*) => {
            match (config.n, workload) {
                $(
                    ($n, Workload::ElementWise) => routine_bridge!(Evs<$n, { Workload::ElementWise.tag() }>),
                    ($n, Workload::Branching) => routine_bridge!(Evs<$n, { Workload::Branching.tag() }>),
                    ($n, Workload::Accumulator) => routine_bridge!(Evs<$n, { Workload::Accumulator.tag() }>),
                    ($n, Workload::WideParallel) => routine_bridge!(Evs<$n, { Workload::WideParallel.tag() }>),
                )*
                _ => return None,
            }
        };
    }
    let bridge = pick!(4096, 65536, 1048576, 4194304);
    Some(RoutineSpec {
        name: config.bench_name.rsplit('/').next()?.to_string(),
        bridge,
    })
}

//! Orchestrator for the single-trunk convergence bench.
//!
//! The harness's own driver (`driver::drive`) runs everything: selection,
//! validation across arms before timing, one worker subprocess per arm and
//! mode, the statistics, `results/`, history. This binary registers the two
//! things that are this bench's own: the routine of each section, and the
//! workload program around the measured call.
//!
//! A section is named `<workload>_w<W>`, with an optional `_<band>` suffix
//! for a size band timed on a smaller budget: the workload row (or `short`,
//! the one-to-four-morsel band of `fold_light`) and the worker count. Every
//! section runs the same routine, `Conv<N>`, and differs only in the input
//! builder, which writes the workload id and `W` into the input each arm
//! reads.
//!
//! A run starts from a stage: `../stage.sh .` builds the member and prints a
//! directory, and `<that>/release/single-trunk-convergence` is run from this
//! directory. `bench.toml` names the arms under `target/`, which this binary
//! points at its own stage, so the arms timed are the ones staged beside it
//! and never what another worktree last built into the shared target
//! directory. The stage is verified before the run and after it.
//!
//! `--summary [<results dir>]` folds the promoted CSVs into
//! `<results dir>/summary.csv` instead of running anything (see `summary`).

use std::path::Path;
use std::process::ExitCode;

use bench_stage::Stage;
use mockspace_bench_core::{byte_routine_dispatch, routine_bridge};
use mockspace_bench_harness::driver::{DriverRegistry, drive};
use mockspace_bench_harness::{self as harness, BenchConfig, RoutineSpec, Workload};
use stc_support::workload::{APPEND, FOLD_HEAVY, FOLD_LIGHT, by_name};
use stc_support::{Conv, input_bytes};

mod section;
mod summary;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    if args.next().as_deref() == Some("--summary") {
        let dir = args.next().unwrap_or_else(|| "results".to_string());
        return match summary::run(Path::new(&dir)) {
            Ok(path) => {
                println!("{}", path.display());
                ExitCode::SUCCESS
            },
            Err(e) => {
                eprintln!("error: {e}");
                ExitCode::FAILURE
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

/// The program around the measured call: the light surrounding work of the
/// harness (a scalar dependency stream, branchy context) on the caller
/// thread between frames, as a frame of a real pipeline has.
fn build_workload(name: &str, _n: usize) -> Workload {
    let mut w = Workload::new();
    w.program(name, |b| {
        b.stage(vec![
            harness::algo_call(),
            harness::scalar_work(32),
            harness::branch_work(16),
            harness::light_scalar(),
        ]);
    });
    w
}

/// The routine of one section: `Conv<N>` with the input builder for its
/// workload and worker count.
fn routine_for(config: &BenchConfig) -> Option<RoutineSpec> {
    let section = section::parse(&config.bench_name)?;
    let kind = match section.workload {
        "short" => FOLD_LIGHT,
        name => by_name(name)?.id,
    };
    let w = section.workers;
    let mut bridge = match config.n {
        16 => routine_bridge!(Conv<16>),
        64 => routine_bridge!(Conv<64>),
        256 => routine_bridge!(Conv<256>),
        1024 => routine_bridge!(Conv<1024>),
        4096 => routine_bridge!(Conv<4096>),
        16384 => routine_bridge!(Conv<16384>),
        65536 => routine_bridge!(Conv<65536>),
        262144 => routine_bridge!(Conv<262144>),
        1048576 => routine_bridge!(Conv<1048576>),
        4194304 => routine_bridge!(Conv<4194304>),
        16777216 => routine_bridge!(Conv<16777216>),
        134217728 => routine_bridge!(Conv<134217728>),
        _ => return None,
    };
    bridge.input_builder = builder(kind, w)?;
    Some(RoutineSpec {
        name: config.bench_name.rsplit('/').next()?.to_string(),
        bridge,
    })
}

/// The input builder for workload `kind` at `w` workers.
fn builder(kind: u32, w: u32) -> Option<fn(u64) -> Vec<u8>> {
    macro_rules! by_w {
        ($k:expr) => {
            match w {
                1 => Some(input_bytes::<$k, 1> as fn(u64) -> Vec<u8>),
                2 => Some(input_bytes::<$k, 2>),
                4 => Some(input_bytes::<$k, 4>),
                8 => Some(input_bytes::<$k, 8>),
                _ => None,
            }
        };
    }
    match kind {
        FOLD_LIGHT => by_w!(FOLD_LIGHT),
        FOLD_HEAVY => by_w!(FOLD_HEAVY),
        APPEND => by_w!(APPEND),
        _ => None,
    }
}

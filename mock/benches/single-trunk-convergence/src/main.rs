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
//! Run it from this directory after `cargo build --release --workspace`. The
//! arms build into the shared target directory; `bench.toml` names them
//! under `target/`, which this binary links to the directory it was itself
//! built into when the link is missing.

use std::path::Path;
use std::process::ExitCode;

use mockspace_bench_core::{byte_routine_dispatch, routine_bridge};
use mockspace_bench_harness::driver::{DriverRegistry, drive};
use mockspace_bench_harness::{self as harness, BenchConfig, RoutineSpec, Workload};
use stc_support::workload::{APPEND, FOLD_HEAVY, FOLD_LIGHT, by_name};
use stc_support::{Conv, input_bytes};

fn main() -> ExitCode {
    if let Err(e) = link_target() {
        eprintln!("error: {e}");
        return ExitCode::FAILURE;
    }
    drive(&DriverRegistry {
        build_workload,
        routine_for,
        // Every section resolves through `routine_for`; the byte fallback is
        // never reached.
        byte_dispatch: byte_routine_dispatch!(out = 16, sizes = [16]),
    })
}

/// Point `./target` at the build directory this binary sits in, the shared
/// one the arms built into, when nothing is there yet.
fn link_target() -> Result<(), String> {
    let link = Path::new("target");
    if link.symlink_metadata().is_ok() {
        return Ok(());
    }
    let exe = std::env::current_exe().map_err(|e| format!("locating this binary: {e}"))?;
    // <target>/release/single-trunk-convergence
    let target = exe
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| format!("{} has no build directory above it", exe.display()))?;
    std::os::unix::fs::symlink(target, link)
        .map_err(|e| format!("linking ./target to {}: {e}", target.display()))
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
    let section = config.bench_name.rsplit('/').next()?;
    let (name, w) = section.rsplit_once("_w")?;
    let kind = if name == "short" { FOLD_LIGHT } else { by_name(name)?.id };
    // `<workload>_w<W>` or `<workload>_w<W>_<band>`.
    let w: u32 = w.split('_').next()?.parse().ok()?;
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
        name: section.to_string(),
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

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
//! verified before the run and after it. Each cell the run promoted then gets
//! the stage's build record beside it, and its findings lose the harness's
//! stock verdict lines.
//!
//! `--verdict [<results dir>]` runs nothing. It reads the medians a run
//! promoted into `results/` and holds each cell to the design's ceiling in
//! `evs_support::expected` (see `verdict`). These were the ignored
//! `tests/perf_gate.rs` oracles; a timing assertion means something only over
//! a release build timed by the harness, so that is where they are applied.

use std::path::Path;
use std::process::ExitCode;

use evs_support::{Evs, Workload};
use mockspace_bench_core::{byte_routine_dispatch, routine_bridge};
use mockspace_bench_harness::driver::{DriverRegistry, drive};
use mockspace_bench_harness::staging::StagedBuild;
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
    // `_link` holds `target` on the staged build until the run ends, and
    // removes it then, so the next plain build writes a target of its own.
    let staged = StagedBuild::of_current_exe().and_then(|s| {
        let link = s.link_into(Path::new("target"))?;
        Ok((s, link))
    });
    let (stage, _link) = match staged {
        Ok(staged) => staged,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        },
    };
    let started = std::time::SystemTime::now();
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
    if let Err(e) = settle(Path::new("results"), started, &stage) {
        eprintln!("error: {e}");
        return ExitCode::FAILURE;
    }
    code
}

/// What every cell this run promoted gets before it is committed: its
/// findings without the harness's stock verdict lines and bold
/// (`bench_arm::findings::plain`), and `<cell>.build.txt` beside them, the
/// stage's record of the commit the arms were built from and whatever was
/// dirty against it. A cell is this run's when its findings were written
/// after the run started; the cells of earlier runs keep their own records.
fn settle(
    results: &Path,
    started: std::time::SystemTime,
    stage: &StagedBuild,
) -> Result<(), String> {
    let record = stage.build_record().map_err(|e| e.to_string())?;
    let read = |dir: &Path| std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()));
    for section in read(results)? {
        let section = section.map_err(|e| e.to_string())?.path();
        if !section.is_dir() {
            continue;
        }
        for file in read(&section)? {
            let path = file.map_err(|e| e.to_string())?.path();
            let Some(cell) = path
                .file_name()
                .and_then(|n| n.to_str())
                .and_then(|n| n.strip_suffix("_findings.md"))
            else {
                continue;
            };
            let written = std::fs::metadata(&path)
                .and_then(|m| m.modified())
                .map_err(|e| format!("{}: {e}", path.display()))?;
            if written < started {
                continue;
            }
            let io = |e: std::io::Error| format!("{}: {e}", path.display());
            let report = std::fs::read_to_string(&path).map_err(io)?;
            std::fs::write(&path, bench_arm::findings::plain(&report)).map_err(io)?;
            let build = section.join(format!("{cell}.build.txt"));
            std::fs::write(&build, &record).map_err(|e| format!("{}: {e}", build.display()))?;
        }
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    /// A stage holding one driver, as `stage.sh` lays one out, with the
    /// build record `commit: test`.
    fn stage_in(dir: &Path) -> StagedBuild {
        std::fs::create_dir_all(dir.join("release")).unwrap();
        std::fs::write(dir.join("release/driver"), b"abc").unwrap();
        std::fs::write(
            dir.join("binaries.sha256"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad  release/driver\n",
        )
        .unwrap();
        std::fs::write(dir.join("build.txt"), "commit: test\n").unwrap();
        StagedBuild::of_executable(&dir.join("release/driver")).unwrap()
    }

    /// The cell this run wrote is stripped and gets the stage's record; the
    /// cell an earlier run wrote keeps its findings as they were and gets no
    /// record of a build it was not timed on.
    #[test]
    fn only_this_runs_cells_are_settled() {
        let root = std::env::temp_dir().join(format!("evs-settle-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let stage = stage_in(&root.join("stage"));
        let section = root.join("results/element_wise");
        std::fs::create_dir_all(&section).unwrap();
        let stock = "**Fastest:** x\n_Why it matters:_ switch.\n";
        std::fs::write(section.join("element_wise_n4096_findings.md"), stock).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(20));
        let started = std::time::SystemTime::now();
        std::thread::sleep(std::time::Duration::from_millis(20));
        std::fs::write(section.join("element_wise_n65536_findings.md"), stock).unwrap();

        settle(&root.join("results"), started, &stage).unwrap();

        let read = |name: &str| std::fs::read_to_string(section.join(name)).ok();
        assert_eq!(
            read("element_wise_n65536_findings.md").as_deref(),
            Some("Fastest: x\n")
        );
        assert_eq!(
            read("element_wise_n65536.build.txt").as_deref(),
            Some("commit: test\n")
        );
        assert_eq!(
            read("element_wise_n4096_findings.md").as_deref(),
            Some(stock)
        );
        assert_eq!(read("element_wise_n4096.build.txt"), None);
        std::fs::remove_dir_all(&root).ok();
    }
}

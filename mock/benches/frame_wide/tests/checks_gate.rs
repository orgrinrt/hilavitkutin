//! The driver runs `./run-checks.sh` before it times anything.
//!
//! The gate is worth nothing until it has been seen stopping a run, so this
//! puts a stub `run-checks.sh` in a directory of its own and drives the three
//! paths through it. The stub touches a marker file, which is what tells a
//! refusal that never reached the script from one that ran it and obeyed it.
//!
//! No cell is ever timed here. The gate fires before the manifest is loaded,
//! so a directory with no `bench.toml` is enough: a run that gets past the
//! gate fails on the missing manifest, and that failure is the evidence it
//! got past. Each case is two process spawns and no build.
//!
//! `harness = false`, like `negative_control`, because this drives the built
//! driver rather than calling into the library.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use frame_wide::locate_arm;

/// The driver as built into the target directory, found beside the arms.
///
/// `locate_arm` is the member's own answer to where the build is, so this
/// reuses it rather than reimplementing the search and drifting from it.
fn unstaged_driver() -> Result<PathBuf, String> {
    let any_arm = format!(
        "{}fw_hand{}",
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_SUFFIX
    );
    let arm = locate_arm(Path::new(&any_arm))?;
    let dir = arm
        .parent()
        .ok_or_else(|| format!("`{}` has no parent directory", arm.display()))?;
    // The arm may be found under `deps`, where the test binary runs, while the
    // driver is linked one directory up.
    let beside = dir.join("frame_wide");
    let above = dir.parent().map(|p| p.join("frame_wide"));
    if beside.is_file() {
        return Ok(beside);
    }
    match above {
        Some(bin) if bin.is_file() => Ok(bin),
        _ => {
            Err(format!(
                "no `frame_wide` driver beside or above `{}`. Run `cargo build --release --workspace`",
                dir.display()
            ))
        },
    }
}

/// The driver as a timing run starts it: staged by `../stage.sh` into a
/// directory of its own, and run from there.
fn staged_driver() -> Result<PathBuf, String> {
    let member = Path::new(env!("CARGO_MANIFEST_DIR"));
    let out = std::env::temp_dir().join(format!(
        "frame_wide_checks_gate_stage_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&out);
    let staged = Command::new(member.join("../stage.sh"))
        .arg(member)
        .arg(&out)
        .output()
        .map_err(|e| format!("running ../stage.sh: {e}"))?;
    if !staged.status.success() {
        return Err(format!(
            "../stage.sh failed: {}",
            String::from_utf8_lossy(&staged.stderr)
        ));
    }
    Ok(out.join("release").join("frame_wide"))
}

/// A driver run from the shared target directory refuses before it reaches
/// the checks, so nothing it would time can be another worktree's build.
fn an_unstaged_driver_is_refused(_: &Path) -> Result<(), String> {
    let bin = unstaged_driver()?;
    let dir = stub_dir("unstaged", 0)?;
    let (ok, err) = run(&bin, &dir, &[])?;
    if ok {
        return Err("an unstaged driver ran".to_string());
    }
    if stub_ran(&dir) {
        return Err("an unstaged driver reached the checks".to_string());
    }
    if !err.contains("not started from a stage") {
        return Err(format!("the refusal does not say why: {err}"));
    }
    Ok(())
}

/// A directory holding a stub `run-checks.sh` that exits with `code` and
/// records that it ran.
///
/// The directory has no `bench.toml`, so nothing downstream of the gate can
/// succeed and no cell can be timed by accident.
fn stub_dir(name: &str, code: i32) -> Result<PathBuf, String> {
    let dir = std::env::temp_dir().join(format!("frame_wide_checks_gate_{name}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).map_err(|e| format!("creating {}: {e}", dir.display()))?;

    let script = dir.join("run-checks.sh");
    fs::write(
        &script,
        format!("#!/bin/sh\ntouch \"$(dirname \"$0\")/ran\"\nexit {code}\n"),
    )
    .map_err(|e| format!("writing {}: {e}", script.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755))
            .map_err(|e| format!("chmod {}: {e}", script.display()))?;
    }

    Ok(dir)
}

/// Run the driver in `dir` with `args`, returning its stderr and whether it
/// succeeded.
fn run(bin: &Path, dir: &Path, args: &[&str]) -> Result<(bool, String), String> {
    let out = Command::new(bin)
        .args(args)
        .current_dir(dir)
        .output()
        .map_err(|e| format!("running the driver in {}: {e}", dir.display()))?;
    Ok((
        out.status.success(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    ))
}

/// Whether the stub recorded that it ran.
fn stub_ran(dir: &Path) -> bool {
    dir.join("ran").exists()
}

/// A failing `run-checks.sh` stops the run, and the message says nothing was
/// timed.
fn a_failing_check_stops_the_run(bin: &Path) -> Result<(), String> {
    let dir = stub_dir("failing", 1)?;
    let (ok, err) = run(bin, &dir, &[])?;
    if ok {
        return Err("the driver succeeded although `run-checks.sh` failed".to_string());
    }
    if !stub_ran(&dir) {
        return Err("`run-checks.sh` was never run, so the refusal was not the gate".to_string());
    }
    if !err.contains("run-checks.sh failed") {
        return Err(format!(
            "the refusal does not name the failing checks: {err}"
        ));
    }
    Ok(())
}

/// The control for the one above: a passing `run-checks.sh` lets the run
/// through, so the refusal there was the script's verdict and not the gate
/// refusing everything.
///
/// Past the gate the run dies on the missing `bench.toml`, which is the proof
/// it got past, and no cell is timed.
fn a_passing_check_lets_the_run_through(bin: &Path) -> Result<(), String> {
    let dir = stub_dir("passing", 0)?;
    let (ok, err) = run(bin, &dir, &[])?;
    if ok {
        return Err("the driver succeeded with no `bench.toml`".to_string());
    }
    if !stub_ran(&dir) {
        return Err("`run-checks.sh` was never run".to_string());
    }
    if err.contains("run-checks.sh failed") {
        return Err(format!("a passing script was read as a failure: {err}"));
    }
    // Past the gate the next thing the driver does is load the manifest, and
    // there is none here. Matching the file name or the kind of error keeps
    // this from breaking on the harness's exact wording.
    if !(err.contains("bench.toml") || err.to_lowercase().contains("no such file")) {
        return Err(format!("the run stopped somewhere unexpected: {err}"));
    }
    Ok(())
}

/// `--skip-checks` never reaches the script, and says so.
fn skip_checks_does_not_run_the_script(bin: &Path) -> Result<(), String> {
    let dir = stub_dir("skipped", 1)?;
    let (_, err) = run(bin, &dir, &["--skip-checks"])?;
    if stub_ran(&dir) {
        return Err("`--skip-checks` ran the script anyway".to_string());
    }
    if !err.contains("checks skipped") {
        return Err(format!("a skipped run does not say so: {err}"));
    }
    Ok(())
}

/// `--report-only` times nothing, so it neither runs the checks nor claims to
/// have skipped them.
fn report_only_does_not_reach_the_gate(bin: &Path) -> Result<(), String> {
    let dir = stub_dir("report_only", 1)?;
    let (_, err) = run(bin, &dir, &["--report-only"])?;
    if stub_ran(&dir) {
        return Err("`--report-only` ran the checks".to_string());
    }
    if err.contains("checks skipped") {
        return Err("`--report-only` reported skipping checks it never had to run".to_string());
    }
    Ok(())
}

fn main() -> ExitCode {
    let bin = match staged_driver() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("checks_gate: {e}");
            return ExitCode::FAILURE;
        },
    };

    let cases: [(&str, fn(&Path) -> Result<(), String>); 5] = [
        (
            "an_unstaged_driver_is_refused",
            an_unstaged_driver_is_refused,
        ),
        (
            "a_failing_check_stops_the_run",
            a_failing_check_stops_the_run,
        ),
        (
            "a_passing_check_lets_the_run_through",
            a_passing_check_lets_the_run_through,
        ),
        (
            "skip_checks_does_not_run_the_script",
            skip_checks_does_not_run_the_script,
        ),
        (
            "report_only_does_not_reach_the_gate",
            report_only_does_not_reach_the_gate,
        ),
    ];

    let mut failed = 0;
    for (name, case) in cases {
        match case(&bin) {
            Ok(()) => eprintln!("ok   {name}"),
            Err(e) => {
                eprintln!("FAIL {name}: {e}");
                failed += 1;
            },
        }
    }

    if let Some(stage) = bin.parent().and_then(Path::parent) {
        let _ = fs::remove_dir_all(stage);
    }
    if failed > 0 {
        eprintln!("checks_gate: {failed} of {} failed", cases.len());
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

//! `stage.sh` end to end on a fixture member: what it copies, what it records,
//! and that a driver run from the stage sees the stage while one run from the
//! target directory is refused.

use std::path::{Path, PathBuf};
use std::process::Command;

fn here() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn stage_script() -> PathBuf {
    here().join("../stage.sh")
}

fn fixture() -> PathBuf {
    here().join("tests/fixture-member")
}

fn stage_into(out: &Path) -> String {
    let _ = std::fs::remove_dir_all(out);
    let run = Command::new(stage_script())
        .arg(fixture())
        .arg(out)
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "stage.sh failed:\n{}",
        String::from_utf8_lossy(&run.stderr)
    );
    String::from_utf8(run.stdout).unwrap()
}

#[test]
fn the_stage_holds_the_driver_and_the_arm_and_nothing_else() {
    let out = std::env::temp_dir().join(format!("stage-script-{}", std::process::id()));
    let printed = stage_into(&out);
    assert_eq!(
        printed.trim(),
        out.canonicalize().unwrap().to_string_lossy()
    );

    let mut staged: Vec<String> = std::fs::read_dir(out.join("release"))
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    staged.sort();
    let arm = format!(
        "{}stage_fixture_arm{}",
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_SUFFIX
    );
    assert_eq!(staged, vec![arm, "stage-fixture-driver".to_string()]);

    let record = std::fs::read_to_string(out.join("build.txt")).unwrap();
    assert!(record.contains("commit: "), "{record}");
    assert!(record.contains("dirty"), "{record}");
    // The record is committed beside the results, so it names the member by
    // its path in the repository and never by where one clone sits.
    let repo = here().join("../../..").canonicalize().unwrap();
    assert!(
        !record.contains(repo.to_str().unwrap()),
        "the build record names the clone's own path: {record}"
    );

    let seen = Command::new(out.join("release/stage-fixture-driver"))
        .output()
        .unwrap();
    assert!(
        seen.status.success(),
        "{}",
        String::from_utf8_lossy(&seen.stdout)
    );
    assert!(String::from_utf8_lossy(&seen.stdout).starts_with("staged "));
    std::fs::remove_dir_all(&out).ok();
}

/// `stage.sh --verify`, which the shell runs of `frame_wide` call between
/// cells, is silent on a stage as it was written and names the file that
/// changed when an arm is replaced after staging.
#[test]
fn the_script_verifies_a_stage_and_names_an_arm_replaced_after_it() {
    let out = std::env::temp_dir().join(format!("stage-script-verify-{}", std::process::id()));
    stage_into(&out);
    let verify = || {
        Command::new(stage_script())
            .arg("--verify")
            .arg(&out)
            .output()
            .unwrap()
    };
    let intact = verify();
    assert!(
        intact.status.success() && intact.stderr.is_empty(),
        "an intact stage failed --verify: {}",
        String::from_utf8_lossy(&intact.stderr)
    );
    let arm = format!(
        "{}stage_fixture_arm{}",
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_SUFFIX
    );
    std::fs::write(out.join("release").join(&arm), b"rebuilt elsewhere").unwrap();
    let changed = verify();
    let err = String::from_utf8_lossy(&changed.stderr);
    assert!(!changed.status.success(), "a replaced arm passed --verify");
    assert!(err.contains(&arm), "--verify does not name the arm: {err}");
    std::fs::remove_dir_all(&out).ok();
}

/// The negative control for the stage itself: an arm replaced after staging
/// makes the staged driver's own check fail.
#[test]
fn an_arm_replaced_after_staging_is_seen_by_the_staged_driver() {
    let out = std::env::temp_dir().join(format!("stage-script-swap-{}", std::process::id()));
    stage_into(&out);
    let arm = out.join("release").join(format!(
        "{}stage_fixture_arm{}",
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_SUFFIX
    ));
    std::fs::write(&arm, b"rebuilt elsewhere").unwrap();
    let seen = Command::new(out.join("release/stage-fixture-driver"))
        .output()
        .unwrap();
    assert_eq!(
        seen.status.code(),
        Some(2),
        "{}",
        String::from_utf8_lossy(&seen.stdout)
    );
    std::fs::remove_dir_all(&out).ok();
}

#[test]
fn a_stage_is_never_written_over() {
    let out = std::env::temp_dir().join(format!("stage-script-twice-{}", std::process::id()));
    stage_into(&out);
    let again = Command::new(stage_script())
        .arg(fixture())
        .arg(&out)
        .output()
        .unwrap();
    assert!(!again.status.success());
    std::fs::remove_dir_all(&out).ok();
}

/// Stage the fixture with its checks set to pass or to fail, and answer the
/// run and whether the checks ran.
fn stage_with_checks(tag: &str, fail: bool) -> (std::process::Output, bool, PathBuf) {
    let out = std::env::temp_dir().join(format!("stage-script-{tag}-{}", std::process::id()));
    let ran = std::env::temp_dir().join(format!("stage-script-{tag}-ran-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out);
    let _ = std::fs::remove_file(&ran);
    let mut cmd = Command::new(stage_script());
    cmd.arg(fixture())
        .arg(&out)
        .env("STAGE_FIXTURE_CHECKS_RAN", &ran);
    if fail {
        cmd.env("STAGE_FIXTURE_CHECKS_FAIL", "1");
    }
    let run = cmd.output().unwrap();
    let checked = ran.exists();
    let _ = std::fs::remove_file(&ran);
    (run, checked, out)
}

/// A member's `run-checks.sh` runs before it is staged.
#[test]
fn the_members_checks_run_before_it_is_staged() {
    let (run, checked, out) = stage_with_checks("checked", false);
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(
        checked,
        "stage.sh staged the member without running its checks"
    );
    assert!(out.join("binaries.sha256").exists());
    std::fs::remove_dir_all(&out).ok();
}

/// The negative control: failing checks stop the stage before anything is
/// built or written, so no number can be taken from a member whose own tests
/// are red.
#[test]
fn a_member_whose_checks_fail_is_not_staged() {
    let (run, checked, out) = stage_with_checks("refused", true);
    assert!(
        !run.status.success(),
        "stage.sh staged a member whose checks failed"
    );
    assert!(checked, "the refusal never reached the checks");
    let stderr = String::from_utf8_lossy(&run.stderr);
    assert!(stderr.contains("checks of"), "{stderr}");
    assert!(
        !out.exists(),
        "a refused stage left {} behind",
        out.display()
    );
}

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

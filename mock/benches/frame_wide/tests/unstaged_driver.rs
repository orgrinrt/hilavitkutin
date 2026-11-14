//! The driver refuses to run from the shared target directory.
//!
//! A timing run starts from a stage: `../stage.sh` runs this member's
//! `run-checks.sh`, builds it and copies the build into a directory of its
//! own, so the arms the driver loads are the ones staged beside it and the
//! checks have passed on them. This holds the driver to that check: run from
//! the target directory, it stops before it times anything and says why.
//! The stage's own gate is tested with the script, in `bench-stage`.

use std::path::{Path, PathBuf};
use std::process::Command;

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

#[test]
fn an_unstaged_driver_is_refused() {
    let bin = unstaged_driver().unwrap();
    let dir = std::env::temp_dir().join(format!("frame_wide_unstaged_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let run = Command::new(&bin).current_dir(&dir).output().unwrap();
    std::fs::remove_dir_all(&dir).ok();
    assert!(!run.status.success(), "an unstaged driver ran");
    let err = String::from_utf8_lossy(&run.stderr);
    assert!(
        err.contains("not started from a stage"),
        "the refusal does not say why: {err}"
    );
}

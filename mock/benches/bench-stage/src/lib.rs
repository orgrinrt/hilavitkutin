//! Where a bench driver's binaries were staged, and whether they still are.
//!
//! Every worktree of this repository builds into one shared target directory,
//! so a driver timing arms out of it can load arms another worktree rebuilt a
//! minute ago, from another branch, and nothing fails. A driver that times
//! anything runs from a stage instead: `mock/benches/stage.sh <member>` builds
//! the member and copies its executable and every arm cdylib into
//! `<stage>/release/`, beside a `binaries.sha256` and a `build.txt` naming the
//! commit and whatever was dirty. The driver is then run as
//! `<stage>/release/<driver>`, so its arms are the ones beside it.
//!
//! [`Stage::of_current_exe`] refuses a driver that was not started from a
//! stage, and [`Stage::verify`] asks the script whether every staged file is
//! still the one it checksummed. The layout and the checksum are the script's;
//! this crate reads them and defines neither.

use std::path::{Path, PathBuf};
use std::process::Command;

/// The file `stage.sh` writes the checksums into, beside `release/`.
const CHECKSUMS: &str = "binaries.sha256";

/// The file `stage.sh` writes the build record into, beside `release/`.
const BUILD_RECORD: &str = "build.txt";

/// A staged build directory: `<dir>/release/` and the two records beside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage {
    dir: PathBuf,
}

impl Stage {
    /// The stage the running driver was started from.
    ///
    /// Refuses unless the executable sits at `<dir>/release/<name>` with
    /// `<dir>/binaries.sha256` beside it, which is what `stage.sh` writes and
    /// what the shared target directory never has.
    pub fn of_current_exe() -> Result<Self, String> {
        let exe = std::env::current_exe().map_err(|e| format!("cannot read current_exe: {e}"))?;
        Self::of_executable(&exe)
    }

    /// The stage an executable at `exe` belongs to, by the same rule as
    /// [`Stage::of_current_exe`].
    pub fn of_executable(exe: &Path) -> Result<Self, String> {
        let release = exe
            .parent()
            .filter(|p| p.file_name().is_some_and(|n| n == "release"));
        let dir = release.and_then(Path::parent);
        match dir {
            Some(dir) if dir.join(CHECKSUMS).is_file() => {
                Ok(Self {
                    dir: dir.to_path_buf(),
                })
            },
            _ => {
                Err(format!(
                    "`{}` was not started from a stage, so the arms beside it are whatever the \
                 shared target directory holds now. Stage the member and run the staged \
                 driver: `mock/benches/stage.sh <member>` prints a directory, run \
                 `<that>/release/{}` from the member directory",
                    exe.display(),
                    exe.file_name()
                        .map(|n| n.to_string_lossy())
                        .unwrap_or_default()
                ))
            },
        }
    }

    /// The stage's root, holding `release/` and the two records.
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Where the staged executable and arms are.
    pub fn release(&self) -> PathBuf {
        self.dir.join("release")
    }

    /// The build record `stage.sh` wrote: the member, the commit, the time
    /// and every path that was dirty against the commit.
    pub fn build_record(&self) -> Result<String, String> {
        let path = self.dir.join(BUILD_RECORD);
        std::fs::read_to_string(&path).map_err(|e| format!("cannot read {}: {e}", path.display()))
    }

    /// Ask `stage.sh --verify` whether every staged file still has the
    /// checksum written at staging.
    pub fn verify(&self) -> Result<(), String> {
        let out = Command::new(stage_script())
            .arg("--verify")
            .arg(&self.dir)
            .output()
            .map_err(|e| format!("could not run {}: {e}", stage_script().display()))?;
        if out.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
        }
    }
}

/// The one staging script, beside this crate.
fn stage_script() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("stage.sh")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("bench-stage-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("release")).unwrap();
        dir
    }

    fn write_stage(dir: &Path) {
        std::fs::write(dir.join("release/driver"), b"one build").unwrap();
        let sums = Command::new("sha256sum")
            .arg("release/driver")
            .current_dir(dir)
            .output()
            .unwrap();
        assert!(sums.status.success());
        std::fs::write(dir.join(CHECKSUMS), sums.stdout).unwrap();
        std::fs::write(dir.join(BUILD_RECORD), "commit: test\n").unwrap();
    }

    #[test]
    fn a_driver_in_a_stage_is_found_and_verifies() {
        let dir = scratch("found");
        write_stage(&dir);
        let stage = Stage::of_executable(&dir.join("release/driver")).unwrap();
        assert_eq!(stage.dir(), dir.as_path());
        assert_eq!(stage.build_record().unwrap(), "commit: test\n");
        stage.verify().unwrap();
        std::fs::remove_dir_all(&dir).ok();
    }

    /// The negative control: a staged file replaced after staging, which is
    /// what a build in another worktree does to the shared target directory.
    #[test]
    fn a_file_replaced_after_staging_fails_verify() {
        let dir = scratch("replaced");
        write_stage(&dir);
        let stage = Stage::of_executable(&dir.join("release/driver")).unwrap();
        std::fs::write(dir.join("release/driver"), b"another build").unwrap();
        let err = stage.verify().unwrap_err();
        assert!(err.contains("changed after staging"), "{err}");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_driver_in_the_target_directory_is_refused() {
        let dir = scratch("target");
        std::fs::write(dir.join("release/driver"), b"unstaged").unwrap();
        let err = Stage::of_executable(&dir.join("release/driver")).unwrap_err();
        assert!(err.contains("not started from a stage"), "{err}");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_driver_outside_a_release_directory_is_refused() {
        let dir = scratch("deps");
        std::fs::create_dir_all(dir.join("release/deps")).unwrap();
        write_stage(&dir);
        assert!(Stage::of_executable(&dir.join("release/deps/driver")).is_err());
        std::fs::remove_dir_all(&dir).ok();
    }
}

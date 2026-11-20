//! What the tests that build share: their state removed when they end, and
//! the arms the benches run, named as the benches name them.

#![allow(dead_code)]

use std::path::PathBuf;

use build_call::{Arm, Build, Relink, Replay};
use hilavitkutin_build::{ChecksumFreshness, NoIncremental};

/// The set of every arm in the tree that builds without incremental
/// compilation: the toolchain's own backend and linker.
pub type Plain = (ChecksumFreshness, NoIncremental);

/// `build-static`'s `llvm_non_incremental`.
pub fn off() -> Arm {
    Arm::of::<Plain, Build>()
}

/// `link-static`'s `default_relink`.
pub fn relink() -> Arm {
    Arm::of::<Plain, Relink>()
}

/// `link-static`'s `default_replay`.
pub fn replay() -> Arm {
    Arm::of::<Plain, Replay>()
}

/// A test bench's state under the build root, removed when this is dropped,
/// which a panic does too, so a failed test leaves nothing on the disk either.
pub struct Removed(PathBuf);

impl Removed {
    /// The state of the test bench `bench`, removed now where a run before
    /// left any, and again when the guard is dropped.
    pub fn bench(bench: &str) -> Removed {
        let dir = build_call::root().join(bench);
        let _ = std::fs::remove_dir_all(&dir);
        Removed(dir)
    }
}

impl Drop for Removed {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

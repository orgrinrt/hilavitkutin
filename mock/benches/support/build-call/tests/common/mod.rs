//! What the tests that build share: their state removed when they end.

use std::path::PathBuf;

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

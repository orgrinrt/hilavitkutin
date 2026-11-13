//! Host scaffolding for the A4 spike: where the extension's cdylib lands.

use std::path::PathBuf;

/// The extension cdylib in the same profile directory as the running test.
pub fn ext_path() -> PathBuf {
    let exe = std::env::current_exe().expect("exe");
    let profile = exe.parent().and_then(|d| d.parent()).expect("profile dir");
    profile.join("liba4_ext.so")
}

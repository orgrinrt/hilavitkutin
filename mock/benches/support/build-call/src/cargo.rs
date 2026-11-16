//! The cargo the fixture is built with, and what its messages say.
//!
//! Every call runs the repository's pinned toolchain, read from
//! `mock/rust-toolchain.toml`, with checksum freshness asked for on the
//! command line so the freshness rule holds wherever the fixture sits, and
//! with every wrapper and flag variable the driver inherited taken out, so
//! what is built is the toolchain's default for the target and the fixture's
//! own profile and nothing else. Target and build directories are the cell's
//! own.

use std::path::Path;
use std::process::Command;

/// Variables an inherited environment could carry into the build and change
/// what it compiles. None of them is set by the benches.
const CLEARED: [&str; 10] = [
    "RUSTFLAGS",
    "CARGO_ENCODED_RUSTFLAGS",
    "RUSTDOCFLAGS",
    "RUSTC_WRAPPER",
    "RUSTC_WORKSPACE_WRAPPER",
    "CARGO_BUILD_RUSTC_WRAPPER",
    "CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER",
    "CARGO_BUILD_RUSTFLAGS",
    "CARGO_TARGET_DIR",
    "CARGO_BUILD_TARGET_DIR",
];

/// The pinned toolchain channel, from the mock workspace's toolchain file.
pub fn channel() -> String {
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../rust-toolchain.toml");
    let text = std::fs::read_to_string(&file).unwrap_or_else(|e| panic!("{}: {e}", file.display()));
    text.lines()
        .find_map(|l| {
            let l = l.trim();
            l.strip_prefix("channel")
                .and_then(|rest| rest.trim_start().strip_prefix('='))
                .map(|v| v.trim().trim_matches('"').to_string())
        })
        .unwrap_or_else(|| panic!("{}: no channel line", file.display()))
}

/// A cargo command in `ws`, building into `target`, with incremental
/// compilation as given. `args` follow the unstable flag.
pub fn command(ws: &Path, target: &Path, tmp: &Path, incremental: bool, args: &[&str]) -> Command {
    let mut c = Command::new("cargo");
    c.current_dir(ws)
        .env("RUSTUP_TOOLCHAIN", channel())
        .env("CARGO_INCREMENTAL", if incremental { "1" } else { "0" })
        .env("TMPDIR", tmp);
    for v in CLEARED {
        c.env_remove(v);
    }
    c.arg("-Zchecksum-freshness").args(args);
    c.args(["--offline", "--locked", "--target-dir"])
        .arg(target);
    c.arg("--config").arg(format!(
        "build.build-dir={:?}",
        target.display().to_string()
    ));
    c
}

/// The fixture crates cargo compiled rather than found fresh, by name, from
/// its `compiler-artifact` messages.
pub fn compiled(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .filter(|l| l.contains("\"reason\":\"compiler-artifact\"") && l.contains("\"fresh\":false"))
        .filter_map(target_name)
        .filter(|n| n.starts_with("fx_"))
        .collect()
}

/// The name of a message's target.
fn target_name(line: &str) -> Option<String> {
    let after = &line[line.find("\"target\":{")? ..];
    let rest = &after[after.find("\"name\":\"")? + 8 ..];
    Some(rest[.. rest.find('"')?].to_string())
}

/// The executable cargo reports for the facade.
pub fn executable(stdout: &str) -> Option<String> {
    stdout
        .lines()
        .filter(|l| l.contains("\"reason\":\"compiler-artifact\""))
        .filter(|l| target_name(l).as_deref() == Some("fx_facade"))
        .find_map(|l| {
            let rest = &l[l.find("\"executable\":\"")? + 14 ..];
            Some(rest[.. rest.find('"')?].to_string())
        })
}

/// Whether cargo said it waited on another process's lock.
pub fn waited(stderr: &str) -> bool {
    stderr.contains("Blocking waiting for file lock")
}

/// The bytes under `dir`, every file counted once by its length.
pub fn bytes_under(dir: &Path) -> u64 {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    entries
        .filter_map(Result::ok)
        .map(|e| {
            match e.file_type() {
                Ok(t) if t.is_dir() => bytes_under(&e.path()),
                Ok(t) if t.is_file() => e.metadata().map_or(0, |m| m.len()),
                _ => 0,
            }
        })
        .sum()
}

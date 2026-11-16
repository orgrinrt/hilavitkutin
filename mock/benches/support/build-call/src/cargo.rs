//! The cargo the fixture is built with, and what its messages say.
//!
//! Every call runs the pinned toolchain's own `cargo` and `rustc` by path
//! ([`crate::toolchain`]), and nothing of the environment it inherits reaches
//! the build but the variables in [`ALLOWED`]: the environment is cleared and
//! only those are put back. Cargo's configuration is read from two places
//! besides the command line, the cargo home and every directory above the
//! working directory, so the cargo home is the cell's own empty `home/` and
//! the working directory is the filesystem root, with the fixture named by
//! `--manifest-path`; a configuration file at the root itself refuses the
//! call. What is built is then the toolchain's default for the target and the
//! fixture's own profile, with checksum freshness asked for on the command
//! line and the cell's own target and build directories.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{Dirs, toolchain};

/// The inherited variables a call keeps: the path, which the link driver is
/// found on.
///
/// FIXME(the_build_and_link_bench_assumes_an_elf_unix_host_and_fails): a
/// Windows host needs `SystemRoot` and its kin for a process to start at all,
/// and has none of them here.
pub const ALLOWED: [&str; 1] = ["PATH"];

/// Clear `c`'s environment and put back the inherited values of [`ALLOWED`].
pub fn keep_only_allowed(c: &mut Command) {
    c.env_clear();
    for v in ALLOWED {
        if let Some(value) = std::env::var_os(v) {
            c.env(v, value);
        }
    }
}

/// The directory cargo is run from: the root of the filesystem `ws` is on,
/// which has no directory above it to carry a configuration. Refuses a root
/// that carries one itself.
pub fn config_free_dir(ws: &Path) -> Result<PathBuf, String> {
    let top = ws
        .ancestors()
        .last()
        .ok_or_else(|| format!("{}: no root", ws.display()))?
        .to_path_buf();
    for name in ["config.toml", "config"] {
        let file = top.join(".cargo").join(name);
        if file.exists() {
            return Err(format!(
                "{} would reach every build of the fixture; the bench cannot run on this filesystem",
                file.display()
            ));
        }
    }
    Ok(top)
}

/// A cargo command for the cell `dirs`, with incremental compilation as given.
/// `args` follow the unstable flag. Panics where no directory free of cargo
/// configuration can be found, which fails the call's setup.
pub fn command(dirs: &Dirs, incremental: bool, args: &[&str]) -> Command {
    let tc = toolchain::get();
    let ws = dirs.ws();
    let mut c = Command::new(&tc.cargo);
    keep_only_allowed(&mut c);
    c.current_dir(config_free_dir(&ws).unwrap_or_else(|e| panic!("build-call: {e}")))
        .env("CARGO_HOME", dirs.home())
        .env("RUSTC", &tc.rustc)
        .env("CARGO_INCREMENTAL", if incremental { "1" } else { "0" })
        .env("TMPDIR", dirs.tmp());
    c.arg("-Zchecksum-freshness").args(args);
    c.arg("--manifest-path").arg(ws.join("Cargo.toml"));
    c.args(["--offline", "--locked", "--target-dir"])
        .arg(dirs.target());
    c.arg("--config").arg(format!(
        "build.build-dir={:?}",
        dirs.target().display().to_string()
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

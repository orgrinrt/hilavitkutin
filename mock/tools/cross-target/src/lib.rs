//! `mock cross-target`: one run of author 1 through the reference runner,
//! built for the host and for `wasm32-unknown-unknown`, its digest streams
//! compared line by line.
//!
//! The loop and its two hosts live in `front/`, a workspace of their own. The
//! native host prints one line per digest; the wasm host is loaded in node by
//! `front/run.mjs`, which prints the same lines from what the module leaves in
//! memory. The run holds when both print the same lines, the lines are not
//! empty, and they equal `front/digests.txt`, so two hosts that drifted
//! together are caught as well.
//!
//! `--control` builds the loop with its `control` feature, which folds the
//! width of a pointer into each case's first digest. The two targets then
//! differ, and the control holds when the comparison sees it.
//!
//! A check rather than a lint because each run builds two release binaries
//! and starts node, which no gate pays for on every commit. A mismatch is
//! reported as an inconclusive run, which exits non-zero: the contract gives a
//! check no blocking finding.

use std::path::{Path, PathBuf};
use std::process::Command;

use mockspace::tool::{ArgSpec, NotALint, Purpose, Tool, ToolContext, ToolReport};

pub struct CrossTarget;

const NAME: &str = "cross-target";
const WASM_TARGET: &str = "wasm32-unknown-unknown";

const ARGS: &[ArgSpec] = &[ArgSpec {
    name:        "--control",
    required:    false,
    description: "build the pointer-width mutant and require the two streams to differ",
}];

impl Tool for CrossTarget {
    fn name(&self) -> &'static str {
        NAME
    }

    fn description(&self) -> &'static str {
        "runs author 1 through the reference runner natively and as wasm under node, and compares the digest streams"
    }

    fn purpose(&self) -> Purpose {
        Purpose::Check(NotALint::NoFailingCase)
    }

    fn inputs(&self) -> &'static [&'static str] {
        &[
            "tools/cross-target/front",
            "crates/hilavitkutin-contracts/src",
            "crates/hilavitkutin-reference/src",
            "crates/hilavitkutin-frozen-authors/src",
            "crates/hilavitkutin-frozen-implementors/src",
        ]
    }

    fn args(&self) -> &[ArgSpec] {
        ARGS
    }

    fn help(&self) -> &'static str {
        "Builds `front/` twice in release, for the host and for wasm32-unknown-unknown, runs the native binary and \
         `node front/run.mjs` on the module, and compares the lines. Needs node on PATH and the wasm32 target \
         installed. Builds into the directory cargo names for `front/`, the workspace's one build directory."
    }

    fn run(&self, ctx: &ToolContext<'_>) -> ToolReport {
        let control = ctx.args.contains(&"--control");
        let front = ctx.mock_dir.join("tools").join(NAME).join("front");
        match streams(&front, control) {
            Err(reason) => ToolReport::inconclusive(reason),
            Ok((native, wasm)) => {
                let golden = std::fs::read_to_string(front.join("digests.txt")).unwrap_or_default();
                match verdict(&native, &wasm, &golden, control) {
                    Ok(said) => ToolReport::reported(said, lines(&native).len()),
                    Err(reason) => ToolReport::inconclusive(reason),
                }
            },
        }
    }
}

/// Where cargo builds `front/`: the workspace's one build directory, as `cargo metadata` names it.
fn target_dir(manifest: &Path) -> Result<PathBuf, String> {
    let mut cargo = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    cargo
        .args(["metadata", "--format-version", "1", "--no-deps", "--manifest-path"])
        .arg(manifest);
    let meta = output(cargo, "cargo metadata")?;
    let key = "\"target_directory\":\"";
    let start = meta
        .find(key)
        .ok_or("cargo metadata names no target directory")?
        + key.len();
    let end = meta[start ..]
        .find('"')
        .ok_or("cargo metadata names no target directory")?;
    Ok(PathBuf::from(&meta[start .. start + end]))
}

/// Builds both hosts and returns what each printed.
fn streams(front: &Path, control: bool) -> Result<(String, String), String> {
    let manifest = front.join("Cargo.toml");
    let target = target_dir(&manifest)?;
    let features: &[&str] =
        if control { &["--features", "cross-target-loop/control"] } else { &[] };
    build(&manifest, &["-p", "cross-target-native"], features)?;
    build(
        &manifest,
        &["-p", "cross-target-wasm", "--target", WASM_TARGET],
        features,
    )?;
    let native = output(
        Command::new(target.join("release").join("cross-target-native")),
        "the native host",
    )?;
    let module = target
        .join(WASM_TARGET)
        .join("release")
        .join("cross_target_wasm.wasm");
    let mut node = Command::new("node");
    node.arg(front.join("run.mjs")).arg(module);
    let wasm = output(node, "node on the wasm host")?;
    Ok((native, wasm))
}

fn build(manifest: &Path, which: &[&str], features: &[&str]) -> Result<(), String> {
    let mut cargo = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    cargo
        .args(["build", "--release", "--quiet", "--manifest-path"])
        .arg(manifest)
        .args(which)
        .args(features);
    output(cargo, &format!("cargo build {}", which.join(" "))).map(drop)
}

/// The stdout of a command that has to succeed, or why it did not.
fn output(mut command: Command, what: &str) -> Result<String, String> {
    let out = command
        .output()
        .map_err(|e| format!("{what} did not start: {e}"))?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        let tail: Vec<&str> = err.lines().rev().take(12).collect();
        let tail: Vec<&str> = tail.into_iter().rev().collect();
        return Err(format!(
            "{what} failed ({}):\n{}",
            out.status,
            tail.join("\n")
        ));
    }
    String::from_utf8(out.stdout).map_err(|_| format!("{what} printed something that is not UTF-8"))
}

fn lines(text: &str) -> Vec<&str> {
    text.lines().filter(|l| !l.is_empty()).collect()
}

/// The first line, 1-based, at which two streams part, with each side's line.
fn first_difference<'a>(a: &[&'a str], b: &[&'a str]) -> Option<(usize, &'a str, &'a str)> {
    let longest = a.len().max(b.len());
    (0 .. longest).find_map(|i| {
        let (x, y) = (
            a.get(i).copied().unwrap_or("<none>"),
            b.get(i).copied().unwrap_or("<none>"),
        );
        (x != y).then_some((i + 1, x, y))
    })
}

/// Whether the run holds: what it says when it does, and why when it does not.
fn verdict(native: &str, wasm: &str, golden: &str, control: bool) -> Result<String, String> {
    let (native, wasm, golden) = (lines(native), lines(wasm), lines(golden));
    if native.is_empty() || wasm.is_empty() {
        return Err(format!(
            "a host printed nothing: native {} lines, wasm {} lines",
            native.len(),
            wasm.len()
        ));
    }
    let parted = first_difference(&native, &wasm);
    if control {
        return match parted {
            Some((at, n, w)) => {
                Ok(format!(
                    "CONTROL HOLDS: the streams part at line {at}\n  native: {n}\n  wasm:   {w}\n"
                ))
            },
            None => {
                Err(format!(
                    "control: native and wasm agree on all {} lines, so the comparison cannot see a target difference",
                    native.len()
                ))
            },
        };
    }
    if let Some((at, n, w)) = parted {
        return Err(format!(
            "native and wasm part at line {at}\n  native: {n}\n  wasm:   {w}"
        ));
    }
    if let Some((at, n, g)) = first_difference(&native, &golden) {
        return Err(format!(
            "both hosts agree but part from front/digests.txt at line {at}\n  hosts:  {n}\n  golden: {g}"
        ));
    }
    Ok(format!(
        "SAME: {} lines, native and wasm agree with each other and with front/digests.txt\n",
        native.len()
    ))
}

mockspace::lint_pack! {
    tools: [CrossTarget],
}

#[cfg(test)]
mod tests;

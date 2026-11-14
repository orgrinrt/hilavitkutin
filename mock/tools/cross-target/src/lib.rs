//! `mock cross-target <target>`: whether the frozen units leave the same
//! digest stream on another target as on the host.
//!
//! The tool builds the probe beside it twice, for the host and for the target
//! asked about, and runs both. The probe runs the frozen units under the
//! reference runner and prints one digest a frame. The two streams are then
//! compared frame by frame, and a frame whose digests differ is a finding. A
//! build or a run that fails says nothing about the units, so it is reported
//! as a run that cannot be trusted, never as a difference and never as none.
//!
//! The target is the question, which is why this is a tool: a build for
//! another target needs that target installed and something to run it, and
//! which target is worth asking about is the asker's to say.

use std::path::Path;
use std::process::Command;

use mockspace::tool::{ArgSpec, NotALint, Outcome, Purpose, Tool, ToolContext, ToolReport};
use mockspace::{LintError, Severity};

/// The tool's name, and its directory under the mock directory's `tools/`.
pub const NAME: &str = "cross-target";

/// The targets the tool can run a build for, each with the program cargo is
/// told to run that build with and the file beside this tool it is handed.
pub const TARGETS: [(&str, &str, &str); 1] = [("wasm32-unknown-unknown", "node", "run.cjs")];

/// The probe's manifest, from the tool's directory.
pub const PROBE: &str = "probe/Cargo.toml";

/// A digest stream out of the probe's output: one digest a line, sixteen hex
/// digits. Anything else on any line and the output is not a stream.
#[must_use]
pub fn stream(text: &str) -> Option<Vec<u64>> {
    let digests: Option<Vec<u64>> = text
        .lines()
        .map(|line| {
            (line.len() == 16 && line.bytes().all(|b| b.is_ascii_hexdigit()))
                .then(|| u64::from_str_radix(line, 16).ok())
                .flatten()
        })
        .collect();
    digests.filter(|d| !d.is_empty())
}

/// Where two streams differ, one line for each frame, frames counted from
/// one. Nothing where they are the same stream.
#[must_use]
pub fn differences(host: &[u64], other: &[u64], target: &str) -> Vec<String> {
    let mut out: Vec<String> = host
        .iter()
        .zip(other)
        .enumerate()
        .filter(|(_, (host, other))| host != other)
        .map(|(at, (host, other))| {
            format!(
                "frame {}: the host's digest is {host:016x} and {target}'s is {other:016x}",
                at + 1
            )
        })
        .collect();
    if host.len() != other.len() {
        out.push(format!(
            "the host's stream is {} frames and {target}'s is {}",
            host.len(),
            other.len()
        ));
    }
    out
}

/// The cargo call that builds the probe and runs it: for the host with no
/// `target`, else for that target under its runner. `tool` is this tool's
/// directory.
#[must_use]
pub fn probe(tool: &Path, target: Option<(&str, &str, &str)>, features: &[&str]) -> Command {
    let mut cargo = Command::new("cargo");
    cargo
        .current_dir(tool.join("probe"))
        .args(["run", "--quiet", "--manifest-path"])
        .arg(tool.join(PROBE));
    if !features.is_empty() {
        cargo.arg("--features").arg(features.join(","));
    }
    if let Some((triple, program, file)) = target {
        // a TOML array, so a directory with a space in it stays one argument
        let runner = format!(
            "target.{triple}.runner=['{program}', '{}']",
            tool.join(file).display()
        );
        cargo.args(["--target", triple, "--config"]).arg(runner);
    }
    cargo
}

/// Run a probe call and read its stream, or say what went wrong with it.
///
/// # Errors
///
/// The call could not be run, it failed, or what it printed is not a stream.
pub fn streamed(mut call: Command, of: &str) -> Result<Vec<u64>, String> {
    let done = call
        .output()
        .map_err(|e| format!("cannot run cargo for {of}: {e}"))?;
    if !done.status.success() {
        let said = String::from_utf8_lossy(&done.stderr);
        let last: Vec<&str> = said.lines().rev().take(12).collect();
        let last: Vec<&str> = last.into_iter().rev().collect();
        return Err(format!(
            "the probe for {of} did not build and run ({}):\n{}",
            done.status,
            last.join("\n")
        ));
    }
    let text = String::from_utf8_lossy(&done.stdout);
    stream(&text).ok_or_else(|| format!("the probe for {of} printed no digest stream:\n{text}"))
}

/// The host's stream and the target's, from the probe under `tool`.
///
/// # Errors
///
/// Either build or run failed, as [`streamed`] says.
pub fn both(
    tool: &Path,
    target: (&str, &str, &str),
    features: &[&str],
) -> Result<(Vec<u64>, Vec<u64>), String> {
    let host = streamed(probe(tool, None, features), "the host")?;
    let other = streamed(probe(tool, Some(target), features), target.0)?;
    Ok((host, other))
}

/// The check.
pub struct CrossTarget;

impl Tool for CrossTarget {
    fn name(&self) -> &'static str {
        NAME
    }

    fn description(&self) -> &'static str {
        "runs the frozen units under the reference runner on the host and on another target and compares the digests"
    }

    fn purpose(&self) -> Purpose {
        Purpose::Check(NotALint::TakesAQuestion)
    }

    fn args(&self) -> &'static [ArgSpec] {
        &[ArgSpec {
            name:        "target",
            required:    true,
            description: "the target triple to hold against the host; `wasm32-unknown-unknown` is the one it can run",
        }]
    }

    fn help(&self) -> &'static str {
        "Builds `probe/` beside this tool for the host and for the target named, and runs both: the \
         host's build directly, a wasm build with `run.cjs` under node. The probe takes the frozen \
         authors' files as they are, runs the nine units under the reference runner for eight frames \
         and prints the digest of each frame's stores. A frame whose digest differs between the two \
         is a finding. A build or a run that fails makes the run inconclusive. Needs the target \
         installed for the pinned toolchain, and node for a wasm target."
    }

    fn run(&self, ctx: &ToolContext<'_>) -> ToolReport {
        let asked = ctx.args.first().copied().unwrap_or_default();
        let Some(target) = TARGETS
            .iter()
            .copied()
            .find(|(triple, ..)| *triple == asked)
        else {
            let known: Vec<&str> = TARGETS.iter().map(|(triple, ..)| *triple).collect();
            return ToolReport::inconclusive(format!(
                "nothing here runs a build for `{asked}`; the targets it can run are: {}",
                known.join(", ")
            ));
        };
        let tool = ctx.mock_dir.join("tools").join(NAME);
        let (host, other) = match both(&tool, target, &[]) {
            Ok(streams) => streams,
            Err(why) => return ToolReport::inconclusive(why),
        };
        let differ = differences(&host, &other, target.0);
        if differ.is_empty() {
            let mut output = format!(
                "the host and {} leave the same digest for each of {} frames\n",
                target.0,
                host.len()
            );
            for digest in &host {
                output.push_str(&format!("{digest:016x}\n"));
            }
            return ToolReport::reported(output, host.len());
        }
        let findings = differ
            .into_iter()
            .map(|message| {
                LintError::with_severity(
                    "hilavitkutin-frozen-authors".to_string(),
                    0,
                    NAME,
                    message,
                    Severity::HARD_ERROR,
                )
            })
            .collect();
        ToolReport {
            outcome: Outcome::Findings(findings),
            output:  String::new(),
        }
    }
}

mockspace::lint_pack! {
    tools: [CrossTarget],
}

#[cfg(test)]
mod tests;

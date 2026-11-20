//! `mock old-arvo-names`: how many paths of the old arvo revision each crate
//! still names.
//!
//! The port of hilavitkutin onto current arvo moves one crate at a time, and
//! what is left is the count of old paths in the crates not moved yet. The tool
//! reads the tracked Rust files under the mock directory's `crates` and
//! `benches`, counts each occurrence of a path of one of the ten crates of the
//! old revision (`arvo::`, `arvo_bits::`, and so on), and prints the counts for
//! each crate, the crates with none among them. It counts text, as the plan's
//! first count did, so a path in a comment or a string counts the same as one in
//! code, and `use arvo::{A, B}` is one path.
//!
//! It is a report. The answer is the output, no count is a failing one, and the
//! run always exits clean unless it could not read the repository.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::Command;

use mockspace::tool::{NotALint, Purpose, Tool, ToolContext, ToolReport};

/// The tool's name, and its directory under the mock directory's `tools/`.
pub const NAME: &str = "old-arvo-names";

/// The crates of the old arvo revision `1b2796e9`, the ten `mock/Cargo.toml`
/// pins, each by the name a path starts with. `arvo_bits` is among them though
/// current arvo has a crate of that name too: what the port writes of it changes
/// shape, and the count says how many sites are not moved yet.
pub const OLD_PATHS: [&str; 10] = [
    "arvo",
    "arvo_bits",
    "arvo_bitmask",
    "arvo_comb",
    "arvo_graph",
    "arvo_hash",
    "arvo_refit",
    "arvo_sparse",
    "arvo_spectral",
    "arvo_tensor",
];

/// One count for each of [`OLD_PATHS`], in its order.
pub type Counts = [usize; OLD_PATHS.len()];

const fn is_ident(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// The old paths `text` names: an identifier that is exactly one of
/// [`OLD_PATHS`], not the tail of a longer one, followed by `::`.
#[must_use]
pub fn count(text: &str) -> Counts {
    let mut counts = [0; OLD_PATHS.len()];
    if cfg!(old_arvo_names_mutant_blind) {
        return counts;
    }
    let bytes = text.as_bytes();
    let mut at = 0;
    while let Some(found) = text[at ..].find("arvo") {
        let start = at + found;
        let mut end = start;
        while end < bytes.len() && is_ident(bytes[end]) {
            end += 1;
        }
        let tail_of_a_longer_name =
            !cfg!(old_arvo_names_mutant_boundary) && start > 0 && is_ident(bytes[start - 1]);
        if !tail_of_a_longer_name && text[end ..].starts_with("::")
            && let Some(slot) = OLD_PATHS
                .iter()
                .position(|path| *path == &text[start .. end])
            {
                counts[slot] += 1;
            }
        at = end;
    }
    counts
}

/// The crate or bench a file belongs to, from its path under the mock
/// directory with `/` separators, or nothing for a file the tool does not
/// read. Directly under `benches` is the group `benches`.
#[must_use]
pub fn group_of(under_mock: &str) -> Option<String> {
    let mut parts = under_mock.split('/');
    match (parts.next(), parts.next(), parts.next()) {
        (Some("crates"), Some(name), Some(_)) => Some(name.to_string()),
        (Some("benches"), Some(name), Some(_)) => Some(format!("benches/{name}")),
        (Some("benches"), Some(_), None) => Some("benches".to_string()),
        _ => None,
    }
}

/// The counts of each group, over files given as their path under the mock
/// directory and their text. Only `.rs` files with a group count.
#[must_use]
pub fn tally<'a>(files: impl IntoIterator<Item = (&'a str, &'a str)>) -> BTreeMap<String, Counts> {
    let mut groups: BTreeMap<String, Counts> = BTreeMap::new();
    for (path, text) in files {
        let Some(group) = path.ends_with(".rs").then(|| group_of(path)).flatten() else {
            continue;
        };
        let total = groups.entry(group).or_insert([0; OLD_PATHS.len()]);
        for (slot, n) in total.iter_mut().zip(count(text)) {
            *slot += n;
        }
    }
    groups
}

/// The tracked files under the mock directory's `crates` and `benches`, as
/// their paths under the mock directory. `mock_rel` is the mock directory under
/// the repository root.
///
/// # Errors
///
/// `git` could not be run in the repository, or it refused.
pub fn tracked(repo_root: &Path, mock_rel: &str) -> Result<Vec<String>, String> {
    let done = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(["ls-files", "-z", "--"])
        .arg(format!("{mock_rel}/crates"))
        .arg(format!("{mock_rel}/benches"))
        .output()
        .map_err(|e| format!("cannot run git in {}: {e}", repo_root.display()))?;
    if !done.status.success() {
        return Err(format!(
            "git ls-files refused in {} ({}): {}",
            repo_root.display(),
            done.status,
            String::from_utf8_lossy(&done.stderr).trim()
        ));
    }
    let prefix = format!("{mock_rel}/");
    Ok(String::from_utf8_lossy(&done.stdout)
        .split('\0')
        .filter_map(|path| path.strip_prefix(&prefix))
        .map(str::to_string)
        .collect())
}

/// The counts of a repository: its tracked Rust files, read from the working
/// tree, and how many were read. A tracked file that is not there or is not
/// text is left out of both.
///
/// # Errors
///
/// The tracked files could not be listed, as [`tracked`] says.
pub fn scan(repo_root: &Path, mock_rel: &str) -> Result<(BTreeMap<String, Counts>, usize), String> {
    let mut files: Vec<(String, String)> = Vec::new();
    for path in tracked(repo_root, mock_rel)? {
        if !path.ends_with(".rs") {
            continue;
        }
        if let Ok(text) = std::fs::read_to_string(repo_root.join(mock_rel).join(&path)) {
            files.push((path, text));
        }
    }
    let read = files.len();
    let groups = tally(files.iter().map(|(p, t)| (p.as_str(), t.as_str())));
    Ok((groups, read))
}

/// The report: one line for each group with its total and the paths it names,
/// the groups that name none included, and a last line for the whole.
#[must_use]
pub fn render(groups: &BTreeMap<String, Counts>, crates: &BTreeSet<String>) -> String {
    let names: BTreeSet<&String> = groups.keys().chain(crates).collect();
    let width = names
        .iter()
        .map(|n| n.len())
        .max()
        .unwrap_or(0)
        .max("all".len());
    let mut whole = [0; OLD_PATHS.len()];
    let mut out = String::new();
    for name in names {
        let counts = groups.get(name).copied().unwrap_or([0; OLD_PATHS.len()]);
        out.push_str(&line(name, width, &counts));
        for (slot, n) in whole.iter_mut().zip(counts) {
            *slot += n;
        }
    }
    out.push_str(&line("all", width, &whole));
    out
}

fn line(name: &str, width: usize, counts: &Counts) -> String {
    let total: usize = counts.iter().sum();
    let named: Vec<String> = OLD_PATHS
        .iter()
        .zip(counts)
        .filter(|(_, n)| **n > 0)
        .map(|(path, n)| format!("{path}:: {n}"))
        .collect();
    if named.is_empty() {
        format!("{name:<width$}  {total:>5}\n")
    } else {
        format!("{name:<width$}  {total:>5}  {}\n", named.join(", "))
    }
}

/// The report.
pub struct OldArvoNames;

impl Tool for OldArvoNames {
    fn name(&self) -> &'static str {
        NAME
    }

    fn description(&self) -> &'static str {
        "counts the old arvo paths each crate's tracked Rust files still name"
    }

    fn purpose(&self) -> Purpose {
        Purpose::Check(NotALint::NoFailingCase)
    }

    fn help(&self) -> &'static str {
        "Reads the tracked `.rs` files under the mock directory's `crates` and `benches` and counts \
         each occurrence of a path of the ten crates of the old arvo revision: `arvo::`, \
         `arvo_bits::`, `arvo_bitmask::`, `arvo_comb::`, `arvo_graph::`, `arvo_hash::`, \
         `arvo_refit::`, `arvo_sparse::`, `arvo_spectral::` and `arvo_tensor::`. Prints a line for \
         each crate, and for each bench directory, with its total and the paths it names, the ones \
         that name none as zero, then a line for the whole. It counts text, so a path in a comment \
         counts, and `use arvo::{A, B}` is one path. It gates nothing and exits clean."
    }

    fn run(&self, ctx: &ToolContext<'_>) -> ToolReport {
        let Ok(mock_rel) = ctx.mock_dir.strip_prefix(ctx.repo_root) else {
            return ToolReport::inconclusive(format!(
                "the mock directory {} is not under the repository {}",
                ctx.mock_dir.display(),
                ctx.repo_root.display()
            ));
        };
        let mock_rel = mock_rel.to_string_lossy().replace('\\', "/");
        match scan(ctx.repo_root, &mock_rel) {
            Ok((groups, read)) => ToolReport::reported(render(&groups, ctx.all_crates), read),
            Err(why) => ToolReport::inconclusive(why),
        }
    }
}

mockspace::lint_pack! {
    tools: [OldArvoNames],
}

#[cfg(test)]
mod tests;

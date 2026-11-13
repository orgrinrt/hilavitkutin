//! Lint: a frozen crate's ledger holds over every file of the crate.
//!
//! A frozen crate changes only through a line in its ledger. `LEDGER.toml`
//! carries one line per file: its path, its form, the content hash, the
//! number of edits made to it so far and whether its form is locked.
//! `BREAKS.toml` carries one line per edit. The lint reads every file of the
//! crate, its manifest and any build script with its sources and tests, and
//! refuses a file with no line, a line naming a file that is gone, a hash that
//! is not the file's, an edit count that is not the number of edits recorded,
//! and a locked line whose count moved since its lock. It also refuses a
//! dependency in the manifest on anything but `hilavitkutin-contracts`, since
//! a frozen crate is written from the contracts alone. The crate's documents
//! (`*.md.tmpl`) and the two ledgers themselves have no line.
//!
//! A ledger line is
//!
//! ```toml
//! [[file]]
//! path = "src/lib.rs"
//! form = "value"
//! hash = "fnv1a64:0123456789abcdef"
//! breaks = 0
//! locked = false
//! ```
//!
//! and a locked line adds `locked_breaks`, the count recorded at its lock. The
//! form is a word the crate chooses, and the lint checks only that it is
//! there. A `BREAKS.toml` line is `[[break]]` with `slice`, `file`, `form`,
//! `hash` and `reason`. Only that subset of TOML is read: one `key = value` a
//! line, strings in double quotes with no escapes, counts in decimal, `#`
//! comments. The hash is FNV-1a over the file's bytes, 64 bits, written as
//! `fnv1a64:` and sixteen hex digits.
//!
//! The crate is read through the lint's view: every file under its
//! directory, and the bytes of each one the ledger lists. There is no escape
//! by `lint:allow`.

use mockspace::{CrateLint, Lint, LintContext, LintError, Severity};

#[path = "frozen/scan.rs"]
mod scan;

pub fn lint() -> Box<dyn CrateLint> {
    Box::new(FrozenLedger)
}

struct FrozenLedger;

const NAME: &str = "frozen-ledger";
const LEDGER: &str = "LEDGER.toml";
const BREAKS: &str = "BREAKS.toml";
const MANIFEST: &str = "Cargo.toml";
/// The one crate a frozen crate may depend on.
const CONTRACTS: &str = "hilavitkutin-contracts";

impl Lint for FrozenLedger {
    fn name(&self) -> &'static str {
        NAME
    }

    fn description(&self) -> &'static str {
        "a frozen crate's ledger holds over every file of the crate"
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }

    fn per_file(&self) -> bool {
        false
    }

    fn markers_decide(&self) -> bool {
        false
    }
}

impl CrateLint for FrozenLedger {
    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        if !scan::applies_to(ctx.crate_name) {
            return Vec::new();
        }
        let view = ctx.view();
        let root = ctx.crate_dir();
        let text = |name: &str| view.text(root.join(name)).unwrap_or_default();
        let files: Vec<String> = view
            .walk(root, |_, _| false)
            .iter()
            .filter_map(|file| file.strip_prefix(root).ok())
            .map(|rel| rel.to_string_lossy().replace('\\', "/"))
            .filter(|rel| listed(rel))
            .collect();
        let found = if view.is_file(root.join(LEDGER)) {
            findings(text(LEDGER), text(BREAKS), text(MANIFEST), &files, |path| {
                view.bytes(root.join(path)).map(<[u8]>::to_vec)
            })
        } else {
            vec![at(
                LEDGER,
                1,
                "a frozen crate carries no `LEDGER.toml`".to_string(),
            )]
        };
        found
            .into_iter()
            .map(|f| {
                let mut e = LintError::with_severity(
                    ctx.crate_name.to_string(),
                    f.line,
                    NAME,
                    f.message,
                    Severity::HARD_ERROR,
                );
                e.path = Some(f.file.to_string());
                e
            })
            .collect()
    }
}

/// Whether a crate-relative file needs a ledger line.
fn listed(rel: &str) -> bool {
    rel != LEDGER && rel != BREAKS && !rel.ends_with(".md.tmpl")
}

/// The content hash as the ledger writes it.
pub(crate) fn hash_text(bytes: &[u8]) -> String {
    let mut h: u64 = 0xCBF2_9CE4_8422_2325;
    for b in bytes {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x0000_0100_0000_01B3);
    }
    format!("fnv1a64:{h:016x}")
}

/// One refused spot: which file, its 1-based line, and what is wrong.
#[derive(Debug, PartialEq)]
pub(crate) struct Finding {
    pub(crate) file:    &'static str,
    pub(crate) line:    usize,
    pub(crate) message: String,
}

fn at(file: &'static str, line: usize, message: String) -> Finding {
    Finding {
        file,
        line,
        message,
    }
}

enum Line<'a> {
    Blank,
    Header(&'a str),
    Pair(&'a str, &'a str),
    Bad,
}

fn line(text: &str) -> Line<'_> {
    let text = match text.split_once('#') {
        Some((before, _)) if !before.contains('"') => before,
        _ => text,
    }
    .trim();
    if text.is_empty() {
        return Line::Blank;
    }
    if let Some(name) = text.strip_prefix("[[").and_then(|t| t.strip_suffix("]]")) {
        return Line::Header(name.trim());
    }
    if let Some(name) = text.strip_prefix('[').and_then(|t| t.strip_suffix(']')) {
        return Line::Header(name.trim());
    }
    match text.split_once('=') {
        Some((key, value)) => Line::Pair(key.trim(), value.trim()),
        None => Line::Bad,
    }
}

fn string(value: &str) -> Option<&str> {
    value
        .strip_prefix('"')
        .and_then(|v| v.strip_suffix('"'))
        .filter(|inner| !inner.contains('"'))
}

#[derive(Default)]
struct Listed<'a> {
    line:          usize,
    path:          Option<&'a str>,
    form:          Option<&'a str>,
    hash:          Option<&'a str>,
    breaks:        Option<usize>,
    locked:        Option<bool>,
    locked_breaks: Option<usize>,
}

/// The ledger's lines, or the malformed lines found reading it.
fn parse_ledger(ledger: &str) -> Result<Vec<Listed<'_>>, Vec<Finding>> {
    let mut rows: Vec<Listed<'_>> = Vec::new();
    let mut bad = Vec::new();
    for (n, text) in ledger.lines().enumerate() {
        let n = n + 1;
        match (line(text), rows.last_mut()) {
            (Line::Blank, _) => {},
            (Line::Header("file"), _) => {
                rows.push(Listed {
                    line: n,
                    ..Listed::default()
                });
            },
            (Line::Pair(key, value), Some(row)) => {
                let ok = match key {
                    "path" => string(value).map(|v| row.path = Some(v)).is_some(),
                    "form" => string(value).map(|v| row.form = Some(v)).is_some(),
                    "hash" => string(value).map(|v| row.hash = Some(v)).is_some(),
                    "breaks" => value.parse().map(|v| row.breaks = Some(v)).is_ok(),
                    "locked_breaks" => value.parse().map(|v| row.locked_breaks = Some(v)).is_ok(),
                    "locked" => value.parse().map(|v| row.locked = Some(v)).is_ok(),
                    _ => false,
                };
                if !ok {
                    bad.push(at(
                        LEDGER,
                        n,
                        format!("`{key} = {value}` is not a ledger field"),
                    ));
                }
            },
            _ => bad.push(at(LEDGER, n, "not a ledger line".to_string())),
        }
    }
    if bad.is_empty() { Ok(rows) } else { Err(bad) }
}

/// The number of `BREAKS.toml` edits for each file, or the malformed lines.
fn parse_breaks(breaks: &str) -> Result<Vec<&str>, Vec<Finding>> {
    let mut files = Vec::new();
    let mut bad = Vec::new();
    let mut in_row = false;
    for (n, text) in breaks.lines().enumerate() {
        match line(text) {
            Line::Blank => {},
            Line::Header("break") => in_row = true,
            Line::Pair("file", value) if in_row => {
                match string(value) {
                    Some(file) => files.push(file),
                    None => bad.push(at(BREAKS, n + 1, "`file` is not a quoted path".to_string())),
                }
            },
            Line::Pair(..) if in_row => {},
            _ => bad.push(at(BREAKS, n + 1, "not a break line".to_string())),
        }
    }
    if bad.is_empty() { Ok(files) } else { Err(bad) }
}

/// Every dependency the manifest names on anything but the contracts.
fn foreign_dependencies(manifest: &str) -> Vec<Finding> {
    let mut found = Vec::new();
    let mut in_dependencies = false;
    for (n, text) in manifest.lines().enumerate() {
        match line(text) {
            Line::Header(section) => {
                in_dependencies = section.ends_with("dependencies");
                let named = section
                    .rsplit_once('.')
                    .map(|(head, name)| (head, name.trim_matches('"')));
                if let Some((head, name)) = named
                    && head.ends_with("dependencies")
                    && name != CONTRACTS
                {
                    found.push(at(MANIFEST, n + 1, format!("a dependency on `{name}`")));
                }
            },
            Line::Pair(key, _) if in_dependencies => {
                let name = key.split('.').next().unwrap_or(key).trim_matches('"');
                if name != CONTRACTS {
                    found.push(at(
                        MANIFEST,
                        n + 1,
                        format!("a dependency on `{name}`: a frozen crate depends on `{CONTRACTS}` alone"),
                    ));
                }
            },
            _ => {},
        }
    }
    found
}

/// Everything the lint refuses, over the text of the two ledgers and the
/// manifest, the files the ledger must list, and the bytes of a file by path.
pub(crate) fn findings<R: Fn(&str) -> Option<Vec<u8>>>(
    ledger: &str,
    breaks: &str,
    manifest: &str,
    files: &[String],
    read: R,
) -> Vec<Finding> {
    let mut found = foreign_dependencies(manifest);
    let rows = match parse_ledger(ledger) {
        Ok(rows) => rows,
        Err(bad) => {
            found.extend(bad);
            return found;
        },
    };
    let edits = match parse_breaks(breaks) {
        Ok(edits) => edits,
        Err(bad) => {
            found.extend(bad);
            return found;
        },
    };
    for file in files {
        if !rows.iter().any(|r| r.path == Some(file.as_str())) {
            found.push(at(LEDGER, 1, format!("`{file}` has no line in the ledger")));
        }
    }
    for row in &rows {
        let (Some(path), Some(hash), Some(count), Some(locked), Some(_)) =
            (row.path, row.hash, row.breaks, row.locked, row.form)
        else {
            found.push(at(
                LEDGER,
                row.line,
                "a line lacks one of path, form, hash, breaks and locked".to_string(),
            ));
            continue;
        };
        if rows.iter().filter(|r| r.path == Some(path)).count() > 1 {
            found.push(at(
                LEDGER,
                row.line,
                format!("`{path}` has more than one line"),
            ));
        }
        let Some(bytes) = read(path) else {
            found.push(at(LEDGER, row.line, format!("`{path}` is listed and gone")));
            continue;
        };
        let now = hash_text(&bytes);
        if now != hash {
            found.push(at(
                LEDGER,
                row.line,
                format!("`{path}` hashes to {now}, not {hash}: an edit needs its line and a break"),
            ));
        }
        let recorded = edits.iter().filter(|e| **e == path).count();
        if recorded != count {
            found.push(at(
                LEDGER,
                row.line,
                format!("`{path}` counts {count} breaks and {recorded} are recorded"),
            ));
        }
        if locked {
            match row.locked_breaks {
                Some(at_lock) if at_lock == count => {},
                Some(at_lock) => {
                    found.push(at(
                        LEDGER,
                        row.line,
                        format!("`{path}` is locked at {at_lock} breaks and counts {count}"),
                    ));
                },
                None => {
                    found.push(at(
                        LEDGER,
                        row.line,
                        format!("`{path}` is locked and records no `locked_breaks`"),
                    ));
                },
            }
        }
    }
    found
}

// Beside the lint in a subdirectory, which the lint discovery does not read.
#[cfg(test)]
#[path = "frozen_ledger/tests.rs"]
mod tests;

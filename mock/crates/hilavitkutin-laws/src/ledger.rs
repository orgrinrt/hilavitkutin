//! The ledger check of a frozen crate, over the text of its `LEDGER.toml` and `BREAKS.toml`.
//!
//! A ledger line is one file:
//!
//! ```toml
//! [[file]]
//! path = "src/lib.rs"
//! form = "store"
//! hash = "fnv1a64:0123456789abcdef"
//! breaks = 0
//! locked = false
//! ```
//!
//! and a locked line adds `locked_breaks`, the count of breaks recorded at its lock. A
//! `BREAKS.toml` line is one edit:
//!
//! ```toml
//! [[break]]
//! slice = "HL0b"
//! file = "src/lib.rs"
//! form = "store"
//! hash = "fnv1a64:0123456789abcdef"
//! reason = "why"
//! ```
//!
//! The check rebuilds each line's hash from its file and refuses on a mismatch, on a `breaks`
//! count that differs from the number of edits recorded for the file, and on a locked line whose `breaks`
//! differs from the count recorded at its lock. Only the subset above is read: one `key = value`
//! a line, strings in double quotes with no escapes, counts in decimal, `#` comments.

use hilavitkutin_contracts::num::{Bool, Count, Index};
use hilavitkutin_contracts::{Maybe, Outcome};

/// The content hash: FNV-1a over the file's bytes, 64 bits, independent of the run digest.
pub const fn hash(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xCBF2_9CE4_8422_2325;
    let mut i = 0;
    while i < bytes.len() {
        h ^= bytes[i] as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01B3);
        i += 1;
    }
    h
}

const PREFIX: &[u8] = b"fnv1a64:";
const HEX: &[u8; 16] = b"0123456789abcdef";

/// The hash of `bytes` as the ledger writes it, `fnv1a64:` and sixteen hex digits.
pub const fn hash_text(bytes: &[u8]) -> [u8; 24] {
    let h = hash(bytes);
    let mut out = [0u8; 24];
    let mut i = 0;
    while i < PREFIX.len() {
        out[i] = PREFIX[i];
        i += 1;
    }
    let mut d = 0;
    while d < 16 {
        out[PREFIX.len() + d] = HEX[((h >> (60 - 4 * d)) & 0xF) as usize];
        d += 1;
    }
    out
}

/// What the check found, the line of the ledger it names counting from one.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Finding {
    /// Every line holds.
    Holds,
    /// A line of either file is outside the subset, or a ledger line lacks a field.
    Malformed {
        line:      Index,
        in_breaks: Bool,
    },
    /// The file a line names could not be read.
    Missing {
        line: Index,
    },
    /// The file's hash is not the line's.
    Mismatch {
        line: Index,
    },
    /// The line's `breaks` is not the number of `BREAKS.toml` edits for its file.
    Breaks {
        line:  Index,
        edits: Count,
    },
    /// A locked line's `breaks` is not the count recorded at its lock.
    Locked {
        line: Index,
    },
}

#[derive(Clone, Copy)]
struct Listed<'a> {
    line:          Index,
    path:          Maybe<&'a str>,
    hash:          Maybe<&'a str>,
    breaks:        Maybe<Count>,
    locked:        Maybe<Bool>,
    locked_breaks: Maybe<Count>,
    form:          Maybe<&'a str>,
}

impl Listed<'_> {
    const fn at(line: Index) -> Self {
        Self {
            line,
            path: Maybe::Isnt,
            hash: Maybe::Isnt,
            breaks: Maybe::Isnt,
            locked: Maybe::Isnt,
            locked_breaks: Maybe::Isnt,
            form: Maybe::Isnt,
        }
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
    match text.split_once('=') {
        Some((key, value)) => Line::Pair(key.trim(), value.trim()),
        None => Line::Bad,
    }
}

fn string(value: &str) -> Maybe<&str> {
    match value.strip_prefix('"').and_then(|v| v.strip_suffix('"')) {
        Some(inner) if !inner.contains('"') => Maybe::Is(inner),
        _ => Maybe::Isnt,
    }
}

fn number(value: &str) -> Maybe<Count> {
    match value.parse() {
        Ok(n) => Maybe::Is(Count::from_host(n)),
        Err(_) => Maybe::Isnt,
    }
}

fn boolean(value: &str) -> Maybe<Bool> {
    match value {
        "true" => Maybe::Is(Bool::TRUE),
        "false" => Maybe::Is(Bool::FALSE),
        _ => Maybe::Isnt,
    }
}

fn malformed(line: Index, in_breaks: Bool) -> Finding {
    Finding::Malformed {
        line,
        in_breaks,
    }
}

/// The `BREAKS.toml` edits for `path`, or the first malformed line.
fn edits_for(breaks: &str, path: &str) -> Outcome<Count, Finding> {
    let mut edits = Count::ZERO;
    let mut in_row = false;
    for (n, text) in breaks.lines().enumerate() {
        match line(text) {
            Line::Blank => {},
            Line::Header("break") => in_row = true,
            Line::Pair("file", value) if in_row => {
                match string(value) {
                    Maybe::Is(file) if file == path => edits = edits.next(),
                    Maybe::Is(_) => {},
                    Maybe::Isnt => {
                        return Outcome::Err(malformed(Index::from_host(n + 1), Bool::TRUE));
                    },
                }
            },
            Line::Pair(..) if in_row => {},
            _ => return Outcome::Err(malformed(Index::from_host(n + 1), Bool::TRUE)),
        }
    }
    Outcome::Ok(edits)
}

fn judge<'a, F: FnMut(&str) -> Maybe<&'a [u8]>>(
    listed: Listed<'_>,
    breaks: &str,
    read: &mut F,
) -> Finding {
    let line = listed.line;
    let (Maybe::Is(path), Maybe::Is(hash_field), Maybe::Is(count), Maybe::Is(locked), Maybe::Is(_)) = (
        listed.path,
        listed.hash,
        listed.breaks,
        listed.locked,
        listed.form,
    ) else {
        return malformed(listed.line, Bool::FALSE);
    };
    let bytes = match read(path) {
        Maybe::Is(bytes) => bytes,
        Maybe::Isnt => {
            return Finding::Missing {
                line,
            };
        },
    };
    if hash_text(bytes).as_slice() != hash_field.as_bytes() {
        return Finding::Mismatch {
            line,
        };
    }
    let edits = match edits_for(breaks, path) {
        Outcome::Ok(edits) => edits,
        Outcome::Err(finding) => return finding,
    };
    if edits.eq(count).not().get() {
        return Finding::Breaks {
            line,
            edits,
        };
    }
    if locked.get() {
        match listed.locked_breaks {
            Maybe::Is(at_lock) if at_lock.eq(count).get() => {},
            Maybe::Is(_) => {
                return Finding::Locked {
                    line,
                };
            },
            Maybe::Isnt => return malformed(listed.line, Bool::FALSE),
        }
    }
    Finding::Holds
}

/// Checks every line of `ledger` against `breaks` and the files `read` returns by path.
pub fn check<'a, F: FnMut(&str) -> Maybe<&'a [u8]>>(
    ledger: &str,
    breaks: &str,
    mut read: F,
) -> Finding {
    let mut listed: Maybe<Listed<'_>> = Maybe::Isnt;
    for (n, text) in ledger.lines().enumerate() {
        let parsed = line(text);
        if let Line::Header(_) = parsed
            && let Maybe::Is(done) = listed {
                let finding = judge(done, breaks, &mut read);
                if finding != Finding::Holds {
                    return finding;
                }
            }
        match (parsed, &mut listed) {
            (Line::Blank, _) => {},
            (Line::Header("file"), _) => listed = Maybe::Is(Listed::at(Index::from_host(n + 1))),
            (Line::Pair(key, value), Maybe::Is(e)) => {
                let ok = match key {
                    "path" => {
                        e.path = string(value);
                        matches!(e.path, Maybe::Is(_))
                    },
                    "form" => {
                        e.form = string(value);
                        matches!(e.form, Maybe::Is(_))
                    },
                    "hash" => {
                        e.hash = string(value);
                        matches!(e.hash, Maybe::Is(_))
                    },
                    "breaks" => {
                        e.breaks = number(value);
                        matches!(e.breaks, Maybe::Is(_))
                    },
                    "locked" => {
                        e.locked = boolean(value);
                        matches!(e.locked, Maybe::Is(_))
                    },
                    "locked_breaks" => {
                        e.locked_breaks = number(value);
                        matches!(e.locked_breaks, Maybe::Is(_))
                    },
                    _ => false,
                };
                if !ok {
                    return malformed(Index::from_host(n + 1), Bool::FALSE);
                }
            },
            _ => return malformed(Index::from_host(n + 1), Bool::FALSE),
        }
    }
    match listed {
        Maybe::Is(done) => judge(done, breaks, &mut read),
        Maybe::Isnt => Finding::Holds,
    }
}

/// Whether `path` has a line in `ledger`.
pub fn lists(ledger: &str, path: &str) -> Bool {
    for text in ledger.lines() {
        if let Line::Pair("path", value) = line(text)
            && let Maybe::Is(p) = string(value)
                && p == path {
                    return Bool::TRUE;
                }
    }
    Bool::FALSE
}

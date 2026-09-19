//! Lint: the engine hands closures to an executor only through its gate.
//!
//! The engine promises every executor one pointer-sized closure per
//! worker (the engine design, Platform implementations). The promise is
//! the api's `OnePointerClosure<F>::FITS`, and the engine forces it in
//! one place, `spawn_one_pointer` in `scheduler/run_parallel.rs`. The
//! gate is a post-monomorphisation assert, so nothing about it shows
//! unless it is actually forced: a `spawn` call somewhere else in the
//! engine, or the forcing statement deleted from `spawn_one_pointer`,
//! builds and passes every test while the promise quietly stops holding.
//!
//! This lint refuses both, in the `hilavitkutin` crate's `src/`:
//!
//! 1. a `spawn` method or path call (`.spawn(`, `::spawn(`, with or
//!    without a turbofish) outside the body of `fn spawn_one_pointer`;
//! 2. a `spawn` call inside that body that no earlier
//!    `let () = OnePointerClosure::<..>::FITS;` statement precedes.
//!
//! Comments, doc comments and string literals are not read. The escape is
//! `// lint:allow(engine-spawn-through-gate)` on the line, with a reason.

use mockspace::{CrateLint, Lint, LintContext, LintError, Severity};

pub fn lint() -> Box<dyn CrateLint> {
    Box::new(EngineSpawnThroughGate)
}

struct EngineSpawnThroughGate;

const NAME: &str = "engine-spawn-through-gate";
const GATE_FN: &str = "spawn_one_pointer";

impl Lint for EngineSpawnThroughGate {
    fn name(&self) -> &'static str {
        NAME
    }

    fn description(&self) -> &'static str {
        "the engine spawns only through `spawn_one_pointer`, which forces `OnePointerClosure` first"
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }

    fn per_file(&self) -> bool {
        false
    }
}

impl CrateLint for EngineSpawnThroughGate {
    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        if !applies_to(ctx.crate_name) {
            return Vec::new();
        }
        let mut errors = Vec::new();
        for file in ctx.all_sources {
            let path = file.rel_path.to_string_lossy().into_owned();
            for finding in findings(&file.text) {
                let mut e = LintError::with_severity(
                    ctx.crate_name.to_string(),
                    finding.line,
                    NAME,
                    finding.message,
                    Severity::HARD_ERROR,
                );
                e.path = Some(path.clone());
                errors.push(e);
            }
        }
        errors
    }
}

/// Only the engine crate hands closures to an executor.
fn applies_to(crate_name: &str) -> bool {
    crate_name == "hilavitkutin"
}

#[derive(Debug, PartialEq)]
struct Finding {
    line:    usize,
    message: String,
}

/// Where the scan is relative to `fn spawn_one_pointer`.
#[derive(Clone, Copy)]
enum Region {
    Outside,
    /// The signature has been seen and its body brace has not.
    Signature,
    /// Inside the body at this brace depth, with whether the gate is forced.
    Body {
        depth:  usize,
        forced: bool,
    },
}

fn findings(text: &str) -> Vec<Finding> {
    let mut out = Vec::new();
    let mut region = Region::Outside;
    let mut in_block_comment = false;

    for (idx, raw) in text.lines().enumerate() {
        let line_no = idx + 1;
        let code = code_of(raw, &mut in_block_comment);
        let allowed = raw.contains("lint:allow(engine-spawn-through-gate)");

        // Everything on the line that moves the region or is judged by it,
        // in the order it appears, so a one-line gate fn reads the same as a
        // multi-line one.
        let mut events: Vec<(usize, Event)> = Vec::new();
        events.extend(
            ident_offsets(&code, &format!("fn {GATE_FN}")).map(|at| (at, Event::Signature)),
        );
        events.extend(forcing_offsets(&code).map(|at| (at, Event::Force)));
        events.extend(spawn_calls(&code).into_iter().map(|at| (at, Event::Spawn)));
        for (at, c) in code.char_indices() {
            match c {
                '{' => events.push((at, Event::Open)),
                '}' => events.push((at, Event::Close)),
                _ => {},
            }
        }
        events.sort_by_key(|&(at, _)| at);

        for (_, event) in events {
            region = match (region, event) {
                (Region::Outside, Event::Signature) => Region::Signature,
                (Region::Signature, Event::Open) => {
                    Region::Body {
                        depth:  1,
                        forced: false,
                    }
                },
                (
                    Region::Body {
                        depth,
                        forced,
                    },
                    Event::Open,
                ) => {
                    Region::Body {
                        depth: depth + 1,
                        forced,
                    }
                },
                (
                    Region::Body {
                        depth: 1,
                        ..
                    },
                    Event::Close,
                ) => Region::Outside,
                (
                    Region::Body {
                        depth,
                        forced,
                    },
                    Event::Close,
                ) => {
                    Region::Body {
                        depth: depth - 1,
                        forced,
                    }
                },
                (
                    Region::Body {
                        depth,
                        ..
                    },
                    Event::Force,
                ) => {
                    Region::Body {
                        depth,
                        forced: true,
                    }
                },
                (r, Event::Spawn) => {
                    let message = match r {
                        Region::Body {
                            forced: true,
                            ..
                        } => None,
                        Region::Body {
                            forced: false,
                            ..
                        } => {
                            Some(format!(
                                "`{GATE_FN}` spawns before `let () = OnePointerClosure::<F>::FITS;` forces the \
                             one-pointer gate; put that statement first, or a wider closure reaches the \
                             executor unchecked"
                            ))
                        },
                        _ => {
                            Some(format!(
                                "a `spawn` call outside `{GATE_FN}`; hand the closure to the executor through \
                             `{GATE_FN}`, which forces `OnePointerClosure` on it"
                            ))
                        },
                    };
                    if let (Some(message), false) = (message, allowed) {
                        out.push(Finding {
                            line: line_no,
                            message,
                        });
                    }
                    r
                },
                (r, _) => r,
            };
        }
    }
    out
}

#[derive(Clone, Copy)]
enum Event {
    Signature,
    Force,
    Spawn,
    Open,
    Close,
}

/// Offsets of every `let () = OnePointerClosure::<..>::FITS` statement.
fn forcing_offsets(code: &str) -> impl Iterator<Item = usize> + '_ {
    code.match_indices("let () =").filter_map(move |(at, _)| {
        let rest = &code[at ..];
        let stmt = &rest[.. rest.find(';').unwrap_or(rest.len())];
        let rhs = stmt["let () =".len() ..].trim();
        (rhs.starts_with("OnePointerClosure") && rhs.ends_with("::FITS")).then_some(at)
    })
}

/// The line with comments and string contents blanked out.
fn code_of(line: &str, in_block_comment: &mut bool) -> String {
    let bytes: Vec<char> = line.chars().collect();
    let mut out = String::with_capacity(line.len());
    let mut i = 0;
    let mut in_str = false;
    while i < bytes.len() {
        let c = bytes[i];
        let next = bytes.get(i + 1).copied();
        if *in_block_comment {
            if c == '*' && next == Some('/') {
                *in_block_comment = false;
                i += 2;
            } else {
                i += 1;
            }
            continue;
        }
        if in_str {
            if c == '\\' {
                i += 2;
                continue;
            }
            if c == '"' {
                in_str = false;
                out.push('"');
            }
            i += 1;
            continue;
        }
        // A char literal, so a quote inside one opens no string. A lifetime
        // (`'a`) has no closing quote two or more places on and is kept.
        if c == '\'' {
            let close = match next {
                Some('\\') => {
                    bytes[i + 2 ..]
                        .iter()
                        .position(|&x| x == '\'')
                        .map(|p| i + 2 + p)
                },
                Some(_) if bytes.get(i + 2) == Some(&'\'') => Some(i + 2),
                _ => None,
            };
            if let Some(end) = close {
                i = end + 1;
                continue;
            }
        }
        match (c, next) {
            ('/', Some('/')) => break,
            ('/', Some('*')) => {
                *in_block_comment = true;
                i += 2;
            },
            ('"', _) => {
                in_str = true;
                out.push('"');
                i += 1;
            },
            _ => {
                out.push(c);
                i += 1;
            },
        }
    }
    out
}

fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Offsets where `needle` occurs in `code` with identifier boundaries on both ends.
fn ident_offsets<'c>(code: &'c str, needle: &str) -> impl Iterator<Item = usize> + 'c {
    let len = needle.len();
    let hits: Vec<usize> = code.match_indices(needle).map(|(at, _)| at).collect();
    hits.into_iter().filter(move |&at| {
        let before = code[.. at].chars().next_back();
        let after = code[at + len ..].chars().next();
        !before.is_some_and(is_ident_char) && !after.is_some_and(is_ident_char)
    })
}

/// Byte offsets of every `spawn` method or path call in `code`.
fn spawn_calls(code: &str) -> Vec<usize> {
    code.match_indices("spawn")
        .filter(|&(at, word)| {
            let before = code[.. at].trim_end();
            let reached = before.ends_with('.') || before.ends_with("::");
            let tight = !code[.. at].chars().next_back().is_some_and(is_ident_char);
            let rest = &code[at + word.len() ..];
            let ends = !rest.chars().next().is_some_and(is_ident_char);
            let rest = rest.trim_start();
            let called = rest.starts_with('(') || rest.starts_with("::<");
            reached && tight && ends && called
        })
        .map(|(at, _)| at)
        .collect()
}

// Beside the lint in a subdirectory, which the lint discovery does not read.
#[cfg(test)]
#[path = "engine_spawn_through_gate/tests.rs"]
mod tests;

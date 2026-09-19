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
//!    `let () = OnePointerClosure::<T>::FITS;` statement precedes, where
//!    `T` is the declared type of the parameter the call spawns.
//!
//! A forcing naming another type checks a closure that is never spawned,
//! and one under `#[cfg` may be compiled out, so neither counts: the
//! attribute is refused on the forcing's line and on the attribute lines
//! directly above it. A `spawn` whose argument is not one of the gate
//! fn's parameters cannot be matched to a forcing and is refused.
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
#[derive(Clone, Copy, PartialEq)]
enum Region {
    Outside,
    /// The signature has been seen and its body brace has not.
    Signature,
    /// Inside the body, at this brace depth.
    Body(usize),
}

/// What the scan knows about the gate fn it is in.
#[derive(Default)]
struct Gate {
    /// The signature text, read up to the body brace.
    signature: String,
    /// `(name, type)` for each parameter, types without whitespace.
    params:    Vec<(String, String)>,
    /// The types a compiled-in forcing has named so far in the body.
    forced:    Vec<String>,
}

fn findings(text: &str) -> Vec<Finding> {
    let mut out = Vec::new();
    let mut region = Region::Outside;
    let mut gate = Gate::default();
    let mut in_block_comment = false;
    // Whether the attribute lines directly above this one carry `#[cfg`.
    let mut cfg_above = false;

    for (idx, raw) in text.lines().enumerate() {
        let line_no = idx + 1;
        let code = code_of(raw, &mut in_block_comment);
        let allowed = raw.contains("lint:allow(engine-spawn-through-gate)");
        // Where on this line the signature text continues from, if it does.
        let mut sig_from = (region == Region::Signature).then_some(0);

        // Everything on the line that moves the region or is judged by it,
        // in the order it appears, so a one-line gate fn reads the same as a
        // multi-line one.
        let mut events: Vec<(usize, Event)> = Vec::new();
        events.extend(
            ident_offsets(&code, &format!("fn {GATE_FN}")).map(|at| (at, Event::Signature)),
        );
        events.extend(
            code.match_indices("let () =")
                .map(|(at, _)| (at, Event::Force)),
        );
        events.extend(spawn_calls(&code).into_iter().map(|at| (at, Event::Spawn)));
        for (at, c) in code.char_indices() {
            match c {
                '{' => events.push((at, Event::Open)),
                '}' => events.push((at, Event::Close)),
                _ => {},
            }
        }
        events.sort_by_key(|&(at, _)| at);

        for (at, event) in events {
            region = match (region, event) {
                (Region::Outside, Event::Signature) => {
                    gate = Gate::default();
                    sig_from = Some(at);
                    Region::Signature
                },
                (Region::Signature, Event::Open) => {
                    gate.signature.push_str(&code[sig_from.unwrap_or(0) .. at]);
                    gate.params = params_of(&gate.signature);
                    sig_from = None;
                    Region::Body(1)
                },
                (Region::Body(d), Event::Open) => Region::Body(d + 1),
                (Region::Body(1), Event::Close) => Region::Outside,
                (Region::Body(d), Event::Close) => Region::Body(d - 1),
                (Region::Body(d), Event::Force) => {
                    let compiled = !cfg_above && !code[.. at].contains("#[cfg");
                    if let (Some(ty), true) = (forced_type(&code[at ..]), compiled) {
                        gate.forced.push(ty);
                    }
                    Region::Body(d)
                },
                (r, Event::Spawn) => {
                    let message = match r {
                        Region::Body(_) => unforced(&gate, &code[at ..]),
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

        if let (Region::Signature, Some(from)) = (region, sig_from) {
            gate.signature.push_str(&code[from ..]);
            gate.signature.push(' ');
        }
        let trimmed = code.trim();
        if trimmed.starts_with("#[") && trimmed.ends_with(']') {
            cfg_above |= trimmed.contains("#[cfg");
        } else if !trimmed.is_empty() {
            cfg_above = false;
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

/// Why a `spawn` inside the gate body is not gated, or `None` when it is.
fn unforced(gate: &Gate, call: &str) -> Option<String> {
    let arg = spawn_argument(call);
    let ty = arg.and_then(|a| gate.params.iter().find(|(name, _)| name == a));
    match (arg, ty) {
        (_, Some((_, ty))) if gate.forced.contains(ty) => None,
        (Some(arg), Some((_, ty))) => {
            Some(format!(
                "`{GATE_FN}` spawns `{arg}` before `let () = OnePointerClosure::<{ty}>::FITS;`, compiled \
                 in with no `#[cfg`, forces the one-pointer gate on its type; put that statement first, or \
                 a wider closure reaches the executor unchecked"
            ))
        },
        _ => {
            Some(format!(
                "`{GATE_FN}` spawns something other than one of its parameters, so no forcing can be \
                 checked against it; spawn the parameter whose type `OnePointerClosure::<..>::FITS` names"
            ))
        },
    }
}

/// The argument of a `spawn(x)` call when it is a single identifier.
fn spawn_argument(call: &str) -> Option<&str> {
    let mut rest = call["spawn".len() ..].trim_start();
    if let Some(turbofish) = rest.strip_prefix("::<") {
        rest = turbofish[turbofish.find('>')? + 1 ..].trim_start();
    }
    let inner = rest.strip_prefix('(')?;
    let arg = inner[.. inner.find(')')?].trim();
    (!arg.is_empty() && arg.chars().all(is_ident_char)).then_some(arg)
}

/// The type a `let () = OnePointerClosure::<T>::FITS;` statement names.
fn forced_type(from_let: &str) -> Option<String> {
    let stmt = &from_let[.. from_let.find(';').unwrap_or(from_let.len())];
    let rhs = without_whitespace(&stmt["let () =".len() ..]);
    let ty = rhs
        .strip_prefix("OnePointerClosure::<")?
        .strip_suffix(">::FITS")?;
    (!ty.is_empty()).then(|| ty.to_string())
}

fn without_whitespace(text: &str) -> String {
    text.chars().filter(|c| !c.is_whitespace()).collect()
}

/// `(name, type)` for each parameter of a signature, types without whitespace.
fn params_of(signature: &str) -> Vec<(String, String)> {
    // The parameter list opens at the first `(` outside the generics.
    let mut angle = 0usize;
    let mut prev = ' ';
    let mut open = None;
    for (i, c) in signature.char_indices() {
        match c {
            '<' => angle += 1,
            '>' if prev != '-' => angle = angle.saturating_sub(1),
            '(' if angle == 0 => {
                open = Some(i + 1);
                break;
            },
            _ => {},
        }
        prev = c;
    }
    let Some(open) = open else {
        return Vec::new();
    };
    let mut params = Vec::new();
    let (mut depth, mut start, mut prev) = (0usize, open, ' ');
    for (i, c) in signature[open ..]
        .char_indices()
        .map(|(i, c)| (i + open, c))
    {
        match c {
            '>' if prev == '-' => {},
            '(' | '[' | '<' => depth += 1,
            ')' if depth == 0 => {
                params.extend(param(&signature[start .. i]));
                break;
            },
            ')' | ']' | '>' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                params.extend(param(&signature[start .. i]));
                start = i + 1;
            },
            _ => {},
        }
        prev = c;
    }
    params
}

/// One `name: Type` parameter; `self` forms and patterns give `None`.
fn param(text: &str) -> Option<(String, String)> {
    let (name, ty) = text.split_once(':')?;
    let name = name.trim().trim_start_matches("mut ").trim();
    let valid = !name.is_empty() && name.chars().all(is_ident_char);
    valid.then(|| (name.to_string(), without_whitespace(ty)))
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

//! Lint: a comment in a crate's source says what the code is, never how it
//! was decided.
//!
//! Doc comments become the published rustdoc, and comments are read beside
//! the code for as long as it lives. Neither names the process that produced
//! the code: a design round by number or stamp, a topic, a changelist, an
//! audit finding or a megaround. The lint reads the comment text of every
//! file under a crate's `src/` and reports the first such word on a line.
//!
//! `round` is also arithmetic. It counts as process unless the word after it
//! makes it arithmetic (`round up`, `round to`, `round trip`) or a verb taking
//! an object (`round a count up`, `rounds the value`). `round-2b` counts, as
//! `audit-2` does; `round-trip` and `rounding` are other words and pass.

use mockspace::{CrateLint, Lint, LintContext, LintError, Severity};

pub fn lint() -> Box<dyn CrateLint> {
    Box::new(NoProcessWordsInComments)
}

struct NoProcessWordsInComments;

const NAME: &str = "no-process-words-in-comments";

/// Words that name the process wherever they stand.
const ALWAYS: [&str; 6] =
    ["megaround", "megarounds", "changelist", "changelists", "topic", "topics"];

/// Words after `round` that make it arithmetic.
const ARITHMETIC_AFTER_ROUND: [&str; 9] =
    ["trip", "robin", "up", "down", "to", "toward", "towards", "off", "half"];

/// Words after `round` that make it a verb taking an object.
const OBJECT_AFTER_VERB: [&str; 10] =
    ["a", "an", "the", "it", "its", "each", "every", "this", "that", "them"];
impl Lint for NoProcessWordsInComments {
    fn name(&self) -> &'static str {
        NAME
    }

    fn description(&self) -> &'static str {
        "a comment says what the code is, and names no round, topic, changelist or audit"
    }

    fn default_severity(&self) -> Severity {
        Severity::ADVISORY
    }

    fn per_file(&self) -> bool {
        false
    }
}

impl CrateLint for NoProcessWordsInComments {
    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        if !ctx.crate_name.starts_with("hilavitkutin") {
            return Vec::new();
        }
        let mut errors = Vec::new();
        for file in ctx.all_sources() {
            let path = file.rel_path.to_string_lossy().into_owned();
            for (line, word) in findings(file.text) {
                let mut e = LintError::with_severity(
                    ctx.crate_name.to_string(),
                    line,
                    NAME,
                    format!(
                        "comment names the process (`{word}`); say what the code is or does, in its own terms"
                    ),
                    Severity::ADVISORY,
                );
                e.path = Some(path.clone());
                errors.push(e);
            }
        }
        errors
    }
}

/// Each 1-based line of `text` whose comment names the process, with the
/// first word that does.
fn findings(text: &str) -> Vec<(usize, String)> {
    let mut depth = 0;
    text.lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            let comment = comment_of(line, &mut depth)?;
            process_word(&comment).map(|word| (idx + 1, word))
        })
        .collect()
}

/// The comment text of a line, or `None` where the line holds no comment.
///
/// Reads the line once, left to right, the way the lexer does: a `//`
/// outside a string or char literal takes the rest of the line, a `/*` opens
/// a block and a `*/` closes it, and blocks nest. `depth` carries the open
/// block count across lines, so a block opened after code, a line inside a
/// block, and code after a block's close are each read for what they are.
fn comment_of(line: &str, depth: &mut usize) -> Option<String> {
    let bytes = line.as_bytes();
    let mut text = String::new();
    let mut seen = *depth > 0;
    let mut in_str = false;
    let mut at = 0;
    while at < bytes.len() {
        let pair = (bytes[at], bytes.get(at + 1).copied());
        if *depth > 0 {
            match pair {
                (b'/', Some(b'*')) => {
                    *depth += 1;
                    at += 2;
                },
                (b'*', Some(b'/')) => {
                    *depth -= 1;
                    text.push(' ');
                    at += 2;
                },
                _ => at += push_char(&mut text, line, at),
            }
            continue;
        }
        match pair {
            (b'\\', _) if in_str => at += 1,
            (b'"', _) => in_str = !in_str,
            (b'\'', _) if !in_str => {
                // A char literal: `'x'` or `'\x'`. A lifetime has no closing quote.
                let width = if pair.1 == Some(b'\\') { 3 } else { 2 };
                if bytes.get(at + width) == Some(&b'\'') {
                    at += width;
                }
            },
            (b'/', Some(b'/')) if !in_str => {
                text.push_str(&line[at + 2 ..]);
                return Some(text);
            },
            (b'/', Some(b'*')) if !in_str => {
                *depth += 1;
                seen = true;
                at += 2;
                continue;
            },
            _ => {},
        }
        at += 1;
    }
    seen.then_some(text)
}

/// Push the char of `line` starting at byte `at` onto `text`, and return its
/// width in bytes.
fn push_char(text: &mut String, line: &str, at: usize) -> usize {
    let c = line[at ..].chars().next().unwrap_or(' ');
    text.push(c);
    c.len_utf8()
}

/// Whether `round` at `words[i]` names a design round. It does unless the
/// word after it makes it arithmetic (`round up`, `round trip`) or a verb
/// taking an object (`round a count up`, `rounds the value`).
fn names_a_round(i: usize, words: &[String]) -> bool {
    !words.get(i + 1).is_some_and(|n| {
        ARITHMETIC_AFTER_ROUND.contains(&n.as_str()) || OBJECT_AFTER_VERB.contains(&n.as_str())
    })
}

/// Whether `word` is a numbered form `<prefix><digit>...`, as `audit-2` or
/// `round-2b`.
fn numbered(word: &str, prefix: &str) -> bool {
    word.strip_prefix(prefix)
        .is_some_and(|n| n.starts_with(|c: char| c.is_ascii_digit()))
}

/// The first word of `comment` that names the process.
fn process_word(comment: &str) -> Option<String> {
    let words: Vec<String> = comment
        .split(|c: char| !(c.is_alphanumeric() || c == '-'))
        .filter(|w| !w.is_empty())
        .map(|w| w.trim_matches('-').to_lowercase())
        .collect();
    for (i, word) in words.iter().enumerate() {
        if ALWAYS.contains(&word.as_str()) || numbered(word, "audit-") || numbered(word, "round-") {
            return Some(word.clone());
        }
        if word == "cl" && i > 0 && matches!(words[i - 1].as_str(), "src" | "doc") {
            return Some(format!("{} CL", words[i - 1]));
        }
        if (word == "round" || word == "rounds") && names_a_round(i, &words) {
            return Some(word.clone());
        }
    }
    None
}

// Beside the lint in a subdirectory, which the lint discovery does not read.
#[cfg(test)]
#[path = "no_process_words_in_comments/tests.rs"]
mod tests;

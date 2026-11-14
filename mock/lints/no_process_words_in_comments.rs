//! Lint: a comment in a crate's source says what the code is, never how it
//! was decided.
//!
//! Doc comments become the published rustdoc, and comments are read beside
//! the code for as long as it lives. Neither names the process that produced
//! the code: a design round by number or stamp, a topic, a changelist, an
//! audit finding or a megaround. The lint reads the comment text of every
//! file under a crate's `src/` and reports the first such word on a line.
//!
//! `round` is also arithmetic. It counts as process only as the noun: followed
//! by a number or stamp (`round 4`), or led by a word that makes it one (`this
//! round`, `a later round`). The verb (`round a count up`, `rounds the value`),
//! the arithmetic phrases (`round up`, `round to`, `round trip`), `round-trip`
//! and `rounding` pass.

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

/// Words before `round` that make it the noun.
const NOUN_BEFORE_ROUND: [&str; 22] = [
    "this", "that", "the", "a", "an", "last", "next", "previous", "earlier", "later", "same",
    "design", "each", "every", "one", "its", "which", "prior", "first", "final", "two", "these",
];

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
    let mut in_block = false;
    text.lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            let comment = comment_of(line, &mut in_block)?;
            process_word(comment).map(|word| (idx + 1, word))
        })
        .collect()
}

/// The comment text of a line: a line inside a `/* */` block, after a
/// leading `//` (`///` and `//!` included), after a `/*` that opens a block,
/// or after a `//` that follows code outside a string or char literal.
/// `in_block` carries whether a block comment is open across lines.
fn comment_of<'a>(line: &'a str, in_block: &mut bool) -> Option<&'a str> {
    if *in_block {
        if line.contains("*/") {
            *in_block = false;
        }
        return Some(line);
    }
    let trimmed = line.trim_start();
    if let Some(rest) = trimmed.strip_prefix("//") {
        return Some(rest);
    }
    if let Some(rest) = trimmed.strip_prefix("/*") {
        *in_block = !rest.contains("*/");
        return Some(rest);
    }
    trailing_comment(line)
}

/// The text after a `//` that follows code, outside any string or char
/// literal.
fn trailing_comment(line: &str) -> Option<&str> {
    let bytes = line.as_bytes();
    let mut in_str = false;
    let mut at = 0;
    while at < bytes.len() {
        match bytes[at] {
            b'\\' if in_str => at += 1,
            b'"' => in_str = !in_str,
            b'\'' if !in_str => {
                // A char literal: `'x'` or `'\x'`. A lifetime has no closing quote.
                let width = if bytes.get(at + 1) == Some(&b'\\') { 3 } else { 2 };
                if bytes.get(at + width) == Some(&b'\'') {
                    at += width;
                }
            },
            b'/' if !in_str && bytes.get(at + 1) == Some(&b'/') => {
                return Some(&line[at + 2 ..]);
            },
            _ => {},
        }
        at += 1;
    }
    None
}

/// Whether `round` at `words[i]` is the noun naming a design round: followed
/// by its number or stamp, or led by a word that makes it a noun. A verb
/// (`round a count up`, `rounds the value`) and the arithmetic phrases pass.
fn names_a_round(i: usize, words: &[String]) -> bool {
    let next = words.get(i + 1).map(String::as_str);
    if next.is_some_and(|n| ARITHMETIC_AFTER_ROUND.contains(&n) || OBJECT_AFTER_VERB.contains(&n)) {
        return false;
    }
    let numbered = next.is_some_and(|n| n.starts_with(|c: char| c.is_ascii_digit()));
    let led = i > 0 && NOUN_BEFORE_ROUND.contains(&words[i - 1].as_str());
    numbered || led
}

/// The first word of `comment` that names the process.
fn process_word(comment: &str) -> Option<String> {
    let words: Vec<String> = comment
        .split(|c: char| !(c.is_alphanumeric() || c == '-'))
        .filter(|w| !w.is_empty())
        .map(|w| w.trim_matches('-').to_lowercase())
        .collect();
    for (i, word) in words.iter().enumerate() {
        let next = words.get(i + 1).map(String::as_str);
        if ALWAYS.contains(&word.as_str()) {
            return Some(word.clone());
        }
        if let Some(n) = word.strip_prefix("audit-")
            && n.starts_with(|c: char| c.is_ascii_digit())
        {
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

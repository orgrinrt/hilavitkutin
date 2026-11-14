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
//! makes it arithmetic (`round up`, `round to`, `round trip`), or it is
//! written `round-trip` or `rounding`, which are other words.

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
    text.lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            let comment = comment_of(line)?;
            process_word(comment).map(|word| (idx + 1, word))
        })
        .collect()
}

/// The comment text of a line: after a leading `//` (`///` and `//!`
/// included), after a `//` that follows code outside a string, or the body
/// of a line inside a `/* */` doc block written with a leading `*`.
fn comment_of(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    if let Some(rest) = trimmed.strip_prefix("//") {
        return Some(rest);
    }
    if let Some(rest) = trimmed.strip_prefix("/*") {
        return Some(rest);
    }
    if let Some(rest) = trimmed.strip_prefix("* ") {
        return Some(rest);
    }
    let mut in_str = false;
    let mut prev = '\0';
    for (at, c) in line.char_indices() {
        match c {
            '"' if prev != '\\' => in_str = !in_str,
            '/' if !in_str && prev == '/' => return Some(&line[at + 1 ..]),
            _ => {},
        }
        prev = c;
    }
    None
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
        if (word == "round" || word == "rounds")
            && !next.is_some_and(|n| ARITHMETIC_AFTER_ROUND.contains(&n))
        {
            return Some(word.clone());
        }
    }
    None
}

// Beside the lint in a subdirectory, which the lint discovery does not read.
#[cfg(test)]
#[path = "no_process_words_in_comments/tests.rs"]
mod tests;

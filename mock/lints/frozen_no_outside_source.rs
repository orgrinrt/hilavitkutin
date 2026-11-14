//! Lint: a frozen file brings in no file from outside the ledger.
//!
//! The ledger has a line for every file of a frozen crate, and a file
//! compiled into the crate is a file of the crate wherever it lies. A `path`
//! attribute on a module and the `include!` macros both compile in a file
//! from wherever they point, the unfrozen `tests/` included, and the ledger
//! lint reads the crate's directory and never follows them. The lint refuses
//! both in the `src/` of the frozen crates.
//!
//! A macro is refused by its name as a whole word, with or without the `!`,
//! since a macro can be imported under another name and called by that.
//!
//! The rest of what the frozen lints share is in `frozen/scan.rs`.

use mockspace::{CrateLint, Lint, LintContext, LintError, Severity};

#[path = "frozen/scan.rs"]
mod scan;
use scan::{Finding, attributes, is_ident_char, words};

pub fn lint() -> Box<dyn CrateLint> {
    Box::new(FrozenNoOutsideSource)
}

struct FrozenNoOutsideSource;

const NAME: &str = "frozen-no-outside-source";

/// The macros that compile a file in, or its content.
const INCLUDES: [&str; 3] = ["include", "include_str", "include_bytes"];

impl Lint for FrozenNoOutsideSource {
    fn name(&self) -> &'static str {
        NAME
    }

    fn description(&self) -> &'static str {
        "a frozen file brings in no file from outside the ledger"
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

impl CrateLint for FrozenNoOutsideSource {
    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        scan::check(ctx, NAME, findings)
    }
}

fn findings(text: &str) -> Vec<Finding> {
    let paths = attributes(text)
        .into_iter()
        .filter(|(_, attribute)| names_a_path(attribute))
        .map(|(line, _)| Finding {
            line,
            message: "a frozen file carries no `path` attribute; a module of a frozen crate is a file under its `src/` with a line in the ledger".into(),
        });
    let includes = words(text, INCLUDES).into_iter().map(|(line, word)| Finding {
        line,
        message: format!(
            "a frozen file names no `{word}!`; what it compiles is written in a file with a line in the ledger"
        ),
    });
    let mut found: Vec<Finding> = paths.chain(includes).collect();
    found.sort_by_key(|finding| finding.line);
    found
}

/// Whether an attribute, with its whitespace removed, sets the key `path`,
/// bare or under `cfg_attr`.
fn names_a_path(attribute: &str) -> bool {
    attribute.match_indices("path=").any(|(at, _)| {
        !attribute[.. at]
            .chars()
            .next_back()
            .is_some_and(is_ident_char)
    })
}

// Beside the lint in a subdirectory, which the lint discovery does not read.
#[cfg(test)]
#[path = "frozen_no_outside_source/tests.rs"]
mod tests;

//! What the three frozen lints share: which crates they read, how a file is
//! walked as code, and how a finding becomes a `LintError`.
//!
//! A frozen crate is written as a consumer would write it, from the contracts
//! alone. Each lint names one way a file could hide what the contracts make
//! its author or implementor do, and refuses it in the `src/` of
//! `hilavitkutin-frozen-authors` and `hilavitkutin-frozen-implementors`.
//!
//! Comments, doc comments and string literals are not read. The escape is
//! `// lint:allow(<lint name>)` on the line, with a reason.

use mockspace::{LintContext, LintError, Severity};

#[path = "../scan/code.rs"]
mod code;
pub(crate) use code::is_ident_char;

/// The crates the frozen lints read.
pub(crate) fn applies_to(crate_name: &str) -> bool {
    crate_name.starts_with("hilavitkutin-frozen-")
}

/// One refused spot: the 1-based line and what is wrong there.
#[derive(Debug, PartialEq)]
pub(crate) struct Finding {
    pub(crate) line:    usize,
    pub(crate) message: String,
}

/// Each line of `text` as code, with its 1-based number, skipping lines that
/// carry the escape for `name`.
pub(crate) fn code_lines<'t>(
    text: &'t str,
    name: &'t str,
) -> impl Iterator<Item = (usize, String)> + 't {
    let mut in_block_comment = false;
    let allow = format!("lint:allow({name})");
    text.lines().enumerate().filter_map(move |(idx, raw)| {
        let code = code::code_of(raw, &mut in_block_comment);
        (!raw.contains(&allow)).then_some((idx + 1, code))
    })
}

/// The code with every whitespace character removed, so `# ! [ feature` and
/// `#![feature` read the same.
pub(crate) fn compact(code: &str) -> String {
    code.chars().filter(|c| !c.is_whitespace()).collect()
}

/// Every finding `find` reports in the crate's sources, as hard errors.
pub(crate) fn check(
    ctx: &LintContext,
    name: &'static str,
    find: fn(&str) -> Vec<Finding>,
) -> Vec<LintError> {
    if !applies_to(ctx.crate_name) {
        return Vec::new();
    }
    let mut errors = Vec::new();
    for file in ctx.all_sources {
        let path = file.rel_path.to_string_lossy().into_owned();
        for finding in find(&file.text) {
            let mut e = LintError::with_severity(
                ctx.crate_name.to_string(),
                finding.line,
                name,
                finding.message,
                Severity::HARD_ERROR,
            );
            e.path = Some(path.clone());
            errors.push(e);
        }
    }
    errors
}

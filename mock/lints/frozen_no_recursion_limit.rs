//! Lint: a frozen crate raises no recursion limit.
//!
//! The contracts promise that an author's kernel list and an implementor's
//! store compile at the default limits. A `recursion_limit` attribute in a
//! frozen crate would let a shape pass there that a consumer would hit the
//! default limit with, so the law would hold over something the contracts do
//! not deliver. The lint refuses the attribute in the `src/` of the frozen
//! crates, the type-length limit's sibling included.
//!
//! The rest of what the frozen lints share is in `frozen/scan.rs`.

use mockspace::{CrateLint, Lint, LintContext, LintError, Severity};

#[path = "frozen/scan.rs"]
mod scan;
use scan::{Finding, code_lines, compact, is_ident_char};

pub fn lint() -> Box<dyn CrateLint> {
    Box::new(FrozenNoRecursionLimit)
}

struct FrozenNoRecursionLimit;

const NAME: &str = "frozen-no-recursion-limit";

/// The crate attributes that raise a limit the compiler holds a consumer to.
const LIMITS: [&str; 2] = ["recursion_limit", "type_length_limit"];

impl Lint for FrozenNoRecursionLimit {
    fn name(&self) -> &'static str {
        NAME
    }

    fn description(&self) -> &'static str {
        "a frozen crate raises no recursion limit"
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }

    fn per_file(&self) -> bool {
        false
    }
}

impl CrateLint for FrozenNoRecursionLimit {
    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        scan::check(ctx, NAME, findings)
    }
}

fn findings(text: &str) -> Vec<Finding> {
    code_lines(text)
        .filter_map(|(line, code)| {
            let code = compact(&code);
            let named = LIMITS.into_iter().find(|limit| names(&code, limit))?;
            Some(Finding {
                line,
                message: format!(
                    "a frozen crate raises no `{named}`; the contracts hold at the default limits"
                ),
            })
        })
        .collect()
}

/// Whether a compacted line has an attribute naming `limit` as a whole word.
fn names(code: &str, limit: &str) -> bool {
    (code.contains("#[") || code.contains("#!["))
        && code.match_indices(limit).any(|(at, _)| {
            let before = code[.. at].chars().next_back();
            let after = code[at + limit.len() ..].chars().next();
            !before.is_some_and(is_ident_char) && !after.is_some_and(is_ident_char)
        })
}

// Beside the lint in a subdirectory, which the lint discovery does not read.
#[cfg(test)]
#[path = "frozen_no_recursion_limit/tests.rs"]
mod tests;

//! Lint: a frozen crate enables no unstable feature.
//!
//! A frozen crate is written as a consumer would write it, against the
//! contracts on the toolchain they ship for. A `#![feature(..)]` in one would
//! let it reach past what the contracts give, and a law that holds over it
//! would then hold over something no consumer can write. The lint refuses an
//! inner `feature` attribute in the `src/` of the frozen crates, bare or
//! under `cfg_attr`.
//!
//! The rest of what the frozen lints share is in `frozen/scan.rs`.

use mockspace::{CrateLint, Lint, LintContext, LintError, Severity};

#[path = "frozen/scan.rs"]
mod scan;
use scan::{Finding, code_lines, compact, is_ident_char};

pub fn lint() -> Box<dyn CrateLint> {
    Box::new(FrozenNoFeatureGate)
}

struct FrozenNoFeatureGate;

const NAME: &str = "frozen-no-feature-gate";

impl Lint for FrozenNoFeatureGate {
    fn name(&self) -> &'static str {
        NAME
    }

    fn description(&self) -> &'static str {
        "a frozen crate enables no unstable feature"
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }

    fn per_file(&self) -> bool {
        false
    }
}

impl CrateLint for FrozenNoFeatureGate {
    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        scan::check(ctx, NAME, findings)
    }
}

fn findings(text: &str) -> Vec<Finding> {
    code_lines(text, NAME)
        .filter(|(_, code)| enables_a_feature(&compact(code)))
        .map(|(line, _)| Finding {
            line,
            message: "a frozen crate enables no unstable feature; write it against the contracts on the toolchain a consumer has".into(),
        })
        .collect()
}

/// Whether a compacted line opens an inner attribute that names `feature(`.
fn enables_a_feature(code: &str) -> bool {
    let Some(at) = code.find("#![") else {
        return false;
    };
    let attr = &code[at + 3 ..];
    let names_feature = attr
        .match_indices("feature(")
        .any(|(i, _)| !attr[.. i].chars().next_back().is_some_and(is_ident_char));
    attr.starts_with("feature(") || (attr.starts_with("cfg_attr(") && names_feature)
}

// Beside the lint in a subdirectory, which the lint discovery does not read.
#[cfg(test)]
#[path = "frozen_no_feature_gate/tests.rs"]
mod tests;

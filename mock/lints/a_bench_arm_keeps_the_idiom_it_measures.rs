//! Lint: a bench arm that measures a hand-written idiom still holds it.
//!
//! `rot_manual_shifts` times the shift-or rotate `(v << k) | (v >> (64 - k))`
//! and `sat_checked` the checked add matched into `u64::MAX`. Each is what
//! clippy's autofix rewrites into the intrinsic a sibling arm already times
//! (`rotate_left`, `saturating_add`), and the port of the bench tree did
//! exactly that to both. The arm then builds, validates and reports under its
//! own name while measuring its sibling a second time, so nothing but reading
//! the source shows it.
//!
//! The lint reads each listed arm's `src/lib.rs` as code, with comments and
//! string contents blanked and whitespace dropped, and refuses an arm that has
//! lost one of its idiom's pieces or holds one of the rewrites. A listed arm
//! that is not there is refused too, so a rename cannot retire the check.

use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

#[path = "scan/code.rs"]
mod code;
use code::code_of;

pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(ABenchArmKeepsTheIdiomItMeasures)
}

struct ABenchArmKeepsTheIdiomItMeasures;

const NAME: &str = "a-bench-arm-keeps-the-idiom-it-measures";

/// One arm, the idiom it measures and the rewrites that would replace it.
pub(crate) struct Idiom {
    /// The arm's source, relative to the mock directory.
    pub(crate) path:     &'static str,
    /// What the code holds, each piece written without whitespace.
    pub(crate) must:     &'static [&'static str],
    /// What the code must not hold, each piece written without whitespace.
    pub(crate) must_not: &'static [&'static str],
    /// The idiom as a reader would write it, for the message.
    pub(crate) idiom:    &'static str,
}

pub(crate) const IDIOMS: &[Idiom] = &[
    Idiom {
        path:     "benches/rotate_strategy/arms/rot_manual_shifts/src/lib.rs",
        must:     &["(v<<k)|(v>>(64-k))"],
        must_not: &[".rotate_left(", ".rotate_right("],
        idiom:    "`(v << k) | (v >> (64 - k))`",
    },
    Idiom {
        path:     "benches/sat_arithmetic/arms/sat_checked/src/lib.rs",
        must:     &["matchacc.checked_add(term){", "Some(v)=>v,", "None=>u64::MAX"],
        must_not: &[".saturating_add(", ".unwrap_or("],
        idiom:    "`match acc.checked_add(term) { Some(v) => v, None => u64::MAX }`",
    },
];

impl Lint for ABenchArmKeepsTheIdiomItMeasures {
    fn name(&self) -> &'static str {
        NAME
    }

    fn description(&self) -> &'static str {
        "a bench arm measuring a hand-written idiom holds it, not the intrinsic a sibling arm times"
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}

impl RepoLint for ABenchArmKeepsTheIdiomItMeasures {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let view = ctx.view();
        IDIOMS
            .iter()
            .filter_map(|idiom| {
                let path = ctx.mock_dir.join(idiom.path);
                let message = match view.text(&path) {
                    Some(text) => finding(text, idiom)?,
                    None => {
                        format!(
                            "`mock/{}` is gone; the arm measuring {} moved or was renamed, so \
                         update its path in this lint",
                            idiom.path, idiom.idiom
                        )
                    },
                };
                Some(error(idiom.path, message))
            })
            .collect()
    }
}

fn error(path: &str, message: String) -> LintError {
    let mut e = LintError::with_severity("mock".into(), 1, NAME, message, Severity::HARD_ERROR);
    e.path = Some(path.to_string());
    e
}

/// The source read as code, with every whitespace character removed.
pub(crate) fn compact_code(text: &str) -> String {
    let mut in_block_comment = false;
    text.lines()
        .map(|line| code_of(line, &mut in_block_comment))
        .flat_map(|code| {
            code.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<Vec<_>>()
        })
        .collect()
}

/// What is wrong with `text` as the arm holding `idiom`, if anything.
pub(crate) fn finding(text: &str, idiom: &Idiom) -> Option<String> {
    let code = compact_code(text);
    if let Some(rewrite) = idiom.must_not.iter().find(|m| code.contains(*m)) {
        return Some(format!(
            "`mock/{}` calls `{rewrite}`, the intrinsic a sibling arm times; this arm measures \
             {}, so write that back and keep its clippy allow",
            idiom.path, idiom.idiom
        ));
    }
    if idiom.must.iter().all(|m| code.contains(*m)) {
        return None;
    }
    Some(format!(
        "`mock/{}` no longer holds {}, the idiom this arm measures; write it back",
        idiom.path, idiom.idiom
    ))
}

// Beside the lint in a subdirectory, which the lint discovery does not read.
#[cfg(test)]
#[path = "a_bench_arm_keeps_the_idiom_it_measures/tests.rs"]
mod tests;

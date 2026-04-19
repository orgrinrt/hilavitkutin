//! Lint: no dead vocabulary terms in `hilavitkutin*` sources.
//!
//! The engine's vocabulary is fixed (DESIGN.md "Vocabulary" section):
//! `pipeline, core, phase, waist, trunk, fiber, branch, bridge,
//! morsel, micro-morsel, entry, record`. The following terms are
//! DEAD — they map onto canonical terms and must not be introduced:
//!
//! | Dead term    | Use instead           |
//! |--------------|-----------------------|
//! | chain        | fiber                 |
//! | chain_group  | trunk                 |
//! | partition    | phase                 |
//! | archetype    | fiber                 |
//! | entity       | record                |
//! | row          | record                |
//! | order        | scheduling hints      |
//!
//! This lint is ADVISORY, not HARD_ERROR, because `row` / `order`
//! / `entity` are common English words and there can be legitimate
//! uses in string literals, doc comments discussing prior art, etc.
//! Rule of thumb: warn, read the warning, decide whether to rename
//! or suppress with `// lint:allow(vocabulary-discipline) -- <reason>`.

use mockspace::{Lint, LintContext, LintError, Severity};

pub fn lint() -> Box<dyn Lint> {
    Box::new(VocabularyDiscipline)
}

struct VocabularyDiscipline;

const DEAD_TERMS: &[(&str, &str)] = &[
    ("chain_group", "use `trunk` (the canonical term for a group of fibers)"),
    ("chain", "use `fiber` (the canonical term for a single ordered sequence of WUs)"),
    ("partition", "use `phase` (the canonical term for a waist-bounded stage)"),
    ("archetype", "use `fiber` (records are columnar; fibers are the canonical grouping, not archetypes)"),
    ("entity", "use `record` (a single point in a column, not an ECS-style entity)"),
    ("Entity", "use `Record` (a single point in a column)"),
    // `row` and `order` are extremely common; only flag in type/field contexts.
];

impl Lint for VocabularyDiscipline {
    fn name(&self) -> &'static str {
        "vocabulary-discipline"
    }

    fn default_severity(&self) -> Severity {
        Severity::ADVISORY
    }

    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        if ctx.is_proc_macro_crate() {
            return Vec::new();
        }
        if !ctx.crate_name.starts_with("hilavitkutin") {
            return Vec::new();
        }

        let mut errors = Vec::new();
        for (line_idx, line) in ctx.source.lines().enumerate() {
            let trimmed = line.trim_start();
            if trimmed.contains("lint:allow(vocabulary-discipline)") {
                continue;
            }

            for (needle, remedy) in DEAD_TERMS {
                // Only match as a word-ish token; avoid firing on substrings like
                // `orderly` for `order`. Simple boundary check: previous and next
                // char (if any) must not be alphanumeric or underscore.
                if let Some(idx) = line.find(needle) {
                    let prev_ok = idx == 0 || {
                        let c = line.as_bytes()[idx - 1] as char;
                        !(c.is_alphanumeric() || c == '_')
                    };
                    let end = idx + needle.len();
                    let next_ok = end >= line.len() || {
                        let c = line.as_bytes()[end] as char;
                        !(c.is_alphanumeric() || c == '_')
                    };
                    if !(prev_ok && next_ok) {
                        continue;
                    }

                    errors.push(LintError::with_severity(
                        ctx.crate_name.to_string(),
                        line_idx + 1,
                        "vocabulary-discipline",
                        format!(
                            "dead vocabulary term `{}` — {}: {}",
                            needle,
                            remedy,
                            trimmed.trim(),
                        ),
                        Severity::ADVISORY,
                    ));
                    break;
                }
            }
        }

        errors
    }
}

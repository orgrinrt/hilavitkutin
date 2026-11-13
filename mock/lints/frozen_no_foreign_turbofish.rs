//! Lint: a frozen crate turbofishes only what its own file declares.
//!
//! The contracts promise that their generic functions and types are reached
//! with every parameter inferred from what the caller already holds. A `::<`
//! in a frozen crate on an item it did not write, such as
//! `bind::<Plane, Kernels, Store>` or `<K as ValueKernel>::at::<..>`, is the
//! caller telling the compiler what the contract should have let it work out,
//! and a law holding over that code would hold over a call a consumer cannot
//! make. The lint refuses a `::<` whose item, the path segment right before
//! it, is not declared in the same file, in the `src/` of the frozen crates.
//!
//! A declaration is an item keyword (`fn`, `struct`, `enum`, `union`,
//! `trait`, `type`, `const`, `static`, `mod`) or `macro_rules!` followed by
//! its name. A turbofish that spans lines is not read.
//!
//! The rest of what the frozen lints share is in `frozen/scan.rs`.

use std::collections::BTreeSet;

use mockspace::{CrateLint, Lint, LintContext, LintError, Severity};

#[path = "frozen/scan.rs"]
mod scan;
use scan::{Finding, code_lines, compact, is_ident_char};

pub fn lint() -> Box<dyn CrateLint> {
    Box::new(FrozenNoForeignTurbofish)
}

struct FrozenNoForeignTurbofish;

const NAME: &str = "frozen-no-foreign-turbofish";

/// The keywords whose next identifier is the item they declare.
const DECLARES: [&str; 9] =
    ["fn", "struct", "enum", "union", "trait", "type", "const", "static", "mod"];

impl Lint for FrozenNoForeignTurbofish {
    fn name(&self) -> &'static str {
        NAME
    }

    fn description(&self) -> &'static str {
        "a frozen crate turbofishes only items its own file declares"
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }

    fn per_file(&self) -> bool {
        false
    }
}

impl CrateLint for FrozenNoForeignTurbofish {
    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        scan::check(ctx, NAME, findings)
    }
}

fn findings(text: &str) -> Vec<Finding> {
    let declared = declared(text);
    let mut out = Vec::new();
    for (line, code) in code_lines(text) {
        let code = compact(&code);
        for (at, _) in code.match_indices("::<") {
            let item = item_before(&code[.. at]);
            if !declared.contains(item) {
                let shown = if item.is_empty() { "a qualified path" } else { item };
                out.push(Finding {
                    line,
                    message: format!(
                        "`{shown}::<` names an item this file did not declare; a frozen crate reaches the contracts with every parameter inferred"
                    ),
                });
            }
        }
    }
    out
}

/// The names the file's item keywords declare.
fn declared(text: &str) -> BTreeSet<String> {
    let mut words = Vec::new();
    for (_, code) in code_lines(text) {
        let mut rest = code.as_str();
        while !rest.is_empty() {
            let start = rest
                .find(|c: char| is_ident_char(c) || c == '!')
                .unwrap_or(rest.len());
            rest = &rest[start ..];
            let end = rest
                .find(|c: char| !is_ident_char(c) && c != '!')
                .unwrap_or(rest.len());
            if end > 0 {
                words.push(rest[.. end].to_string());
            }
            rest = &rest[end ..];
        }
    }
    let mut names = BTreeSet::new();
    for pair in words.windows(2) {
        let declares = DECLARES.contains(&pair[0].as_str()) || pair[0] == "macro_rules!";
        let named = !DECLARES.contains(&pair[1].as_str()) && pair[1] != "_";
        if declares && named {
            names.insert(pair[1].trim_end_matches('!').to_string());
        }
    }
    names
}

/// The identifier that ends `code`, which is the item a `::<` after it names.
fn item_before(code: &str) -> &str {
    let start = code
        .char_indices()
        .rev()
        .take_while(|&(_, c)| is_ident_char(c))
        .last()
        .map_or(code.len(), |(i, _)| i);
    &code[start ..]
}

// Beside the lint in a subdirectory, which the lint discovery does not read.
#[cfg(test)]
#[path = "frozen_no_foreign_turbofish/tests.rs"]
mod tests;

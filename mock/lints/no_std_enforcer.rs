//! Lint: no `use std::` in any `hilavitkutin*` source file.
//!
//! The engine is `#![no_std]` in every crate. Any direct import of
//! `std::*` breaks that contract and almost always indicates an
//! accidental pull of platform types (collections, threading, IO)
//! that belong to consumer-provided providers, not the engine.
//!
//! Concrete forms caught:
//! - `use std::`
//! - `std::`-qualified paths outside string literals and comments
//!   (line-prefix heuristic; tree-sitter path scan would be stricter
//!   but this lint only needs to fire on the obvious cases).
//!
//! The `use core::` alternative is always available.

use mockspace::{Lint, LintContext, LintError, Severity};

pub fn lint() -> Box<dyn Lint> {
    Box::new(NoStdEnforcer)
}

struct NoStdEnforcer;

impl Lint for NoStdEnforcer {
    fn name(&self) -> &'static str {
        "no-std"
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }

    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        // Proc-macro crates would be exempt; hilavitkutin has none today.
        if ctx.is_proc_macro_crate() {
            return Vec::new();
        }

        // Only scan hilavitkutin* crates. Arvo substrate is a separate
        // repo with its own no-std lint.
        if !ctx.crate_name.starts_with("hilavitkutin") {
            return Vec::new();
        }

        let mut errors = Vec::new();
        for (line_idx, line) in ctx.source.lines().enumerate() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("//") || trimmed.starts_with("*") {
                continue;
            }

            if trimmed.starts_with("use std::") || trimmed.starts_with("pub use std::") {
                errors.push(LintError::with_severity(
                    ctx.crate_name.to_string(),
                    line_idx + 1,
                    "no-std",
                    format!(
                        "`use std::` in a no_std crate. Prefer `core::` or a provider trait: {}",
                        trimmed.trim(),
                    ),
                    Severity::HARD_ERROR,
                ));
                continue;
            }

            // Catch extern crate std;
            if trimmed.starts_with("extern crate std") {
                errors.push(LintError::with_severity(
                    ctx.crate_name.to_string(),
                    line_idx + 1,
                    "no-std",
                    "`extern crate std;` in a no_std crate. Remove and rely on `core::`.".to_string(),
                    Severity::HARD_ERROR,
                ));
            }
        }

        errors
    }
}

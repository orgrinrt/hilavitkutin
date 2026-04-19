//! Lint: no `alloc` use in any `hilavitkutin*` source file.
//!
//! The engine never allocates at runtime. Plan-time structures use
//! const-generic arvo primitives. Runtime buffers come from the
//! consumer-provided `MemoryProvider` trait.
//!
//! Catches:
//! - `use alloc::` / `extern crate alloc;`
//! - `Vec<` / `Vec::new` / `vec!` macro
//! - `String` type reference / `String::new`
//! - `Box<` / `Box::new`
//!
//! Line-scan heuristic is permissive by design — false positives (e.g.
//! a `Vec` referenced in a string literal in a doc comment) are rare
//! and easily suppressed with `// lint:allow(no-alloc) -- <reason>`.

use mockspace::{Lint, LintContext, LintError, Severity};

pub fn lint() -> Box<dyn Lint> {
    Box::new(NoAllocEnforcer)
}

struct NoAllocEnforcer;

impl Lint for NoAllocEnforcer {
    fn name(&self) -> &'static str {
        "no-alloc"
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
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
            if trimmed.starts_with("//") || trimmed.starts_with("*") {
                continue;
            }
            if trimmed.contains("lint:allow(no-alloc)") {
                continue;
            }

            let violations: &[(&str, &str)] = &[
                ("use alloc::", "`use alloc::` — the engine is no-alloc. Runtime buffers come from MemoryProvider."),
                ("pub use alloc::", "`pub use alloc::` — re-exporting alloc breaks the no-alloc contract."),
                ("extern crate alloc", "`extern crate alloc;` — remove; the engine does not allocate."),
                ("Vec<", "`Vec<T>` in source. Use arvo const-generic bitmasks, sparse structures, or MemoryProvider-backed buffers."),
                ("Vec::new", "`Vec::new` in source. Use a provider-backed buffer."),
                ("vec![", "`vec![]` macro. Const-size arrays or provider buffers instead."),
                ("String", "`String` in source. Use `hilavitkutin-str` interned IDs or fixed-size byte buffers."),
                ("Box<", "`Box<T>` in source. Monomorphised types only — no heap boxing."),
                ("Box::new", "`Box::new` in source. No runtime heap allocation."),
                ("Rc<", "`Rc<T>` — no ref-counting in the engine."),
                ("Arc<", "`Arc<T>` — no atomic ref-counting in the engine."),
            ];

            for (needle, msg) in violations {
                if trimmed.contains(needle) {
                    errors.push(LintError::with_severity(
                        ctx.crate_name.to_string(),
                        line_idx + 1,
                        "no-alloc",
                        format!("{}: {}", msg, trimmed.trim()),
                        Severity::HARD_ERROR,
                    ));
                    break;
                }
            }
        }

        errors
    }
}

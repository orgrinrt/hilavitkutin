//! Lint: no dynamic dispatch in any `hilavitkutin*` source file.
//!
//! Monomorphisation is the dispatch. WorkUnits are generic over
//! their `AccessSet`; the fused pipeline is one straight-line
//! monomorphised function per core. Dynamic dispatch breaks the
//! devirtualisation contract and prevents the ExpandedLto pragma
//! from doing its job.
//!
//! Catches:
//! - ` dyn ` — trait objects
//! - `TypeId` — runtime type identity
//! - `std::any::` / `core::any::` — type erasure
//!
//! Per-line scan. False positives (e.g. `dyn` in a comment or
//! string) are possible but rare; suppress with
//! `// lint:allow(no-dynamic-dispatch) -- <reason>`.

use mockspace::{Lint, LintContext, LintError, Severity};

pub fn lint() -> Box<dyn Lint> {
    Box::new(NoDynamicDispatch)
}

struct NoDynamicDispatch;

impl Lint for NoDynamicDispatch {
    fn name(&self) -> &'static str {
        "no-dynamic-dispatch"
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
            if trimmed.contains("lint:allow(no-dynamic-dispatch)") {
                continue;
            }

            // Tokenised ` dyn ` check — avoid matching identifiers like `dynamic`.
            let has_dyn = line.contains(" dyn ")
                || line.contains("<dyn ")
                || line.contains("&dyn ")
                || line.contains("&mut dyn ")
                || line.contains("*const dyn ")
                || line.contains("*mut dyn ")
                || line.contains("Box<dyn ")
                || line.contains("Rc<dyn ")
                || line.contains("Arc<dyn ");

            if has_dyn {
                errors.push(LintError::with_severity(
                    ctx.crate_name.to_string(),
                    line_idx + 1,
                    "no-dynamic-dispatch",
                    format!(
                        "`dyn Trait` trait object — monomorphise instead (generic param or enum): {}",
                        trimmed.trim(),
                    ),
                    Severity::HARD_ERROR,
                ));
                continue;
            }

            if trimmed.contains("TypeId")
                || trimmed.contains("std::any::")
                || trimmed.contains("core::any::")
                || trimmed.contains("Any>")
                || trimmed.contains("Any +")
                || trimmed.contains(": Any")
            {
                errors.push(LintError::with_severity(
                    ctx.crate_name.to_string(),
                    line_idx + 1,
                    "no-dynamic-dispatch",
                    format!(
                        "`TypeId` / `std::any` type erasure — the monomorphised function IS the type identity: {}",
                        trimmed.trim(),
                    ),
                    Severity::HARD_ERROR,
                ));
            }
        }

        errors
    }
}

//! Lint: no runtime plugin registration in `hilavitkutin*` sources.
//!
//! Static composition is a core principle: all WorkUnits are known
//! at compile time via the scheduler builder. Runtime registration
//! mechanisms (`inventory::`, `#[ctor]`, `#[distributed_slice]`)
//! introduce order-of-initialisation footguns, break dead-code
//! elimination, and prevent the fused-pipeline monomorphisation
//! from converging.
//!
//! Catches:
//! - `inventory::submit!` / `inventory::collect!`
//! - `#[ctor]` / `#[ctor::ctor]`
//! - `#[distributed_slice]` / `linkme::distributed_slice`
//! - `use inventory` / `use linkme` / `use ctor`

use mockspace::{Lint, LintContext, LintError, Severity};

pub fn lint() -> Box<dyn Lint> {
    Box::new(NoRuntimeRegistration)
}

struct NoRuntimeRegistration;

impl Lint for NoRuntimeRegistration {
    fn name(&self) -> &'static str {
        "no-runtime-registration"
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
            if trimmed.contains("lint:allow(no-runtime-registration)") {
                continue;
            }

            let forbidden: &[(&str, &str)] = &[
                ("inventory::submit!", "`inventory::submit!` — composition is static via the scheduler builder, not a global registry."),
                ("inventory::collect!", "`inventory::collect!` — composition is static via the scheduler builder."),
                ("use inventory", "`use inventory` — the crate has no place in the engine; compose via the scheduler builder."),
                ("#[ctor]", "`#[ctor]` — no pre-main constructors; initialise in Builder::new."),
                ("#[ctor::ctor]", "`#[ctor::ctor]` — no pre-main constructors; initialise in Builder::new."),
                ("use ctor", "`use ctor` — the crate has no place in the engine."),
                ("#[distributed_slice]", "`#[distributed_slice]` — static-link registration; compose via the scheduler builder."),
                ("distributed_slice!", "`distributed_slice!` — compose via the scheduler builder."),
                ("use linkme", "`use linkme` — the crate has no place in the engine; compose statically."),
            ];

            for (needle, msg) in forbidden {
                if trimmed.contains(needle) {
                    errors.push(LintError::with_severity(
                        ctx.crate_name.to_string(),
                        line_idx + 1,
                        "no-runtime-registration",
                        format!("{} {}", msg, trimmed.trim()),
                        Severity::HARD_ERROR,
                    ));
                    break;
                }
            }
        }

        errors
    }
}

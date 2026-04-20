//! Lint: no runtime thread spawn in any `hilavitkutin*` source file.
//!
//! The engine uses a pre-allocated thread pool. Threads are spawned
//! once at pipeline construction via the consumer-provided
//! `ThreadPool` implementation; they park between frames. Any
//! `thread::spawn` / `tokio::spawn` / `async_std::spawn` at runtime
//! violates that contract.
//!
//! The pool itself — the `ThreadPool` implementation that the
//! consumer provides — may legitimately call `thread::spawn`. It is
//! consumer code, not engine code, and lives outside `hilavitkutin*`
//! crates, so this lint never sees it.

use mockspace::{Lint, LintContext, LintError, Severity};

pub fn lint() -> Box<dyn Lint> {
    Box::new(NoRuntimeSpawn)
}

struct NoRuntimeSpawn;

impl Lint for NoRuntimeSpawn {
    fn name(&self) -> &'static str {
        "no-runtime-spawn"
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
            if trimmed.contains("lint:allow(no-runtime-spawn)") {
                continue;
            }

            let forbidden: &[(&str, &str)] = &[
                ("thread::spawn", "`thread::spawn` — pool is pre-allocated; spawn only through consumer's ThreadPool impl."),
                ("std::thread::spawn", "`std::thread::spawn` — pool is pre-allocated."),
                ("std::thread::Builder", "`std::thread::Builder` — pool is pre-allocated."),
                ("tokio::spawn", "`tokio::spawn` — engine is sync, no runtime spawn."),
                ("tokio::task::spawn", "`tokio::task::spawn` — engine is sync, no runtime spawn."),
                ("async_std::task::spawn", "`async_std::task::spawn` — engine is sync, no runtime spawn."),
                ("rayon::spawn", "`rayon::spawn` — engine owns its own thread pool."),
                ("smol::spawn", "`smol::spawn` — engine is sync, no runtime spawn."),
            ];

            for (needle, msg) in forbidden {
                if trimmed.contains(needle) {
                    errors.push(LintError::with_severity(
                        ctx.crate_name.to_string(),
                        line_idx + 1,
                        "no-runtime-spawn",
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

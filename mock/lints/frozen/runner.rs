//! Running a frozen lint through the engine's runner, for the lints' tests.
//!
//! A lint's own `findings` never sees a `lint:allow` marker, but the runner
//! drops a finding carrying one unless the lint says markers do not decide.
//! Only a run through the runner shows which of the two holds.

use mockspace::testkit::{LintFixture, run};
use mockspace::{CrateLint, Lint, LintError, LintPack};

/// A fixture of the frozen authors crate holding `text` as `src/lib.rs`.
pub(crate) fn authors(text: &str) -> LintFixture {
    LintFixture::new(text).with_crate_name("hilavitkutin-frozen-authors", "frozen-authors")
}

/// What the runner reports for `lint` over `fixture`, after it has honoured
/// markers and applied severities.
pub(crate) fn reported(lint: Box<dyn CrateLint>, fixture: LintFixture) -> Vec<LintError> {
    let name = lint.name();
    let pack = LintPack {
        crate_lints: vec![lint],
        ..LintPack::default()
    };
    run(&[fixture], None, Some(&pack))
        .into_iter()
        .filter(|e| e.lint_name == name)
        .collect()
}

/// The lines the runner reports for `lint` over one source file of the frozen
/// authors crate.
pub(crate) fn lines_reported(lint: Box<dyn CrateLint>, text: &str) -> Vec<usize> {
    reported(lint, authors(text))
        .into_iter()
        .map(|e| e.line)
        .collect()
}

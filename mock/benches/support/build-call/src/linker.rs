//! Which linker an arm names, the mark that linker must leave in what it
//! links, and the proof read from an executable after a link.
//!
//! A link arm is named `<linker>_<quantity>`: `default_relink`,
//! `default_replay`. The one linker this bench knows is the toolchain's
//! default for the host, whose mark is worked out from the toolchain at run
//! time ([`crate::toolchain::default_mark`]). Every other linker waits on the
//! hilavitkutin-build round that gives each linker a row with its mark; until
//! then an arm naming one is refused, whatever its output carries.

use std::path::Path;
use std::process::Command;

use crate::{elf, toolchain};

/// The linker an arm's name names: the part before its last `_`.
pub fn named(arm: &str) -> Option<&str> {
    arm.rsplit_once('_').map(|(linker, _)| linker)
}

/// The mark an executable linked by `arm` must carry, or why the arm cannot
/// be held to one.
pub fn expected_mark(arm: &str) -> Result<String, String> {
    match named(arm) {
        Some("default") => toolchain::get().default_mark(),
        Some(other) => {
            Err(format!(
                "arm `{arm}` names the linker `{other}`, and only the toolchain's default has a mark this bench \
                 can hold a link to"
            ))
        },
        None => Err(format!("arm `{arm}` names no linker")),
    }
}

/// Whether the recorded `mark` is `expected`: the same text, or the same text
/// followed by what the linker adds after a space (lld adds the commit of the
/// LLVM it was built from). `Linker: LLD 22.1.60` is not `Linker: LLD 22.1.6`.
pub fn holds(mark: &str, expected: &str) -> bool {
    mark.strip_prefix(expected)
        .is_some_and(|rest| rest.is_empty() || rest.starts_with(' '))
}

/// Why `mark` on a link by `arm` fails, or `None` where it holds.
pub fn refusal(arm: &str, mark: &str) -> Option<String> {
    match expected_mark(arm) {
        Ok(want) if holds(mark, &want) => None,
        Ok(want) => {
            Some(format!(
                "the output carries the mark `{mark}`, and arm `{arm}` must carry `{want}`"
            ))
        },
        Err(why) => Some(why),
    }
}

/// The linker's mark in `exe` and the check line the program prints.
pub fn proof(exe: &Path) -> (String, String) {
    let mark = elf::comment(exe)
        .map(|c| elf::linker_mark(&c).unwrap_or_else(|| format!("none among {c:?}")))
        .unwrap_or_else(|e| e);
    let check = Command::new(exe)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|e| format!("did not run: {e}"));
    (mark, check)
}

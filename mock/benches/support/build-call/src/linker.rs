//! The mark a linker must leave in what it links, and the proof read from an
//! executable after a link.
//!
//! A record's line names the linker of the arm that wrote it, as the arm's
//! set of pragmas gives it ([`crate::Arm::linker`]). What that linker leaves
//! is hilavitkutin-build's to say: a named linker's mark is in its tool's row
//! there, and the toolchain's own is worked out from the toolchain at run time
//! ([`crate::toolchain::own_mark`]). A name the crate does not know is refused,
//! whatever the output carries.

use std::path::Path;
use std::process::Command;

use hilavitkutin_build::{Linker, linker_named};

use crate::{elf, toolchain};

/// What a line's mark starts with where the linker left none: the compilers'
/// own strings follow it, so the record shows what was there.
pub const NO_MARK: &str = "none among ";

/// Why `mark` on a link by the linker named `linker` fails, or `None` where it
/// holds. The toolchain's own linker must leave exactly its mark, which
/// carries its whole version line, so a longer version, another build of the
/// same version or a second linker's string joined after it are each refused.
/// A named linker must leave a text that starts with its mark, and one with no
/// mark must leave none.
pub fn refusal(linker: &str, mark: &str) -> Option<String> {
    let Some(known) = Option::<Linker>::from(linker_named(linker)) else {
        return Some(format!(
            "the line names the linker `{linker}`, which hilavitkutin-build does not know"
        ));
    };
    let read = (!mark.starts_with(NO_MARK)).then_some(mark);
    match known {
        Linker::Toolchain => {
            match toolchain::get().own_mark() {
                Ok(want) if known.leaves(read.into(), Some(want.as_str()).into()) => None,
                Ok(want) => {
                    Some(format!(
                        "the output carries the mark `{mark}`, and the toolchain's own linker must carry `{want}`"
                    ))
                },
                Err(why) => Some(why),
            }
        },
        Linker::Named {
            ..
        } if known.leaves(read.into(), None.into()) => None,
        Linker::Named {
            name,
            mark: want,
        } => {
            Some(match Option::<&str>::from(want) {
                Some(want) => {
                    format!(
                        "the output carries the mark `{mark}`, and {name} leaves one that starts `{want}`"
                    )
                },
                None => format!("the output carries the mark `{mark}`, and {name} leaves none"),
            })
        },
    }
}

/// The linker's mark in `exe` and the check line the program prints.
pub fn proof(exe: &Path) -> (String, String) {
    let mark = elf::comment(exe)
        .map(|c| elf::linker_mark(&c).unwrap_or_else(|| format!("{NO_MARK}{c:?}")))
        .unwrap_or_else(|e| e);
    let check = Command::new(exe)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|e| format!("did not run: {e}"));
    (mark, check)
}

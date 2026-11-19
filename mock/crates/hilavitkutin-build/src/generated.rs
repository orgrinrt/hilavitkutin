//! The generated cargo file of the script that claims the workspace.
//!
//! The pragmas of workspace reach are cargo configuration. The one script
//! written `script!(<prototype>, workspace)` keeps their keys in a file
//! of its own beside the wrapper, which the workspace's `config.toml`
//! takes by one `include` line. This module holds the file's text and
//! what a script reads back from it; the reads and the writes stay the
//! script's.

use core::fmt::Write;

use notko::Maybe;

use crate::pragma::Pragma;
use crate::profile::Profile;
use crate::prototype::Prototype;
use crate::set::{Set, Site, Visit};

/// The generated file's name under the workspace's `.cargo/`.
pub const GENERATED_NAME: &str = "hilavitkutin-build.toml"; // lint:allow(no-bare-static-str) reason: a build-dependency only, it reaches no binary; tracked: no_bare_static_str_does_not_know_a_build_dependency_only_cra

/// What the file's first line says before the crate's name.
const HEAD: &str = "# written by hilavitkutin-build for "; // lint:allow(no-bare-static-str) reason: a build-dependency only, it reaches no binary; tracked: no_bare_static_str_does_not_know_a_build_dependency_only_cra

/// What the file's first line says after the crate's name.
const TAIL: &str = ": edits are lost"; // lint:allow(no-bare-static-str) reason: a build-dependency only, it reaches no binary; tracked: no_bare_static_str_does_not_know_a_build_dependency_only_cra

/// The walk that writes every member's cargo configuration.
struct Configured<'a, 's, W: Write> {
    site: &'a Site<'s>,
    out:  &'a mut W,
}

impl<W: Write> Visit for Configured<'_, '_, W> {
    fn pragma<P: Pragma>(&mut self) -> core::fmt::Result {
        P::write_cargo_config(&self.site.written::<P>(), self.out)
    }
}

/// Write the generated cargo file of the prototype `P`, claimed by the
/// crate `krate`, for the target `triple`.
///
/// The first line names the crate. Then come the keys of the `dev` set
/// and of the `release` set, in dotted keys with no table header. A key
/// that belongs to no profile is written by the first of the two sets
/// that holds its pragma and left by the second. The three custom
/// profiles get no key: cargo tells a script only `debug` or `release`,
/// and a table for a profile the consumer's manifest does not define is
/// an error.
pub fn write_generated<P: Prototype, W: Write>(
    krate: &str, // lint:allow(no-bare-string) reason: the crate's name as the build script read it; tracked: #72
    triple: &str, // lint:allow(no-bare-string) reason: the target triple as the build script read it; tracked: #72
    out: &mut W,
) -> core::fmt::Result {
    writeln!(out, "{HEAD}{krate}{TAIL}")?;
    let dev = Site::new(Profile::Dev, triple, &[]);
    <P::Dev as Set>::each(&mut Configured {
        site: &dev,
        out,
    })?;
    let release = Site::new(Profile::Release, triple, &[]).after::<P::Dev>();
    <P::Release as Set>::each(&mut Configured {
        site: &release,
        out,
    })
}

/// Whose the generated file is, as the script found it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Claim<'a> {
    /// There is no file.
    Free,
    /// The file was written for this crate.
    Mine,
    /// The file was written for another crate, named here, or by
    /// somebody else, where the name is empty.
    Theirs(&'a str), // lint:allow(no-bare-string) reason: a crate's name as the file has it; tracked: #72
}

/// Read whose the generated file is from its text, for the crate `krate`.
///
/// `text` is `Maybe::Isnt` where there is no file. The first line is the
/// claim. A file whose first line is not one this crate writes is
/// somebody else's, and is left alone as another crate's is.
pub fn claim<'a>(
    text: Maybe<&'a str>, // lint:allow(no-bare-string) reason: a file's text as the build script read it; tracked: #72
    krate: &str, // lint:allow(no-bare-string) reason: the crate's name as the build script read it; tracked: #72
) -> Claim<'a> {
    let Maybe::Is(text) = text else {
        return Claim::Free;
    };
    let first = text.lines().next().unwrap_or("");
    match first
        .strip_prefix(HEAD)
        .and_then(|rest| rest.strip_suffix(TAIL))
    {
        Some(name) if name == krate => Claim::Mine,
        Some(name) => Claim::Theirs(name),
        None => Claim::Theirs(""),
    }
}

/// Whether a `config.toml` of this text takes the generated file: an
/// `include` key before any table that names [`GENERATED_NAME`] as a
/// whole file name. A key in a comment or under a table names nothing.
#[rustfmt::skip] // keeps the allow on the signature it governs
pub fn includes_generated(text: &str) -> bool { // lint:allow(no-bare-string) lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a config file's text as the build script read it; tracked: #72
    // Inside the value of an `include` key, which may be an array over
    // several lines: the brackets it opened and the ones it closed.
    let mut including = false;
    let mut opened = 0;
    let mut closed = 0;
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('#') {
            continue;
        }
        if !including {
            if line.starts_with('[') {
                return false;
            }
            if !line.strip_prefix("include").is_some_and(|rest| rest.trim_start().starts_with('=')) {
                continue;
            }
            opened = 0;
            closed = 0;
        }
        if names_the_file(line) {
            return true;
        }
        opened += line.matches('[').count();
        closed += line.matches(']').count();
        including = closed < opened;
    }
    false
}

/// Whether a line holds [`GENERATED_NAME`] in quotes, alone or as the
/// last component of a path.
#[rustfmt::skip] // keeps the allow on the signature it governs
fn names_the_file(line: &str) -> bool { // lint:allow(no-bare-string) lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a config file's text as the build script read it; tracked: #72
    let mut rest = line;
    while let Some(at) = rest.find(GENERATED_NAME) {
        let before = rest[..at].chars().next_back();
        let after = rest[at + GENERATED_NAME.len()..].chars().next();
        let opens = matches!(before, Some('"' | '\'' | '/'));
        let closes = matches!(after, Some('"' | '\''));
        if opens && closes {
            return true;
        }
        rest = &rest[at + GENERATED_NAME.len()..];
    }
    false
}

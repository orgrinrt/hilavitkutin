//! The linker of a set, and the toolchain's own linker for a target.
//!
//! A set is linked by the linker its pragma of
//! [`LinkerGroup`](crate::LinkerGroup) names, or by the toolchain's own
//! where it holds none. Which linker the toolchain's own is differs by
//! target, and rustc says it in its description of the target. Running
//! rustc and the linker stays the caller's; this module reads what they
//! printed.

use core::fmt::{self, Display};

use notko::Maybe;

use crate::choice::{LinkerGroup, Linkers};
use crate::pragma::{Group, Pragma, Words};
use crate::set::{Set, Visit};

/// The name of the toolchain's own linker, where a linker is named.
const TOOLCHAIN: Words = "toolchain";

/// What links a crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Linker {
    /// The toolchain's own linker for the target: the set holds no pragma
    /// of the linker's group.
    Toolchain,
    /// The linker a pragma names.
    Named {
        /// The pragma's name.
        name: Words,
        /// What the linker's mark in `.comment` starts with, or nothing
        /// for a linker that leaves no mark.
        mark: Maybe<Words>,
    },
}

impl Linker {
    /// The linker's name, which [`linker_named`] takes back.
    pub const fn name(&self) -> Words {
        match self {
            Linker::Toolchain => TOOLCHAIN,
            Linker::Named {
                name,
                ..
            } => name,
        }
    }

    /// Whether what was read from an executable is this linker's mark.
    ///
    /// `read` is the text the linker left in `.comment`, or nothing where
    /// it left none. `own` is the mark the caller worked out for the
    /// toolchain's own linker ([`rust_lld_mark`]), or nothing where it has
    /// none. A named linker with a mark leaves a text that starts with
    /// it, a named linker with none leaves nothing, and the toolchain's
    /// own leaves exactly `own`.
    #[rustfmt::skip] // keeps the allow on the signature it governs
    pub fn leaves(&self, read: Maybe<&str>, own: Maybe<&str>) -> bool { // lint:allow(no-bare-string) lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a mark as the caller read it from an executable; tracked: #72
        match (*self, read) {
            (Linker::Toolchain, Maybe::Is(read)) => matches!(own, Maybe::Is(own) if own == read),
            (Linker::Toolchain, Maybe::Isnt) => false,
            (Linker::Named { mark: Maybe::Is(mark), .. }, Maybe::Is(read)) => read.starts_with(mark),
            (Linker::Named { mark: Maybe::Is(_), .. }, Maybe::Isnt) => false,
            (Linker::Named { mark: Maybe::Isnt, .. }, read) => matches!(read, Maybe::Isnt),
        }
    }
}

/// The walk that takes the member of the linker's group.
struct Linked {
    found: Linker,
}

impl Visit for Linked {
    fn pragma<P: Pragma>(&mut self) -> fmt::Result {
        if <P::Group as Group>::BIT == <LinkerGroup as Group>::BIT {
            let mark = match P::NEEDS.first() {
                Some(tool) => tool.proof.mark,
                None => Maybe::Isnt,
            };
            self.found = Linker::Named {
                name: P::NAME,
                mark,
            };
        }
        Ok(())
    }
}

/// The linker of a crate whose set is `S`.
///
/// The mark of a named linker is the one its tool's row gives, the first
/// of the pragma's [`NEEDS`](Pragma::NEEDS), and nothing for a pragma
/// that needs no tool: GNU ld leaves no mark.
pub fn linker_of<S: Set>() -> Linker {
    let mut walk = Linked {
        found: Linker::Toolchain,
    };
    // The walk's only failure is a writer's, and this walk writes nothing.
    let _ = S::each(&mut walk);
    walk.found
}

/// The walk that finds a linker pragma by its name.
struct Named<'a> {
    name:  &'a str, // lint:allow(no-bare-string) reason: a linker's name as the caller read it; tracked: #72
    found: Maybe<Linker>,
}

impl Visit for Named<'_> {
    fn pragma<P: Pragma>(&mut self) -> fmt::Result {
        if P::NAME == self.name {
            self.found = Maybe::Is(linker_of::<(P,)>());
        }
        Ok(())
    }
}

/// The linker of the name `name`, as [`Linker::name`] gave it, or nothing
/// for a name no linker has. It is for a reader that has a recorded name
/// and no type.
#[rustfmt::skip] // keeps the allow on the signature it governs
pub fn linker_named(name: &str) -> Maybe<Linker> { // lint:allow(no-bare-string) reason: a linker's name as the caller read it; tracked: #72
    if name == TOOLCHAIN {
        return Maybe::Is(Linker::Toolchain);
    }
    let mut walk = Named {
        name,
        found: Maybe::Isnt,
    };
    // The walk's only failure is a writer's, and this walk writes nothing.
    let _ = <Linkers as Set>::each(&mut walk);
    walk.found
}

/// The toolchain's own linker for a target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolchainLinker<'a> {
    /// The `rust-lld` the toolchain carries: the target's linker flavor is
    /// an lld one and the toolchain links with its own copy.
    RustLld,
    /// Something the crate has no mark for, with the linker flavor as the
    /// target's description has it, empty where it names none.
    Other(&'a str), // lint:allow(no-bare-string) reason: a field of rustc's target description as the caller read it; tracked: #72
}

impl<'a> ToolchainLinker<'a> {
    /// Read the toolchain's own linker from `spec`, the text rustc prints
    /// for `--print target-spec-json`.
    ///
    /// An lld flavor alone is not enough: where `link-self-contained`
    /// does not name the linker, the lld is the machine's, which is
    /// another build and leaves another mark.
    #[rustfmt::skip] // keeps the allow on the signature it governs
    pub fn of(spec: &'a str) -> Self { // lint:allow(no-bare-string) reason: rustc's target description as the caller read it; tracked: #72
        let flavor = match string_field(spec, "linker-flavor") {
            Maybe::Is(flavor) => flavor,
            Maybe::Isnt => "",
        };
        let own = matches!(field(spec, "link-self-contained"), Maybe::Is(held) if held.contains("\"linker\""));
        if flavor.contains("lld") && own { ToolchainLinker::RustLld } else { ToolchainLinker::Other(flavor) }
    }
}

/// The text after `"key":` in a JSON text, from the first place `key`
/// stands in quotes before a colon.
#[rustfmt::skip] // keeps the allow on the signature it governs
fn after_key<'a>(json: &'a str, key: &str) -> Maybe<&'a str> { // lint:allow(no-bare-string) reason: rustc's target description as the caller read it; tracked: #72
    let mut rest = json;
    while let Some(at) = rest.find(key) {
        let quoted = rest[..at].ends_with('"');
        let tail = &rest[at + key.len()..];
        let value = tail.strip_prefix('"').and_then(|tail| tail.trim_start().strip_prefix(':'));
        if let (true, Some(value)) = (quoted, value) {
            return Maybe::Is(value.trim_start());
        }
        rest = tail;
    }
    Maybe::Isnt
}

/// The string value of `"key": "value"` in a JSON text.
#[rustfmt::skip] // keeps the allow on the signature it governs
fn string_field<'a>(json: &'a str, key: &str) -> Maybe<&'a str> { // lint:allow(no-bare-string) reason: rustc's target description as the caller read it; tracked: #72
    let Maybe::Is(value) = after_key(json, key) else {
        return Maybe::Isnt;
    };
    match value.strip_prefix('"').and_then(|value| value.split_once('"')) {
        Some((text, _)) => Maybe::Is(text),
        None => Maybe::Isnt,
    }
}

/// The text of the value after `"key":` in a JSON text: up to its
/// closing brace where it is an object with no object inside it, else up
/// to the end of its line.
#[rustfmt::skip] // keeps the allow on the signature it governs
fn field<'a>(json: &'a str, key: &str) -> Maybe<&'a str> { // lint:allow(no-bare-string) reason: rustc's target description as the caller read it; tracked: #72
    let Maybe::Is(value) = after_key(json, key) else {
        return Maybe::Isnt;
    };
    match value.strip_prefix('{') {
        Some(body) => match body.split_once('}') {
            Some((held, _)) => Maybe::Is(held),
            None => Maybe::Isnt,
        },
        None => Maybe::Is(value.split(['\n', ',']).next().unwrap_or(value)),
    }
}

/// What GNU-flavor lld prints after its version for `--version`. Into
/// `.comment` it writes `Linker: ` and the version alone.
const GNU_COMPATIBLE: &str = " (compatible with GNU linkers)"; // lint:allow(no-bare-static-str) reason: a build-dependency only, it reaches no binary; tracked: no_bare_static_str_does_not_know_a_build_dependency_only_cra

/// The mark the toolchain's own `rust-lld` leaves in what it links.
///
/// It displays as `Linker: ` and the version, the LLVM commit the linker
/// was built from included, so a machine's lld of the same LLVM version
/// has another mark.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RustLldMark<'a> {
    version: &'a str, // lint:allow(no-bare-string) reason: a version line as the caller read it; tracked: #72
}

impl Display for RustLldMark<'_> {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str("Linker: ")?;
        out.write_str(self.version)
    }
}

/// The mark of the `rust-lld` that said `said`, the first line it
/// printed for `--version` as the GNU flavor, or nothing where the line
/// is not lld's in that form.
#[rustfmt::skip] // keeps the allow on the signature it governs
pub fn rust_lld_mark(said: &str) -> Maybe<RustLldMark<'_>> { // lint:allow(no-bare-string) reason: a version line as the caller read it; tracked: #72
    match said.strip_suffix(GNU_COMPATIBLE) {
        Some(version) if version.starts_with("LLD ") => Maybe::Is(RustLldMark {
            version,
        }),
        _ => Maybe::Isnt,
    }
}

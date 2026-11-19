//! The seven build choices: linker, edge form, LTO, incremental, backend,
//! link driver and C compiler.
//!
//! Each is a group with one pragma per value that differs from the
//! toolchain's own, and a group with no pragma in the set leaves the
//! choice to the toolchain. The linker and the edge form reach the crate
//! whose script names them. The other five are cargo configuration and
//! reach the whole workspace.

use core::fmt::{self, Write};

use notko::Maybe;

use crate::pragma::{Group, GroupSlot, Mask, Not, Pragma, Reach, Shared, Slot, Words, Written};
use crate::roster::ExpandedLto;
use crate::target::{ObjectFormat, Target};
use crate::tools::{CLANG, CRANELIFT, GCC, MOLD, Tool, WILD};

/// A group: its place and the sentence for a set that holds two of it.
macro_rules! group {
    ($(#[$doc:meta])* $name:ident, $slot:ident, $shared:expr, $clash:expr) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy)]
        pub struct $name;

        impl Group for $name {
            const BIT: Mask = GroupSlot::$slot.bit();
            const CLASH: Words = $clash;
            const SHARED: bool = $shared; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a build-time fact, the crate takes no arvo dependency; tracked: #72
        }
    };
}

group!(
    /// The linker of what a crate links.
    LinkerGroup,
    Linker,
    false,
    "the set holds two linkers, and a crate is linked by one"
);
group!(
    /// The form of a crate's edge to what depends on it.
    EdgeGroup,
    Edge,
    false,
    "the set holds two edge forms, and a crate has one"
);
group!(
    /// The LTO of a profile.
    LtoGroup,
    Lto,
    false,
    "the set holds two LTO pragmas, and a profile has one"
);
group!(
    /// Whether a profile builds incrementally.
    IncrementalGroup,
    Incremental,
    false,
    "the set holds Incremental and NoIncremental, and a profile is one of the two"
);
group!(
    /// The codegen backend of a profile.
    BackendGroup,
    Backend,
    false,
    "the set holds two backends, and a profile has one"
);
group!(
    /// The program rustc links through. Its key belongs to no profile.
    DriverGroup,
    Driver,
    true,
    "the set holds two link drivers, and a target has one"
);
group!(
    /// The C compiler a build script finds. Its keys belong to no profile.
    CcGroup,
    Cc,
    true,
    "the set holds two C compilers, and a build has one"
);

/// What a linker that writes ELF says of another target.
#[rustfmt::skip] // keeps the allow on the signature it governs
fn elf_only(target: &Target) -> Maybe<&'static str> {
    match target.format {
        ObjectFormat::Elf => Maybe::Isnt,
        _ => Maybe::Is("links ELF only, and this target's objects are not ELF"),
    }
}

/// What a pragma with no meaning on wasm says of a wasm target.
fn not_wasm(target: &Target) -> Maybe<&'static str> {
    match target.format {
        ObjectFormat::Wasm => Maybe::Is("has no meaning on wasm, which this target is"),
        _ => Maybe::Isnt,
    }
}

/// Link with mold, `-fuse-ld=mold`.
///
/// The argument is a rustc flag the wrapper appends, like every flag of
/// the crate, so the first build of a workspace, which runs without the
/// wrapper, links as the toolchain does by itself.
#[derive(Debug, Clone, Copy)]
pub struct Mold;

impl Pragma for Mold {
    type Group = LinkerGroup;
    type Requires = ();

    const BIT: Mask = Slot::Mold.bit();
    const NAME: Words = "Mold";
    const TOOL: Maybe<&'static Tool> = Maybe::Is(&MOLD);

    fn admitted(target: &Target) -> Maybe<&'static str> {
        elf_only(target)
    }

    fn write_rustflags<W: Write>(_with: &Written<'_>, out: &mut W) -> fmt::Result {
        out.write_str("-Clink-arg=-fuse-ld=mold")
    }
}

/// Link with GNU ld, `-fuse-ld=bfd`.
#[derive(Debug, Clone, Copy)]
pub struct Bfd;

impl Pragma for Bfd {
    type Group = LinkerGroup;
    type Requires = ();

    const BIT: Mask = Slot::Bfd.bit();
    const NAME: Words = "Bfd";

    fn admitted(target: &Target) -> Maybe<&'static str> {
        elf_only(target)
    }

    fn write_rustflags<W: Write>(_with: &Written<'_>, out: &mut W) -> fmt::Result {
        out.write_str("-Clink-arg=-fuse-ld=bfd")
    }
}

/// Link with wild, named to clang by its path.
///
/// A gcc older than 15 knows a fixed list of linkers and wild is not on
/// it, so the crate links through clang, which takes any linker by
/// `--ld-path`. The driver and the path are one pair of flags the
/// wrapper appends. Without a path for the tool nothing is written, and
/// the directives say the path is missing.
#[derive(Debug, Clone, Copy)]
pub struct Wild;

impl Pragma for Wild {
    type Group = LinkerGroup;
    type Requires = ();

    const BIT: Mask = Slot::Wild.bit();
    const NAME: Words = "Wild";
    const TOOL: Maybe<&'static Tool> = Maybe::Is(&WILD);

    fn admitted(target: &Target) -> Maybe<&'static str> {
        elf_only(target)
    }

    fn write_rustflags<W: Write>(with: &Written<'_>, out: &mut W) -> fmt::Result {
        match with.tool() {
            Maybe::Is(path) => write!(out, "-Clinker=clang -Clink-arg=--ld-path={path}"),
            Maybe::Isnt => Ok(()),
        }
    }
}

/// The crate is a dylib its dependents link dynamically,
/// `-Cprefer-dynamic`.
///
/// The flag is all it writes: that the crate's manifest says
/// `crate-type = ["dylib"]` is the consumer's to write.
#[derive(Debug, Clone, Copy)]
pub struct DylibDomain;

impl Pragma for DylibDomain {
    type Group = EdgeGroup;
    type Requires = ();

    const BIT: Mask = Slot::DylibDomain.bit();
    const NAME: Words = "DylibDomain";

    fn admitted(target: &Target) -> Maybe<&'static str> {
        not_wasm(target)
    }

    fn write_rustflags<W: Write>(_with: &Written<'_>, out: &mut W) -> fmt::Result {
        out.write_str("-Cprefer-dynamic")
    }
}

/// Thin LTO for the profile.
#[derive(Debug, Clone, Copy)]
pub struct ThinLto;

impl Pragma for ThinLto {
    type Group = LtoGroup;
    type Requires = ();

    const BIT: Mask = Slot::ThinLto.bit();
    const NAME: Words = "ThinLto";
    const REACH: Reach = Reach::Workspace;

    fn write_cargo_config<W: Write>(with: &Written<'_>, out: &mut W) -> fmt::Result {
        writeln!(out, "profile.{}.lto = \"thin\"", with.profile())
    }
}

/// The profile builds incrementally.
#[derive(Debug, Clone, Copy)]
pub struct Incremental;

impl Pragma for Incremental {
    type Group = IncrementalGroup;
    type Requires = ();

    const BIT: Mask = Slot::Incremental.bit();
    const NAME: Words = "Incremental";
    const REACH: Reach = Reach::Workspace;

    fn write_cargo_config<W: Write>(with: &Written<'_>, out: &mut W) -> fmt::Result {
        writeln!(out, "profile.{}.incremental = true", with.profile())
    }
}

/// The profile does not build incrementally.
#[derive(Debug, Clone, Copy)]
pub struct NoIncremental;

impl Pragma for NoIncremental {
    type Group = IncrementalGroup;
    type Requires = ();

    const BIT: Mask = Slot::NoIncremental.bit();
    const NAME: Words = "NoIncremental";
    const REACH: Reach = Reach::Workspace;

    fn write_cargo_config<W: Write>(with: &Written<'_>, out: &mut W) -> fmt::Result {
        writeln!(out, "profile.{}.incremental = false", with.profile())
    }
}

/// The profile's code is generated by cranelift.
///
/// Cargo takes the profile key behind its unstable flag of the same
/// name, which is in the file once.
#[derive(Debug, Clone, Copy)]
pub struct Cranelift;

impl Pragma for Cranelift {
    type Group = BackendGroup;
    type Requires = Not<(ThinLto, ExpandedLto)>;

    const BIT: Mask = Slot::Cranelift.bit();
    const NAME: Words = "Cranelift";
    const REACH: Reach = Reach::Workspace;
    const TOOL: Maybe<&'static Tool> = Maybe::Is(&CRANELIFT);
    const UNMET: Words = "Cranelift builds without LTO, and the set holds an LTO pragma";

    fn admitted(target: &Target) -> Maybe<&'static str> {
        not_wasm(target)
    }

    fn write_cargo_config<W: Write>(with: &Written<'_>, out: &mut W) -> fmt::Result {
        if matches!(with.shared(), Shared::Owed) {
            out.write_str("unstable.codegen-backend = true\n")?;
        }
        writeln!(
            out,
            "profile.{}.codegen-backend = \"cranelift\"",
            with.profile()
        )
    }
}

/// A link driver: the program rustc links the target through.
macro_rules! driver {
    ($(#[$doc:meta])* $name:ident, $program:literal, $tool:ident) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy)]
        pub struct $name;

        impl Pragma for $name {
            type Group = DriverGroup;
            type Requires = ();

            const BIT: Mask = Slot::$name.bit();
            const NAME: Words = stringify!($name);
            const REACH: Reach = Reach::Workspace;
            const TOOL: Maybe<&'static Tool> = Maybe::Is(&$tool);

            fn write_cargo_config<W: Write>(with: &Written<'_>, out: &mut W) -> fmt::Result {
                match with.shared() {
                    Shared::Owed => writeln!(out, concat!("target.\"{}\".linker = \"", $program, "\""), with.triple()),
                    Shared::Written => Ok(()),
                }
            }
        }
    };
}

driver!(
    /// rustc links through gcc.
    GccDriver,
    "gcc",
    GCC
);
driver!(
    /// rustc links through clang.
    ClangDriver,
    "clang",
    CLANG
);

/// A C compiler: the `CC`, `CXX` and `AR` a build script finds.
macro_rules! cc {
    ($(#[$doc:meta])* $name:ident, $cc:literal, $cxx:literal, $ar:literal, $tool:ident) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy)]
        pub struct $name;

        impl Pragma for $name {
            type Group = CcGroup;
            type Requires = ();

            const BIT: Mask = Slot::$name.bit();
            const NAME: Words = stringify!($name);
            const REACH: Reach = Reach::Workspace;
            const TOOL: Maybe<&'static Tool> = Maybe::Is(&$tool);

            fn write_cargo_config<W: Write>(with: &Written<'_>, out: &mut W) -> fmt::Result {
                match with.shared() {
                    Shared::Owed => out.write_str(concat!(
                        "env.CC = \"",
                        $cc,
                        "\"\nenv.CXX = \"",
                        $cxx,
                        "\"\nenv.AR = \"",
                        $ar,
                        "\"\n"
                    )),
                    Shared::Written => Ok(()),
                }
            }
        }
    };
}

cc!(
    /// Build scripts compile C with gcc.
    GccCc,
    "gcc",
    "g++",
    "ar",
    GCC
);
cc!(
    /// Build scripts compile C with clang.
    ClangCc,
    "clang",
    "clang++",
    "llvm-ar",
    CLANG
);

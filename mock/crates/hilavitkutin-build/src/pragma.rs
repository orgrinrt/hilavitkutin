//! A pragma is one build choice, a unit type.
//!
//! What a pragma is, what it requires of the set it is in, which choice
//! it is a value of and what it writes are all on the [`Pragma`] trait. A
//! set of pragmas is a tuple of the types ([`Set`](crate::Set)), so what
//! a set holds is known before any build script runs.

use core::fmt::Write;
use core::marker::PhantomData;

use notko::Maybe;

use crate::set::Set;
use crate::target::Target;
use crate::tools::Tool;

/// The bits of a set: one per pragma in a set's mask, one per group in
/// its group mask.
pub type Mask = u64; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a set's mask, the crate takes no arvo dependency; tracked: #72

/// The words a pragma, a group or a set is said in: a name, or the
/// sentence of what is wrong.
pub type Words = &'static str; // lint:allow(no-bare-static-str) reason: a build-dependency only, it reaches no binary; tracked: no_bare_static_str_does_not_know_a_build_dependency_only_cra

/// The place of every pragma in a set's mask, one list.
///
/// A pragma's [`BIT`](Pragma::BIT) is its place here, so two pragmas
/// cannot be given one bit by two files that do not know of each other.
#[derive(Clone, Copy)]
pub(crate) enum Slot {
    LoopOptimization,
    Polly,
    MathPeephole,
    FastMath,
    ExpandedLto,
    Pgo,
    Bolt,
    Profiling,
    BuildStd,
    ParallelCodegen,
    SharedGenerics,
    LoopFusion,
    MimallocAllocator,
    Mold,
    Wild,
    Bfd,
    DylibDomain,
    ThinLto,
    Incremental,
    NoIncremental,
    Cranelift,
    GccDriver,
    ClangDriver,
    GccCc,
    ClangCc,
}

impl Slot {
    /// The bit of this place.
    pub(crate) const fn bit(self) -> Mask {
        1 << self as Mask
    }
}

/// The place of every group in a set's group mask, one list.
#[derive(Clone, Copy)]
pub(crate) enum GroupSlot {
    Linker,
    Edge,
    Lto,
    Incremental,
    Backend,
    Driver,
    Cc,
}

impl GroupSlot {
    /// The bit of this place.
    pub(crate) const fn bit(self) -> Mask {
        1 << self as Mask
    }
}

/// What a pragma reaches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reach {
    /// The crate whose script names the set: the flags the wrapper
    /// appends to its rustc, the linker's among them, and its cfg.
    Crate,
    /// Cargo's configuration, which cargo reads before any script runs
    /// and applies to every crate it builds.
    Workspace,
}

impl Reach {
    /// The wider of the two.
    pub const fn widest(self, other: Reach) -> Reach {
        match (self, other) {
            (Reach::Crate, Reach::Crate) => Reach::Crate,
            _ => Reach::Workspace,
        }
    }
}

/// Whether the keys a pragma's group shares across profiles are still to
/// be written.
///
/// A key that belongs to no profile is in the generated cargo file once,
/// however many profiles' sets hold its pragma.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Shared {
    /// No profile before this one wrote them, so this writer does.
    Owed,
    /// A profile before this one wrote them, so this writer leaves them.
    Written,
}

/// What a pragma's writer is handed.
#[derive(Debug, Clone, Copy)]
pub struct Written<'a> {
    profile: &'a str, // lint:allow(no-bare-string) reason: the profile's name as cargo has it; tracked: #72
    triple:  &'a str, // lint:allow(no-bare-string) reason: the target triple as the build script read it; tracked: #72
    tool:    Maybe<&'a str>, // lint:allow(no-bare-string) reason: a path as the build script found it; tracked: #72
    shared:  Shared,
}

impl<'a> Written<'a> {
    /// What a writer is handed, from its four parts.
    pub const fn new(
        profile: &'a str, // lint:allow(no-bare-string) reason: the profile's name as cargo has it; tracked: #72
        triple: &'a str, // lint:allow(no-bare-string) reason: the target triple as the build script read it; tracked: #72
        tool: Maybe<&'a str>, // lint:allow(no-bare-string) reason: a path as the build script found it; tracked: #72
        shared: Shared,
    ) -> Self {
        Written {
            profile,
            triple,
            tool,
            shared,
        }
    }

    /// The profile's name as cargo has it: `dev`, `release`.
    #[rustfmt::skip] // keeps the allow on the signature it governs
    pub const fn profile(&self) -> &'a str { // lint:allow(no-bare-string) reason: the profile's name as cargo has it; tracked: #72
        self.profile
    }

    /// The target triple.
    #[rustfmt::skip] // keeps the allow on the signature it governs
    pub const fn triple(&self) -> &'a str { // lint:allow(no-bare-string) reason: the target triple as the build script read it; tracked: #72
        self.triple
    }

    /// Where the pragma's tool is, where the script found one.
    #[rustfmt::skip] // keeps the allow on the signature it governs
    pub const fn tool(&self) -> Maybe<&'a str> { // lint:allow(no-bare-string) reason: a path as the build script found it; tracked: #72
        self.tool
    }

    /// Whether the keys the pragma's group shares across profiles are owed.
    pub const fn shared(&self) -> Shared {
        self.shared
    }
}

/// A choice of which a set holds one value at most.
pub trait Group {
    /// The group's bit, or zero for [`Free`].
    const BIT: Mask;
    /// What is said of a set that holds two of the group.
    const CLASH: Words;
    /// Whether the group's keys belong to no profile, so the sets of a
    /// prototype that claims the workspace hold one value of it in every
    /// profile.
    const SHARED: bool = false; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a build-time fact, the crate takes no arvo dependency; tracked: #72
}

/// The group of a pragma that is one value of no choice.
#[derive(Debug, Clone, Copy)]
pub struct Free;

impl Group for Free {
    const BIT: Mask = 0;
    const CLASH: Words = "";
}

/// What a pragma requires of the set it is in, as three masks.
pub trait Requirement {
    /// Every one of these bits is in the set.
    const ALL: Mask;
    /// At least one of these bits is in the set, where any is named.
    const ANY: Mask;
    /// None of these bits is in the set.
    const NONE: Mask;
}

impl Requirement for () {
    const ALL: Mask = 0;
    const ANY: Mask = 0;
    const NONE: Mask = 0;
}

/// Every pragma of the list is in the set.
#[derive(Debug, Clone, Copy)]
pub struct All<L>(PhantomData<L>);

/// At least one pragma of the list is in the set.
#[derive(Debug, Clone, Copy)]
pub struct Any<L>(PhantomData<L>);

/// No pragma of the list is in the set.
#[derive(Debug, Clone, Copy)]
pub struct Not<L>(PhantomData<L>);

impl<L: Set> Requirement for All<L> {
    const ALL: Mask = L::MASK;
    const ANY: Mask = 0;
    const NONE: Mask = 0;
}

impl<L: Set> Requirement for Any<L> {
    const ALL: Mask = 0;
    const ANY: Mask = L::MASK;
    const NONE: Mask = 0;
}

impl<L: Set> Requirement for Not<L> {
    const ALL: Mask = 0;
    const ANY: Mask = 0;
    const NONE: Mask = L::MASK;
}

/// One build choice.
///
/// The three writers are the routes a pragma's text takes, and each
/// writes nothing unless the pragma says otherwise. Rustc flags are
/// separated by single spaces, with none before the first or after the
/// last, and a linker's argument is one of them, `-Clink-arg=`. Cfgs are
/// one to a line. Cargo configuration
/// is whole lines in dotted keys with no table header, since two pragmas
/// write keys of one table and TOML takes a header once.
pub trait Pragma {
    /// The pragma's name in every sentence said about it.
    const NAME: Words;
    /// The pragma's place in a set's mask.
    const BIT: Mask;
    /// What the pragma reaches.
    const REACH: Reach = Reach::Crate;
    /// What is said of a set that does not meet [`Requires`](Pragma::Requires).
    const UNMET: Words = "";
    /// The tool the pragma waits on, a row of [`TOOLS`](crate::TOOLS).
    const TOOL: Maybe<&'static Tool> = Maybe::Isnt;

    /// The choice the pragma is one value of.
    type Group: Group;
    /// What the pragma requires of the set it is in.
    type Requires: Requirement;

    /// The sentence of a refusal where the target cannot carry the
    /// pragma, or nothing where it can.
    #[allow(unused_variables)]
    fn admitted(target: &Target) -> Maybe<&'static str> {
        Maybe::Isnt
    }

    /// The flags the wrapper appends to the crate's rustc.
    #[allow(unused_variables)]
    fn write_rustflags<W: Write>(with: &Written<'_>, out: &mut W) -> core::fmt::Result {
        Ok(())
    }

    /// The cfgs set for the crate, one to a line.
    #[allow(unused_variables)]
    fn write_cfgs<W: Write>(out: &mut W) -> core::fmt::Result {
        Ok(())
    }

    /// The lines of the generated cargo file, in dotted keys.
    #[allow(unused_variables)]
    fn write_cargo_config<W: Write>(with: &Written<'_>, out: &mut W) -> core::fmt::Result {
        Ok(())
    }
}

/// The folded value of a set: every member's bit.
///
/// Nothing changes one. A set is changed by naming another type.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct PragmaSet {
    mask: Mask,
}

impl PragmaSet {
    /// The folded value of the set `S`.
    pub const fn of<S: Set>() -> Self {
        PragmaSet {
            mask: S::MASK,
        }
    }

    /// Whether the set holds the pragma `P`. A pragma with a value,
    /// `ParallelCodegen<N>`, is held whatever its value.
    #[rustfmt::skip] // keeps the allow on the signature it governs
    pub const fn contains<P: Pragma>(self) -> bool { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a build-time predicate, the crate takes no arvo dependency; tracked: #72
        self.mask & P::BIT != 0
    }
}

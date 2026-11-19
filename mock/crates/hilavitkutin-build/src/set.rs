//! A set of pragmas is a flat tuple of their types.
//!
//! [`Set`] folds its members into constants, so what is wrong with a set
//! is known where the set is named: [`checked`] and [`crate_reach`] are
//! the assertions a script's `const` items make, which a check evaluates.
//! A set also walks its members, handing each as a type to a [`Visit`].

use notko::Maybe;

use crate::pragma::{Group, Mask, Pragma, Reach, Requirement, Shared, Words, Written};
use crate::profile::Profile;
use crate::tools::Tool;

/// What a walk over a set does with one member.
pub trait Visit {
    /// Take the member `P`.
    fn pragma<P: Pragma>(&mut self) -> core::fmt::Result;
}

/// A set of pragmas.
pub trait Set {
    /// Every member's bit.
    const MASK: Mask;
    /// Every member's group.
    const GROUPS: Mask;
    /// The bits of the members whose group is shared across profiles.
    const SHARED: Mask;
    /// The widest reach among the members.
    const REACH: Reach;
    /// The first thing wrong with the set, as the sentence of the pragma
    /// or the group it is wrong with, or the empty string.
    const VERDICT: Words;

    /// Hand every member to `visit`, in the order the set names them.
    fn each<V: Visit>(visit: &mut V) -> core::fmt::Result;
}

/// What is said of a set that names one pragma twice.
const TWICE: &str = "the set names one pragma twice"; // lint:allow(no-bare-static-str) reason: a build-dependency only, it reaches no binary; tracked: no_bare_static_str_does_not_know_a_build_dependency_only_cra

/// One member of a set, as the fold reads it.
struct Row {
    bit:   Mask,
    group: Mask,
    clash: &'static str,
    all:   Mask,
    any:   Mask,
    none:  Mask,
    unmet: &'static str,
}

impl Row {
    const fn of<P: Pragma>() -> Row {
        Row {
            bit:   P::BIT,
            group: <P::Group as Group>::BIT,
            clash: <P::Group as Group>::CLASH,
            all:   <P::Requires as Requirement>::ALL,
            any:   <P::Requires as Requirement>::ANY,
            none:  <P::Requires as Requirement>::NONE,
            unmet: P::UNMET,
        }
    }
}

/// The first thing wrong with the rows of a set, or the empty string.
///
/// A pragma named twice comes first, then a group held twice, then a
/// requirement the whole set does not meet, each at the first member it
/// shows at.
const fn verdict(rows: &[Row]) -> &'static str {
    let mut mask = 0;
    let mut at = 0;
    while at < rows.len() {
        mask |= rows[at].bit;
        at += 1;
    }
    let mut seen = 0;
    let mut groups = 0;
    at = 0;
    while at < rows.len() {
        let row = &rows[at];
        if row.bit & seen != 0 {
            return TWICE;
        }
        seen |= row.bit;
        if row.group & groups != 0 {
            return row.clash;
        }
        groups |= row.group;
        let all_held = row.all & mask == row.all;
        let any_held = row.any == 0 || row.any & mask != 0;
        let none_held = row.none & mask == 0;
        if !(all_held && any_held && none_held) {
            return row.unmet;
        }
        at += 1;
    }
    ""
}

/// The bit of `P` where its group is shared across profiles, or zero.
const fn shared_bit<P: Pragma>() -> Mask {
    if <P::Group as Group>::SHARED { P::BIT } else { 0 }
}

macro_rules! set {
    ($($p:ident),*) => {
        impl<$($p: Pragma),*> Set for ($($p,)*) {
            const MASK: Mask = 0 $(| $p::BIT)*;
            const GROUPS: Mask = 0 $(| <$p::Group as Group>::BIT)*;
            const SHARED: Mask = 0 $(| shared_bit::<$p>())*;
            const REACH: Reach = Reach::Crate $(.widest($p::REACH))*;
            const VERDICT: Words = {
                let rows: &[Row] = &[$(Row::of::<$p>()),*];
                verdict(rows)
            };

            #[allow(unused_variables)]
            fn each<V: Visit>(visit: &mut V) -> core::fmt::Result {
                $(
                    visit.pragma::<$p>()?;
                )*
                Ok(())
            }
        }
    };
}

macro_rules! sets {
    () => {
        set!();
    };
    ($head:ident $(, $tail:ident)*) => {
        set!($head $(, $tail)*);
        sets!($($tail),*);
    };
}

sets!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);

/// The assertion a script's `const` item makes of its set: nothing is
/// wrong with it. Where something is, the build stops at that item with
/// the set's verdict.
pub const fn checked<S: Set>() {
    assert!(S::VERDICT.is_empty(), "{}", S::VERDICT);
}

/// The assertion of a script that does not claim the workspace: its set
/// holds no pragma of workspace reach.
pub const fn crate_reach<S: Set>() {
    assert!(
        matches!(S::REACH, Reach::Crate),
        "the set holds a pragma that reaches the whole workspace, which only the script written \
         `script!(<prototype>, workspace)` may name"
    );
}

/// The walk that hands over every member's tool.
struct Tools<F: FnMut(&'static Tool)> {
    each: F,
}

impl<F: FnMut(&'static Tool)> Visit for Tools<F> {
    fn pragma<P: Pragma>(&mut self) -> core::fmt::Result {
        if let Maybe::Is(tool) = P::TOOL {
            (self.each)(tool);
        }
        Ok(())
    }
}

/// Hand `each` the tool of every member of the set `S` that names one,
/// in the order the set names its members. Two members that wait on one
/// tool hand it over twice.
pub fn each_tool<S: Set>(each: impl FnMut(&'static Tool)) {
    // The walk's only failure is a writer's, and this walk writes nothing.
    let _ = S::each(&mut Tools {
        each,
    });
}

/// A tool and where the script found it.
#[derive(Debug, Clone, Copy)]
pub struct ToolPath<'a> {
    tool: &'static Tool,
    path: &'a str, // lint:allow(no-bare-string) reason: a path as the build script found it; tracked: #72
}

impl<'a> ToolPath<'a> {
    /// The tool at `path`.
    pub const fn new(
        tool: &'static Tool,
        path: &'a str, // lint:allow(no-bare-string) reason: a path as the build script found it; tracked: #72
    ) -> Self {
        ToolPath {
            tool,
            path,
        }
    }
}

/// What a walk over a set is handed: the profile, the target triple and
/// the tools the script found. The walk makes every member its
/// [`Written`] from it.
#[derive(Debug, Clone, Copy)]
pub struct Site<'a> {
    profile: Profile,
    triple:  &'a str, // lint:allow(no-bare-string) reason: the target triple as the build script read it; tracked: #72
    tools:   &'a [ToolPath<'a>],
    mask:    Mask,
    groups:  Mask,
}

impl<'a> Site<'a> {
    /// The site of one profile's set.
    pub const fn new(
        profile: Profile,
        triple: &'a str, // lint:allow(no-bare-string) reason: the target triple as the build script read it; tracked: #72
        tools: &'a [ToolPath<'a>],
    ) -> Self {
        Site {
            profile,
            triple,
            tools,
            mask: 0,
            groups: 0,
        }
    }

    /// This site once the set `S` has been written: every member of `S`
    /// and every group of `S` has its shared keys in the file already.
    pub const fn after<S: Set>(mut self) -> Self {
        self.mask |= S::MASK;
        self.groups |= S::GROUPS;
        self
    }

    /// The profile.
    pub const fn profile(&self) -> Profile {
        self.profile
    }

    /// Where `tool` is, where the script found it.
    #[rustfmt::skip] // keeps the allow on the signature it governs
    pub fn path_of(&self, tool: &Tool) -> Maybe<&'a str> { // lint:allow(no-bare-string) reason: a path as the build script found it; tracked: #72
        for found in self.tools {
            if found.tool.name == tool.name {
                return Maybe::Is(found.path);
            }
        }
        Maybe::Isnt
    }

    /// What the writers of the member `P` are handed.
    ///
    /// The shared keys count as written where a set before this one held
    /// a pragma of the same group, or held this pragma where it is of no
    /// group.
    pub fn written<P: Pragma>(&self) -> Written<'a> {
        let group = <P::Group as Group>::BIT;
        let before = if group == 0 { self.mask & P::BIT } else { self.groups & group };
        let shared = if before == 0 { Shared::Owed } else { Shared::Written };
        let tool = match P::TOOL {
            Maybe::Is(tool) => self.path_of(tool),
            Maybe::Isnt => Maybe::Isnt,
        };
        Written::new(self.profile.name(), self.triple, tool, shared)
    }
}

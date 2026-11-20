//! A pragma as a unit type, a set as a flat tuple of them, and the set's
//! requirements folded into one constant a const item asserts over.
#![no_std]

use core::fmt::{self, Write};
use core::marker::PhantomData;

/// A choice of which a set holds one value at most.
pub trait Group {
    /// The group's bit, or zero for a pragma in no group.
    const BIT: u64;
    /// What is said of a set that holds two of the group.
    const CLASH: &'static str;
}

/// The group of a pragma that is one value of no choice.
pub struct Free;

impl Group for Free {
    const BIT: u64 = 0;
    const CLASH: &'static str = "";
}

/// What a pragma needs of the set it is in.
pub trait Requirement {
    /// Every one of these bits is in the set.
    const ALL: u64;
    /// One of these bits is in the set, where any is asked.
    const ANY: u64;
}

impl Requirement for () {
    const ALL: u64 = 0;
    const ANY: u64 = 0;
}

/// Every pragma of the list is in the set.
pub struct All<L>(PhantomData<L>);

/// At least one pragma of the list is in the set.
pub struct Any<L>(PhantomData<L>);

impl<L: Set> Requirement for All<L> {
    const ALL: u64 = L::MASK;
    const ANY: u64 = 0;
}

impl<L: Set> Requirement for Any<L> {
    const ALL: u64 = 0;
    const ANY: u64 = L::MASK;
}

/// One build choice.
pub trait Pragma {
    const NAME: &'static str;
    const BIT: u64;
    /// Whether the pragma reaches the whole workspace, through cargo's
    /// configuration, and so belongs to the workspace's one script.
    const WORKSPACE: bool = false;
    /// What is said of a set that does not meet `Requires`.
    const UNMET: &'static str = "";
    type Group: Group;
    type Requires: Requirement;
    fn write_flags<W: Write>(out: &mut W) -> fmt::Result;
}

/// One pragma of a set, as the fold reads it.
pub struct Row {
    bit:   u64,
    group: u64,
    clash: &'static str,
    all:   u64,
    any:   u64,
    unmet: &'static str,
}

impl Row {
    pub const fn of<P: Pragma>() -> Row {
        Row {
            bit:   P::BIT,
            group: <P::Group as Group>::BIT,
            clash: <P::Group as Group>::CLASH,
            all:   <P::Requires as Requirement>::ALL,
            any:   <P::Requires as Requirement>::ANY,
            unmet: P::UNMET,
        }
    }
}

/// The first thing wrong with a set, or the empty string.
pub const fn verdict(rows: &[Row]) -> &'static str {
    let mut mask = 0;
    let mut at = 0;
    while at < rows.len() {
        mask |= rows[at].bit;
        at += 1;
    }
    let mut groups = 0;
    at = 0;
    while at < rows.len() {
        let row = &rows[at];
        if row.group & groups != 0 {
            return row.clash;
        }
        groups |= row.group;
        if row.all & mask != row.all || (row.any != 0 && row.any & mask == 0) {
            return row.unmet;
        }
        at += 1;
    }
    ""
}

/// A set of pragmas.
pub trait Set {
    const MASK: u64;
    const WORKSPACE: bool;
    const VERDICT: &'static str;
    fn write_flags<W: Write>(out: &mut W) -> fmt::Result;
}

macro_rules! set {
    ($($p:ident),*) => {
        impl<$($p: Pragma),*> Set for ($($p,)*) {
            const MASK: u64 = 0 $(| $p::BIT)*;
            const WORKSPACE: bool = false $(|| $p::WORKSPACE)*;
            const VERDICT: &'static str = {
                let rows: &[Row] = &[$(Row::of::<$p>()),*];
                verdict(rows)
            };
            #[allow(unused_variables)]
            fn write_flags<W: Write>(out: &mut W) -> fmt::Result {
                $(
                    $p::write_flags(out)?;
                )*
                Ok(())
            }
        }
    };
}

set!();
set!(A);
set!(A, B);
set!(A, B, C);
set!(A, B, C, D);
set!(A, B, C, D, E);

/// The assertion a script's const item makes of its set.
pub const fn checked<S: Set>() {
    assert!(S::VERDICT.is_empty(), "{}", S::VERDICT);
}

/// The assertion the script of a crate that is not the workspace's makes.
pub const fn crate_reach<S: Set>() {
    assert!(
        !S::WORKSPACE,
        "the set holds a pragma that reaches the whole workspace, and this script is not the workspace's"
    );
}

pub struct LinkerGroup;

impl Group for LinkerGroup {
    const BIT: u64 = 1;
    const CLASH: &'static str = "the set holds two linkers";
}

macro_rules! pragma {
    ($name:ident, $bit:expr, $group:ty, $requires:ty, $workspace:expr, $unmet:expr, $flag:expr) => {
        pub struct $name;

        impl Pragma for $name {
            type Group = $group;
            type Requires = $requires;

            const BIT: u64 = 1 << $bit;
            const NAME: &'static str = stringify!($name);
            const UNMET: &'static str = $unmet;
            const WORKSPACE: bool = $workspace;

            fn write_flags<W: Write>(out: &mut W) -> fmt::Result {
                out.write_str($flag)
            }
        }
    };
}

pragma!(FastMath, 0, Free, (), false, "", "fast-math ");
pragma!(
    MathPeephole,
    1,
    Free,
    All<(FastMath,)>,
    false,
    "MathPeephole stands on FastMath, which the set does not hold",
    "math-peephole "
);
pragma!(Pgo, 2, Free, (), false, "", "pgo ");
pragma!(Bolt, 3, Free, (), false, "", "bolt ");
pragma!(
    Profiling,
    4,
    Free,
    Any<(Pgo, Bolt)>,
    false,
    "Profiling stands on Pgo or Bolt, and the set holds neither",
    "profiling "
);
pragma!(Mold, 5, LinkerGroup, (), false, "", "mold ");
pragma!(Wild, 6, LinkerGroup, (), false, "", "wild ");
pragma!(ThinLto, 7, Free, (), true, "", "");

/// A pragma with a value.
pub struct ParallelCodegen<const N: u8>;

impl<const N: u8> Pragma for ParallelCodegen<N> {
    type Group = Free;
    type Requires = ();

    const BIT: u64 = 1 << 8;
    const NAME: &'static str = "ParallelCodegen";

    fn write_flags<W: Write>(out: &mut W) -> fmt::Result {
        write!(out, "-Zthreads={N} ")
    }
}

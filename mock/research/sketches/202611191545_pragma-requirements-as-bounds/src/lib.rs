//! Sketch: a pragma is a unit type, a set of them is a cons list, and the
//! three requirements (all of, any of, one per group) are checked before
//! the build script runs.
//!
//! Two routes sit side by side. `bounds` states the requirements as trait
//! bounds over the list. `mask` folds the list into constants and asserts
//! over them in a const block. The examples hold one coherent set and one
//! set per broken requirement for each route.

#![no_std]

pub mod bounds;
pub mod mask;

/// What every pragma is, whichever route checks it.
pub trait Pragma {
    /// The group the pragma is the only member of in a set. A pragma
    /// that excludes nothing names `Free`.
    type Group;
    /// Its position in the mask route.
    const BIT: u64;
    /// The group as a number for the mask route; zero excludes nothing.
    const GROUP: u8;
    /// Pragmas that must all be in the set, as a mask.
    const ALL: u64;
    /// Pragmas of which one must be in the set, as a mask; zero asks for
    /// none.
    const ANY: u64;
    /// What the refusal says when the requirement on this pragma breaks.
    const REFUSAL: &'static str;
    /// The flag text, written into whatever the caller formats into.
    fn write<W: core::fmt::Write>(w: &mut W) -> core::fmt::Result;
}

/// Group markers.
pub struct Free;
pub struct LinkerGroup;
pub struct BackendGroup;

macro_rules! pragma {
    ($name:ident, $group:ty, $gnum:expr, $bit:expr, all = $all:expr, any = $any:expr, $text:expr, $refusal:expr) => {
        pub struct $name;
        impl Pragma for $name {
            type Group = $group;

            const ALL: u64 = $all;
            const ANY: u64 = $any;
            const BIT: u64 = 1 << $bit;
            const GROUP: u8 = $gnum;
            const REFUSAL: &'static str = $refusal;

            fn write<W: core::fmt::Write>(w: &mut W) -> core::fmt::Result {
                w.write_str($text)
            }
        }
    };
}

// The pragmas the crate has today whose requirements `PragmaSet::coherent`
// states, and two linkers and a backend from the round's groups.
pragma!(FastMath, Free, 0, 0, all = 0, any = 0, "-Zfast-math", "");
pragma!(
    MathPeephole,
    Free,
    0,
    1,
    all = 1 << 0,
    any = 0,
    "-Zllvm-plugins=math-peephole",
    "MathPeephole stands on FastMath, which the set does not hold"
);
pragma!(Pgo, Free, 0, 2, all = 0, any = 0, "-Cprofile-use", "");
pragma!(Bolt, Free, 0, 3, all = 0, any = 0, "", "");
pragma!(
    Profiling,
    Free,
    0,
    4,
    all = 0,
    any = (1 << 2) | (1 << 3),
    "-Cprofile-generate",
    "Profiling stands on Pgo or Bolt, and the set holds neither"
);
pragma!(
    Mold,
    LinkerGroup,
    1,
    5,
    all = 0,
    any = 0,
    "-Clink-arg=-fuse-ld=mold",
    "the set holds two linkers"
);
pragma!(
    Wild,
    LinkerGroup,
    1,
    6,
    all = 0,
    any = 0,
    "-Clink-arg=-fuse-ld=wild",
    "the set holds two linkers"
);
pragma!(
    Cranelift,
    BackendGroup,
    2,
    7,
    all = 0,
    any = 0,
    "-Zcodegen-backend=cranelift",
    "the set holds two backends"
);

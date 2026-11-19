//! Route two: the list folds into constants, and a const block asserts
//! over them.

use crate::Pragma;
use crate::bounds::WriteAll;

/// The list as constants.
pub trait Fold {
    /// Every member's bit.
    const MASK: u64;
    /// One bit per group a member is in.
    const GROUPS: u64;
    /// The refusal of the first member that shares a group with a later
    /// one, or the empty string.
    const CLASH: &'static str;
}

impl Fold for () {
    const CLASH: &'static str = "";
    const GROUPS: u64 = 0;
    const MASK: u64 = 0;
}

impl<H: Pragma, T: Fold> Fold for (H, T) {
    const CLASH: &'static str = if H::GROUP != 0 && (T::GROUPS & (1 << H::GROUP)) != 0 {
        H::REFUSAL
    } else {
        T::CLASH
    };
    const GROUPS: u64 = if H::GROUP == 0 { T::GROUPS } else { (1 << H::GROUP) | T::GROUPS };
    const MASK: u64 = H::BIT | T::MASK;
}

/// The refusal of the first member whose `ALL` or `ANY` the set `S` does
/// not meet, or the empty string.
pub trait Against<S: Fold> {
    const UNMET: &'static str;
}

impl<S: Fold> Against<S> for () {
    const UNMET: &'static str = "";
}

impl<S: Fold, H: Pragma, T: Against<S>> Against<S> for (H, T) {
    const UNMET: &'static str = if (H::ALL & S::MASK) != H::ALL {
        H::REFUSAL
    } else if H::ANY != 0 && (H::ANY & S::MASK) == 0 {
        H::REFUSAL
    } else {
        T::UNMET
    };
}

/// A set whose constants are asserted over.
pub trait Checked: Fold + Against<Self> + Sized {
    const OK: () = {
        if !Self::CLASH.is_empty() {
            panic!("{}", Self::CLASH);
        }
        if !<Self as Against<Self>>::UNMET.is_empty() {
            panic!("{}", <Self as Against<Self>>::UNMET);
        }
    };
}

impl<S: Fold + Against<S>> Checked for S {}

/// What a build script would call. The assertion runs when the function
/// is instantiated for a set.
pub fn configure<S, W>(w: &mut W) -> core::fmt::Result
where
    S: Checked + WriteAll,
    W: core::fmt::Write,
{
    const { S::OK };
    S::write_all(w)
}

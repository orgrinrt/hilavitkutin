//! `Lane`, the arithmetic vocabulary a kernel generic over a number is bounded by.
//!
//! The names are arvo's value page's (form D of the exact accumulator): `widen` into the
//! accumulator, `wide_add` and `wide_sub` on it, `narrow` back. Becomes a re-export of
//! `arvo-numeral`'s `Lane` at the swap.

use core::fmt::Debug;

use super::USize;

/// A number a kernel can accumulate exactly and adapt back.
// FIXME(69b1c00469a6): becomes a re-export of arvo-numeral's `Lane` at the swap.
pub trait Lane: Copy + Send + Sync + 'static {
    /// The exact accumulator: twice the lane's width, signed.
    type Wide: Copy + PartialEq + Debug + Send + Sync + 'static;
    /// The empty sum.
    const WIDE_ZERO: Self::Wide;
    /// How many widened members, each added or subtracted, the accumulator holds exactly.
    const MAX_TERMS: USize;
    /// One term of the accumulator.
    fn widen(self) -> Self::Wide;
    /// The exact sum; a debug build asserts it is inside the accumulator.
    fn wide_add(a: Self::Wide, b: Self::Wide) -> Self::Wide;
    /// The exact difference; a debug build asserts it is inside the accumulator.
    fn wide_sub(a: Self::Wide, b: Self::Wide) -> Self::Wide;
    /// The one adaptation back to a lane value.
    fn narrow(wide: Self::Wide) -> Self;
}

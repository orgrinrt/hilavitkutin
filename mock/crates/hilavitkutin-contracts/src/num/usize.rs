//! `USize`, a stand-in for `arvo-numeral`'s platform-width unsigned value.
//!
//! The carrier is private and exposed by no door but the two host doors, `from_host` and `host`,
//! because arvo's carrier is the ladder's (`u64` or `u32`) and differs per target. There is no
//! `raw` and no `from_raw` for the same reason. Arithmetic wraps at the target's width, as arvo's
//! `Adapt<TowardZero, Wrap>` does.

use core::cmp::Ordering;

use super::{Additive, Bool, Identity};

/// A platform-width unsigned value.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct USize(usize);

impl USize {
    /// The door in from a host count.
    pub const fn from_host(n: usize) -> Self {
        Self(n)
    }

    /// The door out to a host count.
    pub const fn host(self) -> usize {
        self.0
    }

    /// The sum, wrapping at the target's width.
    pub const fn add(self, rhs: Self) -> Self {
        Self(self.0.wrapping_add(rhs.0))
    }

    /// The difference, wrapping at the target's width.
    pub const fn sub(self, rhs: Self) -> Self {
        Self(self.0.wrapping_sub(rhs.0))
    }

    /// Whether the two are one value.
    pub const fn eq(self, rhs: Self) -> Bool {
        Bool::of(self.0 == rhs.0)
    }

    /// The order of the two.
    pub const fn cmp(self, rhs: Self) -> Ordering {
        if self.0 < rhs.0 {
            Ordering::Less
        } else if self.0 > rhs.0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    /// Below `rhs`.
    pub const fn lt(self, rhs: Self) -> Bool {
        Bool::of(self.0 < rhs.0)
    }

    /// Not above `rhs`.
    pub const fn le(self, rhs: Self) -> Bool {
        Bool::of(self.0 <= rhs.0)
    }

    /// Above `rhs`.
    pub const fn gt(self, rhs: Self) -> Bool {
        Bool::of(self.0 > rhs.0)
    }

    /// Not below `rhs`.
    pub const fn ge(self, rhs: Self) -> Bool {
        Bool::of(self.0 >= rhs.0)
    }

    /// The smaller.
    pub const fn min(self, rhs: Self) -> Self {
        if self.0 <= rhs.0 { self } else { rhs }
    }

    /// The larger.
    pub const fn max(self, rhs: Self) -> Self {
        if self.0 >= rhs.0 { self } else { rhs }
    }
}

impl Identity<Additive> for USize {
    const IDENTITY: Self = Self(0);
}

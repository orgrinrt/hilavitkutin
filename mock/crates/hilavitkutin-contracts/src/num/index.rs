//! `Index`, `Count`, `Ordinal` and `ColumnNo`, this crate's own: a position among records, a
//! number of records, the place of an item in a list of plain data, and the number of a stored
//! column.
//!
//! Four distinct types over one `USize`, so a count passed where an index goes, or a binding's
//! place passed where a record's goes, is refused at build in every implementor. Only the
//! arithmetic that keeps its meaning exists: an index after a count is an index, an index since an
//! index is a count, a count plus a count is a count, and a count less a count is a count or
//! nothing below zero.

use core::cmp::Ordering;

use notko::Maybe;

use super::{Additive, Bool, Identity, USize};

/// A position among the records of one store.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Index(USize);

/// A number of records, morsels or frames.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Count(USize);

/// The place of an item in a list of plain data: a binding in its table, a role or a coefficient in
/// its binding, a run among the runs a law compares.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Ordinal(USize);

/// The number of a stored column of a store, which a binding names and a store finds a column by.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ColumnNo(USize);

/// The comparisons both types share, each over the one carrier beneath.
macro_rules! compared {
    ($t:ident) => {
        impl $t {
            /// The door in from a host count.
            pub const fn from_host(n: usize) -> Self {
                Self(USize::from_host(n))
            }

            /// The door out to a host count.
            pub const fn host(self) -> usize {
                self.0.host()
            }

            /// Whether the two are one value.
            pub const fn eq(self, rhs: Self) -> Bool {
                self.0.eq(rhs.0)
            }

            /// The order of the two.
            pub const fn cmp(self, rhs: Self) -> Ordering {
                self.0.cmp(rhs.0)
            }

            /// Below `rhs`.
            pub const fn lt(self, rhs: Self) -> Bool {
                self.0.lt(rhs.0)
            }

            /// Not above `rhs`.
            pub const fn le(self, rhs: Self) -> Bool {
                self.0.le(rhs.0)
            }

            /// Above `rhs`.
            pub const fn gt(self, rhs: Self) -> Bool {
                self.0.gt(rhs.0)
            }

            /// Not below `rhs`.
            pub const fn ge(self, rhs: Self) -> Bool {
                self.0.ge(rhs.0)
            }

            /// The smaller.
            pub const fn min(self, rhs: Self) -> Self {
                Self(self.0.min(rhs.0))
            }

            /// The larger.
            pub const fn max(self, rhs: Self) -> Self {
                Self(self.0.max(rhs.0))
            }
        }
    };
}

compared!(Index);
compared!(Count);
compared!(Ordinal);
compared!(ColumnNo);

impl Ordinal {
    /// The first place.
    pub const FIRST: Self = Self(USize::from_host(0));

    /// The next place.
    pub const fn next(self) -> Self {
        Self(self.0.add(USize::from_host(1)))
    }

    /// The place `by` places after this one.
    pub const fn after(self, by: Count) -> Self {
        Self(self.0.add(by.0))
    }
}

impl Index {
    /// The first position.
    pub const FIRST: Self = Self(USize::from_host(0));

    /// The position `by` records after this one.
    pub const fn after(self, by: Count) -> Self {
        Self(self.0.add(by.0))
    }

    /// The next position.
    pub const fn next(self) -> Self {
        self.after(Count::ONE)
    }

    /// The position `by` records before this one, or nothing below the first.
    pub const fn before(self, by: Count) -> Maybe<Self> {
        if by.0.le(self.0).get() {
            Maybe::Is(Self(self.0.sub(by.0)))
        } else {
            Maybe::Isnt
        }
    }

    /// How many records lie from `earlier` up to this one; nothing when `earlier` is past it.
    pub const fn since(self, earlier: Self) -> Maybe<Count> {
        if earlier.0.le(self.0).get() {
            Maybe::Is(Count(self.0.sub(earlier.0)))
        } else {
            Maybe::Isnt
        }
    }

    /// Whether this position lies among the first `extent` records.
    pub const fn within(self, extent: Count) -> Bool {
        self.0.lt(extent.0)
    }
}

impl Count {
    /// One record.
    pub const ONE: Self = Self(USize::from_host(1));
    /// No records.
    pub const ZERO: Self = Self(USize::from_host(0));

    /// The sum, wrapping at the target's width.
    pub const fn add(self, rhs: Self) -> Self {
        Self(self.0.add(rhs.0))
    }

    /// The difference, or nothing where `rhs` is the larger.
    pub const fn sub(self, rhs: Self) -> Maybe<Self> {
        if rhs.0.le(self.0).get() {
            Maybe::Is(Self(self.0.sub(rhs.0)))
        } else {
            Maybe::Isnt
        }
    }

    /// One more.
    pub const fn next(self) -> Self {
        self.add(Self::ONE)
    }

    /// The position one past the last of this many records, which is the count read as an index.
    pub const fn end(self) -> Index {
        Index(self.0)
    }
}

impl Identity<Additive> for Count {
    const IDENTITY: Self = Self::ZERO;
}

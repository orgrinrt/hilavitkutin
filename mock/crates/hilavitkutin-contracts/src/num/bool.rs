//! `Bool`, a stand-in for `arvo_format::Bool`; replaced by a re-export of it at the swap.

/// A truth value. The doors are `of` and `get`; the operations `and`, `or` and `not`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Bool(bool);

impl Bool {
    /// False.
    pub const FALSE: Self = Self(false);
    /// True.
    pub const TRUE: Self = Self(true);

    /// The door in: a host truth value.
    pub const fn of(host: bool) -> Self {
        Self(host)
    }

    /// The door out: the host truth value.
    pub const fn get(self) -> bool {
        self.0
    }

    /// Both.
    pub const fn and(self, rhs: Self) -> Self {
        Self(self.0 && rhs.0)
    }

    /// Either.
    pub const fn or(self, rhs: Self) -> Self {
        Self(self.0 || rhs.0)
    }

    /// The other one.
    pub const fn not(self) -> Self {
        Self(!self.0)
    }
}

//! `Value`, what a column holds, and `Cell`, its stored form.
//!
//! A store keeps each record of a column as one `Word`, eight little-endian bytes, and a value's
//! cell is read from and written to that word by safe code. `Value::from_cell` is total, every bit
//! pattern naming a value, so a cell read under the wrong value type is a wrong value and never
//! undefined behaviour; the typed column handle keeps that from happening in the first place.

use crate::digest::Absorb;
use crate::num::USize;

/// Eight little-endian bytes: the widest cell, and the unit a store keeps a record's value in.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Word([u8; 8]);

impl Word {
    /// All bytes zero.
    pub const ZERO: Self = Self([0; 8]);

    /// The word of these bytes.
    pub const fn of(bytes: [u8; 8]) -> Self {
        Self(bytes)
    }

    /// The bytes, little-endian.
    pub const fn bytes(self) -> [u8; 8] {
        self.0
    }
}

/// The format a value is in: plain data the bind compares.
///
/// Only this crate makes one, one for each value type it declares, so no two value types share a
/// format and a binding cannot pass one value's cells off as another's.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FormatId(USize);

impl FormatId {
    /// The format numbered `code`. Unlocked: the representation settles at the storage step.
    pub(crate) const fn new(code: USize) -> Self {
        Self(code)
    }
}

pub(crate) mod sealed {
    pub(crate) trait Sealed {}
}

/// A stored form: one of a closed set of fixed widths.
#[allow(private_bounds)]
pub trait Cell: Copy + Send + Sync + 'static + sealed::Sealed {
    /// The cell in the low bytes of `word`.
    fn from_word(word: Word) -> Self;
    /// The word holding this cell in its low bytes, the rest zero.
    fn word(self) -> Word;
}

macro_rules! cells {
    ($($name:ident $n:literal $doc:literal,)*) => {$(
        #[doc = $doc]
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub struct $name([u8; $n]);

        impl $name {
            /// The cell of these bytes, little-endian.
            pub const fn of(bytes: [u8; $n]) -> Self {
                Self(bytes)
            }

            /// The bytes, little-endian.
            pub const fn bytes(self) -> [u8; $n] {
                self.0
            }
        }

        impl sealed::Sealed for $name {}

        impl Cell for $name {
            fn from_word(word: Word) -> Self {
                let all = word.bytes();
                let mut out = [0u8; $n];
                let mut i = 0;
                while i < $n {
                    out[i] = all[i];
                    i += 1;
                }
                Self(out)
            }

            fn word(self) -> Word {
                let mut all = [0u8; 8];
                let mut i = 0;
                while i < $n {
                    all[i] = self.0[i];
                    i += 1;
                }
                Word(all)
            }
        }
    )*};
}

cells! {
    Cell8 1 "A one-byte cell.",
    Cell16 2 "A two-byte cell.",
    Cell32 4 "A four-byte cell.",
    Cell64 8 "An eight-byte cell.",
}

/// What a column holds: a format's value.
///
/// Sealed: the value types are this crate's, each with its own `FormatId`. A new format is an edit
/// here, which is what keeps the formats distinct.
#[allow(private_bounds)]
pub trait Value: Copy + Absorb + Send + Sync + 'static + sealed::Sealed {
    /// Its stored form.
    type Cell: Cell;
    /// Its format, which the bind compares.
    const FORMAT: FormatId;
    /// The value a cell names. Total: every bit pattern names a value.
    fn from_cell(cell: Self::Cell) -> Self;
    /// Its stored form.
    fn cell(self) -> Self::Cell;
}

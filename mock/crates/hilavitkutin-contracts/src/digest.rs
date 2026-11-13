//! `Digest`, the fold a run's state is compared by, and `Absorb`, the one way into it.
//!
//! The fold is the cross-target sketch's: the accumulator starts at `0xCBF29CE484222325` and each
//! word folds in as `m = (acc ^ w) * 0x9E3779B97F4A7C15` over 128 bits, `acc = low(m) ^ high(m)`.
//! Every word is eight little-endian bytes, so nothing that enters depends on the target's pointer
//! width. A count or an index enters as one word; a proxy number as its slot index in two. The
//! encoding is unlocked until the journal format locks.

use crate::domain::Word;
use crate::num::{Count, Index, USize};

/// The running fold.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Digest(u64);

impl Digest {
    /// The fold before any word.
    pub const fn new() -> Self {
        Self(0xCBF2_9CE4_8422_2325)
    }

    /// Folds one word in.
    pub const fn word(&mut self, word: Word) {
        let w = u64::from_le_bytes(word.bytes());
        let m = ((self.0 ^ w) as u128).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        self.0 = (m as u64) ^ ((m >> 64) as u64);
    }

    /// The fold as one word, for printing and comparing outside the crate.
    pub const fn value(self) -> Word {
        Word::of(self.0.to_le_bytes())
    }
}

/// A value that enters a digest, as fixed-width little-endian words, the same on every target.
pub trait Absorb {
    /// Folds this value into `into`.
    fn absorb(self, into: &mut Digest);
}

impl Absorb for USize {
    fn absorb(self, into: &mut Digest) {
        into.word(Word::of((self.host() as u64).to_le_bytes()));
    }
}

impl Absorb for Count {
    fn absorb(self, into: &mut Digest) {
        USize::from_host(self.host()).absorb(into);
    }
}

impl Absorb for Index {
    fn absorb(self, into: &mut Digest) {
        USize::from_host(self.host()).absorb(into);
    }
}

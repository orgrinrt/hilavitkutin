//! The digest: a fold of 64-bit words, the same on every target.
//!
//! The fold is FNV-1a over 64 bits, from `arvo-hash`, over each word's bytes
//! least significant first, so neither the host's byte sequence nor its
//! pointer width enters.

use arvo::{Bits, Hot};
use arvo_hash::{Fnv1a, Hasher};

/// What every recorded run is made of: 64 bits.
pub type Word = Bits<64, Hot>;

/// A digest being folded.
///
/// ```
/// use hilavitkutin_conformance::{Fold, Word};
///
/// let mut one = Fold::new();
/// one.word(Word::from_raw(7));
/// let mut other = Fold::new();
/// other.word(Word::from_raw(8));
/// assert!(one.finish().to_raw() != other.finish().to_raw());
/// ```
pub struct Fold(Fnv1a<64>);

impl Fold {
    /// A fold that has taken no word.
    pub const fn new() -> Self {
        Fold(Fnv1a::<64>::new())
    }

    /// Fold one word in.
    pub fn word(&mut self, word: Word) {
        self.0.update(&word.to_raw().to_le_bytes());
    }

    /// Fold every word of `words` in, in the sequence given.
    pub fn words(&mut self, words: &[Word]) {
        for word in words {
            self.word(*word);
        }
    }

    /// The digest of the words folded so far.
    pub fn finish(self) -> Word {
        self.0.finalize()
    }
}

impl Default for Fold {
    fn default() -> Self {
        Self::new()
    }
}

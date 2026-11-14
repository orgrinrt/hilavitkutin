//! The digest: a fold of 64-bit words, the same on every target.
//!
//! The fold is `arvo-hash`'s FNV-1a over the words' bytes, least significant
//! byte first, so nothing of the host's byte sequence or pointer width enters.

use arvo::{Bits, Hot};
use arvo_hash::{ContentHash, Fnv1a, Hasher};

/// A digest being folded.
pub struct Fold(Fnv1a<64>);

impl Fold {
    /// A fold that has taken no word.
    pub const fn new() -> Self {
        Fold(Fnv1a::<64>::new())
    }

    /// Fold one word in.
    pub fn word(&mut self, word: Bits<64, Hot>) {
        self.0.update(&word.to_raw().to_le_bytes());
    }

    /// The digest of the words folded so far.
    pub fn finish(self) -> ContentHash {
        self.0.finalize()
    }
}

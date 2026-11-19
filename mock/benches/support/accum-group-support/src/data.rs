//! The input column, built from a seed.

use crate::law::Sat;

/// `n` values spread over the whole range of the law, so most folds
/// saturate somewhere. A linear congruential sequence started from `seed`:
/// the same seed and `n` give the same column on every run and every target.
pub fn column(seed: u64, n: usize) -> Vec<Sat> {
    // Fold the seed into the 32-bit state; the low bit is set so a zero seed
    // still walks the full period.
    let mut x = ((seed ^ (seed >> 32)) as u32) | 1;
    (0 .. n)
        .map(|_| {
            x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            Sat::from_raw(((x >> 24) as i16) - 128)
        })
        .collect()
}

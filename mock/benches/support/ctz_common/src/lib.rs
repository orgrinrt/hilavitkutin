//! The loop the `bit_count_ops` arms share, and the checks each arm's count
//! has to pass.
//!
//! The arms once counted `(v | 1).trailing_zeros()`, which is zero for every
//! word, so all three timed a constant and every median was 0 ns. The checks
//! here keep that from happening again: an arm's count agrees with
//! `u64::trailing_zeros` on words with every count from 0 to 64, and the arm's
//! whole kernel gives a different result for different input. The old form
//! fails both, and the tests below show it failing.

/// The sum of `ctz` over the `N / 8` little-endian words of `input`.
#[inline(always)]
pub fn sum_trailing_zeros<const N: usize>(input: &[u8; N], ctz: impl Fn(u64) -> u32) -> u64 {
    let mut acc: u64 = 0;
    for word in input.chunks_exact(8) {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(word);
        acc = acc.wrapping_add(u64::from(ctz(u64::from_le_bytes(bytes))));
    }
    acc
}

/// Whether `ctz` gives `u64::trailing_zeros` on a word with each count from
/// 0 to 64, with the high bits set and clear, and on zero.
pub fn agrees_with_std(ctz: impl Fn(u64) -> u32) -> Result<(), String> {
    let mut words = vec![0u64, u64::MAX, 0x8000_0000_0000_0000];
    for k in 0 .. 64 {
        words.push(1u64 << k);
        words.push(u64::MAX << k);
        words.push(0xA5A5_A5A5_A5A5_A5A5u64 << k);
    }
    for v in words {
        let (got, want) = (ctz(v), v.trailing_zeros());
        if got != want {
            return Err(format!(
                "ctz({v:#018x}) is {got}, u64::trailing_zeros says {want}"
            ));
        }
    }
    Ok(())
}

pub use kernel_checks::varies;

#[cfg(test)]
mod tests {
    use super::*;

    /// The form every arm used before, kept as the control.
    fn constant_form(v: u64) -> u32 {
        (v | 1).trailing_zeros()
    }

    #[test]
    fn the_std_count_passes_both_checks() {
        agrees_with_std(u64::trailing_zeros).unwrap();
        assert!(varies(|i| sum_trailing_zeros(i, u64::trailing_zeros)));
    }

    #[test]
    fn the_old_constant_form_fails_both_checks() {
        assert!(agrees_with_std(constant_form).is_err());
        assert!(!varies(|i| sum_trailing_zeros(i, constant_form)));
    }
}

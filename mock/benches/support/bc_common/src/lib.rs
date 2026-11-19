//! What the `bounds_check` arms share.
//!
//! The arms used to index a `&[u8; N]` with `0 .. N`, where the compiler
//! proves every index in bounds in all three, and they ran a serial FNV
//! multiply chain whose three-to-four-cycle latency per byte would hide a
//! one-cycle compare even where it stayed. So the bench could not show a
//! bounds check at all, and the claim drawn from it (that safe indexing is
//! elided completely) rested on nothing it measured.
//!
//! Now every arm reads the input through `opaque`, a slice whose length the
//! compiler cannot see, so an index from `0 .. N` is no longer provably in
//! bounds and `input[i]` keeps whatever check the compiler cannot hoist. The
//! body is a sum of independent adds, throughput-bound and vectorisable where
//! nothing stops it, so a check that stays shows in the time.

use core::hint::black_box;

/// `input` as a slice whose length is hidden from the optimiser.
#[inline(always)]
pub fn opaque<const N: usize>(input: &[u8; N]) -> &[u8] {
    black_box(&input[..])
}

/// The sum of the bytes, which every arm computes.
pub fn reference(input: &[u8]) -> u64 {
    input.iter().map(|b| u64::from(*b)).sum()
}

/// Whether `kernel` computes `reference` on every check input and its result
/// depends on the input.
pub fn check(kernel: impl Fn(&[u8; 256]) -> u64) -> Result<(), String> {
    for input in kernel_checks::inputs() {
        let (got, want) = (kernel(&input), reference(&input));
        if got != want {
            return Err(format!("the kernel gives {got}, the byte sum is {want}"));
        }
    }
    if !kernel_checks::varies(&kernel) {
        return Err("the kernel gives one result for every input".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_reference_passes() {
        check(|i| reference(i)).unwrap();
    }

    #[test]
    fn a_kernel_that_skips_a_byte_or_ignores_its_input_fails() {
        assert!(check(|i| reference(&i[1 ..])).is_err());
        assert!(check(|_| 0).is_err());
    }
}

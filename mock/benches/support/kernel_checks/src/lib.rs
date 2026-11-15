//! Whether a microkernel arm's result depends on its input.
//!
//! An arm whose kernel gives one value for every input is timing a constant,
//! which the compiler is free to fold, and its median reads as the cost of
//! nothing. The `bit_count_ops` arms did exactly that until 13 November. The check
//! runs the kernel over inputs that differ in every byte and in the shape of
//! their words, and asks for more than one result.

/// The inputs `varies` runs a kernel over: four 256-byte buffers whose
/// little-endian words differ in value and in trailing-zero count.
pub fn inputs() -> [[u8; 256]; 4] {
    let mut all = [[0u8; 256]; 4];
    for (shift, input) in all.iter_mut().enumerate() {
        for (i, word) in input.chunks_exact_mut(8).enumerate() {
            let v = (i as u64 | 1).wrapping_mul(0x9E37_79B9_7F4A_7C15) << ((i + shift * 7) % 64);
            word.copy_from_slice(&v.to_le_bytes());
        }
    }
    all
}

/// Whether `kernel` gives more than one result over `inputs()`.
pub fn varies(kernel: impl Fn(&[u8; 256]) -> u64) -> bool {
    let results: Vec<u64> = inputs().iter().map(&kernel).collect();
    results.iter().any(|r| *r != results[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_kernel_reading_its_input_varies() {
        assert!(varies(|i| i.iter().map(|b| u64::from(*b)).sum()));
    }

    #[test]
    fn a_constant_kernel_does_not() {
        assert!(!varies(|_| 7));
    }
}

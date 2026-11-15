//! The record values, built from the seed.

/// `n` record values from `seed`, a splitmix64 stream truncated to `u32`.
///
/// Uniform values keep the fold's sum and the append's hash sensitive to
/// every record, so an arm that skips or repeats one disagrees with the rest.
pub fn column(seed: u64, n: usize) -> Vec<u32> {
    let mut x = seed ^ 0x9E37_79B9_7F4A_7C15;
    (0 .. n)
        .map(|_| {
            x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = x;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            (z ^ (z >> 31)) as u32
        })
        .collect()
}

/// FNV-1a over a `u32` sequence, shared with `evs-support`.
pub fn hash(values: &[u32]) -> u64 {
    evs_support::fnv1a_u32(values)
}

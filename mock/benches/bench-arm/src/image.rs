//! What one call receives, how it is seeded, and the digest of what it writes.

/// The input one call receives: a tag and the record column.
///
/// `Routine::Input` has to be `Copy` and flat, and it is the only channel the
/// harness gives a variant, so whatever the variant has to know about its
/// section (a regime, a workload) travels in `tag`. The driver resolves a
/// routine per section and that routine writes the tag; the variant reads it.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TaggedRecords<const N: usize> {
    /// What the section asks of the arm, in the member's own encoding.
    pub tag:     u32,
    /// Keeps `records` eight-byte aligned inside the image.
    pub pad:     u32,
    /// One u32 per record.
    pub records: [u32; N],
}

/// One record of the column from the seed stream.
#[inline]
fn record_from(x: &mut u64) -> u32 {
    *x ^= *x >> 30;
    *x = x.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    (*x >> 32) as u32
}

/// The input for `seed`, by value.
///
/// A routine's `build_input`. The harness reaches `tagged_bytes` instead, and a
/// column of millions of records would not fit on the stack here; the test
/// below holds the two to the same image so they cannot drift while only one
/// is reached by a run.
pub fn tagged_records<const N: usize>(tag: u32, seed: u64) -> TaggedRecords<N> {
    let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    let mut out = TaggedRecords {
        tag,
        pad: 0,
        records: [0u32; N],
    };
    for r in out.records.iter_mut() {
        *r = record_from(&mut x);
    }
    out
}

/// The byte image of `tagged_records::<N>(tag, seed)`, filled directly so a
/// multi-megabyte input never lives on the stack. A routine's
/// `build_input_bytes`.
pub fn tagged_bytes<const N: usize>(tag: u32, seed: u64) -> Vec<u8> {
    let mut buf = vec![0u8; core::mem::size_of::<TaggedRecords<N>>()];
    buf[0 .. 4].copy_from_slice(&tag.to_le_bytes());
    let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    for chunk in buf[8 ..].chunks_mut(4) {
        chunk.copy_from_slice(&record_from(&mut x).to_le_bytes());
    }
    buf
}

/// The least per-call bound any cell gets, 400 milliseconds.
///
/// Forced by the harness on x86_64, not by any workload. Every timing worker
/// calibrates its counter by sleeping three times one second before its first
/// call (`counter_frequency` in `mockspace-bench-core`), and the preflight
/// probe that validation runs is killed after ten times this bound, at least
/// two seconds. Below 400 milliseconds the probe is killed during the
/// calibration and every arm is skipped as slow, whatever it costs.
// FIXME(placeholders_tagged_3ec38cb5c7c8): drop the floor once the harness calibrates its counter
// without a three second sleep per worker, or gives the probe room for it.
pub const MAX_CALL_US_FLOOR: u64 = 400_000;

const FNV_OFFSET: u64 = 0xCBF2_9CE4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01B3;

/// A running FNV-1a hash over the little-endian bytes of u32 records.
#[derive(Copy, Clone)]
pub struct Fnv1a(u64);

impl Fnv1a {
    /// The empty hash.
    pub const fn new() -> Self {
        Self(FNV_OFFSET)
    }

    /// Fold one record in.
    #[inline]
    pub fn push(&mut self, v: u32) {
        for b in v.to_le_bytes() {
            self.0 = (self.0 ^ b as u64).wrapping_mul(FNV_PRIME);
        }
    }

    /// The hash so far.
    pub const fn finish(self) -> u64 {
        self.0
    }
}

impl Default for Fnv1a {
    fn default() -> Self {
        Self::new()
    }
}

/// FNV-1a over the little-endian bytes of a u32 slice.
pub fn fnv1a_u32(vals: &[u32]) -> u64 {
    fnv1a_by(vals, |&v| v)
}

/// FNV-1a over records, each folded in as the u32 `value` reads out of it.
///
/// A column's records are read back as their own type and hashed by the
/// field that holds the value, so the hash names what it reads.
pub fn fnv1a_by<T>(records: &[T], value: impl Fn(&T) -> u32) -> u64 {
    let mut h = Fnv1a::new();
    for r in records {
        h.push(value(r));
    }
    h.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    const RT: usize = 1024;

    #[test]
    fn the_two_builders_give_one_image() {
        let by_value = tagged_records::<RT>(7, 0xABCD);
        let bytes = tagged_bytes::<RT>(7, 0xABCD);
        assert_eq!(bytes.len(), core::mem::size_of::<TaggedRecords<RT>>());
        assert_eq!(
            u32::from_le_bytes(bytes[0 .. 4].try_into().unwrap()),
            by_value.tag
        );
        let records: Vec<u32> = bytes[8 ..]
            .chunks_exact(4)
            .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect();
        assert_eq!(records, by_value.records);
    }

    #[test]
    fn a_seed_changes_the_column_and_a_tag_does_not() {
        let a = tagged_records::<RT>(1, 1);
        assert_ne!(a.records, tagged_records::<RT>(1, 2).records);
        assert_eq!(a.records, tagged_records::<RT>(2, 1).records);
    }

    #[test]
    fn the_digest_is_fnv1a_over_the_bytes() {
        // Literals computed outside this crate, so a wrong constant or a
        // wrong byte order here cannot agree with itself. The empty hash is
        // the published 64-bit offset basis. The two others were computed by
        // an independent FNV-1a in shell arithmetic, checked first against
        // the published digest of "foobar", 0x85944171f73967e8.
        assert_eq!(fnv1a_u32(&[]), 0xCBF2_9CE4_8422_2325);
        assert_eq!(
            fnv1a_u32(&[u32::from_le_bytes(*b"abcd")]),
            0xFC17_9F83_EE07_24DD
        );
        assert_eq!(fnv1a_u32(&[1]), 0xAD2A_CA77_4798_5764);
        assert_ne!(fnv1a_u32(&[1, 2]), fnv1a_u32(&[2, 1]));
    }
}

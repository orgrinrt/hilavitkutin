//! What one call receives, and the digest taken over what it produces.
//!
//! The digest covers the three output columns and nothing else. Hashing the
//! intermediate columns would defeat the point: a fused arm keeps them in
//! registers and never materialises them. Every pipeline feeds the join, so a
//! stage dropped anywhere inside one still moves the first word, which is what
//! the negative control relies on.
//!
//! Nothing here reaches a sibling module: the digest is over plain words.

/// The input one call receives: a regime tag and the record column.
///
/// `Routine::Input` has to be `Copy` and flat, and it is the only channel the
/// harness gives a variant, so the regime travels in it. The driver resolves a
/// different routine per bench section and that routine writes the tag; a
/// variant reads it and picks its kernel. Nothing else tells a variant which
/// sweep it is running under.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FrameInput<const N: usize> {
    /// `TAG_HEAVY` or `TAG_LIGHT`.
    pub regime:  u32,
    /// Keeps `records` eight-byte aligned inside the image.
    pub pad:     u32,
    /// One u32 per record.
    pub records: [u32; N],
}

/// The digest one call writes: the FNV-1a hash of the join column and of the
/// two transform columns.
///
/// The three pipeline tails are not hashed directly. A fused arm keeps them in
/// registers and never materialises them, and hashing them would force it to.
/// Every pipeline feeds the join, so a stage dropped anywhere inside one moves
/// the first word, which is what the negative control relies on.
pub type FrameOut = [u64; 3];

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

/// FNV-1a over the little-endian bytes of a u32 slice.
pub fn fnv1a_u32(vals: &[u32]) -> u64 {
    let mut h = Fnv1a::new();
    for &v in vals {
        h.push(v);
    }
    h.finish()
}

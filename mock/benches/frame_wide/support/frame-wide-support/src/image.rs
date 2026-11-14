//! What one call receives, and the digest taken over what it produces.
//!
//! The digest covers the three output columns and nothing else. Hashing the
//! intermediate columns would defeat the point: a fused arm keeps them in
//! registers and never materialises them. Every pipeline feeds the join, so a
//! stage dropped anywhere inside one still moves the first word, which is what
//! the negative control relies on.
//!
//! The image layout, its seeding and the hash are `bench-arm`'s, which
//! `engine_vs_std` shares; this file says what they mean here.

pub use bench_arm::{Fnv1a, fnv1a_u32};

/// The input one call receives: the regime tag (`TAG_HEAVY` or `TAG_LIGHT`) and
/// the record column.
///
/// The regime travels in the input because the input is the only channel the
/// harness gives a variant. The driver resolves a different routine per bench
/// section and that routine writes the tag; a variant reads it and picks its
/// kernel.
pub type FrameInput<const N: usize> = bench_arm::TaggedRecords<N>;

/// The digest one call writes: the FNV-1a hash of the join column and of the
/// two transform columns.
///
/// The three pipeline tails are not hashed directly. A fused arm keeps them in
/// registers and never materialises them, and hashing them would force it to.
/// Every pipeline feeds the join, so a stage dropped anywhere inside one moves
/// the first word, which is what the negative control relies on.
pub type FrameOut = [u64; 3];

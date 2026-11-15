//! The per-record kernels every arm of every workload computes.
//!
//! Each workload's engine units and its std loops call these same functions,
//! so two arms can only disagree by how they schedule the work, never by what
//! a record computes.

pub const M1: u32 = 2654435761; // Knuth multiplicative hash
pub const M2: u32 = 2246822519;
pub const SH: u32 = 13;
pub const M4: u32 = 3266489917;

#[inline(always)]
pub fn stage1(i: u32) -> u32 {
    i.wrapping_mul(M1)
}
#[inline(always)]
pub fn stage2(a: u32) -> u32 {
    a.wrapping_mul(M2).wrapping_add(1)
}
#[inline(always)]
pub fn stage3(b: u32) -> u32 {
    (b >> SH) ^ b
}
#[inline(always)]
pub fn stage4(c: u32) -> u32 {
    c.wrapping_mul(M4)
}

/// The full element-wise chain, shared by the element-wise and accumulator
/// workloads so both arms of each compute provably identical values.
#[inline(always)]
pub fn chain(seed: u32) -> u32 {
    stage4(stage3(stage2(stage1(seed))))
}

/// Heavy per-record kernel: the four-stage chain iterated `HEAVY_ROUNDS` times.
/// Win-path workloads use it so real per-record compute dominates dispatch, and
/// the engine's parallel spread (not the dispatch overhead) is what the bench
/// measures. Shared across arms so both compute provably identical values.
pub const HEAVY_ROUNDS: usize = 8;
#[inline(always)]
pub fn heavy(seed: u32) -> u32 {
    let mut x = seed;
    let mut r = 0;
    while r < HEAVY_ROUNDS {
        x = chain(x);
        r += 1;
    }
    x
}

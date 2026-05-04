//! Three-axis classification for wrapper-script flag selection.
//!
//! DESIGN Q4 §RUSTC_WORKSPACE_WRAPPER defines three orthogonal axes
//! (Target, Tier, Passes). The skeleton ships the enums; the real
//! flag-set computation (each `(target, tier, passes)` triple mapped
//! to a concrete rustc invocation) lives in the wrapper-script
//! generation follow-up round per BACKLOG.

/// ISA / feature-set classification. Detected from
/// `CARGO_CFG_TARGET_FEATURE` at build-script time.
///
/// `Iss64` stands for the baseline 64-bit instruction set: the
/// neutral default when no specific feature is detected.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TargetAxis {
    /// Baseline 64-bit instruction set; no SIMD assumed.
    Iss64,
    /// x86_64 with AVX2.
    Avx2,
    /// x86_64 with AVX-512.
    Avx512,
    /// AArch64 with NEON.
    Neon,
    /// AArch64 with SVE.
    Sve,
}

impl Default for TargetAxis {
    fn default() -> Self {
        TargetAxis::Iss64
    }
}

/// Optimisation tier. Additive per DESIGN §Optimisation tiers;
/// `PgoBolt` implies PGO data + BOLT post-link rewriting.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TierAxis {
    /// Plain rustc output, no post-link rewriting.
    Static,
    /// Static BOLT (+3-5% via reordered blocks).
    StaticBolt,
    /// PGO (+10-20% from profile-driven inlining / layout).
    Pgo,
    /// Profile-guided BOLT atop PGO (+PGO +5-10%).
    PgoBolt,
}

impl Default for TierAxis {
    fn default() -> Self {
        TierAxis::Static
    }
}

/// LLVM pass-pipeline classification. Whether the pass plugin
/// registers callbacks at `VectorizerStartEP`, `OptimizerLastEP`,
/// both, or neither.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PassesAxis {
    /// Default LLVM pipeline; no extra callbacks.
    Standard,
    /// `VectorizerStartEP` callbacks (IRCE, LoopPredication, etc.).
    Vectorizer,
    /// `OptimizerLastEP` callbacks (LoopDataPrefetch, etc.).
    OptimizerLast,
    /// Both `VectorizerStartEP` and `OptimizerLastEP` callbacks.
    Full,
}

impl Default for PassesAxis {
    fn default() -> Self {
        PassesAxis::Standard
    }
}

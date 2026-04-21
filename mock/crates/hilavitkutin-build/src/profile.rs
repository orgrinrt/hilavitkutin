//! Cargo profile abstraction + default pragma sets per profile.
//!
//! DESIGN Q3 maps each profile to a deterministic baseline
//! `PragmaSet`. Consumer `build.rs` starts from the baseline and may
//! add / remove pragmas before calling `configure().run()`.

use crate::pragma::{Pragma, PragmaSet};

/// One of the five built-in profiles documented in `DESIGN.md` §Five
/// profiles (Q3).
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Profile {
    Dev,
    DevOpt,
    Release,
    Profiling,
    Ci,
}

impl Default for Profile {
    /// Safe fallback for build scripts invoked without a known
    /// `$PROFILE` env var.
    fn default() -> Self {
        Profile::Dev
    }
}

impl Profile {
    /// The default `PragmaSet` for this profile per DESIGN Q3.
    ///
    /// `ParallelCodegen(0)` means "auto-detect codegen units" — the
    /// rustc wrapper resolves `0` via `available_parallelism()` at
    /// generation time. A concrete consumer may replace with
    /// `ParallelCodegen(N)` post-construction.
    pub const fn default_pragmas(self) -> PragmaSet {
        let base = PragmaSet::new();
        match self {
            Profile::Dev => base
                .with(Pragma::ParallelCodegen(0))
                .with(Pragma::SharedGenerics),
            Profile::DevOpt => base
                .with(Pragma::LoopOptimization)
                .with(Pragma::ParallelCodegen(0))
                .with(Pragma::SharedGenerics),
            Profile::Release => base
                .with(Pragma::LoopOptimization)
                .with(Pragma::MathPeephole)
                .with(Pragma::FastMath)
                .with(Pragma::ExpandedLto)
                .with(Pragma::Pgo)
                .with(Pragma::Bolt)
                .with(Pragma::ParallelCodegen(0)),
            Profile::Profiling => base
                .with(Pragma::LoopOptimization)
                .with(Pragma::MathPeephole)
                .with(Pragma::FastMath)
                .with(Pragma::Pgo)
                .with(Pragma::Bolt)
                .with(Pragma::Profiling)
                .with(Pragma::ParallelCodegen(0)),
            Profile::Ci => base
                .with(Pragma::LoopOptimization)
                .with(Pragma::ExpandedLto)
                .with(Pragma::ParallelCodegen(0)),
        }
    }

    /// Resolve from a cargo `$PROFILE` env value. Unknown names fall
    /// through to `Dev` per src CL §Impl-time decisions item 2.
    pub fn from_cargo_profile(name: &str) -> Self { // lint:allow(no-bare-string) reason: cargo env `$PROFILE` value; tracked: #72
        match name {
            "dev" => Profile::Dev,
            "dev-opt" => Profile::DevOpt,
            "release" => Profile::Release,
            "profiling" => Profile::Profiling,
            "ci" => Profile::Ci,
            _ => Profile::Dev,
        }
    }
}

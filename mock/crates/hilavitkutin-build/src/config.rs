//! Fully-resolved build configuration.
//!
//! `BuildConfig` is the result of `from_cargo_env()`: a snapshot of
//! the profile, pragma set, target axis, tier axis, and passes axis
//! as resolved from `CARGO_CFG_*` / `PROFILE` env vars at the moment
//! a consumer's build.rs invokes `bootstrap_from_buildscript`.
//!
//! `fast_math()` is a dedicated accessor because it drives the
//! single cfg emission (`arvo_fast_math`) and is therefore hot in
//! the bootstrap path.

use crate::axis::{PassesAxis, TargetAxis, TierAxis};
use crate::pragma::{Pragma, PragmaSet};
use crate::profile::Profile;

/// Fully-resolved build configuration. Consumer `build.rs` may
/// construct via `from_cargo_env()` or field-by-field for tests.
#[derive(Debug, Clone, Default)]
pub struct BuildConfig {
    pub profile: Profile,
    pub pragmas: PragmaSet,
    pub target: Option<TargetAxis>, // lint:allow(no-bare-option) reason: cargo env optionality; mirrors `std::env::var` result semantics; tracked: #72
    pub tier: TierAxis,
    pub passes: PassesAxis,
}

impl BuildConfig {
    /// Build a `BuildConfig` by reading the cargo-supplied
    /// environment at build-script time.
    ///
    /// Unknown / missing vars degrade gracefully:
    /// - `$PROFILE` missing → `Profile::Dev`
    /// - `$CARGO_CFG_TARGET_FEATURE` missing → `target = None`
    ///
    /// Tier + passes are not yet derivable from env alone; the
    /// wrapper-script round wires them up.
    pub fn from_cargo_env() -> Self {
        let profile = std::env::var("PROFILE")
            .map(|p| Profile::from_cargo_profile(&p))
            .unwrap_or_default();

        let target = std::env::var("CARGO_CFG_TARGET_FEATURE")
            .ok()
            .map(|features| resolve_target_axis(&features));

        BuildConfig {
            profile,
            pragmas: profile.default_pragmas(),
            target,
            tier: TierAxis::default(),
            passes: PassesAxis::default(),
        }
    }

    /// `true` when the `FastMath` pragma is active. Drives the
    /// `arvo_fast_math` cfg emission in `bootstrap_from_buildscript`.
    pub const fn fast_math(&self) -> bool { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: build-time predicate; drives the `arvo_fast_math` cfg emission; tracked: #72
        self.pragmas.contains(Pragma::FastMath)
    }
}

/// Rough mapping from `CARGO_CFG_TARGET_FEATURE`'s comma-separated
/// feature list to the coarsest matching `TargetAxis`.
///
/// Precedence (richest first): Avx512 > Avx2 > Sve > Neon > Iss64.
fn resolve_target_axis(features: &str) -> TargetAxis { // lint:allow(no-bare-string) reason: cargo env `CARGO_CFG_TARGET_FEATURE` value; tracked: #72
    let has = |needle: &str| features.split(',').any(|f| f.trim() == needle); // lint:allow(no-bare-string) reason: feature-list closure operand; tracked: #72

    if has("avx512f") || has("avx512") {
        TargetAxis::Avx512
    } else if has("avx2") {
        TargetAxis::Avx2
    } else if has("sve") {
        TargetAxis::Sve
    } else if has("neon") {
        TargetAxis::Neon
    } else {
        TargetAxis::Iss64
    }
}

//! Fully-resolved build configuration.
//!
//! `BuildConfig` is a snapshot of the profile, pragma set, target axis,
//! tier axis and passes axis, resolved from the values cargo hands a
//! build script in `$PROFILE` and `$CARGO_CFG_TARGET_FEATURE`. The crate
//! reads neither variable itself: the consumer's `build.rs` reads its own
//! environment and passes what it found to [`BuildConfig::from_cargo`].
//!
//! `fast_math()` is a dedicated accessor because it drives the
//! single cfg emission (`arvo_fast_math`) and is therefore hot in
//! the bootstrap path.

use notko::Maybe;

use crate::axis::{PassesAxis, TargetAxis, TierAxis};
use crate::pragma::{Pragma, PragmaSet};
use crate::profile::Profile;

/// Fully-resolved build configuration. Consumer `build.rs` may
/// construct via `from_cargo()` or field-by-field for tests.
#[derive(Debug, Clone, Default)]
pub struct BuildConfig {
    pub profile: Profile,
    pub pragmas: PragmaSet,
    pub target:  Maybe<TargetAxis>,
    pub tier:    TierAxis,
    pub passes:  PassesAxis,
}

impl BuildConfig {
    /// Build a `BuildConfig` from the values of cargo's `$PROFILE` and
    /// `$CARGO_CFG_TARGET_FEATURE`, as the build script read them.
    ///
    /// A variable cargo did not set is `Maybe::Isnt`, and degrades
    /// gracefully:
    /// - `profile` absent or unknown → `Profile::Dev`
    /// - `target_features` absent → `target = Maybe::Isnt`
    ///
    /// Tier + passes are not yet derivable from these values; the
    /// wrapper-script round wires them up.
    pub fn from_cargo(
        profile: Maybe<&str>, // lint:allow(no-bare-string) reason: cargo env values as the build script read them; tracked: #72
        target_features: Maybe<&str>, // lint:allow(no-bare-string) reason: cargo env values as the build script read them; tracked: #72
    ) -> Self {
        let profile = match profile {
            Maybe::Is(p) => Profile::from_cargo_profile(p),
            Maybe::Isnt => Profile::default(),
        };

        let target = match target_features {
            Maybe::Is(features) => Maybe::Is(resolve_target_axis(features)),
            Maybe::Isnt => Maybe::Isnt,
        };

        BuildConfig {
            profile,
            pragmas: profile.default_pragmas(),
            target,
            tier: TierAxis::default(),
            passes: PassesAxis::default(),
        }
    }

    /// `true` when the `FastMath` pragma is active. Drives the
    /// `arvo_fast_math` cfg emission in [`crate::write_directives`].
    #[rustfmt::skip] // keeps the allow on the signature it governs
    pub const fn fast_math(
        &self,
    ) -> bool { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: build-time predicate; drives the `arvo_fast_math` cfg emission; tracked: #72
        self.pragmas.contains(Pragma::FastMath)
    }
}

/// Rough mapping from `CARGO_CFG_TARGET_FEATURE`'s comma-separated
/// feature list to the coarsest matching `TargetAxis`.
///
/// Precedence (richest first): Avx512 > Avx2 > Sve > Neon > Iss64.
fn resolve_target_axis(
    features: &str, // lint:allow(no-bare-string) reason: cargo env `CARGO_CFG_TARGET_FEATURE` value; tracked: #72
) -> TargetAxis {
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

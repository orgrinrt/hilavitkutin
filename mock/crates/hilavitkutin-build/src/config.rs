//! The build as a script resolved it.
//!
//! `BuildConfig` is the profile, the folded set of the profile's pragmas
//! and the target, resolved from the values cargo hands a build script.
//! The crate reads no variable itself: the consumer's script reads its
//! own environment and hands over what it found, the values to
//! [`BuildConfig::from_cargo`] or a closure to [`BuildConfig::from_env`].

use notko::Maybe;

use crate::bootstrap::{Env, borrowed};
use crate::pragma::PragmaSet;
use crate::profile::Profile;
use crate::prototype::Prototype;
use crate::roster::FastMath;
use crate::target::Target;

/// The build as a script resolved it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BuildConfig {
    pub profile: Profile,
    pub pragmas: PragmaSet,
    pub target:  Target,
}

impl BuildConfig {
    /// The build of the prototype `P` from the value of cargo's
    /// `$PROFILE` and the target.
    ///
    /// A profile cargo did not name, or one the crate does not know, is
    /// `Profile::Dev`.
    pub fn from_cargo<P: Prototype>(
        profile: Maybe<&str>, // lint:allow(no-bare-string) reason: cargo env value as the build script read it; tracked: #72
        target: Target,
    ) -> Self {
        let profile = match profile {
            Maybe::Is(name) => Profile::from_cargo_profile(name),
            Maybe::Isnt => Profile::default(),
        };
        let pragmas = match profile {
            Profile::Dev => PragmaSet::of::<P::Dev>(),
            Profile::DevOpt => PragmaSet::of::<P::DevOpt>(),
            Profile::Release => PragmaSet::of::<P::Release>(),
            Profile::Profiling => PragmaSet::of::<P::Profiling>(),
            Profile::Ci => PragmaSet::of::<P::Ci>(),
        };
        BuildConfig {
            profile,
            pragmas,
            target,
        }
    }

    /// The build of the prototype `P`, read through the script's closure:
    /// `PROFILE` and the four `CARGO_CFG_TARGET_*` values the target is
    /// made of.
    pub fn from_env<P: Prototype, S: AsRef<str>>(mut env: impl Env<S>) -> Self {
        let profile = env("PROFILE");
        let os = env("CARGO_CFG_TARGET_OS");
        let abi = env("CARGO_CFG_TARGET_ENV");
        let arch = env("CARGO_CFG_TARGET_ARCH");
        let features = env("CARGO_CFG_TARGET_FEATURE");
        let target = Target::from_cargo(
            borrowed(&os),
            borrowed(&abi),
            borrowed(&arch),
            borrowed(&features),
        );
        Self::from_cargo::<P>(borrowed(&profile), target)
    }

    /// `true` when the `FastMath` pragma is in the set, which is when the
    /// `arvo_fast_math` cfg is set for the crate.
    #[rustfmt::skip] // keeps the allow on the signature it governs
    pub const fn fast_math(
        &self,
    ) -> bool { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: build-time predicate; tracked: #72
        self.pragmas.contains::<FastMath>()
    }
}

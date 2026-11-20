//! The profiles a build is one of.
//!
//! Which pragmas a profile builds with is a prototype's to say
//! ([`Prototype`](crate::Prototype)); this module names the profiles and
//! reads the one cargo passes a script.

/// One of the five profiles of the design.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum Profile {
    /// The fallback of a script that got no profile it knows.
    #[default]
    Dev,
    DevOpt,
    Release,
    Profiling,
    Ci,
}

impl Profile {
    /// Resolve from a cargo `$PROFILE` env value.
    ///
    /// Cargo passes a build script only `debug` or `release`, by which
    /// built-in profile the one being built inherits from, so the
    /// custom profiles are not reachable from it. Anything else falls
    /// back to `Dev`.
    #[rustfmt::skip] // keeps the allow on the signature it governs
    pub fn from_cargo_profile(name: &str) -> Self { // lint:allow(no-bare-string) reason: cargo env value as the build script read it; tracked: #72
        match name {
            "release" => Profile::Release,
            _ => Profile::Dev,
        }
    }

    /// The profile as cargo spells it, in a manifest and in its
    /// configuration.
    pub const fn name(self) -> &'static str {
        match self {
            Profile::Dev => "dev",
            Profile::DevOpt => "dev-opt",
            Profile::Release => "release",
            Profile::Profiling => "profiling",
            Profile::Ci => "ci",
        }
    }
}

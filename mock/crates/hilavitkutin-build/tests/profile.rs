//! The profiles: the one cargo's `PROFILE` names, and each as cargo spells it.

use hilavitkutin_build::Profile;

/// Cargo passes a build script `debug` or `release` and nothing else.
#[test]
fn the_two_names_cargo_passes_resolve() {
    assert_eq!(Profile::from_cargo_profile("debug"), Profile::Dev);
    assert_eq!(Profile::from_cargo_profile("release"), Profile::Release);
}

/// A name cargo never passes, the custom profile names among them, is
/// `Dev`: cargo does not tell a build script which custom profile it builds.
#[test]
fn from_cargo_profile_unknown_falls_through_to_dev() {
    for name in ["", "bench", "Release", "dev", "dev-opt", "profiling", "ci"] {
        assert_eq!(Profile::from_cargo_profile(name), Profile::Dev, "{name:?}");
    }
    assert_eq!(Profile::default(), Profile::Dev);
}

/// The names cargo has for the profiles in a manifest and in its
/// configuration, which the generated cargo file's keys are written with.
#[test]
fn a_profile_is_named_as_cargo_spells_it() {
    assert_eq!(
        [Profile::Dev, Profile::DevOpt, Profile::Release, Profile::Profiling, Profile::Ci]
            .map(Profile::name),
        ["dev", "dev-opt", "release", "profiling", "ci"]
    );
}

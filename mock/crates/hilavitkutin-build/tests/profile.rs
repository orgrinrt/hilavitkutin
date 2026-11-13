//! `Profile::default_pragmas` matches DESIGN Q3 for every profile.

use hilavitkutin_build::{Pragma, Profile};

#[test]
fn dev_defaults() {
    let s = Profile::Dev.default_pragmas();
    assert!(s.contains(Pragma::ParallelCodegen(0)));
    assert!(s.contains(Pragma::SharedGenerics));

    assert!(!s.contains(Pragma::LoopOptimization));
    assert!(!s.contains(Pragma::FastMath));
    assert!(!s.contains(Pragma::ExpandedLto));
    assert!(!s.contains(Pragma::Pgo));
    assert!(!s.contains(Pragma::Bolt));
}

#[test]
fn dev_opt_defaults() {
    let s = Profile::DevOpt.default_pragmas();
    assert!(s.contains(Pragma::LoopOptimization));
    assert!(s.contains(Pragma::ParallelCodegen(0)));
    assert!(s.contains(Pragma::SharedGenerics));

    assert!(!s.contains(Pragma::FastMath));
    assert!(!s.contains(Pragma::ExpandedLto));
}

#[test]
fn release_defaults() {
    let s = Profile::Release.default_pragmas();
    assert!(s.contains(Pragma::LoopOptimization));
    assert!(s.contains(Pragma::MathPeephole));
    assert!(s.contains(Pragma::FastMath));
    assert!(s.contains(Pragma::ExpandedLto));
    assert!(s.contains(Pragma::Pgo));
    assert!(s.contains(Pragma::Bolt));
    assert!(s.contains(Pragma::ParallelCodegen(0)));

    assert!(!s.contains(Pragma::Profiling));
    assert!(!s.contains(Pragma::SharedGenerics));
    assert!(!s.contains(Pragma::BuildStd));
}

#[test]
fn profiling_defaults() {
    let s = Profile::Profiling.default_pragmas();
    assert!(s.contains(Pragma::LoopOptimization));
    assert!(s.contains(Pragma::MathPeephole));
    assert!(s.contains(Pragma::FastMath));
    assert!(s.contains(Pragma::Pgo));
    assert!(s.contains(Pragma::Bolt));
    assert!(s.contains(Pragma::Profiling));
    assert!(s.contains(Pragma::ParallelCodegen(0)));

    assert!(!s.contains(Pragma::ExpandedLto));
    assert!(!s.contains(Pragma::SharedGenerics));
}

#[test]
fn ci_defaults() {
    let s = Profile::Ci.default_pragmas();
    assert!(s.contains(Pragma::LoopOptimization));
    assert!(s.contains(Pragma::ExpandedLto));
    assert!(s.contains(Pragma::ParallelCodegen(0)));

    assert!(!s.contains(Pragma::FastMath));
    assert!(!s.contains(Pragma::Pgo));
    assert!(!s.contains(Pragma::Bolt));
    assert!(!s.contains(Pragma::Profiling));
}

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
}

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

#[test]
fn from_cargo_profile_known_values() {
    assert_eq!(Profile::from_cargo_profile("dev"), Profile::Dev);
    assert_eq!(Profile::from_cargo_profile("dev-opt"), Profile::DevOpt);
    assert_eq!(Profile::from_cargo_profile("release"), Profile::Release);
    assert_eq!(Profile::from_cargo_profile("profiling"), Profile::Profiling);
    assert_eq!(Profile::from_cargo_profile("ci"), Profile::Ci);
}

#[test]
fn from_cargo_profile_unknown_falls_through_to_dev() {
    assert_eq!(Profile::from_cargo_profile(""), Profile::Dev);
    assert_eq!(Profile::from_cargo_profile("bench"), Profile::Dev);
    assert_eq!(Profile::from_cargo_profile("Release"), Profile::Dev); // case-sensitive
}

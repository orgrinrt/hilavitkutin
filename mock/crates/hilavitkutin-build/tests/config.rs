//! The build as a script resolved it: the profile, the folded set of the
//! prototype for that profile, and the target.

mod common;

use common::held;
// The prototype is named `Default`, which a test that also calls
// `default()` keeps apart from the trait of that name.
use hilavitkutin_build::{
    Arch,
    BuildConfig,
    Default as Defaults,
    Deterministic,
    ObjectFormat,
    Os,
    PragmaSet,
    Profile,
    Target,
    Workspace,
};
use notko::Maybe;

#[test]
fn the_profile_cargo_names_picks_the_prototypes_set() {
    for (name, profile, pragmas) in [
        (Maybe::Is("debug"), Profile::Dev, vec![
            "ParallelCodegen",
            "SharedGenerics",
        ]),
        (Maybe::Is("release"), Profile::Release, vec![
            "LoopOptimization",
            "MathPeephole",
            "FastMath",
            "Pgo",
            "Bolt",
            "ParallelCodegen",
        ]),
        // Absent, unknown, empty and in another case all fall back to dev.
        (Maybe::Isnt, Profile::Dev, vec![
            "ParallelCodegen",
            "SharedGenerics",
        ]),
        (Maybe::Is("bench"), Profile::Dev, vec![
            "ParallelCodegen",
            "SharedGenerics",
        ]),
        (Maybe::Is(""), Profile::Dev, vec![
            "ParallelCodegen",
            "SharedGenerics",
        ]),
        (Maybe::Is("Release"), Profile::Dev, vec![
            "ParallelCodegen",
            "SharedGenerics",
        ]),
    ] {
        let cfg = BuildConfig::from_cargo::<Defaults>(name, Target::default());
        assert_eq!(cfg.profile, profile, "{name:?}");
        assert_eq!(held(cfg.pragmas), pragmas, "{name:?}");
    }
}

#[test]
fn the_set_is_the_named_prototypes() {
    let release = |cfg: BuildConfig| held(cfg.pragmas);
    assert_eq!(
        release(BuildConfig::from_cargo::<Deterministic>(
            Maybe::Is("release"),
            Target::default()
        )),
        ["LoopOptimization", "Pgo", "Bolt", "ParallelCodegen"]
    );
    assert_eq!(
        release(BuildConfig::from_cargo::<Workspace>(
            Maybe::Is("release"),
            Target::default()
        )),
        [
            "LoopOptimization",
            "MathPeephole",
            "FastMath",
            "ExpandedLto",
            "Pgo",
            "Bolt",
            "ParallelCodegen"
        ]
    );
}

#[test]
fn fast_math_is_on_where_the_set_holds_it() {
    assert!(
        BuildConfig::from_cargo::<Defaults>(Maybe::Is("release"), Target::default()).fast_math()
    );
    assert!(
        !BuildConfig::from_cargo::<Defaults>(Maybe::Is("debug"), Target::default()).fast_math()
    );
    assert!(
        !BuildConfig::from_cargo::<Deterministic>(Maybe::Is("release"), Target::default())
            .fast_math()
    );
    assert!(!BuildConfig::default().fast_math());
    assert_eq!(BuildConfig::default().pragmas, PragmaSet::of::<()>());
}

#[test]
fn the_target_handed_over_is_the_one_carried() {
    let target = Target::from_cargo(
        Maybe::Is("macos"),
        Maybe::Isnt,
        Maybe::Is("aarch64"),
        Maybe::Is("neon"),
    );
    let cfg = BuildConfig::from_cargo::<Defaults>(Maybe::Is("release"), target);
    assert_eq!(cfg.target, target);
    assert_ne!(cfg.target, Target::default());
}

/// `from_env` asks for the profile and the four target values, in that
/// order, and reads each into its place.
#[test]
fn the_closure_is_asked_for_the_profile_and_the_four_target_values() {
    let mut asked = Vec::new();
    let cfg = BuildConfig::from_env::<Defaults, _>(|key| {
        asked.push(key);
        match key {
            "PROFILE" => Maybe::Is("release"),
            "CARGO_CFG_TARGET_OS" => Maybe::Is("linux"),
            "CARGO_CFG_TARGET_ENV" => Maybe::Is("gnu"),
            "CARGO_CFG_TARGET_ARCH" => Maybe::Is("x86_64"),
            "CARGO_CFG_TARGET_FEATURE" => Maybe::Is("sse2,avx2"),
            _ => Maybe::Isnt,
        }
    });
    assert_eq!(asked, [
        "PROFILE",
        "CARGO_CFG_TARGET_OS",
        "CARGO_CFG_TARGET_ENV",
        "CARGO_CFG_TARGET_ARCH",
        "CARGO_CFG_TARGET_FEATURE"
    ]);
    assert_eq!(cfg.profile, Profile::Release);
    assert_eq!(
        cfg.target,
        Target::from_cargo(
            Maybe::Is("linux"),
            Maybe::Is("gnu"),
            Maybe::Is("x86_64"),
            Maybe::Is("sse2,avx2")
        )
    );
    assert_eq!(
        (cfg.target.os, cfg.target.arch, cfg.target.format),
        (Os::Linux, Arch::X86_64, ObjectFormat::Elf)
    );

    let bare = BuildConfig::from_env::<Defaults, &str>(|_| Maybe::Isnt);
    assert_eq!(
        bare,
        BuildConfig::from_cargo::<Defaults>(Maybe::Isnt, Target::default())
    );
}

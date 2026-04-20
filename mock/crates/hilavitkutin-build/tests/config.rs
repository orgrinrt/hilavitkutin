//! `BuildConfig::fast_math` honours the `FastMath` pragma flag.

use hilavitkutin_build::{BuildConfig, Pragma, Profile};

#[test]
fn fast_math_true_when_pragma_set() {
    let cfg = BuildConfig {
        profile: Profile::Release,
        pragmas: Profile::Release.default_pragmas(),
        ..BuildConfig::default()
    };
    assert!(cfg.fast_math());
}

#[test]
fn fast_math_false_for_dev() {
    let cfg = BuildConfig {
        profile: Profile::Dev,
        pragmas: Profile::Dev.default_pragmas(),
        ..BuildConfig::default()
    };
    assert!(!cfg.fast_math());
}

#[test]
fn fast_math_follows_explicit_pragmas() {
    let cfg = BuildConfig {
        profile: Profile::Dev,
        pragmas: Profile::Dev.default_pragmas().with(Pragma::FastMath),
        ..BuildConfig::default()
    };
    assert!(cfg.fast_math());

    let cfg = BuildConfig {
        profile: Profile::Release,
        pragmas: Profile::Release
            .default_pragmas()
            .without(Pragma::FastMath),
        ..BuildConfig::default()
    };
    assert!(!cfg.fast_math());
}

#[test]
fn default_config_has_fast_math_off() {
    let cfg = BuildConfig::default();
    assert!(!cfg.fast_math());
}

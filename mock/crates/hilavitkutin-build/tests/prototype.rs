//! A prototype is a whole build script by name. These hold each one to the
//! directives it writes, read against `configure` and `configure_with`,
//! which a script that names no prototype still calls.

use hilavitkutin_build::{Pragma, Profile, Prototype, configure, configure_with};
use notko::Maybe;

/// The five LLVM options LoopOptimization owns, as the release flags carry them.
const LOOP_FLAGS: &str = "-Cllvm-args=-enable-loop-distribute \
    -Cllvm-args=-enable-loop-versioning-licm \
    -Cllvm-args=-enable-interleaved-mem-accesses \
    -Cllvm-args=-loop-prefetch-writes \
    -Cllvm-args=-enable-histogram-loop-vectorization";

/// A release build with no workspace wrapper named, where `configure`
/// warns.
fn release_env(key: &'static str) -> Maybe<&'static str> {
    match key {
        "PROFILE" => Maybe::Is("release"),
        _ => Maybe::Isnt,
    }
}

fn written(prototype: Prototype) -> String {
    let mut out = String::new();
    prototype
        .configure(release_env, &mut out)
        .expect("writing to a String cannot fail");
    out
}

#[test]
fn default_writes_what_configure_writes_less_the_warning() {
    let mut configured = String::new();
    configure(release_env, &mut configured).expect("writing to a String cannot fail");
    let (directives, warning) = configured
        .split_once("cargo::warning=")
        .expect("configure warns where no wrapper is named");
    assert!(warning.contains("so they are dropped"));
    assert_eq!(written(Prototype::Default), directives);
    assert!(directives.contains("cargo::rustc-cfg=arvo_fast_math\n"));
}

#[test]
fn deterministic_writes_no_fast_math_cfg_and_keeps_the_loop_flags() {
    let out = written(Prototype::Deterministic);
    assert!(!out.contains("cargo::rustc-cfg=arvo_fast_math"), "{out}");
    assert!(
        out.contains("cargo::rustc-check-cfg=cfg(arvo_fast_math)\n"),
        "{out}"
    );
    assert!(out.contains(LOOP_FLAGS), "{out}");

    let mut tuned = String::new();
    configure_with(
        |key| {
            match key {
                "RUSTC_WORKSPACE_WRAPPER" => Maybe::Is(".cargo/hilavitkutin-rustc-wrapper"),
                _ => release_env(key),
            }
        },
        |pragmas| pragmas.without(Pragma::FastMath),
        &mut tuned,
    )
    .expect("writing to a String cannot fail");
    assert_eq!(out, tuned);
}

#[test]
fn no_prototype_writes_a_warning() {
    for prototype in [Prototype::Default, Prototype::Deterministic] {
        let out = written(prototype);
        assert!(!out.contains("cargo::warning"), "{prototype:?}: {out}");
    }
}

#[test]
fn a_prototype_hands_back_the_build_it_resolved() {
    let mut out = String::new();
    let default = Prototype::Default
        .configure(release_env, &mut out)
        .expect("writing to a String cannot fail");
    assert_eq!(default.profile, Profile::Release);
    assert_eq!(default.pragmas, Profile::Release.default_pragmas());

    let deterministic = Prototype::Deterministic
        .configure(release_env, &mut out)
        .expect("writing to a String cannot fail");
    assert!(!deterministic.fast_math());
    assert!(!deterministic.pragmas.contains(Pragma::MathPeephole));
    assert!(deterministic.pragmas.contains(Pragma::LoopOptimization));
}

#[test]
fn tune_takes_fast_math_out_for_deterministic_alone() {
    const RELEASE: hilavitkutin_build::PragmaSet = Profile::Release.default_pragmas();
    const TUNED: hilavitkutin_build::PragmaSet = Prototype::Deterministic.tune(RELEASE);
    assert_eq!(Prototype::Default.tune(RELEASE), RELEASE);
    assert!(RELEASE.contains(Pragma::FastMath));
    assert!(!TUNED.contains(Pragma::FastMath));
    assert_eq!(TUNED, RELEASE.without(Pragma::FastMath));
}

/// A sink that refuses every write.
struct Refusing;

impl core::fmt::Write for Refusing {
    fn write_str(&mut self, _: &str) -> core::fmt::Result {
        Err(core::fmt::Error)
    }
}

#[test]
fn a_refusing_writer_is_reported() {
    assert!(
        Prototype::Default
            .configure(release_env, &mut Refusing)
            .is_err()
    );
}

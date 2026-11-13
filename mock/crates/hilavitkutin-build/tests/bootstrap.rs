//! The build script's half is reading cargo's environment and doing the
//! I/O; the crate's half is resolving what it read and producing the text.
//! These pin the crate's half: `BuildConfig::from_cargo` over every shape of
//! value cargo can hand over, and the exact text of the directives and the
//! config file.

use hilavitkutin_build::{
    BuildConfig,
    Pragma,
    PragmaSet,
    Profile,
    TargetAxis,
    configure,
    configure_with,
    write_config_file,
    write_directives,
    write_rustflags,
    write_wrapper,
};
use notko::Maybe;

fn directives(cfg: &BuildConfig) -> String {
    let mut out = String::new();
    write_directives(cfg, &mut out).expect("writing to a String cannot fail");
    out
}

fn config_file(cfg: &BuildConfig) -> String {
    let mut out = String::new();
    write_config_file(cfg, &mut out).expect("writing to a String cannot fail");
    out
}

#[test]
fn every_cargo_profile_name_resolves_to_its_profile() {
    for (name, expected) in [("debug", Profile::Dev), ("release", Profile::Release)] {
        let cfg = BuildConfig::from_cargo(Maybe::Is(name), Maybe::Isnt);
        assert_eq!(cfg.profile, expected, "profile name {name:?}");
        assert_eq!(
            cfg.pragmas,
            expected.default_pragmas(),
            "pragmas follow {name:?}"
        );
    }
}

#[test]
fn an_absent_or_unknown_profile_falls_back_to_dev() {
    assert_eq!(
        BuildConfig::from_cargo(Maybe::Isnt, Maybe::Isnt).profile,
        Profile::Dev
    );
    assert_eq!(
        BuildConfig::from_cargo(Maybe::Is("bench"), Maybe::Isnt).profile,
        Profile::Dev
    );
    assert_eq!(
        BuildConfig::from_cargo(Maybe::Is(""), Maybe::Isnt).profile,
        Profile::Dev
    );
    assert_eq!(
        BuildConfig::from_cargo(Maybe::Is("Release"), Maybe::Isnt).profile,
        Profile::Dev
    );
}

#[test]
fn an_absent_feature_list_leaves_the_target_unset() {
    assert_eq!(
        BuildConfig::from_cargo(Maybe::Is("release"), Maybe::Isnt).target,
        Maybe::Isnt
    );
}

#[test]
fn the_richest_listed_feature_picks_the_target() {
    for (features, expected) in [
        ("avx512f,avx2,sse2", TargetAxis::Avx512),
        ("avx512", TargetAxis::Avx512),
        ("sse2,avx2", TargetAxis::Avx2),
        ("neon,sve", TargetAxis::Sve),
        ("neon", TargetAxis::Neon),
        ("sse2, avx2 ", TargetAxis::Avx2),
        ("sse2,sse4.1", TargetAxis::Iss64),
        ("", TargetAxis::Iss64),
    ] {
        let cfg = BuildConfig::from_cargo(Maybe::Isnt, Maybe::Is(features));
        assert_eq!(cfg.target, Maybe::Is(expected), "feature list {features:?}");
    }
}

#[test]
fn a_feature_is_matched_whole_never_as_a_prefix() {
    for features in ["avx2x", "avx512vl", "neonx,sve2"] {
        let cfg = BuildConfig::from_cargo(Maybe::Isnt, Maybe::Is(features));
        assert_eq!(
            cfg.target,
            Maybe::Is(TargetAxis::Iss64),
            "feature list {features:?}"
        );
    }
}

/// The five LLVM options LoopOptimization owns, as the release flags carry them.
const LOOP_FLAGS: &str = "-Cllvm-args=-enable-loop-distribute \
    -Cllvm-args=-enable-loop-versioning-licm \
    -Cllvm-args=-enable-interleaved-mem-accesses \
    -Cllvm-args=-loop-prefetch-writes \
    -Cllvm-args=-enable-histogram-loop-vectorization";

#[test]
fn a_release_build_writes_the_cfg_and_the_loop_and_thread_flags() {
    let release = BuildConfig::from_cargo(Maybe::Is("release"), Maybe::Isnt);
    assert!(release.fast_math());
    assert_eq!(
        directives(&release),
        format!(
            "cargo::rerun-if-changed=build.rs\n\
             cargo::rustc-check-cfg=cfg(arvo_fast_math)\n\
             cargo::rustc-cfg=arvo_fast_math\n\
             cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS={LOOP_FLAGS} -Zthreads=0\n"
        ),
    );
}

#[test]
fn a_dev_build_writes_no_cfg_and_its_two_flags() {
    let dev = BuildConfig::from_cargo(Maybe::Is("debug"), Maybe::Isnt);
    assert!(!dev.fast_math());
    assert_eq!(
        directives(&dev),
        "cargo::rerun-if-changed=build.rs\n\
         cargo::rustc-check-cfg=cfg(arvo_fast_math)\n\
         cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS=-Zthreads=0 -Zshare-generics=y\n",
    );
}

/// No pragma that writes a flag: the variable is set, and empty.
#[test]
fn a_set_with_no_flag_writing_pragma_leaves_the_flags_empty() {
    let mut cfg = BuildConfig::from_cargo(Maybe::Is("release"), Maybe::Isnt);
    cfg.pragmas = PragmaSet::new().with(Pragma::Pgo).with(Pragma::ExpandedLto);
    let mut flags = String::new();
    write_rustflags(cfg.pragmas, &mut flags).expect("writing to a String cannot fail");
    assert_eq!(flags, "");
    assert!(directives(&cfg).ends_with("cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS=\n"));
}

/// Every flag the release set writes is one the pinned rustc accepts, and
/// none of the two the design once named and the toolchain refuses.
#[test]
fn no_flag_is_one_the_pinned_toolchain_refuses() {
    let mut flags = String::new();
    write_rustflags(Profile::Release.default_pragmas(), &mut flags)
        .expect("writing to a String cannot fail");
    for refused in ["-enable-unsafe-fp-math", "-vectorize-with-block-frequency", "target-cpu"] {
        assert!(!flags.contains(refused), "{refused} in {flags:?}");
    }
    assert!(!flags.starts_with(' ') && !flags.ends_with(' ') && !flags.contains("  "));
}

#[test]
fn configure_reads_profile_and_features_through_the_closure() {
    let mut asked = Vec::new();
    let mut out = String::new();
    configure(
        |key| {
            asked.push(key);
            match key {
                "PROFILE" => Maybe::Is(String::from("release")),
                _ => Maybe::Isnt,
            }
        },
        &mut out,
    )
    .expect("writing to a String cannot fail");
    assert_eq!(asked, ["PROFILE", "CARGO_CFG_TARGET_FEATURE"]);
    assert_eq!(
        out,
        directives(&BuildConfig::from_cargo(Maybe::Is("release"), Maybe::Isnt))
    );
}

/// A deterministic simulation crate turns FastMath off, and the cfg goes
/// while the loop flags stay.
#[test]
fn configure_with_tunes_the_default_set() {
    let mut out = String::new();
    configure_with(
        |key| {
            match key {
                "PROFILE" => Maybe::Is("release"),
                _ => Maybe::Isnt,
            }
        },
        |p| p.without(Pragma::FastMath),
        &mut out,
    )
    .expect("writing to a String cannot fail");
    assert!(!out.contains("cargo::rustc-cfg=arvo_fast_math"), "{out}");
    assert!(out.contains(LOOP_FLAGS), "{out}");
}

/// The wrapper this repository commits is the text the crate writes.
#[test]
fn the_committed_wrapper_is_the_written_one() {
    let mut text = String::new();
    write_wrapper(&mut text).expect("writing to a String cannot fail");
    assert_eq!(
        include_str!("../../../.cargo/hilavitkutin-rustc-wrapper"),
        text
    );
    assert!(text.contains("$HILAVITKUTIN_BUILD_RUSTFLAGS\n"));
}

#[test]
fn the_config_file_records_profile_and_fast_math() {
    let release = BuildConfig::from_cargo(Maybe::Is("release"), Maybe::Isnt);
    assert_eq!(
        config_file(&release),
        "# generated by hilavitkutin-build (skeleton placeholder)\n\
         # real schema lands with the wrapper-generation round\n\
         profile = Release\n\
         fast_math = true\n",
    );

    let dev = BuildConfig::from_cargo(Maybe::Isnt, Maybe::Isnt);
    assert_eq!(
        config_file(&dev),
        "# generated by hilavitkutin-build (skeleton placeholder)\n\
         # real schema lands with the wrapper-generation round\n\
         profile = Dev\n\
         fast_math = false\n",
    );
}

/// A sink that refuses every write, so an error from the writer has to come
/// back out of both functions rather than being swallowed.
struct Refusing;

impl core::fmt::Write for Refusing {
    fn write_str(&mut self, _: &str) -> core::fmt::Result {
        Err(core::fmt::Error)
    }
}

#[test]
fn a_failing_writer_is_reported_not_swallowed() {
    let release = BuildConfig::from_cargo(Maybe::Is("release"), Maybe::Isnt);
    assert!(write_directives(&release, &mut Refusing).is_err());
    assert!(write_config_file(&release, &mut Refusing).is_err());

    let dev = BuildConfig::from_cargo(Maybe::Is("debug"), Maybe::Isnt);
    assert!(write_directives(&dev, &mut Refusing).is_err());
    assert!(write_wrapper(&mut Refusing).is_err());
}

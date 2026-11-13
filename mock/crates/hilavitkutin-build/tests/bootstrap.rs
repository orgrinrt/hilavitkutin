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

/// Compile a one-function library with the pinned `rustc` under `flags`,
/// optimised and through LLVM's codegen, so an option LLVM does not know
/// is refused. True when rustc succeeded.
fn pinned_rustc_takes(flags: &str, name: &str) -> bool {
    let dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    let source = dir.join("probe.rs");
    std::fs::write(
        &source,
        "pub fn sum(xs: &[u32]) -> u32 { xs.iter().fold(0, |a, x| a.wrapping_add(*x)) }\n",
    )
    .expect("the probe source");
    let status = std::process::Command::new("rustc")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["--crate-type", "lib", "--edition", "2024", "-O", "--emit=obj", "-o"])
        .arg(dir.join("probe.o"))
        .arg(&source)
        .args(flags.split(' ').filter(|f| !f.is_empty()))
        .status()
        .expect("rustc on the path");
    status.success()
}

/// Every flag the release set writes is one the pinned rustc takes: a
/// library is compiled under them. The control, an option LLVM dropped,
/// is refused by the same call.
#[test]
fn the_pinned_rustc_takes_every_release_flag() {
    let mut flags = String::new();
    write_rustflags(Profile::Release.default_pragmas(), &mut flags)
        .expect("writing to a String cannot fail");
    assert!(
        pinned_rustc_takes(&flags, "release"),
        "rustc refused {flags:?}"
    );
    assert!(
        !pinned_rustc_takes("-Cllvm-args=-enable-unsafe-fp-math", "control"),
        "the control was taken, so the probe cannot see a refusal"
    );
    assert!(!flags.starts_with(' ') && !flags.ends_with(' ') && !flags.contains("  "));
}

/// The env `configure` reads, with cargo's workspace wrapper named.
fn release_env(key: &'static str) -> Maybe<String> {
    match key {
        "PROFILE" => Maybe::Is(String::from("release")),
        "RUSTC_WORKSPACE_WRAPPER" => {
            Maybe::Is(String::from("/repo/.cargo/hilavitkutin-rustc-wrapper"))
        },
        _ => Maybe::Isnt,
    }
}

#[test]
fn configure_reads_profile_features_and_wrapper_through_the_closure() {
    let mut asked = Vec::new();
    let mut out = String::new();
    configure(
        |key| {
            asked.push(key);
            release_env(key)
        },
        &mut out,
    )
    .expect("writing to a String cannot fail");
    assert_eq!(asked, [
        "PROFILE",
        "CARGO_CFG_TARGET_FEATURE",
        "RUSTC_WORKSPACE_WRAPPER"
    ]);
    assert_eq!(
        out,
        format!(
            "cargo::rerun-if-changed=build.rs\n\
             cargo::rustc-check-cfg=cfg(arvo_fast_math)\n\
             cargo::rustc-cfg=arvo_fast_math\n\
             cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS={LOOP_FLAGS} -Zthreads=0\n"
        )
    );
}

/// With flags to hand over and no wrapper named, or one named empty, the
/// build script says the flags are dropped; with none to hand over it says
/// nothing.
#[test]
fn a_missing_wrapper_is_warned_of_when_there_are_flags() {
    for wrapper in [Maybe::Isnt, Maybe::Is(String::new())] {
        let mut out = String::new();
        configure(
            |key| {
                match key {
                    "RUSTC_WORKSPACE_WRAPPER" => wrapper.clone(),
                    _ => release_env(key),
                }
            },
            &mut out,
        )
        .expect("writing to a String cannot fail");
        assert!(
            out.contains("cargo::warning=hilavitkutin-build chose rustc flags"),
            "{out}"
        );
    }
    let mut out = String::new();
    configure_with(
        |key| {
            match key {
                "RUSTC_WORKSPACE_WRAPPER" => Maybe::Isnt,
                _ => release_env(key),
            }
        },
        |_| PragmaSet::new().with(Pragma::FastMath),
        &mut out,
    )
    .expect("writing to a String cannot fail");
    assert!(!out.contains("cargo::warning"), "{out}");
}

/// Taking a pragma out takes what stands on it: MathPeephole goes with
/// FastMath, Profiling with the last of Pgo and Bolt.
#[test]
fn a_tuned_set_is_made_coherent() {
    let release = Profile::Release.default_pragmas();
    assert!(release.contains(Pragma::MathPeephole));
    let tuned = release.without(Pragma::FastMath).coherent();
    assert!(!tuned.contains(Pragma::MathPeephole));
    assert!(tuned.contains(Pragma::LoopOptimization));

    let profiling = Profile::Profiling.default_pragmas();
    assert!(
        profiling
            .without(Pragma::Pgo)
            .coherent()
            .contains(Pragma::Profiling)
    );
    assert!(
        !profiling
            .without(Pragma::Pgo)
            .without(Pragma::Bolt)
            .coherent()
            .contains(Pragma::Profiling)
    );
    assert_eq!(release.coherent(), release);
}

/// The committed wrapper, run with `echo` standing in for rustc, appends
/// the variable's words and nothing else: unset, empty, set, and holding
/// a glob that must not be expanded against the directory.
#[test]
fn the_wrapper_appends_the_variable_and_nothing_else() {
    let wrapper = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../.cargo/hilavitkutin-rustc-wrapper"
    );
    for (value, expected) in [
        (None, "rustc --crate-name x\n"),
        (Some(""), "rustc --crate-name x\n"),
        (
            Some("-Zthreads=0 -Zshare-generics=y"),
            "rustc --crate-name x -Zthreads=0 -Zshare-generics=y\n",
        ),
        (Some("*"), "rustc --crate-name x *\n"),
    ] {
        let mut command = std::process::Command::new("sh");
        command
            .args([wrapper, "echo", "rustc", "--crate-name", "x"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .env_remove("HILAVITKUTIN_BUILD_RUSTFLAGS");
        if let Some(value) = value {
            command.env("HILAVITKUTIN_BUILD_RUSTFLAGS", value);
        }
        let output = command.output().expect("sh on the path");
        assert!(output.status.success());
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            expected,
            "{value:?}"
        );
    }
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
    assert!(text.contains("set -f\nexec \"$@\" $HILAVITKUTIN_BUILD_RUSTFLAGS\n"));
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

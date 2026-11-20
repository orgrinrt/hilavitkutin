//! An arm is its set of pragmas and its quantity: what a call builds with is
//! what hilavitkutin-build writes for the set, on cargo's command line, and
//! nothing of it is set in the environment or read from a name. A set that
//! leaves incremental to cargo, and one that needs a tool the bench cannot
//! find yet, are refused before anything is built.

use std::ffi::OsStr;

use build_call::{Arm, Build, Dirs, Kind, Profile, Relink, Replay, cargo};
use hilavitkutin_build::{
    Bfd,
    ChecksumFreshness,
    ClangDriver,
    Cranelift,
    Incremental,
    Linker,
    Mold,
    NoIncremental,
    ThinLto,
    Wild,
};

type Plain = (ChecksumFreshness, NoIncremental);

#[test]
fn an_arm_takes_its_quantity_and_its_linker_from_its_types() {
    assert_eq!(Arm::of::<Plain, Build>().kind(), Kind::Build);
    assert_eq!(Arm::of::<Plain, Relink>().kind(), Kind::Relink);
    assert_eq!(Arm::of::<Plain, Replay>().kind(), Kind::Replay);
    assert_eq!(Arm::of::<Plain, Relink>().linker(), Linker::Toolchain);
    assert_eq!(
        Arm::of::<(Bfd, NoIncremental), Relink>().linker().name(),
        "Bfd"
    );
    assert_eq!(
        Arm::of::<(NoIncremental, Mold), Replay>().linker().name(),
        "Mold"
    );
    for kind in Kind::ALL {
        assert_eq!(Kind::from_label(kind.label()), Some(kind));
    }
    assert_eq!(Kind::from_label("default"), None);
}

#[test]
fn incremental_is_what_the_set_says() {
    assert!(Arm::of::<(ChecksumFreshness, Incremental), Build>().incremental());
    assert!(!Arm::of::<Plain, Build>().incremental());
    assert!(!Arm::of::<(ChecksumFreshness,), Build>().incremental());
}

#[test]
fn a_set_writes_its_keys_per_profile_and_its_flags() {
    let triple = "x86_64-unknown-linux-gnu";
    let plain = Arm::of::<Plain, Relink>();
    let dev = plain.written(Profile::Dev, triple);
    assert_eq!(dev.config, [
        "unstable.checksum-freshness = true",
        "profile.dev.incremental = false"
    ]);
    assert!(dev.rustflags.is_empty());
    assert_eq!(plain.written(Profile::Release, triple).config, [
        "unstable.checksum-freshness = true",
        "profile.release.incremental = false"
    ]);
    let on = Arm::of::<(ChecksumFreshness, Incremental, ThinLto), Build>();
    assert_eq!(on.written(Profile::Dev, triple).config, [
        "unstable.checksum-freshness = true",
        "profile.dev.incremental = true",
        "profile.dev.lto = \"thin\""
    ]);
    let bfd = Arm::of::<(Bfd, NoIncremental), Relink>();
    assert_eq!(bfd.written(Profile::Dev, triple).rustflags, [
        "-Clink-arg=-fuse-ld=bfd"
    ]);
}

/// The arguments of a cargo command, as text.
fn args(c: &std::process::Command) -> Vec<String> {
    c.get_args()
        .map(|a| a.to_string_lossy().into_owned())
        .collect()
}

/// Whether `args` holds `--config` followed by `value`.
fn configured(args: &[String], value: &str) -> bool {
    args.windows(2)
        .any(|pair| pair[0] == "--config" && pair[1] == value)
}

#[test]
fn the_command_carries_the_sets_keys_and_flags_and_sets_no_build_choice_in_the_environment() {
    let dirs = Dirs::of("test-arm", "an-arm", 25);
    let on = Arm::of::<(ChecksumFreshness, Incremental), Build>();
    let c = cargo::command(&dirs, &on, Profile::Dev, &["build"]);
    let said = args(&c);
    assert!(
        configured(&said, "profile.dev.incremental = true"),
        "{said:?}"
    );
    assert!(
        configured(&said, "unstable.checksum-freshness = true"),
        "{said:?}"
    );
    assert!(
        !said.iter().any(|a| a.starts_with("-Z")),
        "no unstable flag is written by hand: {said:?}"
    );
    assert!(
        !said.iter().any(|a| a.starts_with("build.rustflags")),
        "a set with no flag hands cargo none: {said:?}"
    );
    for (name, _) in c.get_envs() {
        assert!(
            ![OsStr::new("CARGO_INCREMENTAL"), OsStr::new("RUSTFLAGS")].contains(&name),
            "{name:?} is set in the environment"
        );
    }

    let off = Arm::of::<Plain, Relink>();
    let said = args(&cargo::command(&dirs, &off, Profile::Release, &["build"]));
    assert!(
        configured(&said, "profile.release.incremental = false"),
        "{said:?}"
    );
    assert!(
        !said.iter().any(|a| a.contains("profile.dev.")),
        "a release call carries no key of dev: {said:?}"
    );

    let bfd = Arm::of::<(Bfd, NoIncremental), Relink>();
    let said = args(&cargo::command(&dirs, &bfd, Profile::Dev, &["build"]));
    assert!(
        configured(&said, "build.rustflags=[\"-Clink-arg=-fuse-ld=bfd\"]"),
        "{said:?}"
    );
}

#[test]
fn a_set_that_leaves_incremental_to_cargo_is_refused() {
    assert_eq!(Arm::of::<Plain, Build>().refusal(), None);
    assert_eq!(
        Arm::of::<(ChecksumFreshness, Incremental), Build>().refusal(),
        None
    );
    assert_eq!(Arm::of::<(Bfd, NoIncremental), Relink>().refusal(), None);
    for arm in [Arm::of::<(), Build>(), Arm::of::<(ChecksumFreshness,), Relink>()] {
        let why = arm.refusal().expect("a set with neither value is refused");
        assert!(
            why.contains("neither Incremental nor NoIncremental"),
            "{why}"
        );
    }
}

#[test]
fn a_set_that_needs_a_tool_is_refused_with_the_tools_name() {
    let needs = [
        (Arm::of::<(Mold, NoIncremental), Relink>(), "mold"),
        (Arm::of::<(Wild, NoIncremental), Relink>(), "wild, clang"),
        (Arm::of::<(Cranelift, NoIncremental), Build>(), "cranelift"),
        (Arm::of::<(NoIncremental, ClangDriver), Relink>(), "clang"),
    ];
    for (arm, tools) in needs {
        let why = arm.refusal().expect("a set that needs a tool is refused");
        assert!(why.contains(&format!("needs {tools},")), "{why}");
    }
}

#[test]
#[should_panic(expected = "the arm's set needs mold")]
fn a_call_of_a_refused_arm_does_not_start() {
    let input = [4, 0, 0, 0, 0, 0, 0, 0];
    let _ = build_call::Call::ready(
        "test-arm-refused",
        "an-arm",
        Arm::of::<(Mold, NoIncremental), Relink>(),
        &input,
        25,
    );
}

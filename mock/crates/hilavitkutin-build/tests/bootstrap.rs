//! The build script's half is reading cargo's environment and doing the
//! I/O; the crate's half is producing the text. These pin the crate's
//! half: the exact text of the directives for a set, the errors that fail
//! a build, the warning for dropped flags, and the wrapper.

use hilavitkutin_build::tools::WILD;
use hilavitkutin_build::{
    Bolt,
    BuildConfig,
    Default as Defaults,
    DylibDomain,
    FastMath,
    Mold,
    Pgo,
    PragmaSet,
    Profile,
    Prototype,
    Set,
    SharedGenerics,
    Site,
    Target,
    ThinLto,
    ToolPath,
    Wild,
    configure,
    configure_as,
    write_directives,
    write_rustflags,
    write_wrapper,
};
use notko::Maybe;

const HEAD: &str = "cargo::rerun-if-env-changed=RUSTC_WORKSPACE_WRAPPER\n\
                    cargo::rustc-check-cfg=cfg(arvo_fast_math)\n";

fn target(os: &str, arch: &str) -> Target {
    Target::from_cargo(Maybe::Is(os), Maybe::Isnt, Maybe::Is(arch), Maybe::Isnt)
}

fn elf() -> Target {
    target("linux", "x86_64")
}

/// The directives of the set `S` for a release build on `target`, with
/// the tools at `tools`.
fn directives<S: Set>(target: Target, tools: &[ToolPath<'_>]) -> String {
    let cfg = BuildConfig {
        profile: Profile::Release,
        pragmas: PragmaSet::of::<S>(),
        target,
    };
    let site = Site::new(Profile::Release, "x86_64-unknown-linux-gnu", tools);
    let mut out = String::new();
    write_directives::<S, _>(&cfg, &site, &mut out).expect("writing to a String cannot fail");
    out
}

/// The five LLVM options LoopOptimization owns, as the release flags carry them.
const LOOP_FLAGS: &str = "-Cllvm-args=-enable-loop-distribute \
    -Cllvm-args=-enable-loop-versioning-licm \
    -Cllvm-args=-enable-interleaved-mem-accesses \
    -Cllvm-args=-loop-prefetch-writes \
    -Cllvm-args=-enable-histogram-loop-vectorization";

#[test]
fn a_set_writes_its_cfg_its_flags_and_its_link_arguments() {
    assert_eq!(
        directives::<(Mold, FastMath, SharedGenerics)>(elf(), &[]),
        format!(
            "{HEAD}cargo::rustc-cfg=arvo_fast_math\n\
             cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS=-Zshare-generics=y\n\
             cargo::rustc-link-arg=-fuse-ld=mold\n"
        )
    );
    assert_eq!(
        directives::<<Defaults as Prototype>::Release>(elf(), &[]),
        format!(
            "{HEAD}cargo::rustc-cfg=arvo_fast_math\n\
             cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS={LOOP_FLAGS} -Zthreads=0\n"
        )
    );
}

/// No pragma that writes anything a crate's rustc takes: the variable is
/// set, and empty. A pragma of workspace reach writes nothing here.
#[test]
fn a_set_with_nothing_to_write_leaves_the_flags_empty() {
    let empty = format!("{HEAD}cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS=\n");
    assert_eq!(directives::<()>(elf(), &[]), empty);
    assert_eq!(directives::<(Pgo, Bolt, ThinLto)>(elf(), &[]), empty);
}

#[test]
fn a_tool_found_is_named_to_the_link_by_its_path() {
    let tools = [ToolPath::new(
        &WILD,
        "/home/u/.cache/hilavitkutin-build/tools/host/wild-0.10.0/bin/wild",
    )];
    assert_eq!(
        directives::<(Wild,)>(elf(), &tools),
        format!(
            "{HEAD}cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS=-Clinker=clang \
             -Clink-arg=--ld-path=/home/u/.cache/hilavitkutin-build/tools/host/wild-0.10.0/bin/wild\n"
        )
    );
}

/// A pragma whose tool is built on the machine, with no path for it,
/// fails the build, and so does a path the wrapper cannot carry. With a
/// path it can carry the same set writes no error.
#[test]
fn a_locally_built_tool_with_no_path_fails_the_build() {
    assert_eq!(
        directives::<(Wild,)>(elf(), &[]),
        format!(
            "{HEAD}cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS=\n\
             cargo::error=hilavitkutin-build: Wild was given no path for wild, which only `script!` finds or \
             builds: name Wild in a script that is `script!`\n"
        )
    );
    let spaced = directives::<(Wild,)>(elf(), &[ToolPath::new(&WILD, "/home/a user/tools/wild")]);
    assert!(
        spaced.ends_with(
            "cargo::error=hilavitkutin-build: Wild was given `/home/a user/tools/wild` for wild, and the wrapper \
             cannot carry a path with a space: set HILAVITKUTIN_BUILD_TOOLS_HOME to a directory without one\n"
        ),
        "{spaced}"
    );
    assert!(
        !directives::<(Wild,)>(elf(), &[ToolPath::new(&WILD, "/t/wild")]).contains("cargo::error")
    );
    // A tool that is a package needs no path: it is on the machine's own.
    assert!(!directives::<(Mold,)>(elf(), &[]).contains("cargo::error"));
}

/// A pragma the target does not admit fails the build with one error
/// each. On a target that admits them the same set writes none.
#[test]
fn a_pragma_the_target_does_not_admit_fails_the_build() {
    let wasm = directives::<(Mold, FastMath, DylibDomain)>(target("unknown", "wasm32"), &[]);
    assert_eq!(
        wasm,
        format!(
            "{HEAD}cargo::rustc-cfg=arvo_fast_math\n\
             cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS=-Cprefer-dynamic\n\
             cargo::rustc-link-arg=-fuse-ld=mold\n\
             cargo::error=hilavitkutin-build: Mold links ELF only, and this target's objects are not ELF: name a \
             prototype whose set for this profile leaves Mold out on this target\n\
             cargo::error=hilavitkutin-build: DylibDomain has no meaning on wasm, which this target is: name a \
             prototype whose set for this profile leaves DylibDomain out on this target\n"
        )
    );
    assert!(!directives::<(Mold, FastMath, DylibDomain)>(elf(), &[]).contains("cargo::error"));
    let mach_o = directives::<(Mold, DylibDomain)>(target("macos", "aarch64"), &[]);
    assert_eq!(mach_o.matches("cargo::error=").count(), 1, "{mach_o}");
    assert!(
        mach_o.contains("cargo::error=hilavitkutin-build: Mold links ELF only"),
        "{mach_o}"
    );
}

/// `Bolt` is ELF only and in every release set. It is not refused on a
/// target of another format, so a release build there is as on ELF.
#[test]
fn the_default_release_set_is_refused_on_no_target() {
    let on_elf = directives::<<Defaults as Prototype>::Release>(elf(), &[]);
    for other in [
        target("macos", "aarch64"),
        target("windows", "x86_64"),
        target("unknown", "wasm32"),
        Target::default(),
    ] {
        assert_eq!(
            directives::<<Defaults as Prototype>::Release>(other, &[]),
            on_elf
        );
    }
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
    write_rustflags::<<Defaults as Prototype>::Release, _>(
        &Site::new(Profile::Release, "", &[]),
        &mut flags,
    )
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
        "CARGO_CFG_TARGET_OS" => Maybe::Is(String::from("linux")),
        "CARGO_CFG_TARGET_ARCH" => Maybe::Is(String::from("x86_64")),
        "RUSTC_WORKSPACE_WRAPPER" => {
            Maybe::Is(String::from("/repo/.cargo/hilavitkutin-rustc-wrapper"))
        },
        _ => Maybe::Isnt,
    }
}

#[test]
fn configure_reads_the_build_the_triple_and_the_wrapper_through_the_closure() {
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
        "CARGO_CFG_TARGET_OS",
        "CARGO_CFG_TARGET_ENV",
        "CARGO_CFG_TARGET_ARCH",
        "CARGO_CFG_TARGET_FEATURE",
        "TARGET",
        "RUSTC_WORKSPACE_WRAPPER"
    ]);
    assert_eq!(
        out,
        format!(
            "{HEAD}cargo::rustc-cfg=arvo_fast_math\n\
             cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS={LOOP_FLAGS} -Zthreads=0\n"
        )
    );
}

/// A prototype of a consumer's own, with a linker for release builds.
struct Linked;

impl Prototype for Linked {
    type Ci = ();
    type Dev = (Pgo,);
    type DevOpt = ();
    type Profiling = ();
    type Release = (Mold, SharedGenerics);
}

/// A prototype that names a tool only `script!` can find.
struct Wilded;

impl Prototype for Wilded {
    type Ci = ();
    type Dev = ();
    type DevOpt = ();
    type Profiling = ();
    type Release = (Wild,);
}

fn configured<P: Prototype>(env: impl Fn(&'static str) -> Maybe<String>) -> String {
    let mut out = String::new();
    configure_as::<P, _, _>(env, &mut out).expect("writing to a String cannot fail");
    out
}

/// A script written by hand names a prototype of its own: the set is the
/// one of the profile cargo names, and the target is read from cargo.
#[test]
fn configure_as_writes_the_named_prototypes_set_for_the_profile() {
    assert_eq!(
        configured::<Linked>(release_env),
        format!(
            "{HEAD}cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS=-Zshare-generics=y\n\
             cargo::rustc-link-arg=-fuse-ld=mold\n"
        )
    );
    let dev = configured::<Linked>(|key| {
        match key {
            "PROFILE" => Maybe::Is(String::from("debug")),
            _ => release_env(key),
        }
    });
    assert_eq!(
        dev,
        format!("{HEAD}cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS=\n")
    );

    // The same release build for a target cargo says is not ELF is refused.
    let on_mac = configured::<Linked>(|key| {
        match key {
            "CARGO_CFG_TARGET_OS" => Maybe::Is(String::from("macos")),
            _ => release_env(key),
        }
    });
    assert!(
        on_mac.contains("cargo::error=hilavitkutin-build: Mold links ELF only"),
        "{on_mac}"
    );
}

#[test]
fn configure_as_refuses_a_tool_only_script_finds() {
    let out = configured::<Wilded>(release_env);
    assert!(
        out.contains("cargo::error=hilavitkutin-build: Wild was given no path for wild"),
        "{out}"
    );
}

/// With flags to hand over and no wrapper named, or one named empty, the
/// build script says the flags are dropped; with none to hand over it says
/// nothing.
#[test]
fn a_missing_wrapper_is_warned_of_when_there_are_flags() {
    for wrapper in [Maybe::Isnt, Maybe::Is(String::new()), Maybe::Is(String::from("rustc-wrapper"))]
    {
        let out = configured::<Defaults>(|key| {
            match key {
                "RUSTC_WORKSPACE_WRAPPER" => wrapper.clone(),
                _ => release_env(key),
            }
        });
        assert!(
            out.contains("cargo::warning=hilavitkutin-build chose rustc flags"),
            "{out}"
        );
    }
    // The dev set of `Linked` writes no flag, so nothing is dropped.
    let out = configured::<Linked>(|key| {
        match key {
            "PROFILE" => Maybe::Is(String::from("debug")),
            "RUSTC_WORKSPACE_WRAPPER" => Maybe::Isnt,
            _ => release_env(key),
        }
    });
    assert!(!out.contains("cargo::warning"), "{out}");
}

/// The wrapper the crate writes, run with `echo` standing in for rustc,
/// appends the variable's words and nothing else: unset, empty, set, and
/// holding a glob that must not be expanded against the directory.
#[test]
fn the_wrapper_appends_the_variable_and_nothing_else() {
    let mut text = String::new();
    write_wrapper(&mut text).expect("writing to a String cannot fail");
    let dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("wrapper");
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    let wrapper = dir.join("hilavitkutin-rustc-wrapper");
    std::fs::write(&wrapper, &text).expect("the wrapper's text");
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
            .arg(&wrapper)
            .args(["echo", "rustc", "--crate-name", "x"])
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

/// A sink that refuses every write, so an error from the writer has to come
/// back out of every function rather than being swallowed.
struct Refusing;

impl core::fmt::Write for Refusing {
    fn write_str(&mut self, _: &str) -> core::fmt::Result {
        Err(core::fmt::Error)
    }
}

#[test]
fn a_failing_writer_is_reported_not_swallowed() {
    let cfg = BuildConfig::from_cargo::<Defaults>(Maybe::Is("release"), elf());
    let site = Site::new(Profile::Release, "", &[]);
    assert!(
        write_directives::<<Defaults as Prototype>::Release, _>(&cfg, &site, &mut Refusing)
            .is_err()
    );
    assert!(write_directives::<(), _>(&cfg, &site, &mut Refusing).is_err());
    assert!(write_rustflags::<(SharedGenerics,), _>(&site, &mut Refusing).is_err());
    assert!(write_wrapper(&mut Refusing).is_err());
}

/// `cargo clippy` names `clippy-driver` as the workspace wrapper; a check
/// generates no code, so the dropped flags cost it nothing and the build
/// script says nothing, where a wrapper of any other name is still warned of.
#[test]
fn clippy_driver_as_the_wrapper_is_not_warned_of() {
    let run = |name: &'static str| {
        configured::<Defaults>(move |key| {
            match key {
                "RUSTC_WORKSPACE_WRAPPER" => Maybe::Is(String::from(name)),
                _ => release_env(key),
            }
        })
    };
    let clippy = run("/home/u/.rustup/toolchains/nightly/bin/clippy-driver");
    assert!(!clippy.contains("cargo::warning"), "{clippy}");
    // Somebody else's wrapper, and two whose file names only end with a
    // name that would have been let through.
    for name in [
        "/usr/bin/sccache",
        "/repo/.cargo/my-hilavitkutin-rustc-wrapper",
        "/usr/bin/not-clippy-driver",
    ] {
        let other = run(name);
        assert!(
            other.contains("cargo::warning=hilavitkutin-build chose rustc flags"),
            "{name}: {other}"
        );
    }
}

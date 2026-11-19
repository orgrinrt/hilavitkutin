//! `script!` with the pragmas of the seven build choices, as a consumer
//! meets it: the script that claims the workspace and its generated cargo
//! file, and the tools a set waits on, each in a scratch workspace built
//! by the cargo that runs the suite.

mod harness;

use std::fs;
use std::path::Path;
use std::process::Command;

use harness::{
    WRITTEN_CONFIG,
    build,
    build_with,
    host,
    prototype,
    run_cargo,
    scratch,
    workspace,
    workspace_with,
    write_crate_with,
};

/// What `script!` is handed by the script that claims the workspace.
const CLAIMING: &str = "Workspace, workspace";

fn generated_for(krate: &str) -> String {
    format!(
        "# written by hilavitkutin-build for {krate}: edits are lost\n\
         profile.release.lto = \"fat\"\n\
         profile.release.codegen-units = 1\n"
    )
}

/// Give the crate at `root` a binary, so a build of it links.
fn with_a_binary(root: &Path) {
    fs::write(root.join("src/main.rs"), "fn main() {}\n").expect("the binary");
}

/// The rustc call that built the binary of `krate`.
fn link_of<'a>(built: &'a harness::Built, krate: &str) -> &'a str {
    let links: Vec<_> = built
        .compiles_of(krate)
        .into_iter()
        .filter(|line| line.contains("--crate-type bin"))
        .collect();
    assert_eq!(links.len(), 1, "{}", built.rustc);
    links[0]
}

/// The claiming script writes the generated file beside the wiring in the
/// first build, and a release build after it takes the file's keys. The
/// control is the same crate with a script that claims nothing: no file,
/// and a release build with no LTO.
#[test]
fn the_claiming_script_writes_the_generated_file_and_a_later_build_takes_its_keys() {
    let root = workspace_with("claims", "", CLAIMING);
    with_a_binary(&root);
    let generated = root.join(".cargo/hilavitkutin-build.toml");

    let first = build(&root);
    assert_eq!(
        fs::read_to_string(&generated).expect("the generated file was written"),
        generated_for("scratch-claims")
    );
    assert_eq!(
        fs::read_to_string(root.join(".cargo/config.toml")).expect("the config file"),
        WRITTEN_CONFIG
    );
    assert_eq!(
        first
            .said
            .matches("wrote the workspace's pragmas to")
            .count(),
        1,
        "{}",
        first.said
    );
    assert!(!first.said.contains("does not take"), "{}", first.said);
    let left = fs::read_dir(root.join(".cargo"))
        .expect("the config directory")
        .count();
    assert_eq!(
        left, 3,
        "the wrapper, the config file and the generated file"
    );

    let release = build_with(&root, &["--release"]);
    let link = link_of(&release, "scratch_claims");
    assert!(
        link.contains("-C lto=fat") && link.contains("-C codegen-units=1"),
        "{link}"
    );
    assert!(
        !release.said.contains("wrote the workspace's pragmas"),
        "{}",
        release.said
    );

    let again = build_with(&root, &["--release"]);
    assert!(!again.said.contains("warning:"), "{}", again.said);

    let plain = workspace("claims-control");
    with_a_binary(&plain);
    build(&plain);
    let release = build_with(&plain, &["--release"]);
    assert!(!plain.join(".cargo/hilavitkutin-build.toml").exists());
    assert!(!link_of(&release, "scratch_claims_control").contains("lto=fat"));
}

/// Two crates of one workspace that both claim it: one places its file,
/// the other finds it written for the first, the build fails naming both,
/// and the file stays the first one's. Cargo runs the two scripts side by
/// side here, so both may look before either has written.
#[test]
fn a_second_crate_that_claims_the_workspace_fails_the_build() {
    let root = scratch("two-claims").join("ws");
    fs::create_dir_all(&root).expect("the workspace");
    fs::write(
        root.join("Cargo.toml"),
        "[workspace]\nresolver = \"3\"\nmembers = [\"a\", \"b\"]\n",
    )
    .expect("the workspace's manifest");
    write_crate_with(&root.join("a"), "scratch-two-a", "", "", CLAIMING);
    write_crate_with(&root.join("b"), "scratch-two-b", "", "", CLAIMING);

    let built = run_cargo(&root, "build", &[], &[]);
    assert!(!built.passed, "{}", built.said);
    let left: Vec<_> = fs::read_dir(root.join(".cargo"))
        .expect("the config directory")
        .flatten()
        .map(|file| file.file_name())
        .filter(|name| name.to_string_lossy().starts_with('.'))
        .collect();
    assert!(
        left.is_empty(),
        "a file under a script's own name is left: {left:?}"
    );
    let text = fs::read_to_string(root.join(".cargo/hilavitkutin-build.toml"))
        .expect("one of the two wrote the file");
    let (first, second) = if text == generated_for("scratch-two-a") {
        ("scratch-two-a", "scratch-two-b")
    } else {
        assert_eq!(text, generated_for("scratch-two-b"));
        ("scratch-two-b", "scratch-two-a")
    };
    assert!(
        built.said.contains(&format!(
            "{second} claims the workspace with `script!(.., workspace)`"
        )) && built.said.contains(&format!("was written for {first}")),
        "{}",
        built.said
    );
}

/// The file is held to its whole text: one a key was added to by hand is
/// written again, and one that is current is left as it is.
#[test]
fn a_generated_file_edited_by_hand_is_written_again() {
    let root = workspace_with("edited", "", CLAIMING);
    let generated = root.join(".cargo/hilavitkutin-build.toml");
    build(&root);
    build(&root);
    let written = fs::metadata(&generated)
        .and_then(|file| file.modified())
        .expect("the file has a date");
    build(&root);
    assert_eq!(
        fs::metadata(&generated)
            .and_then(|file| file.modified())
            .expect("the file has a date"),
        written,
        "a current file is not written again"
    );

    fs::write(
        &generated,
        generated_for("scratch-edited") + "profile.dev.debug = 1\n",
    )
    .expect("the edit");
    let built = build(&root);
    assert_eq!(
        fs::read_to_string(&generated).expect("the generated file"),
        generated_for("scratch-edited")
    );
    assert_eq!(
        built
            .said
            .matches("wrote the workspace's pragmas to")
            .count(),
        1,
        "{}",
        built.said
    );
}

/// A file under the generated file's name that this crate did not write
/// is never written over: the build fails and the file is as it was.
#[test]
fn a_file_somebody_else_wrote_is_left_and_fails_the_build() {
    let root = workspace_with("foreign-file", "", CLAIMING);
    fs::create_dir_all(root.join(".cargo")).expect("the config directory");
    let theirs = "# kept by hand\nprofile.dev.debug = 1\n";
    fs::write(root.join(".cargo/hilavitkutin-build.toml"), theirs).expect("somebody's file");

    let built = run_cargo(&root, "build", &[], &[]);
    assert!(!built.passed, "{}", built.said);
    assert!(
        built.said.contains("was written for somebody else"),
        "{}",
        built.said
    );
    assert_eq!(
        fs::read_to_string(root.join(".cargo/hilavitkutin-build.toml")).expect("the file"),
        theirs
    );
}

/// A `config.toml` of the consumer's own with no include line is left as
/// it is, and the notice names the line. Once the consumer adds the line
/// the notice stops.
#[test]
fn a_config_file_without_the_include_line_is_left_and_the_line_is_named() {
    let root = workspace_with("no-include", "", CLAIMING);
    fs::create_dir_all(root.join(".cargo")).expect("the config directory");
    let theirs = "# the consumer's own\n[build]\nincremental = true\n";
    fs::write(root.join(".cargo/config.toml"), theirs).expect("the consumer's config");

    let built = build(&root);
    assert_eq!(
        fs::read_to_string(root.join(".cargo/config.toml")).expect("the config file"),
        theirs
    );
    assert!(
        built.said.contains(
            "add `include = [{ path = \"hilavitkutin-build.toml\", optional = true }]` at its top"
        ),
        "{}",
        built.said
    );
    assert_eq!(
        fs::read_to_string(root.join(".cargo/hilavitkutin-build.toml"))
            .expect("the generated file was written"),
        generated_for("scratch-no-include")
    );

    let with_line =
        format!("include = [{{ path = \"hilavitkutin-build.toml\", optional = true }}]\n{theirs}");
    fs::write(root.join(".cargo/config.toml"), &with_line).expect("the consumer adds the line");
    let built = build(&root);
    assert!(!built.said.contains("does not take"), "{}", built.said);
    assert_eq!(
        fs::read_to_string(root.join(".cargo/config.toml")).expect("the config file"),
        with_line
    );
}

fn machine_has(program: &str) -> bool {
    Command::new(program)
        .arg("--version")
        .output()
        .is_ok_and(|asked| asked.status.success())
}

/// A set that names a package tool. Where the machine has mold, the first
/// build links as the toolchain does by itself, like every flag of a
/// first build, and the second links with mold by the flag the wrapper
/// appends. Where it has none, the build fails with the command that
/// brings it.
#[test]
fn a_linker_from_the_machines_packages_is_used_or_the_build_says_how_to_get_it() {
    let root = workspace_with(
        "mold",
        &prototype("Linked", "(hilavitkutin_build::Mold,)"),
        "crate::Linked",
    );
    with_a_binary(&root);
    let first = run_cargo(&root, "build", &[], &[]);
    assert!(
        !first.said.contains("rustc-link-arg") && !first.rustc.contains("fuse-ld"),
        "the script named a link argument of its own\n{}\n{}",
        first.said,
        first.rustc
    );
    if machine_has("mold") {
        assert!(first.passed, "{}", first.said);
        let second = run_cargo(&root, "build", &[], &[]);
        assert!(second.passed, "{}", second.said);
        let link = link_of(&second, "scratch_mold");
        assert!(link.ends_with(" -Clink-arg=-fuse-ld=mold"), "{link}");
    } else {
        assert!(!first.passed, "{}", first.said);
        assert!(
            first.said.contains("install the package `mold`"),
            "{}",
            first.said
        );
    }
}

/// A sysroot of the test's own, with the cranelift backend or without.
fn sysroot(beside: &Path, with_backend: bool) -> std::path::PathBuf {
    let sysroot = beside.join("sysroot");
    let backends = sysroot
        .join("lib/rustlib")
        .join(host())
        .join("codegen-backends");
    fs::create_dir_all(&backends).expect("the sysroot");
    if with_backend {
        fs::write(backends.join("librustc_codegen_cranelift-stand-in.so"), "")
            .expect("the backend");
    }
    sysroot
}

/// A set that names a toolchain component, in the workspace's script. The
/// test hands the script a sysroot, so both answers are run on every
/// machine. With the backend there the script writes the backend's keys.
/// Without it the build fails with the rustup command, said once, and no
/// generated file is written: cargo reads the file before any script
/// runs, and a key for a backend the toolchain lacks would stop every
/// build after this one before a script could say why.
#[test]
fn a_backend_the_toolchain_lacks_fails_the_build_and_writes_no_key() {
    let items = prototype("Backend", "(hilavitkutin_build::Cranelift,)");

    let root = workspace_with("cranelift-there", &items, "crate::Backend, workspace");
    let there = sysroot(root.parent().expect("a scratch directory"), true);
    let built = run_cargo(&root, "build", &[], &[("SPY_SYSROOT", there.as_path())]);
    assert!(built.passed, "{}", built.said);
    assert_eq!(
        fs::read_to_string(root.join(".cargo/hilavitkutin-build.toml"))
            .expect("the generated file"),
        "# written by hilavitkutin-build for scratch-cranelift-there: edits are lost\n\
         unstable.codegen-backend = true\n\
         profile.dev.codegen-backend = \"cranelift\"\n"
    );

    let root = workspace_with("cranelift-gone", &items, "crate::Backend, workspace");
    let gone = sysroot(root.parent().expect("a scratch directory"), false);
    let built = run_cargo(&root, "build", &[], &[("SPY_SYSROOT", gone.as_path())]);
    assert!(!built.passed, "{}", built.said);
    assert_eq!(
        built
            .said
            .matches("run `rustup component add rustc-codegen-cranelift-preview`")
            .count(),
        1,
        "{}",
        built.said
    );
    assert!(
        built.said.contains(
            "Cranelift, in the dev set of the prototype that claims the workspace, waits on cranelift, which this \
             build has not got: the generated cargo file is left as it was"
        ),
        "{}",
        built.said
    );
    assert!(
        !root.join(".cargo/hilavitkutin-build.toml").exists(),
        "a key the build cannot follow was written"
    );
}

/// A set that names wild takes it from the tools home, by its full path,
/// and links through it. The stand-in here answers the version question
/// as wild 0.10.0 does and hands every link on to the system's `ld`, so
/// no network and no build of wild is needed; it writes down that it ran.
/// An older version beside it is left where nothing was built.
#[cfg(target_os = "linux")]
#[test]
fn wild_is_taken_from_the_tools_home_by_its_full_path() {
    use std::os::unix::fs::PermissionsExt as _;

    if !machine_has("clang") {
        // Wild links through clang; without it this machine cannot link so.
        return;
    }
    let root = workspace_with(
        "wild",
        &prototype("Wilded", "(hilavitkutin_build::Wild,)"),
        "crate::Wilded",
    );
    with_a_binary(&root);
    let beside = root
        .parent()
        .expect("the workspace sits in a scratch directory");
    let home = beside.join("tools");
    let dir = home.join(host()).join("wild-0.10.0");
    let older = home.join(host()).join("wild-0.9.0");
    fs::create_dir_all(dir.join("bin")).expect("the tool's directory");
    fs::create_dir_all(&older).expect("an older version's directory");
    let ran = beside.join("wild-ran");
    let wild = dir.join("bin/wild");
    fs::write(
        &wild,
        format!(
            "#!/bin/sh\nif [ \"$1\" = --version ]; then\n    echo 'Wild 0.10.0 stand-in (compatible with GNU \
             linkers)'\n    exit 0\nfi\necho ran >> '{}'\nexec ld \"$@\"\n",
            ran.display()
        ),
    )
    .expect("the stand-in");
    fs::set_permissions(&wild, fs::Permissions::from_mode(0o755)).expect("the stand-in runs");
    let vars = [("HILAVITKUTIN_BUILD_TOOLS_HOME", home.as_path())];

    // The first build runs without the wrapper, so with neither flag, and
    // links as the toolchain does by itself.
    let first = run_cargo(&root, "build", &[], &vars);
    assert!(first.passed, "{}", first.said);
    assert!(
        !link_of(&first, "scratch_wild").contains("ld-path"),
        "{}",
        first.rustc
    );
    assert!(!ran.exists());

    let second = run_cargo(&root, "build", &[], &vars);
    assert!(second.passed, "{}", second.said);
    let link = link_of(&second, "scratch_wild");
    assert!(
        link.ends_with(&format!(
            "-Clinker=clang -Clink-arg=--ld-path={}",
            wild.display()
        )),
        "{link}"
    );
    assert!(
        ran.exists(),
        "the link did not go through the tool in the home"
    );
    assert!(older.exists(), "nothing was built, so nothing is removed");
    assert!(!second.said.contains("is building wild"), "{}", second.said);
}

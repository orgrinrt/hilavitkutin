//! The local build of a tool, run whole: the lock, `cargo install`, and
//! the removal of the older versions beside what was built.
//!
//! No network is reached and no real linker is built. The build runs
//! under a cargo home of the test's own, whose configuration replaces the
//! registry with a directory the test writes. `cargo install` reads no
//! configuration from where it runs, only the one of its cargo home, so
//! that is where it has to be. In that directory is a crate under wild's name and pin
//! whose program answers the version question as wild does. So the script
//! runs the command it runs for a consumer, and cargo installs what the
//! test put there.

mod harness;

use std::fs;
use std::path::{Path, PathBuf};

use harness::{host, prototype, run_cargo, workspace_with};

/// A workspace whose one crate names wild, with a tools home beside it
/// and, in that, a cargo home that takes its registry from a directory.
/// Answers the workspace, the tools home, and the directory of the
/// registry.
fn workspace_naming_wild(name: &str) -> (PathBuf, PathBuf, PathBuf) {
    let root = workspace_with(
        name,
        &prototype("Wilded", "(hilavitkutin_build::Wild,)"),
        "crate::Wilded",
    );
    let beside = root
        .parent()
        .expect("the workspace sits in a scratch directory");
    let home = beside.join("tools");
    let registry = beside.join("registry");
    fs::create_dir_all(home.join(".cargo")).expect("the home's cargo directory");
    fs::create_dir_all(&registry).expect("the registry");
    fs::write(
        home.join(".cargo/config.toml"),
        format!(
            "[source.crates-io]\nreplace-with = \"stand-in\"\n\n[source.stand-in]\ndirectory = \"{}\"\n",
            registry.display()
        ),
    )
    .expect("the home's cargo configuration");
    // The build crate's own git dependencies are in the cargo home the
    // suite runs under, and an offline build finds them nowhere else.
    let ours = std::env::var_os("CARGO_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| Path::new(&home).join(".cargo")))
        .expect("a cargo home the suite runs under");
    #[cfg(unix)]
    std::os::unix::fs::symlink(ours.join("git"), home.join(".cargo/git"))
        .expect("the fetched git dependencies");
    (root, home, registry)
}

/// Put a crate under wild's name and pin into the registry. Its program
/// prints wild's version line whatever it is asked.
///
/// It depends on a second, empty crate. Cargo reads a directory registry
/// when a dependency is looked up in it, and a crate that depends on
/// nothing is then found for the install and not found for the build.
fn publish_the_stand_in(registry: &Path) {
    let part = registry.join("stand-in-part-0.0.1");
    fs::create_dir_all(part.join("src")).expect("the part's source directory");
    fs::write(
        part.join("Cargo.toml"),
        "[package]\nname = \"stand-in-part\"\nversion = \"0.0.1\"\nedition = \"2021\"\n",
    )
    .expect("the part's manifest");
    fs::write(part.join("src/lib.rs"), "").expect("the part's source");
    fs::write(part.join(".cargo-checksum.json"), "{\"files\":{}}").expect("the part's checksums");
    let krate = registry.join("wild-linker-0.10.0");
    fs::create_dir_all(krate.join("src")).expect("the stand-in's source directory");
    fs::write(
        krate.join("Cargo.toml"),
        "[package]\nname = \"wild-linker\"\nversion = \"0.10.0\"\nedition = \"2021\"\n\n[[bin]]\nname = \
         \"wild\"\npath = \"src/main.rs\"\n\n[dependencies]\nstand-in-part = \"0.0.1\"\n",
    )
    .expect("the stand-in's manifest");
    fs::write(
        krate.join("src/main.rs"),
        "fn main() {\n    println!(\"Wild 0.10.0 stand-in (compatible with GNU linkers)\");\n}\n",
    )
    .expect("the stand-in's source");
    fs::write(krate.join(".cargo-checksum.json"), "{\"files\":{}}")
        .expect("the stand-in's checksums");
}

/// The first build of a workspace that names wild, on a machine that has
/// it nowhere, builds it into the tools home. Beside it sit an older
/// version, a newer one, the lock file of another pin and a lock file of
/// this pin that a process long gone left behind. The older version goes
/// and nothing else does, the stale lock holds nobody up, and no scratch
/// directory of the build is left. The second build finds the tool and
/// builds nothing, and its rustc is handed the tool by its full path.
#[test]
fn a_tool_the_machine_lacks_is_built_into_the_home_and_older_versions_go() {
    let (root, home, registry) = workspace_naming_wild("local-build");
    publish_the_stand_in(&registry);
    let at = home.join(host());
    let dir = at.join("wild-0.10.0");
    let program = dir.join("bin/wild");
    let older = at.join("wild-0.9.0");
    let newer = at.join("wild-0.11.0");
    let other_lock = at.join("wild-0.9.0.lock");
    let stale_lock = at.join("wild-0.10.0.lock");
    fs::create_dir_all(older.join("bin")).expect("an older version");
    fs::write(older.join("bin/wild"), "").expect("an older version's program");
    fs::create_dir_all(newer.join("bin")).expect("a newer version");
    fs::write(&other_lock, "").expect("another pin's lock");
    fs::write(&stale_lock, "").expect("a lock nobody holds");
    let cargo_home = home.join(".cargo");
    let vars = [
        ("HILAVITKUTIN_BUILD_TOOLS_HOME", home.as_path()),
        ("CARGO_HOME", cargo_home.as_path()),
    ];

    let first = run_cargo(&root, "build", &[], &vars);
    assert!(first.passed, "{}", first.said);
    assert_eq!(
        first.said.matches("is building wild 0.10.0 into").count(),
        1,
        "{}",
        first.said
    );
    assert!(program.is_file(), "the program is not in the home");
    assert!(
        !dir.join("build").exists(),
        "the child's target directory is left"
    );
    assert!(!older.exists(), "the older version is left");
    assert!(newer.exists(), "a newer version, another clone's pin, went");
    assert!(other_lock.exists(), "another pin's lock went");

    let second = run_cargo(&root, "build", &[], &vars);
    assert!(second.passed, "{}", second.said);
    assert!(!second.said.contains("is building wild"), "{}", second.said);
    let compiles = second.compiles_of("scratch_local_build");
    let last = compiles.last().expect("the crate was compiled");
    assert!(
        last.ends_with(&format!(
            "-Clinker=clang -Clink-arg=--ld-path={}",
            program.display()
        )),
        "{last}"
    );
}

/// The control: the same workspace over a registry that has no such
/// crate. The install fails, so the build fails with the command to run
/// by hand, no program is in the home, and the older version stays, since
/// nothing took its place.
#[test]
fn a_local_build_that_fails_fails_the_build_and_removes_nothing() {
    let (root, home, _registry) = workspace_naming_wild("local-build-fails");
    let at = home.join(host());
    let older = at.join("wild-0.9.0");
    fs::create_dir_all(&older).expect("an older version");
    let cargo_home = home.join(".cargo");
    let vars = [
        ("HILAVITKUTIN_BUILD_TOOLS_HOME", home.as_path()),
        ("CARGO_HOME", cargo_home.as_path()),
    ];

    let built = run_cargo(&root, "build", &[], &vars);
    assert!(!built.passed, "{}", built.said);
    assert!(
        built.said.contains("could not build wild 0.10.0 into")
            && built
                .said
                .contains("run `cargo install --locked --version 0.10.0 --root"),
        "{}",
        built.said
    );
    assert!(!at.join("wild-0.10.0/bin/wild").exists());
    assert!(older.exists(), "a build that failed removed a version");

    // The lock went with the script that held it: a second build takes it
    // again and fails the same way, at once.
    let again = run_cargo(&root, "build", &[], &vars);
    assert!(!again.passed, "{}", again.said);
    assert!(
        again.said.contains("could not build wild 0.10.0 into"),
        "{}",
        again.said
    );
}

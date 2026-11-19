//! `script!` as a consumer meets it: one line in a build script kept at a
//! path of the consumer's own, in a workspace that has never been wired.
//! Each test writes a scratch workspace and builds it with the cargo that
//! runs the suite, offline, with no workspace wrapper named.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use hilavitkutin_build::write_wrapper;

/// A scratch workspace of one crate whose build script is the one line.
///
/// The script carries a line that differs on every run, so cargo builds
/// and runs it again where an earlier run of the suite left its output.
fn scratch(name: &str) -> PathBuf {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    if root.exists() {
        fs::remove_dir_all(&root).expect("the old scratch workspace goes");
    }
    fs::create_dir_all(root.join("scripts")).expect("the scripts directory");
    fs::create_dir_all(root.join("src")).expect("the source directory");
    let build_crate = env!("CARGO_MANIFEST_DIR");
    fs::write(
        root.join("Cargo.toml"),
        format!(
            "[package]\nname = \"scratch-{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\nbuild = \
             \"scripts/anywhere.rs\"\n\n[build-dependencies]\nhilavitkutin-build = {{ path = \
             \"{build_crate}\" }}\n\n[workspace]\n"
        ),
    )
    .expect("the manifest");
    let run = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("a clock after 1970")
        .as_nanos();
    fs::write(
        root.join("scripts/anywhere.rs"),
        format!("// run {run} of process {}\nfn main() {{\n    hilavitkutin_build::script!(Default);\n}}\n", std::process::id()),
    )
    .expect("the build script");
    fs::write(root.join("src/lib.rs"), "").expect("the library");
    root
}

/// Build the workspace and hand back what cargo said.
fn build(root: &Path) -> String {
    let output = Command::new(std::env::var_os("CARGO").expect("cargo runs the suite"))
        .args(["build", "--offline"])
        .current_dir(root)
        .env_remove("RUSTC_WORKSPACE_WRAPPER")
        .env_remove("CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER")
        .output()
        .expect("cargo starts");
    let said = String::from_utf8_lossy(&output.stderr).into_owned();
    assert!(output.status.success(), "{said}");
    said
}

fn wrapper_text() -> String {
    let mut text = String::new();
    write_wrapper(&mut text).expect("writing to a String cannot fail");
    text
}

#[test]
fn the_first_build_wires_the_workspace_and_the_second_says_nothing() {
    let root = scratch("unwired");
    let wrapper = root.join(".cargo/hilavitkutin-rustc-wrapper");
    let config = root.join(".cargo/config.toml");
    assert!(!root.join(".cargo").exists());

    let first = build(&root);
    assert_eq!(
        fs::read_to_string(&wrapper).expect("the wrapper was written"),
        wrapper_text()
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let mode = fs::metadata(&wrapper)
            .expect("the wrapper is there")
            .permissions()
            .mode();
        assert_eq!(mode & 0o111, 0o111, "mode {mode:o}");
    }
    assert_eq!(
        fs::read_to_string(&config).expect("the config file was written"),
        "[build]\nrustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"\n"
    );
    assert_eq!(
        first.matches("hilavitkutin-build wrote").count(),
        1,
        "{first}"
    );
    assert!(first.contains("the next one takes the flags"), "{first}");

    // Cargo now names the wrapper, so the script runs again, through it.
    let second = build(&root);
    assert!(second.contains("Compiling scratch-unwired"), "{second}");
    assert!(!second.contains("warning:"), "{second}");
}

#[test]
fn a_config_file_without_the_line_is_left_as_it_is() {
    let root = scratch("own-config");
    let config = root.join(".cargo/config.toml");
    let theirs = "# the consumer's own\n[build]\nincremental = true\n";
    fs::create_dir_all(root.join(".cargo")).expect("the config directory");
    fs::write(&config, theirs).expect("the consumer's config");

    let said = build(&root);
    assert_eq!(
        fs::read_to_string(&config).expect("the config file"),
        theirs
    );
    assert!(
        said.contains(
            "add `rustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"` under `[build]`"
        ),
        "{said}"
    );
    assert!(!said.contains("hilavitkutin-build wrote"), "{said}");
    assert_eq!(
        fs::read_to_string(root.join(".cargo/hilavitkutin-rustc-wrapper"))
            .expect("the wrapper was written"),
        wrapper_text()
    );
}

#[test]
fn an_unpacked_crate_is_written_nothing() {
    let root = scratch("unpacked");
    fs::write(
        root.join("Cargo.toml.orig"),
        "# what cargo keeps of the manifest it rewrote\n",
    )
    .expect("the marker of an unpacked crate");

    let said = build(&root);
    assert!(!root.join(".cargo").exists());
    assert!(!said.contains("warning:"), "{said}");
}

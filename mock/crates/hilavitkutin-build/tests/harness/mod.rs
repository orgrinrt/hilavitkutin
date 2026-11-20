//! Scratch workspaces for the tests that build one: a crate whose build
//! script is the one line, built by the cargo that runs the suite,
//! offline, with a rustc that writes down the arguments it was run with.

#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use hilavitkutin_build::write_wrapper;

/// A fresh directory for one test.
pub fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    if dir.exists() {
        fs::remove_dir_all(&dir).expect("the old scratch directory goes");
    }
    fs::create_dir_all(&dir).expect("the scratch directory");
    dir
}

/// Write a crate at `dir` whose build script, at a path of its own, is the
/// one line `script!(Default)`. `manifest_tail` is what follows the
/// build-dependency.
pub fn write_crate(dir: &Path, name: &str, manifest_tail: &str) {
    write_crate_with(dir, name, manifest_tail, "", "Default");
}

/// [`write_crate`] with the script's own items, `items`, before its `main`,
/// and `script` between the parentheses of `script!`.
///
/// The script carries a line that differs on every run, so cargo builds
/// and runs it again where an earlier run of the suite left its output.
pub fn write_crate_with(dir: &Path, name: &str, manifest_tail: &str, items: &str, script: &str) {
    fs::create_dir_all(dir.join("scripts")).expect("the scripts directory");
    fs::create_dir_all(dir.join("src")).expect("the source directory");
    let build_crate = env!("CARGO_MANIFEST_DIR");
    fs::write(
        dir.join("Cargo.toml"),
        format!(
            "[package]\nname = \"{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\nbuild = \
             \"scripts/anywhere.rs\"\n\n[build-dependencies]\nhilavitkutin-build = {{ path = \
             \"{build_crate}\" }}\n\n{manifest_tail}"
        ),
    )
    .expect("the manifest");
    let run = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("a clock after 1970")
        .as_nanos();
    fs::write(
        dir.join("scripts/anywhere.rs"),
        format!(
            "// run {run} of process {}\n{items}\nfn main() {{\n    hilavitkutin_build::script!({script});\n}}\n",
            std::process::id()
        ),
    )
    .expect("the build script");
    fs::write(dir.join("src/lib.rs"), "").expect("the library");
}

/// A workspace of one crate, in a fresh directory.
pub fn workspace(name: &str) -> PathBuf {
    workspace_with(name, "", "Default")
}

/// [`workspace`] with the script's own items and what `script!` names.
pub fn workspace_with(name: &str, items: &str, script: &str) -> PathBuf {
    let root = scratch(name).join("ws");
    write_crate_with(
        &root,
        &format!("scratch-{name}"),
        "[workspace]\n",
        items,
        script,
    );
    root
}

/// What one build said and what its rustc was run with.
pub struct Built {
    /// Whether cargo succeeded.
    pub passed: bool,
    /// Cargo's standard error.
    pub said:   String,
    /// One line for each rustc call, its arguments joined by spaces.
    pub rustc:  String,
}

impl Built {
    /// The rustc calls that compiled the crate `krate`.
    pub fn compiles_of(&self, krate: &str) -> Vec<&str> {
        let name = format!("--crate-name {krate} ");
        self.rustc
            .lines()
            .filter(|line| line.contains(&name))
            .collect()
    }
}

/// Build the workspace at `root`, which has to pass. The rustc cargo runs
/// is a script beside the workspace that appends its arguments to a log
/// and runs the real one, so what the wrapper appended can be read back.
pub fn build(root: &Path) -> Built {
    build_with(root, &[])
}

/// [`build`] with more arguments for cargo.
pub fn build_with(root: &Path, more: &[&str]) -> Built {
    let built = run_cargo(root, "build", more, &[]);
    assert!(built.passed, "{}", built.said);
    built
}

/// Run `cargo <verb> --offline` in the workspace at `root`, with more
/// arguments and more variables, and say how it went. It may fail.
pub fn run_cargo(root: &Path, verb: &str, more: &[&str], vars: &[(&str, &Path)]) -> Built {
    let beside = root
        .parent()
        .expect("the workspace sits in a scratch directory");
    let spy = beside.join("rustc-spy");
    let log = beside.join("rustc.log");
    // Asked for its sysroot alone, which is how a script looks for a
    // toolchain component, the spy answers with `SPY_SYSROOT` where a test
    // set one, so the test decides what the toolchain has.
    fs::write(
        &spy,
        "#!/bin/sh\nprintf '%s\\n' \"$*\" >> \"$SPY_LOG\"\nif [ -n \"$SPY_SYSROOT\" ] && [ \"$*\" = '--print sysroot' \
         ]; then\n    printf '%s\\n' \"$SPY_SYSROOT\"\n    exit 0\nfi\nexec rustc \"$@\"\n",
    )
    .expect("the spying rustc");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(&spy, fs::Permissions::from_mode(0o755)).expect("the spy runs");
    }
    if log.exists() {
        fs::remove_file(&log).expect("the old log goes");
    }
    let mut cargo = Command::new(std::env::var_os("CARGO").expect("cargo runs the suite"));
    cargo
        .args([verb, "--offline"])
        .args(more)
        .current_dir(root)
        .env("RUSTC", &spy)
        .env("SPY_LOG", &log)
        .env_remove("RUSTC_WORKSPACE_WRAPPER")
        .env_remove("CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER")
        .env_remove("HILAVITKUTIN_BUILD_TOOLS_HOME")
        .env_remove("SPY_SYSROOT");
    for (name, value) in vars {
        cargo.env(name, value);
    }
    let output = cargo.output().expect("cargo starts");
    Built {
        passed: output.status.success(),
        said:   String::from_utf8_lossy(&output.stderr).into_owned(),
        rustc:  fs::read_to_string(&log).unwrap_or_default(),
    }
}

/// A path for a build on a machine without the programs `gone`: one
/// directory beside the workspace with a link to every program on the
/// suite's path but those. Handed to [`run_cargo`] as `PATH`, it lets a
/// test decide what the machine has.
#[cfg(unix)]
pub fn path_without(beside: &Path, gone: &[&str]) -> PathBuf {
    let dir = beside.join("path");
    fs::create_dir_all(&dir).expect("the path's directory");
    let path = std::env::var_os("PATH").expect("the suite runs under a path");
    for from in std::env::split_paths(&path) {
        for entry in fs::read_dir(&from).into_iter().flatten().flatten() {
            let name = entry.file_name();
            if gone.iter().any(|gone| name.to_str() == Some(*gone)) {
                continue;
            }
            // The first directory of the path that has a name wins, as on
            // the path itself: a link that is already there stays.
            let _ = std::os::unix::fs::symlink(entry.path(), dir.join(&name));
        }
    }
    dir
}

/// A path whose first directory holds a program `name` that prints `line`
/// when asked for its version and fails at anything else, before the
/// suite's own path. A test that needs the machine to have a tool it does
/// not run hands this to [`run_cargo`] as `PATH`.
#[cfg(unix)]
pub fn path_with(beside: &Path, name: &str, line: &str) -> PathBuf {
    use std::os::unix::fs::PermissionsExt as _;

    let dir = beside.join("stand-ins");
    fs::create_dir_all(&dir).expect("the stand-ins' directory");
    let program = dir.join(name);
    fs::write(
        &program,
        format!("#!/bin/sh\nif [ \"$1\" = --version ]; then\n    echo '{line}'\n    exit 0\nfi\nexit 1\n"),
    )
    .expect("the stand-in");
    fs::set_permissions(&program, fs::Permissions::from_mode(0o755)).expect("the stand-in runs");
    let path = std::env::var_os("PATH").expect("the suite runs under a path");
    let all = std::iter::once(dir).chain(std::env::split_paths(&path));
    PathBuf::from(std::env::join_paths(all).expect("a path of directories"))
}

/// The directory cargo builds the workspace at `root` into.
pub fn target_dir(root: &Path) -> PathBuf {
    let asked = Command::new(std::env::var_os("CARGO").expect("cargo runs the suite"))
        .args(["metadata", "--offline", "--no-deps", "--format-version", "1"])
        .current_dir(root)
        .output()
        .expect("cargo starts");
    let text = String::from_utf8_lossy(&asked.stdout);
    let key = "\"target_directory\":\"";
    let from = text.find(key).expect("cargo names its target directory") + key.len();
    let to = text[from ..].find('"').expect("the directory's name ends");
    PathBuf::from(&text[from .. from + to])
}

/// A prototype of the script's own file whose dev set is `dev` and whose
/// other sets are empty, named in `script!` by its path, `crate::<name>`.
pub fn prototype(name: &str, dev: &str) -> String {
    format!(
        "struct {name};\nimpl hilavitkutin_build::Prototype for {name} {{\n    type Dev = {dev};\n    type DevOpt = \
         ();\n    type Release = ();\n    type Profiling = ();\n    type Ci = ();\n}}\n"
    )
}

/// The host triple of the rustc on the path.
pub fn host() -> String {
    let asked = Command::new("rustc")
        .arg("-vV")
        .output()
        .expect("rustc on the path");
    String::from_utf8_lossy(&asked.stdout)
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .expect("rustc names its host")
        .to_owned()
}

pub fn wrapper_text() -> String {
    let mut text = String::new();
    write_wrapper(&mut text).expect("writing to a String cannot fail");
    text
}

/// The `config.toml` a script writes where there is none.
pub const WRITTEN_CONFIG: &str = "include = [{ path = \"hilavitkutin-build.toml\", optional = true }]\n\n\
                                  [build]\nrustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"\n";

/// A flag the dev profile's pragmas choose, which only the wrapper puts on
/// a rustc line.
pub const A_FLAG_OF_OURS: &str = "-Zshare-generics=y";

//! What a build script does about the workspace's wiring, decided here.
//!
//! The wiring is two files under the workspace's `.cargo/`: the rustc
//! wrapper, and a `config.toml` that names it. The script reads what is
//! on disk and asks [`plan`], which answers what to write and what to
//! say. The reads and the writes stay the script's; this module holds the
//! decision and the text.

use core::fmt::Write;

use notko::Maybe;

/// The wrapper's file name under the workspace's `.cargo/`.
pub const WRAPPER_NAME: &str = "hilavitkutin-rustc-wrapper"; // lint:allow(no-bare-static-str) reason: a build-dependency only, it reaches no binary; tracked: no_bare_static_str_does_not_know_a_build_dependency_only_cra

/// The name cargo gives its own wrapper under `cargo clippy`.
const CLIPPY_DRIVER: &str = "clippy-driver"; // lint:allow(no-bare-static-str) reason: a build-dependency only, it reaches no binary; tracked: no_bare_static_str_does_not_know_a_build_dependency_only_cra

/// The key a cargo config file names a workspace wrapper under.
const WRAPPER_KEY: &str = "rustc-workspace-wrapper"; // lint:allow(no-bare-static-str) reason: a build-dependency only, it reaches no binary; tracked: no_bare_static_str_does_not_know_a_build_dependency_only_cra

/// Whose the workspace root is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tree {
    /// The consumer's own workspace, which the script may write into.
    Own,
    /// A root under `CARGO_HOME`, or one that holds `Cargo.toml.orig`:
    /// a tree cargo fetched or unpacked, where nothing is written.
    Fetched,
}

/// The wrapper file as the script found it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WrapperFile {
    /// It holds the text of [`write_wrapper`](crate::write_wrapper).
    Current,
    /// It holds another text.
    Stale,
    /// There is none.
    Missing,
}

/// The workspace's `.cargo/config.toml` as the script found it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigFile {
    /// It names the wrapper.
    Named,
    /// It exists and does not name the wrapper.
    Unnamed,
    /// There is none.
    Missing,
}

impl ConfigFile {
    /// What a config file of this text is: [`Named`](ConfigFile::Named)
    /// where a `rustc-workspace-wrapper` key has a value that ends with
    /// [`WRAPPER_NAME`], [`Unnamed`](ConfigFile::Unnamed) otherwise.
    ///
    /// The key is read where a line starts with it, bare under `[build]`
    /// or dotted as `build.rustc-workspace-wrapper`, so a commented line
    /// names nothing.
    pub fn read(text: &str) -> ConfigFile {
        // lint:allow(no-bare-string) reason: a config file's text as the build script read it; tracked: #72
        let names = text.lines().any(|line| {
            let line = line.trim_start();
            let line = line.strip_prefix("build.").unwrap_or(line);
            let Some(rest) = line.strip_prefix(WRAPPER_KEY) else {
                return false;
            };
            let Some(value) = rest.trim_start().strip_prefix('=') else {
                return false;
            };
            let value = value.split(" #").next().unwrap_or(value).trim();
            value.trim_end_matches(['"', '\'']).ends_with(WRAPPER_NAME)
        });
        if names { ConfigFile::Named } else { ConfigFile::Unnamed }
    }
}

/// What the script says once it has looked at the wiring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Notice {
    /// Nothing.
    Silent,
    /// The files were written, this build ran without them and the next
    /// one takes the flags.
    Wired,
    /// A `config.toml` exists without the line, and the line is named.
    AddLine,
    /// The wiring is in place, cargo runs without it or names another
    /// wrapper, and the flags are dropped.
    Dropped,
}

/// What the script does next.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Plan {
    /// Whether the wrapper file is written.
    pub wrapper: bool, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-public-raw-field) reason: a build-time decision, the crate takes no arvo dependency; tracked: #72
    /// Whether the config file is written.
    pub config:  bool, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-public-raw-field) reason: a build-time decision, the crate takes no arvo dependency; tracked: #72
    /// What the script prints.
    pub notice:  Notice,
}

/// Whether `text` ends with `suffix`, usable where the plan is made.
const fn ends_with(text: &str, suffix: &str) -> bool {
    // lint:allow(no-bare-string) lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: env value as the build script read it; tracked: #72
    let text = text.as_bytes();
    let suffix = suffix.as_bytes();
    if suffix.len() > text.len() {
        return false;
    }
    let skip = text.len() - suffix.len();
    let mut at = 0;
    while at < suffix.len() {
        if text[skip + at] != suffix[at] {
            return false;
        }
        at += 1;
    }
    true
}

/// Whether the value of `RUSTC_WORKSPACE_WRAPPER` names the wrapper, or
/// `clippy-driver`, under which a check loses nothing without the flags.
pub(crate) const fn names_the_wrapper(wrapper: Maybe<&str>) -> bool {
    // lint:allow(no-bare-string) lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: env value as the build script read it; tracked: #72
    match wrapper {
        Maybe::Is(path) => ends_with(path, WRAPPER_NAME) || ends_with(path, CLIPPY_DRIVER),
        Maybe::Isnt => false,
    }
}

/// Decide what the script writes and says.
///
/// `wrapper` is the value of `RUSTC_WORKSPACE_WRAPPER`, and `flags` is
/// whether the crate has rustc flags to hand over. A
/// [`Fetched`](Tree::Fetched) tree is written nothing and told nothing.
/// In the consumer's own tree a wrapper file that is not
/// [`Current`](WrapperFile::Current) is written, and a config file is
/// written where there is none. The wrapper variable decides only what
/// is said: nothing where it names the wrapper or `clippy-driver`, and
/// otherwise [`Wired`](Notice::Wired) where the config file was just
/// written, [`AddLine`](Notice::AddLine) where one exists without the
/// line, and [`Dropped`](Notice::Dropped) where the line is there, held
/// back where there are no flags to drop.
pub const fn plan(
    wrapper: Maybe<&str>, // lint:allow(no-bare-string) reason: env value as the build script read it; tracked: #72
    tree: Tree,
    wrapper_file: WrapperFile,
    config_file: ConfigFile,
    flags: bool, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a build-time predicate, the crate takes no arvo dependency; tracked: #72
) -> Plan {
    if let Tree::Fetched = tree {
        return Plan {
            wrapper: false,
            config:  false,
            notice:  Notice::Silent,
        };
    }
    let notice = if names_the_wrapper(wrapper) {
        Notice::Silent
    } else {
        match config_file {
            ConfigFile::Missing => Notice::Wired,
            ConfigFile::Unnamed => Notice::AddLine,
            ConfigFile::Named if flags => Notice::Dropped,
            ConfigFile::Named => Notice::Silent,
        }
    };
    Plan {
        wrapper: !matches!(wrapper_file, WrapperFile::Current),
        config: matches!(config_file, ConfigFile::Missing),
        notice,
    }
}

/// Write the `config.toml` a workspace gets where it has none: a
/// `[build]` table with the one line that names the wrapper.
pub fn write_cargo_config<W: Write>(out: &mut W) -> core::fmt::Result {
    writeln!(out, "[build]\n{WRAPPER_KEY} = \".cargo/{WRAPPER_NAME}\"")
}

/// Write the `cargo::warning` line of `notice`, or nothing for
/// [`Silent`](Notice::Silent).
///
/// `dir` is the workspace's `.cargo` directory as the script shows it.
pub fn write_notice<W: Write, D: core::fmt::Display>(
    notice: Notice,
    dir: D,
    out: &mut W,
) -> core::fmt::Result {
    match notice {
        Notice::Silent => Ok(()),
        Notice::Wired => {
            writeln!(
                out,
                "cargo::warning=hilavitkutin-build wrote {WRAPPER_NAME} and the config.toml that names it under \
                 {dir}: this build ran without the wrapper, and the next one takes the flags"
            )
        },
        Notice::AddLine => {
            writeln!(
                out,
                "cargo::warning=hilavitkutin-build left {dir}/config.toml as it is, and it names no wrapper: add \
                 `{WRAPPER_KEY} = \".cargo/{WRAPPER_NAME}\"` under `[build]`, or the flags chosen for this \
                 crate are dropped"
            )
        },
        Notice::Dropped => {
            writeln!(
                out,
                "cargo::warning=hilavitkutin-build chose rustc flags for this crate and cargo runs it without the \
                 wrapper {dir}/config.toml names, so they are dropped: run cargo from inside the workspace"
            )
        },
    }
}

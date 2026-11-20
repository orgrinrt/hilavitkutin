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

/// Whose the workspace root is, and whether the host can run the wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tree {
    /// The consumer's own workspace, which the script may write into.
    Own,
    /// A root under `CARGO_HOME`, or one that holds `Cargo.toml.orig`:
    /// a tree cargo fetched or unpacked, where nothing is written.
    Fetched,
    /// A host that is not unix. The wrapper is a POSIX `sh` script, and a
    /// config file naming one there stops every later build, so nothing
    /// is written.
    NoShell,
    /// A root other than the one the wrapper cargo names sits in: a
    /// workspace wired from a `config.toml` above it, where a second
    /// copy of the wiring would serve nothing.
    Foreign,
}

/// Whether the root holds `Cargo.toml.orig`, which cargo leaves in a
/// crate it unpacked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Unpacked {
    /// The root holds the file.
    Is,
    /// It does not.
    Isnt,
}

/// Where the wrapper cargo names sits, for a wrapper of ours.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Placed {
    /// Cargo names this root's own wrapper file, or no wrapper of ours.
    Here,
    /// Cargo names a wrapper of ours that is not this root's own file.
    Elsewhere,
}

/// The kind of host the script runs on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Host {
    /// A unix host, which runs a POSIX `sh` wrapper.
    Unix,
    /// Any other host.
    Other,
}

/// Whether the crate has rustc flags for the wrapper to hand over.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Flags {
    /// A pragma of the set writes a flag.
    Written,
    /// None does.
    Nothing,
}

impl Tree {
    /// Decide the tree from what the script found.
    ///
    /// `root` is the workspace root and `cargo_home` cargo's home, both as
    /// the script shows them. A fetched tree comes first, so it is told
    /// nothing on any host. A root is under the cargo home by whole path
    /// components: one that only starts with the same letters is not.
    #[rustfmt::skip] // keeps the allows on the parameters they govern
    pub const fn of(
        root: &str, // lint:allow(no-bare-string) reason: a path as the build script read it; tracked: #72
        cargo_home: Maybe<&str>, // lint:allow(no-bare-string) reason: a path as the build script read it; tracked: #72
        unpacked: Unpacked,
        placed: Placed,
        host: Host,
    ) -> Tree {
        let under_home = match cargo_home {
            Maybe::Is(home) => is_under(root, home),
            Maybe::Isnt => false,
        };
        if under_home || matches!(unpacked, Unpacked::Is) {
            Tree::Fetched
        } else if matches!(host, Host::Other) {
            Tree::NoShell
        } else if matches!(placed, Placed::Elsewhere) {
            Tree::Foreign
        } else {
            Tree::Own
        }
    }
}

/// Whether the byte separates the components of a path on some host.
const fn is_separator(byte: u8) -> bool {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a path's bytes as the build script read them; tracked: #72
    byte == b'/' || byte == b'\\'
}

/// Whether `path` is `dir` or lies under it, by whole components.
#[rustfmt::skip] // keeps the allow on the signature it governs
const fn is_under(path: &str, dir: &str) -> bool { // lint:allow(no-bare-string) lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: paths as the build script read them; tracked: #72
    let path = path.as_bytes();
    let dir = dir.as_bytes();
    if dir.is_empty() || dir.len() > path.len() {
        return false;
    }
    let mut at = 0;
    while at < dir.len() {
        if path[at] != dir[at] {
            return false;
        }
        at += 1;
    }
    path.len() == dir.len() || is_separator(dir[dir.len() - 1]) || is_separator(path[dir.len()])
}

/// Whether the last component of `path` is `name`.
#[rustfmt::skip] // keeps the allow on the signature it governs
const fn has_file_name(path: &str, name: &str) -> bool { // lint:allow(no-bare-string) lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a path as the build script read it; tracked: #72
    if !ends_with(path, name) {
        return false;
    }
    let path = path.as_bytes();
    path.len() == name.len() || is_separator(path[path.len() - name.len() - 1])
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
    /// where a `rustc-workspace-wrapper` key has a path whose file name is
    /// [`WRAPPER_NAME`], [`Unnamed`](ConfigFile::Unnamed) otherwise.
    ///
    /// The key is read in the three places cargo takes it from: at the
    /// start of a line under the `[build]` table, dotted as
    /// `build.rustc-workspace-wrapper` before any table, and inside an
    /// inline `build = { .. }` table. The same key under another table, or
    /// in a comment, names nothing.
    #[rustfmt::skip] // keeps the allow on the signature it governs
    pub fn read(text: &str) -> ConfigFile { // lint:allow(no-bare-string) reason: a config file's text as the build script read it; tracked: #72
        let mut table = Table::Top;
        for line in text.lines() {
            let line = line.trim();
            if let Some(header) = line.strip_prefix('[') {
                let name = header.split(']').next().unwrap_or(header).trim();
                table = if name == "build" { Table::Build } else { Table::Other };
                continue;
            }
            let entry = match table {
                Table::Build => Maybe::Is(line),
                Table::Top => top_level_entry(line),
                Table::Other => Maybe::Isnt,
            };
            if let Maybe::Is(entry) = entry
                && entry_names_the_wrapper(entry)
            {
                return ConfigFile::Named;
            }
        }
        ConfigFile::Unnamed
    }
}

/// The table a line of a config file sits in.
#[derive(Clone, Copy)]
enum Table {
    /// Before any table header.
    Top,
    /// Under `[build]`.
    Build,
    /// Under any other header.
    Other,
}

/// The part of a line before any table that could hold the key: what
/// follows `build.`, or the key onwards inside an inline `build` table.
#[rustfmt::skip] // keeps the allow on the signature it governs
fn top_level_entry(line: &str) -> Maybe<&str> { // lint:allow(no-bare-string) reason: a config file's text as the build script read it; tracked: #72
    if let Some(dotted) = line.strip_prefix("build.") {
        return Maybe::Is(dotted);
    }
    let inline = line
        .strip_prefix("build")
        .and_then(|rest| rest.trim_start().strip_prefix('='))
        .and_then(|rest| rest.trim_start().strip_prefix('{'));
    match inline {
        Some(inside) => match inside.find(WRAPPER_KEY) {
            Some(at) => Maybe::Is(&inside[at..]),
            None => Maybe::Isnt,
        },
        None => Maybe::Isnt,
    }
}

/// Whether `entry` starts with the key and gives it a quoted path whose
/// file name is [`WRAPPER_NAME`].
#[rustfmt::skip] // keeps the allow on the signature it governs
fn entry_names_the_wrapper(entry: &str) -> bool { // lint:allow(no-bare-string) lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a config file's text as the build script read it; tracked: #72
    let quoted = entry
        .strip_prefix(WRAPPER_KEY)
        .and_then(|rest| rest.trim_start().strip_prefix('='))
        .map(str::trim_start);
    let Some(quoted) = quoted else {
        return false;
    };
    let Some(quote) = quoted.chars().next().filter(|first| matches!(first, '"' | '\'')) else {
        return false;
    };
    let value = &quoted[1..];
    match value.find(quote) {
        Some(end) => has_file_name(&value[..end], WRAPPER_NAME),
        None => false,
    }
}

/// What the script says once it has looked at the wiring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Notice {
    /// Nothing.
    Silent,
    /// The wiring was put in place, this build ran without it, and a
    /// build of that workspace takes the flags from the next one on.
    Wired,
    /// A `config.toml` exists without the line, and the line is named.
    AddLine,
    /// The wiring is in place and cargo was started without it, or with
    /// another wrapper, so the flags of this build are dropped. It is
    /// what every script but one says in the first build of a workspace
    /// of several members, and what a cargo started elsewhere gets.
    Dropped,
    /// The host runs no POSIX `sh` wrapper, so the flags are dropped.
    NoShell,
    /// The generated cargo file was written, this build ran before cargo
    /// read it, and the next one takes its keys.
    Generated,
    /// The generated cargo file is in place and `config.toml` has no
    /// `include` line for it, so cargo never reads it. The line is named.
    AddInclude,
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
        Maybe::Is(path) => has_file_name(path, WRAPPER_NAME) || has_file_name(path, CLIPPY_DRIVER),
        Maybe::Isnt => false,
    }
}

/// Decide what the script writes and says.
///
/// `wrapper` is the value of `RUSTC_WORKSPACE_WRAPPER`. A
/// [`Fetched`](Tree::Fetched) or [`Foreign`](Tree::Foreign) tree is
/// written nothing and told nothing, and a [`NoShell`](Tree::NoShell)
/// tree is written nothing and told the flags are dropped where there
/// are any.
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
    flags: Flags,
) -> Plan {
    let flags = matches!(flags, Flags::Written);
    let left_alone = match tree {
        Tree::Own => Maybe::Isnt,
        Tree::Fetched | Tree::Foreign => Maybe::Is(Notice::Silent),
        Tree::NoShell if flags => Maybe::Is(Notice::NoShell),
        Tree::NoShell => Maybe::Is(Notice::Silent),
    };
    if let Maybe::Is(notice) = left_alone {
        return Plan {
            wrapper: false,
            config: false,
            notice,
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

/// The line by which a `config.toml` takes the generated cargo file. The
/// path is relative to the file that holds the line, and `optional` makes
/// a file that is not there yet no error.
#[rustfmt::skip] // keeps the allow on the line it governs
pub const INCLUDE_LINE: &str = "include = [{ path = \"hilavitkutin-build.toml\", optional = true }]"; // lint:allow(no-bare-static-str) reason: a build-dependency only, it reaches no binary; tracked: no_bare_static_str_does_not_know_a_build_dependency_only_cra

/// Write the `config.toml` a workspace gets where it has none: the line
/// that takes the generated cargo file, and a `[build]` table with the
/// one line that names the wrapper.
pub fn write_cargo_config<W: Write>(out: &mut W) -> core::fmt::Result {
    writeln!(
        out,
        "{INCLUDE_LINE}\n\n[build]\n{WRAPPER_KEY} = \".cargo/{WRAPPER_NAME}\""
    )
}

/// Write the two `cargo::rerun-if-changed` lines that name the wrapper
/// and the config file under `dir`, the workspace's `.cargo` directory.
///
/// With them the script runs again when either file changes, so a wrapper
/// that holds another text is written again by the next build, and a
/// config file that gains the line stops the notice.
pub fn write_watch<W: Write, D: core::fmt::Display>(dir: D, out: &mut W) -> core::fmt::Result {
    writeln!(
        out,
        "cargo::rerun-if-changed={dir}/{WRAPPER_NAME}\ncargo::rerun-if-changed={dir}/config.toml"
    )
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
                "cargo::warning=hilavitkutin-build put {WRAPPER_NAME} and the config.toml that names it in place \
                 under {dir}: this build ran without the wrapper, and a build of that workspace takes the flags \
                 from the next one on"
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
                "cargo::warning=hilavitkutin-build chose rustc flags for this crate and cargo was started without \
                 the wrapper {dir}/config.toml names, so they are dropped in this build: a build started inside \
                 the workspace, once that file is there, takes them"
            )
        },
        Notice::NoShell => {
            writeln!(
                out,
                "cargo::warning=hilavitkutin-build chose rustc flags for this crate and hands them over through \
                 a POSIX sh wrapper, which this host does not run, so they are dropped"
            )
        },
        Notice::Generated => {
            writeln!(
                out,
                "cargo::warning=hilavitkutin-build wrote the workspace's pragmas to {dir}/hilavitkutin-build.toml: \
                 this build ran before cargo read it, and the next one takes its keys"
            )
        },
        Notice::AddInclude => {
            writeln!(
                out,
                "cargo::warning=hilavitkutin-build left {dir}/config.toml as it is, and it does not take \
                 {dir}/hilavitkutin-build.toml: add `{INCLUDE_LINE}` at its top, or the workspace's pragmas \
                 are dropped"
            )
        },
    }
}

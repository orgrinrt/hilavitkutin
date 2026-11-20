//! The pinned toolchain, resolved once to the binaries a call runs, and what
//! it says of itself: its host, the version its own `rust-lld` reports and the
//! target's own description, from which hilavitkutin-build reads which linker
//! is the toolchain's own and the mark it leaves.
//!
//! The channel is read from `mock/rust-toolchain.toml`. It is resolved through
//! rustup once, with every inherited variable but the ones rustup needs to find
//! its home taken out; after that the toolchain's `cargo` and `rustc` are run
//! by path, so no proxy and no override reaches a call.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

use hilavitkutin_build::{RustLldMark, ToolchainLinker, rust_lld_mark};

/// The variables rustup's proxy needs to find its toolchains, and nothing else.
const RUSTUP_NEEDS: [&str; 3] = ["PATH", "HOME", "RUSTUP_HOME"];

/// The pinned toolchain.
#[derive(Clone, Debug)]
pub struct Toolchain {
    pub channel: String,
    pub sysroot: PathBuf,
    pub cargo:   PathBuf,
    pub rustc:   PathBuf,
    /// The host triple, which is the target every call builds for.
    pub host:    String,
    /// What the toolchain's own `rust-lld` says of itself with `--version`,
    /// where the toolchain carries one for the host and it answered.
    pub lld:     Option<String>,
    /// rustc's description of the host target, as JSON text.
    pub spec:    String,
}

impl Toolchain {
    /// The mark the toolchain's own linker leaves in an executable it links
    /// for the host. See [`own_mark`].
    pub fn own_mark(&self) -> Result<String, String> {
        own_mark(&self.spec, self.lld.as_deref(), &self.host)
    }
}

/// The pinned toolchain, resolved on first use. Panics where it cannot be,
/// which fails the call that asked.
pub fn get() -> &'static Toolchain {
    static TOOLCHAIN: OnceLock<Toolchain> = OnceLock::new();
    TOOLCHAIN.get_or_init(|| resolve().unwrap_or_else(|e| panic!("build-call: {e}")))
}

// The resolving stays here: hilavitkutin-build touches neither the
// environment nor the filesystem, so whoever calls it does the looking and
// hands it what the toolchain printed.
fn resolve() -> Result<Toolchain, String> {
    let channel = channel();
    let mut proxy = Command::new("rustc");
    proxy.env_clear();
    for v in RUSTUP_NEEDS {
        if let Some(value) = std::env::var_os(v) {
            proxy.env(v, value);
        }
    }
    proxy
        .arg(format!("+{channel}"))
        .args(["--print", "sysroot"]);
    let sysroot = PathBuf::from(said(proxy, "the toolchain's sysroot")?.trim());
    let bin = |name: &str| {
        sysroot
            .join("bin")
            .join(format!("{name}{}", std::env::consts::EXE_SUFFIX))
    };
    let (cargo, rustc) = (bin("cargo"), bin("rustc"));
    for b in [&cargo, &rustc] {
        if !b.is_file() {
            return Err(format!("{}: not in the pinned toolchain", b.display()));
        }
    }
    let verbose = said(own(&rustc, &["-vV"]), "rustc -vV")?;
    let field = |key: &str| {
        verbose
            .lines()
            .find_map(|l| l.strip_prefix(key))
            .map(|v| v.trim().to_string())
            .ok_or_else(|| format!("rustc -vV names no `{key}`"))
    };
    let host = field("host:")?;
    let lld = lld_version(&sysroot, &host);
    let spec = said(
        own(&rustc, &[
            "-Z",
            "unstable-options",
            "--print",
            "target-spec-json",
        ]),
        "the host target's description",
    )?;
    Ok(Toolchain {
        channel,
        sysroot,
        cargo,
        rustc,
        host,
        lld,
        spec,
    })
}

/// The first line `rust-lld` under `<sysroot>/lib/rustlib/<host>/bin/` prints
/// for `--version` as the GNU flavor, or `None` where the toolchain carries no
/// such binary or it does not answer.
///
/// FIXME(the_build_and_link_bench_assumes_an_elf_unix_host_and_fails): the GNU
/// flavor is the one an ELF target links with; a Mach-O or PE host's own lld
/// is asked as the wrong flavor, so its link cells fail on the mark.
fn lld_version(sysroot: &Path, host: &str) -> Option<String> {
    let lld = sysroot
        .join("lib/rustlib")
        .join(host)
        .join("bin")
        .join(format!("rust-lld{}", std::env::consts::EXE_SUFFIX));
    if !lld.is_file() {
        return None;
    }
    said(
        own(&lld, &["-flavor", "gnu", "--version"]),
        "rust-lld --version",
    )
    .ok()
    .and_then(|t| t.lines().next().map(|l| l.trim().to_string()))
}

/// A toolchain binary, run with no inherited environment but the path.
fn own(binary: &Path, args: &[&str]) -> Command {
    let mut c = Command::new(binary);
    crate::cargo::keep_only_allowed(&mut c);
    c.args(args);
    c
}

/// What `c` printed, or why it failed.
fn said(mut c: Command, what: &str) -> Result<String, String> {
    let out = c.output().map_err(|e| format!("{what}: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "{what} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// The pinned toolchain channel, from the mock workspace's toolchain file.
pub fn channel() -> String {
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../rust-toolchain.toml");
    let text = std::fs::read_to_string(&file).unwrap_or_else(|e| panic!("{}: {e}", file.display()));
    text.lines()
        .find_map(|l| {
            let l = l.trim();
            l.strip_prefix("channel")
                .and_then(|rest| rest.trim_start().strip_prefix('='))
                .map(|v| v.trim().trim_matches('"').to_string())
        })
        .unwrap_or_else(|| panic!("{}: no channel line", file.display()))
}

/// The mark the toolchain's own linker for the target `spec` describes leaves
/// in what it links, where `lld` is what the toolchain's `rust-lld` said for
/// `--version`.
///
/// Which linker that is and what it writes are hilavitkutin-build's to say
/// (`ToolchainLinker::of`, `rust_lld_mark`); this function words the refusal
/// where the crate has no mark. A toolchain linker other than its own
/// `rust-lld` has none.
pub fn own_mark(spec: &str, lld: Option<&str>, host: &str) -> Result<String, String> {
    match ToolchainLinker::of(spec) {
        ToolchainLinker::RustLld => {
            let said = lld.ok_or_else(|| {
                format!("the toolchain links {host} with its own lld, and no rust-lld of it said its version")
            })?;
            Option::from(rust_lld_mark(said))
                .map(|mark: RustLldMark<'_>| mark.to_string())
                .ok_or_else(|| {
                    format!("rust-lld --version said `{said}`, which is not the version line of a GNU-flavor lld")
                })
        },
        // FIXME(the_build_and_link_bench_assumes_an_elf_unix_host_and_fails): a
        // target whose own linker is another one (GNU ld through cc, ld64,
        // link.exe) has no mark, so its link cells fail instead of being left
        // out.
        ToolchainLinker::Other(flavor) => {
            Err(format!(
                "the toolchain's own linker for {host} is of flavor `{flavor}` or is not its own copy, and \
                 hilavitkutin-build has a mark for the toolchain's rust-lld only"
            ))
        },
    }
}

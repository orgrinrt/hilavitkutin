//! The pinned toolchain, resolved once to the binaries a call runs, and what
//! it says of itself: its host, its LLVM version and the target's own
//! description, from which the default linker's mark is worked out.
//!
//! The channel is read from `mock/rust-toolchain.toml`. It is resolved through
//! rustup once, with every inherited variable but the ones rustup needs to find
//! its home taken out; after that the toolchain's `cargo` and `rustc` are run
//! by path, so no proxy and no override reaches a call.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

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
    /// The LLVM version rustc reports, `22.1.6` and the like.
    pub llvm:    String,
    /// rustc's description of the host target, as JSON text.
    pub spec:    String,
}

impl Toolchain {
    /// The mark the toolchain's default linker leaves in an executable it
    /// links for the host. See [`default_mark`].
    pub fn default_mark(&self) -> Result<String, String> {
        default_mark(&self.spec, &self.llvm, &self.host)
    }
}

/// The pinned toolchain, resolved on first use. Panics where it cannot be,
/// which fails the call that asked.
pub fn get() -> &'static Toolchain {
    static TOOLCHAIN: OnceLock<Toolchain> = OnceLock::new();
    TOOLCHAIN.get_or_init(|| resolve().unwrap_or_else(|e| panic!("build-call: {e}")))
}

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
    let (host, llvm) = (field("host:")?, field("LLVM version:")?);
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
        llvm,
        spec,
    })
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

/// The mark the default linker of the target `spec` describes leaves in what
/// it links, worked out from the description and the toolchain's LLVM version.
///
/// Where the target's linker flavor is an lld one and the toolchain links with
/// its own copy (`link-self-contained` names the linker), the linker is the
/// toolchain's `rust-lld`, built from the same LLVM as rustc, and its mark is
/// `Linker: LLD <llvm>`. Any other default is refused: the bench has no way
/// yet to know what that linker writes.
pub fn default_mark(spec: &str, llvm: &str, host: &str) -> Result<String, String> {
    let flavor = string_field(spec, "linker-flavor").unwrap_or_default();
    let own_linker =
        object_field(spec, "link-self-contained").is_some_and(|o| o.contains("\"linker\""));
    if flavor.contains("lld") && own_linker {
        return Ok(format!("Linker: LLD {llvm}"));
    }
    // FIXME(the_build_and_link_bench_assumes_an_elf_unix_host_and_fails): a
    // target whose default is another linker (GNU ld through cc, ld64, link.exe)
    // has no mark here, so its link cells fail instead of being left out.
    Err(format!(
        "the toolchain's default linker for {host} is of flavor `{flavor}`{}, and this bench knows the mark of the \
         toolchain's own lld only",
        if own_linker { "" } else { ", not the toolchain's own" }
    ))
}

/// The string value of `"key": "value"` in a JSON text.
fn string_field(json: &str, key: &str) -> Option<String> {
    let after = &json[json.find(&format!("\"{key}\""))? + key.len() + 2 ..];
    let after = after
        .trim_start()
        .strip_prefix(':')?
        .trim_start()
        .strip_prefix('"')?;
    Some(after[.. after.find('"')?].to_string())
}

/// The text of the object or value after `"key":` in a JSON text, up to its
/// closing brace where it is an object.
fn object_field<'a>(json: &'a str, key: &str) -> Option<&'a str> {
    let after = &json[json.find(&format!("\"{key}\""))? + key.len() + 2 ..];
    let after = after.trim_start().strip_prefix(':')?.trim_start();
    Some(match after.strip_prefix('{') {
        Some(body) => &body[.. body.find('}')?],
        None => &after[.. after.find([',', '\n']).unwrap_or(after.len())],
    })
}

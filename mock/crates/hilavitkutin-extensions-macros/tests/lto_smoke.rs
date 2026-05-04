//! Verifies that `#[export_extension]`-emitted trampolines survive
//! aggressive `lto = "fat"` cdylib builds.
//!
//! The fixture under `tests/lto_smoke_fixture/` is a standalone cdylib
//! that uses `#[export_extension(init = Init, shutdown = Shutdown)]`. We
//! build it with `cargo build --release`, then run `nm` on the resulting
//! shared library and assert that:
//!
//! 1. `__hilavitkutin_extension_descriptor` is exported (the entry point).
//! 2. The init / shutdown trampoline names appear as substrings in the
//!    symbol table. Rust mangling keeps the original ident as a substring
//!    within the mangled form (Itanium-style `_ZN<len><name>17h<hash>E`),
//!    so `ext_init_trampoline` and `ext_shutdown_trampoline` are findable
//!    even though the full mangled symbol differs.
//!
//! Skipped on non-Unix targets: `nm` is not the standard tool on Windows;
//! parity verification via `dumpbin /SYMBOLS` is a separate harness
//! tracked in the macros crate's BACKLOG.

use std::path::PathBuf;
use std::process::Command;

const FIXTURE_PKG: &str = "lto-smoke-fixture";
const FIXTURE_LIB: &str = "lto_smoke_fixture";

#[cfg(unix)]
#[test]
fn trampolines_survive_lto() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_manifest = manifest_dir
        .join("tests")
        .join("lto_smoke_fixture")
        .join("Cargo.toml");
    assert!(
        fixture_manifest.exists(),
        "lto_smoke_fixture/Cargo.toml missing at {:?}",
        fixture_manifest,
    );

    let build = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--manifest-path")
        .arg(&fixture_manifest)
        .output()
        .expect("invoke cargo build");
    if !build.status.success() {
        panic!(
            "cargo build --release failed for the LTO fixture\n--- stdout ---\n{}\n--- stderr ---\n{}",
            String::from_utf8_lossy(&build.stdout),
            String::from_utf8_lossy(&build.stderr),
        );
    }

    let target_dir = manifest_dir
        .join("tests")
        .join("lto_smoke_fixture")
        .join("target")
        .join("release");
    let candidates = [
        target_dir.join(format!("lib{FIXTURE_LIB}.dylib")),
        target_dir.join(format!("lib{FIXTURE_LIB}.so")),
    ];
    let cdylib = candidates
        .iter()
        .find(|p| p.exists())
        .unwrap_or_else(|| panic!("no cdylib output found among {:?}", candidates));

    let nm = Command::new("nm")
        .arg(cdylib)
        .output()
        .expect("invoke nm");
    if !nm.status.success() {
        panic!(
            "nm failed on {:?}\n--- stderr ---\n{}",
            cdylib,
            String::from_utf8_lossy(&nm.stderr),
        );
    }
    let symbols = String::from_utf8_lossy(&nm.stdout);

    assert!(
        symbols.contains("__hilavitkutin_extension_descriptor"),
        "exported descriptor function missing from {:?}; nm output:\n{}",
        cdylib,
        symbols,
    );
    assert!(
        symbols.contains("ext_init_trampoline"),
        "init trampoline did not survive LTO in {:?}; nm output:\n{}",
        cdylib,
        symbols,
    );
    assert!(
        symbols.contains("ext_shutdown_trampoline"),
        "shutdown trampoline did not survive LTO in {:?}; nm output:\n{}",
        cdylib,
        symbols,
    );

    // Suppress the unused-import warning on platforms that compile this
    // module but skip the body via cfg.
    let _ = FIXTURE_PKG;
}

#[cfg(not(unix))]
#[test]
fn trampolines_survive_lto() {
    eprintln!("LTO smoke test is unix-only; tracked in BACKLOG.");
}

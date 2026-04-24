//! Integration tests for hilavitkutin-extensions.
//!
//! These tests load a well-known system library (`libc`) and resolve
//! a known symbol (`getpid`). This avoids the complexity of building a
//! custom cdylib fixture at test time; a custom fixture is tracked in
//! BACKLOG.md.tmpl as a follow-up.

#![cfg_attr(not(any(unix, windows)), allow(unused))]

use hilavitkutin_extensions::{Extension, ExtensionError, compatibility_check};
use notko::Outcome;

#[cfg(target_os = "macos")]
const LIBC_PATH: &[u8] = b"/usr/lib/libSystem.B.dylib\0";

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const LIBC_PATH: &[u8] = b"/lib/x86_64-linux-gnu/libc.so.6\0";

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
const LIBC_PATH: &[u8] = b"/lib/aarch64-linux-gnu/libc.so.6\0";

#[cfg(windows)]
const LIBC_PATH: &[u8] = b"msvcrt.dll\0";

#[cfg(any(target_os = "macos", target_os = "linux", windows))]
#[test]
fn load_system_libc() {
    match Extension::load(LIBC_PATH) {
        Outcome::Ok(_ext) => {}
        Outcome::Err(_) => panic!("system libc should load"),
    }
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[test]
fn resolve_known_symbol() {
    let ext = match Extension::load(LIBC_PATH) {
        Outcome::Ok(ext) => ext,
        Outcome::Err(_) => panic!("system libc should load"),
    };
    // `getpid` is a signature-free 0-arity function returning i32
    type GetPid = extern "C" fn() -> i32;
    match ext.resolve::<GetPid>(b"getpid\0") {
        Outcome::Ok(sym) => {
            let fn_ptr = sym.get();
            let pid = fn_ptr();
            assert!(pid > 0, "getpid returned non-positive value");
        }
        Outcome::Err(_) => panic!("getpid should resolve in libc"),
    }
}

#[test]
fn reject_missing_path() {
    match Extension::load(b"/nonexistent/library/path.so\0") {
        Outcome::Ok(_) => panic!("nonexistent path should not load"),
        Outcome::Err(ExtensionError::LoadFailed { .. }) => {}
        Outcome::Err(ExtensionError::PathNotFound) => {}
        Outcome::Err(_) => panic!("wrong error variant"),
    }
}

#[test]
fn reject_path_without_null_terminator() {
    match Extension::load(b"/some/path") {
        Outcome::Ok(_) => panic!("un-terminated path should not load"),
        Outcome::Err(ExtensionError::PathNotFound) => {}
        Outcome::Err(_) => panic!("wrong error variant"),
    }
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[test]
fn reject_missing_symbol() {
    let ext = match Extension::load(LIBC_PATH) {
        Outcome::Ok(ext) => ext,
        Outcome::Err(_) => panic!("system libc should load"),
    };
    type Nothing = extern "C" fn() -> i32;
    match ext.resolve::<Nothing>(b"absolutely_does_not_exist_xyz\0") {
        Outcome::Ok(_) => panic!("nonexistent symbol should not resolve"),
        Outcome::Err(ExtensionError::SymbolMissing) => {}
        Outcome::Err(_) => panic!("wrong error variant"),
    }
}

#[test]
fn compatibility_check_rejects_empty() {
    match compatibility_check(b"") {
        Outcome::Ok(()) => panic!("empty bytes should be rejected"),
        Outcome::Err(_) => {}
    }
}

#[test]
fn compatibility_check_accepts_nonempty() {
    match compatibility_check(b"version-1") {
        Outcome::Ok(()) => {}
        Outcome::Err(_) => panic!("non-empty bytes should pass v1 stub"),
    }
}

//! Integration tests for hilavitkutin-linking.
//!
//! These tests load a well-known system library (`libc`) and exercise
//! the public surface: function-pointer resolution via `Symbol`,
//! static-data resolution via `StaticRef`, explicit `Library::close`,
//! and compatibility_check.

#![cfg_attr(not(any(unix, windows)), allow(unused))]

use hilavitkutin_linking::{Library, LinkError, compatibility_check};
use notko::Outcome;

#[cfg(target_os = "macos")]
const LIBC_PATH: &[u8] = b"/usr/lib/libSystem.B.dylib\0";

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const LIBC_PATH: &[u8] = b"/lib/x86_64-linux-gnu/libc.so.6\0";

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
const LIBC_PATH: &[u8] = b"/lib/aarch64-linux-gnu/libc.so.6\0";

#[cfg(windows)]
const LIBC_PATH: &[u8] = b"msvcrt.dll\0";

// `optopt` is a standard libc global of type `int` on unix. It lives at
// a known symbol and serves as a test fixture for StaticRef::get
// (pointer-to-static-data resolution, the typical extension descriptor
// access pattern).
#[cfg(target_os = "macos")]
const STATIC_SYMBOL_NAME: &[u8] = b"optopt\0";

#[cfg(target_os = "linux")]
const STATIC_SYMBOL_NAME: &[u8] = b"optopt\0";

#[cfg(any(target_os = "macos", target_os = "linux", windows))]
#[test]
fn load_system_libc() {
    match Library::load(LIBC_PATH) {
        Outcome::Ok(_ext) => {}
        Outcome::Err(_) => panic!("system libc should load"),
    }
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[test]
fn resolve_known_symbol() {
    let ext = match Library::load(LIBC_PATH) {
        Outcome::Ok(ext) => ext,
        Outcome::Err(_) => panic!("system libc should load"),
    };
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

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[test]
fn resolve_static_known_symbol() {
    let ext = match Library::load(LIBC_PATH) {
        Outcome::Ok(ext) => ext,
        Outcome::Err(_) => panic!("system libc should load"),
    };
    match ext.resolve_static::<i32>(STATIC_SYMBOL_NAME) {
        Outcome::Ok(static_ref) => {
            // Deref through the typed wrapper. The value is libc's
            // optopt global (0 if no getopt error has occurred, which
            // is the state in a fresh test process). The important
            // assertion is that the deref does not SIGSEGV / SIGBUS.
            let value: i32 = *static_ref.get();
            let _ = value; // suppress unused_assignments lint on some toolchains
        }
        Outcome::Err(_) => panic!(
            "optopt should resolve as static-data symbol in libc"
        ),
    }
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[test]
fn explicit_close_happy_path() {
    let ext = match Library::load(LIBC_PATH) {
        Outcome::Ok(ext) => ext,
        Outcome::Err(_) => panic!("system libc should load"),
    };
    match ext.close() {
        Outcome::Ok(()) => {}
        Outcome::Err(_) => panic!("close should succeed on libc"),
    }
}

// Compile-only test for higher-arity fn pointer resolution. The
// symbol will not exist, so the test only verifies the type compiles
// through the LibrarySymbol sealed trait at arity 5.
#[cfg(any(target_os = "macos", target_os = "linux"))]
#[test]
fn extended_arity_type_compiles() {
    let ext = match Library::load(LIBC_PATH) {
        Outcome::Ok(ext) => ext,
        Outcome::Err(_) => panic!("system libc should load"),
    };
    type FiveArgFn = extern "C" fn(i32, i32, i32, i32, i32) -> i32;
    // Resolution will fail (no such symbol in libc); we only assert
    // that the type-level plumbing through arity 5 works.
    match ext.resolve::<FiveArgFn>(b"this_symbol_does_not_exist_5args\0") {
        Outcome::Ok(_) => panic!("symbol should not exist"),
        Outcome::Err(LinkError::SymbolMissing) => {}
        Outcome::Err(_) => panic!("wrong error variant"),
    }
}

#[test]
fn reject_missing_path() {
    match Library::load(b"/nonexistent/library/path.so\0") {
        Outcome::Ok(_) => panic!("nonexistent path should not load"),
        Outcome::Err(LinkError::LoadFailed { .. }) => {}
        Outcome::Err(LinkError::PathNotFound) => {}
        Outcome::Err(_) => panic!("wrong error variant"),
    }
}

#[test]
fn reject_path_without_null_terminator() {
    match Library::load(b"/some/path") {
        Outcome::Ok(_) => panic!("un-terminated path should not load"),
        Outcome::Err(LinkError::PathNotFound) => {}
        Outcome::Err(LinkError::PathEncodingUnsupported) => {}
        Outcome::Err(_) => panic!("wrong error variant"),
    }
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[test]
fn reject_missing_symbol() {
    let ext = match Library::load(LIBC_PATH) {
        Outcome::Ok(ext) => ext,
        Outcome::Err(_) => panic!("system libc should load"),
    };
    type Nothing = extern "C" fn() -> i32;
    match ext.resolve::<Nothing>(b"absolutely_does_not_exist_xyz\0") {
        Outcome::Ok(_) => panic!("nonexistent symbol should not resolve"),
        Outcome::Err(LinkError::SymbolMissing) => {}
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

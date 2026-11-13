//! Loading a library the process has not mapped yet.
//!
//! `libc` is mapped into every process before a test starts, so loading it
//! succeeds whatever flags `dlopen` is given. A flag that glibc reads as
//! `RTLD_NOLOAD` only shows up on a library nobody has loaded, which is why
//! this file loads one that the test binary does not link against.

#![cfg(all(target_os = "linux", target_env = "gnu"))]

use core::ffi::{c_char, c_int, c_void};

use hilavitkutin_linking::Library;
use notko::Outcome;

const RTLD_NOW: c_int = 2;
/// Apple's value for `RTLD_LOCAL`. On glibc the same bit is `RTLD_NOLOAD`.
const APPLE_RTLD_LOCAL: c_int = 4;

unsafe extern "C" {
    fn dlopen(path: *const c_char, flags: c_int) -> *mut c_void;
}

#[cfg(target_arch = "x86_64")]
const CANDIDATES: &[&[u8]] = &[
    b"/lib/x86_64-linux-gnu/libanl.so.1\0",
    b"/lib/x86_64-linux-gnu/libresolv.so.2\0",
    b"/lib/x86_64-linux-gnu/libBrokenLocale.so.1\0",
    b"/lib/x86_64-linux-gnu/libthread_db.so.1\0",
];

#[cfg(target_arch = "aarch64")]
const CANDIDATES: &[&[u8]] = &[
    b"/lib/aarch64-linux-gnu/libanl.so.1\0",
    b"/lib/aarch64-linux-gnu/libresolv.so.2\0",
    b"/lib/aarch64-linux-gnu/libBrokenLocale.so.1\0",
    b"/lib/aarch64-linux-gnu/libthread_db.so.1\0",
];

fn not_mapped(path: &[u8]) -> bool {
    let name = core::str::from_utf8(path).unwrap().trim_end_matches('\0');
    let file = name.rsplit('/').next().unwrap();
    let maps = std::fs::read_to_string("/proc/self/maps").unwrap();
    !maps.contains(file)
}

fn candidate() -> &'static [u8] {
    for path in CANDIDATES {
        let name = core::str::from_utf8(path).unwrap().trim_end_matches('\0');
        if std::path::Path::new(name).exists() && not_mapped(path) {
            return path;
        }
    }
    panic!("none of the candidate libraries is present and unmapped on this host");
}

#[test]
fn a_library_not_yet_resident_loads() {
    let path = candidate();

    // The control comes first and shows the premise: with 4 as the flag, as
    // it was before this was made per target, glibc refuses the library, so
    // it is not mapped and loading it proves something.
    let refused = unsafe { dlopen(path.as_ptr() as *const c_char, RTLD_NOW | APPLE_RTLD_LOCAL) };
    assert!(
        refused.is_null(),
        "with RTLD_NOLOAD a library not yet mapped must not open"
    );

    match Library::load(path) {
        Outcome::Ok(_lib) => {},
        Outcome::Err(_) => panic!("a library the process has not mapped should load"),
    }
}

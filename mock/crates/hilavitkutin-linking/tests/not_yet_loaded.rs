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

/// Small glibc libraries the test binary does not link against, so none of
/// them is mapped before the test asks for it.
const LIBRARIES: &[&str] = &[
    "libanl.so.1",
    "libresolv.so.2",
    "libBrokenLocale.so.1",
    "libthread_db.so.1",
    "libutil.so.1",
];

/// Where glibc keeps them across the common layouts: Debian multiarch, the
/// Fedora and Arch lib64, and a plain lib.
const DIRECTORIES: &[&str] = &[
    "/lib/x86_64-linux-gnu",
    "/usr/lib/x86_64-linux-gnu",
    "/lib/aarch64-linux-gnu",
    "/usr/lib/aarch64-linux-gnu",
    "/lib64",
    "/usr/lib64",
    "/lib",
    "/usr/lib",
];

fn not_mapped(file: &str) -> bool {
    let maps = std::fs::read_to_string("/proc/self/maps").unwrap();
    !maps.contains(file)
}

/// A NUL-terminated path to a library that exists on this host and is not
/// yet mapped into the process.
fn candidate() -> std::vec::Vec<u8> {
    for dir in DIRECTORIES {
        for file in LIBRARIES {
            let path = std::format!("{dir}/{file}");
            if std::path::Path::new(&path).is_file() && not_mapped(file) {
                let mut bytes = path.into_bytes();
                bytes.push(0);
                return bytes;
            }
        }
    }
    panic!("none of the candidate libraries is present and unmapped on this host");
}

#[test]
fn a_library_not_yet_resident_loads() {
    let path = candidate();
    let path = path.as_slice();

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

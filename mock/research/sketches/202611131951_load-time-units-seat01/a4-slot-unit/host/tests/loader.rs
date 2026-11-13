//! Found on the way to arm A4: hilavitkutin-linking cannot open a library
//! that is not already mapped into the process, on linux glibc.
//!
//! hilavitkutin-linking/src/backend/unix.rs:8-9 declares `RTLD_NOW = 2` and
//! `RTLD_LOCAL = 4`. On linux glibc RTLD_LOCAL is 0 (libc
//! src/unix/linux_like/mod.rs:1205) and 4 is RTLD_NOLOAD (libc
//! src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:288): "do not load, only
//! return a handle if the image is already resident". 4 is RTLD_LOCAL only on
//! the apple targets. The crate's own tests open libc.so.6
//! (hilavitkutin-linking/tests/basic.rs:17), which every process has mapped,
//! so they pass with the wrong flag.
//!
//! This binary holds only these tests, so the extension is not resident when
//! the first one runs. Run with `--test-threads=1`: the second test maps the
//! image and the order is by name.

use std::os::unix::ffi::OsStrExt;

use a4_host::ext_path;
use hilavitkutin_linking::Library;
use notko::Outcome;

fn path() -> Vec<u8> {
    let mut p = ext_path().as_os_str().as_bytes().to_vec();
    p.push(0);
    p
}

#[test]
fn a_library_not_yet_mapped_loads() {
    let p = path();
    assert!(ext_path().exists(), "run `cargo build -p a4-ext` first");
    let loaded = matches!(Library::load(&p), Outcome::Ok(_));
    assert!(
        loaded,
        "Library::load refused {:?}, a file that exists and that dlopen with the platform's own RTLD_LOCAL opens",
        ext_path()
    );
}

#[test]
fn b_control_the_same_image_opens_with_the_platform_flags_and_then_with_library_load() {
    let p = path();
    let h = unsafe {
        libc::dlopen(
            p.as_ptr() as *const libc::c_char,
            libc::RTLD_NOW | libc::RTLD_LOCAL,
        )
    };
    assert!(
        !h.is_null(),
        "control: dlopen with libc's constants opens the image"
    );
    assert!(
        matches!(Library::load(&p), Outcome::Ok(_)),
        "once resident, the NOLOAD flag finds it"
    );
}

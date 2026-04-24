//! Unix backend: raw dlopen / dlsym / dlclose over libc.

use crate::error::LinkError;
use arvo::USize;
use core::ffi::{c_char, c_int, c_void};
use notko::Outcome;

const RTLD_NOW: c_int = 2;

unsafe extern "C" {
    fn dlopen(path: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
}

pub(crate) fn platform_load(
    path: &[u8], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: null-terminated byte path for dlopen; byte string is the loader input unit; tracked: #206
) -> Outcome<*mut c_void, LinkError> {
    if !is_null_terminated(path) {
        return Outcome::Err(LinkError::PathNotFound);
    }
    // SAFETY: path is non-empty, null-terminated, and passed to a
    // libc function that treats it as a C string.
    let handle = unsafe { dlopen(path.as_ptr() as *const c_char, RTLD_NOW) };
    if handle.is_null() {
        return Outcome::Err(LinkError::LoadFailed {
            platform_code: read_errno(),
        });
    }
    Outcome::Ok(handle)
}

pub(crate) fn platform_resolve(
    handle: *mut c_void,
    name: &[u8], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: null-terminated byte symbol name for dlsym; byte string is the resolver input unit; tracked: #206
) -> Outcome<*const c_void, LinkError> {
    if !is_null_terminated(name) {
        return Outcome::Err(LinkError::SymbolMissing);
    }
    // SAFETY: handle was produced by a prior successful dlopen; name
    // is null-terminated.
    let ptr = unsafe { dlsym(handle, name.as_ptr() as *const c_char) };
    if ptr.is_null() {
        return Outcome::Err(LinkError::SymbolMissing);
    }
    Outcome::Ok(ptr as *const c_void)
}

pub(crate) fn platform_close(handle: *mut c_void) -> Outcome<(), LinkError> {
    // SAFETY: handle was produced by a prior successful dlopen.
    let rc = unsafe { dlclose(handle) };
    if rc == 0 {
        Outcome::Ok(())
    } else {
        Outcome::Err(LinkError::LoadFailed {
            platform_code: read_errno(),
        })
    }
}

fn is_null_terminated(bytes: &[u8]) -> bool { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: helper over FFI byte string input; tracked: #206
    !bytes.is_empty() && bytes[bytes.len() - 1] == 0
}

fn read_errno() -> USize {
    // libc exposes errno via thread-local; extracting it portably
    // without std means calling through __errno_location / __error
    // per-platform. For v1 we return a sentinel 0. The variant
    // conveys the error category even when the numeric code is not
    // captured. Follow-up round refines this if callers need the
    // exact errno value.
    USize(0)
}

//! Unix backend: raw dlopen / dlsym / dlclose over libc.

use crate::error::LinkError;
use arvo::USize;
use core::ffi::{c_char, c_int, c_void};
use notko::Outcome;

const RTLD_NOW: c_int = 2;
const RTLD_LOCAL: c_int = 4;

unsafe extern "C" {
    fn dlopen(path: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
}

pub(crate) fn platform_load(
    path: &[u8],
) -> Outcome<*mut c_void, LinkError> {
    if !is_null_terminated(path) {
        return Outcome::Err(LinkError::PathNotFound);
    }
    // SAFETY: path is non-empty, null-terminated, and passed to a
    // libc function that treats it as a C string.
    let handle =
        unsafe { dlopen(path.as_ptr() as *const c_char, RTLD_NOW | RTLD_LOCAL) };
    if handle.is_null() {
        return Outcome::Err(LinkError::LoadFailed {
            platform_code: read_errno(),
        });
    }
    Outcome::Ok(handle)
}

pub(crate) fn platform_resolve(
    handle: *mut c_void,
    name: &[u8],
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

fn is_null_terminated(bytes: &[u8]) -> bool {
    !bytes.is_empty() && bytes[bytes.len() - 1] == 0
}

fn read_errno() -> USize {
    // Per-libc symbol that returns the address of the thread-local
    // `errno` integer. Linux/Android use `__errno_location`; Darwin
    // and the BSDs use `__error`. Solaris/Haiku/etc. fall through to
    // the zero sentinel.
    #[cfg(any(target_os = "linux", target_os = "android"))]
    unsafe extern "C" {
        fn __errno_location() -> *mut c_int;
    }
    #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "watchos",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
    ))]
    unsafe extern "C" {
        fn __error() -> *mut c_int;
    }

    // SAFETY: each per-platform symbol returns a thread-local pointer
    // that is always valid for the lifetime of the calling thread per
    // libc contract. Reading a single `c_int` at that address is the
    // documented way to observe errno.
    #[cfg(any(target_os = "linux", target_os = "android"))]
    let val: c_int = unsafe { *__errno_location() };
    #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "watchos",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
    ))]
    let val: c_int = unsafe { *__error() };
    #[cfg(not(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "watchos",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
    )))]
    let val: c_int = 0;

    // errno values are non-negative when set; the cast is lossless on
    // every supported platform.
    USize(val as usize)
}

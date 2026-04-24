//! Windows backend: LoadLibraryW / GetProcAddress / FreeLibrary.

use crate::error::LinkError;
use arvo::USize;
use core::ffi::c_void;
use notko::Outcome;
use windows_sys::Win32::Foundation::{GetLastError, HMODULE};
use windows_sys::Win32::System::LibraryLoader::{
    FreeLibrary, GetProcAddress, LoadLibraryW,
};

// v1 supports ASCII paths only on Windows. Extended Unicode (full
// UTF-16 path support) is a BACKLOG item.
const MAX_PATH_WIDE: usize = 260; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: Windows MAX_PATH constant; platform-defined width; tracked: #206

pub(crate) fn platform_load(
    path: &[u8], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: null-terminated byte path for LoadLibraryW (converted to wide internally); byte string is the loader input unit; tracked: #206
) -> Outcome<*mut c_void, LinkError> {
    let Some(wide) = ascii_to_wide(path) else {
        return Outcome::Err(LinkError::PathNotFound);
    };
    // SAFETY: wide is null-terminated UTF-16.
    let module: HMODULE = unsafe { LoadLibraryW(wide.as_ptr()) };
    if module.is_null() {
        return Outcome::Err(LinkError::LoadFailed {
            platform_code: read_last_error(),
        });
    }
    Outcome::Ok(module as *mut c_void)
}

pub(crate) fn platform_resolve(
    handle: *mut c_void,
    name: &[u8], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: null-terminated byte symbol name for GetProcAddress; tracked: #206
) -> Outcome<*const c_void, LinkError> {
    if !is_null_terminated(name) {
        return Outcome::Err(LinkError::SymbolMissing);
    }
    // SAFETY: handle came from LoadLibraryW; name is null-terminated.
    let proc = unsafe {
        GetProcAddress(handle as HMODULE, name.as_ptr() as *const u8) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: Win32 GetProcAddress signature requires *const u8; tracked: #206
    };
    match proc {
        Some(p) => Outcome::Ok(p as *const c_void),
        None => Outcome::Err(LinkError::SymbolMissing),
    }
}

pub(crate) fn platform_close(handle: *mut c_void) -> Outcome<(), LinkError> {
    // SAFETY: handle came from LoadLibraryW.
    let rc = unsafe { FreeLibrary(handle as HMODULE) };
    if rc != 0 {
        Outcome::Ok(())
    } else {
        Outcome::Err(LinkError::LoadFailed {
            platform_code: read_last_error(),
        })
    }
}

fn ascii_to_wide(bytes: &[u8]) -> Option<[u16; MAX_PATH_WIDE]> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-bare-option) reason: Win32 wide-char path conversion; UTF-16 u16 is platform-fixed; Option-returning conversion helper; tracked: #206
    if !is_null_terminated(bytes) || bytes.len() > MAX_PATH_WIDE {
        return None;
    }
    let mut wide = [0u16; MAX_PATH_WIDE];
    for (i, &b) in bytes.iter().enumerate() {
        if b > 0x7F {
            return None; // non-ASCII; v1 rejects
        }
        wide[i] = b as u16; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: UTF-16 code unit; platform-fixed width; tracked: #206
    }
    Some(wide)
}

fn is_null_terminated(bytes: &[u8]) -> bool { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: helper over FFI byte string input; tracked: #206
    !bytes.is_empty() && bytes[bytes.len() - 1] == 0
}

fn read_last_error() -> USize {
    // SAFETY: GetLastError is always safe to call.
    let code = unsafe { GetLastError() };
    USize(code as usize) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: GetLastError DWORD widened to host-width usize for USize(pub usize); tracked: #206
}

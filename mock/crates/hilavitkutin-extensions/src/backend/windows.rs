//! Windows backend: LoadLibraryW / GetProcAddress / FreeLibrary.

use crate::error::ExtensionError;
use arvo::USize;
use core::ffi::c_void;
use notko::Outcome;
use windows_sys::Win32::Foundation::{GetLastError, HMODULE};
use windows_sys::Win32::System::LibraryLoader::{
    FreeLibrary, GetProcAddress, LoadLibraryW,
};

// v1 supports ASCII paths only on Windows. Extended Unicode (full
// UTF-16 path support) is a BACKLOG item.
const MAX_PATH_WIDE: usize = 260;

pub(crate) fn platform_load(
    path: &[u8],
) -> Outcome<*mut c_void, ExtensionError> {
    let Some(wide) = ascii_to_wide(path) else {
        return Outcome::Err(ExtensionError::PathNotFound);
    };
    // SAFETY: wide is null-terminated UTF-16.
    let module: HMODULE = unsafe { LoadLibraryW(wide.as_ptr()) };
    if module.is_null() {
        return Outcome::Err(ExtensionError::LoadFailed {
            platform_code: read_last_error(),
        });
    }
    Outcome::Ok(module as *mut c_void)
}

pub(crate) fn platform_resolve(
    handle: *mut c_void,
    name: &[u8],
) -> Outcome<*const c_void, ExtensionError> {
    if !is_null_terminated(name) {
        return Outcome::Err(ExtensionError::SymbolMissing);
    }
    // SAFETY: handle came from LoadLibraryW; name is null-terminated.
    let proc = unsafe {
        GetProcAddress(handle as HMODULE, name.as_ptr() as *const u8)
    };
    match proc {
        Some(p) => Outcome::Ok(p as *const c_void),
        None => Outcome::Err(ExtensionError::SymbolMissing),
    }
}

pub(crate) fn platform_close(handle: *mut c_void) -> Outcome<(), ExtensionError> {
    // SAFETY: handle came from LoadLibraryW.
    let rc = unsafe { FreeLibrary(handle as HMODULE) };
    if rc != 0 {
        Outcome::Ok(())
    } else {
        Outcome::Err(ExtensionError::LoadFailed {
            platform_code: read_last_error(),
        })
    }
}

fn ascii_to_wide(bytes: &[u8]) -> Option<[u16; MAX_PATH_WIDE]> {
    if !is_null_terminated(bytes) || bytes.len() > MAX_PATH_WIDE {
        return None;
    }
    let mut wide = [0u16; MAX_PATH_WIDE];
    for (i, &b) in bytes.iter().enumerate() {
        if b > 0x7F {
            return None; // non-ASCII; v1 rejects
        }
        wide[i] = b as u16;
    }
    Some(wide)
}

fn is_null_terminated(bytes: &[u8]) -> bool {
    !bytes.is_empty() && bytes[bytes.len() - 1] == 0
}

fn read_last_error() -> USize {
    // SAFETY: GetLastError is always safe to call.
    let code = unsafe { GetLastError() };
    USize(code as usize)
}

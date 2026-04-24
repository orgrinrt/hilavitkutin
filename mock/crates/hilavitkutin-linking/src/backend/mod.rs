//! Platform dispatch. Each backend exposes the same pub(crate)
//! interface so `lib.rs` can call a single set of function names
//! regardless of target.
//!
//! Backend contract (both platforms implement the same shape):
//!
//! ```ignore
//! fn platform_load(path: &[u8]) -> Outcome<PlatformHandle, LinkError>;
//! fn platform_resolve(handle: PlatformHandle, name: &[u8]) -> Outcome<*const c_void, LinkError>;
//! fn platform_close(handle: PlatformHandle) -> Outcome<(), LinkError>;
//! ```
//!
//! Both functions enforce null-termination expectations by rejecting
//! paths / names that lack a trailing 0 byte.

use core::ffi::c_void;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub(crate) use unix::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub(crate) use windows::*;

/// Opaque platform loader handle.
pub(crate) type PlatformHandle = *mut c_void;

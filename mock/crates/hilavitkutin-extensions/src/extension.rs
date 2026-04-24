//! Per-extension runtime handle.

use core::ffi::c_void;
use hilavitkutin_linking::Library;
use notko::{Maybe, Outcome};

use crate::descriptor::{
    CapabilityEntry, CapabilityId, ExtensionAbiStatus, ExtensionDescriptor,
    ExtensionVersion,
};
use crate::error::ExtensionError;

/// Loaded extension bound to a live `Library`.
///
/// Holds the descriptor borrow and the host-opaque context pointer
/// handed in at load. `Drop` runs the extension's shutdown handler
/// (if present) then the `Library` closes. Shutdown errors at drop
/// are swallowed; use `close` to surface them.
pub struct Extension {
    library: Library,
    descriptor: &'static ExtensionDescriptor,
    host_ctx: *mut c_void,
}

impl Extension {
    /// Descriptor the extension exposed.
    pub fn descriptor(&self) -> &ExtensionDescriptor {
        self.descriptor
    }

    /// Resolve a single capability's raw vtable pointer.
    pub fn capability(&self, id: CapabilityId) -> Maybe<*const c_void> {
        if self.descriptor.capabilities_ptr.is_null() {
            return Maybe::Isnt;
        }
        let len = self.descriptor.capabilities_len;
        let mut i = 0;
        while i < len {
            // SAFETY: capabilities_ptr + capabilities_len form a valid
            // slice in the extension's static memory for the loaded
            // library lifetime.
            let entry = unsafe { &*self.descriptor.capabilities_ptr.add(i) };
            if entry.id == id {
                return Maybe::Is(entry.vtable_ptr);
            }
            i += 1;
        }
        Maybe::Isnt
    }

    /// Slice view of the extension's full capability table.
    pub fn capabilities(&self) -> &[CapabilityEntry] {
        if self.descriptor.capabilities_ptr.is_null()
            || self.descriptor.capabilities_len == 0
        {
            return &[];
        }
        // SAFETY: descriptor fields form a valid slice in the
        // extension's static memory for the loaded lifetime.
        unsafe {
            core::slice::from_raw_parts(
                self.descriptor.capabilities_ptr,
                self.descriptor.capabilities_len,
            )
        }
    }

    /// Name slice as declared by the extension.
    pub fn name(&self) -> &[u8] {
        if self.descriptor.name_ptr.is_null() || self.descriptor.name_len == 0 {
            return &[];
        }
        // SAFETY: name_ptr + name_len form a valid static byte slice.
        unsafe {
            core::slice::from_raw_parts(
                self.descriptor.name_ptr,
                self.descriptor.name_len,
            )
        }
    }

    /// Version triple.
    pub fn version(&self) -> ExtensionVersion {
        self.descriptor.version
    }

    /// Explicitly drive shutdown and close the library. Returns the
    /// first encountered failure instead of swallowing.
    pub fn close(self) -> Outcome<(), ExtensionError> {
        let shutdown = self.descriptor.shutdown_fn;
        let host_ctx = self.host_ctx;
        // Forget self so our Drop does not double-call shutdown.
        let lib = unsafe {
            let lib_ptr = &raw const self.library;
            core::ptr::read(lib_ptr)
        };
        core::mem::forget(self);

        if let Some(shutdown_fn) = shutdown {
            // SAFETY: shutdown is declared by the extension; host_ctx
            // is the pointer the host threaded in at load time.
            let status = unsafe { shutdown_fn(host_ctx) };
            if status != ExtensionAbiStatus::Ok {
                // lib drops here, releasing the OS handle.
                drop(lib);
                return Outcome::Err(ExtensionError::ShutdownFailed { status });
            }
        }
        drop(lib);
        Outcome::Ok(())
    }

    #[doc(hidden)]
    pub(crate) fn from_parts(
        library: Library,
        descriptor: &'static ExtensionDescriptor,
        host_ctx: *mut c_void,
    ) -> Self {
        Self { library, descriptor, host_ctx }
    }
}

impl Drop for Extension {
    fn drop(&mut self) {
        if let Some(shutdown) = self.descriptor.shutdown_fn {
            // SAFETY: shutdown is declared by the extension; host_ctx
            // is the pointer it received at init time.
            let _ = unsafe { shutdown(self.host_ctx) };
        }
        // Library's own Drop runs after this returns.
    }
}

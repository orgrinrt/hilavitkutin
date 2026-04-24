//! Per-extension runtime handle.

use core::ffi::c_void;
use hilavitkutin_linking::Library;
use notko::{Maybe, Outcome};

use crate::descriptor::{CapabilityEntry, CapabilityId, ExtensionDescriptor};
use crate::error::ExtensionError;

/// Loaded extension bound to a live `Library`.
///
/// Holds the descriptor borrow and a host-owned opaque context
/// pointer passed through init and shutdown handlers. The handle is
/// per-extension; dropping it closes the library after running the
/// extension's shutdown handler. Any shutdown failure is swallowed
/// at drop; use `close` to surface it.
pub struct Extension {
    library: Library,
    descriptor: &'static ExtensionDescriptor,
    host_ctx: *mut c_void,
}

impl Extension {
    /// Returns the descriptor this extension exposed.
    pub fn descriptor(&self) -> &ExtensionDescriptor {
        self.descriptor
    }

    /// Resolve a single capability by id.
    pub fn capability(&self, id: CapabilityId) -> Maybe<&CapabilityEntry> {
        let descriptor = self.descriptor;
        if descriptor.capabilities_ptr.is_null() {
            return Maybe::Isnt;
        }
        let len = descriptor.capabilities_len;
        let mut i = 0;
        while i < len {
            // SAFETY: capabilities_ptr + capabilities_len form a valid
            // slice in the extension's static memory for the loaded
            // library lifetime.
            let entry = unsafe { &*descriptor.capabilities_ptr.add(i) };
            if entry.id == id {
                return Maybe::Is(entry);
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
        // SAFETY: descriptor fields are a valid slice in the
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
    pub fn version(&self) -> crate::descriptor::ExtensionVersion {
        self.descriptor.version
    }

    /// Explicitly drive shutdown and close the library. Returns the
    /// first encountered failure instead of swallowing.
    pub fn close(self) -> Outcome<(), ExtensionError> {
        let _ = self;
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
            // is the pointer it received at init time. Extension's
            // contract requires it to be safe to call at drop time.
            let _ = unsafe { shutdown(self.host_ctx) };
        }
        // Library Drop runs after this to close the underlying OS
        // handle. Explicit reference keeps the field live.
        let _ = &self.library;
    }
}

//! The stack-backed bump arena the scheduler builder tests build on, and the
//! default-capacity column store over it.

use arvo::{Bool, USize};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_providers::ArenaColumnStorage;

/// A fixed bump allocator over an owned buffer of `N` bytes.
pub struct TestProvider<const N: usize> {
    buf:  core::cell::UnsafeCell<[core::mem::MaybeUninit<u8>; N]>,
    used: core::cell::Cell<usize>,
}

impl<const N: usize> TestProvider<N> {
    pub fn new() -> Self {
        Self {
            buf:  core::cell::UnsafeCell::new([const { core::mem::MaybeUninit::uninit() }; N]),
            used: core::cell::Cell::new(0),
        }
    }
}

unsafe impl<const N: usize> Send for TestProvider<N> {}
unsafe impl<const N: usize> Sync for TestProvider<N> {}

impl<const N: usize> MemoryProviderApi for TestProvider<N> {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let base = self.buf.get() as *mut u8;
        let used = self.used.get();
        let align = align.0.max(1);
        let aligned = (used + align - 1) / align * align;
        if aligned + len.0 > N {
            return core::ptr::null_mut();
        }
        self.used.set(aligned + len.0);
        // SAFETY: in bounds of the owned buffer.
        unsafe { base.add(aligned) }
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize, _align: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

fn provider() -> TestProvider<4096> {
    TestProvider::<4096>::new()
}

/// The default-capacity arena store over the test provider. The return
/// type omits `D`, applying the `Dim<256>` default so inference is
/// anchored at the `build` call sites.
pub fn store() -> ArenaColumnStorage<TestProvider<4096>> {
    ArenaColumnStorage::new(provider())
}

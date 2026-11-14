//! The arena, the input column and the hint every `run_parallel` fixture shares.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Column;
use hilavitkutin_providers::ArenaColumnStorage;

pub fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

/// A bump allocator over an owned buffer of `N` bytes. It never frees: one
/// scheduler builds on it once and the test drops both together.
pub struct BumpProvider<const N: usize> {
    buf:  UnsafeCell<[MaybeUninit<u8>; N]>,
    used: Cell<usize>,
}

impl<const N: usize> BumpProvider<N> {
    pub fn new() -> Self {
        Self {
            buf:  UnsafeCell::new([const { MaybeUninit::uninit() }; N]),
            used: Cell::new(0),
        }
    }
}

unsafe impl<const N: usize> Send for BumpProvider<N> {}
unsafe impl<const N: usize> Sync for BumpProvider<N> {}

impl<const N: usize> MemoryProviderApi for BumpProvider<N> {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let base = self.buf.get() as *mut u8;
        let used = self.used.get();
        let align = align.0.max(1);
        let aligned = (used + align - 1) / align * align;
        if aligned + len.0 > N {
            return core::ptr::null_mut();
        }
        self.used.set(aligned + len.0);
        // SAFETY: `aligned + len <= N`, in bounds of the owned buffer.
        unsafe { base.add(aligned) }
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize, _align: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

/// The host-populated input every producer reads. A producer reads it rather
/// than synthesising a value from the `each()` index: that index is
/// morsel-relative, so a unit sourcing from it breaks once a phase is split
/// across cores.
#[derive(Copy, Clone, Default)]
pub struct Inv(pub u32);

pub type OneIn = Cons<Column<Inv>, Empty>;

pub type HintT = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);

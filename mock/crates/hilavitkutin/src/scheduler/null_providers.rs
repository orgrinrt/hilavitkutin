//! Null/default providers: the type-level defaults a bare `Scheduler` names.
//!
//! Split out of `scheduler/mod.rs` (file-size lint). No behaviour change.

use arvo::USize;
use hilavitkutin_api::platform::{ClockApi, MemoryProviderApi, Nanos};
use hilavitkutin_api::{ColumnStorage, ColumnValue, StoreId};

/// Null memory provider: the default `M` for a bare `Scheduler` type.
///
/// Every allocation returns null. It exists so the `Scheduler` type
/// has a default `M` parameter for type-level uses (alias defaults,
/// turbofish-free naming). A scheduler that actually owns resources is
/// always built with a real provider via `build(memory_provider)`.
pub struct NullMemoryProvider;

// SAFETY: zero-sized, holds no state; trivially Send + Sync.
unsafe impl Send for NullMemoryProvider {}
unsafe impl Sync for NullMemoryProvider {}

impl Default for NullMemoryProvider {
    fn default() -> Self {
        NullMemoryProvider
    }
}

impl MemoryProviderApi for NullMemoryProvider {
    #[rustfmt::skip] // keeps the allow on the signature it governs
    unsafe fn allocate(&self, _len: arvo::USize, _align: arvo::USize) -> *mut u8 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI returns raw pointer by contract; tracked: #72
        core::ptr::null_mut()
    }

    unsafe fn deallocate(
        &self,
        _ptr: *mut u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw pointer by contract; tracked: #72
        _len: arvo::USize,
    ) {
    }

    unsafe fn protect(
        &self,
        _ptr: *mut u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw pointer by contract; tracked: #72
        _len: arvo::USize,
        _read: arvo::Bool,
        _write: arvo::Bool,
    ) {
    }
}

/// Null clock: `now_ns` always returns zero.
///
/// The unconditional fallback `Clk` for builds without the os tier: the
/// pass-duration EMA stays zero until a real clock is supplied via the
/// builder's `clock(...)` slot (the no_os DI path). With the default
/// `platform-os` feature the builder starts on `OsClock` instead, so this
/// type is only ever the live clock when a no_os consumer leaves the slot
/// untouched.
pub struct NullClock;

impl NullClock {
    /// Construct the null clock.
    #[inline]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for NullClock {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl ClockApi for NullClock {
    #[inline]
    fn now_ns(&self) -> Nanos {
        Nanos::from_raw(0) // lint:allow(no-bare-numeric) reason: null clock zero reading; tracked: #121
    }
}

/// The builder's starting clock: the os-tier monotonic clock when the
/// default `platform-os` feature is on, the null clock otherwise (no_os
/// consumers supply their own via the builder's `clock(...)` slot).
#[cfg(feature = "platform-os")]
pub type DefaultClock = crate::platform::OsClock;
/// The builder's starting clock (no_os fallback; see the platform-os arm).
#[cfg(not(feature = "platform-os"))]
pub type DefaultClock = NullClock;

/// Null column storage: the default `CS` for a bare `Scheduler` type.
///
/// Reserves nothing and hands back null pointers. It exists so the
/// `Scheduler` type has a default `CS` parameter for type-level uses
/// (alias defaults, turbofish-free naming). A scheduler that actually
/// owns resources is always built with a real store via
/// `build(storage)`. Reserving on it fails, which is correct: a no-store
/// scheduler registers no resources, so the drain never reserves.
pub struct NullColumnStorage;

impl Default for NullColumnStorage {
    fn default() -> Self {
        NullColumnStorage
    }
}

impl ColumnStorage for NullColumnStorage {
    type Error = ();

    fn reserve<T: ColumnValue>(&mut self, _id: StoreId, _len: USize) -> notko::Outcome<(), ()> {
        notko::Outcome::Err(())
    }

    unsafe fn column_ptr<T: ColumnValue>(&self, _id: StoreId) -> *const T {
        core::ptr::null()
    }

    unsafe fn column_ptr_mut<T: ColumnValue>(&self, _id: StoreId) -> *mut T {
        core::ptr::null_mut()
    }

    fn count(&self, _id: StoreId) -> USize {
        <USize as arvo::strategy::Identity<arvo::strategy::Additive>>::IDENTITY
    }

    fn release(&mut self, _id: StoreId) {}
}

//! Phase-join sync points + S3 fence emission (domain 17).
//!
//! A SyncPoint is the gate phase N+1 reads to decide whether
//! phase N has produced enough records to start the next morsel.
//!
//! `emit_progress_release_fence` emits the architectural store-store
//! fence that Topic 3 S3 (corrected) requires immediately before
//! every progress-counter Release store. aarch64: `dmb ishst`.
//! x86_64: `_mm_sfence`. Other targets: a `compiler_fence(Release)`
//! barrier (no asm) on the assumption that ordering needs are met
//! by the AtomicUsize Release semantics alone; revisit per target as
//! the platform tier set widens.

use arvo::USize;

use crate::plan::FiberId;

/// Phase-join gate. `fiber_id` is the producing fiber, `min_records`
/// is the record count the consumer waits for before running.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SyncPoint {
    pub fiber_id: FiberId,
    pub min_records: USize,
}

impl SyncPoint {
    /// Construct a new sync point with the given producer and
    /// minimum-record threshold.
    pub const fn new(fiber_id: FiberId, min_records: USize) -> Self {
        Self {
            fiber_id,
            min_records,
        }
    }
}

/// Emit the architectural store-store fence that Topic 3 S3 requires
/// immediately before a progress-counter Release store. Pairs with
/// `super::progress::store_progress_arena` at every fiber-tail path.
///
/// On aarch64 lowers to `dmb ishst` (inner-shareable store-store
/// barrier). On x86_64 lowers to `_mm_sfence`. On unsupported
/// targets emits `compiler_fence(Release)` as a soft fallback;
/// architectural ordering semantics there will need a target-aware
/// audit before that arm is exercised.
#[inline(always)]
pub fn emit_progress_release_fence() {
    #[cfg(target_arch = "aarch64")]
    {
        // SAFETY: `dmb ishst` is a store-store barrier; it has no
        // memory operands and cannot violate any safety invariant.
        unsafe {
            core::arch::asm!("dmb ishst", options(nostack, preserves_flags));
        }
    }
    #[cfg(target_arch = "x86_64")]
    {
        // SAFETY: `_mm_sfence` is an architectural store-store
        // barrier with no memory operands.
        unsafe {
            core::arch::x86_64::_mm_sfence();
        }
    }
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Release);
    }
}

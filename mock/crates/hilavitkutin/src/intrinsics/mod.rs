//! Low-level engine intrinsics shim (domain 13).
//!
//! Portable, no-alloc, no-std. Ships as zero-cost stubs on stable
//! (`prefetch_l1`/`prefetch_l2` are no-ops; `noinline_barrier` is
//! identity) with optional upgrades under the `intrinsics-std`
//! feature (still core-only — pulls in `core::hint::black_box`).
//!
//! Real `core::intrinsics::prefetch_*` wiring and platform-specific
//! inline asm paths (x86_64 PREFETCHT0, aarch64 PRFM) land as
//! follow-ups — see BACKLOG → Engine 5a3 follow-ups.

use core::sync::atomic::{Ordering, compiler_fence};

/// L1 prefetch hint.
///
/// Skeleton: no-op on stable. Swapped for
/// `core::intrinsics::prefetch_read_data(_, 3)` once nightly
/// intrinsics land (BACKLOG).
pub fn prefetch_l1<T>(ptr: *const T) {
    let _ = ptr;
}

/// L2 prefetch hint.
///
/// Skeleton: no-op on stable. Swapped for
/// `core::intrinsics::prefetch_read_data(_, 1)` once nightly
/// intrinsics land (BACKLOG).
pub fn prefetch_l2<T>(ptr: *const T) {
    let _ = ptr;
}

/// # WARNING — compiler fence only, not a hardware barrier
///
/// This prevents the *compiler* from reordering memory ops past
/// this fence, but emits NO CPU-level memory barrier. On
/// architectures with weak memory models (ARM, RISC-V) this is
/// INSUFFICIENT for cross-thread ordering. Use atomic
/// Release/Acquire loads/stores for thread-to-thread handoff.
///
/// Release-ordered compiler fence. Prevents the compiler from
/// reordering prior stores past the fence.
pub fn compiler_fence_release() {
    compiler_fence(Ordering::Release);
}

/// # WARNING — compiler fence only, not a hardware barrier
///
/// This prevents the *compiler* from reordering memory ops past
/// this fence, but emits NO CPU-level memory barrier. On
/// architectures with weak memory models (ARM, RISC-V) this is
/// INSUFFICIENT for cross-thread ordering. Use atomic
/// Release/Acquire loads/stores for thread-to-thread handoff.
///
/// Acquire-ordered compiler fence. Prevents the compiler from
/// reordering later loads before the fence.
pub fn compiler_fence_acquire() {
    compiler_fence(Ordering::Acquire);
}

/// Optimiser barrier. Under `intrinsics-std`, wraps
/// `core::hint::black_box`; otherwise identity.
#[cfg(feature = "intrinsics-std")]
pub fn noinline_barrier<T>(val: T) -> T {
    core::hint::black_box(val)
}

/// Optimiser barrier (identity fallback when `intrinsics-std` is
/// disabled).
#[cfg(not(feature = "intrinsics-std"))]
pub fn noinline_barrier<T>(val: T) -> T {
    val
}

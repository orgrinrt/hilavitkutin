//! Three-tier parking with predictive-parking plumbing (Topic 6
//! axes B + J).
//!
//! Tier selection driven by `predicted_wait_ns` (Topic 5 axis G)
//! against the `WakeStrategy` thresholds:
//!
//! 1. `< futex_threshold_ns`: spin-only via `core::hint::spin_loop`.
//!    The default spin budget (`{p,e}_spin_iters`) bounds the
//!    busy-wait before yielding control even at this tier.
//! 2. `< park_threshold_ns`: spin-then-atomic-wait. Per-OS futex /
//!    ulock / WaitOnAddress wrapper takes the worker off-CPU for
//!    short waits. Wakeups are explicit (`wake_all` after a
//!    barrier reset).
//! 3. `>= park_threshold_ns`: park immediately via the same
//!    atomic-wait primitive; the spin step is skipped.
//!
//! AdaptWu (Pass 5) writes per-phase `predicted_wait_ns` at
//! `ScheduleEnd`; workers Relaxed-load it on phase entry.
//!
//! Predictive-parking plumbing surface:
//!
//! - `predicted_wait_ns_load(pool, phase)` Relaxed-load the
//!   per-phase atomic slot.
//! - `predicted_wait_ns_store(pool, phase, value)` Relaxed-store
//!   by AdaptWu at ScheduleEnd.
//!
//! The atomic-wait wrappers (`atomic_wait` / `atomic_wake_all`) are
//! cfg-gated per OS. On `platform-no-os` or unsupported targets
//! the wait wrapper degrades to `core::hint::spin_loop` and the
//! wake is a no-op.

use core::sync::atomic::{AtomicU32, Ordering};

use arvo::USize;
use arvo::strategy::Identity;
use hilavitkutin_api::platform::{PoolFrame, WakeStrategy};

use super::class::CoreClass;

/// Park tier picked for the current phase.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ParkTier {
    /// Spin-only via `core::hint::spin_loop`.
    Spin,
    /// Spin a short budget, then atomic-wait.
    SpinThenWait,
    /// Atomic-wait immediately; skip the spin budget.
    WaitImmediate,
}

/// Pick the parking tier for `predicted_wait_ns` against the
/// `WakeStrategy` thresholds. Inline-able + branch-free at the
/// hot-path call site (workers call this once per phase entry).
#[inline]
pub fn pick_tier(predicted_wait_ns: USize, strat: &WakeStrategy) -> ParkTier {
    if predicted_wait_ns.0 < strat.futex_threshold_ns.0 {
        ParkTier::Spin
    } else if predicted_wait_ns.0 < strat.park_threshold_ns.0 {
        ParkTier::SpinThenWait
    } else {
        ParkTier::WaitImmediate
    }
}

/// Per-class spin budget. P-cores get the wider budget per the
/// WakeStrategy's `p_spin_iters`; E-cores get the narrower
/// `e_spin_iters`.
#[inline]
pub fn spin_budget_for(class: CoreClass, strat: &WakeStrategy) -> USize {
    match class {
        CoreClass::P => strat.p_spin_iters,
        CoreClass::E => strat.e_spin_iters,
    }
}

/// Spin `n` iterations. `core::hint::spin_loop` emits the host's
/// PAUSE / YIELD hint where available (x86 `PAUSE`, aarch64 `YIELD`).
#[inline]
pub fn spin(n: USize) {
    let mut i: usize = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: tight loop counter; tracked: #121
    while i < n.0 {
        core::hint::spin_loop();
        i += 1; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: tight loop increment; tracked: #121
    }
}

/// Relaxed-load the per-phase predicted wait time from the
/// `PoolFrame.predicted_wait_ns` atomic slot. Workers read at phase
/// entry to pick a tier; the staleness window between AdaptWu's
/// Release at ScheduleEnd and the worker's Relaxed-load is
/// acceptable (the adapt loop ratchets one phase at a time).
#[inline]
pub fn predicted_wait_ns_load<'arena, const C: usize, const P: usize>(
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    pool: &PoolFrame<'arena, C, P>,
    phase: USize,
) -> USize {
    if phase.0 >= P {
        return USize::ZERO;
    }
    let v = pool.predicted_wait_ns[phase.0].load(Ordering::Relaxed);
    USize(v as usize) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic u32 widened to USize; tracked: #121
}

/// Relaxed-store the per-phase predicted wait time. Called by
/// AdaptWu at ScheduleEnd. Relaxed pairs with the worker's
/// Relaxed-load: prediction freshness is a soft property; the
/// loop self-corrects via the next phase's measurement.
#[inline]
pub fn predicted_wait_ns_store<'arena, const C: usize, const P: usize>(
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    pool: &PoolFrame<'arena, C, P>,
    phase: USize,
    value: USize,
) {
    if phase.0 >= P {
        return;
    }
    let v = value.0 as u32; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: nanos narrows to u32 for atomic; tracked: #121
    pool.predicted_wait_ns[phase.0].store(v, Ordering::Relaxed);
}

// ---------------------------------------------------------------------
// Atomic-wait wrappers. Per-OS thin shims over the platform's
// futex-style primitive. Each wrapper takes a borrowed AtomicU32, an
// `expected` value, and waits until the atomic's value diverges
// from `expected` (or wakeup fires).
// ---------------------------------------------------------------------

/// Wait on `addr` until its value diverges from `expected`. The
/// platform wrappers are best-effort: spurious wakeups are allowed
/// and callers re-check the condition. On unsupported platforms
/// the wrapper degrades to a single `spin_loop` step so callers
/// re-check immediately.
#[inline]
pub fn atomic_wait(addr: &AtomicU32, expected: u32) {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI ABI takes u32 by contract; tracked: #72
    platform::wait_u32(addr, expected);
}

/// Wake every worker parked on `addr`.
#[inline]
pub fn atomic_wake_all(addr: &AtomicU32) {
    platform::wake_all_u32(addr);
}

#[cfg(all(target_os = "linux", feature = "platform-os"))]
mod platform {
    use core::sync::atomic::AtomicU32;

    // FUTEX op constants. Private syscalls; values stable per Linux
    // ABI.
    const FUTEX_WAIT_PRIVATE: i32 = 0 | 128; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: kernel ABI constant; tracked: #72
    const FUTEX_WAKE_PRIVATE: i32 = 1 | 128; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: kernel ABI constant; tracked: #72
    const SYS_FUTEX: libc::c_long = 202; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: kernel ABI constant; tracked: #72
    const WAKE_ALL: i32 = i32::MAX; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: kernel ABI sentinel "wake every waiter"; tracked: #72

    pub fn wait_u32(addr: &AtomicU32, expected: u32) {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI signature; tracked: #72
        let ptr = addr as *const AtomicU32 as *const u32; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI cast; tracked: #72
        unsafe {
            libc::syscall(
                SYS_FUTEX,
                ptr,
                FUTEX_WAIT_PRIVATE,
                expected,
                core::ptr::null::<libc::timespec>(),
            );
        }
    }

    pub fn wake_all_u32(addr: &AtomicU32) {
        let ptr = addr as *const AtomicU32 as *const u32; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI cast; tracked: #72
        unsafe {
            libc::syscall(SYS_FUTEX, ptr, FUTEX_WAKE_PRIVATE, WAKE_ALL);
        }
    }
}

#[cfg(all(target_os = "macos", feature = "platform-os"))]
mod platform {
    use core::sync::atomic::AtomicU32;

    // `__ulock_wait2` / `__ulock_wake` ABI constants. ULOCK private
    // API; stable enough to be used by libdispatch + tokio's parking_lot
    // ports. UL_COMPARE_AND_WAIT64 = 1 for u32 too because the kernel
    // ABI compares the low 32 bits of the 64-bit value. ULF_NO_ERRNO
    // returns the negated errno via the syscall return rather than
    // setting global errno.
    const UL_COMPARE_AND_WAIT: u32 = 1; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: kernel ABI constant; tracked: #72
    const ULF_NO_ERRNO: u32 = 0x0100_0000; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: kernel ABI constant; tracked: #72
    const ULF_WAKE_ALL: u32 = 0x0000_0100; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: kernel ABI constant; tracked: #72
    const TIMEOUT_NEVER: u64 = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: kernel ABI sentinel "no timeout"; tracked: #72
    const VALUE2_UNUSED: u64 = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: kernel ABI unused arg; tracked: #72
    const WAKE_VALUE_IGNORED: u64 = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: kernel ABI unused under ULF_WAKE_ALL; tracked: #72

    unsafe extern "C" {
        fn __ulock_wait2(
            operation: u32, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI ABI; tracked: #72
            addr: *const core::ffi::c_void,
            value: u64, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI ABI; tracked: #72
            timeout_ns: u64, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI ABI; tracked: #72
            value2: u64, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI ABI; tracked: #72
        ) -> core::ffi::c_int;
        fn __ulock_wake(
            operation: u32, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI ABI; tracked: #72
            addr: *const core::ffi::c_void,
            wake_value: u64, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI ABI; tracked: #72
        ) -> core::ffi::c_int;
    }

    pub fn wait_u32(addr: &AtomicU32, expected: u32) {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI signature; tracked: #72
        let ptr = addr as *const AtomicU32 as *const core::ffi::c_void;
        let val: u64 = expected as u64; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: widening for ABI; tracked: #72
        unsafe {
            __ulock_wait2(
                UL_COMPARE_AND_WAIT | ULF_NO_ERRNO,
                ptr,
                val,
                TIMEOUT_NEVER,
                VALUE2_UNUSED,
            );
        }
    }

    pub fn wake_all_u32(addr: &AtomicU32) {
        let ptr = addr as *const AtomicU32 as *const core::ffi::c_void;
        unsafe {
            __ulock_wake(
                UL_COMPARE_AND_WAIT | ULF_NO_ERRNO | ULF_WAKE_ALL,
                ptr,
                WAKE_VALUE_IGNORED,
            );
        }
    }
}

#[cfg(all(target_os = "windows", feature = "platform-os"))]
mod platform {
    use core::sync::atomic::AtomicU32;

    // WaitOnAddress / WakeByAddressAll live in API-MS-Win-Core-
    // Synch-l1-2-0.dll (synchapi.h). The hilavitkutin-linking +
    // hilavitkutin-extensions crates are the workspace's Windows
    // FFI host today; this module declares the raw extern shapes
    // and lets the libc workspace dep pull in the import library
    // (windows-sys is the future replacement; tracked under
    // hilavitkutin-build's no-std FFI follow-up).
    unsafe extern "system" {
        fn WaitOnAddress(
            address: *const core::ffi::c_void,
            compare_address: *const core::ffi::c_void,
            address_size: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI ABI; tracked: #72
            milliseconds: u32, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI ABI; tracked: #72
        ) -> i32; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI ABI; tracked: #72
        fn WakeByAddressAll(address: *const core::ffi::c_void);
    }
    const INFINITE: u32 = 0xFFFF_FFFF; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: kernel ABI constant; tracked: #72
    const SIZEOF_U32: usize = 4; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: sizeof(u32) for ABI arg; tracked: #72

    pub fn wait_u32(addr: &AtomicU32, expected: u32) {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI signature; tracked: #72
        let cmp = expected;
        let ptr = addr as *const AtomicU32 as *const core::ffi::c_void;
        let cmp_ptr = &cmp as *const u32 as *const core::ffi::c_void; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI ptr cast; tracked: #72
        unsafe {
            WaitOnAddress(ptr, cmp_ptr, SIZEOF_U32, INFINITE);
        }
    }

    pub fn wake_all_u32(addr: &AtomicU32) {
        let ptr = addr as *const AtomicU32 as *const core::ffi::c_void;
        unsafe {
            WakeByAddressAll(ptr);
        }
    }
}

#[cfg(not(any(
    all(target_os = "linux", feature = "platform-os"),
    all(target_os = "macos", feature = "platform-os"),
    all(target_os = "windows", feature = "platform-os"),
)))]
mod platform {
    use core::sync::atomic::AtomicU32;

    pub fn wait_u32(_addr: &AtomicU32, _expected: u32) {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: stub signature matches FFI ABI; tracked: #72
        // No supported wait primitive on this target. Degrade to a
        // single spin step so the caller re-checks the condition
        // on the next iteration.
        core::hint::spin_loop();
    }

    pub fn wake_all_u32(_addr: &AtomicU32) {
        // No-op on unsupported targets.
    }
}

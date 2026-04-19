//! Intrinsics smoke tests (5a3 skeleton).
//!
//! Verifies the portable shims don't panic and behave as identity /
//! no-op on stable.

use hilavitkutin::intrinsics::{
    compiler_fence_acquire, compiler_fence_release, noinline_barrier, prefetch_l1, prefetch_l2,
};

#[test]
fn compiler_fence_release_does_not_panic() {
    compiler_fence_release();
}

#[test]
fn compiler_fence_acquire_does_not_panic() {
    compiler_fence_acquire();
}

#[test]
fn prefetch_l1_on_valid_pointer_is_noop() {
    let x: u32 = 7;
    prefetch_l1(&x as *const u32);
    // No effect observable; just verifies it compiles + doesn't
    // panic on platforms without the nightly intrinsic.
    assert_eq!(x, 7);
}

#[test]
fn prefetch_l2_on_valid_pointer_is_noop() {
    let x: u64 = 42;
    prefetch_l2(&x as *const u64);
    assert_eq!(x, 42);
}

#[test]
fn noinline_barrier_is_identity() {
    let v = noinline_barrier(1234u32);
    assert_eq!(v, 1234);
}

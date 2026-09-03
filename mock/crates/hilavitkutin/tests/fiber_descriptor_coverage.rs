//! Catalogued: fiber-descriptor coverage is a correctness invariant on both paths.
//!
//! Since A2b (`run`) and A4 (`run_core_phase`), both dispatch paths run under the
//! union of the per-fiber member masks rather than an all-ones mask. So a live
//! unit that no fiber descriptor covers does not run at all.
//!
//! Coverage is established by the flatten in `derive_phase_dispatch_order`, which
//! assigns each live unit to exactly one fiber. But the flatten's bound is
//! `while u < uc && u < units.len() && next < cap`, so once the descriptor pool
//! fills it truncates silently. Two `debug_assert`s catch that in a debug build.
//! In release there is no signal: the scheduler builds, runs, and quietly never
//! executes the units it could not describe.
//!
//! The intended behaviour, asserted below: a plan too large to describe must not
//! produce a scheduler that under-dispatches. `build` should refuse it rather
//! than return one, so the failure is visible at construction instead of showing
//! up as missing output.
//!
//! Catalogued red rather than fixed here: the fix changes the `build` contract
//! (a silent degradation becomes a build failure, which is correct but is a
//! behaviour change consumers see) and wants its own round and its own decision
//! about the error shape.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin_api::platform::MemoryProviderApi;

// Stack-backed test memory provider (mirrors tests/column_dispatch.rs).
struct BumpProvider<const N: usize> {
    buf: UnsafeCell<[MaybeUninit<u8>; N]>,
    used: Cell<usize>,
}

impl<const N: usize> BumpProvider<N> {
    #[allow(dead_code)] // constructed by the catalogued case once it is unignored
    fn new() -> Self {
        Self {
            buf: UnsafeCell::new([const { MaybeUninit::uninit() }; N]),
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

#[test]
#[ignore = "catalogue: the flatten in derive_phase_dispatch_order truncates silently once the descriptor pool fills, so a plan with more fibers than capacity yields a scheduler that under-dispatches in release with no signal; the intended resolution is for build to refuse a plan it cannot fully describe rather than return one, which is a build-contract change needing its own round; tracked #340"]
fn build_refuses_a_plan_it_cannot_fully_describe() {
    // Intended shape once the guard lands: register more column-disjoint units
    // than `D::Fibers` capacity admits, so the flatten would truncate, and assert
    // that `build` returns an error naming the capacity rather than a scheduler.
    //
    // Written as the assertion the fix must satisfy, not as the behaviour today.
    // Today `build` succeeds and the overflow units silently never dispatch,
    // which is exactly what this case exists to stop being lost.
    //
    // The fixture is deliberately not constructed here: the carrier arity needed
    // to exceed the default `Fibers` capacity is large enough that writing it
    // before the contract is decided would bake in an assumption about which
    // capacity the guard checks against. The guard's round supplies both.
    panic!(
        "unimplemented catalogue case: build must refuse a plan whose fiber count \
         exceeds the descriptor capacity, rather than returning a scheduler that \
         silently under-dispatches"
    );
}

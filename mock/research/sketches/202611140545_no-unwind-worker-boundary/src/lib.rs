//! Two ways to make a generic worker body a boundary a panic cannot unwind
//! through, compiled in a build that unwinds (`panic = "unwind"`, the test
//! profile's default).
//!
//! The worker entry the engine spawns is generic over the scheduler's types and
//! takes one raw pointer, as `worker_main` does. Arm A makes that entry an
//! `extern "C"` function: an unwind reaching a non-unwinding ABI boundary
//! aborts the process. Arm B keeps a Rust entry and holds a guard whose drop
//! panics while the body unwinds, which a panic during a panic turns into an
//! abort. Neither names `std`.

#![no_std]

use core::marker::PhantomData;

/// Stands for the scheduler's type parameters, so the entry is generic as
/// `worker_main` is.
pub struct Ctx<T> {
    pub body:  fn(&T),
    pub value: T,
}

/// Arm A: the entry itself cannot unwind.
pub extern "C" fn entry_extern_c<T>(ctx: *const Ctx<T>) {
    // SAFETY: the caller hands a live `Ctx`.
    let ctx = unsafe { &*ctx };
    (ctx.body)(&ctx.value);
}

/// Arm B: a Rust entry whose guard turns an unwind into an abort.
pub fn entry_guard<T>(ctx: *const Ctx<T>) {
    struct AbortOnUnwind<T>(PhantomData<T>);
    impl<T> Drop for AbortOnUnwind<T> {
        fn drop(&mut self) {
            // Runs only while unwinding, since the body forgets the guard on
            // the way out; a panic during a panic aborts.
            panic!("a unit body unwound out of the worker");
        }
    }
    let guard = AbortOnUnwind::<T>(PhantomData);
    // SAFETY: the caller hands a live `Ctx`.
    let ctx = unsafe { &*ctx };
    (ctx.body)(&ctx.value);
    core::mem::forget(guard);
}

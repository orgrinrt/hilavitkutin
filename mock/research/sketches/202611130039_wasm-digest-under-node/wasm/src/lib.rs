//! The wasm front end: a `cdylib` for `wasm32-unknown-unknown` that runs a
//! case of the kernel and leaves the digests and the report in its linear
//! memory for the host to read.
//!
//! What this crate needed beyond the kernel: a panic handler, because a
//! `no_std` `cdylib` has none; `#[unsafe(no_mangle)]` `extern "C"` exports;
//! and two statics for the host to read. No allocator, no imports, no
//! `main`.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

use core::ptr::{addr_of, addr_of_mut, copy_nonoverlapping};

use wasm_digest_kernel::{STEPS, run_case};

#[cfg(not(feature = "omit-panic-handler"))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    core::arch::wasm32::unreachable()
}

static mut OUT: [u64; STEPS] = [0; STEPS];
static mut STATUS: [u64; 5] = [0; 5];

/// Runs one case. The digests land in the buffer `out_ptr` names and the
/// report words in the one `status_ptr` names. Returns the digests written.
#[unsafe(no_mangle)]
pub extern "C" fn run_case_export(case: u32) -> u32 {
    let mut out = [0u64; STEPS];
    let report = run_case(case, &mut out);
    let words = report.words();
    // SAFETY: the module is single-threaded and the host reads the statics
    // only after this call returns; both copies stay inside their statics.
    unsafe {
        copy_nonoverlapping(out.as_ptr(), addr_of_mut!(OUT).cast::<u64>(), STEPS);
        copy_nonoverlapping(words.as_ptr(), addr_of_mut!(STATUS).cast::<u64>(), 5);
    }
    report.written as u32
}

#[unsafe(no_mangle)]
pub extern "C" fn out_ptr() -> *const u64 {
    addr_of!(OUT).cast::<u64>()
}

#[unsafe(no_mangle)]
pub extern "C" fn status_ptr() -> *const u64 {
    addr_of!(STATUS).cast::<u64>()
}

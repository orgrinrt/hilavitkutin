//! The wasm host: the loop exported to a JavaScript engine.
//!
//! `run_case_export` runs one case and leaves its digests in `OUT` and its
//! report in `STATUS`, which `run.mjs` reads through `out_ptr` and
//! `status_ptr`. `STATUS` is three words: the digests written, whether the run
//! refused, and the frame it refused at.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

use core::ptr::{addr_of, addr_of_mut};

use cross_target_loop::{FRAMES, run_case};
use hilavitkutin_contracts::Maybe;
use hilavitkutin_contracts::domain::Word;
use hilavitkutin_contracts::num::USize;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    core::arch::wasm32::unreachable()
}

static mut OUT: [u64; FRAMES] = [0; FRAMES];
static mut STATUS: [u64; 3] = [0; 3];

#[unsafe(no_mangle)]
pub extern "C" fn run_case_export(case: u32) -> u32 {
    let mut out = [Word::of([0; 8]); FRAMES];
    let report = run_case(USize::from_host(case as usize), &mut out);
    let (refused, frame) = match report.refused {
        Maybe::Is(frame) => (1, frame.host() as u64),
        Maybe::Isnt => (0, 0),
    };
    let written = report.written.host();
    // SAFETY: the module is single-threaded, and the host reads the statics
    // only after this call returns.
    unsafe {
        let o = &mut *addr_of_mut!(OUT);
        for (slot, word) in o.iter_mut().zip(out.iter()) {
            *slot = u64::from_le_bytes(word.bytes());
        }
        *addr_of_mut!(STATUS) = [written as u64, refused, frame];
    }
    written as u32
}

#[unsafe(no_mangle)]
pub extern "C" fn out_ptr() -> *const u64 {
    addr_of!(OUT).cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn status_ptr() -> *const u64 {
    addr_of!(STATUS).cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn cases() -> u32 {
    cross_target_loop::CASES.host() as u32
}

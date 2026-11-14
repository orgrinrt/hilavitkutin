//! The gate's negative control: a call through a stored function pointer.
//!
//! Check 1 refuses exactly this, so the gate requires it to fail here. A check
//! that passed this fixture would be blind, as the old text match was on every
//! x86_64 listing objdump prints, and the gate fails the whole run when it does.

use super::*;

type Step = fn(u32) -> u32;

fn step(x: u32) -> u32 {
    x.wrapping_mul(0x9E37_79B1) ^ 7
}

#[unsafe(no_mangle)]
pub extern "C" fn asm_gate_control_stored_pointer() -> u64 {
    // `black_box` hides which function the pointer holds, so the call cannot
    // be turned direct.
    let f: Step = black_box(step as Step);
    let mut acc = 0u32;
    for i in 0 .. N as u32 {
        acc = acc.wrapping_add(f(i));
    }
    black_box(u64::from(acc))
}

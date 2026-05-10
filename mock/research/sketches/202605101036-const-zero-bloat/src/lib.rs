//! Sketch — verify that `pub const` declarations emit zero binary
//! symbols, while `pub static` allocates space.
//!
//! Hypothesis: `pub const FOO: T = expr;` is purely a compile-time
//! entity. It does not appear in `nm` output and does not contribute
//! to binary size. Each use site inlines the value.
//!
//! Counter (sanity check): `pub static FOO: T = expr;` DOES appear
//! in `nm` output and DOES contribute bytes to the binary.

#![no_std]

// THE CONST. We expect this to NOT appear in `nm` output.
pub const SKETCH_CONST_DEFAULT_DECAY: u64 = 0xDEAD_BEEF_DEAD_BEEF;

// THE STATIC. We expect this DOES appear in `nm` output.
#[unsafe(no_mangle)]
pub static SKETCH_STATIC_DEFAULT_DECAY: u64 = 0xCAFE_BABE_CAFE_BABE;

// Reference the const at one use site, so we can confirm:
//   - The use site has the literal value baked in.
//   - The const itself is still not a symbol.
#[unsafe(no_mangle)]
pub extern "C" fn read_const() -> u64 {
    SKETCH_CONST_DEFAULT_DECAY
}

#[unsafe(no_mangle)]
pub extern "C" fn read_static() -> u64 {
    SKETCH_STATIC_DEFAULT_DECAY
}

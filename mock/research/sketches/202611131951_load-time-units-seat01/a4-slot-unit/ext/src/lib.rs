#![feature(const_trait_impl)]
//! A mod's native library. Plain C ABI, as op put it on 13 November ("The
//! extension model is a literal c abi, it's standard ffi"), so the host does
//! not care what built it.

use hilavitkutin_str::{Str, static_entries, str_const};

/// The kernel a slot unit calls once per morsel: `out[i] = pos[i] + vel[i] * 2`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ext_kernel(pos: *const i32, vel: *const i32, out: *mut i32, len: usize) {
    for i in 0 .. len {
        unsafe {
            out.add(i)
                .write((*pos.add(i)).wrapping_add((*vel.add(i)).wrapping_mul(2)))
        };
    }
}

/// A content name the extension declares, as the 28-bit id of its `Str`.
#[unsafe(no_mangle)]
pub extern "C" fn ext_mint() -> u32 {
    let s: Str = str_const!("mod.kind.heat");
    s.id().to_raw()
}

/// How many const strings this image's own section holds.
#[unsafe(no_mangle)]
pub extern "C" fn ext_entries() -> usize {
    static_entries().len()
}

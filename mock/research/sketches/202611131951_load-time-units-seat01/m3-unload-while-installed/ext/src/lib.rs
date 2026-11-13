//! A mod's kernel over one morsel: `out[i] = a[i] * 7 + 1`. Plain C ABI.

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ext_kernel(a: *const i64, out: *mut i64, len: usize) {
    for i in 0 .. len {
        unsafe {
            out.add(i)
                .write((*a.add(i)).wrapping_mul(7).wrapping_add(1))
        };
    }
}

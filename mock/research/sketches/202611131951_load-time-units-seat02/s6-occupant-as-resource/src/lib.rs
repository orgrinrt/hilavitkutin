//! Spike, seat 02 phase two. Stand-in occupants for an extension surface:
//! two C-ABI kernels over one morsel, as a loaded library would export them.
//! The transport (a fresh cdylib through the loader) is s1's and seat 01's
//! a4; this spike is about what the plan does when the occupant changes.

/// out[i] = a[i] + 1
pub extern "C" fn occupant_v1(a: *const i64, out: *mut i64, len: usize) {
    for i in 0 .. len {
        unsafe { *out.add(i) = *a.add(i) + 1 };
    }
}

/// out[i] = a[i] * 3
pub extern "C" fn occupant_v2(a: *const i64, out: *mut i64, len: usize) {
    for i in 0 .. len {
        unsafe { *out.add(i) = *a.add(i) * 3 };
    }
}

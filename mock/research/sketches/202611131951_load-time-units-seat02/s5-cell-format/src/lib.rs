//! Spike s5, seat 02: what the trunk's `ColumnValue` admits, and the shape of the
//! code a consumer's own lane format gets.

use hilavitkutin_api::ColumnValue;

/// Only a `ColumnValue` gets through; the trunk's bound is `Copy + 'static`.
pub const fn width_of<T: ColumnValue>() -> usize {
    T::BIT_WIDTH.0
}

/// A consumer's own fixed point, sixteen fraction bits in an `i32`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Q16(pub i32);

/// kaski's lane shape, `Fixed<FRAC>(i64)` with an `i128` product.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Q32(pub i64);

/// A consumer's packed cell: four sixteen-bit lanes in one record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(8))]
pub struct Lanes4x16(pub [u16; 4]);

/// A consumer's wide record, wider than the spec's sixteen bytes (R3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Wide32(pub [u64; 4]);

/// The morsel loop a unit over a `Column<Q16>` would run: product, then shift.
#[unsafe(no_mangle)]
#[inline(never)]
pub fn s5_mul_q16(a: &[Q16], b: &[Q16], out: &mut [Q16]) {
    let n = out.len().min(a.len()).min(b.len());
    for i in 0..n {
        out[i] = Q16(((a[i].0 as i64 * b[i].0 as i64) >> 16) as i32);
    }
}

/// The same over kaski's `Q32` with the `i128` product.
#[unsafe(no_mangle)]
#[inline(never)]
pub fn s5_mul_q32(a: &[Q32], b: &[Q32], out: &mut [Q32]) {
    let n = out.len().min(a.len()).min(b.len());
    for i in 0..n {
        out[i] = Q32(((a[i].0 as i128 * b[i].0 as i128) >> 32) as i64);
    }
}

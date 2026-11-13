//! A crate writing block kernels against the engine. It writes no `unsafe`
//! and holds no slice, pointer or integer row: every row it touches is an
//! `Index` the engine minted.

#![no_std]
#![forbid(unsafe_code)]

use engine::{BlockKernel, Morsel};

/// Writes each row's read value doubled.
pub struct Double;

impl BlockKernel<i64, i64> for Double {
    fn block(&self, mut morsel: Morsel<'_, i64, i64>) {
        morsel.each(|mut block| {
            for index in block.indices() {
                let value = block.read(index);
                block.write(index, value * 2);
            }
        });
    }
}

/// Writes each row's read value less the morsel's largest: two passes over
/// one morsel's indices, the range only a block kernel sees.
pub struct BelowMax;

impl BlockKernel<i64, i64> for BelowMax {
    fn block(&self, mut morsel: Morsel<'_, i64, i64>) {
        morsel.each(|mut block| {
            let mut max = i64::MIN;
            for index in block.indices() {
                max = max.max(block.read(index));
            }
            for index in block.indices() {
                let value = block.read(index);
                block.write(index, value - max);
            }
        });
    }
}

/// Writes at each row the read value of the row the engine yielded before it
/// in the same morsel, and the first row its own: an index kept across rows
/// within one `each`.
pub struct Previous;

impl BlockKernel<i64, i64> for Previous {
    fn block(&self, mut morsel: Morsel<'_, i64, i64>) {
        morsel.each(|mut block| {
            let mut previous = None;
            for index in block.indices() {
                let from = previous.unwrap_or(index);
                let value = block.read(from);
                block.write(index, value);
                previous = Some(index);
            }
        });
    }
}

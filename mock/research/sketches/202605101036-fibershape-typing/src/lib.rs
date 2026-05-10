//! Sketch — FiberShape as a sealed generic-parameter trait.
//!
//! Validates that monomorphising dispatch over `S: FiberShape` produces
//! one specialised fn per shape, with shape-specific constants baked
//! into each body. Sketch only — sketch crate names match
//! conceptually but do not depend on substrate types.

#![no_std]
#![allow(dead_code)]

// ---------- sealed family ----------

mod private {
    pub trait Sealed {}
}

pub trait FiberShape: private::Sealed {
    /// stride between record groups in the access pattern, in records
    const STRIDE: usize;
    /// prefetch lookahead distance, in records
    const PREFETCH_AHEAD: usize;
    /// morsel window size in records
    const MORSEL_RECORDS: usize;
}

// ---------- four shape variants ----------

pub struct Sequential;
impl private::Sealed for Sequential {}
impl FiberShape for Sequential {
    const STRIDE: usize = 1;
    const PREFETCH_AHEAD: usize = 8;
    const MORSEL_RECORDS: usize = 256;
}

pub struct Strided;
impl private::Sealed for Strided {}
impl FiberShape for Strided {
    const STRIDE: usize = 8;
    const PREFETCH_AHEAD: usize = 16;
    const MORSEL_RECORDS: usize = 128;
}

pub struct Scattered;
impl private::Sealed for Scattered {}
impl FiberShape for Scattered {
    const STRIDE: usize = 64;
    const PREFETCH_AHEAD: usize = 32;
    const MORSEL_RECORDS: usize = 64;
}

pub struct Pointwise;
impl private::Sealed for Pointwise {}
impl FiberShape for Pointwise {
    const STRIDE: usize = 1;
    const PREFETCH_AHEAD: usize = 0;
    const MORSEL_RECORDS: usize = 512;
}

// ---------- the dispatch fn (monomorphised per shape) ----------

/// One reduce-style fiber loop, parameterised by shape.
///
/// The hypothesis is that monomorphising this fn over each `S`
/// produces a body where `S::STRIDE`, `S::PREFETCH_AHEAD`, and
/// `S::MORSEL_RECORDS` are inlined as immediates, and no
/// shape-discriminating branch exists.
#[inline(never)]
pub fn dispatch_per_shape<S: FiberShape>(
    column: &[u64],
    record_count: usize,
) -> u64 {
    let mut acc: u64 = 0;
    let stride = S::STRIDE;
    let morsel = S::MORSEL_RECORDS;
    let lookahead = S::PREFETCH_AHEAD;
    let mut i = 0;
    while i < record_count {
        // morsel-level window
        let morsel_end = core::cmp::min(i + morsel, record_count);
        let mut j = i;
        while j < morsel_end {
            // unsafe-style prefetch hint emulated as a benign read
            let prefetch_at = j + lookahead * stride;
            if prefetch_at < column.len() {
                acc = acc.wrapping_add(column[prefetch_at]);
            }
            acc = acc.wrapping_add(column[j]);
            j += stride;
        }
        i = morsel_end;
    }
    acc
}

// ---------- explicit instantiation points for asm inspection ----------

#[inline(never)]
pub fn call_sequential(column: &[u64], record_count: usize) -> u64 {
    dispatch_per_shape::<Sequential>(column, record_count)
}

#[inline(never)]
pub fn call_strided(column: &[u64], record_count: usize) -> u64 {
    dispatch_per_shape::<Strided>(column, record_count)
}

#[inline(never)]
pub fn call_scattered(column: &[u64], record_count: usize) -> u64 {
    dispatch_per_shape::<Scattered>(column, record_count)
}

#[inline(never)]
pub fn call_pointwise(column: &[u64], record_count: usize) -> u64 {
    dispatch_per_shape::<Pointwise>(column, record_count)
}

// ---------- counter-example: runtime enum (Topic 4 D1) ----------

#[derive(Clone, Copy)]
pub enum ShapeKind {
    Sequential,
    Strided,
    Scattered,
    Pointwise,
}

#[inline(never)]
pub fn dispatch_runtime_match(
    kind: ShapeKind,
    column: &[u64],
    record_count: usize,
) -> u64 {
    let (stride, morsel, lookahead) = match kind {
        ShapeKind::Sequential => (1usize, 256usize, 8usize),
        ShapeKind::Strided => (8, 128, 16),
        ShapeKind::Scattered => (64, 64, 32),
        ShapeKind::Pointwise => (1, 512, 0),
    };
    let mut acc: u64 = 0;
    let mut i = 0;
    while i < record_count {
        let morsel_end = core::cmp::min(i + morsel, record_count);
        let mut j = i;
        while j < morsel_end {
            let prefetch_at = j + lookahead * stride;
            if prefetch_at < column.len() {
                acc = acc.wrapping_add(column[prefetch_at]);
            }
            acc = acc.wrapping_add(column[j]);
            j += stride;
        }
        i = morsel_end;
    }
    acc
}

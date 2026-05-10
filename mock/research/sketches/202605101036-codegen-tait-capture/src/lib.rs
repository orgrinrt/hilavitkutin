//! Sketch — DispatchCodegen TAIT with realistic state capture.
//!
//! Audit-2 C3 follow-up to the original `codegen-entrypoint-tait`
//! sketch. The original used three trivial inline-always WU bodies
//! and an empty closure. This sketch stresses the realistic envelope:
//!
//! - Ten const-generic parameters on `Cfg`.
//! - WU bodies that do real work (column reads + fixed-point math +
//!   conditional branches).
//! - The TAIT closure captures the WU tuple AND mutable state.
//! - Two sealed `DispatchCodegen` impls.
//!
//! The goal is to verify that LLVM still proves through the
//! abstraction at full envelope. The asm should show:
//!
//! 1. Distinct symbols per `<Codegen, Cfg>` monomorphisation.
//! 2. Const-generic values from `Cfg` baked as immediates.
//! 3. Zero `blr` indirect calls in the dispatch body.

#![no_std]
#![allow(dead_code)]
#![feature(impl_trait_in_assoc_type)]

// ---------- sealed trait family ----------

mod private {
    pub trait Sealed {}
}

/// Codegen configuration: ten const-generic parameters representing the
/// real envelope.
pub struct CodegenCfg<
    const WU_COUNT: usize,
    const MORSEL_RECORDS: usize,
    const FIBER_SHAPE_KIND: u8,
    const EMA_HISTORY_DEPTH_FAST: usize,
    const EMA_HISTORY_DEPTH_MID: usize,
    const EMA_HISTORY_DEPTH_SLOW: usize,
    // four derived parameters via CeilingDiv (modelled as raw constants here)
    const MORSEL_CHUNKS: usize,
    const EMA_BUCKETS_FAST: usize,
    const EMA_BUCKETS_MID: usize,
    const EMA_BUCKETS_SLOW: usize,
>;

/// Realistic captured state — a column borrow + mutable accumulator.
pub struct DispatchCtx<'a> {
    pub column_a: &'a [u64],
    pub column_b: &'a [u32],
    pub accumulator: &'a mut u64,
    pub branch_counter: &'a mut u32,
}

/// Adapt sidecar — three EMA history rings (representing the per-fiber state).
pub struct AdaptSidecar<
    const D_FAST: usize,
    const D_MID: usize,
    const D_SLOW: usize,
> {
    pub fast: [u32; D_FAST],
    pub mid: [u32; D_MID],
    pub slow: [u32; D_SLOW],
    pub head_fast: usize,
    pub head_mid: usize,
    pub head_slow: usize,
}

/// The TAIT-using sealed trait.
pub trait DispatchCodegen<Cfg>: private::Sealed {
    /// TAIT-bound closure type.
    type CoreDispatch: for<'a> Fn(&mut DispatchCtx<'a>, usize);

    fn build() -> Self::CoreDispatch;
}

// ---------- realistic WU bodies ----------

#[inline(always)]
fn wu_alpha(ctx: &mut DispatchCtx<'_>, i: usize) -> u64 {
    // Column read + fixed-point-ish op + branch.
    if i >= ctx.column_a.len() {
        return 0;
    }
    let a = ctx.column_a[i];
    // Q32.32 multiply emulated as plain u64 arith.
    let mixed = a.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    if mixed & 1 == 0 {
        *ctx.branch_counter = ctx.branch_counter.wrapping_add(1);
        mixed.rotate_left(17)
    } else {
        mixed.rotate_right(13)
    }
}

#[inline(always)]
fn wu_beta(ctx: &mut DispatchCtx<'_>, i: usize) -> u64 {
    if i >= ctx.column_b.len() {
        return 0;
    }
    let b = ctx.column_b[i] as u64;
    b.wrapping_mul(0xBF58_476D_1CE4_E5B9).wrapping_add(i as u64)
}

#[inline(always)]
fn wu_gamma(ctx: &mut DispatchCtx<'_>, i: usize) -> u64 {
    let a = if i < ctx.column_a.len() { ctx.column_a[i] } else { 0 };
    let b = if i < ctx.column_b.len() { ctx.column_b[i] as u64 } else { 0 };
    (a ^ b.rotate_left(11)).wrapping_add(*ctx.accumulator)
}

// ---------- impl A: StandardCodegen ----------

pub struct StandardCodegen;
impl private::Sealed for StandardCodegen {}

impl<
    const WU_COUNT: usize,
    const MORSEL_RECORDS: usize,
    const FIBER_SHAPE_KIND: u8,
    const EMA_HISTORY_DEPTH_FAST: usize,
    const EMA_HISTORY_DEPTH_MID: usize,
    const EMA_HISTORY_DEPTH_SLOW: usize,
    const MORSEL_CHUNKS: usize,
    const EMA_BUCKETS_FAST: usize,
    const EMA_BUCKETS_MID: usize,
    const EMA_BUCKETS_SLOW: usize,
>
DispatchCodegen<
    CodegenCfg<
        WU_COUNT,
        MORSEL_RECORDS,
        FIBER_SHAPE_KIND,
        EMA_HISTORY_DEPTH_FAST,
        EMA_HISTORY_DEPTH_MID,
        EMA_HISTORY_DEPTH_SLOW,
        MORSEL_CHUNKS,
        EMA_BUCKETS_FAST,
        EMA_BUCKETS_MID,
        EMA_BUCKETS_SLOW,
    >,
> for StandardCodegen
{
    // TAIT: the closure type is opaque-but-named.
    type CoreDispatch = impl for<'a> Fn(&mut DispatchCtx<'a>, usize);

    fn build() -> Self::CoreDispatch {
        // Closure captures all const-generic state as immediates.
        // Body iterates one morsel chunk: MORSEL_RECORDS records, three WU calls per record.
        |ctx: &mut DispatchCtx<'_>, base: usize| {
            let end = base + MORSEL_RECORDS;
            let mut acc: u64 = 0;
            let mut i = base;
            while i < end {
                let a = wu_alpha(ctx, i);
                let b = wu_beta(ctx, i);
                let g = wu_gamma(ctx, i);
                acc = acc.wrapping_add(a ^ b ^ g);
                i += 1;
            }
            *ctx.accumulator = ctx.accumulator.wrapping_add(acc);
        }
    }
}

// ---------- impl B: BenchInstrumentedCodegen ----------

pub struct BenchInstrumentedCodegen;
impl private::Sealed for BenchInstrumentedCodegen {}

impl<
    const WU_COUNT: usize,
    const MORSEL_RECORDS: usize,
    const FIBER_SHAPE_KIND: u8,
    const EMA_HISTORY_DEPTH_FAST: usize,
    const EMA_HISTORY_DEPTH_MID: usize,
    const EMA_HISTORY_DEPTH_SLOW: usize,
    const MORSEL_CHUNKS: usize,
    const EMA_BUCKETS_FAST: usize,
    const EMA_BUCKETS_MID: usize,
    const EMA_BUCKETS_SLOW: usize,
>
DispatchCodegen<
    CodegenCfg<
        WU_COUNT,
        MORSEL_RECORDS,
        FIBER_SHAPE_KIND,
        EMA_HISTORY_DEPTH_FAST,
        EMA_HISTORY_DEPTH_MID,
        EMA_HISTORY_DEPTH_SLOW,
        MORSEL_CHUNKS,
        EMA_BUCKETS_FAST,
        EMA_BUCKETS_MID,
        EMA_BUCKETS_SLOW,
    >,
> for BenchInstrumentedCodegen
{
    type CoreDispatch = impl for<'a> Fn(&mut DispatchCtx<'a>, usize);

    fn build() -> Self::CoreDispatch {
        |ctx: &mut DispatchCtx<'_>, base: usize| {
            // Same loop body PLUS a branch-count snapshot at end.
            let end = base + MORSEL_RECORDS;
            let mut acc: u64 = 0;
            let pre_branches = *ctx.branch_counter;
            let mut i = base;
            while i < end {
                let a = wu_alpha(ctx, i);
                let b = wu_beta(ctx, i);
                let g = wu_gamma(ctx, i);
                acc = acc.wrapping_add(a.wrapping_mul(b).wrapping_add(g));
                i += 1;
            }
            *ctx.accumulator = ctx.accumulator.wrapping_add(acc);
            *ctx.branch_counter = ctx.branch_counter.wrapping_add(
                (*ctx.branch_counter).wrapping_sub(pre_branches),
            );
        }
    }
}

// ---------- explicit instantiations for asm inspection ----------

/// Concrete Cfg used as the test envelope.
type TestCfg = CodegenCfg<
    3,    // WU_COUNT
    256,  // MORSEL_RECORDS
    1,    // FIBER_SHAPE_KIND (Sequential)
    16,   // EMA_HISTORY_DEPTH_FAST
    64,   // EMA_HISTORY_DEPTH_MID
    256,  // EMA_HISTORY_DEPTH_SLOW
    4,    // MORSEL_CHUNKS
    2,    // EMA_BUCKETS_FAST
    8,    // EMA_BUCKETS_MID
    32,   // EMA_BUCKETS_SLOW
>;

/// Alternate Cfg with different MORSEL_RECORDS to verify per-instantiation constants.
type AltCfg = CodegenCfg<
    3,
    128,  // MORSEL_RECORDS = 128 (different from TestCfg)
    1,
    16,
    64,
    256,
    8,
    2,
    8,
    32,
>;

#[inline(never)]
pub fn call_standard_test(ctx: &mut DispatchCtx<'_>, base: usize) {
    let dispatch = <StandardCodegen as DispatchCodegen<TestCfg>>::build();
    dispatch(ctx, base);
}

#[inline(never)]
pub fn call_standard_alt(ctx: &mut DispatchCtx<'_>, base: usize) {
    let dispatch = <StandardCodegen as DispatchCodegen<AltCfg>>::build();
    dispatch(ctx, base);
}

#[inline(never)]
pub fn call_bench_test(ctx: &mut DispatchCtx<'_>, base: usize) {
    let dispatch = <BenchInstrumentedCodegen as DispatchCodegen<TestCfg>>::build();
    dispatch(ctx, base);
}

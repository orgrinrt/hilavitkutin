//! Sketch — sealed trait + TAIT for dispatch codegen entrypoint.
//!
//! Hypothesis: a sealed `trait DispatchCodegen<Cfg>: Sealed` with
//! `type CoreDispatch = impl Fn(&CoreCtx) -> u64` lowers in a way LLVM
//! can prove through. The call site through `<StandardCodegen as
//! DispatchCodegen<MyCfg>>::build(...)` returns a singleton anonymous
//! fn type; calling it should emit a direct `bl` (or be inlined),
//! NOT an indirect `blr` through a register.
//!
//! Counter-hypothesis (what we want to AVOID): a struct holding fn
//! pointers (`struct CoreDispatch { fns: [WuFn; N] }`) lowers to
//! 12.6x penalty per Domain 17 L1540.

#![no_std]
#![feature(impl_trait_in_assoc_type)]

// Sealed family per Topic 3 S2.
mod sealed {
    pub trait Sealed {}
}

// Standin types for the real Cfg / Ctx that the engine ships.
// We only need shapes the trait can quantify over, not the real
// hilavitkutin types — this is a sketch about *lowering*, not API.
pub struct CoreCtx {
    pub record_index: u64,
    pub fiber_id: u64,
}

pub trait RunCfg: 'static {
    type Err: 'static;
}

pub struct MyCfg;
impl RunCfg for MyCfg {
    type Err = ();
}

// THE TRAIT under test.
pub trait DispatchCodegen<Cfg: RunCfg>: sealed::Sealed {
    type CoreDispatch: Fn(&CoreCtx) -> u64;

    fn build() -> Self::CoreDispatch;
}

// The single sealed impl shipped in v1.
pub struct StandardCodegen;
impl sealed::Sealed for StandardCodegen {}

impl<Cfg: RunCfg> DispatchCodegen<Cfg> for StandardCodegen {
    type CoreDispatch = impl Fn(&CoreCtx) -> u64;

    #[inline(never)] // matches Domain 17 inline-discipline for fiber-dispatch
    fn build() -> Self::CoreDispatch {
        // The emitted closure mimics what the real codegen would emit:
        // a per-core fn that runs a sequence of "WU bodies" with
        // resources cached locally.
        //
        // For the sketch the bodies are inline arithmetic so there's
        // something concrete for LLVM to optimise through. Real
        // codegen emits the rust-pipe pattern (Domain 17 L1564-1581).
        |ctx: &CoreCtx| -> u64 {
            // simulated WU bodies; #[inline(always)] discipline applies
            // to real WU::execute calls.
            let a = wu_a(ctx);
            let b = wu_b(ctx);
            let c = wu_c(ctx);
            a.wrapping_mul(b).wrapping_add(c)
        }
    }
}

#[inline(always)]
fn wu_a(ctx: &CoreCtx) -> u64 {
    ctx.record_index.wrapping_mul(7).wrapping_add(13)
}

#[inline(always)]
fn wu_b(ctx: &CoreCtx) -> u64 {
    ctx.fiber_id.wrapping_mul(31).wrapping_add(ctx.record_index)
}

#[inline(always)]
fn wu_c(ctx: &CoreCtx) -> u64 {
    ctx.record_index ^ ctx.fiber_id
}

// THE CALL-SITE under inspection.
//
// This fn is the equivalent of what the thread pool will do: call
// the codegen-emitted fn for each record. If LLVM proves through the
// trait + TAIT, this body should compile to a tight loop with the
// emitted closure body inlined directly — zero `blr`.
#[inline(never)] // matches Domain 17 inline-discipline for per-core program
pub fn call_through_trait(record_count: u64) -> u64 {
    let dispatch =
        <StandardCodegen as DispatchCodegen<MyCfg>>::build();

    let mut acc: u64 = 0;
    let mut i: u64 = 0;
    while i < record_count {
        let ctx = CoreCtx { record_index: i, fiber_id: 0 };
        // The call below is the hot-path indirection question.
        // Direct call: LLVM proved through TAIT. WIN.
        // Indirect call: LLVM saw an opaque fn type. FAIL.
        acc = acc.wrapping_add(dispatch(&ctx));
        i = i.wrapping_add(1);
    }
    acc
}

// FAIL-pattern reference for comparison: the same workload through
// a struct holding a fn pointer. Domain 17 L1540 says this loses
// 12.6x. Including it lets the disasm comparison make the
// difference visible in the same crate.
type WuFn = fn(&CoreCtx) -> u64;

pub struct StructDispatch {
    pub f: WuFn,
}

#[inline(never)]
pub fn call_through_struct_field(
    record_count: u64,
    dispatch: &StructDispatch,
) -> u64 {
    let mut acc: u64 = 0;
    let mut i: u64 = 0;
    while i < record_count {
        let ctx = CoreCtx { record_index: i, fiber_id: 0 };
        // This is the FAIL pattern. Expected: indirect `blr` /
        // `call *reg` through the field load.
        acc = acc.wrapping_add((dispatch.f)(&ctx));
        i = i.wrapping_add(1);
    }
    acc
}

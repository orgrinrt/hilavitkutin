//! E4 slice-3 de-risk: the engine-to-meta bridge accessor (candidate 1).
//!
//! The resource Copy wall (resolution `202606090100` Correction 2) proved that
//! mutable meta state cannot ride a consumer `Resource<T>` (arena values are
//! `Copy`, read-only via `ctx.resource()`). So meta state is engine-owned mutable
//! state. Candidate 1: an engine-owned meta-state block (a scheduler field,
//! interior-mutable via `Cell`, NOT `Copy`-constrained, NOT registered in
//! `Stores`), read by an `OnMeta` work unit through a dedicated accessor on its
//! `Ctx`, distinct from the normal access-set resource accessor.
//!
//! This sketch proves the mechanism + its gating compile:
//!   1. the meta block holds a mutable (`Cell`) field the engine writes directly;
//!   2. an `OnMeta` work unit's `Ctx` carries a reference to the block and reads
//!      it through a `meta::<T>()` accessor (`T: MetaAccess`);
//!   3. the accessor is GATED: a consumer work unit's `Ctx` (no meta pointer)
//!      does NOT have it, so a consumer cannot reach meta state at compile time
//!      (the natural `MetaAccess` enforcement). Shown by the `wall` mod, which
//!      does not compile if uncommented.
//!
//! The meta-pointer is a `Ctx` type parameter defaulted to `MetaNil` for
//! consumers (mirrors the slice-1 `write_virtuals = VirtNil` default, so existing
//! consumer Ctx aliases need no change); the engine wires a real `MetaRef` only
//! for `OnMeta` work units.
//!
//! Outcome at the bottom.

#![allow(dead_code)]

use core::cell::Cell;
use core::marker::PhantomData;

// ---- meta resource markers + MetaAccess (mirror api meta.rs) ----
struct SchedulerMetrics {
    pass_count: Cell<u32>,
}
trait MetaAccess {}
impl MetaAccess for SchedulerMetrics {}

// ---- engine-owned meta-state block (a scheduler field, NOT a Store) ----
// Not `Copy`, holds `Cell`s; the engine writes it directly. A type-keyed
// `MetaField` projects each meta resource out of the block (the real engine uses
// a Selector-style walk; here one field stands in for the mechanism).
struct MetaBlock {
    metrics: SchedulerMetrics,
}
trait MetaField {
    fn project(block: &MetaBlock) -> &Self;
}
impl MetaField for SchedulerMetrics {
    fn project(block: &MetaBlock) -> &Self {
        &block.metrics
    }
}

// ---- the meta-pointer Ctx parameter ----
// `MetaNil` for consumers (no meta pointer); `MetaRef<'f>` for OnMeta WUs.
#[derive(Clone, Copy)]
struct MetaNil;
#[derive(Clone, Copy)]
struct MetaRef<'f>(&'f MetaBlock);

// ---- minimal Ctx generic over the meta pointer `MP` (defaulted MetaNil) ----
struct Ctx<'f, MP = MetaNil> {
    // (real EngineCtx has many params; only the meta pointer matters here)
    meta_ptr: MP,
    _f: PhantomData<&'f ()>,
}

// The meta accessor exists ONLY on a Ctx carrying a `MetaRef` (an OnMeta WU's
// Ctx). A consumer Ctx (`MP = MetaNil`) has no such impl, so `ctx.meta()` does
// not resolve there: compile-time MetaAccess enforcement, for free.
impl<'f> Ctx<'f, MetaRef<'f>> {
    #[inline]
    fn meta<T: MetaAccess + MetaField>(&self) -> &T {
        T::project(self.meta_ptr.0)
    }
}

// ---- schedule markers (mirror api) ----
struct Always;
struct OnMeta<V>(PhantomData<V>);
struct ScheduleEnd;

// ---- a minimal WorkUnit with a meta-pointer-parameterised Ctx ----
trait WorkUnit<Schedule> {
    type Ctx<'f>;
    fn execute<'f>(&self, ctx: &Self::Ctx<'f>);
}

// OnMeta<ScheduleEnd> adaptation hook: Ctx carries a MetaRef, reads + observes
// SchedulerMetrics through the bridge accessor.
struct AdaptHook {
    observed: Cell<u32>,
}
impl WorkUnit<OnMeta<ScheduleEnd>> for AdaptHook {
    type Ctx<'f> = Ctx<'f, MetaRef<'f>>;
    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        let m: &SchedulerMetrics = ctx.meta();
        self.observed.set(m.pass_count.get());
    }
}

// A consumer (Always): Ctx has the default MetaNil meta pointer, so no meta
// accessor. Its body cannot reach meta state.
struct ConsumerWu;
impl WorkUnit<Always> for ConsumerWu {
    type Ctx<'f> = Ctx<'f, MetaNil>;
    fn execute<'f>(&self, _ctx: &Self::Ctx<'f>) {
        // no `_ctx.meta()` available here (MetaNil Ctx has no accessor)
    }
}

// The wall: a consumer trying to reach meta state does NOT compile.
mod wall {
    // use super::*;
    // fn consumer_reaches_meta(ctx: &super::Ctx<super::MetaNil>) {
    //     let _ = ctx.meta::<super::SchedulerMetrics>();
    //     // ^ error[E0599]: no method named `meta` found for `Ctx<MetaNil>`
    //     //   (the accessor is impl'd only for Ctx<MetaRef>) -> consumer cannot
    //     //   reach meta state: compile-time MetaAccess enforcement.
    // }
}

fn main() {
    // Engine owns the meta block as a field; writes it directly (no registration,
    // no Copy, no Selector witness, no specialization).
    let block = MetaBlock { metrics: SchedulerMetrics { pass_count: Cell::new(0) } };

    let hook = AdaptHook { observed: Cell::new(999) };
    let consumer = ConsumerWu;

    // Frame 1: engine increments pass_count (its own field), then dispatches the
    // consumer band, then the OnMeta hook with a Ctx carrying a MetaRef.
    block.metrics.pass_count.set(block.metrics.pass_count.get() + 1);
    consumer.execute(&Ctx { meta_ptr: MetaNil, _f: PhantomData });
    hook.execute(&Ctx { meta_ptr: MetaRef(&block), _f: PhantomData });
    assert_eq!(hook.observed.get(), 1, "hook read engine-owned pass_count = 1");

    // Frame 2: same, count advances; the hook reads the updated value.
    block.metrics.pass_count.set(block.metrics.pass_count.get() + 1);
    consumer.execute(&Ctx { meta_ptr: MetaNil, _f: PhantomData });
    hook.execute(&Ctx { meta_ptr: MetaRef(&block), _f: PhantomData });
    assert_eq!(hook.observed.get(), 2, "hook read updated pass_count = 2");

    println!(
        "WORKS: engine-owned meta block (Cell, not Copy, not a Store) written by the engine; \
         OnMeta Ctx reaches it via meta::<T>() accessor; consumer Ctx (MetaNil) lacks the \
         accessor (compile-time MetaAccess enforcement). observed={}",
        hook.observed.get()
    );
}

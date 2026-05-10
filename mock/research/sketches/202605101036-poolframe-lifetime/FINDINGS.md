# Findings: PoolFrame lifetime propagation

**Outcome:** WORKS.
**Date:** 2026-05-11
**Verified:** `cargo test` from `mock/research/sketches/202605101036-poolframe-lifetime/`. Three tests pass; lib compiles under `#![no_std]` + edition 2024.

## Hypothesis result

`Pin<&'frame PoolFrame>` propagates cleanly through Scheduler /
ExecutionPlan / per-worker entry. The lifetime stays contained inside the
engine boundary even when a WU's Ctx exposes a borrow into PoolFrame, **provided
the trait uses a Generic Associated Type (GAT) for Ctx**.

## What was validated

### Mod 1: scheduler_frame_only

```rust
struct Scheduler<'frame, Cfg: RunCfg, E: Executor, const MAX_CORES: usize, const MAX_PHASES: usize> {
    plan: ExecutionPlan<'frame, MAX_CORES, MAX_PHASES>,
    executor: E,
    _cfg: PhantomData<Cfg>,
}
```

Compiles. Dispatch via `self.executor.run(self.plan.frame, core_id)` passes the
`Pin<&'frame PoolFrame>` cleanly to the executor's monomorphised entry. No
friction from the lifetime + multiple const generics.

### Mod 2: with_workunit_no_borrow

A WU with `type Ctx = MyCtx;` (non-borrowing). The `impl WorkUnit for MyWu`
block is `<>`-clean — no `'frame` lifetime required. The Scheduler still holds
`Pin<&'frame PoolFrame>`. Lifetime is fully contained at the engine boundary.

```rust
impl WorkUnit for MyWu {
    type Read = Empty;
    type Write = Empty;
    type Ctx = MyCtx;
    fn execute(&self, ctx: &Self::Ctx) { ... }
}
```

### Mod 3 (THE risky case): with_workunit_borrowed_ctx

A WU whose Ctx exposes a `&'frame AtomicUsize` slot from PoolFrame's
`progress_slots[]` array. This is the AdaptWu-shaped case where the WU genuinely
needs to read from PoolFrame at execute time.

The trick is **GAT on Ctx**:

```rust
pub trait WorkUnit {
    type Read: AccessSet;
    type Write: AccessSet;
    type Ctx<'frame>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>);
}
```

Consumer impl is still `<>`-clean:

```rust
impl WorkUnit for AdaptWu {
    type Read = Empty;
    type Write = Empty;
    type Ctx<'frame> = ProgressView<'frame>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) { ... }
}
```

Dispatch site uses HRTB to bridge:

```rust
fn dispatch_adapt<'frame, Wu>(wu: &Wu, frame: Pin<&'frame PoolFrame<...>>, core_id: usize)
where
    Wu: for<'a> WorkUnit<Ctx<'a> = ProgressView<'a>>,
{
    let ctx = ProgressView { slot: &frame.progress_slots[core_id] };
    wu.execute(&ctx);
}
```

**Key win:** the consumer's `impl WorkUnit for AdaptWu` line does NOT carry
`'frame`. The lifetime is hidden inside the GAT. From the consumer's vantage,
they declare a Ctx that's parametric over the engine-supplied lifetime; the
engine instantiates it at dispatch time with the live `'frame`.

## Implications for Topic 6 axis C

Lock the typed `Pin<&'frame PoolFrame>` shape. No fallback to raw pointer
needed. The src CL covering hilavitkutin-api should:

1. Promote `WorkUnit::Ctx` from a concrete associated type to a GAT
   (`type Ctx<'frame>`). This is a breaking change on the api crate, but the
   no-legacy-shims-pre-1.0 rule covers the migration: existing WU impls update
   to the GAT shape (most will be `type Ctx<'_> = (...)` since they don't
   borrow PoolFrame, or `type Ctx<'frame> = ProviderTuple<'frame>` for those
   that do via providers).

2. Thread `<'frame>` through `Scheduler`, `ExecutionPlan`, and the per-worker
   executor entry.

3. PoolFrame itself goes in the engine crate; it never reaches consumers.

## Implications for Topic 6 axis H

Sketch budget consumed: one focused session. Outcome was WORKS on the first
session. No follow-up sketch needed for the lifetime path.

## What this sketch does NOT cover (deferred to src CL)

- The full Executor trait method set (only `run` shown here; real Executor
  exposes per-phase-barrier hooks, parking primitive, etc.).
- futex/__ulock/WaitOnAddress FFI shape.
- Interaction with the existing `Context<P>` provider tuple from hilavitkutin-ctx
  — needs verification that `Context<P>` carries `'frame` cleanly. Sketch
  showed GAT works; the next-layer check is whether existing `HasResourceProvider`
  / `HasColumnProvider` accessors can fit the GAT shape without redesign.
- CoreClass detection per-OS (separate bench/sketch).

## Cross-references

- Topic file: `mock/design_rounds/202605101036_topic.thread-pool.md` axes C, H.
- Workspace rule: `~/Dev/clause-dev/.claude/rules/hilavitkutin-workunit-mental-model.md` — confirms
  that scheduler-owned data plane is the right pattern; this sketch shows the
  lifetime story without breaking the "WUs declare AccessSet, engine dispatches"
  contract.
- Workspace rule: `~/Dev/clause-dev/.claude/rules/no-legacy-shims-pre-1.0.md` — applies to the
  Ctx-to-GAT migration: delete the old shape, no deprecated alias.

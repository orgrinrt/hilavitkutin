# Sketch: PoolFrame lifetime propagation

**Date:** 2026-05-11
**Topic:** Round 202605101036, topic 6 (thread pool), axis C + axis H
**Status:** in-flight

## Hypothesis

The Topic 6 axis-C decision is `Pin<&'frame PoolFrame>` propagated through:

- `Scheduler<'frame, Cfg: RunCfg, E: Executor>`
- `ExecutionPlan<'frame, MAX_*...>`
- Per-worker entry function (the codegen output from Topic 4)

Without forcing `'frame` to bleed into:

- `WorkUnit::Ctx` associated type
- `AccessSet` trait bounds
- Consumer `impl WorkUnit` declarations

The framing: PoolFrame is engine-internal data plane (per-pool cache lines per
Topic 3 amendment M11). It holds AtomicBool shutdown, [AtomicU32; MAX_PHASES]
predicted_wait_ns, AtomicU32 phase_arrived, and the per-core CoreProgram
pointer table. WUs interact with the engine via `Context<P>` parameterised by
their Provider tuple, never by PoolFrame directly. The lifetime should be
contained inside the engine boundary.

The risk: any place where a WU-visible type holds a borrow into PoolFrame
forces 'frame propagation to that consumer surface. The most likely friction
point is the AdaptArena pointer or the ProgressCounter slot: if Topic 3's
ProgressCounter sketch (already validated as `Pin<&'frame [AtomicUsize]>`-style
indirection) shows up in Context, the lifetime contagion is unavoidable.

## Test plan

Three nested experiments, each a separate `mod` in `src/lib.rs`:

1. **`mod scheduler_frame_only`** — minimal Scheduler holding `Pin<&'frame
   PoolFrame>` + ExecutionPlan. No WorkUnit involvement. Validates the
   lifetime propagates through Scheduler<'frame, Cfg, E> + ExecutionPlan<'frame,
   ...> without compile errors.

2. **`mod with_workunit_no_borrow`** — adds WorkUnit declarations whose Ctx
   does NOT contain any borrow related to PoolFrame. Validates 'frame stays
   contained at the engine boundary: WU impls remain `<>` (no lifetime
   parameter required from consumer).

3. **`mod with_workunit_borrowed_ctx`** — adds a WU whose Ctx exposes a
   `*const PoolFrameProgressSlot`-like accessor (the kind of thing AdaptWu
   would need). Tests whether the borrow can be opaqued (cast to *const at the
   boundary, exposed as `&'_` only briefly during execute()) without forcing
   'frame into the WU impl signature.

## Success criteria

**WORKS** (lock the axis-C decision):
- All three mods compile under nightly with `feature(adt_const_params)` and the
  other features the round uses.
- Mod 3's WU impl has NO `'frame` lifetime parameter in its `impl WorkUnit`
  declaration. The boundary is opaque to the consumer.

**FAILS WITH ...** (fall back to raw pointer):
- Mod 3 forces `<'frame>` to thread through the WU impl. Document the specific
  bound that demands it. The fallback is `*const PoolFrame` with a documented
  safety contract; the WU impl stays `<>`-clean at the cost of one unsafe
  boundary at the executor entry point.

**INCONCLUSIVE — needs deeper investigation**:
- Compiles but trait-solver behaviour produces spurious errors at consumer
  sites. Document the rustc behaviour; escalate to a follow-up sketch with a
  larger AccessSet integration.

## What this sketch does NOT validate

- The actual Executor trait method set (deferred to src CL).
- futex/__ulock/WaitOnAddress FFI shape (separate sketch if needed).
- CoreClass detection per-OS (deferred to follow-up bench).

## Next step after WORKS

Topic 6 axis C + H lock. The round's src CL adopts `Pin<&'frame PoolFrame>`
across Scheduler, ExecutionPlan, worker entry. No further sketch needed for
the lifetime path.

## Next step after FAILS

1. Capture the exact bound in `FINDINGS.md` with file:line + the rustc error.
2. Update Topic 6 axis C decision to the raw-pointer fallback.
3. Update `mock/design_rounds/202605101036_topic.thread-pool.md` axis C to
   "Fallback decided: raw pointer with safety contract; reason captured in
   sketch FINDINGS.md".

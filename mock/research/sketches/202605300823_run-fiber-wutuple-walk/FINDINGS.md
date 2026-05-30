# Findings: run_fiber WuTuple walk with per-WU EngineCtx (C2 / #340)

**Outcome: WORKS** (nightly-2026-05-28, rustc 1.98.0-nightly 57d06900f). Compiled clean, ran, assert passed.

## Hypothesis

A monomorphic recursive walk over a fiber's typed WU sequence can construct EACH WU's own projected `EngineCtx` from a shared arena and call `wu.execute(&ctx)`, type-checking on the pinned nightly with no codegen or LLVM.

The single-WU construct-and-execute is already shipping (`tests/engine_ctx.rs::context_drives_wu_execute`), but there the WU is concrete, so its `Ctx<'_>` resolves directly. The open question was the RECURSIVE walk over a HETEROGENEOUS sequence, where each abstract `W` has a distinct Read set, distinct projection index, distinct bundle type, and therefore a distinct `Ctx<'frame>` GAT instantiation.

## Result

The walk compiles and runs. Two WUs reading distinct resources (`RA` at arena index `Here`, `RB` at `There<Here>`) ran in order, each resolving its own resource: `OBSERVED == [10, 20]`.

The load-bearing bound the solver accepted and used:

```rust
for<'f> W: WorkUnit<Ctx<'f> = EngineCtx<'f, W::Read, <A as Project<W::Read, RIdx>>::Out>>
```

This is an HRTB associated-type-equality bound on a GAT whose right side is a projected associated type (the resource bundle), which is lifetime-independent. rustc resolves it against each concrete WU's own `type Ctx<'frame> = EngineCtx<'frame, Read, Bundle>` declaration.

The per-WU projection index `RIdx` is constrained by carrying a parallel `Witnesses` list as a `RunFiber<A, Witnesses>` trait parameter, mirroring the engine's `BundleProject<Stores, Witnesses, ...>`. That dodges E0207 (unconstrained type parameter) and the nested index list infers at the entry call `run_fiber(&fiber, &arena)` with no turbofish.

## What this unblocks

C2 slice 1: a `RunFiber<A, Witnesses>` walk trait plus an entry fn, reusing the shipped `Project` / `Selector` / `EngineCtx` directly. The engine source slice introduces only the walk trait and entry fn; the projection machinery already exists. The walk is pure Rust generics, no codegen, no LLVM, matching the architect's framing of the C2 dispatch core.

## What this does NOT answer

WU value sourcing. This sketch carries WU values in the walk list to isolate the trait-solver question. In the engine the WU values come from the registered bundle, which `build()` currently erases (the scheduler retains `Vals`, not `Wus`). Capturing the WU sequence and sourcing per-WU values is the separate slice-2 question.

Columns are omitted (resource-only). Column projection adds more `Project` / `Selector` recursion of the same shape, not a new trait-solver question; column buffer allocation (B2b) is independent.

## Retention

Stays committed per `cl-claim-sketch-discipline.md`. The C2 slice-1 doc CL references this file.

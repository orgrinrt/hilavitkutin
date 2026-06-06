# Findings: column-capable inline fiber walk (Phase D / #340)

**Outcome: WORKS** (nightly-2026-05-28, rustc 1.98.0-nightly cced03bfd). Compiled and ran against the real engine crates; the three-unit column chain `In -> A -> B -> C` dispatched in order through the inline walk and `Cv[i] == stage3(stage2(stage1(i)))` for all 64 records.

## Hypothesis

The resource-only `RunFiber` walk (shipped in `dispatch/fiber_walk.rs`) can be extended to column-reading and column-writing WorkUnits without erasing to a function pointer, so a fiber's unit sequence monomorphises into one inlined body. That body is the devirtualization half of Phase D Shape B: no stored fn pointer means LLVM sees one straight-line per-fiber function (the risk-R2 fix), and it is the precondition the fusion half (scratch-backed internal columns) needs for dead-store elimination across unit boundaries.

This is the architect read's named first de-risking question. It is NOT already answered by `CollectFiber` (`dispatch/fiber_codegen.rs`), which carries the same four-witness bound shape but writes a `fiber_shim` function pointer into a slot array and resolves the 7-param `EngineCtx` GAT equality inside the concrete-per-W shim, never in the recursive trait impl. The inline walk forces that GAT equality to normalize WITHIN the recursive impl.

## Result

The inline column-capable walk type-checks and runs for a heterogeneous three-deep `WuCons` with distinct column Read/Write sets. No overflow, no recursion-limit, no normalization failure under the pinned nightly. The devirtualization half of Shape B is feasible.

One precise constraint surfaced, with two fixes. The GAT-equality tie can restate a Ctx param as an unresolved projection (for example `<A as ColProject<W::Read, RCIdx>>::Out`) only where that param genuinely varies and the solver can pin `A` independently. Where the WorkUnit declares a param CONCRETELY (the 7th param, the accumulator bundle, defaults to `AccPtrNil`), restating it as `<A as AccumProject<..>>::Out` deadlocks inference at a free entry call: normalizing the projection needs the witness index, and the witness is what the solver is trying to infer from this very bound (an E0271 type mismatch, projection-versus-concrete, with `A` still a variable).

The two fixes, both available to the engine:

1. Pin the concrete param with an `Out = AccPtrNil` associated-type-equality bound and write `AccPtrNil` concretely in the tie. Used in this sketch; faithful because the column WorkUnits write no accumulator. The genuinely-varying column projections (read and write columns) stay restated as projections and normalize fine with `A` arg-inferred.
2. Drive the walk from a context where `A` is pinned by `Self`, which is exactly `Scheduler::run<Witnesses>` where the shipped `CollectFiber` already resolves the same tie (including a non-empty accumulator bundle). The witness-inference deadlock does not arise there because `A = <Vals as BindingsFor>::Bindings` is fixed by `Self` before `Witnesses` is inferred.

## Devirtualization confirmed (release ASM check)

Built in release (fat LTO, codegen-units=1, the Cargo.toml profile) and disassembled. No `fiber_shim`, `run_fiber`, or `RunFiberCol` dispatch symbol survives in the binary: the column-capable inline walk and the per-unit dispatch inlined completely into their caller, leaving no function-pointer dispatch site, and the main dispatch region carries zero indirect branches (`blr`). That is devirtualization: the monomorphised cons-list recursion plus `#[inline]` collapses to straight-line static code under fat LTO. (The whole-binary `blr` count of 170 is std formatting, panic, and `Vec` machinery from the sketch's own harness, not the dispatch.) This confirms the load-bearing premise of Shape B's devirtualization half: removing the stored function pointer in favour of the inline cons-list walk produces a devirtualised per-fiber body.

## What this unblocks and what it constrains for Phase D

The column-capable inline fiber walk is the devirtualized per-fiber body Shape B needs. It slots into `Scheduler::run<Witnesses>` in place of the `CollectFiber` + `fiber_shim` fn-pointer dispatch (fix path 2), where the full projected tie resolves because `A` is `Self`-pinned. That is also where the fusion half lands next: the read/write column projections this walk already resolves are where scratch-backed internal columns will be substituted for arena columns.

Follow-on probe not done here: a non-empty accumulator bundle restated inline. The bench already runs accumulator WorkUnits through `CollectFiber` + `run<Witnesses>`, so that context is proven; the inline-with-non-empty-accumulator variant is a small delta to confirm when the engine slice wires the walk into `run<Witnesses>`.

## Aside: cross-repo drift caught while setting up the sketch

A fresh dependency resolution (no lock) pulled arvo `dev` HEAD, whose `arvo_graph::waist_detect<C: Capacity, B>` now takes two generic arguments (the row-width parameter from arvo #663). The engine call site `plan/steps.rs:272` still passes one (`waist_detect::<D::Units>`), so the engine fails to compile against current arvo `dev`. The engine workspace lock pins the older one-argument arvo (`549628815f5b4e6df34c0eb77c731377b28bd7b5`), so `cargo test -p hilavitkutin` passes today, but the #663 transitive consumer update (bump arvo, pass the second `waist_detect` argument) is pending. Filed as a task. The sketch pins the same compatible arvo rev via a copied lock; the drift is unrelated to the dispatch trait-solver question.

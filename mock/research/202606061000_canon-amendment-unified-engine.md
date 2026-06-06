# CANON AMENDMENT: the engine is unified; core count is configuration, not a code fork

**Status: CANON. Authority: op (O. R. Toimela), 2026-06-06, explicit ruling.** This is not a proposal or a research finding to be weighed; it is an authoritative addition to the canonical hilavitkutin design, made by op under op's authority. It has the same standing as the consolidation spec (`mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`). Where any intermediate artifact (build-plan memo, audit, design rounds, DESIGN.md.tmpl, agent memory) conflicts with this, this wins.

## The ruling

There is no separate "single-core" or "single-threaded" engine, run, or dispatch path. The engine is one thing. You configure the runner with a core count; it then adapts by its usual rules. The same primitives and the same code paths run at 1 core, 2 cores, or 7 cores. The whole plan/algorithm pipeline always computes the single best, most optimal sequence and parallelised per-core programs it can from all statically-available data, within the configured core count. At 1 core that best sequence is serial, as a natural consequence of the same algorithms (the plan assigns every fiber to core 0 in the optimal order; phase sync points degenerate to one arriver; convergence is trivial; the pool has one worker). It is always distinctly the best, most optimal sequence computable from the static data, for whatever core count is configured.

Single-core mode is therefore never special-cased against multi-core mode in code. Both use the identical computations, calculations, and the identical per-core-program dispatch; both strive for the best possible sequence and parallelised core programs within their given config. "Sequential" is a per-phase strategy the plan may select (consolidation spec `:1916-1956`), not a separate engine.

## What I found (the verification op asked for)

op believed the canonical design already encodes this ("I did the canonical too"). Cross-checked against the consolidation spec, confirmed: the spec never special-cases single-core as a code path.

- "sequential, 1.00x, single core, no overhead" (`:1956`) is one of the per-phase STRATEGY modes (MAX_FUSE / BALANCED / MAX_SPLIT / sequential, `:1916-1956`) the plan selects. A strategy, not an engine.
- "single-core = trivially ordered" (`:605`) is a statement about ordering being trivial at one core, not a separate path.
- Pool size = `physical_core_count` (`:1799`); thread count = `min(physical_cores, parallelisable ...)` (`:1826`); morsel-to-core affinity is an adaptive plan-stage parameter (R6, `:2442-2446`). Core count is a parameter the unified machinery adapts to.
- The compiled per-core program (domain 17, `:1596-1613`) is THE dispatch. With one core there is one program; its phase sync points have one arriver; the pool has one worker. Nothing forks.

So the unified model is already canonical; this amendment makes op's authoritative ruling explicit and removes the ambiguity that let intermediate artifacts drift toward a special-cased single-core path.

## What this corrects (intermediate drift, now superseded)

- The build-plan memo's "single-core-correct first ... this single-core path is the correctness oracle" (`202605282100:52`), read as a distinct single-core build, drifted toward special-casing. Re-read: it is the same unified engine validated/benched at one core first.
- The `RunFiberCol` cons-list-walk framed as a "single-core dispatch core, distinct from the parallel-path shape" (`DESIGN.md.tmpl:143-145` and the 2026-06 dispatch sketches' single-core framing) is exactly the special-casing this ruling forbids. The unified per-core program (per-fiber devirtualised LOCAL `&[WuFn]` slices, the canonical domain-17 dispatch) is the only dispatch shape. The cons-list-walk sketches' proven machinery (EngineCtx projection, the 7-param GAT tie, accumulator-inline, devirtualisation of monomorphised trait dispatch) is reusable as an internal detail of how a fiber's WU sequence is walked, but not as a separate single-core dispatch shape.
- The two-gate mandate (single-core to completion, then parallel) is re-read as validation/bench milestones of the one engine: Gate 1 = the unified engine runs correctly and benches at parity configured to one core (real plan, real per-core-program dispatch, pool with one worker, strategy=sequential, adapt; the multi-core scaffolding present but degenerate); Gate 2 = the same engine benches well at N cores (barriers, convergence, multi-worker distribution now doing real work). No code is special-cased by core count.

## Consequence for the dispatch design

The easy escape hatch is gone: there is no "registration-order cons-list walk that works for single core" to ship first. The real unified mechanism is unavoidable, the runtime-plan-to-compile-time per-core-program bridge ("build time = the type system": the fiber/phase/core topology is a function of the WU `AccessSet` associated types, which only the trait solver sees), for all core counts. The per-fiber `FiberShape` typestate is its likely realization and the highest-risk unproven step the engine-completion roadmap must sketch-prove.

## Formal fold-in

The consolidation spec is a locked, archived design round; it cannot be edited inline (the canon-amendment-mechanism gap tracked as #667). This file is the authoritative canon record until that mechanism (or a dedicated canon design round) folds the ruling into the spec text proper. Treat this file as canon now.

## See also

`mock/research/202606060900_engine-completion-strategic-synthesis.md` (the strategic synthesis, section 2.35), workspace rule `canonical-design-outranks-intermediate-rounds.md`, consolidation spec domain 17 + R6.

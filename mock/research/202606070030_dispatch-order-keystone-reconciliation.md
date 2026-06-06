**Date:** 2026-06-07
**Phase:** research / reconciliation (resolves the Phase D dispatch-order keystone fork)
**Scope:** hilavitkutin engine, single-core dispatch (HILA-RUNTIME-C2 #340, GATE-1 #661/#664)
**Source:** canonical consolidation spec (R6 line 2435-2446, plan-time pipeline line 1261, Step 5 line 1328, domain 17 line 1564-1617); proven sketches 202606052130 (rcm-order-into-static-body), 202606061400 (D1b type-level partition), 202606061500 (D1a plan-fed schedule-mega), 202606060500 (approach E morsel-outer); dual-agent verdicts (neutral domain expert + op-persona) on the dispatch-order fork.

# The fork

How does `Scheduler::run` dispatch in the plan's order while devirtualising? The shipped body walks a runtime permutation (`topo_order[k]`) through a stored `FiberSlot` fn pointer (the 12.6x indirect anti-pattern). The inline `RunFiber` walk devirtualises but walks a compile-time cons-list in structural order. Three candidate resolutions surfaced: (A) registration-order-as-dispatch, (B) runtime-plan-permutation, (C) compile-time order.

# Resolution (converges across canonical spec + all proven sketches + both agents)

The dispatch order is a **compile-time fact carried by the type-level carrier structure** (the `WuCons` / `FiberCons` cons-list as built), NOT a runtime permutation and NOT registration-order-as-a-contract. The plan supplies **runtime params** (morsel size, record count), per R6's "adaptive at plan time". This is the D1a keystone-bridge finding verbatim.

Both A and B are rejected:
- B (runtime permutation into dispatch) reintroduces the shim indirection. Rejected by D1a and both agents.
- A as a permanent contract (consumer must register in topo order) is the caricature. Rejected by op-persona.

## Two spec facts that were being conflated

1. R6 (line 2435): "The WU set, DAG structure, fiber/trunk/phase topology, and monomorphised dispatch functions are all fixed at build time. This is what enables LLVM devirtualisation." This is about **composition** being statically determined from the compile-time-known WU set.
2. Line 1261: "The entire plan-time pipeline runs once at schedule construction (or when the WU set changes). Zero runtime cost." The topology **computation** (access matrix, topo sort, RCM, fiber grouping) runs once at plan time = schedule construction (runtime-once), with zero *per-frame* cost.

These coexist: the topology is deterministic from the compile-time WU set (so it is "fixed at build time" in that it never changes frame-to-frame and is fully determined by types), and the engine computes it once at plan time rather than in the trait solver. The op-persona verdict over-read R6 to mean "the RCM permutation must be a const-eval value", missing line 1261; that over-read is the intermediate-artifact drift `canonical-design-outranks-intermediate-rounds.md` warns about. Corrected here against the canonical source directly.

## const-eval RCM is NOT a prerequisite

The op-persona guardrail demanded a const-eval-RCM-over-access-matrix sketch before locking, because it (correctly) insisted the dispatch order be a compile-time fact. But the proven sketches already make the order a compile-time fact by a different mechanism: the **carrier structure**. 202606052130 OUTCOME: the inline walk is ORDER-AGNOSTIC, walks the list as built; any statically-known order (including an RCM permutation) devirtualises identically. So compile-time order is achieved by building the carrier in the right order, not by const-eval arithmetic. The flattener (`codegen_fiber`) emits the ordered carrier; whether it reorders via a proc-macro (imperative Rust at macro time) or const-eval is a future codegen-mechanism choice, not an increment-1 blocker. The const-eval RCM sketch is therefore superseded, not failed.

# Single-core scope (the GATE-1 increment)

The perf gate (#664, `mock/benches/engine_vs_std`) has three workloads: `element_wise` (4-stage RAW chain, single fiber, fusion target), `branching` (diamond DAG), `accumulator` (transform + append). On a **single core** all three are flat single-sequence walks:

- A diamond dispatches as one flat RCM-ordered walk (202606052130 proved exactly BranchY->BranchX->JoinZ as one flat `WuCons`). Branching needs no nested carrier on one core.
- The whole per-core program is one flat `WuCons` walked **morsel-outer** (Approach E schedule-mega, proven D1a + 202606060500): the whole sequence runs over one morsel window before the next, keeping each stage's morsel-sized column window cache-resident. This whole-program morsel-outer is what ENABLES within-fiber fusion (increments 2+3: classify_columns + scratch-backed internal columns).
- An accumulator that is appended-then-read crosses a cross-record phase boundary, so it stays unit-outer (each unit completes its record range before the next), the cross-record-safe form (also the record-less-frame path). A genuine reduction-then-read is a phase split (D1c, proven 202606062000), a later increment.

Fiber *grouping* into a nested `FiberCons` carrier matters for **multi-core** trunk-to-core assignment (Gate-2, #662) and is not a single-core dispatch-structure concern. The full type-level grouping fold is infeasible on the pinned nightly (D1b Tier 3 fails, needs forbidden `specialization`); the hybrid is plan-computed grouping + flattener-emitted carrier, and the flattener is `codegen_fiber` (a later increment in #340).

# Increment 1 (revised, honest)

Replace the `FiberSlot` shim walk in `Scheduler::run` with the inline `RunFiber` walk over the compile-time `WuCons` carrier (`self.wu_values`, registration order). Whole-program morsel-outer for the common case; unit-outer when an accumulator is present (cross-record safety) and for the record-less frame. The plan's `topo_order` is used to **validate** that registration order is topo-valid (every unit after its deps), erroring honestly if not (surfacing the not-yet-built flattener-reorder), and to supply runtime params; it is no longer a runtime dispatch permutation. Delete `CollectFiber` / `FiberSlot` / `noop_fiber_shim`. The within-level RCM reorder (the benched ~2% refinement) and the nested multi-core carrier defer to `codegen_fiber`.

This is op-persona's explicitly-acceptable intermediate ("shim dies + order is compile-time, RCM-within-depth / per-core slicing land in a scoped follow-on"), not the rejected caricature. The design contract (consumer registers WUs in any order) is preserved as the design; the intermediate validates instead of reorders, and the validation is honest (it errors rather than silently mis-dispatching).

# CORRECTION (2026-06-07, after attempting the rewire): the carrier-order assumption was wrong

Implementing the flat single-core walk surfaced a wall this note's first pass missed. The WU carrier (`wu_values`) is built by the builder via PREPEND, so the cons-list is in REVERSE registration order. For the basic `column_dispatch` test (register Column, ProducerWu, ConsumerWu), the carrier is `[ConsumerWu, ProducerWu]`, which is ANTI-topological: a flat carrier-order walk runs the consumer first, reading uninitialised records. The test (`tests/column_dispatch.rs:156-161`) documents exactly this. The flat walk produced wrong output (the consumer's reads, not `[0,10,20,30]`).

So the plan's `topo_order` is LOAD-BEARING FOR CORRECTNESS, not merely the RCM ~2% refinement. The claim "single-core accepts registration order as the carrier (validated topo-valid)" is false in the common case: prepend makes the carrier order anti-topological, and the consumer registering in dependency order does NOT help (prepend reverses it). Requiring reverse-topo registration is a worse caricature than requiring topo registration.

The genuine mechanism question, unresolved: devirtualised dispatch in the plan's topo order, where (a) the cons-list carrier is anti-topological by construction and cannot be runtime-reordered (heterogeneous types); (b) the type-level grouping/sort fold needs forbidden `specialization` (D1b Tier 3); (c) a proc-macro flattener cannot resolve `<W as WorkUnit>::Write` to compute dependency order (proc-macros see tokens, not resolved AccessSets); (d) the stored-fn-ptr shim is the 12.6x anti-pattern. The unexamined canonical candidate is Approach A ("local `&[WuFn]` slice with known values" devirtualises; struct-field arrays do not, 12.6x): build a stack-local fn-pointer array in topo order inside `run` and walk it. Whether a local array with cons-list-known contents but a runtime-permuted index devirtualises (LLVM switch/jump-table) versus needing static order is a bench + design question. This is the fork the dual-agent consensus + spec cross-check must resolve before the rewire can land. The 2130 sketch flagged this exact gap ("WHAT THIS DOES NOT SETTLE: how the per-fiber cons-list is ASSEMBLED in RCM-reordered topo order"); it is load-bearing, not deferrable.

The working-tree rewire (FiberDriver A-pin + test rehab + bound swap are correct; the `run` body flat walk is incorrect) is left uncommitted pending the resolution. Committed clean state: 137365a (research + CL only).

# Dual-agent consensus (2026-06-07, neutral domain expert + op-persona)

Both agents converged (verdicts recorded in the session transcript):

1. The carrier (`WuCons`) must be assembled in TOPO ORDER AT BUILD TIME by the flattener (`codegen_fiber`), from the access matrix. The static-order carrier walk then devirtualises (proven 2130/D1a/D1b); the walk is trivial once the carrier is ordered.
2. Approach A with a runtime-permuted local `&[WuFn]` slice does NOT devirtualise: the "known values" devirt condition requires the element ORDER be statically known, not just the contents. A runtime permutation index fails it. Rejected as a caricature.
3. The stored-fn-ptr shim is the documented 12.6x anti-pattern. Rejected.
4. The "single-core can skip the flattener / accept registration order" framing (this note's first pass) is DRIFT. `topo_order` is load-bearing for correctness on the trivial two-WU case (prepend makes the carrier anti-topological). The flattener is NOT deferrable; it is the keystone (`codegen_fiber`, task #340).
5. `codegen_fiber` is an imperative codegen step operating on RESOLVED types (it can see `<W as WorkUnit>::Read/Write`), so it is neither a token-level proc-macro (can't see AccessSets) nor a `specialization`-needing trait-solver fold (forbidden). The builder prepend stays; the plan stays; only the carrier-assembly + `run` change.
6. The dispatch-order mechanism is NOT a bench question (Approach A is wrong by design, not measurement). What IS bench-decided is downstream: within-fiber fusion shape and the domain-17 Approach D-vs-E (<10K / >10K) selection.

op-persona's sharpest correction: the wall ("carrier is anti-topological, can't reorder") is an artifact of building the carrier in registration order then trying to fix it. The carrier should be built in topo order in the first place. Reframe from "how do I reorder" to "where does the carrier get assembled topo-ordered" (answer: `codegen_fiber`).

# The concrete HOW to prove next (the codegen_fiber mechanism)

The consensus settles WHAT (flattener emits topo-ordered carrier, resolved-types, build-time, no specialization, no proc-macro, no runtime reorder) but the concrete Rust mechanism needs a proving sketch. The leading candidate, not ruled out by any constraint:

CONST-DRIVEN CARRIER ORDERING. (1) Each `AccessSet` exposes a const access mask as an associated constant (const-derivable from types: store IDs are const, the mask is a const fold over the cons-list, no specialization, no proc-macro). (2) A `const fn` computes `topo_order` (a `[usize; N]` permutation) from the const masks at compile time. (3) A const-generic `Nth<const K: usize>` accessor indexes the heterogeneous carrier at COMPILE-TIME position K (const indexing of a cons-list works where runtime indexing does not, because K is a type-level const). (4) A recursive dispatch carrier walks `topo_order` positions, dispatching `Nth<{topo_order[k]}>` at each const k. All indices are compile-time, so the dispatch is static-order and devirtualises (per 2130).

The risks to prove on the pinned nightly: (a) does a `const fn` over const access masks produce the permutation array, (b) does the const-generic `Nth<K>` heterogeneous accessor compile and devirtualise, (c) does the const-unrolled dispatch over a const `[usize; N]` topo array resolve each position's type at compile time without `generic_const_exprs`-extreme machinery. This is the next sketch (`mock/research/sketches/`), per the chart-the-path discipline: a step that cannot be done as the roadmap assumed gets a proving sketch before implementation.

# Working-tree disposition

The uncommitted working-tree rewire (FiberDriver A-pin + test rehab + `WuVals: RunFiber` bound swap + flat `run` walk + `fiber_codegen.rs` deletion) is the RIGHT end-direction for `run` (inline walk over the carrier), but premature: it walks the carrier in anti-topological registration order, so `column_dispatch` fails until the carrier is assembled topo-ordered by `codegen_fiber`. Held uncommitted pending the const-driven-carrier sketch. Committed clean state: 137365a (research + CL only).

# Why the original "no third dispatch" reasoning was SUPERSEDED

Tonight's special rule resolves blocked design calls via dual-agent consensus then cross-check against the canonical spec. Both agents already ran on this exact fork (both rejected A and B; both agreed order must be compile-time). The canonical spec was then read directly (R6 + line 1261 + Step 5 + domain 17) and confirms the proven-sketch resolution while correcting the op-persona R6 over-read. The new determinations this turn (flat single-core carrier, no flattener for single-core, const-eval RCM superseded) are readings of already-proven committed sketches, not a new fork. A reviewer can redirect with a follow-up PR; the redirect cost is small. Recorded here as the audit trail.

# UPDATE 2026-06-06: index-space divergence scopes the increment; full devirt is #669

Wiring `carrier_order_dyn` into `Scheduler::run` surfaced a divergence the earlier
flat-walk plan missed. `run` is NOT a flat whole-program walk: the canonical Gate-1
dispatch is the per-core program with per-fiber LOCAL slices and per-fiber morsel
locality (spec domain 17 :1596-1613; locked round 202606051500; test
`fiber_structured_dispatch`). A flat whole-program morsel-outer walk drops that
per-fiber locality and is the drift, not the test. The flat rewire was reverted.

The two axes are orthogonal (dual-agent consensus, neutral `feature-dev:code-architect`
+ op-persona, both read domain 17 + the roadmap D1a/D1b directly):

- dispatch ORDER = compile-time-foldable, type-derived (`carrier_order_dyn`). PROVEN
  devirt (sketch 202606071400, zero `blr`).
- fiber GROUPING + the per-fiber `morsel_local` bit = runtime plan computation
  (block-diagonalisation; D1b refuted type-level grouping). The codegen flattener
  EMITS the carrier from the runtime grouping (domain 17 :1566/:1732-1733).

The gap both agents glossed: `carrier_order_dyn`'s flat lowest-index Kahn order
INTERLEAVES independent components (e.g. chains A->B and C->D sort to [A,C,B,D], not
[A,B,C,D]), while the plan's `fiber_dispatch` descriptors slice the fiber-GROUPED
`topo_order`. So `carrier_order_dyn[start..len]` does not equal a fiber's units in
general; slicing it with the plan descriptors is incoherent (only coincidentally
aligned for small cases, which strict-by-design forbids shipping). Full per-fiber
devirt therefore needs a fiber-GROUPED compile-time carrier (the flattener that
emits a per-fiber carrier from the runtime grouping), which the roadmap marks as the
remaining keystone bridge.

Decision (owning the call): this increment lands the proven order MACHINERY
(`carrier_order_dyn` / `CarrierMasks` / `BindingsFor::Markers` / const `topo_order`,
all tested) and the dispatch-path cleanup (delete the superseded `RunFiber` walk that
`CollectFiber` replaced). `Scheduler::run` keeps its canonical per-fiber walk sourcing
the order from the plan's `topo_order` field (correct, locality preserved, all tests
green). The flattener bridge that routes `carrier_order_dyn` into a fiber-grouped
`run` dispatch (full #664 devirt + per-fiber locality together) is tracked as #669.

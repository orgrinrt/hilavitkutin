# Findings: RCM order into a statically-derivable dispatch body (Phase D / #340)

**Status: WORKS (body), FORK OPEN (order-assembly mechanism).** nightly-2026-05-28.

## What the probe ran

The branching diamond from `mock/benches/engine_vs_std/src/branching.rs` (BranchX:
In->Xv, BranchY: In->Yv, JoinZ: {Xv,Yv}->Zv) against the real engine crates,
driven through the column-capable inline `RunFiberCol` walk (proven for a linear
chain in `202606051601`). WUs registered in BranchX, BranchY, JoinZ order; the
walk cons-list hand-built in the DIFFERENT order BranchY, BranchX, JoinZ.

## What WORKS

1. **Multi-column read join.** JoinZ reads `Two<Xv, Yv>`; the read ColProject
   witness (`ColPtrCons<Xv, ColPtrCons<Yv, ColPtrNil>>`) resolved with no extra
   bound over the linear-chain sketch. Arg-inference found the four-witness list
   at the call site, no turbofish.
2. **Branching diamond dispatches correctly.** Zv[i] == join(branch_x(i),
   branch_y(i)) for all 64 records.
3. **Order-agnostic + devirtualized.** Built in non-registration order, the walk
   still ran correctly and the release (fat LTO, cgu=1) disassembly shows no
   surviving `run_fiber_col`/`RunFiberCol`/`fiber_shim` symbol and zero indirect
   calls in the dispatch region. The per-record loop auto-vectorized to `eor.16b`
   SIMD; the only two `blr` in `main` are the trailing `println!` stdout path.

Conclusion: the static-body half of the corrected single-core dispatch is
feasible for a branching DAG with a multi-column join, and it devirtualizes for
ANY statically-known order, including an RCM permutation of registration order.
The walk dispatches whatever static cons-list it is handed; it never slices or
reorders at a runtime boundary, so the hypothesis's "cannot reorder a
compile-time cons-list at runtime" wall does not block the walk itself.

## The remaining fork: where the RCM order comes from

The corrected design (MEMORY LATEST-56, consolidation spec L1331-1339 / L1403 /
L1534-1602 / L2437) requires the dispatch order to BE the RCM-reordered topo
order, and the topology to be fixed at build time (the flattener emits a
monomorphised function per fiber). The current engine instead computes the order
at RUNTIME: `derive_phase_dispatch_order` (scheduler/mod.rs:261) produces a
`topo_order` permutation of registration indices; `CollectFiber`
(dispatch/fiber_codegen.rs) records a `fiber_shim` FUNCTION POINTER per unit;
`run()` dispatches `slots[topo_order[step]]` through that stored pointer at a
runtime index. That runtime-indexed call through a stored fn is exactly the
12.6x devirt-failure path (spec L1539-1545).

So the body is proven, but feeding it the RCM order in a devirtualized shape
requires the order to be a COMPILE-TIME fact. The runtime plan-chain is not. The
three candidate mechanisms, re-evaluated against this probe:

1. **Compile-time topology (type-level / const-eval RCM over the access-set
   types), flattener emits per-fiber inline-walk bodies in RCM order.** Most
   design-faithful (matches "topology fixed at build time"). Risk: a full
   RCM/topo/fiber-grouping graph algorithm at the type or const-eval level leans
   on `generic_const_exprs`, which is WATCH/ICE-prone on the pin (#628 already
   tracks GCE migration concern). Needs its own feasibility sketch before it can
   be committed.
2. **Macro / build-time flattener emits the reordered per-fiber compositions.**
   The plan (topo/RCM/grouping) runs in a build step (proc-macro or a
   build-dep), emitting the statically-ordered inline-walk bodies as source the
   compiler then devirtualizes. Lower toolchain risk than (1). But hilavitkutin
   has no plan-flattening proc-macro today (only `hilavitkutin-extensions-macros`),
   and `hilavitkutin-build`'s charter (DESIGN.md.tmpl) is explicitly "LLVM flag
   emission / pragmas / rustc-wrapper ONLY, NOT plan codegen", so this either
   adds a new proc-macro surface or widens the build crate's charter.
3. **Constrain registration order to be a valid topo order, validate at build,
   walk the flat registration-order cons-list inline; treat the RCM within-depth
   reorder as a separate (benched) refinement.** Lowest risk, ships devirt now.
   But this is the direction MEMORY LATEST-56 explicitly REJECTED: it reasons
   "registration order is the dispatch order, RCM is deferred", which is the
   current-code punt, not the design. Recording it for completeness; it
   under-delivers on "RCM is the execution order".

Per op's standing ruling (MEMORY LATEST-56: the mechanism "doesn't matter HOW,
as long as it gets done as designed" and is an implementation + bench choice for
the agent, verified against the disasm_5check ASM gate, NOT a blocking op-fork),
this fork is the agent's to pick and sketch, not surface. Candidate 3 is out: it
is the rejected "registration is dispatch, RCM deferred" punt. The choice is
between (1) and (2), both of which keep RCM as the execution order.

Next: sketch candidate (1) first, because the proven `RunFiberCol` walk is
already type-recursion over a cons-list, so a position-permuted walk driven by a
type-level order witness (built from the same access-set facts the solver
already consumes for the four-witness lists) is the smallest delta from the
proven body and the most design-faithful ("topology fixed at build time"). If a
type-level reorder of a heterogeneous cons-list hits a GCE/solver wall on the
pin, that is a recorded finding that steers to (2), the macro/build flattener.
The keystone discipline (sketch the load-bearing structure question before the
SRC CL) governs: the assembly mechanism is sketched and devirt-verified before
the #340 DOC CL rewrite commits to it.

## Narrowing: only the type system sees the access sets

A proc-macro or a `build.rs` codegen step (the token-level forms of candidate 2)
cannot compute the fiber grouping or the RCM order: both are functions of each
WU's `Read`/`Write` `AccessSet`, which are associated types resolved by the type
system. A macro sees tokens, a build step sees source text; neither sees a
resolved associated type. So the topology computation that feeds the static
dispatch order is essentially forced into the type system. This is what the
spec's "monomorphised function per fiber" (L1566) and "topology fixed at build
time" (L2437) mean concretely: build time here is monomorphisation time, i.e.
the type system, not a pre-compile token pass.

This both narrows the fork and de-risks candidate 1: the assembly is a
type-level computation, and it need not be the GCE-heavy `generic_const_exprs`
form. The proven `RunFiberCol` walk is already trait-level cons-list recursion
(no const-eval); a grouping-and-reorder expressed as further trait-level
recursion over the access-set cons-lists stays on the same proven footing and
sidesteps GCE. The remaining genuinely-hard part is expressing a permutation (or
a block-grouping) of a heterogeneous cons-list at the trait level, which the
next sketch probes directly.

## Refined next-sketch target: type-level GROUPING, not permutation

Decomposing the assembly by granularity shrinks the hard part:

- Within a fiber, the order is the fiber's topo chain order, which is the
  registration order restricted to that fiber's units. No permutation of a
  heterogeneous cons-list is needed (that is the GCE-adjacent hard operation).
- Across fibers, the sequencing (including the RCM choice among equal-depth
  fibers) lives outside the per-record hot loop. Each fiber body is its own
  devirtualized SIMD body; calling N fiber bodies in a runtime-chosen order is N
  direct calls and does not reintroduce per-record indirection. So fiber
  sequencing can stay runtime and still honor "RCM is the execution order": RCM
  shapes the grouping and the fiber sequence, and within a fiber topo == RCM
  chain order.
- The irreducible hard core is therefore the type-level PARTITION of the flat
  registration-order `WuCons` into per-fiber typed sub-cons-lists, where the
  grouping (block-diagonalization of the access matrix: which units share stores
  transitively) is a function of the `AccessSet` types.

The next sketch probes that type-level grouping directly: can a trait-level
computation over the registered units' Read/Write access-set cons-lists produce
per-fiber typed sub-cons-lists (grouping by shared-store reachability), on the
pinned nightly, without GCE? If trait-level grouping is intractable, the
recorded fallback is whether the grouping stays a runtime fact while the
per-fiber bodies are still selected in a devirt-preserving way (a separate probe
shape). Either way the within-fiber static body and the runtime fiber sequencing
are settled by this sketch's order-agnostic result.

## Decisive simplification for single-core: Approach E (schedule-mega)

The consolidation spec's dispatch approaches (T6, L1551-1615) include
"E: schedule mega, all trunks in one fn, 0.97x", preferred for >10K records
because the call setup amortises over the morsel iterations. That is the shape
this sketch already proved: the diamond is the whole schedule walked as one
cons-list, devirtualized.

For the single-core GATE-1 engine (many records, so Approach E), this removes
the type-level fiber partition entirely. One monomorphised body walks the whole
flat cons-list; fiber and phase boundaries and morsel sizes become runtime
control flow and compile-time constants WITHIN that single body, which is
exactly the spec's "compiled per-core dispatch" (L1596-1613: a per-core fn
encoding phases + record ranges + per-fiber devirt local slices + morsel
constants + phase sync points). Step-8 grouping (greedy or the DP with the
cache-budget feasibility check) still runs at plan time to DECIDE morsel sizes
and materialisation points, but it parameterises the runtime control flow, not
the cons-list type. The per-fiber partition the earlier "type-level grouping"
framing worried about is a >1-core concern (trunks map to cores; on one core
the partition collapses to morsel/materialisation boundaries inside one body).

So the assembly problem shrinks from "type-level per-fiber partition" to "build
ONE flat cons-list in the plan's RCM-reordered topo order". And within-level RCM
among independent equal-depth units (the only part registration order does not
already give, assuming topo-valid registration) is perf-marginal on single core:
the independent units touch disjoint columns, so their relative order changes
only which disjoint column is written first within a morsel. Whether the RCM
within-level reorder beats validated-registration-order on the gate workloads is
therefore a BENCH question (disasm_5check + the #664 perf gate), not a structural
blocker, and not the rejected "defer RCM" punt: RCM is consumed by building the
cons-list in its order; the bench only measures whether the within-level degrees
of freedom move single-core time at all.

Chosen single-core direction to sketch next: Approach E. One schedule-mega inline
walk over the flat cons-list (proven here), with (a) a build-time validation that
registration order is topo-valid, (b) the plan's RCM-reordered topo order
realised as the cons-list order, and (c) morsel/phase/materialisation boundaries
as in-body runtime control flow. The next sketch builds the schedule-mega body
over a multi-fiber workload with in-body phase boundaries and benches the
RCM-reorder-vs-registration question. The type-level per-fiber partition is
deferred to the GATE-2 parallel path, where trunks genuinely map to cores. This
keeps RCM in the single-core execution order (not arena-only, not punted) while
not forcing a type-level partition the single-core path does not need.

## Side finding

The engine source does not compile against arvo `dev` HEAD: `waist_detect`
(steps.rs:272) is called with one generic arg but post-#663 arvo takes two
(`<C: Capacity, B>`). The mock workspace is pinned to the pre-#663 arvo rev
(549628), so it builds today; this sketch pins the same rev to match. The 2-arg
call-site fix plus the arvo bump is task #666 and is a hard prerequisite for any
work that resolves arvo at `dev` HEAD.

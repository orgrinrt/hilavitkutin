# Hypothesis: carrying the plan's RCM-ordered fiber grouping into a statically-derivable dispatch body (Phase D / #340)

**Status: HYPOTHESIS (probe not yet run).** Pinned nightly-2026-05-28.

## What this sketch resolves

The design-oracle correction (see workspace rule `design-is-the-oracle.md`, memory `feedback_design_is_the_oracle.md`, MEMORY LATEST-56) overturned the Shape Q/B framing. The corrected single-core dispatch, per the consolidation spec (`mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md` L1331-1339, L1403, L1534-1602):

- RCM produces a row reordering that **is the WU execution order** (not arena-layout-only), fed into `group_fibers` (step 8 walks WUs in RCM-reordered topo order). For wide DAGs with fan-out, RCM picks the order among equal-depth units.
- Dispatch is per-fiber **statically-derivable bodies** (the `RunFiber` column-capable walk, already devirt-proven in `202606051601`), each walking its fiber's units in that RCM order, composed into the schedule via inlining (the bench A/E shapes in `mock/research/imported-from-polka-dots/bench/{dispatch_optimization_test,full_schedule_dispatch_test}.rs`, 0.97-1.0x vs hand-fused).
- "Codegen" here means MIR to ASM (the statically-derivable body LLVM compiles), not Rust source emission. The mechanism that gets the body statically derivable (type structure, macro, const, or a `hilavitkutin-build` pass) is an implementation and bench choice, verified by the `disasm_5check` ASM gate, not pre-committed.

The devirt of a single fiber's column-capable walk is already proven. The unresolved, load-bearing question this sketch answers:

**Given the scheduler builder hands one flat `WuCons` in registration order, and the plan computes (at runtime) a fiber grouping plus an RCM execution order in which fibers can be NON-contiguous in registration order and within-depth units are reordered, how does each fiber's RCM-ordered unit sub-sequence reach a statically-derivable dispatch body that devirtualises?**

A compile-time-nested `WuCons` cannot be sliced or reordered at a runtime boundary. So either the fiber composition plus order must be a compile-time fact (a type-level or const function of the WorkUnit access-set types, since the access matrix that drives topo/RCM/grouping IS a compile-time fact about the registered types), or a macro / build pass must emit the per-fiber bodies in plan order. This is the spec's "topology fixed at build time."

## Test workload (the real one, not a toy)

The branching gate workload (`mock/benches/engine_vs_std/src/branching.rs`): a diamond.

- `BranchX`: `Read = One<Inv>`, `Write = One<Xv>`.
- `BranchY`: `Read = One<Inv>`, `Write = One<Yv>`.
- `JoinZ`: `Read = Two<Xv, Yv>`, `Write = One<Zv>`.

`BranchX` and `BranchY` are at the same topological depth (both read `Inv`, independent, neither reads the other's output). This is exactly the case where RCM determines the within-depth order and where the fiber partition is non-trivial: the diamond is multiple fibers with a join, not one linear chain. The element_wise workload (linear chain, one fiber) and the accumulator workload (one unit) are the cases the `202606051601` sketch and the shipped `run<Witnesses>` path already cover; the diamond is the one that tests the mechanism.

## Candidate mechanisms to probe (bench/feasibility choice, verified by disasm_5check)

1. **Compile-time fiber structure from the access-set types.** The fiber grouping plus RCM order is derived at compile time (const fn over the access matrix, or a type-level fold) so per-fiber `WuCons` sub-sequence TYPES are constructed in RCM order; each fiber dispatches via the proven column-capable `RunFiber` walk; the schedule composes them. Risk: const-eval / `generic_const_exprs` on the pin (WATCH, ICE-prone); type-level reorder of a heterogeneous cons-list is the hard part.
2. **Macro-emitted per-fiber bodies.** A macro (or `hilavitkutin-build` step) takes the plan and emits per-fiber dispatch fns with their unit sequence as a statically-derivable composition in RCM order (literally the hand-written bench shape, generated). Sidesteps const-eval-on-pin risk; the plan runs as ordinary code.
3. **Whole-schedule single body in plan order.** For the single-trunk single-core case, one statically-derivable body dispatches the whole schedule in RCM-reordered topo order; fiber boundaries set morsel-locality within it. Reduces the "non-contiguous fiber" problem to "one body, plan-ordered."

The probe builds the diamond against the real engine crates and tests, per candidate, whether the diamond's units dispatch correctly in RCM order through a statically-derivable body and whether a release-ASM disassembly shows zero `blr` in the dispatch region (the `disasm_5check` invariant). It records WORKS / FAILS-WITH per candidate and names the one to carry into the doc CL.

## Success criterion

At least one candidate dispatches the diamond correctly (`Zv[i]` matches the hand-fused reference for all records) with the dispatch region devirtualised (no surviving dispatch symbol, zero `blr`), under the pinned nightly. That candidate's shape is what the rewritten DOC CL describes and the SRC CL implements. If candidate 1 ICEs or deadlocks the solver, that is itself a recorded finding steering toward 2 or 3.

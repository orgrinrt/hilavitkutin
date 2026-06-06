# Findings: inline RunFiberCol with a non-empty accumulator (Phase D / #340)

**Outcome: WORKS** (nightly-2026-05-28, rustc 1.98.0-nightly cced03bfd, release
fat-LTO). Compiled and ran against the real engine crates resolved at arvo `dev`
HEAD (the rev the engine builds against after the #663 transitive bump, #666). A
heterogeneous two-unit cons-list mixing a column writer (`S1`: In -> Av) and an
accumulator appender (`Tally`: Av -> append `Accum<Sum>`) dispatched in order
through the inline `RunFiberCol` walk; the Av column equalled `stage1(i)` for all
64 records and the accumulator held 64 appends in record order with live length
64.

This is the one follow-on delta sketch `202606051601` named and did not run, and
the last open feasibility question on the dispatch mechanism before the SRC slice.

## What this settles

Sketch `202606051601` settled the column dimension (read/write column
projections restated inline) but PINNED the 7th `EngineCtx` GAT param (the
accumulator bundle) to the concrete `AccPtrNil` via an `Out = AccPtrNil` bound,
because its workload wrote no accumulator and restating the projection at a FREE
entry call deadlocks witness inference. It recorded fix path (b): drive the walk
from a context where `A` is pinned by `Self`, exactly `Scheduler::run<Witnesses>`.

This sketch runs fix path (b) directly. The `RunFiberCol` bound here restates the
accumulator projection (`for<'f> A: AccumProject<'f, W::Write, WAIdx>` plus the
7th GAT param `<A as AccumProject<'f, W::Write, WAIdx>>::Out`), which is
byte-for-byte the bound the shipped `CollectFiber` carries. The only differences
from `CollectFiber` are: the body is `RunFiber`'s inline recursion rather than a
`fiber_shim` fn pointer written into a slot, and the `A`-pin is provided by a
small `Harness<'b, A>` whose `drive` method fixes `A` from `Self` before
`Witnesses` is inferred (mirroring `run<Witnesses>` where
`A = <Vals as BindingsFor>::Bindings` is fixed by `Self`).

It type-checks and runs. So:

- The inline column-and-accumulator walk resolves the full 7-param `EngineCtx`
  GAT tie, including the lifetime-dependent accumulator projection, in the
  `A`-pinned context the engine has. No overflow, no recursion-limit, no
  normalization failure.
- The deadlock sketch `202606051601` hit is confined to FREE entry calls (where
  `A` and the witness infer together); it does not arise in `run<Witnesses>`,
  where `Self` pins `A` first. The `Harness` reproduces that pin and confirms it.
- `RunFiberCol` is a faithful drop-in for `CollectFiber` + `fiber_shim`: it
  resolves the identical trait tie in the identical context, so the slot path
  (`CollectFiber`, `FiberSlot`, `fiber_shim`, `noop_fiber_shim`) can be deleted
  with no loss of dispatch capability. Devirtualization of the inline walk was
  already confirmed by `202606051601` (linear chain) and `202606060500`
  (multi-phase morsel-outer, 131072 records); this sketch adds the
  accumulator-resolution and correctness delta, not a new devirt question (the
  append is the same inline body the prior sketches devirtualized).

## The plan-order to compile-time-order bridge (design resolution)

The locked doc CL (`202606051659_changelist.doc.lock.md`, increment item 1) named
one open mechanism to resolve before the SRC lock: how the plan-computed RCM
order becomes the compile-time order of the dispatch cons-list while still
devirtualizing. A runtime index into a stored order array (`live[order[k]]`) is
the 12.6x indirect anti-pattern being deleted, so that path is out. The
resolution, grounded in the canonical consolidation spec (Step 5 / Step 8), the
GCE soundness gate (#628), and the three sketches:

The cons-list (`Scheduler::wu_values`) is built at compile time in registration
order; its type and element order are fixed before any plan runs. The plan's RCM
row order is computed at runtime by `build_plan`. There are exactly two ways to
make the compile-time cons-list walk order equal the RCM-reordered topological
order:

1. The codegen flattener (spec domain 17): const-evaluate the plan and emit the
   cons-list in RCM order. This is the spec's eventual ideal. It requires the
   plan algorithms to be const-evaluable, which is `generic_const_exprs` /
   `generic_const_args` territory, gated behind the GCE soundness migration
   (#628). Not available for GATE 1.

2. Registration-is-canonical with build-time validation (GATE 1): the cons-list
   walks in registration order, devirtualized (`RunFiberCol`), and the engine
   requires/validates that registration order is the plan's RCM-reordered
   topological order. `group_fibers` (step 8) consumes the RCM row order
   (currently computed and left unused) to define the canonical fiber/unit
   sequence; a unit registered before its dependency is a build-time
   `BuildError` (the topo-validity check the doc CL claim 2 already names).

GATE 1 takes (2). This honors the canonical spec: RCM IS the dispatch order (the
plan computes it, `group_fibers` consumes it, the walk realizes it, the order is
validated against the plan), NOT arena-layout-only. The only deferred piece is
the AUTOMATION of the within-equal-depth reordering: who physically reorders the
cons-list when registration order and the plan's RCM order differ among
independent equal-depth units. At GATE 1 the consumer registers in the order
(forced for a linear chain, where exactly one topological order exists; a degree
of freedom only for fan-out, where the Approach-E sketch measured the RCM effect
at a reproducible ~2%), and the engine validates topo-validity. The engine
reordering the cons-list ITSELF is the codegen flattener, bounded precisely to
the GCE gate (#628). This is distinct from the corrected drift ("RCM is
arena-only; dispatch runs plain topological, RCM untouched"): here RCM is
consumed by dispatch and defines the required order; what is deferred is the
reordering automation, not whether RCM touches dispatch. See the workspace rule
`canonical-design-outranks-intermediate-rounds.md`.

Consequence for the first SRC CL (increment item 1): the dispatch code is "walk
the cons-list devirtualized in registration order; `group_fibers` consumes the
RCM row order to define the canonical sequence; validate registration order is
topologically valid against the plan." Stricter enforcement (reject a topo-valid
registration order that is not the RCM-preferred one among equal-depth units) and
full automation (the flattener) are bounded follow-ups; the dispatch code path is
the same either way (registration-order devirtualized walk + topo validation), so
item 1 does not depend on that choice.

## What this unblocks

The SRC slice can now: add `RunFiberCol` (this trait, shipped into `dispatch/`),
rewire `Scheduler::run<Witnesses>` to drive it over `self.wu_values` per fiber in
place of the `CollectFiber` slot-array dispatch, and delete `CollectFiber`,
`FiberSlot`, `fiber_shim`, `noop_fiber_shim`. The per-fiber morsel-locality
(morsel-outer / unit-outer on the fiber's `morsel_local` bit) wraps the walk
unchanged. The fusion half (real `classify_columns` Input/Output/Internal split,
scratch-backed internal columns) is the next increment and is what moves the
`#664` perf gate; this slice is the devirtualization half.

## Aside: build resolves clean against arvo dev HEAD

Unlike sketch `202606051601` (which copied a lock pinning pre-#663 arvo), this
sketch carries no lock and resolves arvo `dev` HEAD fresh. It compiles clean,
confirming the engine crates build against current arvo `dev` after #666 and that
the dispatch trait machinery is unaffected by the #663 graph-algorithm
parameterization.

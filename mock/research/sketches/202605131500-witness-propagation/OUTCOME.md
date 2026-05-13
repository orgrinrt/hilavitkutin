# Outcome S2: witness-propagation cost through engine-shaped caller graph

**Date:** 2026-05-13
**Status:** HYPOTHESIS CONFIRMED

## Verdict

The path-1 engine-side bridge is viable. Witness propagation through a
3-level call chain with 11 const generics and a 10-generic return type
costs the trait solver roughly nothing. Empirical numbers:

| Run | Wall clock |
|---|---|
| Cold (full arvo dep graph + 3 monomorphisations) | 20.06 s |
| Warm `cargo check` | 2.48 s |
| Warm `cargo check` after no-op-edit | 2.40 s |

Three concrete monomorphisations at `MAX_UNITS = 64`, `128`, `256`
all compile clean. Each instantiation forces full trait-solver work
through the witness clause at every chain level.

## What this changes about the prior plan

Sketch S1 (`202605121400-call-site-cap-turbofish`) validated the
const-generic-arg shape for a single wrapper in isolation. Its OUTCOME
flagged witness propagation as the load-bearing unresolved question
and recommended a probe sub-round before any wire-up. S2 IS that
probe, run in a controlled isolated crate rather than against the
live engine.

S2's evidence: witness propagation does not blow up. The src CL's
fear (path-1 propagation triggers exponential trait-solver work) is
not supported by S2. The src CL's Pass 2.5 (reverted) attempts hung
because they swept witness clauses over EVERY MAX_X (cartesian
product across many const generics on every fn), not because of the
single-witness-per-fn shape S1+S2 establish.

The real wire-up plan uses exactly one witness clause per fn:
`[(); cap_size(cap_of(MAX_UNITS))]:`. All three stubs (rcm, block,
spectral) only consume MAX_UNITS, not MAX_PHASES or MAX_FIBERS. The
witness is identical across the three stubs; propagation up the
caller chain duplicates this one clause, not a cartesian product.

## Acceptance scorecard

| Criterion | Threshold | Result |
|---|---|---|
| `cargo check` warm wall-clock | < 30 s | 2.48 s |
| `cargo check` cold wall-clock | < 6 min | 20 s |
| Per-level witness duplication count | exactly 1 per fn | confirmed |
| Trait-solver growth 1 to 3 levels | sub-linear or linear | sub-second across 3 levels with 3 instantiations |

All pass. Witness propagation through the engine's caller graph is
viable.

## Why the attempt-3 hang happened (working theory)

Attempt 3 (per src CL Pass 2.5 line 348-353) swept every engine
`<const MAX_*: usize>` to `<const MAX_*: Cap>` AND added a `[();
cap_size(MAX_X)]:` clause for every MAX_X on every fn. Eleven const
generics on `compute_execution_plan` would carry eleven witness
clauses, every fn calling it would also need eleven, every fn calling
those would need them too.

That is a cartesian shape: O(N const generics) witnesses per fn x
O(N callers) propagation depth. The trait solver's per-impl
constraint set scales multiplicatively.

The S2 shape is different: one witness over one MAX_X (the one that
actually feeds the arvo turbofish), propagated through O(N) callers.
Linear in propagation depth, constant in fan-out.

The misread is in attempting to lift EVERY engine MAX_X to Cap. The
correct lift is only the one MAX_X that the arvo wrappers consume.
Everything else stays usize.

## What this sketch does NOT yet test

S3: CSR-to-BitMatrix conversion at the wrapper site. The engine has
`DependencyGraph<MAX_UNITS, MAX_EDGES>` (CSR); arvo takes
`BitMatrix<W, N>` (dense). The wrapper must build the BitMatrix from
the CSR. Code is straightforward but new; will sketch.

S4: UnitId/NodeId width adaptation. UnitId is `Uint<16>` (u16-wide);
NodeId is `pub struct NodeId(pub USize)` (usize-wide). Cross-width
conversion at both ends.

S5: Array reshape between `[T; MAX_UNITS]` and `[T; cap_size(cap_of(MAX_UNITS))]`.
Numerically equal sizes, syntactically distinct to rustc. Either
explicit element copy or transmute through repr layout.

None of these is a trait-solver risk; they are all mechanical
integration questions. S2 has resolved the only solver risk.

## Path forward

1. S3: sketch CSR-to-BitMatrix conversion at the wrapper.
2. S4: sketch UnitId/NodeId width-mismatch handling.
3. S5: sketch the array reshape.
4. With S2-S5 all passing, write a Pass 2.5-resumption src CL section
   for the megaround (replacing the current "out of scope" framing
   with the now-validated path-1 plan).
5. Wire `rcm_reorder` first as the smallest of the three. Run
   `cargo check -p hilavitkutin` on the real engine; expect warm
   single-digit seconds based on S2 evidence.
6. Repeat for `block_diagonalise` then `spectral_partition`.
7. Pass 3 (engine dispatch) then proceeds with real arvo outputs in
   the chain.

## Educational notes on what made the test honest

Three deliberate choices to avoid the test passing for the wrong reason:

- **3-level chain, not 2.** The real call graph is engine entry →
  step fn → wrapper → arvo. Two levels would have missed any growth
  in the third hop.
- **10-generic return type via `PlanProxy`.** The real engine returns
  `ExecutionPlan<G1..G10>` through the chain. A simpler return type
  would have hidden trait-solver substitution-table cost.
- **Three different concrete instantiations.** Each forces independent
  monomorphisation. Two would have permitted accidental dedup; one
  would have made the test meaningless.

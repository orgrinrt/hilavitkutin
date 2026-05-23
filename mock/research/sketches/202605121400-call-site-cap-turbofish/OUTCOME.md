# Outcome: call-site Cap turbofish from usize-typed engine wrapper

**Date:** 2026-05-12
**Status:** PASSES (with const-fn refinement to the original hypothesis)

## Verdict

Option 1 (call-site Cap construction at the three arvo-stub call sites in
`plan/steps.rs`) is viable. The pattern that works:

```rust
const fn cap_of(n: usize) -> Cap {
    Cap(USize(n))
}

pub fn rcm_wrapper<W, const MAX_UNITS: usize>(
    adjacency: &BitMatrix<W, { cap_of(MAX_UNITS) }>,
) -> [NodeId; cap_size(cap_of(MAX_UNITS))]
where
    W: BitSequence + BitAccess + BitLogic + Identity + Copy + Default,
    [(); cap_size(cap_of(MAX_UNITS))]:,
{
    arvo_sparse::rcm_reorder::<W, { cap_of(MAX_UNITS) }>(adjacency)
}
```

The witness count per wrapper is exactly 1 (`[(); cap_size(cap_of(MAX_UNITS))]:`).
No propagation past the wrapper.

## What did not work

The original hypothesis used the inline form `{ Cap(USize(MAX_UNITS)) }` as
the const-arg. rustc rejected it:

```
error: overly complex generic constant
  --> src/lib.rs:40:35
   |
40 |     arvo_sparse::rcm_reorder::<W, { Cap(USize(MAX_UNITS)) }>(adjacency)
   |                                   ^^---------------------^^
   |                                     |
   |                                     struct/enum construction is not
   |                                     supported in generic constants
   |
   = help: consider moving this anonymous constant into a `const` function
```

This is the same `generic_const_exprs` limitation that requires every
non-trivial expression in a const-arg position to live behind a const fn.
Tuple-struct constructor application (`Cap(USize(...))`) counts as
"struct construction" and is rejected.

The fix follows the compiler's suggestion: wrap the conversion in
`const fn cap_of(n: usize) -> Cap`, then use `{ cap_of(MAX_UNITS) }`
everywhere the const-arg appears. Function-call expressions in const-arg
position ARE supported.

## The prior-attempt ICE did not reproduce

The memory file `project_round_202605111719_resume.md` recorded attempt 2:

> Direct bridge via `cap_of(MAX_UNITS)` triggers rustc ICE in
> `arvo_sparse::rcm::rcm_reorder::{constant#1}` when propagated through
> associated types.

This sketch uses exactly that `cap_of` bridge shape and does NOT ICE. The
working hypothesis: the ICE in attempt 2 came from a specific
associated-type propagation pattern (engine code projecting through
`<T as Trait>::Assoc` chains, which the sketch does not exercise). The
three real call sites in `plan/steps.rs` invoke arvo functions directly
with `BitMatrix` inputs constructed from raw arrays, no assoc-type
projection, so the ICE path is avoided.

If the ICE resurfaces during the src CL apply, the fallback is option 3
(arvo-side `*_usize` adapter shim). That fallback is cheap to add (one
arvo PR) and does not unblock anything else, so deferring it is safe.

## Compile-time numbers

| Run | Wall clock | Notes |
|---|---|---|
| Cold (first cargo check, full arvo dep graph) | ~6 min | Dominated by arvo build, not the sketch |
| Warm no-op edit | 3.65 s | Sketch typechecks; deps are cached |
| Concrete monomorphisation (`monomorphise_at_64`) | 2.88 s | Full end-to-end resolution through wrapper body |

Acceptance criterion 2 (compile time bounded by arvo deps, not sketch
trait-solver work) is satisfied. The 26-CPU-minute hang seen in attempt 3
of the engine-wide cap-typing path does NOT reproduce in this scoped
shape. Witness propagation past one wrapper is what blew up the engine;
keeping it bounded to the wrapper itself keeps it cheap.

## Acceptance scorecard

| Criterion | Status |
|---|---|
| `cargo check` zero errors | PASS |
| Compile dominated by arvo deps, not sketch trait-solver | PASS (3-second warm) |
| Witness propagation count = 1 per wrapper | PASS (`[(); cap_size(cap_of(MAX_UNITS))]:`) |
| Concrete monomorphisation works end-to-end | PASS (`monomorphise_at_64` compiles) |

## Path forward (revised)

The original "wire the three stubs" plan was scoped on the assumption
that wrappers ARE the call sites. The corrected reading (see the
witness-propagation section above) revealed that assumption was wrong:
`compute_execution_plan` is the actual entry point, and witness
clauses on stubs propagate up.

The revised next step is a probe sub-round that measures witness
propagation in isolation, not a wire-up sub-round that bundles three
unknowns. Concretely:

1. Add `const fn cap_of(n: usize) -> Cap` to a single engine module
   (suggested: `mock/crates/hilavitkutin/src/plan/steps.rs`).
2. Add a single new fn `probe_cap_witness<const MAX_UNITS: usize>()`
   with body `let _: USize = cap_size(cap_of(MAX_UNITS)).into();` and
   the one witness clause. Do NOT wire any stub.
3. Add ONE call to `probe_cap_witness::<MAX_UNITS>()` inside
   `compute_execution_plan` (top of body, behind `let _ = ...`).
4. `cargo check -p hilavitkutin` cold and warm. Measure wall-clock and
   the trait-solver work via `-Z self-profile` if needed.
5. If cold is under 8 minutes and warm is under 30 seconds: the
   witness-propagation path is viable; open a follow-up sub-round
   that wires the three stubs with the same shape.
6. If timings blow past those budgets: the engine's caller graph
   triggers exponential trait-solver work. Pivot to the src CL's path 2
   (arvo-side `*_usize` adapter shims) and open an arvo round for them.

The probe is one fn body and one call site. It can revert in seconds.
That is the right shape for the next question to answer; bundling it
with the wire-up risks re-running attempt 3.

## Caveats

The sketch returns arvo-shape arrays (`[NodeId; cap_size(cap_of(MAX_UNITS))]`)
not engine-shape (`[NodeId; MAX_UNITS]`). The two are numerically equal
but rustc does not unify them syntactically. The integration choice for
the three real wrappers:

- **Option A (simpler):** wrappers return arvo shape; engine callers
  index through `[T; cap_size(cap_of(MAX_UNITS))]`. The shape lives on
  the wrapper return type. Engine arrays around the call site stay
  `[T; MAX_UNITS]`-shaped, and the conversion happens through
  `arr.as_slice()` views or explicit reshape into the engine-shape
  array via a copy. Acceptable since the wrappers are called once per
  plan and the cost is trivial.
- **Option B (more work):** wrappers return engine shape via explicit
  `core::mem::transmute_copy::<[T; cap_size(cap_of(MAX_UNITS))], [T; MAX_UNITS]>`.
  Layout-identical by spec, but unsafe. Skip unless option A's perf
  cost matters.

The src CL picks option A unless the per-frame cost is measurable.

## What this sketch did NOT test (deferred; honest scoping)

The sketch validated the const-generic-arg shape for a wrapper in isolation.
Three integration questions remain open, and a closer read of the engine
post-sketch reveals one of them is load-bearing in a way the original
framing missed.

### Witness propagation through real callers (load-bearing, unresolved)

The original OUTCOME paragraph said "the wrappers ARE the call sites" and
treated witness propagation as a non-issue. That was wrong. The three
stubs in `plan/steps.rs` are called from
`mock/crates/hilavitkutin/src/plan/mod.rs:259-264` inside
`compute_execution_plan<const MAX_UNITS: usize, ...>`. That fn is itself
generic over MAX_UNITS. Adding `where [(); cap_size(cap_of(MAX_UNITS))]:`
to a wrapper forces the same clause onto `compute_execution_plan`,
onto every caller of `compute_execution_plan`, onto every caller of
those, and so on until a fully monomorphised entry point. That is the
witness propagation pattern that killed attempt 3 with the 22-minute
rustc hang (or, per the src CL revision, the 23-minute cold-build floor).

The sketch's "warm cache 3s" finding does not transfer to the engine.
The sketch has zero callers; the engine has many. The trait solver work
is per-constraint per-impl, and the engine has hundreds of impls inside
its const-generic graph.

### Engine→arvo data conversion (mechanical but not free)

The engine uses CSR `DependencyGraph<MAX_UNITS, MAX_EDGES>`; arvo wants
dense `BitMatrix<W, N>`. The wrapper must build the BitMatrix from CSR,
which is straightforward (set bit per edge) but is new code per stub.

Engine uses `UnitId = Uint<16>` (u16-wide); arvo uses
`NodeId = pub struct NodeId(pub USize)` (usize-wide). Cross-width
conversion at both ends, again straightforward but new code.

Engine arrays are `[T; MAX_UNITS]`; arvo returns
`[T; cap_size(cap_of(MAX_UNITS))]`. Numerically equal, syntactically
distinct. Either array-element copy or unsafe layout transmute through
repr(transparent).

### Arvo's other algorithm crates

Whether `arvo-comb` and `arvo-graph` work with the same shape is moot
for the three megaround-blocking stubs; defer until a fourth call site
appears.

## Revised verdict

The sketch confirms `const fn cap_of(n: usize) -> Cap` is accepted in
generic-const-arg position. It does NOT confirm that the engine's
caller graph can absorb the witness clause without trait-solver blowup.
The src CL's path "Arvo-side `usize` bridge adapters" (line 356) would
sidestep the witness propagation by accepting raw `usize`-sized arrays
on the arvo side, with arvo doing the Cap wrapping internally. The
sketch does not refute that path; the sketch establishes only that the
engine-side bridge is locally syntactically viable.

The honest current-session output is this sketch + this OUTCOME, not
the full wire-up. Wiring the three stubs without first validating
witness propagation through `compute_execution_plan` would risk
re-entering attempt-3 territory.

The remaining work (subject to a future sub-round):

1. Decide between path 2 (arvo-side adapter) and path 1 (engine-side
   bridge with witness propagation). The choice depends on the trait
   solver's actual behaviour under propagation through the engine's
   const-generic caller graph, which is exactly what attempt 3 could
   not measure cleanly.
2. If path 1: scope a probe round that adds ONE witness clause to
   `compute_execution_plan` (no stub wire-up yet) and measures
   `cargo check -p hilavitkutin` cold and warm. That probe answers
   the witness-propagation question in isolation, without committing
   to the wire-up itself.
3. If path 2: open an arvo round for the `*_usize` adapter shims,
   then return to the hilavitkutin sub-round with the simpler engine
   surface.

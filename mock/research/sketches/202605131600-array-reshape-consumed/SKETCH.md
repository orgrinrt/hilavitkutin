# Sketch S5: end-to-end consumption with array reshape

**Date:** 2026-05-13
**Sequel to:** S2 (witness propagation) + S3 (CSR-to-BitMatrix + UnitId/NodeId bridge)

## Why this sketch exists

The earlier sketches validated the call path from engine to arvo. They
left "the engine consumes arvo's output" as a deferred question, on
the working theory that the current stubs discard the output anyway.
That framing was wrong: the stubs discard the output because they ARE
stubs, not because the engine never wants to consume. The reordered
permutation, the block-feasibility decision, and the spectral
partition all feed real engine plan-stage logic in the design
(steps 7 through 13). Wiring the wrappers without validating that the
consumed output is shape-compatible commits us to a wire-up that
must be redone when consumption lands.

S5 validates the full consumption shape: arvo returns its array, the
wrapper materialises it back into the engine's expected
`[UnitId; MAX_UNITS]` shape, and downstream engine code reads from
that array using its existing patterns. If the consumption path
compiles end-to-end, the wire-up is shape-stable; if it does not,
we learn the failure mode NOW, while the wrapper shape is still in
the sketch.

## Hypothesis

For each of the three stubs:

1. **rcm_reorder**: arvo returns `[NodeId; cap_size(cap_of(MAX_UNITS))]`.
   The engine wants `[UnitId; MAX_UNITS]` where each entry is the
   `UnitId` for "the old-position unit now at the new-position index".
   The reshape is a per-element conversion: for each `i` in
   `0..cap_size(cap_of(MAX_UNITS))`, read `arvo_result[i].0.0` (the
   `usize` raw value), narrow to `u16`, build a `UnitId` via the
   engine's existing `transmute_copy(&u32)` pattern. The arvo size
   and the engine size are numerically equal but syntactically
   distinct; rustc needs a concrete strategy to bridge them. Three
   candidate strategies in the sketch.

2. **block_diagonalise**: arvo returns `(USize, [USize; cap_size(...)])`.
   The engine wants `Bool` for feasibility. The reshape is "did arvo
   succeed and produce >= 1 block?" — purely a scalar projection, no
   array reshape needed. The sketch confirms this is genuinely scalar
   (no hidden array consumption downstream).

3. **spectral_partition**: arvo returns `(USize, [USize; cap_size(...)])`.
   The engine wants `FiberGrouping<MAX_UNITS, MAX_FIBERS>`. This is
   the heaviest reshape: arvo emits a per-node class id (0 or 1, since
   spectral bisection is two-class); the engine's `FiberGrouping`
   stores a per-unit `FiberId`. The mapping needs class-id-to-FiberId
   and the engine's `MAX_FIBERS` cap to align with arvo's two-class
   output. If MAX_FIBERS == 1, the spectral output collapses; the
   sketch tests that boundary.

## Three candidate reshape strategies

The sketch tests all three, picks the one that compiles with the
cleanest witness story.

### Strategy A: per-element copy

```rust
let mut out: [UnitId; MAX_UNITS] = [UnitId::ZERO; MAX_UNITS];
let arvo_out: [NodeId; cap_size(cap_of(MAX_UNITS))] = arvo_call(...);
let mut i = 0usize;
while i < MAX_UNITS {
    let raw_usize = arvo_out[i].0.0;
    let raw_u32 = raw_usize as u32;
    let unit: UnitId = unsafe { transmute_copy(&raw_u32) };
    out[i] = unit;
    i += 1;
}
out
```

Pros: zero unsafety on the array, explicit bounds, the only unsafe is
the existing engine pattern for UnitId construction.

Cons: O(MAX_UNITS) runtime copy. Fine for plan-stage one-shot.

### Strategy B: `core::mem::transmute_copy` of the whole array

```rust
let arvo_out: [NodeId; cap_size(cap_of(MAX_UNITS))] = arvo_call(...);
unsafe { transmute_copy::<_, [UnitId; MAX_UNITS]>(&arvo_out) }
```

Pros: zero runtime cost.

Cons: requires layout-identical arrays. `NodeId` is `pub struct
NodeId(pub USize)` (usize-wide); `UnitId` is `Uint<16>` (u32-wide
container). Layouts ARE NOT identical (8 bytes vs 4 bytes on
64-bit). This strategy is unsound.

### Strategy C: const-eval reshape via `array::from_fn`

```rust
let arvo_out: [NodeId; cap_size(cap_of(MAX_UNITS))] = arvo_call(...);
core::array::from_fn::<UnitId, MAX_UNITS, _>(|i| {
    let raw_usize = arvo_out[i].0.0;
    let raw_u32 = raw_usize as u32;
    unsafe { transmute_copy(&raw_u32) }
})
```

Pros: idiomatic, no manual `while` loop.

Cons: indexing into `arvo_out[i]` with `i: usize` against an array of
length `cap_size(cap_of(MAX_UNITS))` requires rustc to prove
`i < cap_size(cap_of(MAX_UNITS))` when `i < MAX_UNITS`. The two
expressions are numerically equal but syntactically distinct. This
is the same shape-unification question that motivated S5. If
strategy C compiles, the unification works at the indexing site
(rustc's bounds check is runtime, so no proof needed; the indexing
panics only if MAX_UNITS != cap_size(cap_of(MAX_UNITS)) at runtime,
which it can't given how the values relate).

## Acceptance

| Criterion | Threshold |
|---|---|
| All three reshapes (rcm, block, spectral) compile end-to-end | required |
| Engine consumer can read from the wrapper's return | required |
| The choice between strategy A and C lands with documented rationale | required |
| `cargo check` warm under 8 seconds | required |
| Layout-soundness assertions cover any unsafe used | required |

## What this sketch does NOT yet test

- Witness clause propagation when the wrapper returns engine-shape
  AND consumer code on the caller side reads from the return.
  S2 tested propagation with arvo-shape returns; this sketch tests
  propagation with engine-shape returns, which is the realistic case.
- Whether the engine's existing `topo` permutation (the input to rcm)
  also needs reshape, since rcm reads from `topo: &[UnitId; MAX_UNITS]`
  and arvo's input is the adjacency matrix (built from CSR), not the
  topo array. The current stub passes `topo` through unchanged; the
  real wire-up may or may not feed topo into the arvo call.

## Outcome

See `OUTCOME.md` after `cargo check`.

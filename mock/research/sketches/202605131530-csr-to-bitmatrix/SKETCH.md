# Sketch S3+S4: CSR-to-BitMatrix + UnitId/NodeId bridge

**Date:** 2026-05-13
**Sequel to:** S2 (`202605131500-witness-propagation`, witness propagation viable)

## Hypothesis

A standalone conversion fn

```rust
fn csr_to_bitmatrix<const MAX_UNITS: usize, const MAX_EDGES: usize>(
    graph: &DependencyGraphLike<MAX_UNITS, MAX_EDGES>,
) -> BitMatrix<W, { cap_of(MAX_UNITS) }>
```

(where `W = Bits<64, Hot>`) materialises a dense `BitMatrix` from the
engine's CSR representation. The UnitId-to-NodeId bridge is
`transmute_copy`-extract-u32 then `NodeId::new(USize(u32 as usize))`,
mirroring the projection already used at `graph.rs:120`. The result
is a `BitMatrix` ready to feed `arvo_sparse::rcm_reorder` via the
option-1 wrapper from S1.

S3 confirms:
1. The CSR walk fits inside the wrapper body.
2. UnitId-to-NodeId conversion produces a sound `NodeId` value.
3. The resulting BitMatrix compiles and is shape-compatible with arvo's
   API.
4. End-to-end call (CSR → BitMatrix → arvo_sparse::rcm_reorder)
   produces a runnable monomorphisation.

## What this sketch does NOT test

S5 (array reshape: arvo returns `[NodeId; cap_size(cap_of(MAX_UNITS))]`,
engine wants `[UnitId; MAX_UNITS]`). The S3 wrapper accepts whatever
arvo returns and discards it. S5 lands separately.

## Acceptance scorecard

| Criterion | Threshold |
|---|---|
| Sketch compiles | required |
| Conversion fn is `<const MAX_UNITS: usize, const MAX_EDGES: usize>`-shaped | required |
| `transmute_copy` projection sound (verifies via size assertion) | required |
| End-to-end call produces an `[NodeId; cap_size(cap_of(MAX_UNITS))]` array | required |
| Warm `cargo check` under 5 seconds | required |

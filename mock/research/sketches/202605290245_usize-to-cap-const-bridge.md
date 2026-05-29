# Sketch: usize -> Cap const-param bridge for the DependencyGraph -> Csr adapter

**Hypothesis:** the C1a adapter can return `Csr<{cap(MAX_UNITS)}, {cap(MAX_EDGES)}, EdgeKind>` from a `DependencyGraph<const MAX_UNITS: usize, const MAX_EDGES: usize>`, bridging the engine's `usize` const-params to arvo's `Cap` const-params, under `generic_const_exprs`.

**Outcome: the naive form FAILS; the const-fn form WORKS.** Validated by a standalone stable-shape crate replicating arvo's `Cap(USize)` + `cap_size` + `Csr<const ROWS: Cap, ..>` shape (no arvo dep, `feature(adt_const_params, generic_const_exprs)`).

## What fails

Writing the construction expression directly in const-generic position:

```rust
fn to_csr(&self) -> Csr<{ Cap(USize(MAX_UNITS)) }, ..>
where [(); cap_size(Cap(USize(MAX_UNITS)))]:,
```

rustc rejects every occurrence with:

```
error: overly complex generic constant
   |  [(); cap_size(Cap(USize(MAX_UNITS)))]:,
   |       ^^^^^^^^^---------------------^
   |                struct/enum construction is not supported in generic constants
```

The blocker is narrow: a literal struct constructor (`Cap(USize(N))`) is not allowed in a generic-constant context.

## What works

Hide the construction inside a `const fn`; a const-fn CALL is permitted in generic-const position (this is exactly how `cap_size(ROWS)` is already used by arvo):

```rust
pub const fn cap_of(n: usize) -> Cap { Cap(USize(n)) }

impl<const MAX_UNITS: usize, const MAX_EDGES: usize> DepGraph<MAX_UNITS, MAX_EDGES> {
    pub fn to_csr(&self) -> Csr<{ cap_of(MAX_UNITS) }, { cap_of(MAX_EDGES) }, u8>
    where
        [(); cap_size(cap_of(MAX_UNITS))]:,
        [(); cap_size(cap_of(MAX_EDGES))]:,
    { /* element-wise copy within unit_count / edge_count */ }
}
```

Compiles clean; the round-trip test (`row_ptr`/`col_idx`/`values` copied within the live counts) passes. `cap_size(cap_of(N)) == N`, so the engine's `[T; MAX_UNITS]` arrays and arvo's `[T; cap_size(ROWS)]` arrays have equal length and the copy is sound.

## What this unblocks / decides for C1a

- The adapter is `DependencyGraph::to_csr_bidirectional(&self) -> CsrBidirectional<{cap_of(MAX_UNITS)}, {cap_of(MAX_EDGES)}, EdgeKind>` (build `Csr` then `.with_transpose()`), with the `[(); cap_size(cap_of(..))]:` where-bounds. Column indices fill via `NodeId::new(unit_id.index())` (R1's accessor).
- A `const fn cap_of(usize) -> Cap` is required. arvo has `cap_size` (Cap -> usize) but NOT its inverse, so this is a real arvo Cap-surface gap. C1a defines the helper engine-side to keep C1a a single-repo round; FOLLOW-UP: upstream the canonical `arvo::cap(usize) -> Cap` (the inverse of `cap_size`, belongs next to it in arvo-tensor) and have the engine consume it. Documented in the C1a PR for reviewer redirect.

Sketch crate lived at `/tmp/cap_bridge_sketch` (not committed; this note is the audit record).

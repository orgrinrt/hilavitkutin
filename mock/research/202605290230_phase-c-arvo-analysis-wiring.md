# Phase C design note: wire arvo analysis into the plan chain

Self-standing implementation guide for Phase C of the engine-dispatch arc (the
spine step after the Phase B data-plane keystone). Read alongside the arc memo
`202605282100_engine-dispatch-build-plan.md`. Phase C makes the plan stage
consume the arvo analysis primitives it currently computes-and-discards or
stubs, so trunks and fibers are formed from real dependency structure instead of
left at defaults.

## Current state (verified 2026-05-29 by code-explorer)

`compute_execution_plan` (`plan/mod.rs:190-342`) is a 13-step chain. Three steps
are genuine dead ends:

- Step 4 `steps::rcm_reorder` (`steps.rs:200-216`): pass-through stub, output
  assigned to `_reordered` and never read.
- Step 5 `steps::block_diagonalise` (`steps.rs:222-232`): returns `Bool::TRUE`
  unconditionally; produces no component records.
- Step 6 `steps::spectral_partition` (`steps.rs:241-249`): returns
  `FiberGrouping::new()` (empty); output assigned to `_clusters`, never read.

Working hand-rolled steps that ARE consumed: step 2 `topo_sort`, step 3
`compute_waists`, step 7 `group_fibers`, step 8 `compute_upward_rank_and_dirty`,
steps 9-11 (morsels, phase configs, column classes). Steps 12-13 (core
assignment, core-program synthesis) are stubs that belong to Phase D/E.

Plan structures left unpopulated by the chain:

- `Phase` (`plan/phase.rs:63-78`): `id`, `trunks`, `trunk_count` never set (only
  `config` is). They stay at `Phase::new()` defaults.
- `Trunk` (`plan/trunk.rs:95-104`): `components`, `component_count` never set; the
  block-diagonalise step is meant to emit the component sequence but stubs out.
- `Fiber` (`plan/fiber.rs:173-188`): all fields at defaults; the `FiberGrouping`
  from step 7 is never projected back onto `Phase.trunks`.

The `DependencyGraph` (`plan/graph.rs:59-70`) is already a bespoke CSR
(`row_offsets: [USize; MAX_UNITS]`, `col_indices: [UnitId; MAX_EDGES]`,
`edge_kinds: [EdgeKind; MAX_EDGES]`), NOT a dense matrix. The arc memo's "dense
matrix, #337" framing was wrong; the real work is adapting this CSR to the arvo
shape the arvo algorithms consume.

## The arvo surface (all exists, non-stub)

- `arvo-sparse`: `Csr<ROWS: Cap, NNZ: Cap, W: Copy>` with PUBLIC fields
  (`row_ptr: [USize; cap_size(ROWS)]`, `col_idx: [NodeId; cap_size(NNZ)]`,
  `values: [W; cap_size(NNZ)]`), `Csr::new()`, `.with_transpose() ->
  CsrBidirectional`. `CsrBidirectional` impls `BidirectionalSparseAdjacency<N>`.
  `rcm_reorder_via`, `block_diagonal_via`, `dulmage_mendelsohn_via` all take
  `&impl BidirectionalSparseAdjacency<N>` and are const-generic in N (NO node
  cap).
- `arvo-graph`: `topo_sort`, `waist_detect`, `components`, `upward_rank` take
  `&BitMatrix<Bits<64, Hot, Unsigned>, N>`, hardcoded to a 64-NODE CAP.
- `arvo-spectral`: `fiedler_vector`, `spectral_bisection`, `k_way_partition`,
  `SparseLaplacian` (impls `LinearOperator`). The CSR-backed `SparseLaplacian`
  path is const-generic (no node cap).

## Load-bearing decision: CSR path for stubs, keep the hand-rolled rest

The engine's `UnitId` is `Uint<16>` (up to 65535 units); `DependencyGraph` is
const-generic to `MAX_UNITS`. arvo-graph's `BitMatrix` algorithms cap at 64
nodes. Therefore:

- DO wire the stubbed steps (rcm, block-diagonal, spectral) via the CSR path
  (`*_via` on `CsrBidirectional`, `SparseLaplacian` for spectral). These are
  const-generic, no cap, and scale to `MAX_UNITS`.
- DO NOT replace the working hand-rolled `topo_sort` / `compute_waists` /
  `compute_upward_rank_and_dirty` with arvo-graph's `BitMatrix` versions: that
  would regress the engine from `MAX_UNITS` to 64 nodes. The hand-rolled impls
  are correct and const-generic; leave them.
- FILE an arvo follow-up to widen arvo-graph beyond the 64-node `Bits<64>`
  container (multi-limb bit matrix), so a future round MAY unify topo/waist/rank
  on arvo-graph without the cap. Until then, the split (hand-rolled scalar algos,
  arvo CSR/spectral for the partitioning steps) is the non-regressing shape.

This keeps Phase C purely engine-side wiring (no arvo implementation round); the
only arvo touch is the deferred widening follow-up, which does not block C.

## Sub-round decomposition (one branch, sequential rounds)

- **C1a (adapter, the foundation):** add `DependencyGraph::to_csr_bidirectional()
  -> arvo_sparse::CsrBidirectional<MAX_UNITS, MAX_EDGES, EdgeKind>`. Build a
  `Csr` by writing `row_ptr` from `row_offsets`, `col_idx` from `col_indices`
  (convert each `UnitId` via `.index() -> USize -> NodeId`; verify the `NodeId`
  constructor), `values` from `edge_kinds`; then `.with_transpose()`. Bound
  `MAX_UNITS`/`MAX_EDGES` as `Cap`. Test: build a small DAG, adapt, assert
  `nnz`/`row_col_indices`/`row_values` per row match the source edges, and the
  transpose has the reverse adjacency. EdgeKind must be `Copy` (it is).
- **C1b (rcm):** step 4 calls `arvo_sparse::rcm_reorder_via(&csr_bidi)`; consume
  the reordering where the plan currently discards `_reordered` (fiber ordering /
  renumber). Test: a graph with known bandwidth reorders as expected.
- **C1c (block-diagonal + trunks):** step 5 calls `block_diagonal_via`; project
  the block structure into `Phase.trunks[i]` + `Trunk.components` +
  `trunk_count`, and set `Phase.id`. Replace the `Bool::TRUE` stub with the real
  alignment check. Test: a block-structured graph yields the expected trunk
  components; mismatched alignment still raises `PhaseAlignmentMismatch`.
- **C1d (spectral + fibers):** step 6 builds a `SparseLaplacian` from the CSR and
  calls `k_way_partition` (or `SpectralBipartitioner`); consume the resulting
  `FiberGrouping` and project fibers onto `Phase.trunks`. Reconcile with the
  existing `group_fibers` (step 7): decide whether spectral replaces or refines
  it. Test: a graph with a clear cut partitions as expected.

C1a is unambiguous and unblocks the rest; start there. C1b-C1d each consume the
adapter output and populate progressively more of the plan structures. Per-round
the `pr-reviewer-senior` flow and the mockspace v1 ceremony apply.

## Risks

- `NodeId` construction from `USize`: verify arvo-bitmask's `NodeId` has a public
  ctor from an index value (used in the adapter's `col_idx` fill). If absent, that
  is a one-line arvo addition, the only possible arvo touch in C.
- `Cap` vs the engine's current `MAX_UNITS`/`MAX_EDGES` const-param types: the
  adapter's arvo `Csr` needs `Cap`-typed params; confirm the engine's graph
  const params are `Cap` or convert at the boundary.
- `cap_size(ROWS)` vs `MAX_UNITS` array-length identity: arvo sizes its arrays by
  `cap_size(ROWS)`; the engine sizes by `MAX_UNITS`. The adapter copies element
  by element within `unit_count` / `edge_count`, so a length mismatch between the
  two backing arrays is fine as long as both exceed the live counts.

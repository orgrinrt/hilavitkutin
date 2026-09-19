# A1-1 Resolution: RCM Row Order Wins the Dispatch-Order Bench

**Date:** 2026-07-19
**Status:** bench-decided per A1-1's registered oracle ("a cache-locality
bench on wide fan-out DAGs decides which wording canon keeps"; bench-decided
forks self-rule). Canon keeps the consolidation spec's Step 5/8 dual-ordering
wording: the RCM row reordering is the WU execution order.
**Bench:** `mock/benches/` bench `rcm_order` (variants `rcm_adj`,
`rcm_scr`, shared workload `variants/rcm_common`), run under the upgraded
harness (per-bench timing override, cross-variant byte-exact validation,
determinism check, two harness runs). Results committed at
`mock/benches/results/rcm_order/`.

## The question

A1-1 registered the fork: canon's Step 5/8 says the RCM row order is the WU
execution order (cache-optimal among valid topological orders on wide
fan-out DAGs); the shipped engine dispatches in the registration/waist-phase
order. Both are buildable behind the const grouping mechanism; the bench
decides which wording canon keeps.

## The bench

Eight write-disjoint WUs at one topological depth; WU k reads input columns
C{k} and C{k+1} and writes O{k}, so consecutive indices share exactly one
input column and any permutation is a valid topological order. Arm `rcm_adj`
registers (and therefore dispatches, verified by an execution-order probe
per prepared scheduler) in column-adjacent order 0..8; arm `rcm_scr` in a
stride-4 scramble where no two consecutive fibers share a column. Identical
column registration isolates dispatch order as the only variable; outputs
validated byte-exact across arms. Record counts 64K / 256K / 1M (columns
256 KiB / 1 MiB / 4 MiB); one marked-dirty engine frame per timed call, the
scheduler cached per worker process.

## The result (warm medians, M1, two harness runs)

| records | column | rcm_adj | rcm_scr | scr/adj |
|---|---|---|---|---|
| 65536 | 256 KiB | 89.22 us | 90.24 us | 1.01x |
| 262144 | 1 MiB | 429.82 us | 412.78 us | 0.96x |
| 1048576 | 4 MiB | 2.13 ms | 2.34 ms | 1.10x |

At the large-column regime the adjacency order wins 1.10x: a shared column
streamed by one fiber is still L2-resident when the next fiber re-reads it,
while the scramble's intervening traffic (about 96 MiB between shares)
evicts it. At 256 KiB everything is cache-resident and order is neutral; the
1 MiB point's 4 percent inversion is within run-to-run spread and carries no
consistent mechanism.

## The ruling this records

1. Canon's Step 5/8 wording stands: the RCM row order is the execution
   order. The fork registered by A1-1 closes; the seed plan chapter drops
   its open-fork paragraph in favour of this record.
2. The shipped waist-phase dispatch order is now measured drift at large
   column sizes, not a neutral alternative. The auto-RCM application (the
   guarded-walk mechanism, sketch `202606090300`, proven devirt-free)
   remains the implementing work, and landing it dissolves the provisional
   producer-before-consumer registration constraint, as canon already
   marks.
3. The execution-order probe finding rides along: the engine's fiber-level
   dispatch order equals carrier (registration) order under the shipped
   const grouping, confirmed empirically against the real engine.

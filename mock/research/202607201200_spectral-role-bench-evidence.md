# Spectral Role Deviation: the Definitive Fiber Bench (#644)

**Date:** 2026-07-19
**Status:** evidence delivered under the evidence-then-bless standard
(A2-4); seed governance item 3. The oracle A1-era canon registered: "the
definitive spectral-versus-greedy fiber bench decides whether the
shipped role swap is blessed or the canonical Step 7/8 split is
restored."
**Bench:** `mock/benches/` bench `fiber_theory` (arms `fib_greedy`,
`fib_spectral`, shared workload `variants/fiber_common`), timing the two
SHIPPED plan functions (`plan::steps::group_fibers` and
`plan::steps::spectral_partition`) on one layered wide-fan-out DAG
family at unit counts 8 through 64 (the engine's default capacity).
Quality proxies from the crate's companion test. Results at
`mock/benches/results/fiber_theory/`.

## The deviation under judgment

Canon Step 7 forms TRUNKS spectrally (Fiedler bisection minimising the
cut of shared column bytes, applied when a phase carries more than five
fibers) and Step 8 forms FIBERS greedily (topo-order walk under the
holistic feasibility check, matrix-chain DP above ten ops). The shipped
plan chain swaps the roles: trunks come from block-diagonal connected
components, and spectral forms fibers within wide blocks.

## Evidence

Plan-time cost (warm medians, one clean run under the transactional
harness):

| units | greedy | spectral | ratio |
|---|---|---|---|
| 8 | 38 ns | 17.20 us | 449x |
| 16 | 96 ns | 65.69 us | 686x |
| 32 | 185 ns | 82.93 us | 448x |
| 64 | 398 ns | 128.22 us | 322x |

Grouping character at 64 units, four seeds (fiber count / cut edges):

| seed | greedy | spectral |
|---|---|---|
| 3 | 45 / 110 | 2 / 4 |
| 17 | 49 / 119 | 2 / 3 |
| 51 | 48 / 117 | 2 / 8 |
| 101 | 51 / 124 | 2 / 8 |

## What the numbers say

The two algorithms do not produce interchangeable answers at different
speeds; they answer DIFFERENT questions. Greedy produces fiber-grained
output: many small groups bounded by the fan-in feasibility rule, the
granularity the register-file budget and morsel windowing require.
Spectral bisection produces trunk-grained output: two coarse halves
with a minimal cut, exactly the shape canon's Step 7 wants for
column-disjoint trunk separation, and far too coarse for fibers (a
32-unit "fiber" has no relation to the domain-14 holistic feasibility
budget, which spectral never consults). On plan-time cost, greedy is
linear and effectively free at engine scale; spectral's eigen iteration
is two to three orders of magnitude heavier, a price that makes sense
paid once per phase for a coarse trunk cut and does not make sense as
the inner fiber-forming primitive.

Both halves of the evidence therefore point the same way: the CANONICAL
role assignment matches what the shipped algorithms actually produce
and cost, and the shipped role swap (spectral for fibers within wide
blocks) uses the expensive coarse-cut tool for the fine-grained
budget-bound job while leaving the cheap budget-aware tool unused in
that position.

## Proposed ruling (op's call, per evidence-then-bless)

Restore canon's Step 7/8 role split in the plan chain: spectral for
trunk formation above the five-fiber threshold, greedy (plus the DP
tier) for fiber grouping under the feasibility check; the
block-diagonal components remain the phase validation they were always
specced as (Step 6), not the trunk former. The corrective is a design
round on the plan chain, sequenced with the B2 spectral-consumption
integration the r6 roadmap already carries; this bench's result answers
r6's open question ("whether B2's spectral consumption is worth its
complexity") with: worth it at the trunk tier only.

## Scope honesty

The bench measures the shipped functions on a bare DAG; canon's Step 7
edge weights (shared column bytes) and Step 8 cost model (records times
union column bytes) need the access matrix, which the bare-DAG family
does not carry. The fiber-count and cut proxies stand in for them, and
the role conclusion does not hinge on the weighting: no weighting turns
a two-way bisection into feasibility-bounded fibers, and none makes the
eigen iteration linear. Dispatch-level A/B is not runtime-steerable
(dispatch follows the compile-time carrier grouping), so execution
confirmation rides the corrective round's own gates once the roles are
restored.

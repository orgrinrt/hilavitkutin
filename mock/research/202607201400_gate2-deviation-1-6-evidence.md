# GATE-2 Deviations 1 and 6: Arena Route Evidence

**Date:** 2026-07-19
**Status:** evidence delivered under evidence-then-bless (A2-4); seed
governance item 5, deviations 1 (inline `PoolFrame` with the `Pin`
receiver) and 6 (inline GATE-2 scratch), which the ledger already ties
together ("arena relocation is the reconciliation, tied to deviation 1").
**Sketch:** `mock/research/sketches/202607201300_arena-poolframe/`
(outcome WORKS; findings in the sketch directory).

## What the sketch established

The shipped `Pin<&mut Self>` on `run_parallel` is not a `PoolFrame`
property: workers hold a type-erased back-pointer to the WHOLE scheduler,
so pinning guards every inline field dispatch dereferences. The sketch
proves the canonical alternative mechanically: when the full
worker-visible plane (pool sync words, worker contexts, the data the
workers walk) lives in a provider-allocated arena block and the owner is
a movable handle around the plane pointer, the frame protocol runs
unchanged across repeated moves of the handle, byte-exact, with clean
shutdown-join, and no `Pin` or `PhantomPinned` anywhere. Arena-placing
the pool ALONE would dissolve nothing; whole-plane residency is the
canonical shape's real content, and it is exactly the relocation
deviation 6 names.

## The cost of the shipped shape (deviation 6's dead weight)

Every scheduler, single-core consumers included, carries the inline
threaded-path scratch: the const-grouping arrays (`gate2_phase` +
`gate2_trunk`, 4 KiB at the 256-unit cap), the per-core accumulator
publish array (`gate2_accum_live`, 256 cores x 16 accumulators, 32 KiB),
the adapt EMAs (`phase_ema` + `phase_accum`, 4 KiB), the worker contexts
(4 KiB), and the pool's per-core counters (4 KiB), about 48 KiB of
struct weight that a single-core consumer never touches. Under the arena
route this block is allocated at the first `run_parallel` and never
exists for single-core apps.

## The tradeoff for the ruling

The REBUILD route (canonical): relocate the worker-visible plane into a
provider allocation at first `run_parallel`; the scheduler becomes
movable, the consumer-facing `Pin` disappears from the public surface,
and the 48 KiB scratch disappears from single-core schedulers. The
mechanism is proven; the cost is a broad but mechanical scheduler-layout
refactor (every worker-reached field moves behind the plane pointer, and
the parked-between-frames aliasing discipline transfers to the plane
unchanged). The BLESS route (shipped): keep `Pin` at the consumer and
the inline weight, changing nothing. Op's standing signal on this
deviation ("interim sounds wrong in principle; it only passes if it is
the final solution") cuts against blessing a shape whose own ledger
entry names its reconciliation; the evidence here says the final
solution is buildable and its mechanism is now proven, so the honest
framing is: bless only if the `Pin` surface is judged acceptable as
PERMANENT, otherwise schedule the relocation round.

## Proposed ruling (op's call)

Rebuild via a scheduler-plane relocation round: one design round moving
the worker-visible plane (pool, worker contexts, GATE-2 scratch, adapt
EMAs) into a provider allocation, dropping `PhantomPinned` and the `Pin`
receiver, with the sketch's whole-plane rule as the acceptance test and
the existing thread tests plus the S6/gate2 suites as the regression
net. This resolves deviations 1 and 6 together, as the ledger intended.

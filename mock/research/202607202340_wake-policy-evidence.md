# GATE-2 Deviation 3: the Wake Spin Pre-Roll, Benched and Ruled by the Numbers

**Date:** 2026-07-19
**Status:** the deviation-3 evidence under evidence-then-bless (A2-4);
seed governance item 5. Closes the last open deviation channel of the
pre-freeze batch.

## What was built (round 202607202310)

The canonical Topic 6 axis K middle tier, wired as the unconditional
bounded pre-roll: `frame_await` and `frame_await_done` gained a
`spin_budget: USize` parameter (re-loading the wake word up to the
budget with `spin_loop` before the unchanged load-check-`atomic_wait`
park, so the lost-wakeup discipline is untouched), fed from a new
consumer-tunable `RunCfg::WAKE_SPIN_BUDGET` const at both scheduler
call sites. Zero restores park-immediately exactly; `await_exit` stays
park-only. The frame-protocol test sweeps budgets 0 and 128 and proves
identical results. The full `pick_tier` selection over
`predicted_wait_ns` telemetry remains the adapt-phase follow-up the
ledger names (a zero prediction would select the unbounded spin tier,
which must not guard a between-frames wait).

## The bench (wake_policy, `mock/benches/results/wake_policy/`)

Two arms on one carrier (`wake_common`): a persistent pool of real
threads parked on a real `PoolFrame` through the shipped frame
helpers; one timed call is one publish-to-done frame round trip with
`n` records of per-core fold work in between. The arms differ ONLY in
the budget (`wp_park` 0, `wp_spin` 128); identical work, strict
cross-variant validation green. Medians:

| n | wp_park | wp_spin | spin vs park |
|---|---|---|---|
| 64 | 7.40 us | 12.12 us | 1.64x |
| 1024 | 11.03 us | 14.73 us | 1.34x |
| 8192 | 28.70 us | 31.79 us | 1.11x |

## Reading

Park-immediately wins at every size. The ratio shrinking as the
in-frame compute grows is the signature of a fixed per-frame penalty
from the spin itself: the spinning side's repeated loads keep the
`seq` / `done` cache lines contended against the writer's
Release-store and the workers' done `fetch_add`s, and the spinning
core competes with the threads doing the actual frame work. On this
host the wake handoff through `atomic_wait` / `atomic_wake_all` is
cheap enough that burning cycles to dodge it costs more than the park
it dodges, in every measured regime including the
wake-latency-dominated small-frame one the tier targets.

## Ruling shape (per the topic's evidence exit)

The middle tier loses, so park-now is blessed by the numbers and the
tier stays tunable: the mechanism ships (the budget parameter and the
`WAKE_SPIN_BUDGET` const are the canonical machinery, covered by the
budget-sweep test), and the DEFAULT flips to 0 so the shipped
behaviour is the measured winner. A consumer on a host where the
trade runs the other way overrides the const, which is exactly the
toolbox-not-policer shape. The adapt-phase `pick_tier` follow-up
inherits this as its baseline: any future telemetry-driven selection
must beat park-immediately on this bench to earn a non-zero tier.

Registry shape at freeze: deviation 3 registers as
resolved-by-bench (park default, spin machinery shipped and tunable),
provenance this record, the round artifacts (202607202310), and the
committed wake_policy CSV/meta/findings trail.

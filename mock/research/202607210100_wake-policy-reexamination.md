# Wake Policy Re-Examined: the Round-1 Margins Were Inflated, the Ruling Survives

**Date:** 2026-07-19
**Status:** supersedes the mechanism story and the margin figures of
`202607202340_wake-policy-evidence.md` (which stays as paper trail). Op
flagged the round-1 result as too convenient (the already-shipped shape
winning at every size) and asked for a second look plus a differently
shaped bench. Both ran; this record carries the corrected evidence.

## The confound the second look found first

`core::hint::spin_loop` on aarch64 lowers to `ISB SY`. Measured on this
host: 9 to 20 ns per iteration, against roughly 0.4 ns for a plain
Acquire re-load. The round-1 spin arm therefore burned 1 to 2.5 us of
pipeline stall per wait before parking, an order of magnitude more than
the canonical Topic 6 budget model assumed per iteration. Round 1
compared "park" against "128 ISBs then park", not against a cheap
re-check tier, and its "wake-word cache-line contention" mechanism story
was at best partial.

## The round-2 bench (seven arms, three fresh invocations)

`wake_common` gained a parameterised wait (same atomics, same
`atomic_wait` park, spin iteration selectable): `wp_park` (shipped,
budget 0), `wp_lp0` (local budget 0, the control that must equal
wp_park), `wp_isb8` (8 hinted iterations, about 100 ns), `wp_nh128` /
`wp_nh2k` / `wp_nh8k` (plain-load spins, about 50 ns / 0.8 us / 3 us
windows), `wp_spin` (shipped hinted 128). The whole seven-arm bench ran
three times in fresh processes.

Findings, in order of importance:

1. **Per-process placement variance is the same magnitude as round 1's
   margins.** The two semantically identical control arms disagreed by
   up to 18 percent (n=64) and 32 percent (n=1024) in one invocation,
   and one invocation showed `wp_spin` best at n=8192, refuted by the
   other two (31.7 / 34.2 us vs park's 25.5 / 25.4 us). Each cdylib arm
   is its own process with its own leaked pool, so thread placement luck
   bakes a coherent offset into all of that arm's samples. Round 1's
   1.64x / 1.34x / 1.11x figures carried this inflation and are retired
   as point estimates.
2. **The direction is reproducible.** Across 7 policies, 3 sizes, and 3
   invocations, park-immediately is best or tied-best in every cell;
   no spin policy beats it anywhere. Short spins (isb8, nh128) run 8 to
   20 percent worse; microsecond windows (nh2k, nh8k, the hinted 128)
   run 1.3x to 2.4x worse at small n.
3. **Three mechanisms, now measured or reproduced.** The ISB cost above;
   spin loads contending the `seq` / `done` lines against the
   publisher's store and the workers' arrivals (plain-load spins lose
   dose-dependently, and the worker-side spin leaks contention into the
   next frame's publish); and a ulock wake path cheap enough that no
   spin window pays for itself on this host.

## Corrected ruling

The round-1 conclusion stands on better evidence: `WAKE_SPIN_BUDGET`
defaults to 0 (park-immediately), the tier stays consumer-tunable, and
the adapt-phase `pick_tier` follow-up inherits park as its bar. What
changes is the strength and framing: the ruling rests on "park is never
worse and spin is never better, reproducibly", not on the retired
decisive margins, and any future re-decision needs a paired
within-process oracle (the carrier's per-process placement variance
bounds what cross-process arms can resolve to roughly 10 to 30
percent). If the shipped spin tier is ever re-tuned, the iteration must
also drop the ISB hint or budget in wall-time, not iterations; 128 ISBs
is not the canonical cheap tier on aarch64.

Registry shape at freeze: deviation 3 stays resolved-by-bench with this
record as the controlling evidence, `202607202340` and the round-1 CSV
trail as history, and the round-2 seven-arm CSV/meta/findings committed
alongside.

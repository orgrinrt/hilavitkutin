# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (atomic_acquire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline atomic_acquire has the worst median (3.97 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest atomic_seqcst at 2.63 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### atomic_seqcst beats baseline by 39% (significant)

atomic_seqcst is -1.56 us (39%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (atomic_seqcst, atomic_relaxed) are a dead heat (<1%)

atomic_seqcst (2.63 us) and atomic_relaxed (2.63 us) differ by 0.03%, inside the noise, even though the wider field spreads 51.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: atomic_seqcst** at 2629.1 ns median (-33.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.51x (fastest 2629.1 ns, slowest 3971.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x0ddc0ffee055b011 |
| Counter frequency | 2112895079 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| atomic_acquire | 5683ns | 5725ns | 5104ns | 5682ns | 6265ns | base |
| atomic_relaxed | 4348ns | 4513ns | 3693ns | 4456ns | 4679ns | -23.49% |
| atomic_seqcst | 4317ns | 4511ns | 3694ns | 4448ns | 4545ns | -24.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 3965ns | 3565ns | 4369ns | base | 1.033 |
| atomic_relaxed | 2532ns | 2153ns | 2719ns | -36.15% | 1.618 |
| atomic_seqcst | 2513ns | 2152ns | 2642ns | -36.62% | 1.630 |

## Performance model

- Peak throughput: **1.903 Gops/s** (atomic_seqcst; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 1.031 | 54.2% |
| atomic_relaxed | 1.558 | 81.8% |
| atomic_seqcst | 1.558 | 81.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 5683ns | 5683ns | base |
| atomic_relaxed | 4348ns | 4348ns | -23.49% |
| atomic_seqcst | 4317ns | 4317ns | -24.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 3971ns | base | --- | [3575, 4357] | --- | --- | --- | --- |
| atomic_relaxed | 2630ns | -1554.1ns (-39.1%) | [-1724, -1412]ns | [2629, 2631] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 2629ns | -1564.9ns (-39.4%) | [-1726, -1412]ns | [2628, 2630] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 4359ns | -39.6% | -50.6% |
| 1 | 2 | 4358ns | -39.6% | -39.7% |
| 1 | 3 | 3566ns | -39.6% | -26.2% |
| 1 | 4 | 4358ns | -39.6% | -39.6% |
| 1 | 5 | 3566ns | -26.2% | -26.2% |
| 1 | 6 | 3565ns | -26.2% | -26.2% |
| 1 | 7 | 3566ns | -26.2% | -39.6% |
| 1 | 8 | 4357ns | -50.6% | -39.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | -0.125 | ok |
| atomic_relaxed | -0.211 | moderate- |
| atomic_seqcst | -0.207 | moderate- |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 21.8ns | 3965.4ns | 0.6% |  |
| atomic_relaxed | 22.9ns | 2532.1ns | 0.9% |  |
| atomic_seqcst | 22.7ns | 2513.1ns | 0.9% |  |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 3565.2-4369.2 ns)
   3565.2 |########################################
   3605.4 |
   3645.6 |
   3685.8 |
   3726.0 |
   3766.2 |
   3806.4 |
   3846.6 |
   3886.8 |
   3927.0 |
   3967.2 |
   4007.4 |
   4047.6 |
   4087.8 |
   4128.0 |
   4168.2 |
   4208.4 |
   4248.6 |
   4288.8 |
   4329.0 |#######################################
  (12 below, 13 above range)

atomic_relaxed (n=160, range 2152.7-2719.3 ns)
   2152.7 |########
   2181.0 |
   2209.4 |
   2237.7 |
   2266.0 |
   2294.4 |
   2322.7 |
   2351.0 |
   2379.4 |
   2407.7 |
   2436.0 |
   2464.4 |
   2492.7 |
   2521.0 |
   2549.4 |
   2577.7 |
   2606.0 |########################################
   2634.4 |##############
   2662.7 |
   2691.0 |
  (21 below, 1 above range)

atomic_seqcst (n=160, range 2152.4-2642.0 ns)
   2152.4 |########
   2176.9 |
   2201.4 |
   2225.9 |
   2250.3 |
   2274.8 |
   2299.3 |
   2323.8 |
   2348.3 |
   2372.8 |
   2397.2 |
   2421.7 |
   2446.2 |
   2470.7 |
   2495.2 |
   2519.6 |
   2544.1 |
   2568.6 |
   2593.1 |
   2617.6 |########################################
  (18 below, 15 above range)

```

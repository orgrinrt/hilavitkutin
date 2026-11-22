# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ema_scalar dominates: 66% faster than the next best (ema_autovec)

ema_scalar (1.76 us) leads ema_autovec (2.93 us) by 66%, a clear separation rather than a photo finish. CV 28.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_scalar beats baseline by 40% (significant)

ema_scalar is -1.17 us (40%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_scalar is fastest but the noisiest (CV 28.9%)

ema_scalar wins on median (1.76 us) yet has the highest variance (CV 28.9%), while ema_autovec is the steadiest (CV 7.9%, 2.93 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### ema_scalar is inconsistent: worst-20% is 1.6x its best-20%

ema_scalar's best 20% of batches run at 1.44 us but its worst 20% at 2.24 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ema_scalar** at 1759.3 ns median (-39.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.67x (fastest 1759.3 ns, slowest 2934.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | EMA streaming pass: NEON vs scalar vs autovec under realistic workload |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112807538 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ema_autovec | 4622ns | 4832ns | 3956ns | 4763ns | 4867ns | base |
| ema_neon | 4437ns | 4834ns | 3959ns | 4425ns | 4952ns | -4.00% |
| ema_scalar | 3644ns | 3643ns | 2981ns | 3519ns | 4679ns | -21.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 2797ns | 2394ns | 2940ns | base | 5.858 |
| ema_neon | 2688ns | 2401ns | 2995ns | -3.90% | 6.096 |
| ema_scalar | 1755ns | 1439ns | 2243ns | -37.24% | 9.334 |

## Performance model

- Peak throughput: **11.385 Gops/s** (ema_scalar; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 5.601 | 49.2% |
| ema_neon | 5.584 | 49.0% |
| ema_scalar | 9.313 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 4622ns | 4622ns | base |
| ema_neon | 4437ns | 4437ns | -4.00% |
| ema_scalar | 3644ns | 3644ns | -21.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 2925ns | base | --- | [2925, 2925] | --- | --- | --- | --- |
| ema_neon | 2934ns | no significant difference | [-2, +9]ns | [2411, 2935] | no | 0.9370 | 0.9370 | 0 |
| ema_scalar | 1759ns | -1165.2ns (-39.8%) | [-1167, -1164]ns | [1758, 1760] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 2925ns | -17.9% | -39.8% |
| 1 | 2 | 2393ns | +22.9% | -26.5% |
| 1 | 3 | 2925ns | -17.9% | -49.6% |
| 1 | 4 | 2393ns | +0.4% | -39.8% |
| 1 | 5 | 2925ns | +0.3% | -39.8% |
| 1 | 6 | 2931ns | -18.0% | -40.0% |
| 1 | 7 | 2927ns | +0.3% | -39.9% |
| 1 | 8 | 2931ns | +0.2% | -50.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.365 | moderate- |
| ema_neon | -0.378 | moderate- |
| ema_scalar | 0.043 | ok |

**Consistency summary:**

- **ema_neon**: won 3/8, lost 5/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 22.5ns | 2796.9ns | 0.8% |  |
| ema_neon | 21.4ns | 2687.7ns | 0.8% |  |
| ema_scalar | 24.2ns | 1755.3ns | 1.4% |  |

## Distribution (algo ns)

```
ema_autovec (n=160, range 2394.2-2940.2 ns)
   2394.2 |####
   2421.5 |
   2448.8 |
   2476.1 |
   2503.4 |
   2530.7 |
   2558.0 |
   2585.3 |
   2612.6 |
   2639.9 |
   2667.2 |
   2694.5 |
   2721.8 |
   2749.1 |
   2776.4 |
   2803.7 |
   2831.0 |
   2858.3 |
   2885.6 |
   2912.9 |########################################
  (27 below, 6 above range)

ema_neon (n=160, range 2401.4-2994.5 ns)
   2401.4 |########################################
   2431.0 |
   2460.7 |
   2490.3 |
   2520.0 |
   2549.6 |
   2579.3 |
   2609.0 |
   2638.6 |
   2668.3 |
   2697.9 |
   2727.6 |
   2757.2 |
   2786.9 |
   2816.6 |
   2846.2 |
   2875.9 |
   2905.5 |#############
   2935.2 |####################################
   2964.8 |
  (14 below, 2 above range)

ema_scalar (n=160, range 1439.1-2243.2 ns)
   1439.1 |######################
   1479.3 |
   1519.5 |#
   1559.7 |
   1599.9 |
   1640.1 |
   1680.3 |
   1720.5 |########################################
   1760.8 |######################
   1801.0 |
   1841.2 |
   1881.4 |
   1921.6 |
   1961.8 |
   2002.0 |
   2042.2 |
   2082.4 |
   2122.6 |
   2162.8 |
   2203.0 |#
  (13 below, 6 above range)

```

## Diagnostics

- **ema_scalar**: CV=28.9% (high variance, measurements may be unstable)

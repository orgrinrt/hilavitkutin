# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ema_scalar dominates: 66% faster than the next best (ema_autovec)

ema_scalar (446 ns) leads ema_autovec (742 ns) by 66%, a clear separation rather than a photo finish. CV 69.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_scalar beats baseline by 40% (significant)

ema_scalar is -296 ns (40%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_scalar is fastest but the noisiest (CV 69.4%)

ema_scalar wins on median (446 ns) yet has the highest variance (CV 69.4%), while ema_autovec is the steadiest (CV 61.0%, 742 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### ema_neon shows alternating (throttle bounce) (autocorr -0.68)

ema_neon's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (298 ns) is smaller than the fastest variant's own run-to-run std-dev (309 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### ema_scalar is inconsistent: worst-20% is 2.9x its best-20%

ema_scalar's best 20% of batches run at 365 ns but its worst 20% at 1.05 us (2.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ema_scalar** at 446.0 ns median (-39.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.67x (fastest 446.0 ns, slowest 744.4 ns)

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
| Counter frequency | 2112833612 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ema_autovec | 3114ns | 2634ns | 2175ns | 2633ns | 5495ns | base |
| ema_neon | 3054ns | 2633ns | 2153ns | 2546ns | 5477ns | -1.92% |
| ema_scalar | 2883ns | 2325ns | 1903ns | 2322ns | 5544ns | -7.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 874ns | 616ns | 1527ns | base | 4.686 |
| ema_neon | 861ns | 609ns | 1536ns | -1.55% | 4.759 |
| ema_scalar | 551ns | 365ns | 1053ns | -37.02% | 7.439 |

## Performance model

- Peak throughput: **11.223 Gops/s** (ema_scalar; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 5.519 | 49.2% |
| ema_neon | 5.502 | 49.0% |
| ema_scalar | 9.184 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 3114ns | 3114ns | base |
| ema_neon | 3054ns | 3054ns | -1.92% |
| ema_scalar | 2883ns | 2883ns | -7.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 742ns | base | --- | [742, 742] | --- | --- | --- | --- |
| ema_neon | 744ns | no significant difference | [-5, +2]ns | [744, 744] | no | 0.5801 | 0.5801 | 0 |
| ema_scalar | 446ns | -296.0ns (-39.9%) | [-297, -296]ns | [446, 446] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 609ns | +22.4% | -26.7% |
| 1 | 2 | 742ns | -17.9% | -50.8% |
| 1 | 3 | 742ns | +0.3% | -39.8% |
| 1 | 4 | 743ns | -18.0% | -40.0% |
| 1 | 5 | 742ns | +0.3% | -39.8% |
| 1 | 6 | 742ns | -17.9% | -39.9% |
| 1 | 7 | 743ns | +0.3% | -39.9% |
| 1 | 8 | 607ns | +22.6% | -39.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.043 | ok |
| ema_neon | -0.675 | HIGH- (thermal bounce) |
| ema_scalar | -0.211 | moderate- |

**Consistency summary:**

- **ema_neon**: won 3/8, lost 5/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 28.0ns | 874.1ns | 3.2% |  |
| ema_neon | 27.3ns | 860.6ns | 3.2% |  |
| ema_scalar | 30.1ns | 550.6ns | 5.5% | HIGH |

## Distribution (algo ns)

```
ema_autovec (n=160, range 616.1-1526.6 ns)
    616.1 |#
    661.6 |
    707.1 |########################################
    752.7 |###
    798.2 |
    843.7 |
    889.2 |
    934.8 |
    980.3 |
   1025.8 |
   1071.3 |
   1116.9 |
   1162.4 |
   1207.9 |
   1253.4 |
   1299.0 |
   1344.5 |
   1390.0 |
   1435.6 |
   1481.1 |
  (27 below, 17 above range)

ema_neon (n=160, range 609.1-1536.3 ns)
    609.1 |##################
    655.4 |
    701.8 |########################################
    748.1 |###
    794.5 |
    840.9 |
    887.2 |
    933.6 |
    979.9 |
   1026.3 |
   1072.7 |
   1119.0 |
   1165.4 |
   1211.7 |
   1258.1 |
   1304.5 |
   1350.8 |
   1397.2 |
   1443.5 |
   1489.9 |
  (13 below, 17 above range)

ema_scalar (n=160, range 365.0-1052.9 ns)
    365.0 |#######
    399.4 |
    433.8 |########################################
    468.2 |
    502.6 |
    536.9 |
    571.3 |
    605.7 |
    640.1 |
    674.5 |
    708.9 |
    743.3 |
    777.7 |
    812.1 |
    846.5 |
    880.9 |
    915.3 |
    949.7 |
    984.1 |
   1018.5 |
  (15 below, 20 above range)

```

## Diagnostics

- **ema_autovec**: CV=51.8% (high variance, measurements may be unstable)
- **ema_neon**: CV=53.1% (high variance, measurements may be unstable)
- **ema_scalar**: CV=56.2% (high variance, measurements may be unstable)

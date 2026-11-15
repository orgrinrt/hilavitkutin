# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ema_scalar dominates: 66% faster than the next best (ema_autovec)

ema_scalar (446 ns) leads ema_autovec (742 ns) by 66%, a clear separation rather than a photo finish. CV 74.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_scalar beats baseline by 40% (significant)

ema_scalar is -298 ns (40%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_scalar is fastest but the noisiest (CV 74.6%)

ema_scalar wins on median (446 ns) yet has the highest variance (CV 74.6%), while ema_autovec is the steadiest (CV 50.3%, 742 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### ema_autovec shows alternating (throttle bounce) (autocorr -0.59)

ema_autovec's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (298 ns) is smaller than the fastest variant's own run-to-run std-dev (333 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### ema_scalar is inconsistent: worst-20% is 3.2x its best-20%

ema_scalar's best 20% of batches run at 370 ns but its worst 20% at 1.18 us (3.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ema_scalar** at 446.3 ns median (-39.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.67x (fastest 446.3 ns, slowest 744.4 ns)

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
| Counter frequency | 2112781281 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ema_autovec | 2977ns | 2629ns | 2622ns | 2631ns | 4371ns | base |
| ema_neon | 3206ns | 2629ns | 2150ns | 2659ns | 5906ns | +7.70% |
| ema_scalar | 3024ns | 2324ns | 1931ns | 2326ns | 6212ns | +1.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 839ns | 741ns | 1225ns | base | 4.881 |
| ema_neon | 886ns | 609ns | 1603ns | +5.56% | 4.624 |
| ema_scalar | 578ns | 370ns | 1181ns | -31.13% | 7.087 |

## Performance model

- Peak throughput: **11.070 Gops/s** (ema_scalar; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 5.518 | 49.8% |
| ema_neon | 5.502 | 49.7% |
| ema_scalar | 9.178 | 82.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 2977ns | 2977ns | base |
| ema_neon | 3206ns | 3206ns | +7.70% |
| ema_scalar | 3024ns | 3024ns | +1.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 742ns | base | --- | [742, 744] | --- | --- | --- | --- |
| ema_neon | 744ns | no significant difference | [-2, +2]ns | [744, 744] | no | 0.3846 | 0.3846 | 0 |
| ema_scalar | 446ns | -297.9ns (-40.1%) | [-299, -296]ns | [446, 446] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 742ns | -17.9% | -39.8% |
| 1 | 2 | 744ns | +0.1% | -40.0% |
| 1 | 3 | 745ns | -18.2% | -51.0% |
| 1 | 4 | 742ns | +0.4% | -50.8% |
| 1 | 5 | 746ns | -0.2% | -40.1% |
| 1 | 6 | 742ns | +28.5% | -39.8% |
| 1 | 7 | 743ns | +0.2% | -39.9% |
| 1 | 8 | 743ns | -18.0% | -40.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.594 | HIGH- (thermal bounce) |
| ema_neon | 0.012 | ok |
| ema_scalar | 0.292 | moderate+ |

**Consistency summary:**

- **ema_neon**: won 4/8, lost 4/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 26.9ns | 839.2ns | 3.2% |  |
| ema_neon | 29.2ns | 885.9ns | 3.3% |  |
| ema_scalar | 31.5ns | 577.9ns | 5.4% | HIGH |

## Distribution (algo ns)

```
ema_autovec (n=160, range 740.9-1224.9 ns)
    740.9 |########################################
    765.1 |
    789.3 |
    813.5 |
    837.7 |
    861.9 |
    886.1 |
    910.3 |
    934.5 |
    958.7 |
    982.9 |
   1007.1 |
   1031.3 |
   1055.5 |
   1079.7 |
   1103.9 |
   1128.1 |
   1152.3 |
   1176.5 |
   1200.7 |
  (12 below, 10 above range)

ema_neon (n=160, range 609.1-1602.6 ns)
    609.1 |######################
    658.7 |
    708.4 |########################################
    758.1 |
    807.8 |
    857.4 |
    907.1 |#########
    956.8 |#
   1006.5 |
   1056.1 |
   1105.8 |
   1155.5 |
   1205.2 |
   1254.8 |
   1304.5 |
   1354.2 |
   1403.9 |
   1453.5 |
   1503.2 |
   1552.9 |
  (12 below, 16 above range)

ema_scalar (n=160, range 370.0-1180.7 ns)
    370.0 |
    410.6 |########################################
    451.1 |#
    491.6 |
    532.2 |
    572.7 |
    613.2 |
    653.8 |
    694.3 |
    734.8 |
    775.4 |
    815.9 |
    856.4 |
    897.0 |
    937.5 |
    978.1 |
   1018.6 |
   1059.1 |
   1099.7 |
   1140.2 |
  (29 below, 18 above range)

```

## Diagnostics

- **ema_autovec**: CV=44.4% (high variance, measurements may be unstable)
- **ema_neon**: CV=50.2% (high variance, measurements may be unstable)
- **ema_scalar**: CV=57.6% (high variance, measurements may be unstable)
- **ema_scalar**: worst_20/best_20 = 3.2x (possible bimodal distribution)

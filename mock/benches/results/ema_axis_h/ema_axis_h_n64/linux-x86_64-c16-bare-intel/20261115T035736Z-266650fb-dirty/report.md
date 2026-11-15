# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (11 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader ema_neon vs stability leader ema_autovec (+6% speed for 1.0x steadier)

ema_neon is fastest (12 ns, CV 92.8%); ema_autovec gives up 5.7% median for 1.0x lower variance (CV 91.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### ema_autovec is inconsistent: worst-20% is 3.9x its best-20%

ema_autovec's best 20% of batches run at 11 ns but its worst 20% at 41 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ema_neon** at 12.3 ns median (-5.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.20x (fastest 12.3 ns, slowest 14.8 ns)

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
| ema_autovec | 2748ns | 1889ns | 1544ns | 2059ns | 6018ns | base |
| ema_neon | 2791ns | 1890ns | 1598ns | 2064ns | 6166ns | +1.58% |
| ema_scalar | 2837ns | 1893ns | 1886ns | 2066ns | 6099ns | +3.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 19ns | 11ns | 41ns | base | 3.378 |
| ema_neon | 18ns | 10ns | 39ns | -5.01% | 3.556 |
| ema_scalar | 22ns | 15ns | 49ns | +18.32% | 2.855 |

## Performance model

- Peak throughput: **6.148 Gops/s** (ema_neon; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 4.923 | 80.1% |
| ema_neon | 5.203 | 84.6% |
| ema_scalar | 4.324 | 70.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 2748ns | 2748ns | base |
| ema_neon | 2791ns | 2791ns | +1.58% |
| ema_scalar | 2837ns | 2837ns | +3.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 13ns | base | --- | [13, 13] | --- | --- | --- | --- |
| ema_neon | 12ns | -0.6ns (-4.6%) | [-1, -0]ns | [12, 12] | YES | 0.0140 | 0.0140 | 0 |
| ema_scalar | 15ns | +4.0ns (+30.8%) | [+2, +4]ns | [15, 15] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 21ns | -52.8% | +76.9% |
| 1 | 2 | 13ns | -5.4% | +14.6% |
| 1 | 3 | 11ns | -5.1% | +38.3% |
| 1 | 4 | 11ns | +16.0% | +39.6% |
| 1 | 5 | 13ns | -5.4% | +14.2% |
| 1 | 6 | 11ns | +15.0% | +38.3% |
| 1 | 7 | 11ns | +15.0% | +38.3% |
| 1 | 8 | 13ns | -5.4% | +15.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | 0.106 | ok |
| ema_neon | -0.206 | moderate- |
| ema_scalar | -0.014 | ok |

**Consistency summary:**

- **ema_neon**: won 5/8, lost 3/8
- **ema_scalar**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 34.5ns | 18.9ns | 182.1% | HIGH |
| ema_neon | 34.8ns | 18.0ns | 193.5% | HIGH |
| ema_scalar | 35.6ns | 22.4ns | 158.6% | HIGH |

## Distribution (algo ns)

```
ema_autovec (n=160, range 10.6-41.2 ns)
     10.6 |########################################
     12.1 |######################################
     13.6 |
     15.2 |
     16.7 |
     18.2 |
     19.7 |
     21.3 |############
     22.8 |
     24.3 |
     25.9 |
     27.4 |
     28.9 |
     30.4 |#
     32.0 |
     33.5 |#
     35.0 |###
     36.6 |#
     38.1 |#####
     39.6 |#
  (9 below, 14 above range)

ema_neon (n=160, range 10.4-39.4 ns)
     10.4 |
     11.9 |########################################
     13.3 |
     14.8 |
     16.2 |
     17.6 |
     19.1 |
     20.5 |
     22.0 |
     23.4 |
     24.9 |
     26.3 |#
     27.8 |
     29.2 |#
     30.7 |#
     32.1 |#
     33.6 |#
     35.0 |
     36.5 |
     37.9 |
  (27 below, 18 above range)

ema_scalar (n=160, range 14.7-48.9 ns)
     14.7 |########################################
     16.4 |
     18.1 |
     19.8 |
     21.5 |
     23.2 |
     25.0 |
     26.7 |
     28.4 |
     30.1 |
     31.8 |
     33.5 |
     35.2 |
     36.9 |#
     38.6 |
     40.4 |#
     42.1 |
     43.8 |#
     45.5 |
     47.2 |#
  (5 below, 12 above range)

```

## Diagnostics

- **ema_autovec**: CV=63.0% (high variance, measurements may be unstable)
- **ema_autovec**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **ema_autovec**: bridge=180.8% of algo (FFI overhead may distort results)
- **ema_neon**: CV=63.4% (high variance, measurements may be unstable)
- **ema_neon**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **ema_neon**: bridge=191.1% of algo (FFI overhead may distort results)
- **ema_scalar**: CV=66.2% (high variance, measurements may be unstable)
- **ema_scalar**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **ema_scalar**: bridge=162.2% of algo (FFI overhead may distort results)

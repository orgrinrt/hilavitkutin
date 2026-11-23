# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ema_neon dominates: 27% faster than the next best (ema_autovec)

ema_neon (10 ns) leads ema_autovec (13 ns) by 27%, a clear separation rather than a photo finish. CV 104.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_neon beats baseline by 22% (significant)

ema_neon is -3 ns (22%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_neon is fastest but the noisiest (CV 104.2%)

ema_neon wins on median (10 ns) yet has the highest variance (CV 104.2%), while ema_scalar is the steadiest (CV 85.4%, 15 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (5 ns) is smaller than the fastest variant's own run-to-run std-dev (11 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### ema_autovec is inconsistent: worst-20% is 3.7x its best-20%

ema_autovec's best 20% of batches run at 11 ns but its worst 20% at 41 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ema_neon** at 10.2 ns median (-21.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.46x (fastest 10.2 ns, slowest 14.9 ns)

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
| ema_autovec | 2672ns | 1891ns | 1591ns | 1917ns | 6020ns | base |
| ema_neon | 2637ns | 1560ns | 1543ns | 1958ns | 5767ns | -1.34% |
| ema_scalar | 2835ns | 1896ns | 1597ns | 2247ns | 5837ns | +6.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 18ns | 11ns | 41ns | base | 3.504 |
| ema_neon | 17ns | 10ns | 36ns | -7.86% | 3.803 |
| ema_scalar | 22ns | 13ns | 45ns | +21.16% | 2.892 |

## Performance model

- Peak throughput: **6.368 Gops/s** (ema_neon; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 4.923 | 77.3% |
| ema_neon | 6.275 | 98.5% |
| ema_scalar | 4.295 | 67.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 2672ns | 2672ns | base |
| ema_neon | 2637ns | 2637ns | -1.34% |
| ema_scalar | 2835ns | 2835ns | +6.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 13ns | base | --- | [13, 13] | --- | --- | --- | --- |
| ema_neon | 10ns | -2.8ns (-21.5%) | [-3, -2]ns | [10, 12] | YES | 0.0000 | 0.0000 | 0 |
| ema_scalar | 15ns | +1.8ns (+13.8%) | [+2, +2]ns | [15, 15] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 11ns | -5.6% | +14.0% |
| 1 | 2 | 11ns | +15.0% | +14.0% |
| 1 | 3 | 13ns | -22.3% | +14.6% |
| 1 | 4 | 13ns | -5.4% | +94.6% |
| 1 | 5 | 13ns | -21.5% | +14.6% |
| 1 | 6 | 13ns | -22.6% | +13.4% |
| 1 | 7 | 13ns | -21.5% | +14.6% |
| 1 | 8 | 13ns | -21.9% | +14.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | 0.461 | moderate+ |
| ema_neon | -0.398 | moderate- |
| ema_scalar | 0.019 | ok |

**Consistency summary:**

- **ema_neon**: won 7/8, lost 1/8
- **ema_scalar**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 33.2ns | 18.3ns | 181.8% | HIGH |
| ema_neon | 32.2ns | 16.8ns | 191.4% | HIGH |
| ema_scalar | 35.2ns | 22.1ns | 159.3% | HIGH |

## Distribution (algo ns)

```
ema_autovec (n=160, range 10.9-40.9 ns)
     10.9 |
     12.4 |########################################
     13.9 |#
     15.4 |
     16.9 |
     18.4 |
     19.9 |
     21.4 |
     22.9 |
     24.4 |
     25.9 |
     27.4 |
     28.9 |
     30.4 |
     31.9 |
     33.4 |#
     34.9 |#
     36.4 |
     37.9 |#
     39.4 |#
  (27 below, 14 above range)

ema_neon (n=160, range 10.1-35.8 ns)
     10.1 |########################################
     11.3 |#################
     12.6 |
     13.9 |
     15.2 |
     16.5 |
     17.8 |
     19.1 |
     20.4 |
     21.7 |
     22.9 |
     24.2 |
     25.5 |
     26.8 |#
     28.1 |
     29.4 |####
     30.7 |##
     32.0 |
     33.3 |####
     34.6 |#
  (15 below, 11 above range)

ema_scalar (n=160, range 12.5-45.0 ns)
     12.5 |
     14.2 |########################################
     15.8 |#
     17.4 |#
     19.0 |#
     20.7 |#
     22.3 |
     23.9 |#
     25.5 |#
     27.2 |
     28.8 |
     30.4 |
     32.0 |##
     33.7 |
     35.3 |##
     36.9 |##
     38.5 |#
     40.2 |#
     41.8 |
     43.4 |###
  (27 below, 14 above range)

```

## Diagnostics

- **ema_autovec**: CV=64.2% (high variance, measurements may be unstable)
- **ema_autovec**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **ema_autovec**: bridge=181.5% of algo (FFI overhead may distort results)
- **ema_neon**: CV=63.2% (high variance, measurements may be unstable)
- **ema_neon**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **ema_neon**: bridge=191.2% of algo (FFI overhead may distort results)
- **ema_scalar**: CV=57.5% (high variance, measurements may be unstable)
- **ema_scalar**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **ema_scalar**: bridge=159.4% of algo (FFI overhead may distort results)

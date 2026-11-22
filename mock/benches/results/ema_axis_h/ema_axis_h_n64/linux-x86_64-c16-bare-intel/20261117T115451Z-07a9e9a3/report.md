# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ema_scalar shows warm-up / thermal drift (autocorr +0.59)

ema_scalar's per-pass series has lag-1 autocorrelation +0.59, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (11 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### ema_autovec is inconsistent: worst-20% is 3.8x its best-20%

ema_autovec's best 20% of batches run at 11 ns but its worst 20% at 40 ns (3.8x) - a bimodal or bursty profile the median hides.

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
| Counter frequency | 2112807538 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ema_autovec | 2692ns | 1894ns | 1550ns | 2000ns | 5908ns | base |
| ema_neon | 2676ns | 1895ns | 1549ns | 2001ns | 5829ns | -0.57% |
| ema_scalar | 2725ns | 1898ns | 1551ns | 2103ns | 5768ns | +1.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 18ns | 11ns | 40ns | base | 3.485 |
| ema_neon | 17ns | 10ns | 37ns | -5.45% | 3.686 |
| ema_scalar | 21ns | 12ns | 44ns | +14.98% | 3.031 |

## Performance model

- Peak throughput: **6.362 Gops/s** (ema_neon; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 4.923 | 77.4% |
| ema_neon | 5.203 | 81.8% |
| ema_scalar | 4.324 | 68.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 2692ns | 2692ns | base |
| ema_neon | 2676ns | 2676ns | -0.57% |
| ema_scalar | 2725ns | 2725ns | +1.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 13ns | base | --- | [11, 13] | --- | --- | --- | --- |
| ema_neon | 12ns | -0.5ns (-3.8%) | [-1, -0]ns | [11, 12] | YES | 0.0001 | 0.0001 | 3 |
| ema_scalar | 15ns | +1.6ns (+12.3%) | [+2, +2]ns | [14, 15] | YES | 0.0001 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 13ns | -22.1% | -6.9% |
| 1 | 2 | 13ns | -4.6% | -6.2% |
| 1 | 3 | 11ns | -0.9% | +14.0% |
| 1 | 4 | 11ns | -5.6% | +15.4% |
| 1 | 5 | 13ns | -22.6% | -6.1% |
| 1 | 6 | 11ns | +15.4% | +38.8% |
| 1 | 7 | 11ns | +15.9% | +39.3% |
| 1 | 8 | 11ns | -6.0% | +38.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | 0.059 | ok |
| ema_neon | -0.129 | ok |
| ema_scalar | 0.594 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **ema_neon**: won 6/8, lost 2/8
- **ema_scalar**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 33.3ns | 18.4ns | 181.4% | HIGH |
| ema_neon | 33.6ns | 17.4ns | 193.4% | HIGH |
| ema_scalar | 33.9ns | 21.1ns | 160.5% | HIGH |

## Distribution (algo ns)

```
ema_autovec (n=160, range 10.6-40.1 ns)
     10.6 |########################################
     12.1 |##########################
     13.5 |
     15.0 |
     16.5 |
     18.0 |
     19.4 |
     20.9 |
     22.4 |
     23.9 |
     25.3 |#
     26.8 |
     28.3 |#
     29.8 |#
     31.2 |
     32.7 |#
     34.2 |#
     35.7 |#
     37.1 |##
     38.6 |####
  (3 below, 15 above range)

ema_neon (n=160, range 10.1-37.5 ns)
     10.1 |########################################
     11.4 |##################################
     12.8 |
     14.2 |
     15.5 |
     16.9 |
     18.3 |#
     19.7 |
     21.0 |
     22.4 |
     23.8 |
     25.1 |
     26.5 |#
     27.9 |#
     29.3 |##
     30.6 |####
     32.0 |#
     33.4 |##
     34.7 |##
     36.1 |##
  (13 below, 12 above range)

ema_scalar (n=160, range 12.1-44.1 ns)
     12.1 |########################################
     13.7 |##############################
     15.3 |
     16.9 |
     18.5 |
     20.1 |
     21.7 |
     23.3 |
     24.9 |
     26.5 |
     28.1 |
     29.7 |
     31.3 |#
     32.9 |
     34.5 |###
     36.1 |#
     37.7 |#####
     39.3 |####
     40.9 |#
     42.5 |#
  (3 below, 14 above range)

```

## Diagnostics

- **ema_autovec**: CV=64.0% (high variance, measurements may be unstable)
- **ema_autovec**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **ema_autovec**: bridge=180.8% of algo (FFI overhead may distort results)
- **ema_neon**: CV=63.7% (high variance, measurements may be unstable)
- **ema_neon**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **ema_neon**: bridge=191.1% of algo (FFI overhead may distort results)
- **ema_scalar**: CV=62.0% (high variance, measurements may be unstable)
- **ema_scalar**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **ema_scalar**: autocorrelation=0.59 (measurement drift or warm-up artifact)
- **ema_scalar**: bridge=159.5% of algo (FFI overhead may distort results)

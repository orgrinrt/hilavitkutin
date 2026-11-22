# Intra-resource member gather at M=64: contiguous blob vs scattered columns

2 variants, 120 samples per variant.
Baseline: **wide_blob**

## Highlights

Baseline for all deltas below: **wide_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide_blob dominates: 198% faster than the next best (wide_decomposed)

wide_blob (46 ns) leads wide_decomposed (136 ns) by 198%, a clear separation rather than a photo finish. CV 14091.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wide_blob is fastest but the noisiest (CV 14091.1%)

wide_blob wins on median (46 ns) yet has the highest variance (CV 14091.1%), while wide_decomposed is the steadiest (CV 3.1%, 136 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (91 ns) is smaller than the fastest variant's own run-to-run std-dev (6.45 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (wide_blob)

The baseline wide_blob is the fastest (46 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### wide_blob is inconsistent: worst-20% is 66.6x its best-20%

wide_blob's best 20% of batches run at 45 ns but its worst 20% at 3.01 us (66.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (wide_blob) is the fastest** at 45.8 ns median
- 1 variant significantly slower than baseline
- Spread: 2.98x (fastest 45.8 ns, slowest 136.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Intra-resource member gather at M=64: contiguous blob vs scattered columns |
| Passes | 5 |
| Runs per pass | 2000 |
| Batch size | 250 |
| Harness runs | 3 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide_blob | 699ns | 93ns | 92ns | 93ns | 3125ns | base |
| wide_decomposed | 180ns | 178ns | 176ns | 178ns | 186ns | -74.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide_blob | 638ns | 45ns | 3006ns | base | 6576.962 |
| wide_decomposed | 138ns | 135ns | 144ns | -78.41% | 30465.624 |

## Performance model

- Peak throughput: **92982.908 Gops/s** (wide_blob; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide_blob | 91578.690 | 98.5% |
| wide_decomposed | 30772.590 | 33.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide_blob | 699ns | 699ns | base |
| wide_decomposed | 180ns | 180ns | -74.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide_blob | 46ns | base | --- | [46, 46] | --- | --- | --- | --- |
| wide_decomposed | 136ns | +90.5ns (+197.6%) | [+90, +91]ns | [136, 137] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide_blob | wide_decomposed |
|---|---|---|---|
| 1 | 1 | 46ns | +198.4% |
| 1 | 2 | 46ns | +191.6% |
| 1 | 3 | 46ns | +196.3% |
| 1 | 4 | 46ns | +205.3% |
| 1 | 5 | 46ns | +195.0% |
| 2 | 1 | 46ns | +197.5% |
| 2 | 2 | 46ns | +205.3% |
| 2 | 3 | 46ns | +201.2% |
| 2 | 4 | 50ns | +171.6% |
| 2 | 5 | 46ns | +196.7% |
| 3 | 1 | 46ns | +230.5% |
| 3 | 2 | 46ns | +198.8% |
| 3 | 3 | 46ns | +196.7% |
| 3 | 4 | 46ns | +197.5% |
| 3 | 5 | 46ns | +198.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide_blob | -0.029 | ok |
| wide_decomposed | -0.146 | ok |

**Consistency summary:**

- **wide_decomposed**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide_blob | 151.1ns | 637.7ns | 23.7% | HIGH |
| wide_decomposed | 294.4ns | 137.7ns | 213.8% | HIGH |

## Distribution (algo ns)

```
wide_blob (n=120, range 45.1-3005.9 ns)
     45.1 |########################################
    193.1 |
    341.2 |
    489.2 |
    637.3 |
    785.3 |
    933.3 |
   1081.4 |
   1229.4 |
   1377.5 |
   1525.5 |
   1673.5 |
   1821.6 |
   1969.6 |
   2117.6 |
   2265.7 |
   2413.7 |
   2561.8 |
   2709.8 |
   2857.8 |
  (9 below, 1 above range)

wide_decomposed (n=120, range 135.1-143.8 ns)
    135.1 |########################################
    135.6 |########################################
    136.0 |#####################################
    136.4 |#####################################
    136.9 |###############################
    137.3 |
    137.7 |####
    138.2 |##
    138.6 |########
    139.1 |####
    139.5 |##
    139.9 |######
    140.4 |####
    140.8 |########
    141.2 |
    141.7 |##
    142.1 |
    142.5 |
    143.0 |
    143.4 |
  (9 below, 7 above range)

```

## Diagnostics

- **wide_blob**: CV=1012.0% (high variance, measurements may be unstable)
- **wide_blob**: worst_20/best_20 = 66.6x (possible bimodal distribution)
- **wide_blob**: bridge=274.7% of algo (FFI overhead may distort results)
- **wide_decomposed**: bridge=213.0% of algo (FFI overhead may distort results)

# Intra-resource member gather at M=64: contiguous blob vs scattered columns

2 variants, 120 samples per variant.
Baseline: **wide_blob**

## Highlights

Baseline for all deltas below: **wide_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (10 ns) is smaller than the fastest variant's own run-to-run std-dev (85 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (wide_blob)

The baseline wide_blob is the fastest (183 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### wide_decomposed is inconsistent: worst-20% is 2.2x its best-20%

wide_decomposed's best 20% of batches run at 192 ns but its worst 20% at 428 ns (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (wide_blob) is the fastest** at 182.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.05x (fastest 182.6 ns, slowest 192.2 ns)

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
| Counter frequency | 2112727760 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide_blob | 258ns | 222ns | 222ns | 223ns | 401ns | base |
| wide_decomposed | 292ns | 232ns | 231ns | 234ns | 526ns | +13.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide_blob | 210ns | 182ns | 320ns | base | 19969.865 |
| wide_decomposed | 239ns | 192ns | 428ns | +13.95% | 17524.703 |

## Performance model

- Peak throughput: **23003.495 Gops/s** (wide_blob; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide_blob | 22969.901 | 99.9% |
| wide_decomposed | 21822.601 | 94.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide_blob | 258ns | 258ns | base |
| wide_decomposed | 292ns | 292ns | +13.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide_blob | 183ns | base | --- | [183, 183] | --- | --- | --- | --- |
| wide_decomposed | 192ns | +9.5ns (+5.2%) | [+10, +10]ns | [192, 192] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide_blob | wide_decomposed |
|---|---|---|---|
| 1 | 1 | 183ns | +5.1% |
| 1 | 2 | 183ns | +5.1% |
| 1 | 3 | 183ns | +5.3% |
| 1 | 4 | 182ns | +163.7% |
| 1 | 5 | 502ns | -61.7% |
| 2 | 1 | 183ns | +5.1% |
| 2 | 2 | 183ns | +5.2% |
| 2 | 3 | 183ns | +5.2% |
| 2 | 4 | 182ns | +5.2% |
| 2 | 5 | 183ns | +5.2% |
| 3 | 1 | 183ns | +5.0% |
| 3 | 2 | 182ns | +5.3% |
| 3 | 3 | 183ns | +5.3% |
| 3 | 4 | 183ns | +174.7% |
| 3 | 5 | 183ns | +5.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide_blob | -0.076 | ok |
| wide_decomposed | -0.163 | ok |

**Consistency summary:**

- **wide_decomposed**: won 1/15, lost 14/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide_blob | 99.1ns | 210.0ns | 47.2% | HIGH |
| wide_decomposed | 375.2ns | 239.3ns | 156.8% | HIGH |

## Distribution (algo ns)

```
wide_blob (n=120, range 182.3-319.7 ns)
    182.3 |########################################
    189.2 |
    196.1 |
    202.9 |
    209.8 |
    216.7 |
    223.6 |
    230.4 |
    237.3 |
    244.2 |
    251.0 |
    257.9 |
    264.8 |
    271.7 |
    278.5 |
    285.4 |
    292.3 |
    299.1 |
    306.0 |
    312.9 |
  (12 below, 8 above range)

wide_decomposed (n=120, range 191.9-428.2 ns)
    191.9 |########################################
    203.7 |
    215.5 |
    227.4 |
    239.2 |
    251.0 |
    262.8 |
    274.6 |
    286.4 |
    298.2 |
    310.1 |
    321.9 |
    333.7 |
    345.5 |
    357.3 |
    369.1 |
    380.9 |
    392.7 |
    404.6 |
    416.4 |
  (20 below, 16 above range)

```

## Diagnostics

- **wide_blob**: CV=40.7% (high variance, measurements may be unstable)
- **wide_blob**: bridge=47.2% of algo (FFI overhead may distort results)
- **wide_decomposed**: CV=45.7% (high variance, measurements may be unstable)
- **wide_decomposed**: bridge=156.8% of algo (FFI overhead may distort results)

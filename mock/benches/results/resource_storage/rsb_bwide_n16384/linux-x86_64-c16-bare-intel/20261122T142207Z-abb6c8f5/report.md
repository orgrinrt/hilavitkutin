# Intra-resource member gather at M=64: contiguous blob vs scattered columns

2 variants, 120 samples per variant.
Baseline: **wide_blob**

## Highlights

Baseline for all deltas below: **wide_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.08 us) is smaller than the fastest variant's own run-to-run std-dev (1.13 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (wide_blob)

The baseline wide_blob is the fastest (13.14 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (wide_blob) is the fastest** at 13136.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.08x (fastest 13136.9 ns, slowest 14212.5 ns)

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
| Counter frequency | 2112835454 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide_blob | 12394ns | 13187ns | 10787ns | 12656ns | 13213ns | base |
| wide_decomposed | 13086ns | 14267ns | 11674ns | 13125ns | 14380ns | +5.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide_blob | 12344ns | 10746ns | 13155ns | base | 1.327 |
| wide_decomposed | 13033ns | 11629ns | 14319ns | +5.58% | 1.257 |

## Performance model

- Peak throughput: **1.525 Gops/s** (wide_blob; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide_blob | 1.247 | 81.8% |
| wide_decomposed | 1.153 | 75.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide_blob | 12394ns | 12394ns | base |
| wide_decomposed | 13086ns | 13086ns | +5.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide_blob | 13137ns | base | --- | [13136, 13138] | --- | --- | --- | --- |
| wide_decomposed | 14212ns | +1069.7ns (+8.1%) | [+883, +1075]ns | [11634, 14214] | YES | 0.0003 | 0.0003 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide_blob | wide_decomposed |
|---|---|---|---|
| 1 | 1 | 13137ns | +8.2% |
| 1 | 2 | 10748ns | +8.2% |
| 1 | 3 | 10748ns | +32.3% |
| 1 | 4 | 10748ns | +8.2% |
| 1 | 5 | 13138ns | +8.2% |
| 2 | 1 | 13140ns | +8.2% |
| 2 | 2 | 13138ns | -11.5% |
| 2 | 3 | 10749ns | +32.3% |
| 2 | 4 | 13138ns | -11.4% |
| 2 | 5 | 13140ns | -11.5% |
| 3 | 1 | 10748ns | +32.3% |
| 3 | 2 | 13143ns | +8.2% |
| 3 | 3 | 13137ns | -11.5% |
| 3 | 4 | 13137ns | -11.5% |
| 3 | 5 | 13139ns | +8.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide_blob | 0.067 | ok |
| wide_decomposed | -0.398 | moderate- |

**Consistency summary:**

- **wide_decomposed**: won 5/15, lost 10/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide_blob | 100.9ns | 12344.4ns | 0.8% |  |
| wide_decomposed | 338.6ns | 13033.0ns | 2.6% |  |

## Distribution (algo ns)

```
wide_blob (n=120, range 10746.1-13155.4 ns)
  10746.1 |##############
  10866.5 |
  10987.0 |
  11107.5 |
  11227.9 |
  11348.4 |
  11468.9 |
  11589.3 |
  11709.8 |
  11830.3 |
  11950.7 |
  12071.2 |
  12191.7 |
  12312.1 |
  12432.6 |
  12553.1 |
  12673.5 |
  12794.0 |
  12914.5 |
  13035.0 |########################################
  (12 below, 3 above range)

wide_decomposed (n=120, range 11628.8-14319.0 ns)
  11628.8 |################################
  11763.4 |
  11897.9 |
  12032.4 |
  12166.9 |
  12301.4 |
  12435.9 |
  12570.4 |
  12704.9 |
  12839.4 |
  12973.9 |
  13108.4 |
  13242.9 |
  13377.4 |
  13511.9 |
  13646.4 |
  13780.9 |
  13915.5 |
  14050.0 |
  14184.5 |########################################
  (4 below, 1 above range)

```

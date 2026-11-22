# Intra-resource member gather at M=64: contiguous blob vs scattered columns

2 variants, 120 samples per variant.
Baseline: **wide_blob**

## Highlights

Baseline for all deltas below: **wide_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide_blob dominates: 90% faster than the next best (wide_decomposed)

wide_blob (16.85 us) leads wide_decomposed (32.01 us) by 90%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (wide_blob)

The baseline wide_blob is the fastest (16.85 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (wide_blob) is the fastest** at 16848.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.90x (fastest 16848.2 ns, slowest 32010.0 ns)

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
| wide_blob | 17052ns | 16901ns | 16877ns | 16927ns | 17602ns | base |
| wide_decomposed | 32345ns | 32056ns | 31965ns | 32175ns | 33234ns | +89.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide_blob | 16998ns | 16827ns | 17540ns | base | 0.015 |
| wide_decomposed | 32299ns | 31920ns | 33185ns | +90.01% | 0.008 |

## Performance model

- Peak throughput: **0.015 Gops/s** (wide_blob; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide_blob | 0.015 | 99.9% |
| wide_decomposed | 0.008 | 52.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide_blob | 17052ns | 17052ns | base |
| wide_decomposed | 32345ns | 32345ns | +89.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide_blob | 16848ns | base | --- | [16846, 16863] | --- | --- | --- | --- |
| wide_decomposed | 32010ns | +15134.7ns (+89.8%) | [+15113, +15234]ns | [31973, 32128] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide_blob | wide_decomposed |
|---|---|---|---|
| 1 | 1 | 16855ns | +89.4% |
| 1 | 2 | 16846ns | +90.1% |
| 1 | 3 | 16861ns | +96.1% |
| 1 | 4 | 17499ns | +83.6% |
| 1 | 5 | 16844ns | +89.6% |
| 2 | 1 | 16842ns | +93.2% |
| 2 | 2 | 17162ns | +87.1% |
| 2 | 3 | 16843ns | +90.5% |
| 2 | 4 | 16934ns | +88.7% |
| 2 | 5 | 17352ns | +84.1% |
| 3 | 1 | 16834ns | +90.3% |
| 3 | 2 | 16841ns | +93.2% |
| 3 | 3 | 16902ns | +89.0% |
| 3 | 4 | 16859ns | +89.6% |
| 3 | 5 | 16834ns | +93.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide_blob | -0.229 | moderate- |
| wide_decomposed | -0.185 | ok |

**Consistency summary:**

- **wide_decomposed**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide_blob | 134.4ns | 16997.9ns | 0.8% |  |
| wide_decomposed | 330.8ns | 32298.6ns | 1.0% |  |

## Distribution (algo ns)

```
wide_blob (n=120, range 16827.1-17539.9 ns)
  16827.1 |########################################
  16862.7 |#########
  16898.4 |#
  16934.0 |###
  16969.7 |
  17005.3 |
  17040.9 |
  17076.6 |
  17112.2 |
  17147.9 |
  17183.5 |
  17219.2 |
  17254.8 |
  17290.4 |
  17326.1 |#
  17361.7 |
  17397.4 |##
  17433.0 |
  17468.7 |#
  17504.3 |#
  (9 below, 10 above range)

wide_decomposed (n=120, range 31920.0-33184.7 ns)
  31920.0 |########################################
  31983.2 |########
  32046.5 |####
  32109.7 |#####
  32172.9 |#
  32236.2 |
  32299.4 |
  32362.6 |#
  32425.9 |
  32489.1 |##########
  32552.4 |##
  32615.6 |
  32678.8 |
  32742.1 |#
  32805.3 |#
  32868.5 |
  32931.8 |
  32995.0 |
  33058.3 |##
  33121.5 |
  (13 below, 11 above range)

```

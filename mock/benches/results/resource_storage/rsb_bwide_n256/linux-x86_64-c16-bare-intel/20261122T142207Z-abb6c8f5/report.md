# Intra-resource member gather at M=64: contiguous blob vs scattered columns

2 variants, 120 samples per variant.
Baseline: **wide_blob**

## Highlights

Baseline for all deltas below: **wide_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3.68 us) is smaller than the fastest variant's own run-to-run std-dev (4.33 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (wide_blob)

The baseline wide_blob is the fastest (44.02 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (wide_blob) is the fastest** at 44023.7 ns median
- 1 variant significantly slower than baseline
- Spread: 1.08x (fastest 44023.7 ns, slowest 47699.8 ns)

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
| wide_blob | 46711ns | 44070ns | 44064ns | 45192ns | 53916ns | base |
| wide_decomposed | 52995ns | 47753ns | 47685ns | 51527ns | 62707ns | +13.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide_blob | 46661ns | 44018ns | 53858ns | base | 0.005 |
| wide_decomposed | 52942ns | 47640ns | 62641ns | +13.46% | 0.005 |

## Performance model

- Peak throughput: **0.006 Gops/s** (wide_blob; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide_blob | 0.006 | 100.0% |
| wide_decomposed | 0.005 | 92.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide_blob | 46711ns | 46711ns | base |
| wide_decomposed | 52995ns | 52995ns | +13.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide_blob | 44024ns | base | --- | [44023, 44025] | --- | --- | --- | --- |
| wide_decomposed | 47700ns | +3652.7ns (+8.3%) | [+3625, +4347]ns | [47649, 54244] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide_blob | wide_decomposed |
|---|---|---|---|
| 1 | 1 | 44022ns | +8.2% |
| 1 | 2 | 44023ns | +8.2% |
| 1 | 3 | 53814ns | +8.6% |
| 1 | 4 | 44023ns | +32.3% |
| 1 | 5 | 44024ns | +8.2% |
| 2 | 1 | 53822ns | +8.2% |
| 2 | 2 | 44021ns | +8.2% |
| 2 | 3 | 44022ns | +8.2% |
| 2 | 4 | 44024ns | +8.2% |
| 2 | 5 | 53818ns | +8.2% |
| 3 | 1 | 44024ns | +8.2% |
| 3 | 2 | 53818ns | -11.5% |
| 3 | 3 | 44019ns | +32.3% |
| 3 | 4 | 44022ns | +32.3% |
| 3 | 5 | 44023ns | +8.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide_blob | -0.388 | moderate- |
| wide_decomposed | -0.153 | ok |

**Consistency summary:**

- **wide_decomposed**: won 1/15, lost 14/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide_blob | 102.0ns | 46660.7ns | 0.2% |  |
| wide_decomposed | 337.6ns | 52941.7ns | 0.6% |  |

## Distribution (algo ns)

```
wide_blob (n=120, range 44018.5-53857.6 ns)
  44018.5 |########################################
  44510.4 |
  45002.4 |
  45494.4 |
  45986.3 |
  46478.3 |
  46970.2 |
  47462.2 |
  47954.1 |
  48446.1 |
  48938.0 |
  49430.0 |
  49922.0 |
  50413.9 |
  50905.9 |
  51397.8 |
  51889.8 |
  52381.7 |
  52873.7 |
  53365.6 |############
  (5 below, 6 above range)

wide_decomposed (n=120, range 47640.1-62641.1 ns)
  47640.1 |########################################
  48390.1 |
  49140.2 |
  49890.2 |
  50640.3 |
  51390.3 |
  52140.4 |
  52890.4 |
  53640.5 |
  54390.5 |
  55140.6 |
  55890.7 |
  56640.7 |
  57390.8 |
  58140.8 |#############################
  58890.9 |
  59640.9 |
  60391.0 |
  61141.0 |
  61891.1 |
  (6 below, 2 above range)

```

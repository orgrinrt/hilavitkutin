# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (47 ns) is smaller than the fastest variant's own run-to-run std-dev (134 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (load_aligned)

The baseline load_aligned is the fastest (123 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### load_unaligned is inconsistent: worst-20% is 5.6x its best-20%

load_unaligned's best 20% of batches run at 122 ns but its worst 20% at 687 ns (5.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (load_aligned) is the fastest** at 122.7 ns median
- Spread: 1.38x (fastest 122.7 ns, slowest 169.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x2c8af49106b7ed53 |
| Counter frequency | 2112743394 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| load_aligned | 2808ns | 1680ns | 1662ns | 2071ns | 6166ns | base |
| load_byte_pack | 3255ns | 2298ns | 1662ns | 2740ns | 6391ns | +15.90% |
| load_unaligned | 3935ns | 2043ns | 1663ns | 2867ns | 9410ns | +40.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 206ns | 122ns | 451ns | base | 4.983 |
| load_byte_pack | 233ns | 122ns | 445ns | +13.32% | 4.397 |
| load_unaligned | 289ns | 122ns | 687ns | +40.41% | 3.549 |

## Performance model

- Peak throughput: **8.373 Gops/s** (load_byte_pack; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 8.346 | 99.7% |
| load_byte_pack | 6.041 | 72.1% |
| load_unaligned | 6.838 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 2808ns | 2808ns | base |
| load_byte_pack | 3255ns | 3255ns | +15.90% |
| load_unaligned | 3935ns | 3935ns | +40.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 123ns | base | --- | [123, 149] | --- | --- | --- | --- |
| load_byte_pack | 170ns | no significant difference | [-0, +1]ns | [126, 210] | no | 0.3351 | 0.3351 | 5 |
| load_unaligned | 150ns | no significant difference | [+0, +27]ns | [149, 227] | no | 0.1538 | 0.0769 | 5 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 150ns | -18.1% | -11.9% |
| 1 | 2 | 303ns | -58.6% | -59.6% |
| 1 | 3 | 123ns | +118.9% | +193.9% |
| 1 | 4 | 224ns | -45.4% | -45.5% |
| 1 | 5 | 122ns | +0.0% | +0.1% |
| 1 | 6 | 122ns | -0.1% | +273.3% |
| 1 | 7 | 123ns | +141.8% | +22.1% |
| 1 | 8 | 122ns | +124.5% | +140.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | -0.237 | moderate- |
| load_byte_pack | 0.091 | ok |
| load_unaligned | -0.443 | moderate- |

**Consistency summary:**

- **load_byte_pack**: won 3/8, lost 3/8
- **load_unaligned**: won 3/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 34.3ns | 205.5ns | 16.7% | HIGH |
| load_byte_pack | 37.9ns | 232.9ns | 16.3% | HIGH |
| load_unaligned | 46.6ns | 288.6ns | 16.1% | HIGH |

## Distribution (algo ns)

```
load_aligned (n=160, range 122.3-450.8 ns)
    122.3 |########################################
    138.8 |###########
    155.2 |#
    171.6 |#
    188.0 |##
    204.4 |#
    220.9 |#
    237.3 |#
    253.7 |
    270.1 |#
    286.6 |#
    303.0 |#
    319.4 |
    335.8 |
    352.3 |
    368.7 |##
    385.1 |
    401.5 |##
    417.9 |#
    434.4 |#
  (18 below, 13 above range)

load_byte_pack (n=160, range 122.3-444.6 ns)
    122.3 |########################################
    138.4 |#
    154.5 |##
    170.6 |####
    186.8 |##
    202.9 |##
    219.0 |##
    235.1 |#
    251.2 |##
    267.3 |#
    283.5 |##
    299.6 |
    315.7 |
    331.8 |####
    347.9 |###
    364.0 |#
    380.1 |####
    396.3 |#
    412.4 |####
    428.5 |#
  (6 below, 12 above range)

load_unaligned (n=160, range 122.3-687.0 ns)
    122.3 |########################################
    150.6 |#
    178.8 |##
    207.0 |###
    235.3 |###
    263.5 |###
    291.7 |##
    320.0 |###
    348.2 |######
    376.4 |####
    404.7 |###
    432.9 |###
    461.2 |#
    489.4 |###
    517.6 |###
    545.9 |
    574.1 |
    602.3 |
    630.6 |
    658.8 |
  (23 below, 8 above range)

```

## Diagnostics

- **load_aligned**: CV=65.3% (high variance, measurements may be unstable)
- **load_aligned**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **load_aligned**: bridge=16.2% of algo (FFI overhead may distort results)
- **load_byte_pack**: CV=56.7% (high variance, measurements may be unstable)
- **load_byte_pack**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **load_byte_pack**: bridge=15.9% of algo (FFI overhead may distort results)
- **load_unaligned**: CV=94.3% (high variance, measurements may be unstable)
- **load_unaligned**: worst_20/best_20 = 5.6x (possible bimodal distribution)
- **load_unaligned**: bridge=16.1% of algo (FFI overhead may distort results)

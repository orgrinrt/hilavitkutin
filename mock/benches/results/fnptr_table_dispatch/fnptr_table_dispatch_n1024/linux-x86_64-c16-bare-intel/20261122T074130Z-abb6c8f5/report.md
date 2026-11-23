# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (136 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (fpt_match_16)

The baseline fpt_match_16 is the fastest (182 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fpt_table_16 is inconsistent: worst-20% is 3.3x its best-20%

fpt_table_16's best 20% of batches run at 146 ns but its worst 20% at 485 ns (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fpt_match_16) is the fastest** at 181.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 181.8 ns, slowest 183.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fn-pointer table vs match dispatch for 16-variant hot loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x478ab261e0d9f352 |
| Counter frequency | 2112768962 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fpt_match_16 | 2549ns | 2063ns | 1687ns | 1957ns | 5183ns | base |
| fpt_table_16 | 2712ns | 2065ns | 1685ns | 2148ns | 5429ns | +6.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 223ns | 147ns | 453ns | base | 4.583 |
| fpt_table_16 | 243ns | 146ns | 485ns | +8.65% | 4.218 |

## Performance model

- Peak throughput: **7.006 Gops/s** (fpt_table_16; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 5.631 | 80.4% |
| fpt_table_16 | 5.570 | 79.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 2549ns | 2549ns | base |
| fpt_table_16 | 2712ns | 2712ns | +6.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 182ns | base | --- | [177, 183] | --- | --- | --- | --- |
| fpt_table_16 | 184ns | +13.6ns (+7.5%) | [+11, +29]ns | [178, 189] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 177ns | +6.7% |
| 1 | 2 | 148ns | +20.0% |
| 1 | 3 | 183ns | -20.9% |
| 1 | 4 | 147ns | +19.6% |
| 1 | 5 | 147ns | +6.4% |
| 1 | 6 | 183ns | +72.0% |
| 1 | 7 | 185ns | -21.3% |
| 1 | 8 | 149ns | +28.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.327 | moderate- |
| fpt_table_16 | -0.386 | moderate- |

**Consistency summary:**

- **fpt_table_16**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 30.0ns | 223.4ns | 13.4% | HIGH |
| fpt_table_16 | 31.9ns | 242.8ns | 13.1% | HIGH |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 147.1-452.9 ns)
    147.1 |##############################
    162.4 |#########
    177.7 |########################################
    193.0 |##
    208.3 |
    223.6 |
    238.9 |
    254.1 |
    269.4 |
    284.7 |
    300.0 |
    315.3 |
    330.6 |
    345.9 |
    361.2 |
    376.5 |
    391.8 |
    407.0 |
    422.3 |#
    437.6 |
  (17 below, 20 above range)

fpt_table_16 (n=160, range 146.2-484.6 ns)
    146.2 |#####################
    163.1 |####################################
    180.0 |########################################
    196.9 |#
    213.8 |
    230.8 |
    247.7 |
    264.6 |#
    281.5 |
    298.4 |########
    315.4 |############
    332.3 |#
    349.2 |
    366.1 |
    383.0 |
    400.0 |#
    416.9 |
    433.8 |
    450.7 |##
    467.7 |#
  (28 below, 18 above range)

```

## Diagnostics

- **fpt_match_16**: CV=60.9% (high variance, measurements may be unstable)
- **fpt_match_16**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **fpt_match_16**: bridge=13.4% of algo (FFI overhead may distort results)
- **fpt_table_16**: CV=57.6% (high variance, measurements may be unstable)
- **fpt_table_16**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **fpt_table_16**: bridge=13.3% of algo (FFI overhead may distort results)

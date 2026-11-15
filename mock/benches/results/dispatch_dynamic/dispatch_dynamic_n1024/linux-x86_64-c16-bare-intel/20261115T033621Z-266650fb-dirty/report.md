# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct dominates: 22% faster than the next best (dispatch_volatile)

dispatch_direct (150 ns) leads dispatch_volatile (182 ns) by 22%, a clear separation rather than a photo finish. CV 86.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (43 ns) is smaller than the fastest variant's own run-to-run std-dev (130 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (150 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dispatch_runtime is inconsistent: worst-20% is 3.8x its best-20%

dispatch_runtime's best 20% of batches run at 163 ns but its worst 20% at 617 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 150.1 ns median
- 2 variants significantly slower than baseline
- Spread: 1.29x (fastest 150.1 ns, slowest 193.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x40ffee15d34db33f |
| Counter frequency | 2112780910 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_direct | 2848ns | 2030ns | 1659ns | 2202ns | 5975ns | base |
| dispatch_runtime | 3089ns | 2073ns | 1858ns | 2286ns | 6731ns | +8.47% |
| dispatch_volatile | 2925ns | 2062ns | 1687ns | 2254ns | 6175ns | +2.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 212ns | 123ns | 441ns | base | 4.838 |
| dispatch_runtime | 281ns | 163ns | 617ns | +32.66% | 3.647 |
| dispatch_volatile | 256ns | 149ns | 541ns | +20.83% | 4.004 |

## Performance model

- Peak throughput: **8.340 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 6.822 | 81.8% |
| dispatch_runtime | 5.299 | 63.5% |
| dispatch_volatile | 5.614 | 67.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2848ns | 2848ns | base |
| dispatch_runtime | 3089ns | 3089ns | +8.47% |
| dispatch_volatile | 2925ns | 2925ns | +2.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 150ns | base | --- | [150, 150] | --- | --- | --- | --- |
| dispatch_runtime | 193ns | +57.4ns (+38.2%) | [+56, +59]ns | [182, 198] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 182ns | +31.9ns (+21.3%) | [+31, +32]ns | [182, 182] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 123ns | +45.4% | +21.4% |
| 1 | 2 | 123ns | +48.5% | +113.8% |
| 1 | 3 | 248ns | -40.1% | -26.4% |
| 1 | 4 | 123ns | +46.5% | +48.6% |
| 1 | 5 | 150ns | +136.0% | +21.2% |
| 1 | 6 | 150ns | +42.6% | -0.5% |
| 1 | 7 | 153ns | +26.7% | +19.1% |
| 1 | 8 | 150ns | +19.5% | +21.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | -0.400 | moderate- |
| dispatch_runtime | 0.040 | ok |
| dispatch_volatile | -0.315 | moderate- |

**Consistency summary:**

- **dispatch_runtime**: won 1/8, lost 7/8
- **dispatch_volatile**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 33.0ns | 211.7ns | 15.6% | HIGH |
| dispatch_runtime | 36.8ns | 280.8ns | 13.1% | HIGH |
| dispatch_volatile | 34.3ns | 255.8ns | 13.4% | HIGH |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 122.8-441.1 ns)
    122.8 |#########################
    138.7 |########################################
    154.6 |##
    170.5 |
    186.4 |
    202.4 |
    218.3 |#
    234.2 |###########
    250.1 |#
    266.0 |#
    281.9 |
    297.9 |
    313.8 |
    329.7 |
    345.6 |
    361.5 |
    377.4 |#
    393.4 |
    409.3 |
    425.2 |#
  (8 below, 18 above range)

dispatch_runtime (n=160, range 162.5-616.8 ns)
    162.5 |########################################
    185.2 |############
    207.9 |#############
    230.7 |#
    253.4 |#
    276.1 |
    298.8 |##
    321.5 |
    344.2 |
    366.9 |
    389.7 |
    412.4 |
    435.1 |
    457.8 |
    480.5 |
    503.2 |#
    525.9 |##
    548.7 |
    571.4 |##
    594.1 |###
  (17 below, 15 above range)

dispatch_volatile (n=160, range 149.2-541.5 ns)
    149.2 |######
    168.8 |########################################
    188.4 |#
    208.1 |
    227.7 |
    247.3 |##########
    266.9 |
    286.5 |
    306.1 |
    325.7 |
    345.4 |
    365.0 |
    384.6 |
    404.2 |
    423.8 |
    443.4 |#
    463.0 |
    482.7 |
    502.3 |#
    521.9 |
  (20 below, 21 above range)

```

## Diagnostics

- **dispatch_direct**: CV=61.4% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **dispatch_direct**: bridge=15.8% of algo (FFI overhead may distort results)
- **dispatch_runtime**: CV=63.1% (high variance, measurements may be unstable)
- **dispatch_runtime**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **dispatch_runtime**: bridge=12.7% of algo (FFI overhead may distort results)
- **dispatch_volatile**: CV=62.0% (high variance, measurements may be unstable)
- **dispatch_volatile**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **dispatch_volatile**: bridge=13.3% of algo (FFI overhead may distort results)

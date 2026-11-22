# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct dominates: 21% faster than the next best (dispatch_runtime)

dispatch_direct (150 ns) leads dispatch_runtime (181 ns) by 21%, a clear separation rather than a photo finish. CV 84.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (32 ns) is smaller than the fastest variant's own run-to-run std-dev (127 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (150 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dispatch_runtime is inconsistent: worst-20% is 3.6x its best-20%

dispatch_runtime's best 20% of batches run at 150 ns but its worst 20% at 538 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 150.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.21x (fastest 150.1 ns, slowest 182.2 ns)

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
| Counter frequency | 2112761094 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_direct | 3057ns | 2039ns | 1848ns | 2404ns | 6222ns | base |
| dispatch_runtime | 2899ns | 2067ns | 1694ns | 2284ns | 5947ns | -5.17% |
| dispatch_volatile | 2619ns | 2063ns | 1690ns | 1959ns | 5529ns | -14.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 224ns | 137ns | 455ns | base | 4.562 |
| dispatch_runtime | 260ns | 150ns | 538ns | +15.63% | 3.945 |
| dispatch_volatile | 230ns | 149ns | 483ns | +2.56% | 4.448 |

## Performance model

- Peak throughput: **7.488 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 6.822 | 91.1% |
| dispatch_runtime | 5.661 | 75.6% |
| dispatch_volatile | 5.619 | 75.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 3057ns | 3057ns | base |
| dispatch_runtime | 2899ns | 2899ns | -5.17% |
| dispatch_volatile | 2619ns | 2619ns | -14.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 150ns | base | --- | [150, 150] | --- | --- | --- | --- |
| dispatch_runtime | 181ns | +30.0ns (+20.0%) | [+26, +31]ns | [180, 182] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 182ns | no significant difference | [-0, +27]ns | [182, 182] | no | 0.3846 | 0.3846 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 338ns | -47.0% | -45.9% |
| 1 | 2 | 150ns | +8.4% | -0.3% |
| 1 | 3 | 150ns | +165.0% | +21.7% |
| 1 | 4 | 150ns | -0.9% | -0.4% |
| 1 | 5 | 123ns | +21.0% | +21.7% |
| 1 | 6 | 150ns | +21.6% | +21.6% |
| 1 | 7 | 150ns | +29.4% | +21.8% |
| 1 | 8 | 150ns | +20.5% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | 0.004 | ok |
| dispatch_runtime | -0.277 | moderate- |
| dispatch_volatile | -0.373 | moderate- |

**Consistency summary:**

- **dispatch_runtime**: won 2/8, lost 6/8
- **dispatch_volatile**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 35.5ns | 224.5ns | 15.8% | HIGH |
| dispatch_runtime | 34.7ns | 259.6ns | 13.4% | HIGH |
| dispatch_volatile | 31.1ns | 230.2ns | 13.5% | HIGH |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 136.8-454.7 ns)
    136.8 |########################################
    152.7 |#
    168.5 |
    184.4 |
    200.3 |
    216.2 |
    232.1 |
    248.0 |##
    263.9 |
    279.8 |
    295.7 |
    311.6 |
    327.5 |##
    343.4 |
    359.3 |##
    375.2 |
    391.1 |
    407.0 |
    422.9 |
    438.8 |##
  (16 below, 17 above range)

dispatch_runtime (n=160, range 150.4-538.5 ns)
    150.4 |############
    169.8 |########################################
    189.2 |####
    208.6 |
    228.0 |#
    247.4 |
    266.8 |##
    286.2 |##
    305.6 |##
    325.1 |
    344.5 |
    363.9 |
    383.3 |
    402.7 |##
    422.1 |####
    441.5 |#
    460.9 |#
    480.3 |
    499.7 |##
    519.1 |##
  (25 below, 14 above range)

dispatch_volatile (n=160, range 149.1-483.0 ns)
    149.1 |########################################
    165.8 |############################
    182.5 |#############################
    199.2 |
    215.9 |
    232.6 |
    249.3 |
    266.0 |
    282.7 |
    299.4 |
    316.1 |
    332.8 |
    349.5 |
    366.2 |
    382.9 |
    399.5 |##
    416.2 |
    432.9 |###
    449.6 |
    466.3 |##
  (13 below, 15 above range)

```

## Diagnostics

- **dispatch_direct**: CV=56.5% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **dispatch_direct**: bridge=15.9% of algo (FFI overhead may distort results)
- **dispatch_runtime**: CV=59.3% (high variance, measurements may be unstable)
- **dispatch_runtime**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **dispatch_runtime**: bridge=13.5% of algo (FFI overhead may distort results)
- **dispatch_volatile**: CV=59.9% (high variance, measurements may be unstable)
- **dispatch_volatile**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **dispatch_volatile**: bridge=13.3% of algo (FFI overhead may distort results)

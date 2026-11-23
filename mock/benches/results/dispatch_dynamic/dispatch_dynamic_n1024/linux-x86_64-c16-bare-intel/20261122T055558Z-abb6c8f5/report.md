# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct dominates: 21% faster than the next best (dispatch_runtime)

dispatch_direct (150 ns) leads dispatch_runtime (181 ns) by 21%, a clear separation rather than a photo finish. CV 85.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (32 ns) is smaller than the fastest variant's own run-to-run std-dev (128 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (150 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dispatch_runtime is inconsistent: worst-20% is 3.7x its best-20%

dispatch_runtime's best 20% of batches run at 148 ns but its worst 20% at 544 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 150.1 ns median
- 2 variants significantly slower than baseline
- Spread: 1.22x (fastest 150.1 ns, slowest 182.5 ns)

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
| Counter frequency | 2112860326 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_direct | 2854ns | 2030ns | 1854ns | 2059ns | 6236ns | base |
| dispatch_runtime | 2813ns | 2060ns | 1688ns | 2075ns | 6154ns | -1.42% |
| dispatch_volatile | 2756ns | 2064ns | 1688ns | 2013ns | 6051ns | -3.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 210ns | 137ns | 457ns | base | 4.872 |
| dispatch_runtime | 248ns | 148ns | 544ns | +17.98% | 4.130 |
| dispatch_volatile | 243ns | 149ns | 533ns | +15.69% | 4.212 |

## Performance model

- Peak throughput: **7.460 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 6.822 | 91.4% |
| dispatch_runtime | 5.647 | 75.7% |
| dispatch_volatile | 5.611 | 75.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2854ns | 2854ns | base |
| dispatch_runtime | 2813ns | 2813ns | -1.42% |
| dispatch_volatile | 2756ns | 2756ns | -3.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 150ns | base | --- | [150, 150] | --- | --- | --- | --- |
| dispatch_runtime | 181ns | +30.7ns (+20.5%) | [+30, +32]ns | [181, 182] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 182ns | +32.4ns (+21.6%) | [+32, +33]ns | [182, 183] | YES | 0.0001 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 150ns | -2.1% | +21.9% |
| 1 | 2 | 150ns | +22.1% | +21.9% |
| 1 | 3 | 150ns | +21.2% | +21.7% |
| 1 | 4 | 150ns | +20.6% | -0.5% |
| 1 | 5 | 123ns | +21.1% | +48.6% |
| 1 | 6 | 150ns | +21.8% | -0.4% |
| 1 | 7 | 150ns | +20.3% | +21.8% |
| 1 | 8 | 150ns | +20.1% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | -0.160 | ok |
| dispatch_runtime | -0.239 | moderate- |
| dispatch_volatile | -0.466 | moderate- |

**Consistency summary:**

- **dispatch_runtime**: won 1/8, lost 7/8
- **dispatch_volatile**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 33.2ns | 210.2ns | 15.8% | HIGH |
| dispatch_runtime | 33.3ns | 248.0ns | 13.4% | HIGH |
| dispatch_volatile | 32.3ns | 243.1ns | 13.3% | HIGH |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 137.3-456.8 ns)
    137.3 |########################################
    153.2 |
    169.2 |
    185.2 |
    201.2 |
    217.1 |
    233.1 |
    249.1 |
    265.1 |
    281.0 |
    297.0 |
    313.0 |
    329.0 |
    345.0 |
    360.9 |
    376.9 |
    392.9 |
    408.9 |
    424.8 |
    440.8 |##
  (15 below, 17 above range)

dispatch_runtime (n=160, range 147.8-543.7 ns)
    147.8 |#######
    167.6 |########################################
    187.4 |#
    207.2 |
    227.0 |
    246.8 |
    266.6 |
    286.4 |
    306.2 |
    325.9 |
    345.7 |
    365.5 |
    385.3 |
    405.1 |
    424.9 |
    444.7 |
    464.5 |
    484.3 |
    504.1 |
    523.9 |
  (14 below, 20 above range)

dispatch_volatile (n=160, range 149.2-532.7 ns)
    149.2 |#################
    168.3 |########################################
    187.5 |
    206.7 |
    225.9 |
    245.0 |
    264.2 |
    283.4 |
    302.6 |
    321.7 |
    340.9 |
    360.1 |
    379.3 |
    398.4 |
    417.6 |#
    436.8 |
    456.0 |
    475.1 |
    494.3 |
    513.5 |
  (11 below, 18 above range)

```

## Diagnostics

- **dispatch_direct**: CV=61.0% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **dispatch_direct**: bridge=15.8% of algo (FFI overhead may distort results)
- **dispatch_runtime**: CV=63.4% (high variance, measurements may be unstable)
- **dispatch_runtime**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **dispatch_runtime**: bridge=13.5% of algo (FFI overhead may distort results)
- **dispatch_volatile**: CV=63.6% (high variance, measurements may be unstable)
- **dispatch_volatile**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **dispatch_volatile**: bridge=13.3% of algo (FFI overhead may distort results)

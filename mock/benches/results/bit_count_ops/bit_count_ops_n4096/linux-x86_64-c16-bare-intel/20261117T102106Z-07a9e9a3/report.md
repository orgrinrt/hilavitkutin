# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 3.8x slower than the field

ctz_loop (668 ns) is 3.8x the fastest (177 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ctz_debruijn is fastest but the noisiest (CV 84.0%)

ctz_debruijn wins on median (177 ns) yet has the highest variance (CV 84.0%), while ctz_loop is the steadiest (CV 56.6%, 668 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (ctz_debruijn)

The baseline ctz_debruijn is the fastest (177 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.8x the fastest

Fastest ctz_debruijn (177 ns) to slowest ctz_loop (668 ns): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### ctz_debruijn is inconsistent: worst-20% is 3.5x its best-20%

ctz_debruijn's best 20% of batches run at 145 ns but its worst 20% at 511 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### ctz_intrinsic's edge over baseline is significant but tiny (1 ns, 0.34%)

ctz_intrinsic differs from baseline ctz_debruijn by 1 ns (0.34%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (ctz_debruijn) is the fastest** at 177.1 ns median
- 2 variants significantly slower than baseline
- Spread: 3.77x (fastest 177.1 ns, slowest 667.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6f1038d492c5ab07 |
| Counter frequency | 2112755836 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ctz_debruijn | 2663ns | 2061ns | 1688ns | 1909ns | 5901ns | base |
| ctz_intrinsic | 2704ns | 2064ns | 1690ns | 2009ns | 5803ns | +1.54% |
| ctz_loop | 2865ns | 2551ns | 2115ns | 2527ns | 4628ns | +7.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 231ns | 145ns | 511ns | base | 17.716 |
| ctz_intrinsic | 235ns | 146ns | 505ns | +1.70% | 17.420 |
| ctz_loop | 748ns | 572ns | 1203ns | +223.64% | 5.474 |

## Performance model

- Peak throughput: **28.154 Gops/s** (ctz_debruijn; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 23.135 | 82.2% |
| ctz_intrinsic | 22.883 | 81.3% |
| ctz_loop | 6.134 | 21.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2663ns | 2663ns | base |
| ctz_intrinsic | 2704ns | 2704ns | +1.54% |
| ctz_loop | 2865ns | 2865ns | +7.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 177ns | base | --- | [149, 179] | --- | --- | --- | --- |
| ctz_intrinsic | 179ns | +0.6ns (+0.3%) | [+0, +1]ns | [178, 180] | YES | 0.0251 | 0.0251 | 3 |
| ctz_loop | 668ns | +489.6ns (+276.5%) | [+453, +497]ns | [651, 669] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 180ns | -0.1% | +275.6% |
| 1 | 2 | 147ns | +1.7% | +284.5% |
| 1 | 3 | 148ns | -0.2% | +295.1% |
| 1 | 4 | 180ns | -0.4% | +250.3% |
| 1 | 5 | 148ns | -0.4% | +338.4% |
| 1 | 6 | 149ns | +21.4% | +350.5% |
| 1 | 7 | 148ns | +21.5% | +352.7% |
| 1 | 8 | 181ns | +0.3% | +270.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | -0.287 | moderate- |
| ctz_intrinsic | -0.131 | ok |
| ctz_loop | 0.302 | moderate+ |

**Consistency summary:**

- **ctz_intrinsic**: won 4/8, lost 4/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 30.6ns | 231.2ns | 13.2% | HIGH |
| ctz_intrinsic | 31.1ns | 235.1ns | 13.2% | HIGH |
| ctz_loop | 27.0ns | 748.3ns | 3.6% |  |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 145.5-511.3 ns)
    145.5 |########################################
    163.8 |##########################
    182.1 |#####
    200.4 |
    218.7 |
    236.9 |
    255.2 |#
    273.5 |
    291.8 |
    310.1 |
    328.4 |
    346.7 |
    365.0 |
    383.3 |
    401.6 |
    419.9 |#
    438.1 |
    456.4 |###
    474.7 |#
    493.0 |##
  (12 below, 13 above range)

ctz_intrinsic (n=160, range 146.1-504.7 ns)
    146.1 |###################
    164.0 |########################################
    182.0 |########
    199.9 |
    217.8 |#
    235.8 |
    253.7 |
    271.6 |
    289.5 |
    307.5 |
    325.4 |
    343.3 |
    361.3 |
    379.2 |
    397.1 |
    415.1 |
    433.0 |
    450.9 |
    468.9 |
    486.8 |
  (15 below, 21 above range)

ctz_loop (n=160, range 572.2-1203.5 ns)
    572.2 |##########
    603.7 |##########
    635.3 |##########
    666.9 |########################################
    698.4 |
    730.0 |
    761.6 |
    793.1 |
    824.7 |
    856.3 |
    887.8 |
    919.4 |
    950.9 |
    982.5 |
   1014.1 |
   1045.6 |
   1077.2 |
   1108.8 |
   1140.3 |
   1171.9 |
  (18 below, 13 above range)

```

## Diagnostics

- **ctz_debruijn**: CV=64.3% (high variance, measurements may be unstable)
- **ctz_debruijn**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **ctz_debruijn**: bridge=13.2% of algo (FFI overhead may distort results)
- **ctz_intrinsic**: CV=62.1% (high variance, measurements may be unstable)
- **ctz_intrinsic**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **ctz_intrinsic**: bridge=13.1% of algo (FFI overhead may distort results)
- **ctz_loop**: CV=50.5% (high variance, measurements may be unstable)

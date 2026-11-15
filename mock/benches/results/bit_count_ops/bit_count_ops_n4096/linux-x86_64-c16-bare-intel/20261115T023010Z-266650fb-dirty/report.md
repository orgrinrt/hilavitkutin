# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 3.7x slower than the field

ctz_loop (671 ns) is 3.7x the fastest (180 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ctz_debruijn is fastest but the noisiest (CV 86.9%)

ctz_debruijn wins on median (180 ns) yet has the highest variance (CV 86.9%), while ctz_loop is the steadiest (CV 63.2%, 671 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (ctz_debruijn, ctz_intrinsic) are a dead heat (<1%)

ctz_debruijn (180 ns) and ctz_intrinsic (180 ns) differ by 0.11%, inside the noise, even though the wider field spreads 272.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (ctz_debruijn)

The baseline ctz_debruijn is the fastest (180 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.7x the fastest

Fastest ctz_debruijn (180 ns) to slowest ctz_loop (671 ns): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### ctz_debruijn is inconsistent: worst-20% is 3.7x its best-20%

ctz_debruijn's best 20% of batches run at 150 ns but its worst 20% at 551 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (ctz_debruijn) is the fastest** at 180.0 ns median
- 1 variant significantly slower than baseline
- Spread: 3.73x (fastest 180.0 ns, slowest 670.6 ns)

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
| Counter frequency | 2112792693 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ctz_debruijn | 2902ns | 2063ns | 1721ns | 2148ns | 6347ns | base |
| ctz_intrinsic | 2840ns | 2064ns | 1894ns | 2075ns | 6078ns | -2.16% |
| ctz_loop | 3050ns | 2554ns | 2303ns | 2556ns | 5279ns | +5.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 252ns | 150ns | 551ns | base | 16.239 |
| ctz_intrinsic | 248ns | 164ns | 531ns | -1.73% | 16.525 |
| ctz_loop | 800ns | 592ns | 1392ns | +217.11% | 5.121 |

## Performance model

- Peak throughput: **27.312 Gops/s** (ctz_debruijn; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 22.756 | 83.3% |
| ctz_intrinsic | 22.730 | 83.2% |
| ctz_loop | 6.108 | 22.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2902ns | 2902ns | base |
| ctz_intrinsic | 2840ns | 2840ns | -2.16% |
| ctz_loop | 3050ns | 3050ns | +5.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 180ns | base | --- | [179, 181] | --- | --- | --- | --- |
| ctz_intrinsic | 180ns | no significant difference | [-0, +1]ns | [180, 181] | no | 0.4741 | 0.4741 | 2 |
| ctz_loop | 671ns | +490.1ns (+272.3%) | [+467, +500]ns | [668, 672] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 180ns | +0.1% | +272.0% |
| 1 | 2 | 182ns | -1.2% | +279.7% |
| 1 | 3 | 149ns | +22.5% | +389.0% |
| 1 | 4 | 182ns | -0.9% | +257.8% |
| 1 | 5 | 180ns | -0.2% | +253.8% |
| 1 | 6 | 180ns | -17.6% | +203.1% |
| 1 | 7 | 180ns | +0.0% | +255.3% |
| 1 | 8 | 148ns | +22.0% | +353.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | -0.232 | moderate- |
| ctz_intrinsic | -0.124 | ok |
| ctz_loop | 0.322 | moderate+ |

**Consistency summary:**

- **ctz_intrinsic**: won 4/8, lost 3/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 33.3ns | 252.2ns | 13.2% | HIGH |
| ctz_intrinsic | 32.5ns | 247.9ns | 13.1% | HIGH |
| ctz_loop | 28.6ns | 799.9ns | 3.6% |  |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 150.0-551.0 ns)
    150.0 |
    170.0 |########################################
    190.1 |
    210.1 |
    230.2 |
    250.2 |
    270.3 |
    290.3 |
    310.4 |
    330.5 |
    350.5 |
    370.6 |
    390.6 |
    410.7 |
    430.7 |
    450.8 |#
    470.8 |
    490.9 |###
    510.9 |#
    531.0 |#
  (27 below, 13 above range)

ctz_intrinsic (n=160, range 163.8-531.1 ns)
    163.8 |########################################
    182.2 |#########
    200.6 |
    218.9 |
    237.3 |
    255.6 |
    274.0 |
    292.4 |
    310.7 |
    329.1 |
    347.5 |
    365.8 |
    384.2 |
    402.5 |
    420.9 |
    439.3 |
    457.6 |#
    476.0 |
    494.3 |
    512.7 |
  (14 below, 20 above range)

ctz_loop (n=160, range 591.8-1392.2 ns)
    591.8 |
    631.8 |########################################
    671.8 |#################
    711.8 |#########
    751.9 |
    791.9 |
    831.9 |
    871.9 |
    911.9 |
    952.0 |
    992.0 |
   1032.0 |
   1072.0 |
   1112.1 |#
   1152.1 |
   1192.1 |
   1232.1 |
   1272.2 |
   1312.2 |
   1352.2 |
  (16 below, 14 above range)

```

## Diagnostics

- **ctz_debruijn**: CV=62.0% (high variance, measurements may be unstable)
- **ctz_debruijn**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **ctz_debruijn**: bridge=13.1% of algo (FFI overhead may distort results)
- **ctz_intrinsic**: CV=61.5% (high variance, measurements may be unstable)
- **ctz_intrinsic**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **ctz_intrinsic**: bridge=13.2% of algo (FFI overhead may distort results)
- **ctz_loop**: CV=53.0% (high variance, measurements may be unstable)

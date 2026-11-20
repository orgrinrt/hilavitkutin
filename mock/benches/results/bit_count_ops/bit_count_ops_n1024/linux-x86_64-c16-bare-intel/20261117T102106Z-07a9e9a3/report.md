# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 2.2x slower than the field

ctz_loop (112 ns) is 2.2x the fastest (52 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (ctz_debruijn)

The baseline ctz_debruijn is the fastest (52 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### ctz_intrinsic is inconsistent: worst-20% is 4.0x its best-20%

ctz_intrinsic's best 20% of batches run at 42 ns but its worst 20% at 170 ns (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (ctz_debruijn) is the fastest** at 51.5 ns median
- 1 variant significantly slower than baseline
- Spread: 2.18x (fastest 51.5 ns, slowest 112.2 ns)

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
| ctz_debruijn | 2668ns | 1934ns | 1584ns | 2006ns | 5737ns | base |
| ctz_intrinsic | 2742ns | 1939ns | 1583ns | 1989ns | 6161ns | +2.78% |
| ctz_loop | 2587ns | 1656ns | 1638ns | 1845ns | 5761ns | -3.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 73ns | 42ns | 157ns | base | 14.010 |
| ctz_intrinsic | 75ns | 42ns | 170ns | +3.11% | 13.587 |
| ctz_loop | 164ns | 97ns | 364ns | +125.02% | 6.226 |

## Performance model

- Peak throughput: **24.379 Gops/s** (ctz_debruijn; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 19.883 | 81.6% |
| ctz_intrinsic | 19.673 | 80.7% |
| ctz_loop | 9.131 | 37.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2668ns | 2668ns | base |
| ctz_intrinsic | 2742ns | 2742ns | +2.78% |
| ctz_loop | 2587ns | 2587ns | -3.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 52ns | base | --- | [51, 52] | --- | --- | --- | --- |
| ctz_intrinsic | 52ns | no significant difference | [-0, +1]ns | [52, 54] | no | 0.8115 | 0.8115 | 2 |
| ctz_loop | 112ns | +62.7ns (+121.7%) | [+58, +69]ns | [110, 115] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 54ns | -1.1% | +81.3% |
| 1 | 2 | 53ns | -14.3% | +109.4% |
| 1 | 3 | 52ns | +6.4% | +115.4% |
| 1 | 4 | 44ns | -4.2% | +128.3% |
| 1 | 5 | 52ns | +0.2% | +84.4% |
| 1 | 6 | 43ns | +31.3% | +153.1% |
| 1 | 7 | 47ns | +21.1% | +172.8% |
| 1 | 8 | 42ns | +6.6% | +171.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | 0.093 | ok |
| ctz_intrinsic | -0.347 | moderate- |
| ctz_loop | 0.259 | moderate+ |

**Consistency summary:**

- **ctz_intrinsic**: won 3/8, lost 5/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 32.7ns | 73.1ns | 44.7% | HIGH |
| ctz_intrinsic | 33.9ns | 75.4ns | 45.0% | HIGH |
| ctz_loop | 31.2ns | 164.5ns | 19.0% | HIGH |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 42.0-157.4 ns)
     42.0 |################################
     47.8 |########################################
     53.5 |##############
     59.3 |#
     65.1 |
     70.9 |
     76.6 |
     82.4 |
     88.2 |
     93.9 |
     99.7 |
    105.5 |
    111.3 |##
    117.0 |#
    122.8 |##
    128.6 |##
    134.3 |###
    140.1 |
    145.9 |
    151.7 |###
  (19 below, 15 above range)

ctz_intrinsic (n=160, range 42.4-169.5 ns)
     42.4 |###################
     48.8 |########################################
     55.1 |#####################
     61.5 |##
     67.8 |
     74.2 |
     80.6 |
     86.9 |
     93.3 |
     99.6 |
    106.0 |
    112.3 |
    118.7 |
    125.0 |
    131.4 |
    137.7 |#
    144.1 |####
    150.4 |
    156.8 |####
    163.2 |###
  (20 below, 14 above range)

ctz_loop (n=160, range 97.0-364.4 ns)
     97.0 |########################################
    110.3 |#########################
    123.7 |############
    137.1 |
    150.5 |
    163.8 |
    177.2 |
    190.6 |
    204.0 |
    217.3 |
    230.7 |
    244.1 |#
    257.5 |#
    270.8 |
    284.2 |
    297.6 |##
    310.9 |##
    324.3 |#
    337.7 |###
    351.1 |
  (15 below, 14 above range)

```

## Diagnostics

- **ctz_debruijn**: CV=62.1% (high variance, measurements may be unstable)
- **ctz_debruijn**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **ctz_debruijn**: bridge=46.3% of algo (FFI overhead may distort results)
- **ctz_intrinsic**: CV=65.4% (high variance, measurements may be unstable)
- **ctz_intrinsic**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **ctz_intrinsic**: bridge=45.9% of algo (FFI overhead may distort results)
- **ctz_loop**: CV=65.3% (high variance, measurements may be unstable)
- **ctz_loop**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **ctz_loop**: bridge=17.8% of algo (FFI overhead may distort results)

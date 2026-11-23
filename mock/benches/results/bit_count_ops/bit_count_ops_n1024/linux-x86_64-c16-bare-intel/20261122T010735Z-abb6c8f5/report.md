# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 2.4x slower than the field

ctz_loop (123 ns) is 2.4x the fastest (51 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ctz_debruijn is fastest but the noisiest (CV 94.5%)

ctz_debruijn wins on median (51 ns) yet has the highest variance (CV 94.5%), while ctz_loop is the steadiest (CV 87.0%, 123 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (ctz_debruijn)

The baseline ctz_debruijn is the fastest (51 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### ctz_debruijn is inconsistent: worst-20% is 3.9x its best-20%

ctz_debruijn's best 20% of batches run at 42 ns but its worst 20% at 165 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (ctz_debruijn) is the fastest** at 51.3 ns median
- 2 variants significantly slower than baseline
- Spread: 2.40x (fastest 51.3 ns, slowest 123.1 ns)

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
| Counter frequency | 2112774681 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ctz_debruijn | 2654ns | 1928ns | 1576ns | 1875ns | 6069ns | base |
| ctz_intrinsic | 2955ns | 1935ns | 1602ns | 2395ns | 5986ns | +11.34% |
| ctz_loop | 2649ns | 1998ns | 1638ns | 1891ns | 5936ns | -0.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 73ns | 42ns | 165ns | base | 14.083 |
| ctz_intrinsic | 81ns | 44ns | 163ns | +11.40% | 12.642 |
| ctz_loop | 167ns | 102ns | 372ns | +130.09% | 6.121 |

## Performance model

- Peak throughput: **24.406 Gops/s** (ctz_debruijn; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 19.961 | 81.8% |
| ctz_intrinsic | 18.963 | 77.7% |
| ctz_loop | 8.318 | 34.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2654ns | 2654ns | base |
| ctz_intrinsic | 2955ns | 2955ns | +11.34% |
| ctz_loop | 2649ns | 2649ns | -0.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 51ns | base | --- | [51, 52] | --- | --- | --- | --- |
| ctz_intrinsic | 54ns | +1.5ns (+2.8%) | [+0, +4]ns | [53, 57] | YES | 0.0039 | 0.0039 | 3 |
| ctz_loop | 123ns | +69.6ns (+135.7%) | [+68, +72]ns | [115, 124] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 52ns | -13.3% | +86.2% |
| 1 | 2 | 45ns | +16.8% | +139.9% |
| 1 | 3 | 43ns | +0.9% | +157.2% |
| 1 | 4 | 52ns | +9.2% | +135.8% |
| 1 | 5 | 43ns | +28.7% | +161.9% |
| 1 | 6 | 44ns | +196.7% | +197.5% |
| 1 | 7 | 53ns | +4.6% | +139.9% |
| 1 | 8 | 57ns | -9.0% | +102.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | 0.042 | ok |
| ctz_intrinsic | -0.066 | ok |
| ctz_loop | 0.296 | moderate+ |

**Consistency summary:**

- **ctz_intrinsic**: won 2/8, lost 6/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 32.6ns | 72.7ns | 44.8% | HIGH |
| ctz_intrinsic | 36.4ns | 81.0ns | 45.0% | HIGH |
| ctz_loop | 32.1ns | 167.3ns | 19.2% | HIGH |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 42.0-165.4 ns)
     42.0 |#######################################
     48.1 |########################################
     54.3 |###############
     60.5 |####
     66.6 |
     72.8 |
     79.0 |
     85.1 |
     91.3 |
     97.5 |
    103.7 |
    109.8 |
    116.0 |
    122.2 |
    128.3 |
    134.5 |###
    140.7 |#
    146.8 |###
    153.0 |##
    159.2 |
  (15 below, 15 above range)

ctz_intrinsic (n=160, range 43.8-163.5 ns)
     43.8 |######
     49.8 |########################################
     55.8 |##############
     61.7 |
     67.7 |
     73.7 |#
     79.7 |#
     85.7 |
     91.7 |
     97.7 |##
    103.6 |
    109.6 |
    115.6 |#
    121.6 |##
    127.6 |
    133.6 |##
    139.6 |###
    145.5 |##
    151.5 |####
    157.5 |#####
  (21 below, 12 above range)

ctz_loop (n=160, range 101.9-371.8 ns)
    101.9 |########################################
    115.4 |##############################
    128.8 |############
    142.3 |
    155.8 |
    169.3 |
    182.8 |
    196.3 |
    209.8 |
    223.3 |
    236.8 |
    250.3 |
    263.8 |
    277.3 |##
    290.8 |
    304.3 |#
    317.8 |#
    331.3 |##
    344.8 |##
    358.3 |###
  (16 below, 13 above range)

```

## Diagnostics

- **ctz_debruijn**: CV=66.7% (high variance, measurements may be unstable)
- **ctz_debruijn**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **ctz_debruijn**: bridge=46.4% of algo (FFI overhead may distort results)
- **ctz_intrinsic**: CV=58.3% (high variance, measurements may be unstable)
- **ctz_intrinsic**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **ctz_intrinsic**: bridge=44.3% of algo (FFI overhead may distort results)
- **ctz_loop**: CV=64.0% (high variance, measurements may be unstable)
- **ctz_loop**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **ctz_loop**: bridge=19.7% of algo (FFI overhead may distort results)

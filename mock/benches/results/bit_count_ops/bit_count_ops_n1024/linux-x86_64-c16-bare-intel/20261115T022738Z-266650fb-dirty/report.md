# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 2.4x slower than the field

ctz_loop (124 ns) is 2.4x the fastest (52 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ctz_debruijn is fastest but the noisiest (CV 92.7%)

ctz_debruijn wins on median (52 ns) yet has the highest variance (CV 92.7%), while ctz_loop is the steadiest (CV 90.9%, 124 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (ctz_debruijn, ctz_intrinsic) are a dead heat (<1%)

ctz_debruijn (52 ns) and ctz_intrinsic (52 ns) differ by 0.48%, inside the noise, even though the wider field spreads 139.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (ctz_debruijn)

The baseline ctz_debruijn is the fastest (52 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### ctz_loop is inconsistent: worst-20% is 3.6x its best-20%

ctz_loop's best 20% of batches run at 109 ns but its worst 20% at 395 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (ctz_debruijn) is the fastest** at 51.8 ns median
- 1 variant significantly slower than baseline
- Spread: 2.39x (fastest 51.8 ns, slowest 123.9 ns)

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
| ctz_debruijn | 2834ns | 1931ns | 1762ns | 2024ns | 6338ns | base |
| ctz_intrinsic | 2807ns | 1931ns | 1805ns | 2044ns | 6096ns | -0.98% |
| ctz_loop | 2805ns | 2003ns | 1647ns | 2023ns | 6310ns | -1.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 76ns | 47ns | 170ns | base | 13.471 |
| ctz_intrinsic | 76ns | 48ns | 166ns | -0.21% | 13.500 |
| ctz_loop | 177ns | 109ns | 395ns | +132.26% | 5.800 |

## Performance model

- Peak throughput: **21.831 Gops/s** (ctz_debruijn; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 19.768 | 90.6% |
| ctz_intrinsic | 19.673 | 90.1% |
| ctz_loop | 8.265 | 37.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2834ns | 2834ns | base |
| ctz_intrinsic | 2807ns | 2807ns | -0.98% |
| ctz_loop | 2805ns | 2805ns | -1.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 52ns | base | --- | [52, 52] | --- | --- | --- | --- |
| ctz_intrinsic | 52ns | no significant difference | [+0, +1]ns | [52, 53] | no | 0.0788 | 0.0788 | 3 |
| ctz_loop | 124ns | +71.9ns (+138.7%) | [+69, +74]ns | [122, 126] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 52ns | -0.6% | +125.7% |
| 1 | 2 | 52ns | -0.2% | +110.0% |
| 1 | 3 | 52ns | -0.4% | +156.2% |
| 1 | 4 | 53ns | -16.3% | +133.7% |
| 1 | 5 | 52ns | +2.0% | +113.1% |
| 1 | 6 | 43ns | +20.7% | +153.7% |
| 1 | 7 | 52ns | +0.4% | +141.4% |
| 1 | 8 | 52ns | +0.8% | +169.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | -0.117 | ok |
| ctz_intrinsic | -0.200 | ok |
| ctz_loop | 0.056 | ok |

**Consistency summary:**

- **ctz_intrinsic**: won 4/8, lost 4/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 34.9ns | 76.0ns | 45.9% | HIGH |
| ctz_intrinsic | 34.4ns | 75.9ns | 45.4% | HIGH |
| ctz_loop | 34.0ns | 176.5ns | 19.2% | HIGH |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 46.9-169.7 ns)
     46.9 |########################################
     53.0 |######
     59.2 |
     65.3 |
     71.5 |
     77.6 |
     83.7 |
     89.9 |
     96.0 |
    102.2 |
    108.3 |
    114.4 |
    120.6 |
    126.7 |
    132.8 |
    139.0 |
    145.1 |
    151.3 |###
    157.4 |
    163.5 |##
  (15 below, 17 above range)

ctz_intrinsic (n=160, range 47.8-165.8 ns)
     47.8 |########################################
     53.7 |###
     59.6 |
     65.5 |
     71.4 |
     77.3 |
     83.2 |
     89.1 |
     95.0 |
    100.9 |
    106.8 |
    112.7 |
    118.6 |#
    124.5 |
    130.4 |
    136.3 |
    142.2 |
    148.1 |
    154.0 |#
    159.9 |
  (11 below, 18 above range)

ctz_loop (n=160, range 109.0-395.4 ns)
    109.0 |########################################
    123.3 |#######################
    137.6 |#########
    151.9 |
    166.3 |
    180.6 |
    194.9 |
    209.2 |
    223.5 |
    237.9 |
    252.2 |
    266.5 |
    280.8 |
    295.1 |
    309.5 |#
    323.8 |
    338.1 |##
    352.4 |
    366.8 |#
    381.1 |#
  (14 below, 18 above range)

```

## Diagnostics

- **ctz_debruijn**: CV=63.2% (high variance, measurements may be unstable)
- **ctz_debruijn**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **ctz_debruijn**: bridge=45.9% of algo (FFI overhead may distort results)
- **ctz_intrinsic**: CV=62.5% (high variance, measurements may be unstable)
- **ctz_intrinsic**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **ctz_intrinsic**: bridge=45.7% of algo (FFI overhead may distort results)
- **ctz_loop**: CV=63.8% (high variance, measurements may be unstable)
- **ctz_loop**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **ctz_loop**: bridge=19.6% of algo (FFI overhead may distort results)

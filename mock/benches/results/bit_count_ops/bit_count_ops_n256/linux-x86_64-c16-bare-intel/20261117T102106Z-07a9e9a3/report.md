# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_debruijn is fastest but the noisiest (CV 119.2%)

ctz_debruijn wins on median (20 ns) yet has the highest variance (CV 119.2%), while ctz_loop is the steadiest (CV 85.0%, 38 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (ctz_debruijn, ctz_intrinsic) are a dead heat (<1%)

ctz_debruijn (20 ns) and ctz_intrinsic (20 ns) differ by 0.00%, inside the noise, even though the wider field spreads 88.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (18 ns) is smaller than the fastest variant's own run-to-run std-dev (24 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (ctz_debruijn)

The baseline ctz_debruijn is the fastest (20 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### ctz_intrinsic's comparison is tie-heavy (24% tied pairs)

24% of paired samples for ctz_intrinsic are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### ctz_loop is inconsistent: worst-20% is 4.5x its best-20%

ctz_loop's best 20% of batches run at 25 ns but its worst 20% at 110 ns (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (ctz_debruijn) is the fastest** at 20.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.89x (fastest 20.0 ns, slowest 37.8 ns)

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
| ctz_debruijn | 2647ns | 1904ns | 1557ns | 1885ns | 6024ns | base |
| ctz_intrinsic | 2782ns | 1907ns | 1581ns | 2056ns | 6160ns | +5.08% |
| ctz_loop | 2713ns | 1923ns | 1567ns | 2081ns | 5751ns | +2.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 29ns | 16ns | 68ns | base | 8.867 |
| ctz_intrinsic | 29ns | 17ns | 65ns | +1.57% | 8.731 |
| ctz_loop | 51ns | 25ns | 110ns | +77.36% | 5.000 |

## Performance model

- Peak throughput: **15.663 Gops/s** (ctz_debruijn; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 12.800 | 81.7% |
| ctz_intrinsic | 12.800 | 81.7% |
| ctz_loop | 6.781 | 43.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2647ns | 2647ns | base |
| ctz_intrinsic | 2782ns | 2782ns | +5.08% |
| ctz_loop | 2713ns | 2713ns | +2.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 20ns | base | --- | [20, 20] | --- | --- | --- | --- |
| ctz_intrinsic | 20ns | no significant difference | [+0, +0]ns | [20, 20] | no (adj: YES) | 0.0002 | 0.0002 | **39** (24%, HIGH) |
| ctz_loop | 38ns | +17.5ns (+87.5%) | [+16, +18]ns | [37, 39] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 16ns | +22.3% | +49.7% |
| 1 | 2 | 16ns | +0.0% | +92.7% |
| 1 | 3 | 20ns | +0.0% | +86.6% |
| 1 | 4 | 20ns | -0.3% | +88.0% |
| 1 | 5 | 20ns | +0.5% | +93.0% |
| 1 | 6 | 20ns | +0.5% | +81.5% |
| 1 | 7 | 16ns | +22.6% | +50.9% |
| 1 | 8 | 16ns | +0.0% | +147.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | 0.368 | moderate+ |
| ctz_intrinsic | -0.217 | moderate- |
| ctz_loop | -0.099 | ok |

**Consistency summary:**

- **ctz_intrinsic**: won 1/8, lost 4/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 32.6ns | 28.9ns | 112.8% | HIGH |
| ctz_intrinsic | 34.3ns | 29.3ns | 117.1% | HIGH |
| ctz_loop | 34.2ns | 51.2ns | 66.9% | HIGH |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 16.3-67.9 ns)
     16.3 |##########################
     18.9 |########################################
     21.5 |
     24.1 |#
     26.6 |
     29.2 |
     31.8 |
     34.4 |
     37.0 |
     39.5 |
     42.1 |
     44.7 |
     47.3 |#
     49.8 |##
     52.4 |
     55.0 |###
     57.6 |###
     60.1 |#
     62.7 |#
     65.3 |###
  (18 below, 8 above range)

ctz_intrinsic (n=160, range 16.6-65.3 ns)
     16.6 |
     19.0 |########################################
     21.5 |
     23.9 |
     26.3 |
     28.8 |
     31.2 |
     33.6 |
     36.1 |
     38.5 |
     41.0 |
     43.4 |
     45.8 |
     48.3 |
     50.7 |
     53.1 |#
     55.6 |
     58.0 |##
     60.4 |##
     62.9 |
  (30 below, 16 above range)

ctz_loop (n=160, range 24.5-109.6 ns)
     24.5 |#################
     28.8 |####
     33.0 |#######################
     37.3 |########################################
     41.5 |
     45.8 |
     50.0 |
     54.3 |#
     58.6 |
     62.8 |
     67.1 |#
     71.3 |#
     75.6 |#
     79.8 |
     84.1 |###
     88.3 |
     92.6 |#
     96.8 |#####
    101.1 |#
    105.3 |#
  (16 below, 14 above range)

```

## Diagnostics

- **ctz_debruijn**: CV=82.6% (high variance, measurements may be unstable)
- **ctz_debruijn**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **ctz_debruijn**: bridge=117.5% of algo (FFI overhead may distort results)
- **ctz_intrinsic**: CV=65.4% (high variance, measurements may be unstable)
- **ctz_intrinsic**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **ctz_intrinsic**: bridge=118.0% of algo (FFI overhead may distort results)
- **ctz_loop**: CV=62.6% (high variance, measurements may be unstable)
- **ctz_loop**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **ctz_loop**: bridge=64.6% of algo (FFI overhead may distort results)

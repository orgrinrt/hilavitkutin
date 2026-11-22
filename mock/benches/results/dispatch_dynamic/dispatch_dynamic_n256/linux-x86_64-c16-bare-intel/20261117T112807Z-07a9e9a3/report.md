# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct shows alternating (throttle bounce) (autocorr -0.68)

dispatch_direct's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6 ns) is smaller than the fastest variant's own run-to-run std-dev (36 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### dispatch_direct is inconsistent: worst-20% is 3.8x its best-20%

dispatch_direct's best 20% of batches run at 35 ns but its worst 20% at 133 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: dispatch_runtime** at 41.0 ns median (-4.8% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.15x (fastest 41.0 ns, slowest 47.1 ns)

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
| dispatch_direct | 2670ns | 1925ns | 1575ns | 1936ns | 5965ns | base |
| dispatch_runtime | 2631ns | 1929ns | 1770ns | 1930ns | 5595ns | -1.45% |
| dispatch_volatile | 2760ns | 1933ns | 1578ns | 2193ns | 5642ns | +3.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 60ns | 35ns | 133ns | base | 4.287 |
| dispatch_runtime | 56ns | 36ns | 120ns | -6.53% | 4.587 |
| dispatch_volatile | 67ns | 38ns | 137ns | +12.49% | 3.811 |

## Performance model

- Peak throughput: **7.262 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 5.940 | 81.8% |
| dispatch_runtime | 6.236 | 85.9% |
| dispatch_volatile | 5.435 | 74.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2670ns | 2670ns | base |
| dispatch_runtime | 2631ns | 2631ns | -1.45% |
| dispatch_volatile | 2760ns | 2760ns | +3.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 43ns | base | --- | [43, 43] | --- | --- | --- | --- |
| dispatch_runtime | 41ns | no significant difference | [-3, +0]ns | [40, 41] | no | 0.1328 | 0.1328 | 0 |
| dispatch_volatile | 47ns | +3.9ns (+9.0%) | [+4, +4]ns | [47, 47] | YES | 0.0001 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 43ns | -6.0% | +9.3% |
| 1 | 2 | 35ns | +14.3% | +33.3% |
| 1 | 3 | 43ns | +1.2% | +135.3% |
| 1 | 4 | 35ns | +12.0% | +23.3% |
| 1 | 5 | 43ns | -2.1% | -10.7% |
| 1 | 6 | 35ns | +16.9% | +33.6% |
| 1 | 7 | 43ns | -11.6% | -10.7% |
| 1 | 8 | 43ns | -23.3% | +9.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | -0.677 | HIGH- (thermal bounce) |
| dispatch_runtime | 0.189 | ok |
| dispatch_volatile | -0.108 | ok |

**Consistency summary:**

- **dispatch_runtime**: won 4/8, lost 4/8
- **dispatch_volatile**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 32.5ns | 59.7ns | 54.5% | HIGH |
| dispatch_runtime | 32.7ns | 55.8ns | 58.7% | HIGH |
| dispatch_volatile | 35.1ns | 67.2ns | 52.3% | HIGH |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 35.2-133.0 ns)
     35.2 |##############
     40.1 |########################################
     45.0 |
     49.9 |
     54.8 |
     59.7 |
     64.6 |
     69.5 |
     74.4 |
     79.2 |
     84.1 |
     89.0 |
     93.9 |
     98.8 |
    103.7 |
    108.6 |#
    113.5 |
    118.4 |#
    123.2 |
    128.1 |##
  (14 below, 13 above range)

dispatch_runtime (n=160, range 35.9-119.6 ns)
     35.9 |##################
     40.0 |########################################
     44.2 |
     48.4 |
     52.6 |
     56.8 |
     61.0 |#
     65.2 |
     69.4 |
     73.5 |
     77.7 |
     81.9 |
     86.1 |
     90.3 |
     94.5 |
     98.7 |
    102.8 |#
    107.0 |
    111.2 |
    115.4 |#
  (14 below, 18 above range)

dispatch_volatile (n=160, range 38.3-137.0 ns)
     38.3 |#############
     43.3 |########################################
     48.2 |
     53.1 |
     58.1 |
     63.0 |
     67.9 |
     72.9 |
     77.8 |#
     82.7 |
     87.6 |
     92.6 |
     97.5 |####
    102.4 |##
    107.4 |#
    112.3 |##
    117.2 |
    122.2 |#
    127.1 |#
    132.0 |
  (18 below, 14 above range)

```

## Diagnostics

- **dispatch_direct**: CV=65.0% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **dispatch_direct**: bridge=54.8% of algo (FFI overhead may distort results)
- **dispatch_runtime**: CV=65.1% (high variance, measurements may be unstable)
- **dispatch_runtime**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **dispatch_runtime**: bridge=58.7% of algo (FFI overhead may distort results)
- **dispatch_volatile**: CV=59.1% (high variance, measurements may be unstable)
- **dispatch_volatile**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **dispatch_volatile**: bridge=52.4% of algo (FFI overhead may distort results)

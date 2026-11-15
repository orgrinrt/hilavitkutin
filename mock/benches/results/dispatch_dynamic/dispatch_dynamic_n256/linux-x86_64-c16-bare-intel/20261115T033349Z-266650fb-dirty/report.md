# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_runtime shows alternating (throttle bounce) (autocorr -0.70)

dispatch_runtime's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (5 ns) is smaller than the fastest variant's own run-to-run std-dev (37 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### dispatch_runtime is inconsistent: worst-20% is 3.9x its best-20%

dispatch_runtime's best 20% of batches run at 33 ns but its worst 20% at 129 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: dispatch_runtime** at 41.0 ns median (-4.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.13x (fastest 41.0 ns, slowest 46.5 ns)

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
| dispatch_direct | 2856ns | 1923ns | 1766ns | 2054ns | 6353ns | base |
| dispatch_runtime | 2699ns | 1919ns | 1573ns | 1921ns | 6159ns | -5.49% |
| dispatch_volatile | 2757ns | 1925ns | 1649ns | 2069ns | 5931ns | -3.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 64ns | 40ns | 141ns | base | 4.024 |
| dispatch_runtime | 57ns | 33ns | 129ns | -10.18% | 4.480 |
| dispatch_volatile | 67ns | 40ns | 144ns | +4.92% | 3.836 |

## Performance model

- Peak throughput: **7.822 Gops/s** (dispatch_runtime; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 5.940 | 75.9% |
| dispatch_runtime | 6.236 | 79.7% |
| dispatch_volatile | 5.505 | 70.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2856ns | 2856ns | base |
| dispatch_runtime | 2699ns | 2699ns | -5.49% |
| dispatch_volatile | 2757ns | 2757ns | -3.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 43ns | base | --- | [43, 43] | --- | --- | --- | --- |
| dispatch_runtime | 41ns | -2.8ns (-6.5%) | [-5, -3]ns | [40, 41] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 46ns | +3.3ns (+7.7%) | [+3, +3]ns | [46, 47] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 43ns | -5.8% | +8.8% |
| 1 | 2 | 43ns | -23.5% | -11.4% |
| 1 | 3 | 43ns | +1.0% | +8.1% |
| 1 | 4 | 43ns | -24.8% | +7.9% |
| 1 | 5 | 43ns | -2.1% | +7.9% |
| 1 | 6 | 35ns | +16.7% | +7.4% |
| 1 | 7 | 43ns | -11.1% | +8.4% |
| 1 | 8 | 43ns | -6.5% | +7.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | -0.161 | ok |
| dispatch_runtime | -0.696 | HIGH- (thermal bounce) |
| dispatch_volatile | -0.399 | moderate- |

**Consistency summary:**

- **dispatch_runtime**: won 6/8, lost 2/8
- **dispatch_volatile**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 34.8ns | 63.6ns | 54.7% | HIGH |
| dispatch_runtime | 33.6ns | 57.1ns | 58.8% | HIGH |
| dispatch_volatile | 35.7ns | 66.7ns | 53.5% | HIGH |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 39.6-140.7 ns)
     39.6 |########################################
     44.6 |
     49.7 |
     54.7 |
     59.8 |
     64.9 |
     69.9 |
     75.0 |
     80.0 |
     85.1 |
     90.2 |
     95.2 |
    100.3 |
    105.3 |
    110.4 |
    115.4 |#
    120.5 |#
    125.6 |#
    130.6 |
    135.7 |#
  (14 below, 18 above range)

dispatch_runtime (n=160, range 32.7-128.9 ns)
     32.7 |########
     37.5 |########################################
     42.3 |##########
     47.1 |
     52.0 |
     56.8 |
     61.6 |
     66.4 |
     71.2 |#
     76.0 |
     80.8 |
     85.6 |
     90.4 |
     95.2 |
    100.0 |
    104.8 |
    109.6 |#
    114.5 |#
    119.3 |
    124.1 |
  (17 below, 20 above range)

dispatch_volatile (n=160, range 39.5-144.3 ns)
     39.5 |
     44.8 |########################################
     50.0 |
     55.3 |
     60.5 |
     65.7 |
     71.0 |
     76.2 |
     81.5 |
     86.7 |
     91.9 |
     97.2 |#
    102.4 |
    107.6 |#
    112.9 |
    118.1 |#
    123.4 |
    128.6 |
    133.8 |#
    139.1 |#
  (25 below, 15 above range)

```

## Diagnostics

- **dispatch_direct**: CV=63.0% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **dispatch_direct**: bridge=54.5% of algo (FFI overhead may distort results)
- **dispatch_runtime**: CV=65.5% (high variance, measurements may be unstable)
- **dispatch_runtime**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **dispatch_runtime**: bridge=58.5% of algo (FFI overhead may distort results)
- **dispatch_volatile**: CV=62.4% (high variance, measurements may be unstable)
- **dispatch_volatile**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **dispatch_volatile**: bridge=53.1% of algo (FFI overhead may distort results)

# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### minmax_branchless is fastest but the noisiest (CV 89.8%)

minmax_branchless wins on median (50 ns) yet has the highest variance (CV 89.8%), while minmax_branch is the steadiest (CV 87.5%, 52 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### minmax_branchless shows alternating (throttle bounce) (autocorr -0.65)

minmax_branchless's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (45 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader minmax_branchless vs stability leader minmax_branch (+2% speed for 1.0x steadier)

minmax_branchless is fastest (50 ns, CV 89.8%); minmax_branch gives up 2.4% median for 1.0x lower variance (CV 87.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### minmax_intrinsic is inconsistent: worst-20% is 3.8x its best-20%

minmax_intrinsic's best 20% of batches run at 42 ns but its worst 20% at 159 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 2.6% of the fastest

All 3 variants sit between 50 ns and 52 ns - a 2.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: minmax_branchless** at 50.4 ns median (-2.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.03x (fastest 50.4 ns, slowest 51.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5d49a3f1b7c06e82 |
| Counter frequency | 2112833092 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| minmax_branch | 2654ns | 1933ns | 1763ns | 1935ns | 5702ns | base |
| minmax_branchless | 2791ns | 1934ns | 1925ns | 1954ns | 6165ns | +5.14% |
| minmax_intrinsic | 2646ns | 1931ns | 1579ns | 1921ns | 5887ns | -0.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 72ns | 46ns | 153ns | base | 14.319 |
| minmax_branchless | 72ns | 50ns | 159ns | +1.09% | 14.164 |
| minmax_intrinsic | 71ns | 42ns | 159ns | -0.22% | 14.351 |

## Performance model

- Peak throughput: **24.271 Gops/s** (minmax_intrinsic; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 19.845 | 81.8% |
| minmax_branchless | 20.317 | 83.7% |
| minmax_intrinsic | 19.807 | 81.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2654ns | 2654ns | base |
| minmax_branchless | 2791ns | 2791ns | +5.14% |
| minmax_intrinsic | 2646ns | 2646ns | -0.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 52ns | base | --- | [51, 52] | --- | --- | --- | --- |
| minmax_branchless | 50ns | -1.1ns (-2.1%) | [-1, -1]ns | [50, 50] | YES | 0.0000 | 0.0000 | 2 |
| minmax_intrinsic | 52ns | no significant difference | [-0, +0]ns | [51, 52] | no | 0.6852 | 0.6852 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 52ns | -3.5% | -18.6% |
| 1 | 2 | 51ns | -1.1% | +1.2% |
| 1 | 3 | 52ns | -2.6% | +0.9% |
| 1 | 4 | 51ns | -2.4% | +0.3% |
| 1 | 5 | 52ns | -2.8% | +0.3% |
| 1 | 6 | 51ns | -2.3% | +0.6% |
| 1 | 7 | 42ns | +22.0% | +0.4% |
| 1 | 8 | 58ns | -14.2% | -18.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | -0.479 | moderate- |
| minmax_branchless | -0.648 | HIGH- (thermal bounce) |
| minmax_intrinsic | 0.045 | ok |

**Consistency summary:**

- **minmax_branchless**: won 7/8, lost 1/8
- **minmax_intrinsic**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 33.1ns | 71.5ns | 46.3% | HIGH |
| minmax_branchless | 34.2ns | 72.3ns | 47.3% | HIGH |
| minmax_intrinsic | 33.0ns | 71.4ns | 46.2% | HIGH |

## Distribution (algo ns)

```
minmax_branch (n=160, range 46.4-153.3 ns)
     46.4 |########################################
     51.8 |############
     57.1 |#########
     62.5 |
     67.8 |
     73.2 |
     78.5 |
     83.9 |
     89.2 |
     94.5 |
     99.9 |
    105.2 |
    110.6 |
    115.9 |#
    121.3 |
    126.6 |
    132.0 |
    137.3 |
    142.7 |#
    148.0 |#
  (15 below, 18 above range)

minmax_branchless (n=160, range 49.9-158.9 ns)
     49.9 |########################################
     55.3 |
     60.8 |
     66.2 |
     71.7 |
     77.1 |
     82.6 |
     88.0 |
     93.5 |
     98.9 |
    104.4 |
    109.8 |
    115.3 |
    120.7 |
    126.2 |
    131.6 |
    137.1 |
    142.5 |
    148.0 |#
    153.4 |#
  (9 below, 19 above range)

minmax_intrinsic (n=160, range 42.2-158.7 ns)
     42.2 |##########
     48.0 |########################################
     53.8 |
     59.7 |
     65.5 |
     71.3 |
     77.1 |
     83.0 |
     88.8 |
     94.6 |
    100.4 |
    106.3 |
    112.1 |
    117.9 |
    123.7 |
    129.6 |
    135.4 |#
    141.2 |#
    147.0 |
    152.9 |#
  (21 below, 14 above range)

```

## Diagnostics

- **minmax_branch**: CV=63.2% (high variance, measurements may be unstable)
- **minmax_branch**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **minmax_branch**: bridge=46.9% of algo (FFI overhead may distort results)
- **minmax_branchless**: CV=62.6% (high variance, measurements may be unstable)
- **minmax_branchless**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **minmax_branchless**: bridge=47.4% of algo (FFI overhead may distort results)
- **minmax_intrinsic**: CV=63.6% (high variance, measurements may be unstable)
- **minmax_intrinsic**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **minmax_intrinsic**: bridge=46.6% of algo (FFI overhead may distort results)

# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 2.4% (fastest 50 ns to slowest 52 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### minmax_branchless is fastest but the noisiest (CV 92.6%)

minmax_branchless wins on median (50 ns) yet has the highest variance (CV 92.6%), while minmax_branch is the steadiest (CV 87.7%, 51 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (47 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader minmax_branchless vs stability leader minmax_branch (+1% speed for 1.1x steadier)

minmax_branchless is fastest (50 ns, CV 92.6%); minmax_branch gives up 1.5% median for 1.1x lower variance (CV 87.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### minmax_intrinsic is inconsistent: worst-20% is 3.8x its best-20%

minmax_intrinsic's best 20% of batches run at 42 ns but its worst 20% at 160 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: minmax_branchless** at 50.3 ns median (-1.5% vs baseline)
- Spread: 1.02x (fastest 50.3 ns, slowest 51.5 ns)

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
| Counter frequency | 2112457735 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| minmax_branch | 2649ns | 1929ns | 1576ns | 1936ns | 5858ns | base |
| minmax_branchless | 2839ns | 1930ns | 1727ns | 2072ns | 6251ns | +7.18% |
| minmax_intrinsic | 2743ns | 1928ns | 1576ns | 2052ns | 5982ns | +3.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 71ns | 42ns | 156ns | base | 14.414 |
| minmax_branchless | 74ns | 46ns | 163ns | +4.23% | 13.829 |
| minmax_intrinsic | 74ns | 42ns | 160ns | +3.93% | 13.869 |

## Performance model

- Peak throughput: **24.569 Gops/s** (minmax_branch; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 20.059 | 81.6% |
| minmax_branchless | 20.358 | 82.9% |
| minmax_intrinsic | 19.883 | 80.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2649ns | 2649ns | base |
| minmax_branchless | 2839ns | 2839ns | +7.18% |
| minmax_intrinsic | 2743ns | 2743ns | +3.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 51ns | base | --- | [51, 51] | --- | --- | --- | --- |
| minmax_branchless | 50ns | no significant difference | [-1, +5]ns | [50, 50] | no | 0.3382 | 0.3382 | 3 |
| minmax_intrinsic | 52ns | no significant difference | [+0, +0]ns | [51, 52] | no | 0.2066 | 0.1033 | 9 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 51ns | -0.6% | +1.8% |
| 1 | 2 | 42ns | +20.5% | +0.1% |
| 1 | 3 | 42ns | +18.8% | +0.4% |
| 1 | 4 | 51ns | -2.0% | -18.2% |
| 1 | 5 | 42ns | +20.0% | +22.0% |
| 1 | 6 | 51ns | -1.7% | +0.9% |
| 1 | 7 | 51ns | -18.0% | +0.3% |
| 1 | 8 | 58ns | -13.3% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | 0.013 | ok |
| minmax_branchless | -0.150 | ok |
| minmax_intrinsic | 0.349 | moderate+ |

**Consistency summary:**

- **minmax_branchless**: won 5/8, lost 3/8
- **minmax_intrinsic**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 32.9ns | 71.0ns | 46.3% | HIGH |
| minmax_branchless | 34.8ns | 74.0ns | 47.0% | HIGH |
| minmax_intrinsic | 34.4ns | 73.8ns | 46.5% | HIGH |

## Distribution (algo ns)

```
minmax_branch (n=160, range 41.7-156.2 ns)
     41.7 |#################
     47.4 |########################################
     53.1 |##########
     58.9 |
     64.6 |
     70.3 |
     76.0 |
     81.8 |
     87.5 |
     93.2 |
     98.9 |
    104.7 |
    110.4 |
    116.1 |
    121.8 |
    127.6 |####
    133.3 |
    139.0 |#
    144.7 |#
    150.5 |#
  (15 below, 14 above range)

minmax_branchless (n=160, range 45.5-162.8 ns)
     45.5 |########################################
     51.4 |
     57.3 |
     63.1 |
     69.0 |
     74.8 |
     80.7 |
     86.6 |
     92.4 |
     98.3 |
    104.2 |
    110.0 |
    115.9 |
    121.7 |
    127.6 |
    133.5 |
    139.3 |#
    145.2 |#
    151.1 |#
    156.9 |
  (17 below, 17 above range)

minmax_intrinsic (n=160, range 41.8-160.4 ns)
     41.8 |################
     47.7 |########################################
     53.7 |#########
     59.6 |
     65.5 |
     71.5 |
     77.4 |
     83.3 |
     89.2 |
     95.2 |
    101.1 |
    107.0 |
    113.0 |
    118.9 |#
    124.8 |
    130.8 |#
    136.7 |###
    142.6 |##
    148.5 |#
    154.5 |#
  (14 below, 15 above range)

```

## Diagnostics

- **minmax_branch**: CV=63.0% (high variance, measurements may be unstable)
- **minmax_branch**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **minmax_branch**: bridge=47.2% of algo (FFI overhead may distort results)
- **minmax_branchless**: CV=62.9% (high variance, measurements may be unstable)
- **minmax_branchless**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **minmax_branchless**: bridge=47.1% of algo (FFI overhead may distort results)
- **minmax_intrinsic**: CV=62.7% (high variance, measurements may be unstable)
- **minmax_intrinsic**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **minmax_intrinsic**: bridge=46.8% of algo (FFI overhead may distort results)

# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 36677% faster than the next best (fib_spectral)

fib_greedy (44 ns) leads fib_spectral (16.00 us) by 36677%, a clear separation rather than a photo finish. CV 108.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 108.4%)

fib_greedy wins on median (44 ns) yet has the highest variance (CV 108.4%), while fib_spectral is the steadiest (CV 7.1%, 16.00 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (44 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 367.8x the fastest

Fastest fib_greedy (44 ns) to slowest fib_spectral (16.00 us): 367.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fib_greedy is inconsistent: worst-20% is 3.8x its best-20%

fib_greedy's best 20% of batches run at 41 ns but its worst 20% at 158 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 43.5 ns median
- 1 variant significantly slower than baseline
- Spread: 367.77x (fastest 43.5 ns, slowest 15997.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fiber grouping theories: greedy topo walk vs spectral bisection |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3f1be00000001 |
| Counter frequency | 2112925964 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fib_greedy | 2525ns | 1583ns | 1575ns | 1745ns | 5815ns | base |
| fib_spectral | 18090ns | 17571ns | 17433ns | 17620ns | 20157ns | +616.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 69ns | 41ns | 158ns | base | 0.115 |
| fib_spectral | 16480ns | 15868ns | 18373ns | +23662.34% | 0.000 |

## Performance model

- Peak throughput: **0.194 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.184 | 95.0% |
| fib_spectral | 0.001 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2525ns | 2525ns | base |
| fib_spectral | 18090ns | 18090ns | +616.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 44ns | base | --- | [43, 46] | --- | --- | --- | --- |
| fib_spectral | 15998ns | +15938.5ns (+36640.2%) | [+15918, +15984]ns | [15975, 16028] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 41ns | +38619.7% |
| 1 | 2 | 42ns | +39711.4% |
| 1 | 3 | 44ns | +36462.9% |
| 1 | 4 | 43ns | +36714.0% |
| 1 | 5 | 46ns | +34379.5% |
| 1 | 6 | 62ns | +31478.9% |
| 1 | 7 | 42ns | +38180.7% |
| 1 | 8 | 42ns | +38475.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.030 | ok |
| fib_spectral | -0.261 | moderate- |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 84.7ns | 69.4ns | 122.1% | HIGH |
| fib_spectral | 54.1ns | 16479.6ns | 0.3% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 41.3-157.9 ns)
     41.3 |########################################
     47.1 |
     53.0 |
     58.8 |#######
     64.6 |
     70.5 |
     76.3 |
     82.1 |
     88.0 |
     93.8 |
     99.6 |
    105.5 |
    111.3 |
    117.1 |
    122.9 |
    128.8 |##
    134.6 |#
    140.4 |##
    146.3 |
    152.1 |#
  (19 below, 12 above range)

fib_spectral (n=160, range 15867.5-18373.1 ns)
  15867.5 |########################################
  15992.8 |#######################
  16118.1 |###
  16243.3 |#
  16368.6 |
  16493.9 |###########
  16619.2 |#
  16744.5 |
  16869.7 |
  16995.0 |
  17120.3 |
  17245.6 |
  17370.9 |
  17496.1 |
  17621.4 |
  17746.7 |
  17872.0 |
  17997.2 |
  18122.5 |
  18247.8 |
  (19 below, 20 above range)

```

## Diagnostics

- **fib_greedy**: CV=68.0% (high variance, measurements may be unstable)
- **fib_greedy**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **fib_greedy**: bridge=123.2% of algo (FFI overhead may distort results)

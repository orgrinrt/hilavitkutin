# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 31627% faster than the next best (fib_spectral)

fib_greedy (51 ns) leads fib_spectral (16.04 us) by 31627%, a clear separation rather than a photo finish. CV 87.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 87.6%)

fib_greedy wins on median (51 ns) yet has the highest variance (CV 87.6%), while fib_spectral is the steadiest (CV 63.5%, 16.04 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (51 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 317.3x the fastest

Fastest fib_greedy (51 ns) to slowest fib_spectral (16.04 us): 317.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fib_greedy is inconsistent: worst-20% is 3.7x its best-20%

fib_greedy's best 20% of batches run at 41 ns but its worst 20% at 151 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 50.5 ns median
- 1 variant significantly slower than baseline
- Spread: 317.27x (fastest 50.5 ns, slowest 16037.9 ns)

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
| Counter frequency | 2112788474 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fib_greedy | 2532ns | 1597ns | 1582ns | 1837ns | 5564ns | base |
| fib_spectral | 19941ns | 17597ns | 17429ns | 18258ns | 27504ns | +687.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 69ns | 41ns | 151ns | base | 0.116 |
| fib_spectral | 18159ns | 15868ns | 25028ns | +26249.28% | 0.000 |

## Performance model

- Peak throughput: **0.194 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.158 | 81.6% |
| fib_spectral | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2532ns | 2532ns | base |
| fib_spectral | 19941ns | 19941ns | +687.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 51ns | base | --- | [42, 52] | --- | --- | --- | --- |
| fib_spectral | 16038ns | +15966.0ns (+31584.7%) | [+15926, +16017]ns | [15995, 16069] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 56ns | +28308.0% |
| 1 | 2 | 42ns | +38055.5% |
| 1 | 3 | 41ns | +38431.4% |
| 1 | 4 | 42ns | +38164.8% |
| 1 | 5 | 41ns | +46963.3% |
| 1 | 6 | 52ns | +30704.9% |
| 1 | 7 | 41ns | +38255.8% |
| 1 | 8 | 51ns | +38329.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.233 | moderate- |
| fib_spectral | -0.214 | moderate- |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 83.9ns | 68.9ns | 121.8% | HIGH |
| fib_spectral | 61.6ns | 18159.1ns | 0.3% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 41.2-151.0 ns)
     41.2 |########################################
     46.7 |######################
     52.2 |#############
     57.7 |
     63.2 |
     68.7 |
     74.2 |
     79.7 |
     85.2 |##
     90.6 |
     96.1 |
    101.6 |
    107.1 |
    112.6 |
    118.1 |###
    123.6 |##
    129.1 |###
    134.6 |##
    140.1 |##
    145.5 |
  (17 below, 11 above range)

fib_spectral (n=160, range 15867.7-25027.8 ns)
  15867.7 |########################################
  16325.7 |
  16783.8 |
  17241.8 |
  17699.8 |
  18157.8 |
  18615.8 |
  19073.8 |##############
  19531.8 |##
  19989.8 |
  20447.8 |
  20905.8 |
  21363.8 |
  21821.8 |
  22279.8 |
  22737.8 |
  23195.8 |
  23653.8 |
  24111.8 |
  24569.8 |
  (16 below, 5 above range)

```

## Diagnostics

- **fib_greedy**: CV=64.3% (high variance, measurements may be unstable)
- **fib_greedy**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **fib_greedy**: bridge=107.0% of algo (FFI overhead may distort results)
- **fib_spectral**: CV=56.1% (high variance, measurements may be unstable)

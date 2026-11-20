# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 32760% faster than the next best (fib_spectral)

fib_greedy (348 ns) leads fib_spectral (114.19 us) by 32760%, a clear separation rather than a photo finish. CV 71.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 71.5%)

fib_greedy wins on median (348 ns) yet has the highest variance (CV 71.5%), while fib_spectral is the steadiest (CV 10.9%, 114.19 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (348 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 328.6x the fastest

Fastest fib_greedy (348 ns) to slowest fib_spectral (114.19 us): 328.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fib_greedy is inconsistent: worst-20% is 2.7x its best-20%

fib_greedy's best 20% of batches run at 313 ns but its worst 20% at 843 ns (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 347.5 ns median
- 1 variant significantly slower than baseline
- Spread: 328.60x (fastest 347.5 ns, slowest 114186.8 ns)

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
| fib_greedy | 2567ns | 1890ns | 1856ns | 2061ns | 4796ns | base |
| fib_spectral | 123945ns | 115761ns | 113166ns | 121775ns | 141236ns | +4727.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 452ns | 313ns | 843ns | base | 0.141 |
| fib_spectral | 122242ns | 111595ns | 139315ns | +26924.74% | 0.001 |

## Performance model

- Peak throughput: **0.205 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.184 | 90.0% |
| fib_spectral | 0.001 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2567ns | 2567ns | base |
| fib_spectral | 123945ns | 123945ns | +4727.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 348ns | base | --- | [337, 389] | --- | --- | --- | --- |
| fib_spectral | 114187ns | +113494.7ns (+32660.3%) | [+112375, +114591]ns | [112768, 114923] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 423ns | +32818.1% |
| 1 | 2 | 313ns | +43375.7% |
| 1 | 3 | 334ns | +34272.7% |
| 1 | 4 | 332ns | +33499.1% |
| 1 | 5 | 441ns | +31500.6% |
| 1 | 6 | 406ns | +27454.1% |
| 1 | 7 | 346ns | +32528.8% |
| 1 | 8 | 324ns | +34471.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | 0.011 | ok |
| fib_spectral | 0.035 | ok |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 531.7ns | 452.3ns | 117.5% | HIGH |
| fib_spectral | 420.4ns | 122242.2ns | 0.3% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 312.8-843.4 ns)
    312.8 |########################################
    339.4 |##########
    365.9 |####
    392.4 |#########
    418.9 |###################
    445.5 |##
    472.0 |
    498.5 |
    525.1 |
    551.6 |
    578.1 |
    604.6 |
    631.2 |
    657.7 |
    684.2 |
    710.8 |
    737.3 |
    763.8 |
    790.3 |
    816.9 |
  (12 below, 19 above range)

fib_spectral (n=160, range 111595.3-139314.8 ns)
  111595.3 |########################################
  112981.2 |##
  114367.2 |#############
  115753.2 |
  117139.2 |
  118525.2 |
  119911.1 |
  121297.1 |
  122683.1 |
  124069.1 |
  125455.0 |
  126841.0 |
  128227.0 |
  129613.0 |
  130998.9 |
  132384.9 |
  133770.9 |
  135156.9 |############
  136542.8 |
  137928.8 |###############
  (18 below, 17 above range)

```

## Diagnostics

- **fib_greedy**: CV=54.9% (high variance, measurements may be unstable)
- **fib_greedy**: bridge=118.6% of algo (FFI overhead may distort results)

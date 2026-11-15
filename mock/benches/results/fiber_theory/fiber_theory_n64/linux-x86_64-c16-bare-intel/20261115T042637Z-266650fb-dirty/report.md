# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 28193% faster than the next best (fib_spectral)

fib_greedy (401 ns) leads fib_spectral (113.57 us) by 28193%, a clear separation rather than a photo finish. CV 49.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 49.1%)

fib_greedy wins on median (401 ns) yet has the highest variance (CV 49.1%), while fib_spectral is the steadiest (CV 10.8%, 113.57 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (401 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 282.9x the fastest

Fastest fib_greedy (401 ns) to slowest fib_spectral (113.57 us): 282.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fib_greedy is inconsistent: worst-20% is 2.0x its best-20%

fib_greedy's best 20% of batches run at 340 ns but its worst 20% at 687 ns (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 401.4 ns median
- 1 variant significantly slower than baseline
- Spread: 282.93x (fastest 401.4 ns, slowest 113569.6 ns)

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
| Counter frequency | 2112788883 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fib_greedy | 2521ns | 2288ns | 1882ns | 2272ns | 3907ns | base |
| fib_spectral | 123253ns | 115139ns | 112389ns | 121430ns | 139588ns | +4788.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 444ns | 340ns | 687ns | base | 0.144 |
| fib_spectral | 121547ns | 110819ns | 137660ns | +27268.64% | 0.001 |

## Performance model

- Peak throughput: **0.188 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.159 | 84.8% |
| fib_spectral | 0.001 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2521ns | 2521ns | base |
| fib_spectral | 123253ns | 123253ns | +4788.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 401ns | base | --- | [395, 408] | --- | --- | --- | --- |
| fib_spectral | 113570ns | +112695.0ns (+28075.5%) | [+112514, +113806]ns | [112932, 114214] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 416ns | +27028.6% |
| 1 | 2 | 377ns | +29290.1% |
| 1 | 3 | 413ns | +27560.2% |
| 1 | 4 | 334ns | +33097.6% |
| 1 | 5 | 353ns | +39018.6% |
| 1 | 6 | 398ns | +27775.3% |
| 1 | 7 | 409ns | +33433.2% |
| 1 | 8 | 408ns | +33401.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | 0.043 | ok |
| fib_spectral | -0.027 | ok |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 522.7ns | 444.1ns | 117.7% | HIGH |
| fib_spectral | 429.0ns | 121547.4ns | 0.4% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 340.2-686.5 ns)
    340.2 |##############
    357.5 |##########
    374.8 |#####################
    392.1 |#####################################
    409.5 |########################################
    426.8 |#####
    444.1 |
    461.4 |
    478.7 |
    496.0 |#
    513.3 |
    530.7 |#
    548.0 |#
    565.3 |#
    582.6 |
    599.9 |#
    617.2 |
    634.6 |
    651.9 |
    669.2 |#
  (17 below, 10 above range)

fib_spectral (n=160, range 110818.5-137660.2 ns)
  110818.5 |########################################
  112160.6 |#################
  113502.7 |################
  114844.8 |
  116186.9 |
  117529.0 |
  118871.0 |
  120213.1 |
  121555.2 |
  122897.3 |
  124239.4 |
  125581.5 |
  126923.5 |
  128265.6 |
  129607.7 |
  130949.8 |
  132291.9 |
  133633.9 |
  134976.0 |
  136318.1 |##################################
  (14 below, 20 above range)

```

## Diagnostics

- **fib_greedy**: CV=44.4% (high variance, measurements may be unstable)
- **fib_greedy**: bridge=117.1% of algo (FFI overhead may distort results)

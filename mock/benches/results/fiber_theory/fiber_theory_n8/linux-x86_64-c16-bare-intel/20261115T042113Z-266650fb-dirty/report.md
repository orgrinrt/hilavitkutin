# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 31898% faster than the next best (fib_spectral)

fib_greedy (50 ns) leads fib_spectral (16.08 us) by 31898%, a clear separation rather than a photo finish. CV 97.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 97.3%)

fib_greedy wins on median (50 ns) yet has the highest variance (CV 97.3%), while fib_spectral is the steadiest (CV 10.7%, 16.08 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (50 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 320.0x the fastest

Fastest fib_greedy (50 ns) to slowest fib_spectral (16.08 us): 320.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fib_greedy is inconsistent: worst-20% is 4.0x its best-20%

fib_greedy's best 20% of batches run at 41 ns but its worst 20% at 165 ns (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 50.2 ns median
- 1 variant significantly slower than baseline
- Spread: 319.98x (fastest 50.2 ns, slowest 16078.8 ns)

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
| fib_greedy | 2682ns | 1925ns | 1578ns | 1907ns | 6114ns | base |
| fib_spectral | 19030ns | 17638ns | 17486ns | 18716ns | 21516ns | +609.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 73ns | 41ns | 165ns | base | 0.109 |
| fib_spectral | 17334ns | 15920ns | 19602ns | +23593.53% | 0.000 |

## Performance model

- Peak throughput: **0.193 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.159 | 82.4% |
| fib_spectral | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2682ns | 2682ns | base |
| fib_spectral | 19030ns | 19030ns | +609.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 50ns | base | --- | [44, 55] | --- | --- | --- | --- |
| fib_spectral | 16079ns | +16005.0ns (+31850.8%) | [+15966, +16044]ns | [16032, 16123] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 42ns | +38229.6% |
| 1 | 2 | 44ns | +36663.4% |
| 1 | 3 | 42ns | +46978.8% |
| 1 | 4 | 56ns | +34616.8% |
| 1 | 5 | 43ns | +36703.0% |
| 1 | 6 | 42ns | +38344.3% |
| 1 | 7 | 59ns | +32871.0% |
| 1 | 8 | 51ns | +31731.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.139 | ok |
| fib_spectral | -0.138 | ok |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 89.5ns | 73.2ns | 122.3% | HIGH |
| fib_spectral | 58.3ns | 17334.2ns | 0.3% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 41.4-165.4 ns)
     41.4 |########################################
     47.6 |########
     53.8 |####################
     60.0 |
     66.2 |
     72.4 |
     78.6 |
     84.8 |
     91.0 |
     97.2 |
    103.4 |
    109.6 |#
    115.8 |
    122.0 |
    128.2 |
    134.4 |##
    140.6 |#
    146.8 |##
    153.0 |#
    159.2 |###
  (12 below, 14 above range)

fib_spectral (n=160, range 15920.3-19601.8 ns)
  15920.3 |########################################
  16104.3 |####
  16288.4 |#
  16472.5 |
  16656.6 |
  16840.6 |
  17024.7 |
  17208.8 |
  17392.9 |
  17576.9 |
  17761.0 |
  17945.1 |
  18129.2 |
  18313.3 |
  18497.3 |
  18681.4 |
  18865.5 |
  19049.6 |
  19233.6 |
  19417.7 |###########################
  (16 below, 8 above range)

```

## Diagnostics

- **fib_greedy**: CV=66.9% (high variance, measurements may be unstable)
- **fib_greedy**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **fib_greedy**: bridge=124.7% of algo (FFI overhead may distort results)

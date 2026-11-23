# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 460% faster than the next best (evs_engine)

evs_std (7.21 us) leads evs_engine (40.40 us) by 460%, a clear separation rather than a photo finish. CV 6.0%.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (7.21 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 5.6x the fastest

Fastest evs_std (7.21 us) to slowest evs_engine (40.40 us): 5.6x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std) is the fastest at 7211.7 ns median
- 1 variant significantly slower than baseline
- Spread: 5.60x (fastest 7211.7 ns, slowest 40402.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | branching, two transforms joined by a third |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000002 |
| Counter frequency | 2112781463 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 41575ns | 40790ns | 36263ns | 39383ns | 44548ns | +445.11% |
| evs_std | 7627ns | 7483ns | 6869ns | 7352ns | 8023ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 41186ns | 35967ns | 44088ns | +463.53% | 0.099 |
| evs_std | 7309ns | 6628ns | 7645ns | base | 0.560 |

## Performance model

- Peak throughput: 0.618 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.101 | 16.4% |
| evs_std | 0.568 | 91.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 41575ns | 41575ns | +445.11% |
| evs_std | 7627ns | 7627ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 7212ns | base | --- | [6848, 7920] | --- | --- | --- | --- |
| evs_engine | 40402ns | +32839.1ns (+455.4%) | [+30524, +35617]ns | [38443, 42832] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 7032ns | +504.0% |
| 1 | 2 | 7564ns | +416.7% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_std | -0.500 | HIGH- (thermal bounce) |

Consistency summary:

- evs_engine: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 64286.0ns | 41186.3ns | 156.1% | HIGH |
| evs_std | 61655.2ns | 7308.6ns | 843.6% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 35967.1-44088.1 ns)
  35967.1 |########################################
  36373.1 |
  36779.2 |
  37185.2 |
  37591.3 |
  37997.3 |
  38403.4 |########################################
  38809.4 |########################################
  39215.5 |
  39621.5 |########################################
  40027.6 |
  40433.6 |
  40839.7 |########################################
  41245.7 |
  41651.8 |
  42057.8 |########################################
  42463.9 |########################################
  42869.9 |
  43276.0 |
  43682.0 |
  (0 below, 1 above range)

evs_std (n=8, range 6628.4-7644.7 ns)
   6628.4 |#############
   6679.2 |
   6730.0 |
   6780.8 |
   6831.7 |#############
   6882.5 |
   6933.3 |
   6984.1 |
   7034.9 |
   7085.7 |
   7136.5 |
   7187.4 |########################################
   7238.2 |
   7289.0 |
   7339.8 |
   7390.6 |
   7441.4 |
   7492.3 |#############
   7543.1 |
   7593.9 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=157.6% of algo (FFI overhead may distort results)
- evs_std: bridge=847.9% of algo (FFI overhead may distort results)

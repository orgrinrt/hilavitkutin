# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 373% faster than the next best (evs_engine)

evs_std (6.61 us) leads evs_engine (31.23 us) by 373%, a clear separation rather than a photo finish. CV 4.6%.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (6.61 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 4.7x the fastest

Fastest evs_std (6.61 us) to slowest evs_engine (31.23 us): 4.7x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std) is the fastest at 6606.6 ns median
- 1 variant significantly slower than baseline
- Spread: 4.73x (fastest 6606.6 ns, slowest 31228.8 ns)

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
| Counter frequency | 2112872410 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 33363ns | 31507ns | 29749ns | 31038ns | 36011ns | +380.03% |
| evs_std | 6950ns | 6858ns | 6307ns | 6818ns | 7210ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 33032ns | 29482ns | 35618ns | +396.63% | 0.124 |
| evs_std | 6651ns | 6088ns | 6851ns | base | 0.616 |

## Performance model

- Peak throughput: 0.673 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.131 | 19.5% |
| evs_std | 0.620 | 92.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 33363ns | 33363ns | +380.03% |
| evs_std | 6950ns | 6950ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 6607ns | base | --- | [6553, 6973] | --- | --- | --- | --- |
| evs_engine | 31229ns | +24563.8ns (+371.8%) | [+23968, +27218]ns | [30056, 33841] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 6558ns | +374.5% |
| 1 | 2 | 6793ns | +376.3% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 56509.5ns | 33031.6ns | 171.1% | HIGH |
| evs_std | 56153.2ns | 6651.1ns | 844.3% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 29482.1-35617.7 ns)
  29482.1 |#############
  29788.9 |#############
  30095.7 |
  30402.4 |
  30709.2 |
  31016.0 |########################################
  31322.8 |
  31629.5 |
  31936.3 |
  32243.1 |
  32549.9 |
  32856.7 |
  33163.4 |#############
  33470.2 |
  33777.0 |#############
  34083.8 |
  34390.5 |
  34697.3 |
  35004.1 |
  35310.9 |
  (0 below, 1 above range)

evs_std (n=8, range 6087.6-6851.1 ns)
   6087.6 |####################
   6125.8 |
   6164.0 |
   6202.1 |
   6240.3 |
   6278.5 |
   6316.7 |
   6354.8 |
   6393.0 |
   6431.2 |
   6469.4 |
   6507.5 |
   6545.7 |########################################
   6583.9 |########################################
   6622.1 |####################
   6660.2 |
   6698.4 |
   6736.6 |
   6774.8 |
   6812.9 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=179.6% of algo (FFI overhead may distort results)
- evs_std: bridge=848.1% of algo (FFI overhead may distort results)

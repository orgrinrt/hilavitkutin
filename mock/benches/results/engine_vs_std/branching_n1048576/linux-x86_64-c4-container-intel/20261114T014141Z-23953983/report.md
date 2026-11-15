# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 16% faster than the next best (evs_engine)

evs_std (903.14 us) leads evs_engine (1.04 ms) by 16%, a clear separation rather than a photo finish. CV 4.6%.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (903.14 us median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 903139.4 ns median
- 1 variant significantly slower than baseline
- Spread: 1.16x (fastest 903139.4 ns, slowest 1044711.6 ns)

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
| Counter frequency | 2100267710 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 1052103ns | 1047427ns | 995992ns | 1017515ns | 1092072ns | +16.21% |
| evs_std | 905383ns | 905038ns | 851068ns | 880643ns | 937516ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 1049760ns | 994346ns | 1089270ns | +16.22% | 0.999 |
| evs_std | 903276ns | 848771ns | 935513ns | base | 1.161 |

## Performance model

- Peak throughput: 1.235 Gops/s (evs_std; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.004 | 81.2% |
| evs_std | 1.161 | 94.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 1052103ns | 1052103ns | +16.21% |
| evs_std | 905383ns | 905383ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 903139ns | base | --- | [859726, 929349] | --- | --- | --- | --- |
| evs_engine | 1044712ns | +134581.6ns (+14.9%) | [+105712, +195021]ns | [1005933, 1118205] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 913562ns | +19.0% |
| 1 | 2 | 866524ns | +17.6% |

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
| evs_engine | 5604686.1ns | 1049759.7ns | 533.9% | HIGH |
| evs_std | 5807000.6ns | 903276.4ns | 642.9% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 994345.5-1089270.0 ns)
  994345.5 |####################
  999091.7 |
  1003837.9 |########################################
  1008584.2 |
  1013330.4 |
  1018076.6 |
  1022822.8 |
  1027569.1 |
  1032315.3 |####################
  1037061.5 |
  1041807.7 |
  1046554.0 |
  1051300.2 |
  1056046.4 |####################
  1060792.6 |####################
  1065538.9 |
  1070285.1 |
  1075031.3 |
  1079777.5 |
  1084523.8 |
  (0 below, 2 above range)

evs_std (n=8, range 848771.1-935513.2 ns)
  848771.1 |####################
  853108.2 |
  857445.3 |####################
  861782.4 |
  866119.5 |
  870456.6 |####################
  874793.7 |
  879130.9 |
  883468.0 |
  887805.1 |
  892142.2 |
  896479.3 |
  900816.4 |########################################
  905153.5 |
  909490.6 |
  913827.7 |
  918164.8 |
  922501.9 |####################
  926839.0 |####################
  931176.1 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=537.9% of algo (FFI overhead may distort results)
- evs_std: bridge=644.1% of algo (FFI overhead may distort results)

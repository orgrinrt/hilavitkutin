# element_wise, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_startup dominates: 233% faster than the next best (evs_engine_startup)

evs_std_startup (401.86 us) leads evs_engine_startup (1.34 ms) by 233%, a clear separation rather than a photo finish. CV 7.6%.


### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std_startup)

The baseline evs_std_startup is the fastest (401.86 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 3.3x the fastest

Fastest evs_std_startup (401.86 us) to slowest evs_engine_startup (1.34 ms): 3.3x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std_startup) is the fastest at 401862.5 ns median
- 1 variant significantly slower than baseline
- Spread: 3.33x (fastest 401862.5 ns, slowest 1339754.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | element_wise, get-ready |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000010001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine_startup | 1401793ns | 1342321ns | 1324533ns | 1335489ns | 1470835ns | +237.27% |
| evs_std_startup | 415626ns | 404242ns | 395317ns | 400586ns | 431983ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 1399300ns | 1321842ns | 1468460ns | +238.55% | 2.997 |
| evs_std_startup | 413326ns | 393233ns | 429577ns | base | 10.148 |

## Performance model

- Peak throughput: 10.666 Gops/s (evs_std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 3.131 | 29.4% |
| evs_std_startup | 10.437 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 1401793ns | 1401793ns | +237.27% |
| evs_std_startup | 415626ns | 415626ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 401862ns | base | --- | [396742, 412958] | --- | --- | --- | --- |
| evs_engine_startup | 1339754ns | +935400.0ns (+232.8%) | [+911358, +1039800]ns | [1331458, 1436542] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 397129ns | +237.3% |
| 1 | 2 | 411721ns | +227.9% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine_startup | -0.500 | moderate- |
| evs_std_startup | -0.500 | moderate- |

Consistency summary:

- evs_engine_startup: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine_startup | 24034080.2ns | 1399300.0ns | 1717.6% | HIGH |
| evs_std_startup | 23738167.7ns | 413326.0ns | 5743.2% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 1321841.7-1468460.4 ns)
  1321841.7 |#############
  1329172.6 |########################################
  1336503.6 |
  1343834.5 |#############
  1351165.4 |
  1358496.4 |
  1365827.3 |#############
  1373158.3 |
  1380489.2 |
  1387820.1 |
  1395151.1 |
  1402482.0 |
  1409812.9 |
  1417143.9 |
  1424474.8 |
  1431805.7 |#############
  1439136.7 |
  1446467.6 |
  1453798.6 |
  1461129.5 |
  (0 below, 1 above range)

evs_std_startup (n=8, range 393233.3-429577.1 ns)
  393233.3 |########################################
  395050.5 |########################################
  396867.7 |########################################
  398684.9 |
  400502.1 |########################################
  402319.2 |########################################
  404136.4 |
  405953.6 |
  407770.8 |
  409588.0 |########################################
  411405.2 |########################################
  413222.4 |
  415039.6 |
  416856.8 |
  418673.9 |
  420491.1 |
  422308.3 |
  424125.5 |
  425942.7 |
  427759.9 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine_startup: bridge=1791.5% of algo (FFI overhead may distort results)
- evs_std_startup: bridge=5904.7% of algo (FFI overhead may distort results)

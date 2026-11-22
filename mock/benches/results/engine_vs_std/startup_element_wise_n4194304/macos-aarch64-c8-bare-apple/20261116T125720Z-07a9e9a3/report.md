# element_wise, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_startup dominates: 231% faster than the next best (evs_engine_startup)

evs_std_startup (404.01 us) leads evs_engine_startup (1.34 ms) by 231%, a clear separation rather than a photo finish. CV 7.8%.


### evs_std_startup is fastest but the noisiest (CV 7.8%)

evs_std_startup wins on median (404.01 us) yet has the highest variance (CV 7.8%), while evs_engine_startup is the steadiest (CV 2.3%, 1.34 ms).


### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std_startup)

The baseline evs_std_startup is the fastest (404.01 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 3.3x the fastest

Fastest evs_std_startup (404.01 us) to slowest evs_engine_startup (1.34 ms): 3.3x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std_startup) is the fastest at 404008.3 ns median
- 1 variant significantly slower than baseline
- Spread: 3.31x (fastest 404008.3 ns, slowest 1336208.3 ns)

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
| evs_engine_startup | 1348956ns | 1338538ns | 1326842ns | 1331250ns | 1367765ns | +221.81% |
| evs_std_startup | 419182ns | 406162ns | 403917ns | 404867ns | 433735ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 1346673ns | 1324600ns | 1365344ns | +222.82% | 3.115 |
| evs_std_startup | 417162ns | 401483ns | 431952ns | base | 10.054 |

## Performance model

- Peak throughput: 10.447 Gops/s (evs_std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 3.139 | 30.0% |
| evs_std_startup | 10.382 | 99.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 1348956ns | 1348956ns | +221.81% |
| evs_std_startup | 419182ns | 419182ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 404008ns | base | --- | [402383, 412567] | --- | --- | --- | --- |
| evs_engine_startup | 1336208ns | +934275.0ns (+231.3%) | [+913817, +958825]ns | [1324917, 1361217] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 407900ns | +226.8% |
| 1 | 2 | 403583ns | +231.1% |

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
| evs_engine_startup | 23950814.6ns | 1346672.9ns | 1778.5% | HIGH |
| evs_std_startup | 23740066.7ns | 417162.5ns | 5690.8% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 1324600.0-1365343.8 ns)
  1324600.0 |########################################
  1326637.2 |####################
  1328674.4 |
  1330711.6 |
  1332748.8 |
  1334785.9 |####################
  1336823.1 |####################
  1338860.3 |####################
  1340897.5 |
  1342934.7 |
  1344971.9 |
  1347009.1 |
  1349046.2 |
  1351083.4 |
  1353120.6 |
  1355157.8 |
  1357195.0 |
  1359232.2 |####################
  1361269.4 |
  1363306.6 |
  (0 below, 1 above range)

evs_std_startup (n=8, range 401483.3-431952.1 ns)
  401483.3 |########################################
  403006.7 |#############
  404530.2 |#############
  406053.6 |
  407577.1 |
  409100.5 |
  410623.9 |#############
  412147.4 |#############
  413670.8 |
  415194.2 |
  416717.7 |
  418241.1 |
  419764.6 |
  421288.0 |
  422811.4 |
  424334.9 |
  425858.3 |
  427381.8 |
  428905.2 |
  430428.6 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine_startup: bridge=1791.8% of algo (FFI overhead may distort results)
- evs_std_startup: bridge=5874.9% of algo (FFI overhead may distort results)

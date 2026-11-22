# element_wise, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_startup dominates: 218% faster than the next best (evs_engine_startup)

evs_std_startup (429.73 us) leads evs_engine_startup (1.36 ms) by 218%, a clear separation rather than a photo finish. CV 5.4%.


### evs_std_startup is fastest but the noisiest (CV 5.4%)

evs_std_startup wins on median (429.73 us) yet has the highest variance (CV 5.4%), while evs_engine_startup is the steadiest (CV 4.0%, 1.36 ms).


### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std_startup)

The baseline evs_std_startup is the fastest (429.73 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 3.2x the fastest

Fastest evs_std_startup (429.73 us) to slowest evs_engine_startup (1.36 ms): 3.2x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std_startup) is the fastest at 429733.3 ns median
- 1 variant significantly slower than baseline
- Spread: 3.18x (fastest 429733.3 ns, slowest 1364687.5 ns)

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
| evs_engine_startup | 1387523ns | 1366346ns | 1335108ns | 1353175ns | 1426388ns | +215.52% |
| evs_std_startup | 439752ns | 432521ns | 413267ns | 421461ns | 460092ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 1385254ns | 1332767ns | 1424137ns | +216.85% | 3.028 |
| evs_std_startup | 437193ns | 410883ns | 457381ns | base | 9.594 |

## Performance model

- Peak throughput: 10.208 Gops/s (evs_std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 3.073 | 30.1% |
| evs_std_startup | 9.760 | 95.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 1387523ns | 1387523ns | +215.52% |
| evs_std_startup | 439752ns | 439752ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 429733ns | base | --- | [417658, 463683] | --- | --- | --- | --- |
| evs_engine_startup | 1364688ns | +920787.5ns (+214.3%) | [+903500, +1002650]ns | [1336483, 1413533] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 433412ns | +213.9% |
| 1 | 2 | 428417ns | +223.3% |

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
| evs_engine_startup | 24076132.3ns | 1385254.2ns | 1738.0% | HIGH |
| evs_std_startup | 23816077.1ns | 437192.7ns | 5447.5% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 1332766.7-1424137.5 ns)
  1332766.7 |########################################
  1337335.2 |
  1341903.8 |
  1346472.3 |
  1351040.9 |####################
  1355609.4 |
  1360177.9 |####################
  1364746.5 |####################
  1369315.0 |
  1373883.5 |
  1378452.1 |
  1383020.6 |
  1387589.2 |
  1392157.7 |
  1396726.2 |
  1401294.8 |
  1405863.3 |####################
  1410431.9 |####################
  1415000.4 |
  1419568.9 |
  (0 below, 1 above range)

evs_std_startup (n=8, range 410883.3-457381.2 ns)
  410883.3 |########################################
  413208.2 |
  415533.1 |########################################
  417858.0 |########################################
  420182.9 |########################################
  422507.8 |
  424832.7 |
  427157.6 |
  429482.5 |
  431807.4 |
  434132.3 |
  436457.2 |
  438782.1 |########################################
  441107.0 |
  443431.8 |
  445756.7 |########################################
  448081.6 |
  450406.5 |
  452731.4 |
  455056.3 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine_startup: bridge=1761.5% of algo (FFI overhead may distort results)
- evs_std_startup: bridge=5536.6% of algo (FFI overhead may distort results)

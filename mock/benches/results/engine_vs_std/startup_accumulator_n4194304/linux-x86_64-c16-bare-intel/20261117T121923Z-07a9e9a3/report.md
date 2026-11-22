# accumulator, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_startup dominates: 967% faster than the next best (evs_engine_startup)

evs_std_startup (225.36 us) leads evs_engine_startup (2.40 ms) by 967%, a clear separation rather than a photo finish. CV 4.8%.


### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std_startup)

The baseline evs_std_startup is the fastest (225.36 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 10.7x the fastest

Fastest evs_std_startup (225.36 us) to slowest evs_engine_startup (2.40 ms): 10.7x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std_startup) is the fastest at 225362.7 ns median
- 1 variant significantly slower than baseline
- Spread: 10.67x (fastest 225362.7 ns, slowest 2404014.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | accumulator, get-ready |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000010003 |
| Counter frequency | 2112872410 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine_startup | 2390013ns | 2405241ns | 1894071ns | 2130477ns | 2708650ns | +948.12% |
| evs_std_startup | 228028ns | 226655ns | 210929ns | 222787ns | 236233ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 2388556ns | 1892556ns | 2707231ns | +954.27% | 1.756 |
| evs_std_startup | 226561ns | 209681ns | 234721ns | base | 18.513 |

## Performance model

- Peak throughput: 20.003 Gops/s (evs_std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 1.745 | 8.7% |
| evs_std_startup | 18.611 | 93.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 2390013ns | 2390013ns | +948.12% |
| evs_std_startup | 228028ns | 228028ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 225363ns | base | --- | [219363, 231630] | --- | --- | --- | --- |
| evs_engine_startup | 2404015ns | +2173099.0ns (+964.3%) | [+1682876, +2303809]ns | [1896906, 2553313] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 230916ns | +951.5% |
| 1 | 2 | 220372ns | +804.4% |

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
| evs_engine_startup | 23048372.7ns | 2388556.0ns | 965.0% | HIGH |
| evs_std_startup | 25521665.2ns | 226560.7ns | 11264.8% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 1892556.3-2707231.4 ns)
  1892556.3 |########################################
  1933290.1 |
  1974023.8 |
  2014757.6 |
  2055491.3 |####################
  2096225.1 |
  2136958.8 |
  2177692.6 |
  2218426.3 |
  2259160.1 |
  2299893.8 |
  2340627.6 |
  2381361.3 |########################################
  2422095.1 |####################
  2462828.9 |
  2503562.6 |
  2544296.4 |####################
  2585030.1 |
  2625763.9 |
  2666497.6 |
  (0 below, 1 above range)

evs_std_startup (n=8, range 209680.8-234721.1 ns)
  209680.8 |########################################
  210932.8 |
  212184.8 |
  213436.8 |
  214688.9 |
  215940.9 |
  217192.9 |
  218444.9 |########################################
  219696.9 |
  220948.9 |########################################
  222201.0 |########################################
  223453.0 |
  224705.0 |
  225957.0 |
  227209.0 |########################################
  228461.0 |
  229713.1 |########################################
  230965.1 |########################################
  232217.1 |
  233469.1 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine_startup: bridge=959.3% of algo (FFI overhead may distort results)
- evs_std_startup: bridge=11131.9% of algo (FFI overhead may distort results)

# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 263% faster than the next best (evs_engine)

evs_std (7.76 us) leads evs_engine (28.20 us) by 263%, a clear separation rather than a photo finish. CV 0.9%.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (7.76 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 3.6x the fastest

Fastest evs_std (7.76 us) to slowest evs_engine (28.20 us): 3.6x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std) is the fastest at 7762.5 ns median
- 1 variant significantly slower than baseline
- Spread: 3.63x (fastest 7762.5 ns, slowest 28200.0 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 28517ns | 28567ns | 28050ns | 28403ns | 28719ns | +259.50% |
| evs_std | 7932ns | 7917ns | 7850ns | 7892ns | 7983ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 28177ns | 27892ns | 28338ns | +261.34% | 2.326 |
| evs_std | 7798ns | 7733ns | 7850ns | base | 8.404 |

## Performance model

- Peak throughput: 8.475 Gops/s (evs_std; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 2.324 | 27.4% |
| evs_std | 8.443 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 28517ns | 28517ns | +259.50% |
| evs_std | 7932ns | 7932ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 7762ns | base | --- | [7742, 7892] | --- | --- | --- | --- |
| evs_engine | 28200ns | +20462.5ns (+263.6%) | [+20042, +20633]ns | [27933, 28383] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 7817ns | +259.0% |
| 1 | 2 | 7762ns | +265.4% |

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
| evs_engine | 355558.3ns | 28177.1ns | 1261.9% | HIGH |
| evs_std | 354257.3ns | 7797.9ns | 4543.0% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 27891.7-28337.5 ns)
  27891.7 |########################################
  27914.0 |########################################
  27936.3 |
  27958.6 |
  27980.9 |
  28003.2 |
  28025.4 |
  28047.7 |########################################
  28070.0 |
  28092.3 |
  28114.6 |
  28136.9 |
  28159.2 |
  28181.5 |########################################
  28203.8 |########################################
  28226.0 |
  28248.3 |
  28270.6 |
  28292.9 |
  28315.2 |
  (0 below, 3 above range)

evs_std (n=8, range 7733.3-7850.0 ns)
   7733.3 |########################################
   7739.1 |########################################
   7745.0 |########################################
   7750.8 |
   7756.6 |########################################
   7762.5 |########################################
   7768.3 |
   7774.2 |
   7780.0 |
   7785.8 |
   7791.7 |
   7797.5 |
   7803.3 |
   7809.2 |
   7815.0 |########################################
   7820.8 |
   7826.7 |
   7832.5 |
   7838.4 |
   7844.2 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=1255.6% of algo (FFI overhead may distort results)
- evs_std: bridge=4542.5% of algo (FFI overhead may distort results)

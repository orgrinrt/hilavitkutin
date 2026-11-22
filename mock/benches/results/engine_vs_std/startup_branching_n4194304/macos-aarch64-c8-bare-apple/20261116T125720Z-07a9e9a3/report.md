# branching, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_startup dominates: 158% faster than the next best (evs_engine_startup)

evs_std_startup (406.61 us) leads evs_engine_startup (1.05 ms) by 158%, a clear separation rather than a photo finish. CV 2.7%.


### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std_startup)

The baseline evs_std_startup is the fastest (406.61 us median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std_startup) is the fastest at 406608.3 ns median
- 1 variant significantly slower than baseline
- Spread: 2.58x (fastest 406608.3 ns, slowest 1047625.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | branching, get-ready |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000010002 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine_startup | 1052552ns | 1049825ns | 1044092ns | 1046928ns | 1058885ns | +155.17% |
| evs_std_startup | 412490ns | 408921ns | 401817ns | 406000ns | 420025ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 1050017ns | 1041150ns | 1056338ns | +156.06% | 3.995 |
| evs_std_startup | 410064ns | 400108ns | 417754ns | base | 10.228 |

## Performance model

- Peak throughput: 10.483 Gops/s (evs_std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 4.004 | 38.2% |
| evs_std_startup | 10.315 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 1052552ns | 1052552ns | +155.17% |
| evs_std_startup | 412490ns | 412490ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 406608ns | base | --- | [401092, 415508] | --- | --- | --- | --- |
| evs_engine_startup | 1047625ns | +637883.3ns (+156.9%) | [+633608, +642450]ns | [1041258, 1049117] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 411833ns | +154.4% |
| 1 | 2 | 402162ns | +159.8% |

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
| evs_engine_startup | 25468617.7ns | 1050016.7ns | 2425.5% | HIGH |
| evs_std_startup | 23762039.6ns | 410063.5ns | 5794.7% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 1041150.0-1056337.5 ns)
  1041150.0 |########################################
  1041909.4 |
  1042668.8 |
  1043428.1 |
  1044187.5 |####################
  1044946.9 |
  1045706.2 |
  1046465.6 |
  1047225.0 |########################################
  1047984.4 |####################
  1048743.8 |####################
  1049503.1 |
  1050262.5 |
  1051021.9 |
  1051781.2 |
  1052540.6 |
  1053300.0 |
  1054059.4 |
  1054818.8 |
  1055578.1 |
  (0 below, 1 above range)

evs_std_startup (n=8, range 400108.3-417754.1 ns)
  400108.3 |########################################
  400990.6 |########################################
  401872.9 |
  402755.2 |########################################
  403637.5 |
  404519.8 |########################################
  405402.1 |
  406284.3 |
  407166.6 |
  408048.9 |########################################
  408931.2 |
  409813.5 |
  410695.8 |########################################
  411578.1 |
  412460.4 |
  413342.7 |
  414225.0 |
  415107.3 |########################################
  415989.6 |
  416871.9 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine_startup: bridge=2425.8% of algo (FFI overhead may distort results)
- evs_std_startup: bridge=5842.4% of algo (FFI overhead may distort results)

# wide_parallel, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_startup dominates: 1608% faster than the next best (evs_engine_startup)

evs_std_startup (1.01 ms) leads evs_engine_startup (17.32 ms) by 1608%, a clear separation rather than a photo finish. CV 4.1%.


### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std_startup)

The baseline evs_std_startup is the fastest (1.01 ms median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 17.1x the fastest

Fastest evs_std_startup (1.01 ms) to slowest evs_engine_startup (17.32 ms): 17.1x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std_startup) is the fastest at 1014114.4 ns median
- 1 variant significantly slower than baseline
- Spread: 17.08x (fastest 1014114.4 ns, slowest 17317431.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | wide_parallel, get-ready |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000010004 |
| Counter frequency | 2112723270 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine_startup | 17278023ns | 17318756ns | 17046099ns | 17112641ns | 17460040ns | +1573.60% |
| evs_std_startup | 1032389ns | 1015908ns | 989649ns | 1003746ns | 1064555ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 17276672ns | 17044566ns | 17458727ns | +1576.80% | 0.243 |
| evs_std_startup | 1030334ns | 987779ns | 1062533ns | base | 4.071 |

## Performance model

- Peak throughput: 4.246 Gops/s (evs_std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 0.242 | 5.7% |
| evs_std_startup | 4.136 | 97.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 17278023ns | 17278023ns | +1573.60% |
| evs_std_startup | 1032389ns | 1032389ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 1014114ns | base | --- | [999864, 1077994] | --- | --- | --- | --- |
| evs_engine_startup | 17317431ns | +16260583.6ns (+1603.4%) | [+16042750, +16446304]ns | [17054168, 17460209] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 1012507ns | +1623.9% |
| 1 | 2 | 1018039ns | +1577.0% |

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
| evs_engine_startup | 110883268.2ns | 17276672.3ns | 641.8% | HIGH |
| evs_std_startup | 121701551.7ns | 1030333.7ns | 11811.9% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 17044566.3-17458727.2 ns)
  17044566.3 |########################################
  17065274.3 |
  17085982.4 |####################
  17106690.4 |
  17127398.5 |
  17148106.5 |
  17168814.6 |####################
  17189522.6 |
  17210230.7 |
  17230938.7 |
  17251646.8 |
  17272354.8 |
  17293062.8 |
  17313770.9 |
  17334478.9 |
  17355187.0 |
  17375895.0 |
  17396603.1 |
  17417311.1 |
  17438019.2 |########################################
  (0 below, 2 above range)

evs_std_startup (n=8, range 987779.0-1062532.6 ns)
  987779.0 |####################
  991516.7 |
  995254.4 |
  998992.0 |########################################
  1002729.7 |####################
  1006467.4 |
  1010205.1 |
  1013942.8 |
  1017680.5 |
  1021418.1 |####################
  1025155.8 |
  1028893.5 |
  1032631.2 |####################
  1036368.9 |
  1040106.6 |
  1043844.2 |
  1047581.9 |
  1051319.6 |
  1055057.3 |
  1058795.0 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine_startup: bridge=639.5% of algo (FFI overhead may distort results)
- evs_std_startup: bridge=11993.4% of algo (FFI overhead may distort results)

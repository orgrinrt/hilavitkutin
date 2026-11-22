# branching, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_startup dominates: 160% faster than the next best (evs_engine_startup)

evs_std_startup (415.35 us) leads evs_engine_startup (1.08 ms) by 160%, a clear separation rather than a photo finish. CV 4.1%.


### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std_startup)

The baseline evs_std_startup is the fastest (415.35 us median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std_startup) is the fastest at 415354.2 ns median
- 1 variant significantly slower than baseline
- Spread: 2.60x (fastest 415354.2 ns, slowest 1080358.4 ns)

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
| evs_engine_startup | 1097652ns | 1082638ns | 1042367ns | 1059011ns | 1140454ns | +159.89% |
| evs_std_startup | 422356ns | 418271ns | 401908ns | 413344ns | 434227ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 1095278ns | 1039600ns | 1137600ns | +160.76% | 3.829 |
| evs_std_startup | 420028ns | 399292ns | 432160ns | base | 9.986 |

## Performance model

- Peak throughput: 10.504 Gops/s (evs_std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 3.882 | 37.0% |
| evs_std_startup | 10.098 | 96.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 1097652ns | 1097652ns | +159.89% |
| evs_std_startup | 422356ns | 422356ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 415354ns | base | --- | [405183, 430933] | --- | --- | --- | --- |
| evs_engine_startup | 1080358ns | +673262.5ns (+162.1%) | [+630325, +739700]ns | [1046133, 1144883] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 413554ns | +166.4% |
| 1 | 2 | 419742ns | +153.3% |

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
| evs_engine_startup | 25469103.1ns | 1095278.1ns | 2325.4% | HIGH |
| evs_std_startup | 23859200.0ns | 420028.1ns | 5680.4% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 1039600.0-1137600.0 ns)
  1039600.0 |########################################
  1044500.0 |########################################
  1049400.0 |
  1054300.0 |########################################
  1059200.0 |
  1064100.0 |
  1069000.0 |########################################
  1073900.0 |
  1078800.0 |
  1083700.0 |
  1088600.0 |########################################
  1093500.0 |
  1098400.0 |
  1103300.0 |
  1108200.0 |
  1113100.0 |########################################
  1118000.0 |
  1122900.0 |
  1127800.0 |
  1132700.0 |
  (0 below, 2 above range)

evs_std_startup (n=8, range 399291.7-432160.4 ns)
  399291.7 |########################################
  400935.1 |
  402578.6 |
  404222.0 |########################################
  405865.4 |
  407508.9 |
  409152.3 |
  410795.7 |########################################
  412439.2 |
  414082.6 |########################################
  415726.1 |########################################
  417369.5 |
  419012.9 |
  420656.4 |
  422299.8 |########################################
  423943.2 |
  425586.7 |
  427230.1 |
  428873.5 |
  430517.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine_startup: bridge=2357.2% of algo (FFI overhead may distort results)
- evs_std_startup: bridge=5730.3% of algo (FFI overhead may distort results)

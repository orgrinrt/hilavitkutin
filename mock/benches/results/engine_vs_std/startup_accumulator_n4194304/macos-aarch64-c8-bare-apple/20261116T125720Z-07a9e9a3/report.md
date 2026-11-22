# accumulator, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (evs_std_startup) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline evs_std_startup has the worst median (400.50 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest evs_engine_startup at 276.48 us).


### evs_engine_startup dominates: 45% faster than the next best (evs_std_startup)

evs_engine_startup (276.48 us) leads evs_std_startup (400.50 us) by 45%, a clear separation rather than a photo finish. CV 3.6%.


### evs_engine_startup beats baseline by 31% (significant)

evs_engine_startup is -125.54 us (31%) faster than baseline evs_std_startup, with a CI that excludes zero.


### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


## Key findings

- Fastest: evs_engine_startup at 276479.2 ns median (-31.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.45x (fastest 276479.2 ns, slowest 400495.8 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine_startup | 280368ns | 278288ns | 270492ns | 271997ns | 289115ns | -30.78% |
| evs_std_startup | 405021ns | 402671ns | 398550ns | 401239ns | 409475ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 278572ns | 268625ns | 287735ns | -30.78% | 15.056 |
| evs_std_startup | 402463ns | 395942ns | 406998ns | base | 10.422 |

## Performance model

- Peak throughput: 15.614 Gops/s (evs_engine_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 15.170 | 97.2% |
| evs_std_startup | 10.473 | 67.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 280368ns | 280368ns | -30.78% |
| evs_std_startup | 405021ns | 405021ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 400496ns | base | --- | [397300, 407850] | --- | --- | --- | --- |
| evs_engine_startup | 276479ns | -125541.7ns (-31.3%) | [-129475, -119092]ns | [268950, 287750] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 398054ns | -32.4% |
| 1 | 2 | 404088ns | -29.2% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine_startup | -0.500 | moderate- |
| evs_std_startup | -0.500 | moderate- |

Consistency summary:

- evs_engine_startup: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine_startup | 26146962.5ns | 278571.9ns | 9386.1% | HIGH |
| evs_std_startup | 23722603.1ns | 402462.5ns | 5894.4% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 268625.0-287735.4 ns)
  268625.0 |########################################
  269580.5 |
  270536.0 |#############
  271491.6 |
  272447.1 |
  273402.6 |
  274358.1 |
  275313.6 |
  276269.2 |
  277224.7 |
  278180.2 |
  279135.7 |
  280091.2 |
  281046.8 |
  282002.3 |#############
  282957.8 |
  283913.3 |#############
  284868.8 |
  285824.4 |
  286779.9 |
  (0 below, 2 above range)

evs_std_startup (n=8, range 395941.7-406997.9 ns)
  395941.7 |########################################
  396494.5 |
  397047.3 |########################################
  397600.1 |
  398152.9 |
  398705.8 |########################################
  399258.6 |########################################
  399811.4 |
  400364.2 |
  400917.0 |########################################
  401469.8 |
  402022.6 |
  402575.4 |
  403128.2 |
  403681.1 |
  404233.9 |
  404786.7 |
  405339.5 |
  405892.3 |
  406445.1 |########################################
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine_startup: bridge=9450.8% of algo (FFI overhead may distort results)
- evs_std_startup: bridge=5923.9% of algo (FFI overhead may distort results)

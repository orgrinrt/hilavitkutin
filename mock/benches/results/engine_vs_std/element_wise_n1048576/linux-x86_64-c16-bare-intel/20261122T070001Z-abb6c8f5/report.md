# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (evs_std) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline evs_std has the worst median (420.03 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest evs_engine at 380.90 us).


### evs_engine dominates: 10% faster than the next best (evs_std)

evs_engine (380.90 us) leads evs_std (420.03 us) by 10%, a clear separation rather than a photo finish. CV 1.2%.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


## Key findings

- Fastest: evs_engine at 380897.2 ns median (-9.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.10x (fastest 380897.2 ns, slowest 420026.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | element_wise, four-stage RAW chain |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000001 |
| Counter frequency | 2112534967 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 381220ns | 381634ns | 374711ns | 378166ns | 385137ns | -9.29% |
| evs_std | 420261ns | 420572ns | 410337ns | 417609ns | 424731ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 380577ns | 374126ns | 384421ns | -9.31% | 2.755 |
| evs_std | 419632ns | 409603ns | 424168ns | base | 2.499 |

## Performance model

- Peak throughput: 2.803 Gops/s (evs_engine; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 2.753 | 98.2% |
| evs_std | 2.496 | 89.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 381220ns | 381220ns | -9.29% |
| evs_std | 420261ns | 420261ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 420026ns | base | --- | [414004, 425104] | --- | --- | --- | --- |
| evs_engine | 380897ns | -39382.2ns (-9.4%) | [-48911, -35297]ns | [374928, 383328] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 421537ns | -9.6% |
| 1 | 2 | 416637ns | -9.0% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 4083493.0ns | 380577.4ns | 1073.0% | HIGH |
| evs_std | 4108110.8ns | 419632.2ns | 979.0% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 374125.8-384420.8 ns)
  374125.8 |########################################
  374640.6 |########################################
  375155.3 |
  375670.1 |
  376184.8 |
  376699.6 |
  377214.3 |
  377729.1 |
  378243.8 |########################################
  378758.6 |
  379273.3 |########################################
  379788.1 |
  380302.8 |
  380817.6 |
  381332.3 |
  381847.1 |########################################
  382361.8 |########################################
  382876.6 |########################################
  383391.3 |
  383906.1 |
  (0 below, 1 above range)

evs_std (n=8, range 409603.3-424168.0 ns)
  409603.3 |########################################
  410331.5 |
  411059.8 |
  411788.0 |
  412516.2 |
  413244.5 |
  413972.7 |########################################
  414700.9 |
  415429.2 |
  416157.4 |
  416885.6 |########################################
  417613.9 |
  418342.1 |
  419070.3 |########################################
  419798.6 |
  420526.8 |########################################
  421255.0 |
  421983.3 |########################################
  422711.5 |
  423439.7 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=1071.2% of algo (FFI overhead may distort results)
- evs_std: bridge=977.6% of algo (FFI overhead may distort results)

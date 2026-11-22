# accumulator, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (evs_std_startup) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline evs_std_startup has the worst median (465.69 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest evs_engine_startup at 283.08 us).


### evs_engine_startup dominates: 65% faster than the next best (evs_std_startup)

evs_engine_startup (283.08 us) leads evs_std_startup (465.69 us) by 65%, a clear separation rather than a photo finish. CV 4.1%.


### evs_engine_startup beats baseline by 38% (significant)

evs_engine_startup is -177.30 us (38%) faster than baseline evs_std_startup, with a CI that excludes zero.


### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


## Key findings

- Fastest: evs_engine_startup at 283079.2 ns median (-39.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.65x (fastest 283079.2 ns, slowest 465687.5 ns)

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
| evs_engine_startup | 287607ns | 284617ns | 272858ns | 280956ns | 296283ns | -36.95% |
| evs_std_startup | 456193ns | 467788ns | 403500ns | 441386ns | 480471ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 286089ns | 271483ns | 294775ns | -37.03% | 14.661 |
| evs_std_startup | 454320ns | 401358ns | 478556ns | base | 9.232 |

## Performance model

- Peak throughput: 15.450 Gops/s (evs_engine_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 14.817 | 95.9% |
| evs_std_startup | 9.007 | 58.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 287607ns | 287607ns | -36.95% |
| evs_std_startup | 456193ns | 456193ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 465688ns | base | --- | [426042, 478533] | --- | --- | --- | --- |
| evs_engine_startup | 283079ns | -177300.0ns (-38.1%) | [-190525, -139667]ns | [276958, 303408] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 466775ns | -39.9% |
| 1 | 2 | 454488ns | -37.3% |

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
| evs_engine_startup | 26152622.9ns | 286088.5ns | 9141.4% | HIGH |
| evs_std_startup | 23988396.9ns | 454319.8ns | 5280.1% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 271483.3-294775.0 ns)
  271483.3 |########################################
  272647.9 |
  273812.5 |
  274977.1 |
  276141.6 |########################################
  277306.2 |########################################
  278470.8 |
  279635.4 |
  280800.0 |
  281964.6 |########################################
  283129.1 |########################################
  284293.7 |
  285458.3 |########################################
  286622.9 |
  287787.5 |
  288952.1 |
  290116.6 |
  291281.2 |
  292445.8 |
  293610.4 |
  (0 below, 2 above range)

evs_std_startup (n=8, range 401358.3-478556.2 ns)
  401358.3 |####################
  405218.2 |
  409078.1 |
  412938.0 |
  416797.9 |
  420657.8 |
  424517.7 |####################
  428377.6 |
  432237.5 |####################
  436097.4 |
  439957.3 |
  443817.2 |
  447677.1 |
  451537.0 |
  455396.8 |####################
  459256.7 |
  463116.6 |
  466976.5 |
  470836.4 |####################
  474696.3 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine_startup: bridge=9230.6% of algo (FFI overhead may distort results)
- evs_std_startup: bridge=5130.9% of algo (FFI overhead may distort results)

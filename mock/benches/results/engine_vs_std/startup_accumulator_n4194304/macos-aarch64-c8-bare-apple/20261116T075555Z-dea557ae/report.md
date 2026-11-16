# accumulator, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (evs_std_startup) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline evs_std_startup has the worst median (445.54 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest evs_engine_startup at 285.15 us).


### evs_engine_startup dominates: 56% faster than the next best (evs_std_startup)

evs_engine_startup (285.15 us) leads evs_std_startup (445.54 us) by 56%, a clear separation rather than a photo finish. CV 6.3%.


### evs_engine_startup beats baseline by 35% (significant)

evs_engine_startup is -158.14 us (35%) faster than baseline evs_std_startup, with a CI that excludes zero.


### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


## Key findings

- Fastest: evs_engine_startup at 285150.0 ns median (-36.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.56x (fastest 285150.0 ns, slowest 445537.5 ns)

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
| evs_engine_startup | 293177ns | 287696ns | 277350ns | 280261ns | 306821ns | -37.27% |
| evs_std_startup | 467360ns | 447612ns | 410367ns | 418844ns | 517996ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 290559ns | 273958ns | 304162ns | -37.50% | 14.435 |
| evs_std_startup | 464879ns | 407908ns | 515323ns | base | 9.022 |

## Performance model

- Peak throughput: 15.310 Gops/s (evs_engine_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 14.709 | 96.1% |
| evs_std_startup | 9.414 | 61.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 293177ns | 293177ns | -37.27% |
| evs_std_startup | 467360ns | 467360ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 445538ns | base | --- | [409933, 526300] | --- | --- | --- | --- |
| evs_engine_startup | 285150ns | -158141.7ns (-35.5%) | [-237417, -132417]ns | [274933, 303758] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 419950ns | -33.5% |
| 1 | 2 | 499221ns | -40.6% |

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
| evs_engine_startup | 26391826.0ns | 290559.4ns | 9083.1% | HIGH |
| evs_std_startup | 23989787.5ns | 464879.2ns | 5160.4% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 273958.3-304162.5 ns)
  273958.3 |########################################
  275468.5 |
  276978.7 |####################
  278488.9 |
  279999.1 |####################
  281509.3 |
  283019.6 |
  284529.8 |
  286040.0 |
  287550.2 |####################
  289060.4 |
  290570.6 |
  292080.8 |####################
  293591.0 |
  295101.2 |
  296611.4 |
  298121.6 |
  299631.8 |
  301142.1 |
  302652.3 |####################
  (0 below, 1 above range)

evs_std_startup (n=8, range 407908.3-515322.9 ns)
  407908.3 |########################################
  413279.0 |####################
  418649.8 |####################
  424020.5 |
  429391.2 |
  434762.0 |
  440132.7 |
  445503.4 |
  450874.1 |
  456244.9 |
  461615.6 |
  466986.3 |########################################
  472357.1 |
  477727.8 |
  483098.5 |
  488469.3 |
  493840.0 |
  499210.7 |
  504581.5 |
  509952.2 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine_startup: bridge=9239.2% of algo (FFI overhead may distort results)
- evs_std_startup: bridge=5358.7% of algo (FFI overhead may distort results)

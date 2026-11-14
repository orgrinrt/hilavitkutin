# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (evs_std) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline evs_std has the worst median (585.87 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest evs_engine at 566.54 us).


### evs_engine is fastest but the noisiest (CV 10.9%)

evs_engine wins on median (566.54 us) yet has the highest variance (CV 10.9%), while evs_std is the steadiest (CV 6.4%, 585.87 us).


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (19.33 us) is smaller than the fastest variant's own run-to-run std-dev (61.63 us); the ranking is inside the noise.


### Speed leader evs_engine vs stability leader evs_std (+3% speed for 1.7x steadier)

evs_engine is fastest (566.54 us, CV 10.9%); evs_std gives up 3.4% median for 1.7x lower variance (CV 6.4%).


## Key findings

- Fastest: evs_engine at 566541.2 ns median (-3.3% vs baseline)
- Spread: 1.03x (fastest 566541.2 ns, slowest 585871.2 ns)

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
| Counter frequency | 2100267710 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 554007ns | 568767ns | 455679ns | 513062ns | 609298ns | -5.16% |
| evs_std | 584128ns | 588195ns | 497147ns | 578394ns | 610175ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 551859ns | 453893ns | 606752ns | -5.09% | 1.900 |
| evs_std | 581456ns | 493288ns | 607493ns | base | 1.803 |

## Performance model

- Peak throughput: 2.310 Gops/s (evs_engine; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.851 | 80.1% |
| evs_std | 1.790 | 77.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 554007ns | 554007ns | -5.16% |
| evs_std | 584128ns | 584128ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 585871ns | base | --- | [569758, 615804] | --- | --- | --- | --- |
| evs_engine | 566541ns | no significant difference | [-100252, +19033]ns | [469506, 618731] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 596139ns | +1.8% |
| 1 | 2 | 585871ns | -15.5% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 1/2, lost 1/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 5202891.4ns | 551859.3ns | 942.8% | HIGH |
| evs_std | 5583905.5ns | 581456.3ns | 960.3% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 453892.6-606752.5 ns)
  453892.6 |########################################
  461535.6 |
  469178.6 |########################################
  476821.6 |
  484464.6 |
  492107.6 |
  499750.6 |
  507393.6 |
  515036.6 |########################################
  522679.6 |
  530322.6 |
  537965.5 |########################################
  545608.5 |
  553251.5 |
  560894.5 |
  568537.5 |
  576180.5 |
  583823.5 |########################################
  591466.5 |########################################
  599109.5 |
  (0 below, 2 above range)

evs_std (n=8, range 493288.5-607492.9 ns)
  493288.5 |########################################
  498998.7 |
  504708.9 |
  510419.2 |
  516129.4 |
  521839.6 |
  527549.8 |
  533260.0 |
  538970.3 |
  544680.5 |
  550390.7 |
  556100.9 |
  561811.2 |
  567521.4 |########################################
  573231.6 |########################################
  578941.8 |########################################
  584652.0 |########################################
  590362.3 |
  596072.5 |
  601782.7 |
  (0 below, 3 above range)

```

## Diagnostics

- evs_engine: bridge=944.1% of algo (FFI overhead may distort results)
- evs_std: bridge=965.8% of algo (FFI overhead may distort results)

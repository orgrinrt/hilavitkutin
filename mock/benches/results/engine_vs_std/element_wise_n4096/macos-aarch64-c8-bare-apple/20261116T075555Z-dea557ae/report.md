# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 13% faster than the next best (evs_engine)

evs_std (412 ns) leads evs_engine (467 ns) by 13%, a clear separation rather than a photo finish. CV 1.7%.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (412 ns median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 412.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.13x (fastest 412.5 ns, slowest 466.7 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 646ns | 583ns | 550ns | 558ns | 735ns | +25.00% |
| evs_std | 517ns | 508ns | 492ns | 500ns | 535ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 466ns | 458ns | 471ns | +13.45% | 8.797 |
| evs_std | 410ns | 400ns | 417ns | base | 9.980 |

## Performance model

- Peak throughput: 10.240 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 8.777 | 85.7% |
| evs_std | 9.930 | 97.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 646ns | 646ns | +25.00% |
| evs_std | 517ns | 517ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 412ns | base | --- | [400, 417] | --- | --- | --- | --- |
| evs_engine | 467ns | +58.3ns (+14.1%) | [+42, +67]ns | [458, 467] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 417ns | +10.0% |
| 1 | 2 | 408ns | +14.3% |

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
| evs_engine | 22569.8ns | 465.6ns | 4847.2% | HIGH |
| evs_std | 22320.8ns | 410.4ns | 5438.5% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 458.3-470.8 ns)
    458.3 |##############################
    458.9 |
    459.6 |
    460.2 |
    460.8 |
    461.4 |
    462.1 |
    462.7 |
    463.3 |
    463.9 |
    464.6 |
    465.2 |
    465.8 |
    466.5 |########################################
    467.1 |
    467.7 |
    468.3 |
    469.0 |
    469.6 |
    470.2 |
  (0 below, 1 above range)

evs_std (n=8, range 400.0-416.7 ns)
    400.0 |########################################
    400.8 |
    401.7 |
    402.5 |
    403.3 |
    404.2 |
    405.0 |
    405.8 |
    406.7 |
    407.5 |########################################
    408.4 |
    409.2 |
    410.0 |
    410.9 |
    411.7 |
    412.5 |
    413.4 |
    414.2 |
    415.0 |
    415.9 |
  (0 below, 4 above range)

```

## Diagnostics

- evs_engine: bridge=4784.5% of algo (FFI overhead may distort results)
- evs_std: bridge=5407.1% of algo (FFI overhead may distort results)

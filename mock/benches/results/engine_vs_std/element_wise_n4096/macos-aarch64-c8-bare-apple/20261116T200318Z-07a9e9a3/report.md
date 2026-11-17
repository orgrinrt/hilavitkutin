# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 12% faster than the next best (evs_engine)

evs_std (408 ns) leads evs_engine (458 ns) by 12%, a clear separation rather than a photo finish. CV 1.9%.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (408 ns median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 408.3 ns median
- 1 variant significantly slower than baseline
- Spread: 1.12x (fastest 408.3 ns, slowest 458.3 ns)

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
| evs_engine | 571ns | 562ns | 542ns | 553ns | 592ns | +9.82% |
| evs_std | 520ns | 508ns | 483ns | 503ns | 542ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 459ns | 442ns | 465ns | +12.79% | 8.917 |
| evs_std | 407ns | 392ns | 412ns | base | 10.057 |

## Performance model

- Peak throughput: 10.457 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 8.937 | 85.5% |
| evs_std | 10.032 | 95.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 571ns | 571ns | +9.82% |
| evs_std | 520ns | 520ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 408ns | base | --- | [400, 417] | --- | --- | --- | --- |
| evs_engine | 458ns | +50.0ns (+12.2%) | [+42, +67]ns | [458, 467] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 408ns | +12.2% |
| 1 | 2 | 412ns | +11.1% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | 0.000 | ok |
| evs_std | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 22119.8ns | 459.4ns | 4815.3% | HIGH |
| evs_std | 21893.8ns | 407.3ns | 5375.5% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 441.7-464.6 ns)
    441.7 |########
    442.8 |
    444.0 |
    445.1 |
    446.3 |
    447.4 |
    448.6 |
    449.7 |
    450.8 |
    452.0 |
    453.1 |
    454.3 |
    455.4 |
    456.6 |
    457.7 |########################################
    458.9 |
    460.0 |
    461.1 |
    462.3 |
    463.4 |
  (0 below, 2 above range)

evs_std (n=8, range 391.7-412.5 ns)
    391.7 |##########
    392.7 |
    393.8 |
    394.8 |
    395.9 |
    396.9 |
    397.9 |
    399.0 |##########
    400.0 |
    401.1 |
    402.1 |
    403.1 |
    404.2 |
    405.2 |
    406.3 |
    407.3 |########################################
    408.3 |
    409.4 |
    410.4 |
    411.5 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=4777.6% of algo (FFI overhead may distort results)
- evs_std: bridge=5361.7% of algo (FFI overhead may distort results)

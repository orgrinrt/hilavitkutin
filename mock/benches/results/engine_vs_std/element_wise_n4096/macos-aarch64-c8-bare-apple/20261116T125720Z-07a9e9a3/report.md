# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 12% faster than the next best (evs_engine)

evs_std (408 ns) leads evs_engine (458 ns) by 12%, a clear separation rather than a photo finish. CV 1.4%.


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
| evs_engine | 569ns | 558ns | 542ns | 544ns | 594ns | +10.98% |
| evs_std | 512ns | 500ns | 500ns | 500ns | 525ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 460ns | 450ns | 467ns | +12.75% | 8.897 |
| evs_std | 408ns | 400ns | 412ns | base | 10.031 |

## Performance model

- Peak throughput: 10.240 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 8.937 | 87.3% |
| evs_std | 10.032 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 569ns | 569ns | +10.98% |
| evs_std | 512ns | 512ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 408ns | base | --- | [400, 417] | --- | --- | --- | --- |
| evs_engine | 458ns | +50.0ns (+12.2%) | [+42, +67]ns | [450, 467] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 408ns | +11.2% |
| 1 | 2 | 412ns | +12.1% |

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
| evs_engine | 21897.9ns | 460.4ns | 4756.3% | HIGH |
| evs_std | 21894.8ns | 408.3ns | 5362.1% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 450.0-466.6 ns)
    450.0 |####################
    450.8 |
    451.7 |
    452.5 |
    453.3 |
    454.2 |
    455.0 |
    455.8 |
    456.7 |
    457.5 |########################################
    458.3 |
    459.2 |
    460.0 |
    460.8 |
    461.7 |
    462.5 |
    463.3 |
    464.2 |
    465.0 |
    465.8 |
  (0 below, 2 above range)

evs_std (n=8, range 400.0-412.5 ns)
    400.0 |####################
    400.6 |
    401.2 |
    401.9 |
    402.5 |
    403.1 |
    403.8 |
    404.4 |
    405.0 |
    405.6 |
    406.2 |
    406.9 |
    407.5 |
    408.1 |########################################
    408.8 |
    409.4 |
    410.0 |
    410.6 |
    411.2 |
    411.9 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=4778.5% of algo (FFI overhead may distort results)
- evs_std: bridge=5361.7% of algo (FFI overhead may distort results)

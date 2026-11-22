# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (6.47 us median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 6475.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.03x (fastest 6475.0 ns, slowest 6654.1 ns)

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
| evs_engine | 7241ns | 7292ns | 6775ns | 7128ns | 7442ns | -1.14% |
| evs_std | 7324ns | 7367ns | 6642ns | 7103ns | 7660ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 6653ns | 6633ns | 6665ns | +2.75% | 9.850 |
| evs_std | 6475ns | 6458ns | 6485ns | base | 10.121 |

## Performance model

- Peak throughput: 10.148 Gops/s (evs_std; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 9.849 | 97.1% |
| evs_std | 10.121 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 7241ns | 7241ns | -1.14% |
| evs_std | 7324ns | 7324ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 6475ns | base | --- | [6458, 6475] | --- | --- | --- | --- |
| evs_engine | 6654ns | +179.2ns (+2.8%) | [+158, +200]ns | [6642, 6667] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 6475ns | +2.8% |
| 1 | 2 | 6471ns | +2.8% |

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
| evs_engine | 353001.0ns | 6653.1ns | 5305.8% | HIGH |
| evs_std | 352535.4ns | 6475.0ns | 5444.6% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 6633.3-6664.6 ns)
   6633.3 |####################
   6634.9 |
   6636.4 |
   6638.0 |
   6639.6 |
   6641.1 |########################################
   6642.7 |
   6644.2 |
   6645.8 |
   6647.4 |
   6648.9 |####################
   6650.5 |
   6652.1 |
   6653.6 |
   6655.2 |
   6656.8 |########################################
   6658.3 |
   6659.9 |
   6661.4 |
   6663.0 |
  (0 below, 2 above range)

evs_std (n=8, range 6458.3-6485.4 ns)
   6458.3 |####################
   6459.7 |
   6461.0 |
   6462.4 |
   6463.7 |
   6465.1 |
   6466.4 |##########
   6467.8 |
   6469.2 |
   6470.5 |
   6471.9 |
   6473.2 |
   6474.6 |########################################
   6475.9 |
   6477.3 |
   6478.6 |
   6480.0 |
   6481.4 |
   6482.7 |
   6484.1 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=5300.4% of algo (FFI overhead may distort results)
- evs_std: bridge=5441.2% of algo (FFI overhead may distort results)

# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (6.55 us median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 6545.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.03x (fastest 6545.9 ns, slowest 6729.2 ns)

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
| evs_engine | 6976ns | 6925ns | 6750ns | 6872ns | 7110ns | +1.36% |
| evs_std | 6882ns | 6746ns | 6600ns | 6694ns | 7094ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 6716ns | 6625ns | 6781ns | +2.64% | 9.759 |
| evs_std | 6543ns | 6467ns | 6590ns | base | 10.017 |

## Performance model

- Peak throughput: 10.134 Gops/s (evs_std; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 9.739 | 96.1% |
| evs_std | 10.012 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 6976ns | 6976ns | +1.36% |
| evs_std | 6882ns | 6882ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 6546ns | base | --- | [6475, 6592] | --- | --- | --- | --- |
| evs_engine | 6729ns | +183.3ns (+2.8%) | [+108, +217]ns | [6633, 6783] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 6558ns | +1.7% |
| 1 | 2 | 6542ns | +3.5% |

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
| evs_engine | 355719.8ns | 6715.6ns | 5296.9% | HIGH |
| evs_std | 356296.9ns | 6542.7ns | 5445.7% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 6625.0-6781.2 ns)
   6625.0 |########################################
   6632.8 |########################################
   6640.6 |
   6648.4 |########################################
   6656.2 |
   6664.1 |
   6671.9 |
   6679.7 |
   6687.5 |########################################
   6695.3 |
   6703.1 |
   6710.9 |
   6718.8 |
   6726.6 |
   6734.4 |
   6742.2 |
   6750.0 |
   6757.8 |
   6765.6 |########################################
   6773.4 |########################################
  (0 below, 2 above range)

evs_std (n=8, range 6466.7-6589.6 ns)
   6466.7 |########################################
   6472.8 |########################################
   6479.0 |
   6485.1 |
   6491.3 |
   6497.4 |
   6503.6 |
   6509.7 |
   6515.9 |########################################
   6522.0 |########################################
   6528.1 |
   6534.3 |
   6540.4 |
   6546.6 |
   6552.7 |
   6558.9 |
   6565.0 |########################################
   6571.2 |
   6577.3 |########################################
   6583.5 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=5279.1% of algo (FFI overhead may distort results)
- evs_std: bridge=5452.6% of algo (FFI overhead may distort results)

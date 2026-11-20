# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (521 ns) is smaller than the fastest variant's own run-to-run std-dev (544 ns); the ranking is inside the noise.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (5.54 us median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 5539.3 ns median
- 1 variant significantly slower than baseline
- Spread: 1.09x (fastest 5539.3 ns, slowest 6059.9 ns)

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
| Counter frequency | 2112872410 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 6413ns | 6437ns | 5483ns | 5931ns | 7007ns | +5.69% |
| evs_std | 6068ns | 5935ns | 5282ns | 5684ns | 6552ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 6062ns | 5225ns | 6626ns | +6.11% | 0.676 |
| evs_std | 5713ns | 5026ns | 6126ns | base | 0.717 |

## Performance model

- Peak throughput: 0.815 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.676 | 82.9% |
| evs_std | 0.739 | 90.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 6413ns | 6413ns | +5.69% |
| evs_std | 6068ns | 6068ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 5539ns | base | --- | [5117, 6148] | --- | --- | --- | --- |
| evs_engine | 6060ns | +287.8ns (+5.2%) | [+206, +520]ns | [5233, 6668] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 6087ns | +7.2% |
| 1 | 2 | 5318ns | +3.2% |

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
| evs_engine | 70358.3ns | 6061.6ns | 1160.7% | HIGH |
| evs_std | 69787.1ns | 5712.7ns | 1221.6% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 5225.3-6626.4 ns)
   5225.3 |########################################
   5295.4 |
   5365.4 |
   5435.5 |
   5505.5 |
   5575.6 |
   5645.6 |
   5715.7 |####################
   5785.8 |####################
   5855.8 |
   5925.9 |
   5995.9 |
   6066.0 |
   6136.0 |
   6206.1 |
   6276.2 |####################
   6346.2 |####################
   6416.3 |
   6486.3 |
   6556.4 |
  (0 below, 2 above range)

evs_std (n=8, range 5026.5-6126.0 ns)
   5026.5 |####################
   5081.5 |####################
   5136.5 |
   5191.4 |
   5246.4 |
   5301.4 |
   5356.4 |
   5411.3 |
   5466.3 |####################
   5521.3 |########################################
   5576.3 |
   5631.3 |
   5686.2 |
   5741.2 |
   5796.2 |
   5851.2 |
   5906.1 |
   5961.1 |
   6016.1 |####################
   6071.1 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=1171.9% of algo (FFI overhead may distort results)
- evs_std: bridge=1215.0% of algo (FFI overhead may distort results)

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

- Baseline (evs_std) is the fastest at 6466.7 ns median
- 1 variant significantly slower than baseline
- Spread: 1.05x (fastest 6466.7 ns, slowest 6783.3 ns)

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
| evs_engine | 7046ns | 7038ns | 6775ns | 6953ns | 7183ns | +4.38% |
| evs_std | 6750ns | 6583ns | 6567ns | 6572ns | 6929ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 6783ns | 6642ns | 6902ns | +4.44% | 9.661 |
| evs_std | 6495ns | 6458ns | 6527ns | base | 10.091 |

## Performance model

- Peak throughput: 10.148 Gops/s (evs_std; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 9.661 | 95.2% |
| evs_std | 10.134 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 7046ns | 7046ns | +4.38% |
| evs_std | 6750ns | 6750ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 6467ns | base | --- | [6458, 6533] | --- | --- | --- | --- |
| evs_engine | 6783ns | +258.3ns (+4.0%) | [+183, +425]ns | [6667, 6900] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 6471ns | +6.6% |
| 1 | 2 | 6462ns | +3.2% |

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
| evs_engine | 359689.6ns | 6783.3ns | 5302.5% | HIGH |
| evs_std | 353977.1ns | 6494.8ns | 5450.2% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 6641.7-6902.1 ns)
   6641.7 |####################
   6654.7 |########################################
   6667.7 |
   6680.8 |####################
   6693.8 |
   6706.8 |
   6719.8 |
   6732.8 |
   6745.8 |
   6758.9 |
   6771.9 |
   6784.9 |
   6797.9 |
   6810.9 |
   6824.0 |
   6837.0 |
   6850.0 |
   6863.0 |
   6876.0 |####################
   6889.1 |########################################
  (0 below, 1 above range)

evs_std (n=8, range 6458.3-6527.1 ns)
   6458.3 |##########################
   6461.7 |
   6465.2 |########################################
   6468.6 |
   6472.1 |#############
   6475.5 |
   6478.9 |
   6482.4 |
   6485.8 |
   6489.2 |
   6492.7 |
   6496.1 |
   6499.6 |
   6503.0 |
   6506.4 |
   6509.9 |
   6513.3 |
   6516.8 |
   6520.2 |
   6523.6 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=5317.0% of algo (FFI overhead may distort results)
- evs_std: bridge=5454.1% of algo (FFI overhead may distort results)

# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 105% faster than the next best (evs_engine)

evs_std (2.19 ms) leads evs_engine (4.48 ms) by 105%, a clear separation rather than a photo finish. CV 4.6%.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (2.19 ms median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 2187298.3 ns median
- 1 variant significantly slower than baseline
- Spread: 2.05x (fastest 2187298.3 ns, slowest 4481408.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | branching, two transforms joined by a third |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000002 |
| Counter frequency | 2112872410 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 4613056ns | 4482208ns | 4351224ns | 4425713ns | 4819020ns | +104.95% |
| evs_std | 2250769ns | 2188309ns | 2158089ns | 2180490ns | 2326647ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 4612090ns | 4350328ns | 4817944ns | +105.02% | 0.909 |
| evs_std | 2249570ns | 2157191ns | 2325233ns | base | 1.864 |

## Performance model

- Peak throughput: 1.944 Gops/s (evs_std; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.936 | 48.1% |
| evs_std | 1.918 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 4613056ns | 4613056ns | +104.95% |
| evs_std | 2250769ns | 2250769ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 2187298ns | base | --- | [2167798, 2371433] | --- | --- | --- | --- |
| evs_engine | 4481409ns | +2310026.0ns (+105.6%) | [+2161306, +2512440]ns | [4390729, 4948101] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 2280227ns | +94.5% |
| 1 | 2 | 2185319ns | +108.2% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_std | -0.500 | HIGH- (thermal bounce) |

Consistency summary:

- evs_engine: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 16896265.8ns | 4612089.7ns | 366.3% | HIGH |
| evs_std | 16677958.2ns | 2249569.6ns | 741.4% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 4350328.2-4817944.4 ns)
  4350328.2 |####################
  4373709.0 |####################
  4397089.8 |####################
  4420470.6 |
  4443851.4 |
  4467232.2 |########################################
  4490613.1 |
  4513993.9 |
  4537374.7 |
  4560755.5 |
  4584136.3 |
  4607517.1 |####################
  4630897.9 |
  4654278.7 |
  4677659.5 |
  4701040.3 |
  4724421.1 |
  4747801.9 |
  4771182.8 |
  4794563.6 |
  (0 below, 2 above range)

evs_std (n=8, range 2157190.7-2325232.6 ns)
  2157190.7 |#############
  2165592.8 |#############
  2173994.9 |
  2182397.0 |########################################
  2190799.1 |
  2199201.2 |
  2207603.3 |
  2216005.4 |
  2224407.5 |
  2232809.6 |
  2241211.6 |
  2249613.7 |
  2258015.8 |
  2266417.9 |
  2274820.0 |
  2283222.1 |
  2291624.2 |
  2300026.3 |#############
  2308428.4 |
  2316830.5 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=376.2% of algo (FFI overhead may distort results)
- evs_std: bridge=760.5% of algo (FFI overhead may distort results)

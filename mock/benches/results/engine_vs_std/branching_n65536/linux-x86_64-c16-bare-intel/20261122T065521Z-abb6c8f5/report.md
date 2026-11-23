# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 22% faster than the next best (evs_engine)

evs_std (29.75 us) leads evs_engine (36.19 us) by 22%, a clear separation rather than a photo finish. CV 1.6%.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (29.75 us median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 29751.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.22x (fastest 29751.2 ns, slowest 36193.4 ns)

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
| Counter frequency | 2112781463 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 36300ns | 36337ns | 35900ns | 36146ns | 36516ns | +22.61% |
| evs_std | 29605ns | 29881ns | 28749ns | 29339ns | 30019ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 36154ns | 35778ns | 36391ns | +22.64% | 1.813 |
| evs_std | 29479ns | 28645ns | 29874ns | base | 2.223 |

## Performance model

- Peak throughput: 2.288 Gops/s (evs_std; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.811 | 79.1% |
| evs_std | 2.203 | 96.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 36300ns | 36300ns | +22.61% |
| evs_std | 29605ns | 29605ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 29751ns | base | --- | [28862, 29925] | --- | --- | --- | --- |
| evs_engine | 36193ns | +6449.6ns (+21.7%) | [+5854, +7623]ns | [35779, 36369] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 29893ns | +20.3% |
| 1 | 2 | 28982ns | +25.3% |

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
| evs_engine | 245804.5ns | 36153.7ns | 679.9% | HIGH |
| evs_std | 250823.9ns | 29478.7ns | 850.9% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 35778.2-36391.0 ns)
  35778.2 |########################################
  35808.8 |
  35839.5 |
  35870.1 |
  35900.8 |
  35931.4 |
  35962.0 |####################
  35992.7 |
  36023.3 |
  36054.0 |
  36084.6 |
  36115.2 |####################
  36145.9 |
  36176.5 |
  36207.2 |
  36237.8 |
  36268.4 |####################
  36299.1 |
  36329.7 |
  36360.4 |########################################
  (0 below, 1 above range)

evs_std (n=8, range 28645.3-29873.8 ns)
  28645.3 |########################################
  28706.7 |
  28768.1 |
  28829.6 |########################################
  28891.0 |
  28952.4 |
  29013.8 |
  29075.3 |########################################
  29136.7 |
  29198.1 |
  29259.5 |
  29321.0 |
  29382.4 |
  29443.8 |
  29505.2 |
  29566.7 |
  29628.1 |
  29689.5 |########################################
  29750.9 |########################################
  29812.4 |########################################
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=677.8% of algo (FFI overhead may distort results)
- evs_std: bridge=823.6% of algo (FFI overhead may distort results)

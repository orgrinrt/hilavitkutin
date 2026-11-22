# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 166% faster than the next best (evs_engine)

evs_std (126.08 us) leads evs_engine (335.44 us) by 166%, a clear separation rather than a photo finish. CV 1.0%.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (126.08 us median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 126075.0 ns median
- 1 variant significantly slower than baseline
- Spread: 2.66x (fastest 126075.0 ns, slowest 335441.7 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 346761ns | 336400ns | 326658ns | 330403ns | 364056ns | +171.06% |
| evs_std | 127928ns | 127679ns | 126183ns | 127414ns | 128750ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 345473ns | 325250ns | 362735ns | +172.92% | 3.035 |
| evs_std | 126583ns | 125917ns | 127188ns | base | 8.284 |

## Performance model

- Peak throughput: 8.328 Gops/s (evs_std; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 3.126 | 37.5% |
| evs_std | 8.317 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 346761ns | 346761ns | +171.06% |
| evs_std | 127928ns | 127928ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 126075ns | base | --- | [125992, 126492] | --- | --- | --- | --- |
| evs_engine | 335442ns | +209370.8ns (+166.1%) | [+198850, +248950]ns | [326050, 374942] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 126446ns | +158.0% |
| 1 | 2 | 126004ns | +169.7% |

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
| evs_engine | 5695807.3ns | 345472.9ns | 1648.7% | HIGH |
| evs_std | 5662425.0ns | 126583.3ns | 4473.3% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 325250.0-362735.4 ns)
  325250.0 |########################################
  327124.3 |
  328998.5 |
  330872.8 |
  332747.1 |
  334621.4 |##########################
  336495.6 |
  338369.9 |
  340244.2 |
  342118.4 |#############
  343992.7 |
  345867.0 |
  347741.3 |
  349615.5 |
  351489.8 |
  353364.1 |
  355238.3 |
  357112.6 |
  358986.9 |
  360861.2 |
  (0 below, 2 above range)

evs_std (n=8, range 125916.7-127187.5 ns)
  125916.7 |#############
  125980.2 |########################################
  126043.8 |
  126107.3 |#############
  126170.9 |
  126234.4 |
  126297.9 |
  126361.5 |#############
  126425.0 |
  126488.6 |#############
  126552.1 |
  126615.7 |
  126679.2 |
  126742.7 |
  126806.3 |
  126869.8 |
  126933.4 |
  126996.9 |
  127060.4 |
  127124.0 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=1691.6% of algo (FFI overhead may distort results)
- evs_std: bridge=4489.8% of algo (FFI overhead may distort results)

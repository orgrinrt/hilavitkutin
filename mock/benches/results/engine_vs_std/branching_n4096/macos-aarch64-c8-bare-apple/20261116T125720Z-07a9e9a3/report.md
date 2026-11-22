# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 2210% faster than the next best (evs_engine)

evs_std (488 ns) leads evs_engine (11.26 us) by 2210%, a clear separation rather than a photo finish. CV 0.9%.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (488 ns median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 23.1x the fastest

Fastest evs_std (488 ns) to slowest evs_engine (11.26 us): 23.1x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std) is the fastest at 487.5 ns median
- 1 variant significantly slower than baseline
- Spread: 23.10x (fastest 487.5 ns, slowest 11262.5 ns)

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
| evs_engine | 11728ns | 11371ns | 11292ns | 11333ns | 12133ns | +1882.23% |
| evs_std | 592ns | 588ns | 567ns | 581ns | 606ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 11616ns | 11200ns | 12004ns | +2282.69% | 0.353 |
| evs_std | 487ns | 483ns | 492ns | base | 8.402 |

## Performance model

- Peak throughput: 8.475 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.364 | 4.3% |
| evs_std | 8.402 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 11728ns | 11728ns | +1882.23% |
| evs_std | 592ns | 592ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 488ns | base | --- | [483, 492] | --- | --- | --- | --- |
| evs_engine | 11262ns | +10775.0ns (+2210.3%) | [+10717, +11725]ns | [11208, 12217] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 483ns | +2328.6% |
| 1 | 2 | 492ns | +2184.6% |

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
| evs_engine | 21913.5ns | 11615.6ns | 188.7% | HIGH |
| evs_std | 21896.9ns | 487.5ns | 4491.7% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 11200.0-12004.2 ns)
  11200.0 |##########################
  11240.2 |########################################
  11280.4 |
  11320.6 |
  11360.8 |
  11401.0 |
  11441.3 |
  11481.5 |
  11521.7 |
  11561.9 |
  11602.1 |
  11642.3 |
  11682.5 |
  11722.7 |
  11762.9 |
  11803.1 |
  11843.3 |
  11883.5 |
  11923.8 |
  11964.0 |
  (0 below, 3 above range)

evs_std (n=8, range 483.3-491.7 ns)
    483.3 |########################################
    483.7 |
    484.1 |
    484.6 |
    485.0 |
    485.4 |
    485.8 |
    486.2 |
    486.7 |
    487.1 |
    487.5 |
    487.9 |
    488.3 |
    488.8 |
    489.2 |
    489.6 |
    490.0 |
    490.4 |
    490.9 |
    491.3 |
  (0 below, 4 above range)

```

## Diagnostics

- evs_engine: bridge=194.6% of algo (FFI overhead may distort results)
- evs_std: bridge=4491.5% of algo (FFI overhead may distort results)

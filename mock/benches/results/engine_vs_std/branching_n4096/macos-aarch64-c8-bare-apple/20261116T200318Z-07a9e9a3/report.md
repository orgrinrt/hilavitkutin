# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 2202% faster than the next best (evs_engine)

evs_std (488 ns) leads evs_engine (11.22 us) by 2202%, a clear separation rather than a photo finish. CV 1.5%.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (488 ns median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 23.0x the fastest

Fastest evs_std (488 ns) to slowest evs_engine (11.22 us): 23.0x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std) is the fastest at 487.5 ns median
- 1 variant significantly slower than baseline
- Spread: 23.02x (fastest 487.5 ns, slowest 11220.8 ns)

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
| evs_engine | 11390ns | 11312ns | 11042ns | 11228ns | 11598ns | +1808.20% |
| evs_std | 597ns | 588ns | 567ns | 581ns | 617ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 11279ns | 10925ns | 11490ns | +2213.67% | 0.363 |
| evs_std | 487ns | 475ns | 494ns | base | 8.402 |

## Performance model

- Peak throughput: 8.623 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.365 | 4.2% |
| evs_std | 8.402 | 97.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 11390ns | 11390ns | +1808.20% |
| evs_std | 597ns | 597ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 488ns | base | --- | [483, 492] | --- | --- | --- | --- |
| evs_engine | 11221ns | +10729.1ns (+2200.9%) | [+10450, +10750]ns | [10933, 11242] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 483ns | +2222.6% |
| 1 | 2 | 492ns | +2154.1% |

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
| evs_engine | 21902.1ns | 11279.1ns | 194.2% | HIGH |
| evs_std | 21896.9ns | 487.5ns | 4491.7% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 10925.0-11489.6 ns)
  10925.0 |####################
  10953.2 |
  10981.5 |
  11009.7 |
  11037.9 |
  11066.1 |
  11094.4 |
  11122.6 |
  11150.8 |
  11179.1 |
  11207.3 |########################################
  11235.5 |##########
  11263.7 |
  11292.0 |
  11320.2 |
  11348.4 |
  11376.7 |
  11404.9 |
  11433.1 |
  11461.3 |
  (0 below, 1 above range)

evs_std (n=8, range 475.0-493.8 ns)
    475.0 |#############
    475.9 |
    476.9 |
    477.8 |
    478.8 |
    479.7 |
    480.6 |
    481.6 |
    482.5 |########################################
    483.4 |
    484.4 |
    485.3 |
    486.3 |
    487.2 |
    488.1 |
    489.1 |
    490.0 |
    491.0 |########################################
    491.9 |
    492.8 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=195.2% of algo (FFI overhead may distort results)
- evs_std: bridge=4492.3% of algo (FFI overhead may distort results)

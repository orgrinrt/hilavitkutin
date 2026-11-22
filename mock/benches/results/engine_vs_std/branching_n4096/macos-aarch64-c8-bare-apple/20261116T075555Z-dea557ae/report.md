# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 2191% faster than the next best (evs_engine)

evs_std (500 ns) leads evs_engine (11.45 us) by 2191%, a clear separation rather than a photo finish. CV 1.0%.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (500 ns median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 22.9x the fastest

Fastest evs_std (500 ns) to slowest evs_engine (11.45 us): 22.9x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std) is the fastest at 500.0 ns median
- 1 variant significantly slower than baseline
- Spread: 22.91x (fastest 500.0 ns, slowest 11454.2 ns)

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
| evs_engine | 11862ns | 11567ns | 11317ns | 11414ns | 12335ns | +1840.06% |
| evs_std | 611ns | 604ns | 583ns | 594ns | 631ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 11745ns | 11225ns | 12206ns | +2244.10% | 0.349 |
| evs_std | 501ns | 492ns | 504ns | base | 8.175 |

## Performance model

- Peak throughput: 8.330 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.358 | 4.3% |
| evs_std | 8.192 | 98.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 11862ns | 11862ns | +1840.06% |
| evs_std | 611ns | 611ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 500ns | base | --- | [500, 508] | --- | --- | --- | --- |
| evs_engine | 11454ns | +10954.2ns (+2190.8%) | [+10725, +11975]ns | [11225, 12483] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 504ns | +2275.3% |
| 1 | 2 | 500ns | +2146.7% |

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
| evs_engine | 22241.7ns | 11744.8ns | 189.4% | HIGH |
| evs_std | 22462.5ns | 501.0ns | 4483.2% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 11225.0-12206.2 ns)
  11225.0 |########################################
  11274.1 |
  11323.1 |
  11372.2 |
  11421.2 |##########################
  11470.3 |
  11519.4 |
  11568.4 |
  11617.5 |
  11666.6 |
  11715.6 |
  11764.7 |
  11813.8 |
  11862.8 |
  11911.9 |
  11960.9 |
  12010.0 |
  12059.1 |
  12108.1 |
  12157.2 |
  (0 below, 3 above range)

evs_std (n=8, range 491.7-504.1 ns)
    491.7 |########
    492.3 |
    492.9 |
    493.6 |
    494.2 |
    494.8 |
    495.4 |
    496.1 |
    496.7 |
    497.3 |
    497.9 |
    498.5 |
    499.2 |
    499.8 |########################################
    500.4 |
    501.0 |
    501.7 |
    502.3 |
    502.9 |
    503.5 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=194.9% of algo (FFI overhead may distort results)
- evs_std: bridge=4463.3% of algo (FFI overhead may distort results)

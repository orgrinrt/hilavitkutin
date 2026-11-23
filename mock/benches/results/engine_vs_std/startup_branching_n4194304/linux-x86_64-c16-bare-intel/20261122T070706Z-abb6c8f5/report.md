# branching, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_startup dominates: 3604% faster than the next best (evs_engine_startup)

evs_std_startup (239.92 us) leads evs_engine_startup (8.89 ms) by 3604%, a clear separation rather than a photo finish. CV 4.6%.


### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std_startup)

The baseline evs_std_startup is the fastest (239.92 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 37.0x the fastest

Fastest evs_std_startup (239.92 us) to slowest evs_engine_startup (8.89 ms): 37.0x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std_startup) is the fastest at 239923.1 ns median
- 1 variant significantly slower than baseline
- Spread: 37.04x (fastest 239923.1 ns, slowest 8885575.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | branching, get-ready |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000010002 |
| Counter frequency | 2112783416 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine_startup | 8979807ns | 8887065ns | 8691749ns | 8784058ns | 9198633ns | +3658.64% |
| evs_std_startup | 238911ns | 241800ns | 221991ns | 233324ns | 247331ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 8978488ns | 8690506ns | 9197198ns | +3687.11% | 0.467 |
| evs_std_startup | 237080ns | 220007ns | 245590ns | base | 17.692 |

## Performance model

- Peak throughput: 19.064 Gops/s (evs_std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 0.472 | 2.5% |
| evs_std_startup | 17.482 | 91.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 8979807ns | 8979807ns | +3658.64% |
| evs_std_startup | 238911ns | 238911ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 239923ns | base | --- | [220394, 244948] | --- | --- | --- | --- |
| evs_engine_startup | 8885576ns | +8654793.8ns (+3607.3%) | [+8476508, +8962917]ns | [8721456, 9182925] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 227798ns | +3917.2% |
| 1 | 2 | 241936ns | +3509.5% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine_startup | -0.500 | moderate- |
| evs_std_startup | -0.500 | HIGH- (thermal bounce) |

Consistency summary:

- evs_engine_startup: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine_startup | 22024012.2ns | 8978488.1ns | 245.3% | HIGH |
| evs_std_startup | 29890579.3ns | 237079.9ns | 12607.8% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 8690506.2-9197197.9 ns)
  8690506.2 |####################
  8715840.8 |####################
  8741175.4 |####################
  8766510.0 |
  8791844.5 |
  8817179.1 |
  8842513.7 |
  8867848.3 |########################################
  8893182.9 |
  8918517.5 |
  8943852.1 |
  8969186.7 |
  8994521.2 |
  9019855.8 |
  9045190.4 |
  9070525.0 |
  9095859.6 |####################
  9121194.2 |
  9146528.8 |
  9171863.4 |####################
  (0 below, 1 above range)

evs_std_startup (n=8, range 220007.4-245590.0 ns)
  220007.4 |########################################
  221286.5 |
  222565.7 |
  223844.8 |
  225123.9 |
  226403.1 |
  227682.2 |
  228961.3 |
  230240.4 |
  231519.6 |
  232798.7 |
  234077.8 |####################
  235357.0 |
  236636.1 |
  237915.2 |####################
  239194.4 |
  240473.5 |####################
  241752.6 |####################
  243031.8 |
  244310.9 |####################
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine_startup: bridge=246.2% of algo (FFI overhead may distort results)
- evs_std_startup: bridge=12485.1% of algo (FFI overhead may distort results)

# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 478% faster than the next best (evs_engine)

evs_std (2.34 us) leads evs_engine (13.54 us) by 478%, a clear separation rather than a photo finish. CV 113.4%.


### evs_std is fastest but the noisiest (CV 113.4%)

evs_std wins on median (2.34 us) yet has the highest variance (CV 113.4%), while evs_engine is the steadiest (CV 12.6%, 13.54 us).


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (2.34 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 5.8x the fastest

Fastest evs_std (2.34 us) to slowest evs_engine (13.54 us): 5.8x. The strategy choice matters a lot for this workload.


### evs_std is inconsistent: worst-20% is 2.0x its best-20%

evs_std's best 20% of batches run at 2.31 us but its worst 20% at 4.58 us (2.0x) - a bimodal or bursty profile the median hides.


## Key findings

- Baseline (evs_std) is the fastest at 2342.2 ns median
- 1 variant significantly slower than baseline
- Spread: 5.78x (fastest 2342.2 ns, slowest 13539.3 ns)

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
| Counter frequency | 2100267710 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 14362ns | 13733ns | 13625ns | 13682ns | 15058ns | +293.37% |
| evs_std | 3651ns | 2556ns | 2462ns | 2501ns | 4811ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 14170ns | 13419ns | 14859ns | +310.97% | 0.289 |
| evs_std | 3448ns | 2308ns | 4584ns | base | 1.188 |

## Performance model

- Peak throughput: 1.775 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.303 | 17.0% |
| evs_std | 1.749 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 14362ns | 14362ns | +293.37% |
| evs_std | 3651ns | 3651ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 2342ns | base | --- | [2313, 3032] | --- | --- | --- | --- |
| evs_engine | 13539ns | +11157.3ns (+476.4%) | [+10531, +11286]ns | [13482, 13601] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 2314ns | +485.4% |
| 1 | 2 | 2757ns | +391.0% |

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
| evs_engine | 21412.5ns | 14170.3ns | 151.1% | HIGH |
| evs_std | 21763.1ns | 3448.0ns | 631.2% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 13419.1-14858.7 ns)
  13419.1 |##########################
  13491.1 |##########################
  13563.1 |########################################
  13635.0 |
  13707.0 |
  13779.0 |
  13851.0 |
  13923.0 |
  13995.0 |
  14066.9 |
  14138.9 |
  14210.9 |
  14282.9 |
  14354.9 |
  14426.8 |
  14498.8 |
  14570.8 |
  14642.8 |
  14714.8 |
  14786.7 |
  (0 below, 1 above range)

evs_std (n=8, range 2307.5-4583.9 ns)
   2307.5 |########################################
   2421.3 |########
   2535.1 |
   2649.0 |
   2762.8 |
   2876.6 |
   2990.4 |########
   3104.2 |
   3218.0 |
   3331.9 |
   3445.7 |
   3559.5 |
   3673.3 |
   3787.1 |
   3900.9 |
   4014.8 |
   4128.6 |
   4242.4 |
   4356.2 |
   4470.0 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=150.5% of algo (FFI overhead may distort results)
- evs_std: CV=77.1% (high variance, measurements may be unstable)
- evs_std: bridge=882.6% of algo (FFI overhead may distort results)

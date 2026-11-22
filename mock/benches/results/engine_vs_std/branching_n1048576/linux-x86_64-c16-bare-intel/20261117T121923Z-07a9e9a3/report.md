# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 73% faster than the next best (evs_engine)

evs_std (548.70 us) leads evs_engine (951.53 us) by 73%, a clear separation rather than a photo finish. CV 8.0%.


### evs_std is fastest but the noisiest (CV 8.0%)

evs_std wins on median (548.70 us) yet has the highest variance (CV 8.0%), while evs_engine is the steadiest (CV 2.2%, 951.53 us).


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (548.70 us median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 548702.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.73x (fastest 548702.0 ns, slowest 951530.9 ns)

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
| evs_engine | 952340ns | 952718ns | 925556ns | 936899ns | 970617ns | +73.32% |
| evs_std | 549464ns | 549409ns | 502276ns | 506396ns | 593562ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 951225ns | 924416ns | 969396ns | +73.34% | 1.102 |
| evs_std | 548762ns | 501749ns | 592785ns | base | 1.911 |

## Performance model

- Peak throughput: 2.090 Gops/s (evs_std; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.102 | 52.7% |
| evs_std | 1.911 | 91.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 952340ns | 952340ns | +73.32% |
| evs_std | 549464ns | 549464ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 548702ns | base | --- | [502908, 594216] | --- | --- | --- | --- |
| evs_engine | 951531ns | +389993.5ns (+71.1%) | [+338029, +469043]ns | [927875, 970954] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 504823ns | +92.3% |
| 1 | 2 | 593310ns | +58.4% |

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
| evs_engine | 4236702.9ns | 951224.7ns | 445.4% | HIGH |
| evs_std | 4111100.5ns | 548761.7ns | 749.2% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 924416.3-969396.1 ns)
  924416.3 |########################################
  926665.3 |########################################
  928914.3 |########################################
  931163.3 |
  933412.3 |
  935661.3 |
  937910.3 |
  940159.2 |
  942408.2 |
  944657.2 |
  946906.2 |
  949155.2 |########################################
  951404.2 |########################################
  953653.2 |
  955902.2 |
  958151.2 |
  960400.2 |
  962649.2 |
  964898.2 |
  967147.2 |
  (0 below, 3 above range)

evs_std (n=8, range 501749.3-592785.1 ns)
  501749.3 |########################################
  506301.1 |########################################
  510852.9 |
  515404.7 |
  519956.5 |
  524508.3 |
  529060.1 |
  533611.8 |
  538163.6 |
  542715.4 |
  547267.2 |
  551819.0 |
  556370.8 |
  560922.6 |
  565474.4 |
  570026.2 |
  574578.0 |
  579129.8 |
  583681.6 |
  588233.4 |########################################
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=445.4% of algo (FFI overhead may distort results)
- evs_std: bridge=749.7% of algo (FFI overhead may distort results)

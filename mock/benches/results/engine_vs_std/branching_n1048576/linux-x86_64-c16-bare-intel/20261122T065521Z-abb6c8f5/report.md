# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 81% faster than the next best (evs_engine)

evs_std (587.15 us) leads evs_engine (1.06 ms) by 81%, a clear separation rather than a photo finish. CV 5.4%.


### evs_std is fastest but the noisiest (CV 5.4%)

evs_std wins on median (587.15 us) yet has the highest variance (CV 5.4%), while evs_engine is the steadiest (CV 3.1%, 1.06 ms).


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (587.15 us median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 587150.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.81x (fastest 587150.9 ns, slowest 1061438.0 ns)

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
| evs_engine | 1052935ns | 1062199ns | 979313ns | 1042933ns | 1078843ns | +82.82% |
| evs_std | 575933ns | 587757ns | 520610ns | 564946ns | 598004ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 1052097ns | 978637ns | 1077881ns | +82.87% | 0.997 |
| evs_std | 575318ns | 520148ns | 597289ns | base | 1.823 |

## Performance model

- Peak throughput: 2.016 Gops/s (evs_std; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.988 | 49.0% |
| evs_std | 1.786 | 88.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 1052935ns | 1052935ns | +82.82% |
| evs_std | 575933ns | 575933ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 587151ns | base | --- | [526908, 591300] | --- | --- | --- | --- |
| evs_engine | 1061438ns | +484650.8ns (+82.5%) | [+458489, +494992]ns | [1035471, 1081676] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 553276ns | +87.3% |
| 1 | 2 | 589113ns | +83.4% |

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
| evs_engine | 4327253.9ns | 1052096.6ns | 411.3% | HIGH |
| evs_std | 4171609.5ns | 575317.5ns | 725.1% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 978636.7-1077880.7 ns)
  978636.7 |####################
  983598.9 |
  988561.1 |
  993523.3 |
  998485.5 |
  1003447.7 |
  1008409.9 |
  1013372.1 |
  1018334.3 |
  1023296.5 |
  1028258.7 |
  1033220.9 |########################################
  1038183.1 |
  1043145.3 |
  1048107.5 |
  1053069.7 |####################
  1058031.9 |
  1062994.1 |
  1067956.3 |####################
  1072918.5 |
  (0 below, 3 above range)

evs_std (n=8, range 520148.1-597288.8 ns)
  520148.1 |####################
  524005.1 |####################
  527862.2 |
  531719.2 |
  535576.2 |
  539433.3 |
  543290.3 |
  547147.4 |
  551004.4 |
  554861.4 |
  558718.5 |
  562575.5 |
  566432.5 |
  570289.6 |
  574146.6 |
  578003.6 |####################
  581860.7 |
  585717.7 |########################################
  589574.8 |########################################
  593431.8 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=402.3% of algo (FFI overhead may distort results)
- evs_std: bridge=708.3% of algo (FFI overhead may distort results)

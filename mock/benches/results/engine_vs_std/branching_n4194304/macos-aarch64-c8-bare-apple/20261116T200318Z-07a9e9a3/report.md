# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 233% faster than the next best (evs_engine)

evs_std (589.48 us) leads evs_engine (1.97 ms) by 233%, a clear separation rather than a photo finish. CV 6.0%.


### evs_std is fastest but the noisiest (CV 6.0%)

evs_std wins on median (589.48 us) yet has the highest variance (CV 6.0%), while evs_engine is the steadiest (CV 0.9%, 1.97 ms).


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (589.48 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 3.3x the fastest

Fastest evs_std (589.48 us) to slowest evs_engine (1.97 ms): 3.3x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std) is the fastest at 589475.0 ns median
- 1 variant significantly slower than baseline
- Spread: 3.33x (fastest 589475.0 ns, slowest 1965779.1 ns)

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
| evs_engine | 1971205ns | 1968229ns | 1954308ns | 1962153ns | 1982219ns | +224.02% |
| evs_std | 608366ns | 591525ns | 578517ns | 584975ns | 633371ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 1969200ns | 1952525ns | 1980367ns | +224.60% | 2.130 |
| evs_std | 606649ns | 577317ns | 631723ns | base | 6.914 |

## Performance model

- Peak throughput: 7.265 Gops/s (evs_std; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 2.134 | 29.4% |
| evs_std | 7.115 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 1971205ns | 1971205ns | +224.02% |
| evs_std | 608366ns | 608366ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 589475ns | base | --- | [579467, 652167] | --- | --- | --- | --- |
| evs_engine | 1965779ns | +1372412.5ns (+232.8%) | [+1312517, +1379058]ns | [1953992, 1974683] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 593633ns | +230.2% |
| 1 | 2 | 589475ns | +233.6% |

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
| evs_engine | 22803397.9ns | 1969200.0ns | 1158.0% | HIGH |
| evs_std | 22799734.4ns | 606649.0ns | 3758.3% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 1952525.0-1980366.7 ns)
  1952525.0 |########################################
  1953917.1 |########################################
  1955309.2 |
  1956701.3 |
  1958093.3 |
  1959485.4 |
  1960877.5 |########################################
  1962269.6 |
  1963661.7 |########################################
  1965053.8 |
  1966445.8 |########################################
  1967837.9 |########################################
  1969230.0 |
  1970622.1 |
  1972014.2 |
  1973406.3 |########################################
  1974798.3 |
  1976190.4 |
  1977582.5 |
  1978974.6 |
  (0 below, 1 above range)

evs_std (n=8, range 577316.7-631722.9 ns)
  577316.7 |########################################
  580037.0 |####################
  582757.3 |
  585477.6 |
  588197.9 |########################################
  590918.3 |
  593638.6 |
  596358.9 |
  599079.2 |
  601799.5 |
  604519.8 |####################
  607240.1 |
  609960.4 |
  612680.7 |
  615401.1 |
  618121.4 |
  620841.7 |
  623562.0 |
  626282.3 |
  629002.6 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=1159.2% of algo (FFI overhead may distort results)
- evs_std: bridge=3859.8% of algo (FFI overhead may distort results)

# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 12% faster than the next best (evs_engine)

evs_std (5.28 us) leads evs_engine (5.91 us) by 12%, a clear separation rather than a photo finish. CV 11.1%.


### evs_std is fastest but the noisiest (CV 11.1%)

evs_std wins on median (5.28 us) yet has the highest variance (CV 11.1%), while evs_engine is the steadiest (CV 8.4%, 5.91 us).


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (5.28 us median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 5276.4 ns median
- 1 variant significantly slower than baseline
- Spread: 1.12x (fastest 5276.4 ns, slowest 5910.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | element_wise, four-stage RAW chain |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000001 |
| Counter frequency | 2112534967 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 6334ns | 6275ns | 5553ns | 5972ns | 6801ns | +11.95% |
| evs_std | 5658ns | 5554ns | 4852ns | 5166ns | 6229ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 5996ns | 5292ns | 6404ns | +12.43% | 0.683 |
| evs_std | 5333ns | 4609ns | 5869ns | base | 0.768 |

## Performance model

- Peak throughput: 0.889 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.693 | 78.0% |
| evs_std | 0.776 | 87.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 6334ns | 6334ns | +11.95% |
| evs_std | 5658ns | 5658ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 5276ns | base | --- | [4610, 6146] | --- | --- | --- | --- |
| evs_engine | 5910ns | +495.2ns (+9.4%) | [+339, +1174]ns | [5378, 6533] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 4782ns | +16.7% |
| 1 | 2 | 5898ns | +9.8% |

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
| evs_engine | 68509.3ns | 5996.4ns | 1142.5% | HIGH |
| evs_std | 65366.0ns | 5333.4ns | 1225.6% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 5291.6-6403.8 ns)
   5291.6 |########################################
   5347.2 |########################################
   5402.8 |
   5458.4 |
   5514.0 |
   5569.6 |
   5625.3 |
   5680.9 |
   5736.5 |########################################
   5792.1 |
   5847.7 |########################################
   5903.3 |########################################
   5958.9 |
   6014.5 |
   6070.1 |
   6125.8 |
   6181.4 |
   6237.0 |
   6292.6 |
   6348.2 |
  (0 below, 3 above range)

evs_std (n=8, range 4609.1-5868.6 ns)
   4609.1 |########################################
   4672.1 |
   4735.1 |
   4798.0 |
   4861.0 |
   4924.0 |####################
   4987.0 |####################
   5049.9 |
   5112.9 |
   5175.9 |
   5238.9 |
   5301.8 |
   5364.8 |
   5427.8 |
   5490.8 |####################
   5553.7 |
   5616.7 |####################
   5679.7 |
   5742.7 |
   5805.6 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=1137.6% of algo (FFI overhead may distort results)
- evs_std: bridge=1222.4% of algo (FFI overhead may distort results)

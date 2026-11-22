# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 262% faster than the next best (evs_engine)

evs_std (7.75 us) leads evs_engine (28.03 us) by 262%, a clear separation rather than a photo finish. CV 0.1%.


### evs_engine shows alternating (throttle bounce) (autocorr -0.50)

evs_engine's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (7.75 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 3.6x the fastest

Fastest evs_std (7.75 us) to slowest evs_engine (28.03 us): 3.6x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std) is the fastest at 7745.9 ns median
- 1 variant significantly slower than baseline
- Spread: 3.62x (fastest 7745.9 ns, slowest 28033.3 ns)

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
| evs_engine | 28802ns | 28762ns | 28158ns | 28608ns | 29108ns | +248.81% |
| evs_std | 8257ns | 8375ns | 7842ns | 8036ns | 8527ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 28149ns | 27808ns | 28388ns | +263.46% | 2.328 |
| evs_std | 7745ns | 7733ns | 7752ns | base | 8.462 |

## Performance model

- Peak throughput: 8.475 Gops/s (evs_std; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 2.338 | 27.6% |
| evs_std | 8.461 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 28802ns | 28802ns | +248.81% |
| evs_std | 8257ns | 8257ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 7746ns | base | --- | [7733, 7750] | --- | --- | --- | --- |
| evs_engine | 28033ns | +20287.6ns (+261.9%) | [+20133, +20467]ns | [27875, 28217] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 7750ns | +263.9% |
| 1 | 2 | 7738ns | +260.9% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | HIGH- (thermal bounce) |
| evs_std | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 352893.8ns | 28149.0ns | 1253.7% | HIGH |
| evs_std | 352758.3ns | 7744.8ns | 4554.8% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 27808.3-28387.5 ns)
  27808.3 |########################################
  27837.3 |
  27866.2 |########################################
  27895.2 |
  27924.1 |
  27953.1 |########################################
  27982.1 |########################################
  28011.0 |
  28040.0 |
  28069.0 |########################################
  28097.9 |
  28126.9 |
  28155.8 |
  28184.8 |########################################
  28213.8 |########################################
  28242.7 |
  28271.7 |
  28300.6 |
  28329.6 |
  28358.6 |
  (0 below, 1 above range)

evs_std (n=8, range 7733.3-7752.1 ns)
   7733.3 |##########################
   7734.2 |
   7735.2 |
   7736.1 |
   7737.1 |
   7738.0 |
   7738.9 |
   7739.9 |
   7740.8 |##########################
   7741.7 |
   7742.7 |
   7743.6 |
   7744.6 |
   7745.5 |
   7746.4 |
   7747.4 |
   7748.3 |
   7749.3 |########################################
   7750.2 |
   7751.1 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=1257.0% of algo (FFI overhead may distort results)
- evs_std: bridge=4551.0% of algo (FFI overhead may distort results)

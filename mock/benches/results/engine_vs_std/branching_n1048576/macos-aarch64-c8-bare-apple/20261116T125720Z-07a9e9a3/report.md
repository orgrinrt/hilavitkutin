# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 164% faster than the next best (evs_engine)

evs_std (125.36 us) leads evs_engine (330.54 us) by 164%, a clear separation rather than a photo finish. CV 0.7%.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (125.36 us median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 125358.4 ns median
- 1 variant significantly slower than baseline
- Spread: 2.64x (fastest 125358.4 ns, slowest 330541.7 ns)

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
| evs_engine | 331468ns | 332667ns | 328292ns | 330169ns | 333235ns | +159.62% |
| evs_std | 127673ns | 127579ns | 126608ns | 127228ns | 128273ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 329472ns | 326233ns | 331394ns | +161.93% | 3.183 |
| evs_std | 125784ns | 125283ns | 126248ns | base | 8.336 |

## Performance model

- Peak throughput: 8.370 Gops/s (evs_std; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 3.172 | 37.9% |
| evs_std | 8.365 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 331468ns | 331468ns | +159.62% |
| evs_std | 127673ns | 127673ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 125358ns | base | --- | [125325, 126108] | --- | --- | --- | --- |
| evs_engine | 330542ns | +204220.9ns (+162.9%) | [+201467, +205767]ns | [326800, 331150] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 125350ns | +164.1% |
| 1 | 2 | 125371ns | +160.7% |

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
| evs_engine | 5670888.6ns | 329471.9ns | 1721.2% | HIGH |
| evs_std | 5648162.5ns | 125784.4ns | 4490.4% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 326233.3-331393.8 ns)
  326233.3 |####################
  326491.3 |
  326749.3 |########################################
  327007.4 |
  327265.4 |
  327523.4 |
  327781.4 |
  328039.5 |
  328297.5 |
  328555.5 |
  328813.5 |
  329071.5 |
  329329.6 |
  329587.6 |
  329845.6 |
  330103.6 |####################
  330361.7 |
  330619.7 |
  330877.7 |########################################
  331135.7 |####################
  (0 below, 1 above range)

evs_std (n=8, range 125283.3-126247.9 ns)
  125283.3 |##########################
  125331.5 |########################################
  125379.8 |#############
  125428.0 |
  125476.2 |
  125524.4 |
  125572.7 |
  125620.9 |
  125669.1 |
  125717.4 |
  125765.6 |
  125813.8 |
  125862.1 |
  125910.3 |
  125958.5 |
  126006.8 |
  126055.0 |
  126103.2 |#############
  126151.4 |
  126199.7 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=1714.2% of algo (FFI overhead may distort results)
- evs_std: bridge=4502.0% of algo (FFI overhead may distort results)

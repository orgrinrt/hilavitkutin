# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 169% faster than the next best (evs_engine)

evs_std (125.93 us) leads evs_engine (338.66 us) by 169%, a clear separation rather than a photo finish. CV 0.4%.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (125.93 us median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 125933.4 ns median
- 1 variant significantly slower than baseline
- Spread: 2.69x (fastest 125933.4 ns, slowest 338658.3 ns)

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
| evs_engine | 344423ns | 339517ns | 326683ns | 334158ns | 356556ns | +170.12% |
| evs_std | 127506ns | 127408ns | 126508ns | 126961ns | 128165ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 342972ns | 325150ns | 355108ns | +171.77% | 3.057 |
| evs_std | 126198ns | 125833ns | 126525ns | base | 8.309 |

## Performance model

- Peak throughput: 8.333 Gops/s (evs_std; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 3.096 | 37.2% |
| evs_std | 8.326 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 344423ns | 344423ns | +170.12% |
| evs_std | 127506ns | 127506ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 125933ns | base | --- | [125858, 126725] | --- | --- | --- | --- |
| evs_engine | 338658ns | +212116.6ns (+168.4%) | [+199442, +233350]ns | [325300, 359242] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 126075ns | +164.6% |
| 1 | 2 | 125933ns | +168.9% |

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
| evs_engine | 5705268.8ns | 342971.9ns | 1663.5% | HIGH |
| evs_std | 5647590.6ns | 126197.9ns | 4475.2% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 325150.0-355108.3 ns)
  325150.0 |########################################
  326647.9 |
  328145.8 |
  329643.8 |
  331141.7 |
  332639.6 |
  334137.5 |####################
  335635.4 |
  337133.3 |####################
  338631.3 |####################
  340129.2 |
  341627.1 |####################
  343125.0 |
  344622.9 |
  346120.8 |
  347618.8 |
  349116.7 |
  350614.6 |
  352112.5 |
  353610.4 |
  (0 below, 2 above range)

evs_std (n=8, range 125833.3-126525.0 ns)
  125833.3 |########################################
  125867.9 |########################################
  125902.5 |
  125937.1 |####################
  125971.6 |
  126006.2 |
  126040.8 |
  126075.4 |
  126110.0 |
  126144.6 |
  126179.2 |
  126213.7 |
  126248.3 |
  126282.9 |####################
  126317.5 |
  126352.1 |
  126386.7 |
  126421.3 |
  126455.9 |
  126490.4 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=1678.1% of algo (FFI overhead may distort results)
- evs_std: bridge=4483.7% of algo (FFI overhead may distort results)

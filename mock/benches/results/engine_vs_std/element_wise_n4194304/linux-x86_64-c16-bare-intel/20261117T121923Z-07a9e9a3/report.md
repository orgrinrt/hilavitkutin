# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (evs_std) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline evs_std has the worst median (2.18 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest evs_engine at 1.94 ms).


### evs_engine dominates: 12% faster than the next best (evs_std)

evs_engine (1.94 ms) leads evs_std (2.18 ms) by 12%, a clear separation rather than a photo finish. CV 9.4%.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


## Key findings

- Fastest: evs_engine at 1940253.6 ns median (-10.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.12x (fastest 1940253.6 ns, slowest 2178219.9 ns)

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
| Counter frequency | 2112872410 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 2016119ns | 1941138ns | 1869643ns | 1911712ns | 2131042ns | -7.43% |
| evs_std | 2177998ns | 2179267ns | 1774291ns | 2101370ns | 2336396ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 2015072ns | 1868691ns | 2129896ns | -7.43% | 2.081 |
| evs_std | 2176897ns | 1773297ns | 2335197ns | base | 1.927 |

## Performance model

- Peak throughput: 2.365 Gops/s (evs_std; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 2.162 | 91.4% |
| evs_std | 1.926 | 81.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 2016119ns | 2016119ns | -7.43% |
| evs_std | 2177998ns | 2177998ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 2178220ns | base | --- | [1998270, 2452978] | --- | --- | --- | --- |
| evs_engine | 1940254ns | -206044.8ns (-9.5%) | [-279144, -22108]ns | [1882747, 2185752] | YES (adj: no) | 0.0703 | 0.0703 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 2178220ns | -10.4% |
| 1 | 2 | 2109905ns | -10.0% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 16624793.2ns | 2015071.5ns | 825.0% | HIGH |
| evs_std | 17196573.3ns | 2176897.1ns | 790.0% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 1868691.1-2129896.4 ns)
  1868691.1 |####################
  1881751.4 |####################
  1894811.6 |
  1907871.9 |####################
  1920932.2 |
  1933992.4 |########################################
  1947052.7 |####################
  1960113.0 |
  1973173.2 |
  1986233.5 |
  1999293.8 |
  2012354.0 |
  2025414.3 |
  2038474.5 |
  2051534.8 |
  2064595.1 |
  2077655.3 |
  2090715.6 |
  2103775.9 |
  2116836.1 |
  (0 below, 2 above range)

evs_std (n=8, range 1773297.1-2335197.2 ns)
  1773297.1 |####################
  1801392.1 |
  1829487.1 |
  1857582.1 |
  1885677.1 |
  1913772.1 |
  1941867.1 |
  1969962.2 |
  1998057.2 |####################
  2026152.2 |
  2054247.2 |
  2082342.2 |
  2110437.2 |
  2138532.2 |########################################
  2166627.2 |
  2194722.2 |########################################
  2222817.2 |
  2250912.2 |
  2279007.2 |
  2307102.2 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=855.7% of algo (FFI overhead may distort results)
- evs_std: bridge=774.7% of algo (FFI overhead may distort results)

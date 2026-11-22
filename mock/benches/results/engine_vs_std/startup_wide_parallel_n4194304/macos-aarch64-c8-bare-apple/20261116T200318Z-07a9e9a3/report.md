# wide_parallel, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std_startup)

The baseline evs_std_startup is the fastest (1.93 ms median); no rival improves on it (all deltas are >= 0).


### Speed leader evs_std_startup vs stability leader evs_engine_startup (+7% speed for 1.1x steadier)

evs_std_startup is fastest (1.93 ms, CV 3.7%); evs_engine_startup gives up 7.1% median for 1.1x lower variance (CV 3.4%).


## Key findings

- Baseline (evs_std_startup) is the fastest at 1933025.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.07x (fastest 1933025.0 ns, slowest 2070262.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | wide_parallel, get-ready |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000010004 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine_startup | 2103520ns | 2072729ns | 2060367ns | 2064111ns | 2143865ns | +6.62% |
| evs_std_startup | 1972883ns | 1935458ns | 1903283ns | 1928308ns | 2023715ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 2100786ns | 2057783ns | 2141233ns | +6.62% | 1.997 |
| evs_std_startup | 1970287ns | 1899733ns | 2021333ns | base | 2.129 |

## Performance model

- Peak throughput: 2.208 Gops/s (evs_std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 2.026 | 91.8% |
| evs_std_startup | 2.170 | 98.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 2103520ns | 2103520ns | +6.62% |
| evs_std_startup | 1972883ns | 1972883ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 1933025ns | base | --- | [1918558, 2020158] | --- | --- | --- | --- |
| evs_engine_startup | 2070262ns | +130220.8ns (+6.7%) | [+44300, +217967]ns | [2057892, 2117700] | YES (adj: no) | 0.0703 | 0.0703 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 1929988ns | +7.9% |
| 1 | 2 | 1976267ns | +4.3% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine_startup | -0.500 | moderate- |
| evs_std_startup | -0.500 | moderate- |

Consistency summary:

- evs_engine_startup: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine_startup | 103924202.1ns | 2100786.5ns | 4946.9% | HIGH |
| evs_std_startup | 103932315.6ns | 1970287.5ns | 5275.0% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 2057783.3-2141233.4 ns)
  2057783.3 |########################################
  2061955.8 |#############
  2066128.3 |
  2070300.8 |
  2074473.3 |#############
  2078645.8 |
  2082818.3 |
  2086990.8 |#############
  2091163.3 |
  2095335.8 |
  2099508.3 |
  2103680.8 |
  2107853.3 |
  2112025.8 |
  2116198.3 |#############
  2120370.8 |
  2124543.3 |
  2128715.8 |
  2132888.3 |
  2137060.8 |
  (0 below, 1 above range)

evs_std_startup (n=8, range 1899733.3-2021333.3 ns)
  1899733.3 |####################
  1905813.3 |
  1911893.3 |
  1917973.3 |####################
  1924053.3 |####################
  1930133.3 |########################################
  1936213.3 |
  1942293.3 |
  1948373.3 |
  1954453.3 |
  1960533.3 |
  1966613.3 |
  1972693.3 |
  1978773.3 |
  1984853.3 |
  1990933.3 |
  1997013.3 |
  2003093.3 |####################
  2009173.3 |
  2015253.3 |####################
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine_startup: bridge=5015.0% of algo (FFI overhead may distort results)
- evs_std_startup: bridge=5366.7% of algo (FFI overhead may distort results)

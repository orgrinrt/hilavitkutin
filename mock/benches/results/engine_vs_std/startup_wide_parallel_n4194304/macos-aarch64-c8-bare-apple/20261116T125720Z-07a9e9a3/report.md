# wide_parallel, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std_startup)

The baseline evs_std_startup is the fastest (1.92 ms median); no rival improves on it (all deltas are >= 0).


### Speed leader evs_std_startup vs stability leader evs_engine_startup (+9% speed for 2.4x steadier)

evs_std_startup is fastest (1.92 ms, CV 4.8%); evs_engine_startup gives up 8.9% median for 2.4x lower variance (CV 2.0%).


## Key findings

- Baseline (evs_std_startup) is the fastest at 1916445.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.09x (fastest 1916445.9 ns, slowest 2087616.6 ns)

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
| evs_engine_startup | 2102954ns | 2090612ns | 2062783ns | 2078942ns | 2131006ns | +7.37% |
| evs_std_startup | 1958678ns | 1918617ns | 1909908ns | 1915772ns | 2003050ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 2100132ns | 2060142ns | 2128225ns | +7.35% | 1.997 |
| evs_std_startup | 1956286ns | 1907450ns | 2000431ns | base | 2.144 |

## Performance model

- Peak throughput: 2.199 Gops/s (evs_std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 2.009 | 91.4% |
| evs_std_startup | 2.189 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 2102954ns | 2102954ns | +7.37% |
| evs_std_startup | 1958678ns | 1958678ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 1916446ns | base | --- | [1912092, 1955175] | --- | --- | --- | --- |
| evs_engine_startup | 2087617ns | +171787.5ns (+9.0%) | [+127275, +182350]ns | [2060475, 2136333] | YES (adj: no) | 0.0703 | 0.0703 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 1914512ns | +9.2% |
| 1 | 2 | 1925350ns | +8.3% |

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
| evs_engine_startup | 103884524.0ns | 2100132.3ns | 4946.6% | HIGH |
| evs_std_startup | 103750382.3ns | 1956286.5ns | 5303.4% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 2060141.7-2128225.0 ns)
  2060141.7 |########################################
  2063545.9 |
  2066950.0 |
  2070354.2 |
  2073758.4 |
  2077162.5 |####################
  2080566.7 |
  2083970.8 |####################
  2087375.0 |####################
  2090779.2 |
  2094183.3 |####################
  2097587.5 |
  2100991.7 |
  2104395.8 |
  2107800.0 |
  2111204.2 |
  2114608.3 |
  2118012.5 |
  2121416.6 |
  2124820.8 |
  (0 below, 2 above range)

evs_std_startup (n=8, range 1907450.0-2000431.2 ns)
  1907450.0 |########################################
  1912099.1 |########################################
  1916748.1 |####################
  1921397.2 |
  1926046.2 |
  1930695.3 |####################
  1935344.4 |
  1939993.4 |
  1944642.5 |
  1949291.6 |
  1953940.6 |####################
  1958589.7 |
  1963238.8 |
  1967887.8 |
  1972536.9 |
  1977185.9 |
  1981835.0 |
  1986484.1 |
  1991133.1 |
  1995782.2 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine_startup: bridge=4975.5% of algo (FFI overhead may distort results)
- evs_std_startup: bridge=5405.7% of algo (FFI overhead may distort results)

# wide_parallel, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (evs_std_startup)

The baseline evs_std_startup is the fastest (2.02 ms median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std_startup) is the fastest at 2018829.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.10x (fastest 2018829.2 ns, slowest 2216141.7 ns)

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
| evs_engine_startup | 2409615ns | 2218212ns | 2080433ns | 2129431ns | 2702048ns | +19.34% |
| evs_std_startup | 2019108ns | 2022171ns | 1971917ns | 1996669ns | 2047735ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 2406068ns | 2077325ns | 2697869ns | +19.34% | 1.743 |
| evs_std_startup | 2016111ns | 1968567ns | 2044910ns | base | 2.080 |

## Performance model

- Peak throughput: 2.131 Gops/s (evs_std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 1.893 | 88.8% |
| evs_std_startup | 2.078 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 2409615ns | 2409615ns | +19.34% |
| evs_std_startup | 2019108ns | 2019108ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 2018829ns | base | --- | [1976992, 2051708] | --- | --- | --- | --- |
| evs_engine_startup | 2216142ns | +206775.0ns (+10.2%) | [+75367, +650867]ns | [2098442, 2712392] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 1994992ns | +5.2% |
| 1 | 2 | 2046725ns | +30.1% |

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
| evs_engine_startup | 108156029.2ns | 2406067.7ns | 4495.1% | HIGH |
| evs_std_startup | 103835295.8ns | 2016111.5ns | 5150.3% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 2077325.0-2697868.8 ns)
  2077325.0 |########################################
  2108352.2 |
  2139379.4 |
  2170406.6 |#############
  2201433.8 |
  2232460.9 |#############
  2263488.1 |
  2294515.3 |
  2325542.5 |
  2356569.7 |
  2387596.9 |
  2418624.1 |
  2449651.3 |
  2480678.5 |
  2511705.6 |
  2542732.8 |
  2573760.0 |
  2604787.2 |#############
  2635814.4 |
  2666841.6 |
  (0 below, 2 above range)

evs_std_startup (n=8, range 1968566.7-2044910.4 ns)
  1968566.7 |########################################
  1972383.9 |
  1976201.1 |########################################
  1980018.3 |
  1983835.4 |
  1987652.6 |########################################
  1991469.8 |
  1995287.0 |
  1999104.2 |
  2002921.4 |
  2006738.6 |
  2010555.7 |########################################
  2014372.9 |
  2018190.1 |
  2022007.3 |########################################
  2025824.5 |
  2029641.7 |
  2033458.9 |
  2037276.1 |
  2041093.2 |########################################
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine_startup: bridge=4729.8% of algo (FFI overhead may distort results)
- evs_std_startup: bridge=5141.3% of algo (FFI overhead may distort results)

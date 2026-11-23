# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std is fastest but the noisiest (CV 9.8%)

evs_std wins on median (2.12 ms) yet has the highest variance (CV 9.8%), while evs_engine is the steadiest (CV 4.5%, 2.30 ms).


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (181.84 us) is smaller than the fastest variant's own run-to-run std-dev (207.80 us); the ranking is inside the noise.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (2.12 ms median); no rival improves on it (all deltas are >= 0).


### Speed leader evs_std vs stability leader evs_engine (+9% speed for 2.2x steadier)

evs_std is fastest (2.12 ms, CV 9.8%); evs_engine gives up 8.6% median for 2.2x lower variance (CV 4.5%).


## Key findings

- Baseline (evs_std) is the fastest at 2116740.4 ns median
- 1 variant significantly slower than baseline
- Spread: 1.09x (fastest 2116740.4 ns, slowest 2298579.0 ns)

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
| evs_engine | 2331641ns | 2299450ns | 2209418ns | 2274914ns | 2404741ns | +12.28% |
| evs_std | 2076593ns | 2117816ns | 1832816ns | 1939647ns | 2240246ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 2330648ns | 2208229ns | 2403675ns | +12.30% | 1.800 |
| evs_std | 2075358ns | 1831859ns | 2238774ns | base | 2.021 |

## Performance model

- Peak throughput: 2.290 Gops/s (evs_std; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.825 | 79.7% |
| evs_std | 1.981 | 86.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 2331641ns | 2331641ns | +12.28% |
| evs_std | 2076593ns | 2076593ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 2116740ns | base | --- | [1844477, 2191614] | --- | --- | --- | --- |
| evs_engine | 2298579ns | +248139.9ns (+11.7%) | [+108772, +400441]ns | [2258188, 2459529] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 2155704ns | +7.1% |
| 1 | 2 | 1851112ns | +22.2% |

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
| evs_engine | 16781501.6ns | 2330647.5ns | 720.0% | HIGH |
| evs_std | 16773366.6ns | 2075358.2ns | 808.2% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 2208228.8-2403675.4 ns)
  2208228.8 |####################
  2218001.1 |
  2227773.5 |
  2237545.8 |
  2247318.1 |
  2257090.5 |####################
  2266862.8 |####################
  2276635.1 |
  2286407.4 |
  2296179.8 |########################################
  2305952.1 |
  2315724.4 |####################
  2325496.8 |
  2335269.1 |
  2345041.4 |
  2354813.8 |
  2364586.1 |
  2374358.4 |
  2384130.8 |
  2393903.1 |
  (0 below, 2 above range)

evs_std (n=8, range 1831858.6-2238774.0 ns)
  1831858.6 |########################################
  1852204.4 |####################
  1872550.1 |
  1892895.9 |
  1913241.7 |
  1933587.5 |
  1953933.2 |
  1974279.0 |
  1994624.8 |
  2014970.5 |
  2035316.3 |
  2055662.1 |
  2076007.9 |
  2096353.6 |####################
  2116699.4 |####################
  2137045.2 |
  2157390.9 |####################
  2177736.7 |####################
  2198082.5 |
  2218428.3 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=730.2% of algo (FFI overhead may distort results)
- evs_std: bridge=794.3% of algo (FFI overhead may distort results)

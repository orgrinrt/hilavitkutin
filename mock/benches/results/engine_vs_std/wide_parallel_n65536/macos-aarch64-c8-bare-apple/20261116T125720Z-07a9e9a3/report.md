# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_par beats baseline by 43% (significant)

evs_std_par is -79.19 us (43%) faster than baseline evs_std, with a CI that excludes zero.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_std_par, evs_engine_par} vs {evs_std, evs_engine} (63% apart)

The field splits into a fast tier {evs_std_par, evs_engine_par} and a slow tier {evs_std, evs_engine} with a 63% jump between them - a qualitative difference, not a gradient.


## Key findings

- Fastest: evs_std_par at 104016.6 ns median (-43.2% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.87x (fastest 104016.6 ns, slowest 194875.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | wide_parallel, four independent heavy chains |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000004 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 196177ns | 196150ns | 194158ns | 195264ns | 197367ns | +6.21% |
| evs_engine_par | 115211ns | 114421ns | 111708ns | 113147ns | 117635ns | -37.62% |
| evs_std | 184702ns | 184846ns | 183967ns | 184578ns | 184979ns | base |
| evs_std_par | 105830ns | 105542ns | 102783ns | 104242ns | 107783ns | -42.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 195245ns | 194033ns | 196023ns | +6.51% | 0.336 |
| evs_engine_par | 113094ns | 109600ns | 115648ns | -38.31% | 0.579 |
| evs_std | 183314ns | 183117ns | 183465ns | base | 0.358 |
| evs_std_par | 104245ns | 101342ns | 106188ns | -43.13% | 0.629 |

## Performance model

- Peak throughput: 0.647 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.336 | 52.0% |
| evs_engine_par | 0.583 | 90.2% |
| evs_std | 0.358 | 55.3% |
| evs_std_par | 0.630 | 97.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 196177ns | 196177ns | +6.21% |
| evs_engine_par | 115211ns | 115211ns | -37.62% |
| evs_std | 184702ns | 184702ns | base |
| evs_std_par | 105830ns | 105830ns | -42.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 183208ns | base | --- | [183150, 183308] | --- | --- | --- | --- |
| evs_engine | 194875ns | +11683.3ns (+6.4%) | [+10808, +12417]ns | [194517, 195608] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 112325ns | -70883.4ns (-38.7%) | [-73092, -65758]ns | [110217, 117542] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 104017ns | -79191.7ns (-43.2%) | [-81158, -77058]ns | [102150, 106167] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 183171ns | +6.4% | -38.5% | -43.3% |
| 1 | 2 | 183262ns | +6.3% | -38.7% | -43.2% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 0/2, lost 2/2
- evs_engine_par: won 2/2, lost 0/2
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 1418537.5ns | 195244.8ns | 726.5% | HIGH |
| evs_engine_par | 1426401.0ns | 113093.8ns | 1261.3% | HIGH |
| evs_std | 1413468.8ns | 183313.6ns | 771.1% | HIGH |
| evs_std_par | 1420230.2ns | 104244.8ns | 1362.4% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 194033.3-196022.9 ns)
  194033.3 |########################################
  194132.8 |
  194232.3 |
  194331.7 |
  194431.2 |########################################
  194530.7 |########################################
  194630.2 |########################################
  194729.7 |
  194829.1 |
  194928.6 |########################################
  195028.1 |########################################
  195127.6 |
  195227.1 |
  195326.5 |
  195426.0 |
  195525.5 |########################################
  195625.0 |
  195724.5 |
  195823.9 |
  195923.4 |
  (0 below, 1 above range)

evs_engine_par (n=8, range 109600.0-115647.9 ns)
  109600.0 |####################
  109902.4 |
  110204.8 |########################################
  110507.2 |
  110809.6 |
  111112.0 |
  111414.4 |
  111716.8 |####################
  112019.2 |
  112321.6 |
  112624.0 |####################
  112926.4 |
  113228.8 |
  113531.2 |
  113833.6 |
  114136.0 |
  114438.4 |
  114740.8 |####################
  115043.2 |
  115345.6 |
  (0 below, 2 above range)

evs_std (n=8, range 183116.7-183464.6 ns)
  183116.7 |####################
  183134.1 |####################
  183151.5 |
  183168.9 |
  183186.3 |########################################
  183203.7 |
  183221.1 |####################
  183238.5 |
  183255.9 |
  183273.2 |
  183290.6 |####################
  183308.0 |####################
  183325.4 |
  183342.8 |
  183360.2 |
  183377.6 |
  183395.0 |
  183412.4 |
  183429.8 |
  183447.2 |
  (0 below, 1 above range)

evs_std_par (n=8, range 101341.7-106187.5 ns)
  101341.7 |####################
  101584.0 |
  101826.3 |
  102068.6 |####################
  102310.9 |
  102553.1 |####################
  102795.4 |####################
  103037.7 |
  103280.0 |
  103522.3 |
  103764.6 |
  104006.9 |
  104249.2 |
  104491.5 |
  104733.8 |
  104976.1 |########################################
  105218.3 |
  105460.6 |
  105702.9 |
  105945.2 |####################
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=725.5% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=1269.6% of algo (FFI overhead may distort results)
- evs_std: bridge=771.0% of algo (FFI overhead may distort results)
- evs_std_par: bridge=1364.6% of algo (FFI overhead may distort results)

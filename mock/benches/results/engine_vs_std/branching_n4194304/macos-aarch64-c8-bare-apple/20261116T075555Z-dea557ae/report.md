# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 235% faster than the next best (evs_engine)

evs_std (599.02 us) leads evs_engine (2.01 ms) by 235%, a clear separation rather than a photo finish. CV 2.6%.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (599.02 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 3.4x the fastest

Fastest evs_std (599.02 us) to slowest evs_engine (2.01 ms): 3.4x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std) is the fastest at 599020.8 ns median
- 1 variant significantly slower than baseline
- Spread: 3.35x (fastest 599020.8 ns, slowest 2006908.3 ns)

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
| evs_engine | 2040842ns | 2008525ns | 1972583ns | 1987483ns | 2097925ns | +237.79% |
| evs_std | 604167ns | 600462ns | 587508ns | 592489ns | 617090ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 2038674ns | 1970283ns | 2096087ns | +238.57% | 2.057 |
| evs_std | 602139ns | 585400ns | 614942ns | base | 6.966 |

## Performance model

- Peak throughput: 7.165 Gops/s (evs_std; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 2.090 | 29.2% |
| evs_std | 7.002 | 97.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 2040842ns | 2040842ns | +237.79% |
| evs_std | 604167ns | 604167ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 599021ns | base | --- | [586317, 621758] | --- | --- | --- | --- |
| evs_engine | 2006908ns | +1395387.5ns (+232.9%) | [+1371700, +1516325]ns | [1973017, 2102958] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 592812ns | +238.8% |
| 1 | 2 | 616692ns | +225.4% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_std | -0.500 | HIGH- (thermal bounce) |

Consistency summary:

- evs_engine: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 22854417.7ns | 2038673.9ns | 1121.0% | HIGH |
| evs_std | 22812243.8ns | 602138.5ns | 3788.5% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 1970283.3-2096087.5 ns)
  1970283.3 |########################################
  1976573.5 |####################
  1982863.7 |
  1989153.9 |
  1995444.1 |####################
  2001734.3 |
  2008024.6 |
  2014314.8 |####################
  2020605.0 |
  2026895.2 |
  2033185.4 |####################
  2039475.6 |
  2045765.8 |
  2052056.0 |
  2058346.2 |
  2064636.4 |
  2070926.6 |
  2077216.8 |
  2083507.1 |
  2089797.3 |
  (0 below, 2 above range)

evs_std (n=8, range 585400.0-614941.7 ns)
  585400.0 |########################################
  586877.1 |
  588354.2 |
  589831.2 |
  591308.3 |
  592785.4 |
  594262.5 |
  595739.6 |
  597216.7 |
  598693.7 |##########################
  600170.8 |
  601647.9 |
  603125.0 |
  604602.1 |
  606079.2 |
  607556.2 |
  609033.3 |
  610510.4 |#############
  611987.5 |
  613464.6 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=1137.3% of algo (FFI overhead may distort results)
- evs_std: bridge=3807.9% of algo (FFI overhead may distort results)

# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 239% faster than the next best (evs_engine)

evs_std (581.93 us) leads evs_engine (1.97 ms) by 239%, a clear separation rather than a photo finish. CV 1.0%.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (581.93 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 3.4x the fastest

Fastest evs_std (581.93 us) to slowest evs_engine (1.97 ms): 3.4x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std) is the fastest at 581929.2 ns median
- 1 variant significantly slower than baseline
- Spread: 3.39x (fastest 581929.2 ns, slowest 1973000.0 ns)

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
| evs_engine | 1980706ns | 1974817ns | 1952608ns | 1963289ns | 2000794ns | +237.67% |
| evs_std | 586578ns | 583796ns | 580433ns | 582481ns | 591188ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 1978599ns | 1950808ns | 1998685ns | +238.47% | 2.120 |
| evs_std | 584568ns | 578008ns | 589196ns | base | 7.175 |

## Performance model

- Peak throughput: 7.256 Gops/s (evs_std; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 2.126 | 29.3% |
| evs_std | 7.208 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 1980706ns | 1980706ns | +237.67% |
| evs_std | 586578ns | 586578ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 581929ns | base | --- | [580017, 590108] | --- | --- | --- | --- |
| evs_engine | 1973000ns | +1386600.0ns (+238.3%) | [+1376133, +1421075]ns | [1954142, 2009492] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 581929ns | +239.0% |
| 1 | 2 | 584512ns | +236.2% |

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
| evs_engine | 22789342.7ns | 1978599.0ns | 1151.8% | HIGH |
| evs_std | 22748957.3ns | 584567.7ns | 3891.6% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 1950808.3-1998685.4 ns)
  1950808.3 |####################
  1953202.2 |####################
  1955596.0 |
  1957989.9 |####################
  1960383.7 |
  1962777.6 |
  1965171.4 |
  1967565.3 |
  1969959.2 |####################
  1972353.0 |
  1974746.9 |########################################
  1977140.7 |
  1979534.6 |
  1981928.4 |
  1984322.3 |
  1986716.1 |
  1989110.0 |
  1991503.9 |
  1993897.7 |
  1996291.6 |
  (0 below, 2 above range)

evs_std (n=8, range 578008.3-589195.8 ns)
  578008.3 |########################################
  578567.7 |
  579127.1 |
  579686.4 |########################################
  580245.8 |########################################
  580805.2 |########################################
  581364.6 |
  581923.9 |
  582483.3 |########################################
  583042.7 |
  583602.1 |
  584161.4 |
  584720.8 |
  585280.2 |
  585839.6 |
  586398.9 |
  586958.3 |
  587517.7 |
  588077.1 |########################################
  588636.4 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=1154.9% of algo (FFI overhead may distort results)
- evs_std: bridge=3908.9% of algo (FFI overhead may distort results)

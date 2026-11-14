# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 20% faster than the next best (evs_engine)

evs_std (26.58 us) leads evs_engine (31.94 us) by 20%, a clear separation rather than a photo finish. CV 19.6%.


### evs_std is fastest but the noisiest (CV 19.6%)

evs_std wins on median (26.58 us) yet has the highest variance (CV 19.6%), while evs_engine is the steadiest (CV 14.9%, 31.94 us).


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (26.58 us median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 26583.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.20x (fastest 26583.8 ns, slowest 31937.6 ns)

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
| Counter frequency | 2100267710 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 34878ns | 33130ns | 29712ns | 31162ns | 38956ns | +14.30% |
| evs_std | 30514ns | 27031ns | 26050ns | 26126ns | 34921ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 34011ns | 28804ns | 38269ns | +14.04% | 1.927 |
| evs_std | 29825ns | 25822ns | 33816ns | base | 2.197 |

## Performance model

- Peak throughput: 2.538 Gops/s (evs_std; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 2.052 | 80.9% |
| evs_std | 2.465 | 97.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 34878ns | 34878ns | +14.30% |
| evs_std | 30514ns | 30514ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 26584ns | base | --- | [25838, 36722] | --- | --- | --- | --- |
| evs_engine | 31938ns | +2819.5ns (+10.6%) | [+350, +7483]ns | [29116, 39985] | YES (adj: no) | 0.0703 | 0.0703 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 25838ns | +23.6% |
| 1 | 2 | 34151ns | +2.4% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | HIGH- (thermal bounce) |
| evs_std | -0.500 | HIGH- (thermal bounce) |

Consistency summary:

- evs_engine: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 337954.1ns | 34011.4ns | 993.6% | HIGH |
| evs_std | 359693.5ns | 29825.4ns | 1206.0% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 28804.3-38268.8 ns)
  28804.3 |########################################
  29277.5 |
  29750.8 |
  30224.0 |########################################
  30697.2 |
  31170.4 |
  31643.7 |
  32116.9 |
  32590.1 |
  33063.3 |####################
  33536.6 |
  34009.8 |
  34483.0 |
  34956.2 |
  35429.5 |
  35902.7 |
  36375.9 |
  36849.1 |
  37322.4 |
  37795.6 |
  (0 below, 3 above range)

evs_std (n=8, range 25821.8-33815.8 ns)
  25821.8 |########################################
  26221.5 |
  26621.2 |
  27020.9 |##########
  27420.6 |
  27820.3 |
  28220.0 |
  28619.7 |
  29019.4 |
  29419.1 |
  29818.8 |
  30218.5 |
  30618.2 |
  31017.9 |
  31417.6 |##########
  31817.3 |
  32217.0 |
  32616.7 |
  33016.4 |
  33416.1 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=1028.0% of algo (FFI overhead may distort results)
- evs_std: bridge=1304.6% of algo (FFI overhead may distort results)

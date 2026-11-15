# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (evs_std) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline evs_std has the worst median (1.65 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest evs_engine at 1.61 us).


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (49 ns) is smaller than the fastest variant's own run-to-run std-dev (584 ns); the ranking is inside the noise.


### evs_std is inconsistent: worst-20% is 1.7x its best-20%

evs_std's best 20% of batches run at 1.64 us but its worst 20% at 2.73 us (1.7x) - a bimodal or bursty profile the median hides.


## Key findings

- Fastest: evs_engine at 1605.9 ns median (-2.9% vs baseline)
- Spread: 1.03x (fastest 1605.9 ns, slowest 1654.8 ns)

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
| evs_engine | 2002ns | 1772ns | 1543ns | 1659ns | 2374ns | -15.80% |
| evs_std | 2378ns | 1880ns | 1784ns | 1838ns | 2930ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 1817ns | 1382ns | 2187ns | -16.86% | 2.254 |
| evs_std | 2185ns | 1637ns | 2730ns | base | 1.874 |

## Performance model

- Peak throughput: 2.964 Gops/s (evs_engine; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 2.551 | 86.1% |
| evs_std | 2.475 | 83.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 2002ns | 2002ns | -15.80% |
| evs_std | 2378ns | 2378ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 1655ns | base | --- | [1639, 1938] | --- | --- | --- | --- |
| evs_engine | 1606ns | no significant difference | [-263, +466]ns | [1405, 2176] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 1687ns | +14.2% |
| 1 | 2 | 1644ns | -13.0% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 1/2, lost 1/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 20225.0ns | 1817.0ns | 1113.1% | HIGH |
| evs_std | 20848.0ns | 2185.4ns | 954.0% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 1382.0-2187.3 ns)
   1382.0 |########################################
   1422.3 |####################
   1462.5 |
   1502.8 |
   1543.1 |####################
   1583.3 |
   1623.6 |
   1663.9 |########################################
   1704.1 |
   1744.4 |
   1784.7 |
   1824.9 |
   1865.2 |
   1905.5 |
   1945.7 |
   1986.0 |
   2026.3 |
   2066.5 |
   2106.8 |
   2147.1 |####################
  (0 below, 1 above range)

evs_std (n=8, range 1637.4-2730.0 ns)
   1637.4 |########################################
   1692.0 |########
   1746.7 |
   1801.3 |
   1855.9 |
   1910.5 |########
   1965.2 |
   2019.8 |
   2074.4 |
   2129.1 |
   2183.7 |
   2238.3 |
   2292.9 |
   2347.6 |
   2402.2 |
   2456.8 |
   2511.5 |
   2566.1 |
   2620.7 |
   2675.3 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: CV=32.1% (high variance, measurements may be unstable)
- evs_engine: bridge=1224.0% of algo (FFI overhead may distort results)
- evs_std: CV=59.3% (high variance, measurements may be unstable)
- evs_std: bridge=1250.3% of algo (FFI overhead may distort results)

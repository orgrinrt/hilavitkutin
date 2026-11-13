# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: **engine**

## Highlights

Baseline for all deltas below: **engine**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (engine) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline engine has the worst median (1.51 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest std at 1.39 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### std shows alternating (throttle bounce) (autocorr -0.50)

std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: std** at 1388.6 ns median (-7.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.08x (fastest 1388.6 ns, slowest 1506.2 ns)

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
| Counter frequency | 2300408230 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 1721ns | 1656ns | 1595ns | 1612ns | 1835ns | base |
| std | 1563ns | 1526ns | 1499ns | 1506ns | 1622ns | -9.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 1562ns | 1473ns | 1641ns | base | 2.622 |
| std | 1394ns | 1386ns | 1400ns | -10.77% | 2.939 |

## Performance model

- Peak throughput: **2.955 Gops/s** (std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 2.719 | 92.0% |
| std | 2.950 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 1721ns | 1721ns | base |
| std | 1563ns | 1563ns | -9.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine | 1506ns | base | --- | [1478, 1533] | --- | --- | --- | --- |
| std | 1389ns | -102.6ns (-6.8%) | [-140, -89]ns | [1387, 1394] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine | std |
|---|---|---|---|
| 1 | 1 | 1506ns | -7.8% |
| 1 | 2 | 1506ns | -7.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| std | -0.500 | moderate- |

**Consistency summary:**

- **std**: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 18496.4ns | 1562.1ns | 1184.1% | HIGH |
| std | 18713.8ns | 1393.8ns | 1342.6% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 1473.3-1641.5 ns)
   1473.3 |########################################
   1481.7 |
   1490.1 |
   1498.5 |#############
   1506.9 |#############
   1515.3 |#############
   1523.8 |
   1532.2 |#############
   1540.6 |
   1549.0 |
   1557.4 |
   1565.8 |
   1574.2 |
   1582.6 |
   1591.0 |
   1599.4 |
   1607.8 |
   1616.2 |
   1624.7 |
   1633.1 |
  (0 below, 1 above range)

std (n=8, range 1386.1-1400.3 ns)
   1386.1 |####################
   1386.8 |####################
   1387.5 |########################################
   1388.2 |
   1388.9 |####################
   1389.7 |
   1390.4 |####################
   1391.1 |
   1391.8 |
   1392.5 |
   1393.2 |####################
   1393.9 |
   1394.6 |
   1395.3 |
   1396.1 |
   1396.8 |
   1397.5 |
   1398.2 |
   1398.9 |
   1399.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **engine**: bridge=1138.8% of algo (FFI overhead may distort results)
- **std**: bridge=1235.5% of algo (FFI overhead may distort results)

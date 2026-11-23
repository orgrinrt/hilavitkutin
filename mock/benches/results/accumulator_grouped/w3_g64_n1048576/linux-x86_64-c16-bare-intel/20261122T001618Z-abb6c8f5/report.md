# W = 3, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential is an outlier: 2.3x slower than the field

sequential (3.23 ms) is 2.3x the fastest (1.44 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.67)

sequential's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: per_core** at 1435536.0 ns median (-7.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.25x (fastest 1435536.0 ns, slowest 3232997.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 3, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112788431 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 1626488ns | 1553264ns | 1541282ns | 1548793ns | 1785633ns | base |
| per_core | 1444787ns | 1438184ns | 1409426ns | 1435416ns | 1476524ns | -11.17% |
| sequential | 3318676ns | 3234422ns | 3209322ns | 3231713ns | 3503798ns | +104.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 1625565ns | 1540491ns | 1784494ns | base | 0.645 |
| per_core | 1442065ns | 1407443ns | 1473596ns | -11.29% | 0.727 |
| sequential | 3316758ns | 3208480ns | 3500476ns | +104.04% | 0.316 |

## Performance model

- Peak throughput: **0.745 Gops/s** (per_core; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.675 | 90.7% |
| per_core | 0.730 | 98.0% |
| sequential | 0.324 | 43.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 1626488ns | 1626488ns | base |
| per_core | 1444787ns | 1444787ns | -11.17% |
| sequential | 3318676ns | 3318676ns | +104.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 1552326ns | base | --- | [1542834, 1563412] | --- | --- | --- | --- |
| per_core | 1435536ns | -114213.1ns (-7.4%) | [-134121, -102913]ns | [1427400, 1446635] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 3232997ns | +1675731.0ns (+107.9%) | [+1662546, +1695449]ns | [3225456, 3250084] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 1544475ns | -6.3% | +109.2% |
| 1 | 2 | 1549265ns | -7.7% | +110.2% |
| 1 | 3 | 1554185ns | -7.7% | +107.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.000 | ok |
| per_core | -0.371 | moderate- |
| sequential | -0.667 | HIGH- (thermal bounce) |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 1776466.4ns | 1625565.0ns | 109.3% | HIGH |
| per_core | 1439060.2ns | 1442064.9ns | 99.8% | HIGH |
| sequential | 3240925.4ns | 3316758.4ns | 97.7% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 1540490.8-1784494.4 ns)
  1540490.8 |########################################
  1552691.0 |#########################
  1564891.2 |#####
  1577091.4 |#####
  1589291.5 |
  1601491.7 |
  1613691.9 |
  1625892.1 |
  1638092.3 |
  1650292.4 |
  1662492.6 |
  1674692.8 |
  1686893.0 |
  1699093.2 |
  1711293.3 |
  1723493.5 |#####
  1735693.7 |
  1747893.9 |
  1760094.0 |
  1772294.2 |
  (1 below, 1 above range)

per_core (n=18, range 1407443.4-1473595.8 ns)
  1407443.4 |
  1410751.0 |
  1414058.6 |
  1417366.3 |
  1420673.9 |########################################
  1423981.5 |#############
  1427289.1 |##########################
  1430596.7 |##########################
  1433904.3 |
  1437212.0 |##########################
  1440519.6 |#############
  1443827.2 |##########################
  1447134.8 |#############
  1450442.4 |
  1453750.1 |
  1457057.7 |
  1460365.3 |##########################
  1463672.9 |
  1466980.5 |
  1470288.1 |
  (1 below, 1 above range)

sequential (n=18, range 3208480.1-3500476.2 ns)
  3208480.1 |##########################
  3223079.9 |########################################
  3237679.7 |#############
  3252279.5 |######
  3266879.3 |#############
  3281479.1 |######
  3296078.9 |
  3310678.7 |
  3325278.5 |
  3339878.3 |
  3354478.1 |
  3369078.0 |
  3383677.8 |
  3398277.6 |
  3412877.4 |
  3427477.2 |
  3442077.0 |
  3456676.8 |
  3471276.6 |
  3485876.4 |
  (1 below, 1 above range)

```

## Diagnostics

- **grouped**: bridge=109.1% of algo (FFI overhead may distort results)
- **per_core**: bridge=100.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.2% of algo (FFI overhead may distort results)

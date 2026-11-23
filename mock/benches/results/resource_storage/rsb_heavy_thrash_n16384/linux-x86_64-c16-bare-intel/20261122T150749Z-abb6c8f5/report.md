# Resource storage layouts under L1-evicting heavy-memory mix-in

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### v0_blob dominates: 21% faster than the next best (v5_handletable)

v0_blob (1.35 us) leads v5_handletable (1.63 us) by 21%, a clear separation rather than a photo finish. CV 11.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### v4_erased shows alternating (throttle bounce) (autocorr -0.54)

v4_erased's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (v0_blob)

The baseline v0_blob is the fastest (1.35 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### v1_snapshot's edge over baseline is significant but tiny (4 ns, 0.27%)

v1_snapshot differs from baseline v0_blob by 4 ns (0.27%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (v0_blob) is the fastest** at 1347.0 ns median
- 4 variants significantly slower than baseline
- Spread: 1.22x (fastest 1347.0 ns, slowest 1641.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Resource storage layouts under L1-evicting heavy-memory mix-in |
| Passes | 5 |
| Runs per pass | 2000 |
| Batch size | 250 |
| Harness runs | 3 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112746324 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| v0_blob | 5292ns | 4819ns | 4782ns | 5265ns | 5880ns | base |
| v1_snapshot | 5361ns | 5844ns | 4784ns | 5383ns | 5874ns | +1.32% |
| v2_decomposed | 5365ns | 5849ns | 4790ns | 5389ns | 5872ns | +1.40% |
| v3_shapebound | 5499ns | 5854ns | 4793ns | 5409ns | 6476ns | +3.93% |
| v4_erased | 5364ns | 5848ns | 4785ns | 5386ns | 5877ns | +1.36% |
| v5_handletable | 5358ns | 5844ns | 4783ns | 5381ns | 5865ns | +1.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 1480ns | 1338ns | 1646ns | base | 11.070 |
| v1_snapshot | 1502ns | 1341ns | 1646ns | +1.51% | 10.905 |
| v2_decomposed | 1503ns | 1340ns | 1646ns | +1.57% | 10.899 |
| v3_shapebound | 1543ns | 1345ns | 1816ns | +4.25% | 10.618 |
| v4_erased | 1505ns | 1343ns | 1649ns | +1.69% | 10.886 |
| v5_handletable | 1499ns | 1338ns | 1642ns | +1.31% | 10.927 |

## Performance model

- Peak throughput: **12.249 Gops/s** (v0_blob; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 12.163 | 99.3% |
| v1_snapshot | 10.002 | 81.7% |
| v2_decomposed | 10.009 | 81.7% |
| v3_shapebound | 9.980 | 81.5% |
| v4_erased | 9.991 | 81.6% |
| v5_handletable | 10.022 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 5292ns | 5292ns | base |
| v1_snapshot | 5361ns | 5361ns | +1.32% |
| v2_decomposed | 5365ns | 5365ns | +1.40% |
| v3_shapebound | 5499ns | 5499ns | +3.93% |
| v4_erased | 5364ns | 5364ns | +1.36% |
| v5_handletable | 5358ns | 5358ns | +1.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 1347ns | base | --- | [1341, 1634] | --- | --- | --- | --- |
| v1_snapshot | 1638ns | +3.7ns (+0.3%) | [+3, +5]ns | [1347, 1639] | YES | 0.0000 | 0.0000 | 0 |
| v2_decomposed | 1637ns | +3.1ns (+0.2%) | [+1, +5]ns | [1349, 1639] | YES | 0.0040 | 0.0032 | 1 |
| v3_shapebound | 1642ns | +9.8ns (+0.7%) | [+6, +14]ns | [1351, 1645] | YES | 0.0001 | 0.0000 | 0 |
| v4_erased | 1640ns | +7.2ns (+0.5%) | [+5, +9]ns | [1349, 1642] | YES | 0.0000 | 0.0000 | 0 |
| v5_handletable | 1635ns | no significant difference | [-2, +5]ns | [1344, 1636] | no | 0.4635 | 0.4635 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1638ns | -18.0% | -18.0% | -17.7% | +0.4% | +0.3% |
| 1 | 2 | 1338ns | +0.5% | +0.4% | +0.9% | +0.6% | -0.0% |
| 1 | 3 | 1345ns | -0.2% | -0.1% | +0.1% | +22.1% | -0.4% |
| 1 | 4 | 1637ns | +0.5% | -17.8% | +0.6% | -17.8% | -18.1% |
| 1 | 5 | 1339ns | +22.6% | +22.9% | +22.9% | +22.8% | +22.2% |
| 2 | 1 | 1637ns | +0.2% | +0.3% | +0.5% | +0.3% | -18.1% |
| 2 | 2 | 1338ns | +0.3% | +0.3% | +23.0% | +0.4% | +22.4% |
| 2 | 3 | 1338ns | +0.2% | +0.1% | +23.3% | +0.5% | +22.3% |
| 2 | 4 | 1338ns | +0.3% | +22.6% | +23.1% | +22.8% | +22.2% |
| 2 | 5 | 1636ns | +0.2% | -17.8% | -17.6% | -17.9% | -18.1% |
| 3 | 1 | 1339ns | +22.5% | +22.8% | +23.0% | +23.2% | +22.4% |
| 3 | 2 | 1635ns | +0.4% | +0.3% | +0.8% | +0.6% | -18.1% |
| 3 | 3 | 1338ns | +0.2% | +22.7% | +0.6% | +0.5% | +0.1% |
| 3 | 4 | 1636ns | +0.3% | +0.2% | -17.6% | +0.6% | +0.0% |
| 3 | 5 | 1641ns | -0.0% | -0.1% | -17.9% | -18.1% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.411 | moderate- |
| v1_snapshot | 0.261 | moderate+ |
| v2_decomposed | 0.261 | moderate+ |
| v3_shapebound | 0.385 | moderate+ |
| v4_erased | -0.538 | HIGH- (thermal bounce) |
| v5_handletable | -0.132 | ok |

**Consistency summary:**

- **v1_snapshot**: won 2/15, lost 12/15
- **v2_decomposed**: won 4/15, lost 10/15
- **v3_shapebound**: won 4/15, lost 10/15
- **v4_erased**: won 3/15, lost 12/15
- **v5_handletable**: won 6/15, lost 6/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 35100.2ns | 1480.0ns | 2371.6% | HIGH |
| v1_snapshot | 35582.6ns | 1502.4ns | 2368.4% | HIGH |
| v2_decomposed | 35596.1ns | 1503.3ns | 2367.9% | HIGH |
| v3_shapebound | 36404.7ns | 1543.0ns | 2359.3% | HIGH |
| v4_erased | 35559.3ns | 1505.1ns | 2362.6% | HIGH |
| v5_handletable | 35596.4ns | 1499.4ns | 2374.1% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 1337.6-1646.1 ns)
   1337.6 |########################################
   1353.0 |
   1368.5 |
   1383.9 |
   1399.3 |
   1414.7 |
   1430.2 |
   1445.6 |
   1461.0 |
   1476.4 |
   1491.8 |
   1507.3 |
   1522.7 |
   1538.1 |
   1553.5 |
   1569.0 |
   1584.4 |
   1599.8 |
   1615.2 |
   1630.7 |#######################################
  (12 below, 6 above range)

v1_snapshot (n=120, range 1340.7-1645.9 ns)
   1340.7 |#################################
   1355.9 |
   1371.2 |
   1386.5 |
   1401.7 |
   1417.0 |
   1432.3 |
   1447.5 |
   1462.8 |
   1478.0 |
   1493.3 |
   1508.6 |
   1523.8 |
   1539.1 |
   1554.4 |
   1569.6 |
   1584.9 |
   1600.2 |
   1615.4 |
   1630.7 |########################################
  (9 below, 8 above range)

v2_decomposed (n=120, range 1340.5-1646.3 ns)
   1340.5 |##############################
   1355.7 |
   1371.0 |
   1386.3 |
   1401.6 |
   1416.9 |
   1432.2 |
   1447.5 |
   1462.8 |
   1478.1 |
   1493.4 |
   1508.7 |
   1524.0 |
   1539.2 |
   1554.5 |
   1569.8 |
   1585.1 |
   1600.4 |
   1615.7 |
   1631.0 |########################################
  (14 below, 10 above range)

v3_shapebound (n=120, range 1345.1-1816.4 ns)
   1345.1 |##################################
   1368.7 |
   1392.3 |
   1415.8 |
   1439.4 |
   1462.9 |
   1486.5 |
   1510.1 |
   1533.6 |
   1557.2 |
   1580.8 |
   1604.3 |
   1627.9 |########################################
   1651.4 |#######
   1675.0 |
   1698.6 |
   1722.1 |
   1745.7 |
   1769.2 |
   1792.8 |
  (10 below, 3 above range)

v4_erased (n=120, range 1342.7-1649.4 ns)
   1342.7 |##################################
   1358.0 |
   1373.4 |
   1388.7 |
   1404.0 |
   1419.4 |
   1434.7 |
   1450.0 |
   1465.4 |
   1480.7 |
   1496.0 |
   1511.4 |
   1526.7 |
   1542.0 |
   1557.4 |
   1572.7 |
   1588.0 |
   1603.4 |
   1618.7 |
   1634.0 |########################################
  (10 below, 10 above range)

v5_handletable (n=120, range 1338.3-1642.3 ns)
   1338.3 |################################
   1353.5 |
   1368.7 |
   1383.9 |
   1399.1 |
   1414.3 |
   1429.5 |
   1444.7 |
   1459.9 |
   1475.1 |
   1490.3 |
   1505.5 |
   1520.7 |
   1535.9 |
   1551.1 |
   1566.3 |
   1581.5 |
   1596.7 |
   1611.9 |
   1627.1 |########################################
  (12 below, 9 above range)

```

## Diagnostics

- **v0_blob**: bridge=2359.7% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=2371.0% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=2372.6% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=2366.0% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=2368.3% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=2377.8% of algo (FFI overhead may distort results)

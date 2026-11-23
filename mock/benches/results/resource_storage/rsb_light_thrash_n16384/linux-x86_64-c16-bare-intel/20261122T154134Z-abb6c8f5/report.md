# Resource storage layouts under light surrounding work

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6 ns) is smaller than the fastest variant's own run-to-run std-dev (146 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (v0_blob)

The baseline v0_blob is the fastest (1.63 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.4% of the fastest

All 6 variants sit between 1.63 us and 1.64 us - a 0.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### v1_snapshot's edge over baseline is significant but tiny (4 ns, 0.22%)

v1_snapshot differs from baseline v0_blob by 4 ns (0.22%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (v0_blob) is the fastest** at 1634.6 ns median
- 4 variants significantly slower than baseline
- Spread: 1.00x (fastest 1634.6 ns, slowest 1641.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Resource storage layouts under light surrounding work |
| Passes | 5 |
| Runs per pass | 2000 |
| Batch size | 250 |
| Harness runs | 3 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112842590 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| v0_blob | 1668ns | 1792ns | 1466ns | 1689ns | 1809ns | base |
| v1_snapshot | 1656ns | 1796ns | 1474ns | 1662ns | 1819ns | -0.73% |
| v2_decomposed | 1699ns | 1804ns | 1474ns | 1699ns | 1923ns | +1.84% |
| v3_shapebound | 1658ns | 1800ns | 1480ns | 1663ns | 1823ns | -0.58% |
| v4_erased | 1656ns | 1798ns | 1471ns | 1663ns | 1819ns | -0.74% |
| v5_handletable | 1651ns | 1798ns | 1471ns | 1658ns | 1812ns | -1.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 1519ns | 1338ns | 1642ns | base | 10.789 |
| v1_snapshot | 1505ns | 1341ns | 1648ns | -0.93% | 10.890 |
| v2_decomposed | 1544ns | 1342ns | 1743ns | +1.64% | 10.615 |
| v3_shapebound | 1507ns | 1345ns | 1654ns | -0.78% | 10.874 |
| v4_erased | 1505ns | 1342ns | 1649ns | -0.86% | 10.883 |
| v5_handletable | 1501ns | 1339ns | 1643ns | -1.19% | 10.919 |

## Performance model

- Peak throughput: **12.246 Gops/s** (v0_blob; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 10.023 | 81.8% |
| v1_snapshot | 10.010 | 81.7% |
| v2_decomposed | 9.984 | 81.5% |
| v3_shapebound | 9.988 | 81.6% |
| v4_erased | 10.007 | 81.7% |
| v5_handletable | 10.016 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 1668ns | 1668ns | base |
| v1_snapshot | 1656ns | 1656ns | -0.73% |
| v2_decomposed | 1699ns | 1699ns | +1.84% |
| v3_shapebound | 1658ns | 1658ns | -0.58% |
| v4_erased | 1656ns | 1656ns | -0.74% |
| v5_handletable | 1651ns | 1651ns | -1.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 1635ns | base | --- | [1633, 1636] | --- | --- | --- | --- |
| v1_snapshot | 1637ns | +3.6ns (+0.2%) | [+2, +6]ns | [1350, 1639] | YES | 0.0016 | 0.0013 | 0 |
| v2_decomposed | 1641ns | +5.8ns (+0.4%) | [+5, +9]ns | [1639, 1642] | YES | 0.0001 | 0.0001 | 0 |
| v3_shapebound | 1640ns | +8.1ns (+0.5%) | [+6, +9]ns | [1354, 1644] | YES | 0.0000 | 0.0000 | 0 |
| v4_erased | 1637ns | +4.8ns (+0.3%) | [+3, +6]ns | [1353, 1641] | YES | 0.0000 | 0.0000 | 0 |
| v5_handletable | 1636ns | no significant difference | [-1, +3]ns | [1345, 1637] | no | 0.2712 | 0.2712 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1338ns | +0.4% | +22.8% | +0.6% | +22.7% | +22.4% |
| 1 | 2 | 1636ns | -17.7% | +0.7% | -17.6% | -17.7% | +0.1% |
| 1 | 3 | 1339ns | +22.5% | +22.7% | +22.9% | +22.9% | +0.1% |
| 1 | 4 | 1637ns | +0.2% | -17.9% | -17.6% | +0.5% | -18.1% |
| 1 | 5 | 1638ns | -18.1% | +0.5% | +0.6% | -18.0% | -18.2% |
| 2 | 1 | 1637ns | +0.5% | -17.9% | -17.4% | -17.8% | +0.1% |
| 2 | 2 | 1638ns | +0.2% | +0.4% | +0.6% | +0.5% | +0.1% |
| 2 | 3 | 1637ns | +0.4% | +0.4% | -17.6% | +0.5% | -18.1% |
| 2 | 4 | 1339ns | +0.5% | +0.4% | +23.2% | +22.8% | +0.1% |
| 2 | 5 | 1639ns | -18.0% | -17.9% | +0.6% | -17.4% | -18.1% |
| 3 | 1 | 1340ns | +0.3% | +0.2% | +0.4% | +0.3% | +22.5% |
| 3 | 2 | 1340ns | +22.5% | +22.7% | +23.0% | +0.4% | +22.3% |
| 3 | 3 | 1636ns | -17.7% | +0.5% | +0.6% | +0.4% | -18.1% |
| 3 | 4 | 1637ns | +0.3% | -17.7% | +0.6% | +0.1% | -0.0% |
| 3 | 5 | 1339ns | +22.6% | +22.9% | +0.6% | +0.3% | +22.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.211 | moderate- |
| v1_snapshot | -0.006 | ok |
| v2_decomposed | -0.156 | ok |
| v3_shapebound | -0.415 | moderate- |
| v4_erased | -0.003 | ok |
| v5_handletable | 0.139 | ok |

**Consistency summary:**

- **v1_snapshot**: won 4/15, lost 11/15
- **v2_decomposed**: won 4/15, lost 11/15
- **v3_shapebound**: won 4/15, lost 11/15
- **v4_erased**: won 4/15, lost 11/15
- **v5_handletable**: won 5/15, lost 4/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 36034.6ns | 1518.6ns | 2372.9% | HIGH |
| v1_snapshot | 35580.7ns | 1504.5ns | 2364.9% | HIGH |
| v2_decomposed | 36521.1ns | 1543.5ns | 2366.1% | HIGH |
| v3_shapebound | 35540.6ns | 1506.7ns | 2358.9% | HIGH |
| v4_erased | 35569.4ns | 1505.5ns | 2362.7% | HIGH |
| v5_handletable | 35612.3ns | 1500.5ns | 2373.3% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 1337.9-1641.8 ns)
   1337.9 |#########################
   1353.1 |
   1368.3 |
   1383.5 |
   1398.6 |
   1413.8 |
   1429.0 |
   1444.2 |
   1459.4 |
   1474.6 |
   1489.8 |
   1505.0 |
   1520.2 |
   1535.4 |
   1550.6 |
   1565.8 |
   1581.0 |
   1596.2 |
   1611.4 |
   1626.6 |########################################
  (10 below, 12 above range)

v1_snapshot (n=120, range 1341.5-1648.5 ns)
   1341.5 |############################
   1356.8 |
   1372.2 |#
   1387.5 |
   1402.9 |
   1418.2 |
   1433.6 |
   1448.9 |
   1464.3 |
   1479.6 |
   1495.0 |
   1510.3 |
   1525.7 |
   1541.0 |
   1556.4 |
   1571.7 |
   1587.1 |
   1602.4 |
   1617.8 |
   1633.1 |########################################
  (11 below, 5 above range)

v2_decomposed (n=120, range 1342.3-1743.1 ns)
   1342.3 |################################
   1362.3 |
   1382.4 |
   1402.4 |
   1422.4 |
   1442.5 |
   1462.5 |
   1482.6 |
   1502.6 |
   1522.6 |
   1542.7 |
   1562.7 |
   1582.8 |
   1602.8 |
   1622.8 |####################
   1642.9 |########################################
   1662.9 |
   1682.9 |
   1703.0 |
   1723.0 |
  (10 below, 1 above range)

v3_shapebound (n=120, range 1344.7-1653.6 ns)
   1344.7 |##############################
   1360.1 |
   1375.6 |
   1391.0 |
   1406.5 |
   1421.9 |
   1437.3 |
   1452.8 |
   1468.2 |
   1483.7 |
   1499.1 |
   1514.6 |
   1530.0 |
   1545.4 |
   1560.9 |
   1576.3 |
   1591.8 |
   1607.2 |
   1622.7 |
   1638.1 |########################################
  (10 below, 5 above range)

v4_erased (n=120, range 1341.7-1648.6 ns)
   1341.7 |#############################
   1357.1 |##
   1372.4 |
   1387.8 |
   1403.1 |
   1418.4 |
   1433.8 |
   1449.1 |
   1464.5 |
   1479.8 |
   1495.2 |
   1510.5 |
   1525.9 |
   1541.2 |
   1556.5 |
   1571.9 |
   1587.2 |
   1602.6 |
   1617.9 |
   1633.3 |########################################
  (12 below, 9 above range)

v5_handletable (n=120, range 1338.9-1643.5 ns)
   1338.9 |###############################
   1354.1 |
   1369.3 |
   1384.5 |
   1399.8 |
   1415.0 |
   1430.2 |
   1445.5 |
   1460.7 |
   1475.9 |
   1491.2 |
   1506.4 |
   1521.6 |
   1536.9 |
   1552.1 |
   1567.3 |
   1582.5 |
   1597.8 |
   1613.0 |
   1628.2 |########################################
  (12 below, 10 above range)

```

## Diagnostics

- **v0_blob**: bridge=2376.2% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=2373.1% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=2367.1% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=2368.0% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=2372.2% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=2376.5% of algo (FFI overhead may distort results)

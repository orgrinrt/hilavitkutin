# Resource storage layouts under clean morsel loop (algo only)

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (v5_handletable, v1_snapshot) are a dead heat (<1%)

v5_handletable (1.34 us) and v1_snapshot (1.35 us) differ by 0.47%, inside the noise, even though the wider field spreads 22.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### v3_shapebound shows alternating (throttle bounce) (autocorr -0.67)

v3_shapebound's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### v2_decomposed's edge over baseline is significant but tiny (5 ns, 0.30%)

v2_decomposed differs from baseline v0_blob by 5 ns (0.30%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v5_handletable** at 1344.5 ns median (-17.7% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.22x (fastest 1344.5 ns, slowest 1640.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Resource storage layouts under clean morsel loop (algo only) |
| Passes | 5 |
| Runs per pass | 2000 |
| Batch size | 250 |
| Harness runs | 3 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112727760 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| v0_blob | 1581ns | 1697ns | 1387ns | 1567ns | 1816ns | base |
| v1_snapshot | 1545ns | 1415ns | 1394ns | 1537ns | 1722ns | -2.26% |
| v2_decomposed | 1592ns | 1703ns | 1395ns | 1581ns | 1825ns | +0.73% |
| v3_shapebound | 1547ns | 1420ns | 1396ns | 1539ns | 1720ns | -2.15% |
| v4_erased | 1569ns | 1700ns | 1394ns | 1575ns | 1723ns | -0.76% |
| v5_handletable | 1519ns | 1399ns | 1389ns | 1498ns | 1711ns | -3.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 1521ns | 1338ns | 1742ns | base | 10.775 |
| v1_snapshot | 1486ns | 1341ns | 1655ns | -2.28% | 11.026 |
| v2_decomposed | 1530ns | 1342ns | 1749ns | +0.62% | 10.708 |
| v3_shapebound | 1487ns | 1344ns | 1651ns | -2.21% | 11.018 |
| v4_erased | 1507ns | 1343ns | 1650ns | -0.87% | 10.870 |
| v5_handletable | 1461ns | 1339ns | 1643ns | -3.92% | 11.215 |

## Performance model

- Peak throughput: **12.242 Gops/s** (v0_blob; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 10.024 | 81.9% |
| v1_snapshot | 12.129 | 99.1% |
| v2_decomposed | 9.987 | 81.6% |
| v3_shapebound | 12.120 | 99.0% |
| v4_erased | 10.013 | 81.8% |
| v5_handletable | 12.186 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 1581ns | 1581ns | base |
| v1_snapshot | 1545ns | 1545ns | -2.26% |
| v2_decomposed | 1592ns | 1592ns | +0.73% |
| v3_shapebound | 1547ns | 1547ns | -2.15% |
| v4_erased | 1569ns | 1569ns | -0.76% |
| v5_handletable | 1519ns | 1519ns | -3.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 1634ns | base | --- | [1345, 1637] | --- | --- | --- | --- |
| v1_snapshot | 1351ns | no significant difference | [-0, +6]ns | [1348, 1640] | no | 0.1030 | 0.0824 | 0 |
| v2_decomposed | 1641ns | +4.9ns (+0.3%) | [+3, +7]ns | [1351, 1642] | YES | 0.0041 | 0.0024 | 0 |
| v3_shapebound | 1352ns | +5.1ns (+0.3%) | [+3, +7]ns | [1349, 1643] | YES | 0.0000 | 0.0000 | 0 |
| v4_erased | 1636ns | +4.9ns (+0.3%) | [+3, +7]ns | [1355, 1640] | YES | 0.0032 | 0.0013 | 0 |
| v5_handletable | 1344ns | no significant difference | [-4, +1]ns | [1342, 1351] | no | 0.3153 | 0.3153 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1639ns | -18.1% | +0.4% | -17.8% | -17.8% | -0.0% |
| 1 | 2 | 1640ns | -17.8% | -18.1% | +0.4% | +0.3% | -18.2% |
| 1 | 3 | 1339ns | +0.3% | +22.8% | +0.4% | +22.8% | +0.2% |
| 1 | 4 | 1341ns | +0.1% | +0.5% | +22.9% | +0.4% | +22.3% |
| 1 | 5 | 1341ns | +22.6% | +22.7% | +0.4% | +0.3% | +22.2% |
| 2 | 1 | 1645ns | -18.2% | -0.1% | +0.1% | -18.3% | -18.4% |
| 2 | 2 | 1637ns | +0.4% | +0.4% | +0.5% | +0.1% | -18.1% |
| 2 | 3 | 1641ns | -17.9% | -18.1% | -17.9% | +0.4% | -18.3% |
| 2 | 4 | 1640ns | -18.1% | -18.0% | -17.9% | -17.5% | -0.1% |
| 2 | 5 | 1638ns | +0.3% | +0.3% | +0.4% | +0.3% | +0.0% |
| 3 | 1 | 1344ns | +22.4% | +0.5% | +0.3% | +22.8% | -0.3% |
| 3 | 2 | 1639ns | +0.4% | -17.8% | +0.6% | -17.6% | -18.1% |
| 3 | 3 | 1341ns | +0.2% | +22.7% | +0.4% | +22.6% | +22.3% |
| 3 | 4 | 1340ns | +22.8% | +22.9% | +22.8% | +0.4% | +0.2% |
| 3 | 5 | 1340ns | +22.7% | +0.3% | +0.4% | +22.6% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | 0.266 | moderate+ |
| v1_snapshot | -0.005 | ok |
| v2_decomposed | -0.273 | moderate- |
| v3_shapebound | -0.667 | HIGH- (thermal bounce) |
| v4_erased | -0.264 | moderate- |
| v5_handletable | -0.045 | ok |

**Consistency summary:**

- **v1_snapshot**: won 5/15, lost 10/15
- **v2_decomposed**: won 4/15, lost 10/15
- **v3_shapebound**: won 3/15, lost 11/15
- **v4_erased**: won 4/15, lost 11/15
- **v5_handletable**: won 7/15, lost 5/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 36045.9ns | 1520.6ns | 2370.5% | HIGH |
| v1_snapshot | 35088.1ns | 1486.0ns | 2361.3% | HIGH |
| v2_decomposed | 36059.0ns | 1530.0ns | 2356.7% | HIGH |
| v3_shapebound | 35083.2ns | 1487.0ns | 2359.4% | HIGH |
| v4_erased | 35602.8ns | 1507.3ns | 2362.0% | HIGH |
| v5_handletable | 34643.9ns | 1461.0ns | 2371.3% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 1338.3-1742.0 ns)
   1338.3 |########################################
   1358.5 |#
   1378.7 |
   1398.9 |
   1419.0 |
   1439.2 |
   1459.4 |
   1479.6 |
   1499.8 |
   1520.0 |
   1540.2 |
   1560.3 |
   1580.5 |
   1600.7 |
   1620.9 |#####################################
   1641.1 |#####################
   1661.3 |
   1681.4 |
   1701.6 |
   1721.8 |
  (11 below, 1 above range)

v1_snapshot (n=120, range 1341.1-1654.6 ns)
   1341.1 |########################################
   1356.8 |
   1372.5 |
   1388.1 |
   1403.8 |
   1419.5 |
   1435.2 |
   1450.8 |
   1466.5 |
   1482.2 |
   1497.9 |
   1513.5 |
   1529.2 |
   1544.9 |
   1560.6 |
   1576.2 |
   1591.9 |
   1607.6 |
   1623.2 |##
   1638.9 |######################################
  (10 below, 1 above range)

v2_decomposed (n=120, range 1342.5-1749.1 ns)
   1342.5 |#################################
   1362.8 |
   1383.1 |
   1403.5 |
   1423.8 |
   1444.1 |
   1464.5 |
   1484.8 |
   1505.1 |
   1525.4 |
   1545.8 |
   1566.1 |
   1586.4 |
   1606.8 |
   1627.1 |########################################
   1647.4 |############
   1667.8 |
   1688.1 |
   1708.4 |
   1728.7 |
  (11 below, 1 above range)

v3_shapebound (n=120, range 1343.7-1651.5 ns)
   1343.7 |########################################
   1359.1 |
   1374.4 |
   1389.8 |
   1405.2 |
   1420.6 |
   1436.0 |
   1451.4 |
   1466.8 |
   1482.2 |
   1497.6 |
   1513.0 |
   1528.4 |
   1543.7 |
   1559.1 |
   1574.5 |
   1589.9 |
   1605.3 |
   1620.7 |
   1636.1 |###################################
  (11 below, 9 above range)

v4_erased (n=120, range 1343.0-1650.3 ns)
   1343.0 |#################################
   1358.4 |
   1373.7 |
   1389.1 |
   1404.5 |
   1419.8 |
   1435.2 |
   1450.6 |
   1465.9 |
   1481.3 |
   1496.7 |
   1512.0 |
   1527.4 |
   1542.8 |
   1558.1 |
   1573.5 |
   1588.9 |
   1604.2 |
   1619.6 |
   1635.0 |########################################
  (8 below, 10 above range)

v5_handletable (n=120, range 1339.3-1642.5 ns)
   1339.3 |########################################
   1354.4 |
   1369.6 |
   1384.8 |
   1399.9 |
   1415.1 |
   1430.3 |
   1445.4 |
   1460.6 |
   1475.7 |
   1490.9 |
   1506.1 |
   1521.2 |
   1536.4 |
   1551.6 |
   1566.7 |
   1581.9 |
   1597.0 |
   1612.2 |
   1627.4 |########################
  (9 below, 10 above range)

```

## Diagnostics

- **v0_blob**: bridge=2376.1% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=2352.9% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=2367.4% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=2354.5% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=2373.5% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=2365.8% of algo (FFI overhead may distort results)

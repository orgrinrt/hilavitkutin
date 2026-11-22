# Resource storage layouts under light surrounding work

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (v5_handletable, v3_shapebound) are a dead heat (<1%)

v5_handletable (1.34 us) and v3_shapebound (1.35 us) differ by 0.67%, inside the noise, even though the wider field spreads 22.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### v0_blob shows alternating (throttle bounce) (autocorr -0.54)

v0_blob's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### v1_snapshot's edge over baseline is significant but tiny (4 ns, 0.27%)

v1_snapshot differs from baseline v0_blob by 4 ns (0.27%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v5_handletable** at 1344.5 ns median (-17.8% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.22x (fastest 1344.5 ns, slowest 1641.6 ns)

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
| Counter frequency | 2112727760 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| v0_blob | 1651ns | 1796ns | 1470ns | 1657ns | 1815ns | base |
| v1_snapshot | 1699ns | 1800ns | 1474ns | 1731ns | 1829ns | +2.91% |
| v2_decomposed | 1703ns | 1795ns | 1469ns | 1662ns | 2059ns | +3.14% |
| v3_shapebound | 1629ns | 1499ns | 1477ns | 1615ns | 1820ns | -1.38% |
| v4_erased | 1722ns | 1803ns | 1476ns | 1769ns | 1825ns | +4.26% |
| v5_handletable | 1608ns | 1483ns | 1470ns | 1585ns | 1816ns | -2.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 1500ns | 1338ns | 1642ns | base | 10.921 |
| v1_snapshot | 1544ns | 1341ns | 1656ns | +2.94% | 10.609 |
| v2_decomposed | 1547ns | 1339ns | 1864ns | +3.12% | 10.590 |
| v3_shapebound | 1479ns | 1343ns | 1650ns | -1.43% | 11.079 |
| v4_erased | 1565ns | 1343ns | 1652ns | +4.29% | 10.471 |
| v5_handletable | 1463ns | 1339ns | 1650ns | -2.52% | 11.203 |

## Performance model

- Peak throughput: **12.241 Gops/s** (v0_blob; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 10.018 | 81.8% |
| v1_snapshot | 9.995 | 81.7% |
| v2_decomposed | 10.017 | 81.8% |
| v3_shapebound | 12.104 | 98.9% |
| v4_erased | 9.981 | 81.5% |
| v5_handletable | 12.186 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 1651ns | 1651ns | base |
| v1_snapshot | 1699ns | 1699ns | +2.91% |
| v2_decomposed | 1703ns | 1703ns | +3.14% |
| v3_shapebound | 1629ns | 1629ns | -1.38% |
| v4_erased | 1722ns | 1722ns | +4.26% |
| v5_handletable | 1608ns | 1608ns | -2.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 1635ns | base | --- | [1344, 1637] | --- | --- | --- | --- |
| v1_snapshot | 1639ns | +4.5ns (+0.3%) | [+2, +91]ns | [1638, 1640] | YES | 0.0080 | 0.0032 | 1 |
| v2_decomposed | 1636ns | no significant difference | [-1, +3]ns | [1346, 1637] | no | 0.9782 | 0.7826 | 2 |
| v3_shapebound | 1354ns | +5.0ns (+0.3%) | [+3, +9]ns | [1350, 1640] | YES | 0.0223 | 0.0134 | 0 |
| v4_erased | 1642ns | +6.9ns (+0.4%) | [+5, +10]ns | [1640, 1643] | YES | 0.0000 | 0.0000 | 2 |
| v5_handletable | 1344ns | no significant difference | [-3, +1]ns | [1342, 1354] | no | 1.0000 | 1.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1638ns | -18.0% | +0.2% | -17.9% | -18.0% | +0.0% |
| 1 | 2 | 1638ns | -17.9% | -18.1% | +0.7% | +0.6% | -18.1% |
| 1 | 3 | 1343ns | +22.3% | -0.1% | +0.2% | +22.4% | +22.0% |
| 1 | 4 | 1637ns | +0.6% | +0.3% | -17.8% | +0.4% | -18.2% |
| 1 | 5 | 1639ns | +0.1% | +0.1% | -17.6% | +0.1% | +0.0% |
| 2 | 1 | 1342ns | +0.0% | -0.1% | +0.5% | +0.1% | +22.1% |
| 2 | 2 | 1339ns | +22.6% | +25.6% | +23.0% | +0.4% | +0.1% |
| 2 | 3 | 1638ns | -18.0% | -18.3% | -17.8% | +0.2% | -18.2% |
| 2 | 4 | 1339ns | +22.5% | +22.5% | +0.6% | +0.5% | +23.3% |
| 2 | 5 | 1640ns | -18.0% | -0.1% | -17.9% | +0.3% | -18.2% |
| 3 | 1 | 1342ns | +22.4% | -0.0% | +22.8% | +22.6% | -0.0% |
| 3 | 2 | 1639ns | +0.1% | -18.0% | -2.4% | +0.5% | -18.1% |
| 3 | 3 | 1340ns | +22.7% | +0.1% | +22.9% | +22.6% | +0.1% |
| 3 | 4 | 1639ns | +0.1% | +0.2% | +0.5% | +0.6% | -0.1% |
| 3 | 5 | 1341ns | +22.6% | +22.2% | +23.1% | +22.7% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.543 | HIGH- (thermal bounce) |
| v1_snapshot | -0.127 | ok |
| v2_decomposed | -0.167 | ok |
| v3_shapebound | 0.221 | moderate+ |
| v4_erased | 0.054 | ok |
| v5_handletable | -0.327 | moderate- |

**Consistency summary:**

- **v1_snapshot**: won 4/15, lost 8/15
- **v2_decomposed**: won 3/15, lost 8/15
- **v3_shapebound**: won 6/15, lost 9/15
- **v4_erased**: won 1/15, lost 12/15
- **v5_handletable**: won 5/15, lost 4/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 35572.1ns | 1500.2ns | 2371.1% | HIGH |
| v1_snapshot | 36570.7ns | 1544.3ns | 2368.1% | HIGH |
| v2_decomposed | 36654.8ns | 1547.1ns | 2369.3% | HIGH |
| v3_shapebound | 34895.8ns | 1478.9ns | 2359.6% | HIGH |
| v4_erased | 36984.3ns | 1564.6ns | 2363.7% | HIGH |
| v5_handletable | 34668.7ns | 1462.5ns | 2370.5% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 1338.5-1642.1 ns)
   1338.5 |################################
   1353.7 |
   1368.9 |
   1384.0 |
   1399.2 |
   1414.4 |
   1429.6 |
   1444.7 |
   1459.9 |
   1475.1 |
   1490.3 |
   1505.5 |
   1520.6 |
   1535.8 |
   1551.0 |
   1566.2 |
   1581.3 |
   1596.5 |
   1611.7 |
   1626.9 |########################################
  (10 below, 9 above range)

v1_snapshot (n=120, range 1341.4-1655.5 ns)
   1341.4 |######################
   1357.1 |
   1372.8 |
   1388.6 |
   1404.3 |
   1420.0 |
   1435.7 |
   1451.4 |
   1467.1 |
   1482.8 |
   1498.5 |
   1514.2 |
   1529.9 |
   1545.6 |
   1561.3 |
   1577.0 |
   1592.7 |
   1608.4 |
   1624.1 |###################
   1639.8 |########################################
  (11 below, 4 above range)

v2_decomposed (n=120, range 1338.9-1864.5 ns)
   1338.9 |#############################
   1365.1 |
   1391.4 |
   1417.7 |
   1444.0 |
   1470.3 |
   1496.5 |
   1522.8 |
   1549.1 |
   1575.4 |
   1601.7 |
   1628.0 |########################################
   1654.2 |
   1680.5 |
   1706.8 |
   1733.1 |
   1759.4 |
   1785.6 |
   1811.9 |
   1838.2 |
  (11 below, 2 above range)

v3_shapebound (n=120, range 1343.5-1650.2 ns)
   1343.5 |########################################
   1358.8 |
   1374.2 |
   1389.5 |
   1404.8 |
   1420.2 |
   1435.5 |
   1450.8 |
   1466.2 |
   1481.5 |
   1496.8 |
   1512.2 |
   1527.5 |
   1542.9 |
   1558.2 |
   1573.5 |
   1588.9 |
   1604.2 |
   1619.5 |
   1634.9 |#############################
  (10 below, 11 above range)

v4_erased (n=120, range 1342.7-1651.6 ns)
   1342.7 |###########
   1358.2 |
   1373.6 |
   1389.1 |
   1404.5 |
   1420.0 |
   1435.4 |
   1450.8 |
   1466.3 |
   1481.7 |
   1497.2 |
   1512.6 |
   1528.0 |
   1543.5 |
   1558.9 |
   1574.4 |
   1589.8 |
   1605.3 |
   1620.7 |##
   1636.1 |########################################
  (10 below, 9 above range)

v5_handletable (n=120, range 1339.2-1649.8 ns)
   1339.2 |########################################
   1354.8 |
   1370.3 |
   1385.8 |
   1401.3 |
   1416.9 |
   1432.4 |
   1447.9 |
   1463.5 |
   1479.0 |
   1494.5 |
   1510.0 |
   1525.6 |
   1541.1 |
   1556.6 |
   1572.2 |
   1587.7 |
   1603.2 |
   1618.7 |
   1634.3 |#############################
  (12 below, 4 above range)

```

## Diagnostics

- **v0_blob**: bridge=2374.9% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=2369.7% of algo (FFI overhead may distort results)
- **v2_decomposed**: CV=23.9% (high variance, measurements may be unstable)
- **v2_decomposed**: bridge=2374.5% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=2348.7% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=2366.4% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=2366.0% of algo (FFI overhead may distort results)

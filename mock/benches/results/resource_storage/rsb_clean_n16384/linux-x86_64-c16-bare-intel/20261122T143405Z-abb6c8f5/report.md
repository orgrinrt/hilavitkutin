# Resource storage layouts under clean morsel loop (algo only)

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### v3_shapebound dominates: 21% faster than the next best (v0_blob)

v3_shapebound (1.35 us) leads v0_blob (1.63 us) by 21%, a clear separation rather than a photo finish. CV 10.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### v3_shapebound is fastest but the noisiest (CV 10.8%)

v3_shapebound wins on median (1.35 us) yet has the highest variance (CV 10.8%), while v5_handletable is the steadiest (CV 8.1%, 1.64 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### v2_decomposed shows warm-up / thermal drift (autocorr +0.57)

v2_decomposed's per-pass series has lag-1 autocorrelation +0.57, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### v1_snapshot's edge over baseline is significant but tiny (6 ns, 0.37%)

v1_snapshot differs from baseline v0_blob by 6 ns (0.37%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v3_shapebound** at 1350.6 ns median (-17.4% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 1.22x (fastest 1350.6 ns, slowest 1641.6 ns)

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
| Counter frequency | 2112759259 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| v0_blob | 1559ns | 1692ns | 1389ns | 1565ns | 1709ns | base |
| v1_snapshot | 1584ns | 1701ns | 1394ns | 1604ns | 1717ns | +1.64% |
| v2_decomposed | 1607ns | 1706ns | 1395ns | 1640ns | 1719ns | +3.07% |
| v3_shapebound | 1527ns | 1407ns | 1398ns | 1506ns | 1717ns | -2.06% |
| v4_erased | 1586ns | 1702ns | 1392ns | 1605ns | 1721ns | +1.71% |
| v5_handletable | 1624ns | 1703ns | 1391ns | 1670ns | 1718ns | +4.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 1500ns | 1338ns | 1643ns | base | 10.925 |
| v1_snapshot | 1524ns | 1341ns | 1647ns | +1.59% | 10.754 |
| v2_decomposed | 1544ns | 1342ns | 1647ns | +2.93% | 10.614 |
| v3_shapebound | 1468ns | 1345ns | 1651ns | -2.12% | 11.162 |
| v4_erased | 1525ns | 1342ns | 1651ns | +1.71% | 10.741 |
| v5_handletable | 1562ns | 1342ns | 1648ns | +4.17% | 10.488 |

## Performance model

- Peak throughput: **12.246 Gops/s** (v0_blob; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 10.025 | 81.9% |
| v1_snapshot | 9.999 | 81.6% |
| v2_decomposed | 9.981 | 81.5% |
| v3_shapebound | 12.131 | 99.1% |
| v4_erased | 9.984 | 81.5% |
| v5_handletable | 9.993 | 81.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 1559ns | 1559ns | base |
| v1_snapshot | 1584ns | 1584ns | +1.64% |
| v2_decomposed | 1607ns | 1607ns | +3.07% |
| v3_shapebound | 1527ns | 1527ns | -2.06% |
| v4_erased | 1586ns | 1586ns | +1.71% |
| v5_handletable | 1624ns | 1624ns | +4.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 1634ns | base | --- | [1344, 1636] | --- | --- | --- | --- |
| v1_snapshot | 1639ns | +6.0ns (+0.4%) | [+2, +11]ns | [1636, 1641] | YES | 0.0031 | 0.0024 | 0 |
| v2_decomposed | 1642ns | +7.4ns (+0.5%) | [+6, +9]ns | [1641, 1642] | YES | 0.0000 | 0.0000 | 0 |
| v3_shapebound | 1351ns | no significant difference | [-1, +10]ns | [1349, 1359] | no | 0.0824 | 0.0824 | 0 |
| v4_erased | 1641ns | +7.5ns (+0.5%) | [+5, +9]ns | [1634, 1642] | YES | 0.0000 | 0.0000 | 0 |
| v5_handletable | 1640ns | +4.8ns (+0.3%) | [+4, +8]ns | [1638, 1640] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1338ns | +0.7% | +0.5% | +0.9% | +0.4% | +0.4% |
| 1 | 2 | 1636ns | -17.9% | -17.9% | -17.6% | +0.5% | +0.3% |
| 1 | 3 | 1339ns | +22.6% | +22.6% | +0.6% | +0.9% | +22.7% |
| 1 | 4 | 1636ns | +0.4% | +0.4% | +0.9% | +0.7% | -17.9% |
| 1 | 5 | 1637ns | +0.5% | +0.3% | -17.7% | -18.0% | -17.9% |
| 2 | 1 | 1638ns | -17.7% | +0.4% | -17.7% | +0.2% | +0.3% |
| 2 | 2 | 1637ns | +0.3% | +0.4% | -17.8% | +0.3% | +0.3% |
| 2 | 3 | 1340ns | +22.5% | +22.6% | +23.0% | +22.7% | +22.5% |
| 2 | 4 | 1341ns | +22.6% | +22.6% | +0.5% | +22.9% | +22.2% |
| 2 | 5 | 1341ns | +22.3% | +22.5% | +22.8% | +22.5% | +22.4% |
| 3 | 1 | 1638ns | +0.4% | +0.3% | +0.5% | +0.4% | +0.1% |
| 3 | 2 | 1342ns | +22.3% | +22.5% | +22.6% | +0.3% | +22.2% |
| 3 | 3 | 1637ns | -18.0% | -18.0% | -17.7% | +0.3% | +0.3% |
| 3 | 4 | 1639ns | -18.0% | -18.3% | -17.7% | -17.8% | +0.1% |
| 3 | 5 | 1341ns | +0.3% | +0.5% | +22.8% | +0.2% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.148 | ok |
| v1_snapshot | 0.348 | moderate+ |
| v2_decomposed | 0.569 | HIGH+ (drift/warm-up) |
| v3_shapebound | -0.052 | ok |
| v4_erased | -0.211 | moderate- |
| v5_handletable | 0.130 | ok |

**Consistency summary:**

- **v1_snapshot**: won 4/15, lost 11/15
- **v2_decomposed**: won 3/15, lost 12/15
- **v3_shapebound**: won 6/15, lost 9/15
- **v4_erased**: won 2/15, lost 13/15
- **v5_handletable**: won 2/15, lost 11/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 35575.2ns | 1499.7ns | 2372.2% | HIGH |
| v1_snapshot | 36030.6ns | 1523.5ns | 2364.9% | HIGH |
| v2_decomposed | 36518.8ns | 1543.6ns | 2365.9% | HIGH |
| v3_shapebound | 34615.0ns | 1467.9ns | 2358.1% | HIGH |
| v4_erased | 36030.0ns | 1525.3ns | 2362.1% | HIGH |
| v5_handletable | 37009.3ns | 1562.2ns | 2369.1% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 1337.9-1642.9 ns)
   1337.9 |##############################
   1353.1 |
   1368.4 |
   1383.6 |
   1398.9 |
   1414.1 |
   1429.4 |
   1444.6 |
   1459.9 |
   1475.1 |
   1490.4 |
   1505.7 |
   1520.9 |
   1536.2 |
   1551.4 |
   1566.7 |
   1581.9 |
   1597.2 |
   1612.4 |
   1627.7 |########################################
  (14 below, 10 above range)

v1_snapshot (n=120, range 1341.5-1646.6 ns)
   1341.5 |########################
   1356.7 |
   1372.0 |
   1387.2 |
   1402.5 |
   1417.8 |
   1433.0 |
   1448.3 |
   1463.5 |
   1478.8 |
   1494.0 |
   1509.3 |
   1524.5 |
   1539.8 |
   1555.0 |
   1570.3 |
   1585.6 |
   1600.8 |
   1616.1 |
   1631.3 |########################################
  (12 below, 12 above range)

v2_decomposed (n=120, range 1341.5-1647.4 ns)
   1341.5 |################
   1356.8 |
   1372.1 |
   1387.4 |
   1402.7 |
   1418.0 |
   1433.3 |
   1448.6 |
   1463.8 |
   1479.1 |
   1494.4 |
   1509.7 |
   1525.0 |
   1540.3 |
   1555.6 |
   1570.9 |
   1586.2 |
   1601.5 |
   1616.8 |
   1632.1 |########################################
  (11 below, 11 above range)

v3_shapebound (n=120, range 1345.3-1650.8 ns)
   1345.3 |########################################
   1360.6 |
   1375.8 |
   1391.1 |
   1406.4 |
   1421.7 |
   1436.9 |
   1452.2 |
   1467.5 |
   1482.8 |
   1498.0 |
   1513.3 |
   1528.6 |
   1543.9 |
   1559.1 |
   1574.4 |
   1589.7 |
   1605.0 |
   1620.2 |
   1635.5 |####################
  (6 below, 14 above range)

v4_erased (n=120, range 1341.7-1650.6 ns)
   1341.7 |#######################
   1357.1 |
   1372.6 |
   1388.0 |
   1403.4 |
   1418.9 |
   1434.3 |
   1449.8 |
   1465.2 |
   1480.7 |
   1496.1 |
   1511.6 |
   1527.0 |
   1542.5 |
   1557.9 |
   1573.4 |
   1588.8 |
   1604.2 |
   1619.7 |#
   1635.1 |########################################
  (13 below, 11 above range)

v5_handletable (n=120, range 1341.8-1648.2 ns)
   1341.8 |########
   1357.1 |
   1372.4 |
   1387.7 |
   1403.1 |
   1418.4 |
   1433.7 |
   1449.0 |
   1464.4 |
   1479.7 |
   1495.0 |
   1510.3 |
   1525.6 |
   1541.0 |
   1556.3 |
   1571.6 |
   1586.9 |
   1602.3 |
   1617.6 |
   1632.9 |########################################
  (13 below, 2 above range)

```

## Diagnostics

- **v0_blob**: bridge=2376.2% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=2370.2% of algo (FFI overhead may distort results)
- **v2_decomposed**: autocorrelation=0.57 (measurement drift or warm-up artifact)
- **v2_decomposed**: bridge=2366.0% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=2353.0% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=2366.8% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=2371.2% of algo (FFI overhead may distort results)

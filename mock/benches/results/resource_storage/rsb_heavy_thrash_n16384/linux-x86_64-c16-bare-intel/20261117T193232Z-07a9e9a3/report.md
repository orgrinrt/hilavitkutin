# Resource storage layouts under L1-evicting heavy-memory mix-in

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (v2_decomposed, v4_erased) are a dead heat (<1%)

v2_decomposed (1.35 us) and v4_erased (1.35 us) differ by 0.14%, inside the noise, even though the wider field spreads 21.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### v1_snapshot's edge over baseline is significant but tiny (4 ns, 0.31%)

v1_snapshot differs from baseline v0_blob by 4 ns (0.31%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v2_decomposed** at 1345.2 ns median (-0.5% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.22x (fastest 1345.2 ns, slowest 1638.6 ns)

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
| Counter frequency | 2112727760 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| v0_blob | 5298ns | 4836ns | 4787ns | 5273ns | 5887ns | base |
| v1_snapshot | 5513ns | 5859ns | 4792ns | 5627ns | 5893ns | +4.05% |
| v2_decomposed | 5296ns | 4814ns | 4789ns | 5272ns | 5874ns | -0.05% |
| v3_shapebound | 5221ns | 4803ns | 4790ns | 5040ns | 6194ns | -1.46% |
| v4_erased | 5158ns | 4805ns | 4791ns | 5040ns | 5880ns | -2.65% |
| v5_handletable | 5584ns | 5856ns | 4793ns | 5743ns | 5902ns | +5.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 1482ns | 1339ns | 1645ns | base | 11.057 |
| v1_snapshot | 1543ns | 1341ns | 1650ns | +4.11% | 10.621 |
| v2_decomposed | 1480ns | 1339ns | 1643ns | -0.10% | 11.068 |
| v3_shapebound | 1464ns | 1342ns | 1736ns | -1.22% | 11.194 |
| v4_erased | 1445ns | 1340ns | 1650ns | -2.48% | 11.338 |
| v5_handletable | 1561ns | 1340ns | 1650ns | +5.38% | 10.493 |

## Performance model

- Peak throughput: **12.240 Gops/s** (v2_decomposed; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 12.114 | 99.0% |
| v1_snapshot | 9.999 | 81.7% |
| v2_decomposed | 12.180 | 99.5% |
| v3_shapebound | 12.160 | 99.3% |
| v4_erased | 12.163 | 99.4% |
| v5_handletable | 10.007 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 5298ns | 5298ns | base |
| v1_snapshot | 5513ns | 5513ns | +4.05% |
| v2_decomposed | 5296ns | 5296ns | -0.05% |
| v3_shapebound | 5221ns | 5221ns | -1.46% |
| v4_erased | 5158ns | 5158ns | -2.65% |
| v5_handletable | 5584ns | 5584ns | +5.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 1352ns | base | --- | [1345, 1635] | --- | --- | --- | --- |
| v1_snapshot | 1639ns | +4.2ns (+0.3%) | [+2, +13]ns | [1637, 1639] | YES | 0.0111 | 0.0045 | 0 |
| v2_decomposed | 1345ns | no significant difference | [-4, +3]ns | [1342, 1635] | no | 0.9273 | 0.9273 | 0 |
| v3_shapebound | 1347ns | +2.4ns (+0.2%) | [+0, +5]ns | [1347, 1350] | YES (adj: no) | 0.0721 | 0.0433 | 1 |
| v4_erased | 1347ns | no significant difference | [-5, +4]ns | [1345, 1349] | no | 0.9273 | 0.7843 | 0 |
| v5_handletable | 1637ns | +4.8ns (+0.4%) | [+1, +13]ns | [1637, 1638] | YES | 0.0015 | 0.0003 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1640ns | -18.2% | -18.3% | +0.3% | -18.0% | -0.0% |
| 1 | 2 | 1640ns | -0.1% | -18.2% | +0.4% | -18.0% | -18.2% |
| 1 | 3 | 1342ns | +0.0% | +21.9% | +0.1% | +0.1% | +22.2% |
| 1 | 4 | 1636ns | +0.2% | -18.1% | -17.7% | -17.7% | +0.1% |
| 1 | 5 | 1340ns | +22.8% | +22.5% | +0.3% | +0.3% | +0.1% |
| 2 | 1 | 1639ns | -18.1% | -0.1% | -17.9% | +0.3% | -18.3% |
| 2 | 2 | 1640ns | -0.1% | -18.3% | -18.1% | -18.1% | -0.1% |
| 2 | 3 | 1343ns | +0.0% | -0.1% | +0.3% | +22.6% | +22.0% |
| 2 | 4 | 1640ns | -18.1% | -18.2% | -17.9% | -18.0% | +0.1% |
| 2 | 5 | 1342ns | +22.3% | +22.0% | +0.4% | +0.1% | +22.0% |
| 3 | 1 | 1342ns | +22.3% | +22.2% | +23.0% | +0.1% | +22.0% |
| 3 | 2 | 1636ns | +0.3% | +0.3% | +0.5% | +0.6% | +0.1% |
| 3 | 3 | 1343ns | +22.3% | +22.1% | +0.1% | +22.6% | -0.1% |
| 3 | 4 | 1339ns | +22.6% | +0.1% | +22.7% | +22.7% | +22.7% |
| 3 | 5 | 1340ns | +22.4% | +0.1% | +0.3% | +0.3% | +22.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.266 | moderate- |
| v1_snapshot | -0.126 | ok |
| v2_decomposed | 0.142 | ok |
| v3_shapebound | 0.166 | ok |
| v4_erased | 0.064 | ok |
| v5_handletable | -0.046 | ok |

**Consistency summary:**

- **v1_snapshot**: won 3/15, lost 8/15
- **v2_decomposed**: won 6/15, lost 7/15
- **v3_shapebound**: won 4/15, lost 10/15
- **v4_erased**: won 5/15, lost 9/15
- **v5_handletable**: won 3/15, lost 8/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 35111.6ns | 1481.7ns | 2369.6% | HIGH |
| v1_snapshot | 36524.7ns | 1542.7ns | 2367.6% | HIGH |
| v2_decomposed | 35084.6ns | 1480.3ns | 2370.1% | HIGH |
| v3_shapebound | 34557.2ns | 1463.6ns | 2361.1% | HIGH |
| v4_erased | 34171.5ns | 1445.0ns | 2364.8% | HIGH |
| v5_handletable | 37057.0ns | 1561.5ns | 2373.2% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 1338.8-1644.6 ns)
   1338.8 |########################################
   1354.1 |##
   1369.4 |
   1384.7 |
   1400.0 |
   1415.3 |
   1430.6 |
   1445.9 |
   1461.1 |
   1476.4 |
   1491.7 |
   1507.0 |
   1522.3 |
   1537.6 |
   1552.9 |
   1568.2 |
   1583.5 |
   1598.7 |
   1614.0 |
   1629.3 |#######################################
  (12 below, 8 above range)

v1_snapshot (n=120, range 1340.6-1650.4 ns)
   1340.6 |###############
   1356.1 |
   1371.6 |
   1387.1 |
   1402.6 |
   1418.0 |
   1433.5 |
   1449.0 |
   1464.5 |
   1480.0 |
   1495.5 |
   1511.0 |
   1526.5 |
   1542.0 |
   1557.5 |
   1573.0 |
   1588.5 |
   1604.0 |
   1619.4 |#
   1634.9 |########################################
  (12 below, 6 above range)

v2_decomposed (n=120, range 1338.5-1643.4 ns)
   1338.5 |########################################
   1353.8 |
   1369.0 |
   1384.3 |
   1399.5 |
   1414.7 |
   1430.0 |
   1445.2 |
   1460.5 |
   1475.7 |
   1491.0 |
   1506.2 |
   1521.4 |
   1536.7 |
   1551.9 |
   1567.2 |
   1582.4 |
   1597.6 |
   1612.9 |
   1628.1 |##################################
  (12 below, 11 above range)

v3_shapebound (n=120, range 1342.1-1736.3 ns)
   1342.1 |########################################
   1361.8 |#
   1381.5 |
   1401.2 |
   1421.0 |
   1440.7 |
   1460.4 |
   1480.1 |
   1499.8 |
   1519.5 |
   1539.2 |
   1558.9 |
   1578.7 |
   1598.4 |
   1618.1 |
   1637.8 |#####################
   1657.5 |
   1677.2 |
   1696.9 |
   1716.6 |
  (8 below, 1 above range)

v4_erased (n=120, range 1340.4-1649.5 ns)
   1340.4 |########################################
   1355.9 |
   1371.3 |
   1386.8 |
   1402.2 |
   1417.7 |
   1433.2 |
   1448.6 |
   1464.1 |
   1479.5 |
   1495.0 |
   1510.4 |
   1525.9 |
   1541.3 |
   1556.8 |
   1572.3 |
   1587.7 |
   1603.2 |
   1618.6 |
   1634.1 |#################
  (7 below, 9 above range)

v5_handletable (n=120, range 1339.9-1650.2 ns)
   1339.9 |#######
   1355.5 |
   1371.0 |
   1386.5 |
   1402.0 |
   1417.5 |
   1433.0 |
   1448.5 |
   1464.0 |
   1479.6 |
   1495.1 |
   1510.6 |
   1526.1 |
   1541.6 |
   1557.1 |
   1572.6 |
   1588.2 |
   1603.7 |
   1619.2 |
   1634.7 |########################################
  (15 below, 4 above range)

```

## Diagnostics

- **v0_blob**: bridge=2370.9% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=2370.6% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=2368.5% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=2358.8% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=2359.3% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=2374.7% of algo (FFI overhead may distort results)

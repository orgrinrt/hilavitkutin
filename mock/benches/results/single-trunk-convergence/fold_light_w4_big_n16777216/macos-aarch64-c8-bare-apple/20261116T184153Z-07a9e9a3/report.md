# fold_light, far above cache, W = 4

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail shows alternating (throttle bounce) (autocorr -0.60)

head_tail's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### kernel_nway is inconsistent: worst-20% is 1.8x its best-20%

kernel_nway's best 20% of batches run at 1.21 ms but its worst 20% at 2.17 ms (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: nway** at 1385583.4 ns median (-9.9% vs baseline)
- Spread: 1.56x (fastest 1385583.4 ns, slowest 2157411.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 4 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 1648451ns | 1542104ns | 1238615ns | 1495938ns | 2082141ns | base |
| head_tail_dyn | 2141927ns | 2160344ns | 1892823ns | 2099385ns | 2330292ns | +29.94% |
| kernel_nway | 1634226ns | 1443255ns | 1218115ns | 1411837ns | 2175865ns | -0.86% |
| kernel_seq | 1769809ns | 1648802ns | 1520104ns | 1613590ns | 2128990ns | +7.36% |
| nway | 1483061ns | 1389828ns | 1172302ns | 1319684ns | 1883505ns | -10.03% |
| sequential | 1776733ns | 1741849ns | 1652625ns | 1719146ns | 1925167ns | +7.78% |
| std_parallel | 1578262ns | 1524641ns | 1294667ns | 1457101ns | 1901802ns | -4.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 1644589ns | 1235240ns | 2078807ns | base | 10.201 |
| head_tail_dyn | 2138514ns | 1889427ns | 2325938ns | +30.03% | 7.845 |
| kernel_nway | 1630106ns | 1213323ns | 2172120ns | -0.88% | 10.292 |
| kernel_seq | 1767365ns | 1518125ns | 2125839ns | +7.47% | 9.493 |
| nway | 1479729ns | 1169646ns | 1880573ns | -10.02% | 11.338 |
| sequential | 1774670ns | 1650938ns | 1923109ns | +7.91% | 9.454 |
| std_parallel | 1575943ns | 1292688ns | 1899318ns | -4.17% | 10.646 |

## Performance model

- Peak throughput: **14.344 Gops/s** (nway; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 10.907 | 76.0% |
| head_tail_dyn | 7.777 | 54.2% |
| kernel_nway | 11.660 | 81.3% |
| kernel_seq | 10.190 | 71.0% |
| nway | 12.108 | 84.4% |
| sequential | 9.643 | 67.2% |
| std_parallel | 11.022 | 76.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 1648451ns | 1648451ns | base |
| head_tail_dyn | 2141927ns | 2141927ns | +29.94% |
| kernel_nway | 1634226ns | 1634226ns | -0.86% |
| kernel_seq | 1769809ns | 1769809ns | +7.36% |
| nway | 1483061ns | 1483061ns | -10.03% |
| sequential | 1776733ns | 1776733ns | +7.78% |
| std_parallel | 1578262ns | 1578262ns | -4.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 1538271ns | base | --- | [1316688, 2078807] | --- | --- | --- | --- |
| head_tail_dyn | 2157411ns | no significant difference | [-146615, +1009250]ns | [1932193, 2325938] | no | 1.0000 | 0.6875 | 0 |
| kernel_nway | 1438927ns | no significant difference | [-438370, +498422]ns | [1279271, 2172120] | no | 1.0000 | 0.6875 | 0 |
| kernel_seq | 1646500ns | no significant difference | [-340969, +542073]ns | [1529755, 2125839] | no | 1.0000 | 0.6875 | 0 |
| nway | 1385583ns | no significant difference | [-876370, +458151]ns | [1173031, 1880573] | no | 1.0000 | 1.0000 | 0 |
| sequential | 1739755ns | no significant difference | [-245672, +495953]ns | [1661146, 1923109] | no | 1.0000 | 0.6875 | 0 |
| std_parallel | 1522146ns | no significant difference | [-684271, +467516]ns | [1306365, 1899318] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 1701854ns | +24.8% | -6.1% | -8.7% | -18.3% | +1.2% | -2.7% |
| 1 | 2 | 1388214ns | +62.3% | +35.9% | +39.0% | -2.3% | +26.1% | +21.8% |
| 1 | 3 | 1843698ns | +10.5% | -23.8% | -1.4% | -8.2% | +0.4% | -25.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.605 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.561 | HIGH- (thermal bounce) |
| kernel_nway | -0.561 | HIGH- (thermal bounce) |
| kernel_seq | -0.356 | moderate- |
| nway | -0.222 | moderate- |
| sequential | -0.062 | ok |
| std_parallel | -0.227 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 2/3, lost 1/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 1448010.4ns | 1644588.6ns | 88.0% | HIGH |
| head_tail_dyn | 2183597.2ns | 2138513.9ns | 102.1% | HIGH |
| kernel_nway | 1639888.9ns | 1630105.9ns | 100.6% | HIGH |
| kernel_seq | 1800847.2ns | 1767364.6ns | 101.9% | HIGH |
| nway | 1451472.2ns | 1479729.2ns | 98.1% | HIGH |
| sequential | 2035402.8ns | 1774670.1ns | 114.7% | HIGH |
| std_parallel | 1660713.6ns | 1575942.7ns | 105.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 1235239.6-2078807.3 ns)
  1235239.6 |####################
  1277418.0 |
  1319596.4 |
  1361774.8 |####################
  1403953.1 |
  1446131.5 |
  1488309.9 |
  1530488.3 |########################################
  1572666.7 |
  1614845.1 |
  1657023.5 |
  1699201.8 |
  1741380.2 |
  1783558.6 |
  1825737.0 |
  1867915.4 |
  1910093.8 |
  1952272.1 |
  1994450.5 |####################
  2036628.9 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 1889427.1-2325937.5 ns)
  1889427.1 |########################################
  1911252.6 |
  1933078.1 |
  1954903.7 |########################################
  1976729.2 |
  1998554.7 |
  2020380.2 |
  2042205.7 |
  2064031.3 |
  2085856.8 |
  2107682.3 |########################################
  2129507.8 |
  2151333.3 |
  2173158.9 |########################################
  2194984.4 |
  2216809.9 |
  2238635.4 |
  2260460.9 |########################################
  2282286.5 |
  2304112.0 |
  (0 below, 1 above range)

kernel_nway (n=6, range 1213322.9-2172119.8 ns)
  1213322.9 |########################################
  1261262.7 |
  1309202.6 |########################################
  1357142.4 |
  1405082.3 |########################################
  1453022.1 |########################################
  1500962.0 |
  1548901.8 |
  1596841.7 |
  1644781.5 |
  1692721.3 |
  1740661.2 |
  1788601.0 |
  1836540.9 |
  1884480.7 |
  1932420.6 |
  1980360.4 |########################################
  2028300.3 |
  2076240.1 |
  2124180.0 |
  (0 below, 1 above range)

kernel_seq (n=6, range 1518125.0-2125838.5 ns)
  1518125.0 |########################################
  1548510.7 |####################
  1578896.4 |
  1609282.0 |
  1639667.7 |
  1670053.4 |
  1700439.1 |####################
  1730824.7 |
  1761210.4 |
  1791596.1 |
  1821981.8 |
  1852367.5 |
  1882753.1 |####################
  1913138.8 |
  1943524.5 |
  1973910.2 |
  2004295.8 |
  2034681.5 |
  2065067.2 |
  2095452.9 |
  (0 below, 1 above range)

nway (n=6, range 1169645.8-1880572.9 ns)
  1169645.8 |########################################
  1205192.2 |####################
  1240738.5 |
  1276284.9 |
  1311831.2 |
  1347377.6 |
  1382923.9 |
  1418470.3 |
  1454016.6 |
  1489563.0 |
  1525109.4 |####################
  1560655.7 |
  1596202.1 |####################
  1631748.4 |
  1667294.8 |
  1702841.1 |
  1738387.5 |
  1773933.8 |
  1809480.2 |
  1845026.5 |
  (0 below, 1 above range)

sequential (n=6, range 1650937.5-1923109.4 ns)
  1650937.5 |########################################
  1664546.1 |########################################
  1678154.7 |########################################
  1691763.3 |
  1705371.9 |
  1718980.5 |
  1732589.1 |
  1746197.6 |
  1759806.2 |
  1773414.8 |
  1787023.4 |########################################
  1800632.0 |
  1814240.6 |
  1827849.2 |########################################
  1841457.8 |
  1855066.4 |
  1868675.0 |
  1882283.6 |
  1895892.2 |
  1909500.8 |
  (0 below, 1 above range)

std_parallel (n=6, range 1292687.5-1899317.7 ns)
  1292687.5 |########################################
  1323019.0 |
  1353350.5 |
  1383682.0 |
  1414013.5 |
  1444345.1 |####################
  1474676.6 |
  1505008.1 |
  1535339.6 |
  1565671.1 |####################
  1596002.6 |
  1626334.1 |
  1656665.6 |
  1686997.1 |
  1717328.6 |
  1747660.2 |
  1777991.7 |####################
  1808323.2 |
  1838654.7 |
  1868986.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=95.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=96.4% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=24.9% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=105.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=95.4% of algo (FFI overhead may distort results)
- **nway**: CV=23.4% (high variance, measurements may be unstable)
- **nway**: bridge=96.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=119.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=111.8% of algo (FFI overhead may distort results)

# fold_light, far above cache, W = 1

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (head_tail_dyn, kernel_nway) are a dead heat (<1%)

head_tail_dyn (1.43 ms) and kernel_nway (1.43 ms) differ by 0.04%, inside the noise, even though the wider field spreads 6.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.67)

kernel_nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: head_tail_dyn** at 1427791.6 ns median (-0.6% vs baseline)
- Spread: 1.06x (fastest 1427791.6 ns, slowest 1520151.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 1 |
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
| head_tail | 1484142ns | 1437672ns | 1428333ns | 1434854ns | 1585979ns | base |
| head_tail_dyn | 1434654ns | 1429500ns | 1427375ns | 1429052ns | 1446698ns | -3.33% |
| kernel_nway | 1443602ns | 1430312ns | 1427760ns | 1429854ns | 1472146ns | -2.73% |
| kernel_seq | 1430411ns | 1431104ns | 1427448ns | 1430083ns | 1432385ns | -3.62% |
| nway | 1527755ns | 1503156ns | 1440896ns | 1483941ns | 1636906ns | +2.94% |
| sequential | 1439734ns | 1436516ns | 1431625ns | 1435653ns | 1449912ns | -2.99% |
| std_parallel | 1554628ns | 1521854ns | 1464281ns | 1505469ns | 1673542ns | +4.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 1482316ns | 1426927ns | 1583417ns | base | 11.318 |
| head_tail_dyn | 1433092ns | 1426156ns | 1445318ns | -3.32% | 11.707 |
| kernel_nway | 1441913ns | 1425802ns | 1470562ns | -2.73% | 11.635 |
| kernel_seq | 1428578ns | 1425448ns | 1430917ns | -3.63% | 11.744 |
| nway | 1525462ns | 1437896ns | 1634786ns | +2.91% | 10.998 |
| sequential | 1438276ns | 1430271ns | 1448516ns | -2.97% | 11.665 |
| std_parallel | 1552991ns | 1463135ns | 1671620ns | +4.77% | 10.803 |

## Performance model

- Peak throughput: **11.770 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 11.682 | 99.3% |
| head_tail_dyn | 11.750 | 99.8% |
| kernel_nway | 11.745 | 99.8% |
| kernel_seq | 11.740 | 99.8% |
| nway | 11.176 | 95.0% |
| sequential | 11.692 | 99.3% |
| std_parallel | 11.037 | 93.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 1484142ns | 1484142ns | base |
| head_tail_dyn | 1434654ns | 1434654ns | -3.33% |
| kernel_nway | 1443602ns | 1443602ns | -2.73% |
| kernel_seq | 1430411ns | 1430411ns | -3.62% |
| nway | 1527755ns | 1527755ns | +2.94% |
| sequential | 1439734ns | 1439734ns | -2.99% |
| std_parallel | 1554628ns | 1554628ns | +4.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 1436214ns | base | --- | [1427318, 1583417] | --- | --- | --- | --- |
| head_tail_dyn | 1427792ns | no significant difference | [-138870, +969]ns | [1426167, 1445318] | no | 0.6563 | 0.2188 | 0 |
| kernel_nway | 1428422ns | no significant difference | [-112854, +328]ns | [1426755, 1470562] | no | 0.6563 | 0.2188 | 0 |
| kernel_seq | 1429010ns | no significant difference | [-156411, +2286]ns | [1425807, 1430917] | no | 0.8250 | 0.6875 | 0 |
| nway | 1501219ns | no significant difference | [-143036, +206922]ns | [1440380, 1634786] | no | 0.8250 | 0.6875 | 0 |
| sequential | 1434979ns | no significant difference | [-134901, +7661]ns | [1431333, 1448516] | no | 1.0000 | 1.0000 | 0 |
| std_parallel | 1520151ns | no significant difference | [-65344, +235563]ns | [1467203, 1671620] | no | 0.8250 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 1436057ns | -0.5% | -0.5% | -0.4% | +6.8% | -0.2% | +16.4% |
| 1 | 2 | 1583417ns | -8.8% | -7.1% | -9.9% | -9.0% | -8.5% | -4.1% |
| 1 | 3 | 1427474ns | -0.1% | -0.1% | +0.1% | +12.3% | +0.4% | +2.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.665 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.657 | HIGH- (thermal bounce) |
| kernel_nway | -0.666 | HIGH- (thermal bounce) |
| kernel_seq | -0.477 | moderate- |
| nway | -0.545 | HIGH- (thermal bounce) |
| sequential | -0.663 | HIGH- (thermal bounce) |
| std_parallel | -0.055 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 0/3
- **kernel_nway**: won 2/3, lost 0/3
- **kernel_seq**: won 2/3, lost 0/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 1486260.4ns | 1482315.9ns | 100.3% | HIGH |
| head_tail_dyn | 1437578.2ns | 1433092.0ns | 100.3% | HIGH |
| kernel_nway | 1443618.1ns | 1441913.2ns | 100.1% | HIGH |
| kernel_seq | 1434217.0ns | 1428578.1ns | 100.4% | HIGH |
| nway | 1522906.2ns | 1525461.8ns | 99.8% | HIGH |
| sequential | 1437434.0ns | 1438276.1ns | 99.9% | HIGH |
| std_parallel | 1538225.7ns | 1552991.3ns | 99.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 1426927.1-1583416.6 ns)
  1426927.1 |########################################
  1434751.6 |
  1442576.1 |#############
  1450400.5 |
  1458225.0 |
  1466049.5 |
  1473874.0 |
  1481698.4 |
  1489522.9 |
  1497347.4 |
  1505171.9 |
  1512996.4 |
  1520820.8 |
  1528645.3 |
  1536469.8 |
  1544294.3 |
  1552118.7 |
  1559943.2 |
  1567767.7 |
  1575592.2 |#############
  (0 below, 1 above range)

head_tail_dyn (n=6, range 1426156.2-1445317.8 ns)
  1426156.2 |########################################
  1427114.3 |
  1428072.4 |#############
  1429030.4 |
  1429988.5 |#############
  1430946.6 |
  1431904.7 |
  1432862.7 |
  1433820.8 |
  1434778.9 |
  1435737.0 |
  1436695.1 |
  1437653.1 |
  1438611.2 |
  1439569.3 |
  1440527.4 |
  1441485.4 |
  1442443.5 |
  1443401.6 |
  1444359.7 |
  (0 below, 1 above range)

kernel_nway (n=6, range 1425802.1-1470562.5 ns)
  1425802.1 |########################################
  1428040.1 |########################################
  1430278.1 |
  1432516.2 |
  1434754.2 |####################
  1436992.2 |
  1439230.2 |
  1441468.2 |
  1443706.3 |
  1445944.3 |
  1448182.3 |
  1450420.3 |
  1452658.3 |
  1454896.4 |
  1457134.4 |
  1459372.4 |
  1461610.4 |
  1463848.4 |
  1466086.5 |
  1468324.5 |
  (0 below, 1 above range)

kernel_seq (n=6, range 1425447.9-1430916.7 ns)
  1425447.9 |########################################
  1425721.3 |
  1425994.8 |########################################
  1426268.2 |
  1426541.7 |
  1426815.1 |
  1427088.5 |
  1427362.0 |
  1427635.4 |
  1427908.9 |
  1428182.3 |
  1428455.7 |########################################
  1428729.2 |
  1429002.6 |
  1429276.1 |########################################
  1429549.5 |
  1429822.9 |
  1430096.4 |
  1430369.8 |
  1430643.3 |########################################
  (0 below, 1 above range)

nway (n=6, range 1437895.8-1634786.4 ns)
  1437895.8 |########################################
  1447740.3 |
  1457584.9 |
  1467429.4 |
  1477273.9 |####################
  1487118.5 |
  1496963.0 |
  1506807.5 |
  1516652.1 |####################
  1526496.6 |
  1536341.1 |####################
  1546185.7 |
  1556030.2 |
  1565874.7 |
  1575719.3 |
  1585563.8 |
  1595408.3 |
  1605252.9 |
  1615097.4 |
  1624941.9 |
  (0 below, 1 above range)

sequential (n=6, range 1430270.8-1448515.6 ns)
  1430270.8 |####################
  1431183.0 |
  1432095.3 |########################################
  1433007.5 |
  1433919.8 |
  1434832.0 |
  1435744.3 |
  1436656.5 |####################
  1437568.7 |
  1438481.0 |
  1439393.2 |
  1440305.5 |
  1441217.7 |
  1442130.0 |
  1443042.2 |
  1443954.4 |####################
  1444866.7 |
  1445778.9 |
  1446691.2 |
  1447603.4 |
  (0 below, 1 above range)

std_parallel (n=6, range 1463135.4-1671619.8 ns)
  1463135.4 |########################################
  1473559.6 |####################
  1483983.8 |
  1494408.1 |
  1504832.3 |
  1515256.5 |
  1525680.7 |
  1536104.9 |
  1546529.2 |
  1556953.4 |####################
  1567377.6 |####################
  1577801.8 |
  1588226.0 |
  1598650.3 |
  1609074.5 |
  1619498.7 |
  1629922.9 |
  1640347.1 |
  1650771.4 |
  1661195.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=100.0% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.3% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=100.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.3% of algo (FFI overhead may distort results)
- **nway**: bridge=100.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=99.4% of algo (FFI overhead may distort results)

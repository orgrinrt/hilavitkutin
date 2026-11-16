# fold_light, far above cache, W = 2

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 19% faster than the next best (std_parallel)

kernel_nway (1.38 ms) leads std_parallel (1.64 ms) by 19%, a clear separation rather than a photo finish. CV 26.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.67)

kernel_seq's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### nway is inconsistent: worst-20% is 1.9x its best-20%

nway's best 20% of batches run at 1.39 ms but its worst 20% at 2.61 ms (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 1381500.0 ns median (-16.2% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.43x (fastest 1381500.0 ns, slowest 1980599.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 2 |
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
| head_tail | 1616049ns | 1653083ns | 1224917ns | 1553385ns | 1905609ns | base |
| head_tail_dyn | 2044188ns | 1984188ns | 1938833ns | 1980792ns | 2191958ns | +26.49% |
| kernel_nway | 1546720ns | 1385807ns | 1212656ns | 1330319ns | 2038354ns | -4.29% |
| kernel_seq | 1768998ns | 1706609ns | 1562802ns | 1688087ns | 1993464ns | +9.46% |
| nway | 1939113ns | 1755036ns | 1396354ns | 1672132ns | 2610964ns | +19.99% |
| sequential | 1931361ns | 1652698ns | 1497333ns | 1622299ns | 2611969ns | +19.51% |
| std_parallel | 1709318ns | 1640359ns | 1403406ns | 1572490ns | 2067516ns | +5.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 1612040ns | 1221062ns | 1901630ns | base | 10.407 |
| head_tail_dyn | 2040300ns | 1934250ns | 2187536ns | +26.57% | 8.223 |
| kernel_nway | 1542878ns | 1209198ns | 2035234ns | -4.29% | 10.874 |
| kernel_seq | 1766462ns | 1560969ns | 1989792ns | +9.58% | 9.498 |
| nway | 1935075ns | 1392344ns | 2606141ns | +20.04% | 8.670 |
| sequential | 1928958ns | 1494656ns | 2609630ns | +19.66% | 8.698 |
| std_parallel | 1706859ns | 1401531ns | 2064776ns | +5.88% | 9.829 |

## Performance model

- Peak throughput: **13.875 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 10.176 | 73.3% |
| head_tail_dyn | 8.471 | 61.1% |
| kernel_nway | 12.144 | 87.5% |
| kernel_seq | 9.841 | 70.9% |
| nway | 9.581 | 69.1% |
| sequential | 10.168 | 73.3% |
| std_parallel | 10.243 | 73.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 1616049ns | 1616049ns | base |
| head_tail_dyn | 2044188ns | 2044188ns | +26.49% |
| kernel_nway | 1546720ns | 1546720ns | -4.29% |
| kernel_seq | 1768998ns | 1768998ns | +9.46% |
| nway | 1939113ns | 1939113ns | +19.99% |
| sequential | 1931361ns | 1931361ns | +19.51% |
| std_parallel | 1709318ns | 1709318ns | +5.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 1648734ns | base | --- | [1285755, 1901630] | --- | --- | --- | --- |
| head_tail_dyn | 1980599ns | +448812.6ns (+27.2%) | [+74958, +761010]ns | [1952766, 2187536] | YES (adj: no) | 0.1875 | 0.0313 | 0 |
| kernel_nway | 1381500ns | no significant difference | [-669578, +513859]ns | [1211901, 2035234] | no | 0.8250 | 0.6875 | 0 |
| kernel_seq | 1704760ns | no significant difference | [-180568, +595703]ns | [1604833, 1989792] | no | 1.0000 | 1.0000 | 0 |
| nway | 1751151ns | no significant difference | [-205250, +1100078]ns | [1447932, 2606141] | no | 0.8250 | 0.6875 | 0 |
| sequential | 1650031ns | no significant difference | [-231448, +940745]ns | [1527214, 2609630] | no | 0.8250 | 0.6875 | 0 |
| std_parallel | 1637844ns | no significant difference | [-386938, +543052]ns | [1417958, 2064776] | no | 0.8250 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 1541875ns | +26.6% | +5.5% | +9.7% | +67.3% | +31.4% | +17.3% |
| 1 | 2 | 1648734ns | +27.2% | -21.6% | +16.1% | +1.2% | +28.4% | +14.9% |
| 1 | 3 | 1645510ns | +25.8% | +3.8% | +2.9% | -5.3% | -0.1% | -13.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.182 | ok |
| head_tail_dyn | -0.276 | moderate- |
| kernel_nway | -0.643 | HIGH- (thermal bounce) |
| kernel_seq | -0.667 | HIGH- (thermal bounce) |
| nway | -0.113 | ok |
| sequential | -0.281 | moderate- |
| std_parallel | -0.273 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 0/3, lost 2/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 1749118.0ns | 1612039.9ns | 108.5% | HIGH |
| head_tail_dyn | 2108677.1ns | 2040300.3ns | 103.4% | HIGH |
| kernel_nway | 1431347.2ns | 1542878.5ns | 92.8% | HIGH |
| kernel_seq | 1768880.2ns | 1766461.8ns | 100.1% | HIGH |
| nway | 2230663.2ns | 1935074.6ns | 115.3% | HIGH |
| sequential | 1690369.8ns | 1928958.3ns | 87.6% | HIGH |
| std_parallel | 1556435.8ns | 1706859.4ns | 91.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 1221062.5-1901630.2 ns)
  1221062.5 |########################################
  1255090.9 |
  1289119.3 |
  1323147.7 |########################################
  1357176.0 |
  1391204.4 |
  1425232.8 |
  1459261.2 |########################################
  1493289.6 |
  1527318.0 |
  1561346.4 |
  1595374.7 |
  1629403.1 |
  1663431.5 |
  1697459.9 |
  1731488.3 |
  1765516.7 |
  1799545.0 |########################################
  1833573.4 |########################################
  1867601.8 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 1934250.0-2187536.5 ns)
  1934250.0 |####################
  1946914.3 |
  1959578.6 |####################
  1972243.0 |########################################
  1984907.3 |
  1997571.6 |
  2010235.9 |
  2022900.3 |
  2035564.6 |
  2048228.9 |
  2060893.2 |
  2073557.5 |
  2086221.9 |
  2098886.2 |
  2111550.5 |
  2124214.8 |
  2136879.2 |
  2149543.5 |####################
  2162207.8 |
  2174872.1 |
  (0 below, 1 above range)

kernel_nway (n=6, range 1209197.9-2035234.4 ns)
  1209197.9 |########################################
  1250499.7 |
  1291801.5 |
  1333103.4 |####################
  1374405.2 |####################
  1415707.0 |
  1457008.8 |
  1498310.7 |
  1539612.5 |
  1580914.3 |
  1622216.1 |
  1663518.0 |
  1704819.8 |
  1746121.6 |
  1787423.4 |
  1828725.3 |####################
  1870027.1 |
  1911328.9 |
  1952630.8 |
  1993932.6 |
  (0 below, 1 above range)

kernel_seq (n=6, range 1560968.8-1989791.6 ns)
  1560968.8 |########################################
  1582409.9 |
  1603851.1 |
  1625292.2 |
  1646733.4 |########################################
  1668174.5 |########################################
  1689615.7 |
  1711056.8 |
  1732497.9 |########################################
  1753939.1 |
  1775380.2 |
  1796821.4 |
  1818262.5 |########################################
  1839703.7 |
  1861144.8 |
  1882585.9 |
  1904027.1 |
  1925468.2 |
  1946909.4 |
  1968350.5 |
  (0 below, 1 above range)

nway (n=6, range 1392343.8-2606140.6 ns)
  1392343.8 |########################################
  1453033.6 |########################################
  1513723.5 |
  1574413.3 |########################################
  1635103.2 |
  1695793.0 |
  1756482.8 |
  1817172.7 |
  1877862.5 |########################################
  1938552.4 |########################################
  1999242.2 |
  2059932.0 |
  2120621.9 |
  2181311.7 |
  2242001.6 |
  2302691.4 |
  2363381.2 |
  2424071.1 |
  2484760.9 |
  2545450.8 |
  (0 below, 1 above range)

sequential (n=6, range 1494656.2-2609630.2 ns)
  1494656.2 |####################
  1550404.9 |########################################
  1606153.6 |
  1661902.3 |
  1717651.0 |####################
  1773399.7 |
  1829148.4 |
  1884897.1 |
  1940645.8 |
  1996394.5 |
  2052143.2 |
  2107891.9 |
  2163640.6 |
  2219389.3 |
  2275138.0 |
  2330886.7 |
  2386635.4 |
  2442384.1 |
  2498132.8 |
  2553881.5 |####################
  (0 below, 1 above range)

std_parallel (n=6, range 1401531.2-2064776.1 ns)
  1401531.2 |########################################
  1434693.4 |
  1467855.7 |
  1501017.9 |
  1534180.2 |
  1567342.4 |####################
  1600504.7 |
  1633666.9 |
  1666829.1 |####################
  1699991.4 |
  1733153.6 |
  1766315.9 |
  1799478.1 |
  1832640.4 |
  1865802.6 |
  1898964.8 |
  1932127.1 |
  1965289.3 |
  1998451.6 |####################
  2031613.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=105.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=106.1% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=23.9% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=91.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=102.0% of algo (FFI overhead may distort results)
- **nway**: CV=32.4% (high variance, measurements may be unstable)
- **nway**: bridge=118.5% of algo (FFI overhead may distort results)
- **sequential**: CV=25.3% (high variance, measurements may be unstable)
- **sequential**: bridge=99.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=96.8% of algo (FFI overhead may distort results)

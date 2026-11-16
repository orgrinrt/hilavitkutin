# fold_light, far above cache, W = 1

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail_dyn is fastest but the noisiest (CV 25.8%)

head_tail_dyn wins on median (1.66 ms) yet has the highest variance (CV 25.8%), while nway is the steadiest (CV 11.8%, 1.72 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.60)

kernel_seq's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader head_tail_dyn vs stability leader nway (+4% speed for 2.2x steadier)

head_tail_dyn is fastest (1.66 ms, CV 25.8%); nway gives up 3.6% median for 2.2x lower variance (CV 11.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### kernel_nway is inconsistent: worst-20% is 1.6x its best-20%

kernel_nway's best 20% of batches run at 1.46 ms but its worst 20% at 2.29 ms (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 1658927.1 ns median (-4.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.28x (fastest 1658927.1 ns, slowest 2116791.6 ns)

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
| head_tail | 1806359ns | 1730375ns | 1510219ns | 1667108ns | 2163307ns | base |
| head_tail_dyn | 1836769ns | 1660964ns | 1512375ns | 1626260ns | 2314729ns | +1.68% |
| kernel_nway | 1981450ns | 2004995ns | 1462740ns | 1947816ns | 2291255ns | +9.69% |
| kernel_seq | 1793832ns | 1793635ns | 1509854ns | 1716229ns | 2052224ns | -0.69% |
| nway | 1739007ns | 1720672ns | 1495938ns | 1676910ns | 1953688ns | -3.73% |
| sequential | 1847910ns | 1727083ns | 1523469ns | 1660066ns | 2291896ns | +2.30% |
| std_parallel | 2083453ns | 2119135ns | 1668323ns | 1981285ns | 2444271ns | +15.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 1803828ns | 1508615ns | 2160391ns | base | 9.301 |
| head_tail_dyn | 1834295ns | 1510667ns | 2311552ns | +1.69% | 9.146 |
| kernel_nway | 1978839ns | 1460823ns | 2288167ns | +9.70% | 8.478 |
| kernel_seq | 1791448ns | 1507854ns | 2049141ns | -0.69% | 9.365 |
| nway | 1737031ns | 1494396ns | 1951620ns | -3.70% | 9.659 |
| sequential | 1845637ns | 1521698ns | 2288984ns | +2.32% | 9.090 |
| std_parallel | 2081155ns | 1666365ns | 2441766ns | +15.37% | 8.061 |

## Performance model

- Peak throughput: **11.485 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 9.711 | 84.6% |
| head_tail_dyn | 10.113 | 88.1% |
| kernel_nway | 8.379 | 73.0% |
| kernel_seq | 9.366 | 81.5% |
| nway | 9.762 | 85.0% |
| sequential | 9.724 | 84.7% |
| std_parallel | 7.926 | 69.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 1806359ns | 1806359ns | base |
| head_tail_dyn | 1836769ns | 1836769ns | +1.68% |
| kernel_nway | 1981450ns | 1981450ns | +9.69% |
| kernel_seq | 1793832ns | 1793832ns | -0.69% |
| nway | 1739007ns | 1739007ns | -3.73% |
| sequential | 1847910ns | 1847910ns | +2.30% |
| std_parallel | 2083453ns | 2083453ns | +15.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 1727630ns | base | --- | [1523464, 2160391] | --- | --- | --- | --- |
| head_tail_dyn | 1658927ns | no significant difference | [-501464, +727750]ns | [1532406, 2311552] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 2002255ns | no significant difference | [-3875, +407932]ns | [1646094, 2288167] | no | 0.6563 | 0.2188 | 0 |
| kernel_seq | 1791339ns | no significant difference | [-447229, +324583]ns | [1533865, 2049141] | no | 1.0000 | 0.6875 | 0 |
| nway | 1718599ns | no significant difference | [-524094, +362786]ns | [1540875, 1951620] | no | 1.0000 | 1.0000 | 0 |
| sequential | 1725286ns | no significant difference | [-568797, +765521]ns | [1522641, 2288984] | no | 1.0000 | 0.6875 | 0 |
| std_parallel | 2116792ns | +223624.9ns (+12.9%) | [+5984, +602370]ns | [1684906, 2441766] | YES (adj: no) | 0.6563 | 0.2188 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 1905906ns | -7.8% | +1.1% | -8.9% | -9.8% | +15.5% | +8.8% |
| 1 | 2 | 1906927ns | -16.6% | +5.0% | +7.5% | -19.2% | -13.1% | +7.6% |
| 1 | 3 | 1598651ns | +34.8% | +25.5% | -0.6% | +22.1% | +5.0% | +32.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.168 | ok |
| head_tail_dyn | -0.352 | moderate- |
| kernel_nway | -0.139 | ok |
| kernel_seq | -0.601 | HIGH- (thermal bounce) |
| nway | -0.453 | moderate- |
| sequential | -0.187 | ok |
| std_parallel | -0.385 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 2/3, lost 1/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 1769295.2ns | 1803828.1ns | 98.1% | HIGH |
| head_tail_dyn | 1810875.0ns | 1834295.2ns | 98.7% | HIGH |
| kernel_nway | 1832449.6ns | 1978838.5ns | 92.6% | HIGH |
| kernel_seq | 1857322.9ns | 1791447.9ns | 103.7% | HIGH |
| nway | 1707562.5ns | 1737031.2ns | 98.3% | HIGH |
| sequential | 1659757.0ns | 1845637.2ns | 89.9% | HIGH |
| std_parallel | 1849515.6ns | 2081154.5ns | 88.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 1508614.6-2160390.6 ns)
  1508614.6 |########################################
  1541203.4 |
  1573792.2 |
  1606381.0 |
  1638969.8 |####################
  1671558.6 |
  1704147.4 |
  1736736.2 |
  1769325.0 |####################
  1801913.8 |
  1834502.6 |
  1867091.4 |
  1899680.2 |
  1932269.0 |
  1964857.8 |
  1997446.6 |####################
  2030035.4 |
  2062624.2 |
  2095213.0 |
  2127801.8 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 1510666.7-2311552.1 ns)
  1510666.7 |########################################
  1550711.0 |########################################
  1590755.2 |
  1630799.5 |########################################
  1670843.8 |########################################
  1710888.1 |
  1750932.3 |
  1790976.6 |
  1831020.9 |########################################
  1871065.1 |
  1911109.4 |
  1951153.7 |
  1991197.9 |
  2031242.2 |
  2071286.5 |
  2111330.8 |
  2151375.0 |
  2191419.3 |
  2231463.6 |
  2271507.8 |
  (0 below, 1 above range)

kernel_nway (n=6, range 1460822.9-2288166.7 ns)
  1460822.9 |####################
  1502190.1 |
  1543557.3 |
  1584924.5 |
  1626291.6 |
  1667658.8 |
  1709026.0 |
  1750393.2 |
  1791760.4 |####################
  1833127.6 |####################
  1874494.8 |
  1915862.0 |
  1957229.2 |
  1998596.3 |
  2039963.5 |
  2081330.7 |
  2122697.9 |
  2164065.1 |########################################
  2205432.3 |
  2246799.5 |
  (0 below, 1 above range)

kernel_seq (n=6, range 1507854.2-2049140.6 ns)
  1507854.2 |########################################
  1534918.5 |########################################
  1561982.8 |
  1589047.2 |
  1616111.5 |
  1643175.8 |########################################
  1670240.1 |
  1697304.5 |
  1724368.8 |
  1751433.1 |
  1778497.4 |
  1805561.7 |
  1832626.1 |
  1859690.4 |
  1886754.7 |########################################
  1913819.0 |
  1940883.4 |
  1967947.7 |
  1995012.0 |
  2022076.3 |########################################
  (0 below, 1 above range)

nway (n=6, range 1494395.8-1951619.8 ns)
  1494395.8 |####################
  1517257.0 |
  1540118.2 |
  1562979.4 |
  1585840.6 |####################
  1608701.8 |
  1631563.0 |
  1654424.2 |
  1677285.4 |####################
  1700146.6 |
  1723007.8 |
  1745869.0 |########################################
  1768730.2 |
  1791591.4 |
  1814452.6 |
  1837313.8 |
  1860175.0 |
  1883036.2 |
  1905897.4 |
  1928758.6 |
  (0 below, 1 above range)

sequential (n=6, range 1521697.9-2288984.4 ns)
  1521697.9 |########################################
  1560062.2 |
  1598426.5 |
  1636790.9 |####################
  1675155.2 |
  1713519.5 |
  1751883.8 |
  1790248.2 |####################
  1828612.5 |####################
  1866976.8 |
  1905341.1 |
  1943705.4 |
  1982069.8 |
  2020434.1 |
  2058798.4 |
  2097162.7 |
  2135527.1 |
  2173891.4 |
  2212255.7 |
  2250620.0 |
  (0 below, 1 above range)

std_parallel (n=6, range 1666364.6-2441765.6 ns)
  1666364.6 |########################################
  1705134.7 |
  1743904.7 |
  1782674.8 |
  1821444.8 |
  1860214.9 |
  1898984.9 |####################
  1937754.9 |
  1976525.0 |
  2015295.0 |
  2054065.1 |
  2092835.1 |
  2131605.2 |
  2170375.2 |
  2209145.3 |
  2247915.3 |
  2286685.4 |####################
  2325455.4 |
  2364225.5 |
  2402995.5 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=96.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=23.3% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=100.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=91.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=94.7% of algo (FFI overhead may distort results)
- **nway**: bridge=98.8% of algo (FFI overhead may distort results)
- **sequential**: CV=22.7% (high variance, measurements may be unstable)
- **sequential**: bridge=95.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=91.8% of algo (FFI overhead may distort results)

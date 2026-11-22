# fold_light, far above cache, W = 8

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail is fastest but the noisiest (CV 23.9%)

head_tail wins on median (1.33 ms) yet has the highest variance (CV 23.9%), while head_tail_dyn is the steadiest (CV 6.5%, 2.07 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (1.33 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### kernel_nway is inconsistent: worst-20% is 1.8x its best-20%

kernel_nway's best 20% of batches run at 1.19 ms but its worst 20% at 2.16 ms (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 1325218.8 ns median
- 3 variants significantly slower than baseline
- Spread: 1.56x (fastest 1325218.8 ns, slowest 2070687.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 8 |
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
| head_tail | 1417872ns | 1328510ns | 1123396ns | 1266917ns | 1791542ns | base |
| head_tail_dyn | 2120071ns | 2074844ns | 1980146ns | 2050795ns | 2293948ns | +49.52% |
| kernel_nway | 1752370ns | 1766031ns | 1190135ns | 1663538ns | 2166734ns | +23.59% |
| kernel_seq | 1799727ns | 1694031ns | 1538010ns | 1643031ns | 2165630ns | +26.93% |
| nway | 1464884ns | 1443344ns | 1199094ns | 1414882ns | 1672781ns | +3.32% |
| sequential | 1881205ns | 1883188ns | 1551719ns | 1842781ns | 2103583ns | +32.68% |
| std_parallel | 1657625ns | 1606396ns | 1348656ns | 1528628ns | 2005604ns | +16.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 1414486ns | 1120625ns | 1787167ns | base | 11.861 |
| head_tail_dyn | 2116349ns | 1976125ns | 2290599ns | +49.62% | 7.927 |
| kernel_nway | 1748538ns | 1187344ns | 2162786ns | +23.62% | 9.595 |
| kernel_seq | 1797562ns | 1536188ns | 2163589ns | +27.08% | 9.333 |
| nway | 1461736ns | 1196198ns | 1669500ns | +3.34% | 11.478 |
| sequential | 1879187ns | 1550000ns | 2101469ns | +32.85% | 8.928 |
| std_parallel | 1655030ns | 1346458ns | 2002448ns | +17.01% | 10.137 |

## Performance model

- Peak throughput: **14.971 Gops/s** (head_tail; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 12.660 | 84.6% |
| head_tail_dyn | 8.102 | 54.1% |
| kernel_nway | 9.522 | 63.6% |
| kernel_seq | 9.917 | 66.2% |
| nway | 11.647 | 77.8% |
| sequential | 8.920 | 59.6% |
| std_parallel | 10.460 | 69.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 1417872ns | 1417872ns | base |
| head_tail_dyn | 2120071ns | 2120071ns | +49.52% |
| kernel_nway | 1752370ns | 1752370ns | +23.59% |
| kernel_seq | 1799727ns | 1799727ns | +26.93% |
| nway | 1464884ns | 1464884ns | +3.32% |
| sequential | 1881205ns | 1881205ns | +32.68% |
| std_parallel | 1657625ns | 1657625ns | +16.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 1325219ns | base | --- | [1131073, 1787167] | --- | --- | --- | --- |
| head_tail_dyn | 2070688ns | +721708.3ns (+54.5%) | [+292880, +1091000]ns | [1987760, 2290599] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| kernel_nway | 1761901ns | no significant difference | [-366474, +1027063]ns | [1320927, 2162786] | no | 0.6875 | 0.6875 | 0 |
| kernel_seq | 1691745ns | no significant difference | [-17995, +746083]ns | [1537354, 2163589] | no | 0.3281 | 0.2188 | 0 |
| nway | 1440495ns | no significant difference | [-511953, +524266]ns | [1275214, 1669500] | no | 0.6875 | 0.6875 | 0 |
| sequential | 1880953ns | +492145.9ns (+37.1%) | [+100615, +801344]ns | [1655141, 2101469] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| std_parallel | 1604005ns | +178901.0ns (+13.5%) | [+64240, +478490]ns | [1358635, 2002448] | YES (adj: no) | 0.1875 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 1787167ns | +18.0% | -3.4% | +7.0% | -28.6% | +5.6% | +6.8% |
| 1 | 2 | 1229630ns | +68.4% | +42.9% | +49.0% | +17.1% | +65.2% | +17.3% |
| 1 | 3 | 1226661ns | +76.8% | +43.6% | +34.3% | +36.1% | +40.1% | +31.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.164 | ok |
| head_tail_dyn | -0.427 | moderate- |
| kernel_nway | -0.107 | ok |
| kernel_seq | -0.032 | ok |
| nway | -0.006 | ok |
| sequential | -0.472 | moderate- |
| std_parallel | -0.407 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 1427869.8ns | 1414486.1ns | 100.9% | HIGH |
| head_tail_dyn | 2110085.1ns | 2116349.0ns | 99.7% | HIGH |
| kernel_nway | 1612918.4ns | 1748538.2ns | 92.2% | HIGH |
| kernel_seq | 1739349.0ns | 1797562.5ns | 96.8% | HIGH |
| nway | 1394439.2ns | 1461736.1ns | 95.4% | HIGH |
| sequential | 1861343.8ns | 1879187.5ns | 99.1% | HIGH |
| std_parallel | 1679793.4ns | 1655029.5ns | 101.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 1120625.0-1787166.6 ns)
  1120625.0 |########################################
  1153952.1 |
  1187279.2 |
  1220606.2 |
  1253933.3 |
  1287260.4 |####################
  1320587.5 |####################
  1353914.6 |
  1387241.7 |
  1420568.7 |
  1453895.8 |
  1487222.9 |####################
  1520550.0 |
  1553877.1 |
  1587204.2 |
  1620531.2 |
  1653858.3 |
  1687185.4 |
  1720512.5 |
  1753839.6 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 1976125.0-2290599.0 ns)
  1976125.0 |########################################
  1991848.7 |########################################
  2007572.4 |
  2023296.1 |
  2039019.8 |
  2054743.5 |########################################
  2070467.2 |########################################
  2086190.9 |
  2101914.6 |
  2117638.3 |
  2133362.0 |
  2149085.7 |
  2164809.4 |
  2180533.1 |
  2196256.8 |
  2211980.5 |########################################
  2227704.2 |
  2243427.9 |
  2259151.6 |
  2274875.3 |
  (0 below, 1 above range)

kernel_nway (n=6, range 1187343.8-2162786.5 ns)
  1187343.8 |####################
  1236115.9 |
  1284888.1 |
  1333660.2 |
  1382432.3 |
  1431204.5 |####################
  1479976.6 |
  1528748.7 |####################
  1577520.9 |
  1626293.0 |
  1675065.1 |
  1723837.3 |
  1772609.4 |
  1821381.6 |
  1870153.7 |
  1918925.8 |
  1967698.0 |########################################
  2016470.1 |
  2065242.2 |
  2114014.4 |
  (0 below, 1 above range)

kernel_seq (n=6, range 1536187.5-2163588.5 ns)
  1536187.5 |########################################
  1567557.6 |
  1598927.6 |####################
  1630297.7 |
  1661667.7 |
  1693037.8 |
  1724407.8 |
  1755777.9 |####################
  1787147.9 |
  1818518.0 |
  1849888.0 |
  1881258.1 |
  1912628.1 |
  1943998.2 |
  1975368.2 |
  2006738.3 |
  2038108.3 |
  2069478.4 |
  2100848.4 |####################
  2132218.5 |
  (0 below, 1 above range)

nway (n=6, range 1196197.9-1669500.0 ns)
  1196197.9 |########################################
  1219863.0 |
  1243528.1 |
  1267193.2 |
  1290858.3 |
  1314523.4 |
  1338188.5 |########################################
  1361853.6 |########################################
  1385518.7 |
  1409183.8 |
  1432848.9 |
  1456514.1 |
  1480179.2 |
  1503844.3 |########################################
  1527509.4 |########################################
  1551174.5 |
  1574839.6 |
  1598504.7 |
  1622169.8 |
  1645834.9 |
  (0 below, 1 above range)

sequential (n=6, range 1550000.0-2101468.7 ns)
  1550000.0 |########################################
  1577573.4 |
  1605146.9 |
  1632720.3 |
  1660293.7 |
  1687867.2 |
  1715440.6 |
  1743014.0 |########################################
  1770587.5 |
  1798160.9 |
  1825734.4 |
  1853307.8 |########################################
  1880881.2 |########################################
  1908454.7 |
  1936028.1 |
  1963601.5 |
  1991175.0 |########################################
  2018748.4 |
  2046321.8 |
  2073895.3 |
  (0 below, 1 above range)

std_parallel (n=6, range 1346458.3-2002448.0 ns)
  1346458.3 |########################################
  1379257.8 |
  1412057.3 |
  1444856.7 |
  1477656.2 |
  1510455.7 |####################
  1543255.2 |
  1576054.7 |
  1608854.2 |
  1641653.6 |####################
  1674453.1 |
  1707252.6 |
  1740052.1 |
  1772851.6 |
  1805651.1 |
  1838450.5 |####################
  1871250.0 |
  1904049.5 |
  1936849.0 |
  1969648.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=22.4% (high variance, measurements may be unstable)
- **head_tail**: bridge=102.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=96.6% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=22.2% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=90.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=101.6% of algo (FFI overhead may distort results)
- **nway**: bridge=98.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=95.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=96.6% of algo (FFI overhead may distort results)

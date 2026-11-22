# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: rcm_rev** at 1784668.8 ns median (-4.3% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.12x (fastest 1784668.8 ns, slowest 2003656.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | A1-1 dispatch order: column-adjacency dose response across valid topo orders |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c304d312345678 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 1892624ns | 1871254ns | 1846526ns | 1869121ns | 1950926ns | base |
| rcm_half | 1913645ns | 1891217ns | 1874407ns | 1887096ns | 1973088ns | +1.11% |
| rcm_rev | 1823618ns | 1789683ns | 1779807ns | 1792057ns | 1892865ns | -3.65% |
| rcm_scr | 2007930ns | 2009277ns | 1941868ns | 1996573ns | 2057997ns | +6.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 1886772ns | 1840760ns | 1945140ns | base | 0.001 |
| rcm_half | 1908415ns | 1868957ns | 1967915ns | +1.15% | 0.001 |
| rcm_rev | 1818177ns | 1774533ns | 1887109ns | -3.64% | 0.001 |
| rcm_scr | 2002597ns | 1936504ns | 2052591ns | +6.14% | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.001 | 95.1% |
| rcm_half | 0.001 | 94.1% |
| rcm_rev | 0.001 | 99.4% |
| rcm_scr | 0.001 | 88.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 1892624ns | 1892624ns | base |
| rcm_half | 1913645ns | 1913645ns | +1.11% |
| rcm_rev | 1823618ns | 1823618ns | -3.65% |
| rcm_scr | 2007930ns | 2007930ns | +6.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 1865498ns | base | --- | [1849917, 1910912] | --- | --- | --- | --- |
| rcm_half | 1885777ns | no significant difference | [-25769, +41779]ns | [1875133, 1915248] | no | 0.4807 | 0.4807 | 0 |
| rcm_rev | 1784669ns | -80922.9ns (-4.3%) | [-105994, -58210]ns | [1782085, 1819031] | YES | 0.0020 | 0.0013 | 0 |
| rcm_scr | 2003656ns | +125195.8ns (+6.7%) | [+83898, +144333]ns | [1976288, 2030550] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 1861296ns | +0.6% | -4.1% | +4.5% |
| 1 | 2 | 1853738ns | +2.2% | -3.5% | +8.0% |
| 1 | 3 | 1908125ns | -1.8% | -4.8% | +5.1% |
| 2 | 1 | 1897538ns | -1.1% | -6.0% | +6.3% |
| 2 | 2 | 1843358ns | +2.0% | -3.3% | +7.8% |
| 2 | 3 | 1881079ns | +3.4% | -5.1% | +6.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.174 | ok |
| rcm_half | -0.074 | ok |
| rcm_rev | -0.119 | ok |
| rcm_scr | -0.064 | ok |

**Consistency summary:**

- **rcm_half**: won 2/6, lost 4/6
- **rcm_rev**: won 6/6, lost 0/6
- **rcm_scr**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 44731162.5ns | 1886772.0ns | 2370.8% | HIGH |
| rcm_half | 44645857.4ns | 1908414.8ns | 2339.4% | HIGH |
| rcm_rev | 44639794.4ns | 1818177.3ns | 2455.2% | HIGH |
| rcm_scr | 44725919.4ns | 2002596.5ns | 2233.4% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 1840759.7-1945139.6 ns)
  1840759.7 |##########################
  1845978.7 |########################################
  1851197.7 |#############
  1856416.7 |##########################
  1861635.7 |
  1866854.7 |#############
  1872073.7 |
  1877292.7 |#############
  1882511.7 |
  1887730.6 |
  1892949.6 |#############
  1898168.6 |
  1903387.6 |#############
  1908606.6 |#############
  1913825.6 |#############
  1919044.6 |#############
  1924263.6 |
  1929482.6 |
  1934701.6 |
  1939920.6 |
  (1 below, 2 above range)

rcm_half (n=18, range 1868957.0-1967914.6 ns)
  1868957.0 |########################################
  1873904.8 |########################################
  1878852.7 |#############
  1883800.6 |
  1888748.5 |##########################
  1893696.4 |#############
  1898644.3 |
  1903592.1 |#############
  1908540.0 |
  1913487.9 |
  1918435.8 |
  1923383.7 |#############
  1928331.5 |
  1933279.4 |
  1938227.3 |
  1943175.2 |#############
  1948123.1 |
  1953070.9 |
  1958018.8 |#############
  1962966.7 |
  (2 below, 2 above range)

rcm_rev (n=18, range 1774533.4-1887109.0 ns)
  1774533.4 |#####
  1780162.1 |########################################
  1785790.9 |#####
  1791419.7 |
  1797048.5 |
  1802677.3 |
  1808306.1 |#####
  1813934.8 |#####
  1819563.6 |#####
  1825192.4 |
  1830821.2 |
  1836450.0 |
  1842078.8 |
  1847707.5 |
  1853336.3 |#####
  1858965.1 |
  1864593.9 |
  1870222.7 |
  1875851.5 |
  1881480.2 |
  (2 below, 3 above range)

rcm_scr (n=18, range 1936504.2-2052591.0 ns)
  1936504.2 |
  1942308.5 |##########
  1948112.8 |##########
  1953917.2 |
  1959721.5 |
  1965525.9 |
  1971330.2 |##########
  1977134.6 |##########
  1982938.9 |##########
  1988743.2 |
  1994547.6 |
  2000351.9 |########################################
  2006156.3 |
  2011960.6 |##########
  2017764.9 |
  2023569.3 |##########
  2029373.6 |
  2035178.0 |####################
  2040982.3 |##########
  2046786.6 |
  (2 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=2397.0% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=2366.0% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=2500.6% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=2229.2% of algo (FFI overhead may distort results)

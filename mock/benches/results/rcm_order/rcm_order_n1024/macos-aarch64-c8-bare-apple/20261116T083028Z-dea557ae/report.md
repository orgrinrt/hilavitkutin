# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rcm_scr shows alternating (throttle bounce) (autocorr -0.66)

rcm_scr's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: rcm_rev** at 1819508.4 ns median (-2.2% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.11x (fastest 1819508.4 ns, slowest 2012120.9 ns)

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
| rcm_adj | 1863925ns | 1865412ns | 1842954ns | 1861506ns | 1878037ns | base |
| rcm_half | 1930391ns | 1920925ns | 1909881ns | 1923956ns | 1950299ns | +3.57% |
| rcm_rev | 1822820ns | 1824546ns | 1803490ns | 1820331ns | 1836218ns | -2.21% |
| rcm_scr | 2020482ns | 2016960ns | 1985238ns | 2008821ns | 2055597ns | +8.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 1858750ns | 1837685ns | 1873039ns | base | 0.001 |
| rcm_half | 1925511ns | 1905058ns | 1945288ns | +3.59% | 0.001 |
| rcm_rev | 1817863ns | 1798486ns | 1831337ns | -2.20% | 0.001 |
| rcm_scr | 2015449ns | 1980226ns | 2050658ns | +8.43% | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.001 | 96.7% |
| rcm_half | 0.001 | 93.8% |
| rcm_rev | 0.001 | 98.8% |
| rcm_scr | 0.001 | 89.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 1863925ns | 1863925ns | base |
| rcm_half | 1930391ns | 1930391ns | +3.57% |
| rcm_rev | 1822820ns | 1822820ns | -2.21% |
| rcm_scr | 2020482ns | 2020482ns | +8.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 1860442ns | base | --- | [1853419, 1865477] | --- | --- | --- | --- |
| rcm_half | 1916354ns | +60275.0ns (+3.2%) | [+53363, +80529]ns | [1912317, 1940117] | YES | 0.0000 | 0.0000 | 0 |
| rcm_rev | 1819508ns | -41777.1ns (-2.2%) | [-47896, -32917]ns | [1811554, 1828067] | YES | 0.0000 | 0.0000 | 0 |
| rcm_scr | 2012121ns | +147402.1ns (+7.9%) | [+126752, +186879]ns | [1987556, 2028810] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 1855100ns | +5.0% | -2.0% | +9.2% |
| 1 | 2 | 1861171ns | +2.4% | -2.5% | +7.2% |
| 1 | 3 | 1854458ns | +4.4% | -1.7% | +9.4% |
| 2 | 1 | 1844742ns | +3.6% | -1.6% | +7.7% |
| 2 | 2 | 1861483ns | +2.9% | -2.2% | +8.2% |
| 2 | 3 | 1873958ns | +3.6% | -2.2% | +7.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.086 | ok |
| rcm_half | -0.530 | HIGH- (thermal bounce) |
| rcm_rev | -0.012 | ok |
| rcm_scr | -0.657 | HIGH- (thermal bounce) |

**Consistency summary:**

- **rcm_half**: won 0/6, lost 6/6
- **rcm_rev**: won 6/6, lost 0/6
- **rcm_scr**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 44741956.7ns | 1858749.8ns | 2407.1% | HIGH |
| rcm_half | 44734626.4ns | 1925510.7ns | 2323.3% | HIGH |
| rcm_rev | 44750847.2ns | 1817863.2ns | 2461.7% | HIGH |
| rcm_scr | 44803332.2ns | 2015448.6ns | 2223.0% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 1837684.7-1873038.9 ns)
  1837684.7 |#############
  1839452.4 |
  1841220.1 |#############
  1842987.9 |#############
  1844755.6 |
  1846523.3 |
  1848291.0 |
  1850058.7 |
  1851826.4 |#############
  1853594.1 |##########################
  1855361.8 |#############
  1857129.5 |
  1858897.2 |#############
  1860664.9 |########################################
  1862432.6 |
  1864200.3 |#############
  1865968.0 |#############
  1867735.7 |
  1869503.5 |
  1871271.2 |
  (1 below, 4 above range)

rcm_half (n=18, range 1905058.3-1945288.2 ns)
  1905058.3 |####################
  1907069.8 |##########
  1909081.3 |
  1911092.8 |####################
  1913104.3 |
  1915115.8 |########################################
  1917127.3 |
  1919138.8 |
  1921150.3 |
  1923161.8 |
  1925173.3 |
  1927184.8 |
  1929196.3 |
  1931207.8 |
  1933219.3 |
  1935230.7 |##########
  1937242.2 |##########
  1939253.7 |####################
  1941265.2 |
  1943276.7 |##########
  (1 below, 3 above range)

rcm_rev (n=18, range 1798486.1-1831336.8 ns)
  1798486.1 |####################
  1800128.6 |########################################
  1801771.2 |
  1803413.7 |
  1805056.2 |
  1806698.8 |
  1808341.3 |
  1809983.9 |########################################
  1811626.4 |
  1813268.9 |####################
  1814911.5 |####################
  1816554.0 |####################
  1818196.5 |
  1819839.1 |####################
  1821481.6 |########################################
  1823124.1 |
  1824766.7 |
  1826409.2 |
  1828051.7 |####################
  1829694.3 |########################################
  (1 below, 3 above range)

rcm_scr (n=18, range 1980226.4-2050657.6 ns)
  1980226.4 |
  1983748.0 |########################################
  1987269.5 |#############
  1990791.1 |
  1994312.7 |#############
  1997834.2 |
  2001355.8 |#############
  2004877.3 |
  2008398.9 |#############
  2011920.5 |#############
  2015442.0 |
  2018963.6 |#############
  2022485.1 |
  2026006.7 |########################################
  2029528.3 |
  2033049.8 |
  2036571.4 |
  2040093.0 |#############
  2043614.5 |
  2047136.1 |#############
  (2 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=2404.3% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=2334.1% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=2458.5% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=2223.5% of algo (FFI overhead may distort results)

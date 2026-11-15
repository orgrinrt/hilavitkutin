# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: rcm_rev** at 1827156.2 ns median (-1.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.10x (fastest 1827156.2 ns, slowest 2010045.9 ns)

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
| rcm_adj | 1876409ns | 1867529ns | 1834671ns | 1859086ns | 1923263ns | base |
| rcm_half | 2095588ns | 1954560ns | 1903854ns | 1945957ns | 2415899ns | +11.68% |
| rcm_rev | 1844045ns | 1832412ns | 1797876ns | 1825310ns | 1895231ns | -1.72% |
| rcm_scr | 2040221ns | 2015629ns | 1969918ns | 2011052ns | 2119125ns | +8.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 1870831ns | 1829179ns | 1917654ns | base | 0.001 |
| rcm_half | 2090141ns | 1898089ns | 2410503ns | +11.72% | 0.000 |
| rcm_rev | 1838574ns | 1791975ns | 1889766ns | -1.72% | 0.001 |
| rcm_scr | 2034491ns | 1964381ns | 2113194ns | +8.75% | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.001 | 96.2% |
| rcm_half | 0.001 | 91.9% |
| rcm_rev | 0.001 | 98.1% |
| rcm_scr | 0.001 | 89.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 1876409ns | 1876409ns | base |
| rcm_half | 2095588ns | 2095588ns | +11.68% |
| rcm_rev | 1844045ns | 1844045ns | -1.72% |
| rcm_scr | 2040221ns | 2040221ns | +8.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 1862190ns | base | --- | [1844375, 1875554] | --- | --- | --- | --- |
| rcm_half | 1949450ns | +79610.4ns (+4.3%) | [+67833, +133483]ns | [1910712, 2005283] | YES | 0.0020 | 0.0013 | 0 |
| rcm_rev | 1827156ns | -37566.7ns (-2.0%) | [-54252, -9681]ns | [1804869, 1866629] | YES | 0.0309 | 0.0309 | 0 |
| rcm_scr | 2010046ns | +139814.6ns (+7.5%) | [+127425, +197712]ns | [1989904, 2084012] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 1864779ns | +8.2% | -2.5% | +7.9% |
| 1 | 2 | 1848154ns | +5.3% | -2.1% | +8.0% |
| 1 | 3 | 1863317ns | +3.8% | -2.4% | +7.6% |
| 2 | 1 | 1879875ns | +9.3% | -0.7% | +6.8% |
| 2 | 2 | 1855929ns | +7.1% | -0.7% | +11.1% |
| 2 | 3 | 1829296ns | +4.3% | -1.7% | +11.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.018 | ok |
| rcm_half | -0.208 | moderate- |
| rcm_rev | 0.061 | ok |
| rcm_scr | 0.305 | moderate+ |

**Consistency summary:**

- **rcm_half**: won 0/6, lost 6/6
- **rcm_rev**: won 6/6, lost 0/6
- **rcm_scr**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 44688323.4ns | 1870830.5ns | 2388.7% | HIGH |
| rcm_half | 45016034.0ns | 2090141.4ns | 2153.7% | HIGH |
| rcm_rev | 44799356.5ns | 1838573.8ns | 2436.6% | HIGH |
| rcm_scr | 44814104.9ns | 2034491.0ns | 2202.7% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 1829179.1-1917654.2 ns)
  1829179.1 |##########################
  1833602.9 |#############
  1838026.6 |#############
  1842450.4 |
  1846874.1 |##########################
  1851297.9 |
  1855721.6 |#############
  1860145.4 |########################################
  1864569.1 |#############
  1868992.9 |#############
  1873416.6 |
  1877840.4 |#############
  1882264.1 |
  1886687.9 |
  1891111.6 |
  1895535.4 |
  1899959.1 |##########################
  1904382.9 |#############
  1908806.6 |
  1913230.4 |
  (1 below, 1 above range)

rcm_half (n=18, range 1898088.9-2410502.8 ns)
  1898088.9 |########################################
  1923709.6 |########################
  1949330.3 |########
  1974951.0 |########################
  2000571.7 |################
  2026192.4 |
  2051813.1 |################
  2077433.8 |
  2103054.5 |
  2128675.1 |
  2154295.8 |
  2179916.5 |
  2205537.2 |
  2231157.9 |
  2256778.6 |
  2282399.3 |
  2308020.0 |
  2333640.7 |
  2359261.4 |
  2384882.1 |
  (1 below, 1 above range)

rcm_rev (n=18, range 1791975.0-1889766.0 ns)
  1791975.0 |
  1796864.6 |########################################
  1801754.1 |
  1806643.7 |#############
  1811533.2 |
  1816422.8 |##########################
  1821312.3 |#############
  1826201.9 |#############
  1831091.4 |#############
  1835981.0 |
  1840870.5 |#############
  1845760.1 |
  1850649.6 |
  1855539.2 |
  1860428.7 |
  1865318.2 |##########################
  1870207.8 |
  1875097.3 |#############
  1879986.9 |
  1884876.4 |
  (2 below, 3 above range)

rcm_scr (n=18, range 1964380.6-2113194.4 ns)
  1964380.6 |####################
  1971821.3 |
  1979262.0 |####################
  1986702.6 |########################################
  1994143.3 |####################
  2001584.0 |########################################
  2009024.7 |####################
  2016465.4 |
  2023906.1 |####################
  2031346.8 |####################
  2038787.5 |
  2046228.2 |
  2053668.9 |
  2061109.6 |####################
  2068550.3 |
  2075991.0 |
  2083431.7 |
  2090872.4 |
  2098313.1 |
  2105753.8 |####################
  (2 below, 4 above range)

```

## Diagnostics

- **rcm_adj**: bridge=2398.7% of algo (FFI overhead may distort results)
- **rcm_half**: CV=26.0% (high variance, measurements may be unstable)
- **rcm_half**: bridge=2291.9% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=2445.9% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=2227.5% of algo (FFI overhead may distort results)

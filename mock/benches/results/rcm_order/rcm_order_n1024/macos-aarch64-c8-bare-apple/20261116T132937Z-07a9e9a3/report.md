# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rcm_rev shows alternating (throttle bounce) (autocorr -0.57)

rcm_rev's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: rcm_rev** at 1777631.2 ns median (-3.4% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.09x (fastest 1777631.2 ns, slowest 1941977.1 ns)

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
| rcm_adj | 1845511ns | 1845915ns | 1819919ns | 1842962ns | 1862129ns | base |
| rcm_half | 1900045ns | 1886960ns | 1874262ns | 1885489ns | 1934769ns | +2.95% |
| rcm_rev | 1784383ns | 1783200ns | 1773254ns | 1781141ns | 1794810ns | -3.31% |
| rcm_scr | 1957922ns | 1947162ns | 1936649ns | 1944585ns | 1988565ns | +6.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 1839958ns | 1814240ns | 1856408ns | base | 0.001 |
| rcm_half | 1894792ns | 1869103ns | 1929467ns | +2.98% | 0.001 |
| rcm_rev | 1779211ns | 1767990ns | 1789856ns | -3.30% | 0.001 |
| rcm_scr | 1952799ns | 1931239ns | 1983521ns | +6.13% | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.001 | 96.1% |
| rcm_half | 0.001 | 94.0% |
| rcm_rev | 0.001 | 99.5% |
| rcm_scr | 0.001 | 91.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 1845511ns | 1845511ns | base |
| rcm_half | 1900045ns | 1900045ns | +2.95% |
| rcm_rev | 1784383ns | 1784383ns | -3.31% |
| rcm_scr | 1957922ns | 1957922ns | +6.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 1840560ns | base | --- | [1833685, 1850498] | --- | --- | --- | --- |
| rcm_half | 1881833ns | +47535.4ns (+2.6%) | [+42610, +54940]ns | [1876917, 1894690] | YES | 0.0000 | 0.0000 | 0 |
| rcm_rev | 1777631ns | -62741.7ns (-3.4%) | [-72963, -52573]ns | [1772883, 1784550] | YES | 0.0000 | 0.0000 | 0 |
| rcm_scr | 1941977ns | +101368.8ns (+5.5%) | [+86377, +110896]ns | [1936319, 1944798] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 1853592ns | +2.6% | -4.4% | +4.2% |
| 1 | 2 | 1811421ns | +3.3% | -1.4% | +10.0% |
| 1 | 3 | 1837546ns | +2.6% | -3.4% | +5.7% |
| 2 | 1 | 1840038ns | +2.0% | -3.3% | +5.6% |
| 2 | 2 | 1836825ns | +2.4% | -3.4% | +5.7% |
| 2 | 3 | 1854779ns | +2.3% | -3.6% | +4.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.325 | moderate- |
| rcm_half | -0.368 | moderate- |
| rcm_rev | -0.567 | HIGH- (thermal bounce) |
| rcm_scr | -0.344 | moderate- |

**Consistency summary:**

- **rcm_half**: won 0/6, lost 6/6
- **rcm_rev**: won 6/6, lost 0/6
- **rcm_scr**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 44611513.0ns | 1839958.3ns | 2424.6% | HIGH |
| rcm_half | 44702457.9ns | 1894791.9ns | 2359.2% | HIGH |
| rcm_rev | 44623756.0ns | 1779211.3ns | 2508.1% | HIGH |
| rcm_scr | 44652163.6ns | 1952799.1ns | 2286.6% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 1814240.3-1856407.6 ns)
  1814240.3 |
  1816348.6 |
  1818457.0 |
  1820565.4 |
  1822673.7 |
  1824782.1 |
  1826890.5 |#############
  1828998.8 |
  1831107.2 |##########################
  1833215.6 |#############
  1835324.0 |#############
  1837432.3 |#############
  1839540.7 |########################################
  1841649.1 |#############
  1843757.4 |
  1845865.8 |
  1847974.2 |#############
  1850082.5 |
  1852190.9 |##########################
  1854299.3 |#############
  (2 below, 2 above range)

rcm_half (n=18, range 1869102.8-1929467.3 ns)
  1869102.8 |##############################
  1872121.0 |
  1875139.2 |####################
  1878157.5 |
  1881175.7 |########################################
  1884193.9 |####################
  1887212.1 |
  1890230.4 |##########
  1893248.6 |
  1896266.8 |##########
  1899285.1 |##########
  1902303.3 |
  1905321.5 |
  1908339.7 |
  1911358.0 |
  1914376.2 |
  1917394.4 |
  1920412.7 |
  1923430.9 |
  1926449.1 |
  (1 below, 3 above range)

rcm_rev (n=18, range 1767990.3-1789855.6 ns)
  1767990.3 |
  1769083.5 |####################
  1770176.8 |####################
  1771270.1 |
  1772363.3 |########################################
  1773456.6 |####################
  1774549.9 |
  1775643.1 |####################
  1776736.4 |####################
  1777829.6 |####################
  1778922.9 |####################
  1780016.2 |
  1781109.4 |####################
  1782202.7 |
  1783296.0 |####################
  1784389.2 |
  1785482.5 |####################
  1786575.8 |
  1787669.0 |########################################
  1788762.3 |
  (2 below, 2 above range)

rcm_scr (n=18, range 1931238.9-1983520.8 ns)
  1931238.9 |################
  1933853.0 |########
  1936467.1 |################
  1939081.2 |########
  1941695.3 |########################################
  1944309.4 |########
  1946923.5 |########
  1949537.6 |
  1952151.7 |
  1954765.8 |
  1957379.8 |
  1959993.9 |
  1962608.0 |
  1965222.1 |
  1967836.2 |
  1970450.3 |
  1973064.4 |
  1975678.5 |
  1978292.6 |
  1980906.7 |
  (2 below, 3 above range)

```

## Diagnostics

- **rcm_adj**: bridge=2423.7% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=2371.0% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=2509.9% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=2297.7% of algo (FFI overhead may distort results)

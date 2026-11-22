# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_engine is an outlier: 2.1x slower than the field

evs_engine (3.50 ms) is 2.1x the fastest (1.70 ms), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_std_par, evs_engine_par, evs_std} vs {evs_engine} (91% apart)

The field splits into a fast tier {evs_std_par, evs_engine_par, evs_std} and a slow tier {evs_engine} with a 91% jump between them - a qualitative difference, not a gradient.


### Speed leader evs_std_par vs stability leader evs_std (+8% speed for 1.3x steadier)

evs_std_par is fastest (1.70 ms, CV 7.7%); evs_std gives up 8.1% median for 1.3x lower variance (CV 6.0%).


## Key findings

- Fastest: evs_std_par at 1698465.1 ns median (-7.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.06x (fastest 1698465.1 ns, slowest 3498326.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | accumulator, a transform feeding the append surface |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000003 |
| Counter frequency | 2112872410 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 3603254ns | 3499206ns | 3466838ns | 3484116ns | 3726712ns | +92.30% |
| evs_engine_par | 1850828ns | 1791830ns | 1683894ns | 1746246ns | 1970997ns | -1.22% |
| evs_std | 1873735ns | 1836805ns | 1765548ns | 1811824ns | 1947215ns | base |
| evs_std_par | 1720719ns | 1701348ns | 1544163ns | 1628913ns | 1833712ns | -8.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 3602285ns | 3465977ns | 3725577ns | +92.38% | 1.164 |
| evs_engine_par | 1848463ns | 1681704ns | 1968527ns | -1.28% | 2.269 |
| evs_std | 1872436ns | 1764388ns | 1945766ns | base | 2.240 |
| evs_std_par | 1717670ns | 1541472ns | 1830596ns | -8.27% | 2.442 |

## Performance model

- Peak throughput: 2.721 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.199 | 44.1% |
| evs_engine_par | 2.344 | 86.1% |
| evs_std | 2.285 | 84.0% |
| evs_std_par | 2.469 | 90.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 3603254ns | 3603254ns | +92.30% |
| evs_engine_par | 1850828ns | 1850828ns | -1.22% |
| evs_std | 1873735ns | 1873735ns | base |
| evs_std_par | 1720719ns | 1720719ns | -8.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 1835464ns | base | --- | [1789590, 1968419] | --- | --- | --- | --- |
| evs_engine | 3498327ns | +1666670.7ns (+90.8%) | [+1619352, +1733378]ns | [3475321, 3684057] | YES | 0.0117 | 0.0078 | 0 |
| evs_engine_par | 1789650ns | no significant difference | [-59593, +26580]ns | [1704795, 1994999] | no | 0.7266 | 0.7266 | 0 |
| evs_std_par | 1698465ns | -202119.9ns (-11.0%) | [-240311, -37702]ns | [1596070, 1891388] | YES | 0.0117 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 1798746ns | +94.8% | -1.7% | -2.7% |
| 1 | 2 | 1846175ns | +88.3% | +1.0% | -11.1% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 0/2, lost 2/2
- evs_engine_par: won 1/2, lost 1/2
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 16703174.6ns | 3602284.8ns | 463.7% | HIGH |
| evs_engine_par | 17453748.5ns | 1848463.1ns | 944.2% | HIGH |
| evs_std | 16642208.2ns | 1872436.4ns | 888.8% | HIGH |
| evs_std_par | 16899111.5ns | 1717670.0ns | 983.8% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 3465977.1-3725577.5 ns)
  3465977.1 |########################################
  3478957.1 |
  3491937.1 |##########################
  3504917.2 |#############
  3517897.2 |
  3530877.2 |
  3543857.2 |
  3556837.2 |
  3569817.2 |
  3582797.3 |
  3595777.3 |
  3608757.3 |
  3621737.3 |
  3634717.3 |
  3647697.4 |
  3660677.4 |
  3673657.4 |#############
  3686637.4 |
  3699617.4 |
  3712597.5 |
  (0 below, 1 above range)

evs_engine_par (n=8, range 1681704.4-1968527.4 ns)
  1681704.4 |####################
  1696045.5 |####################
  1710386.7 |
  1724727.8 |
  1739069.0 |####################
  1753410.1 |
  1767751.3 |
  1782092.4 |########################################
  1796433.6 |
  1810774.7 |
  1825115.9 |
  1839457.0 |
  1853798.2 |
  1868139.3 |
  1882480.5 |
  1896821.6 |
  1911162.8 |
  1925503.9 |
  1939845.1 |####################
  1954186.2 |
  (0 below, 2 above range)

evs_std (n=8, range 1764388.5-1945765.9 ns)
  1764388.5 |####################
  1773457.4 |
  1782526.2 |####################
  1791595.1 |
  1800664.0 |####################
  1809732.8 |
  1818801.7 |
  1827870.6 |########################################
  1836939.4 |
  1846008.3 |
  1855077.2 |####################
  1864146.0 |
  1873214.9 |
  1882283.8 |
  1891352.6 |
  1900421.5 |
  1909490.4 |
  1918559.2 |
  1927628.1 |
  1936697.0 |
  (0 below, 2 above range)

evs_std_par (n=8, range 1541471.9-1830595.6 ns)
  1541471.9 |########################################
  1555928.1 |
  1570384.3 |
  1584840.4 |########################################
  1599296.6 |
  1613752.8 |########################################
  1628209.0 |
  1642665.2 |
  1657121.4 |########################################
  1671577.5 |
  1686033.7 |
  1700489.9 |
  1714946.1 |
  1729402.3 |########################################
  1743858.5 |
  1758314.6 |########################################
  1772770.8 |
  1787227.0 |
  1801683.2 |
  1816139.4 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=476.4% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=942.3% of algo (FFI overhead may distort results)
- evs_std: bridge=906.4% of algo (FFI overhead may distort results)
- evs_std_par: bridge=995.3% of algo (FFI overhead may distort results)

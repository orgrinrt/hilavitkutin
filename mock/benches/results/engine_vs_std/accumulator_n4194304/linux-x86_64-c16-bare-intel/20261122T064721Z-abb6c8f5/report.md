# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_engine is an outlier: 2.4x slower than the field

evs_engine (3.99 ms) is 2.4x the fastest (1.68 ms), well off the pack.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_std_par, evs_std, evs_engine_par} vs {evs_engine} (120% apart)

The field splits into a fast tier {evs_std_par, evs_std, evs_engine_par} and a slow tier {evs_engine} with a 120% jump between them - a qualitative difference, not a gradient.


## Key findings

- Fastest: evs_std_par at 1681018.2 ns median (-7.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.38x (fastest 1681018.2 ns, slowest 3992617.5 ns)

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
| Counter frequency | 2112831643 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 3901933ns | 3993548ns | 3475338ns | 3749608ns | 4122825ns | +106.03% |
| evs_engine_par | 1856001ns | 1817902ns | 1675440ns | 1761059ns | 1972348ns | -2.00% |
| evs_std | 1893893ns | 1810295ns | 1766359ns | 1793225ns | 2001277ns | base |
| evs_std_par | 1714006ns | 1684300ns | 1537970ns | 1647729ns | 1807723ns | -9.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 3900927ns | 3474598ns | 4121599ns | +106.10% | 1.075 |
| evs_engine_par | 1853296ns | 1672769ns | 1969642ns | -2.08% | 2.263 |
| evs_std | 1892729ns | 1765180ns | 1999995ns | base | 2.216 |
| evs_std_par | 1710538ns | 1534702ns | 1804045ns | -9.63% | 2.452 |

## Performance model

- Peak throughput: 2.733 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.051 | 38.4% |
| evs_engine_par | 2.311 | 84.5% |
| evs_std | 2.318 | 84.8% |
| evs_std_par | 2.495 | 91.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 3901933ns | 3901933ns | +106.03% |
| evs_engine_par | 1856001ns | 1856001ns | -2.00% |
| evs_std | 1893893ns | 1893893ns | base |
| evs_std_par | 1714006ns | 1714006ns | -9.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 1809349ns | base | --- | [1781774, 1989202] | --- | --- | --- | --- |
| evs_engine | 3992618ns | +2019853.4ns (+111.6%) | [+1811017, +2242310]ns | [3592790, 4211003] | YES | 0.0117 | 0.0078 | 0 |
| evs_engine_par | 1815222ns | no significant difference | [-156570, +88103]ns | [1681493, 2095602] | no | 1.0000 | 1.0000 | 0 |
| evs_std_par | 1681018ns | -135620.9ns (-7.5%) | [-271109, -73643]ns | [1629119, 1849998] | YES | 0.0117 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 1821113ns | +108.6% | -3.9% | -6.3% |
| 1 | 2 | 1797448ns | +122.2% | +2.0% | -7.6% |

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
| evs_engine | 16771492.8ns | 3900927.2ns | 429.9% | HIGH |
| evs_engine_par | 16802008.6ns | 1853295.6ns | 906.6% | HIGH |
| evs_std | 16682493.0ns | 1892728.6ns | 881.4% | HIGH |
| evs_std_par | 16949863.9ns | 1710537.8ns | 990.9% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 3474597.9-4121599.1 ns)
  3474597.9 |####################
  3506948.0 |
  3539298.0 |
  3571648.1 |####################
  3603998.1 |
  3636348.2 |
  3668698.3 |####################
  3701048.3 |
  3733398.4 |
  3765748.4 |
  3798098.5 |
  3830448.5 |
  3862798.6 |
  3895148.7 |
  3927498.7 |
  3959848.8 |####################
  3992198.8 |########################################
  4024548.9 |
  4056899.0 |
  4089249.0 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 1672769.0-1969642.2 ns)
  1672769.0 |########################################
  1687612.7 |
  1702456.3 |
  1717300.0 |
  1732143.6 |
  1746987.3 |
  1761831.0 |
  1776674.6 |####################
  1791518.3 |
  1806361.9 |########################################
  1821205.6 |
  1836049.2 |
  1850892.9 |####################
  1865736.6 |
  1880580.2 |
  1895423.9 |
  1910267.5 |
  1925111.2 |
  1939954.9 |
  1954798.5 |
  (0 below, 2 above range)

evs_std (n=8, range 1765180.5-1999994.6 ns)
  1765180.5 |########################################
  1776921.2 |########################################
  1788661.9 |########################################
  1800402.6 |########################################
  1812143.3 |########################################
  1823884.0 |########################################
  1835624.7 |
  1847365.4 |
  1859106.1 |
  1870846.8 |
  1882587.5 |
  1894328.2 |
  1906068.9 |
  1917809.6 |
  1929550.3 |
  1941291.0 |
  1953031.7 |
  1964772.4 |
  1976513.1 |
  1988253.8 |########################################
  (0 below, 1 above range)

evs_std_par (n=8, range 1534701.5-1804045.2 ns)
  1534701.5 |####################
  1548168.7 |
  1561635.9 |
  1575103.1 |
  1588570.2 |
  1602037.4 |
  1615504.6 |
  1628971.8 |########################################
  1642439.0 |
  1655906.2 |
  1669373.4 |####################
  1682840.5 |####################
  1696307.7 |
  1709774.9 |
  1723242.1 |
  1736709.3 |####################
  1750176.5 |
  1763643.6 |
  1777110.8 |
  1790578.0 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=421.8% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=924.5% of algo (FFI overhead may distort results)
- evs_std: bridge=920.7% of algo (FFI overhead may distort results)
- evs_std_par: bridge=1006.9% of algo (FFI overhead may distort results)

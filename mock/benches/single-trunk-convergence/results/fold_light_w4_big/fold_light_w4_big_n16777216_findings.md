# fold_light, far above cache, W = 4

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel beats baseline by 20% (significant)

std_parallel is -452.08 us (20%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### kernel_seq is an outlier: 3.3x slower than the field

kernel_seq (5.36 ms) is 3.3x the fastest (1.65 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_parallel shows alternating (throttle bounce) (autocorr -0.61)

std_parallel's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.3x the fastest

Fastest nway (1.65 ms) to slowest kernel_seq (5.36 ms): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.2x its best-20%

sequential's best 20% of batches run at 2.62 ms but its worst 20% at 8.38 ms (3.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: nway** at 1646146.9 ns median (-25.4% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 3.26x (fastest 1646146.9 ns, slowest 5364746.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 4 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100312120 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 2198123ns | 2209419ns | 1854818ns | 2167791ns | 2415272ns | base |
| head_tail_dyn | 4686296ns | 4680257ns | 4235122ns | 4637971ns | 4984370ns | +113.20% |
| kernel_nway | 1776066ns | 1792124ns | 1315569ns | 1730951ns | 2073988ns | -19.20% |
| kernel_seq | 5713804ns | 5368096ns | 2928165ns | 4683482ns | 8652108ns | +159.94% |
| nway | 1948530ns | 1647532ns | 1555653ns | 1626051ns | 2628688ns | -11.35% |
| sequential | 4672118ns | 2960966ns | 2622803ns | 2876479ns | 8390233ns | +112.55% |
| std_parallel | 1654198ns | 1775383ns | 1133706ns | 1644030ns | 1929695ns | -24.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2194669ns | 1854059ns | 2410728ns | base | 7.645 |
| head_tail_dyn | 4682717ns | 4233091ns | 4979560ns | +113.37% | 3.583 |
| kernel_nway | 1773044ns | 1314164ns | 2069456ns | -19.21% | 9.462 |
| kernel_seq | 5709473ns | 2926173ns | 8645439ns | +160.15% | 2.938 |
| nway | 1946995ns | 1554700ns | 2626725ns | -11.29% | 8.617 |
| sequential | 4669294ns | 2621540ns | 8384838ns | +112.76% | 3.593 |
| std_parallel | 1651826ns | 1132410ns | 1926473ns | -24.73% | 10.157 |

## Performance model

- Peak throughput: **14.815 Gops/s** (std_parallel; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 7.607 | 51.3% |
| head_tail_dyn | 3.587 | 24.2% |
| kernel_nway | 9.377 | 63.3% |
| kernel_seq | 3.127 | 21.1% |
| nway | 10.192 | 68.8% |
| sequential | 5.670 | 38.3% |
| std_parallel | 9.463 | 63.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 2198123ns | 2198123ns | base |
| head_tail_dyn | 4686296ns | 4686296ns | +113.20% |
| kernel_nway | 1776066ns | 1776066ns | -19.20% |
| kernel_seq | 5713804ns | 5713804ns | +159.94% |
| nway | 1948530ns | 1948530ns | -11.35% |
| sequential | 4672118ns | 4672118ns | +112.55% |
| std_parallel | 1654198ns | 1654198ns | -24.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2205367ns | base | --- | [1967912, 2410728] | --- | --- | --- | --- |
| head_tail_dyn | 4676628ns | +2514353.0ns (+114.0%) | [+2210547, +2739243]ns | [4391962, 4979560] | YES | 0.0375 | 0.0313 | 0 |
| kernel_nway | 1789137ns | -420118.4ns (-19.0%) | [-649045, -195712]ns | [1460538, 2069456] | YES | 0.0375 | 0.0313 | 0 |
| kernel_seq | 5364747ns | +3309523.6ns (+150.1%) | [+949690, +6285198]ns | [3118233, 8645439] | YES | 0.0375 | 0.0313 | 0 |
| nway | 1646147ns | no significant difference | [-710508, +353376]ns | [1568113, 2626725] | no | 0.6875 | 0.6875 | 0 |
| sequential | 2959124ns | +775547.0ns (+35.2%) | [+536838, +6111489]ns | [2663919, 8384838] | YES | 0.0375 | 0.0313 | 0 |
| std_parallel | 1772937ns | -452083.6ns (-20.5%) | [-777384, -399060]ns | [1256069, 1926473] | YES | 0.0375 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2142115ns | +118.3% | -15.2% | +154.5% | -23.2% | +32.4% | -21.1% |
| 1 | 2 | 2168543ns | +111.4% | -13.9% | +43.8% | -27.7% | +28.6% | -31.2% |
| 1 | 3 | 2273349ns | +110.6% | -28.1% | +276.5% | +15.5% | +268.8% | -22.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.071 | ok |
| head_tail_dyn | -0.469 | moderate- |
| kernel_nway | -0.297 | moderate- |
| kernel_seq | -0.451 | moderate- |
| nway | -0.206 | moderate- |
| sequential | -0.171 | ok |
| std_parallel | -0.611 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 2/3, lost 1/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 2546453.9ns | 2194669.0ns | 116.0% | HIGH |
| head_tail_dyn | 4775718.5ns | 4682716.7ns | 102.0% | HIGH |
| kernel_nway | 1808201.0ns | 1773043.9ns | 102.0% | HIGH |
| kernel_seq | 6093464.4ns | 5709472.8ns | 106.7% | HIGH |
| nway | 2105274.9ns | 1946994.8ns | 108.1% | HIGH |
| sequential | 5351024.8ns | 4669293.8ns | 114.6% | HIGH |
| std_parallel | 1675086.8ns | 1651826.2ns | 101.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 1854058.8-2410728.4 ns)
  1854058.8 |########################################
  1881892.3 |
  1909725.8 |
  1937559.2 |
  1965392.7 |
  1993226.2 |
  2021059.7 |
  2048893.1 |
  2076726.6 |########################################
  2104560.1 |
  2132393.6 |########################################
  2160227.1 |
  2188060.5 |
  2215894.0 |
  2243727.5 |########################################
  2271561.0 |
  2299394.4 |
  2327227.9 |
  2355061.4 |
  2382894.9 |########################################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 4233091.4-4979560.2 ns)
  4233091.4 |########################################
  4270414.8 |
  4307738.3 |
  4345061.7 |
  4382385.2 |
  4419708.6 |
  4457032.1 |
  4494355.5 |
  4531678.9 |########################################
  4569002.4 |
  4606325.8 |
  4643649.3 |########################################
  4680972.7 |########################################
  4718296.2 |
  4755619.6 |
  4792943.0 |
  4830266.5 |
  4867589.9 |
  4904913.4 |########################################
  4942236.8 |
  (0 below, 1 above range)

kernel_nway (n=6, range 1314164.5-2069456.4 ns)
  1314164.5 |########################################
  1351929.1 |
  1389693.7 |
  1427458.3 |
  1465222.9 |
  1502987.5 |
  1540752.1 |
  1578516.6 |########################################
  1616281.2 |########################################
  1654045.8 |
  1691810.4 |
  1729575.0 |
  1767339.6 |
  1805104.2 |
  1842868.8 |
  1880633.4 |
  1918398.0 |########################################
  1956162.6 |
  1993927.2 |########################################
  2031691.8 |
  (0 below, 1 above range)

kernel_seq (n=6, range 2926173.3-8645438.8 ns)
  2926173.3 |####################
  3212136.6 |########################################
  3498099.8 |
  3784063.1 |
  4070026.4 |
  4355989.7 |
  4641953.0 |
  4927916.2 |
  5213879.5 |
  5499842.8 |
  5785806.0 |
  6071769.3 |
  6357732.6 |
  6643695.9 |
  6929659.2 |
  7215622.4 |####################
  7501585.7 |####################
  7787549.0 |
  8073512.2 |
  8359475.5 |
  (0 below, 1 above range)

nway (n=6, range 1554699.9-2626724.8 ns)
  1554699.9 |########################################
  1608301.1 |####################
  1661902.4 |####################
  1715503.6 |
  1769104.9 |
  1822706.1 |
  1876307.4 |
  1929908.6 |
  1983509.9 |
  2037111.1 |
  2090712.3 |
  2144313.6 |####################
  2197914.8 |
  2251516.1 |
  2305117.3 |
  2358718.6 |
  2412319.8 |
  2465921.1 |
  2519522.3 |
  2573123.6 |
  (0 below, 1 above range)

sequential (n=6, range 2621540.5-8384838.3 ns)
  2621540.5 |########################################
  2909705.4 |########################################
  3197870.3 |
  3486035.2 |
  3774200.1 |
  4062365.0 |
  4350529.9 |
  4638694.7 |
  4926859.6 |
  5215024.5 |
  5503189.4 |####################
  5791354.3 |
  6079519.2 |
  6367684.1 |
  6655849.0 |
  6944013.9 |
  7232178.8 |
  7520343.7 |
  7808508.6 |
  8096673.5 |
  (0 below, 1 above range)

std_parallel (n=6, range 1132410.4-1926472.9 ns)
  1132410.4 |########################################
  1172113.5 |
  1211816.6 |
  1251519.8 |
  1291222.9 |
  1330926.0 |
  1370629.1 |########################################
  1410332.3 |
  1450035.4 |
  1489738.5 |
  1529441.6 |
  1569144.7 |
  1608847.9 |
  1648551.0 |
  1688254.1 |
  1727957.2 |########################################
  1767660.4 |########################################
  1807363.5 |
  1847066.6 |########################################
  1886769.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=111.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=89.9% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=46.0% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=109.8% of algo (FFI overhead may distort results)
- **nway**: CV=28.3% (high variance, measurements may be unstable)
- **nway**: bridge=116.3% of algo (FFI overhead may distort results)
- **sequential**: CV=65.6% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **sequential**: bridge=128.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=95.2% of algo (FFI overhead may distort results)

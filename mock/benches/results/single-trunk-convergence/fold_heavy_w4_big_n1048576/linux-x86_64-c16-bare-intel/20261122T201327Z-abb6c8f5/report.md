# fold_heavy, frames of tens of milliseconds, W = 4

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Two tiers: {head_tail_dyn, head_tail, kernel_nway, nway, std_parallel} vs {sequential, kernel_seq} (41% apart)

The field splits into a fast tier {head_tail_dyn, head_tail, kernel_nway, nway, std_parallel} and a slow tier {sequential, kernel_seq} with a 41% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### nway is inconsistent: worst-20% is 3.4x its best-20%

nway's best 20% of batches run at 1.10 ms but its worst 20% at 3.77 ms (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 1777448.6 ns median (-3.8% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.78x (fastest 1777448.6 ns, slowest 3156434.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, frames of tens of milliseconds, W = 4 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112425402 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 1970235ns | 1854909ns | 1670363ns | 1815813ns | 2351804ns | base |
| head_tail_dyn | 1839017ns | 1786119ns | 1701913ns | 1775690ns | 2002561ns | -6.66% |
| kernel_nway | 2117191ns | 2115010ns | 1926078ns | 2088558ns | 2255697ns | +7.46% |
| kernel_seq | 3160086ns | 3160966ns | 3129169ns | 3151108ns | 3189012ns | +60.39% |
| nway | 2346639ns | 2115823ns | 1100532ns | 1809074ns | 3776040ns | +19.10% |
| sequential | 3034557ns | 3032987ns | 3009223ns | 3030794ns | 3052868ns | +54.02% |
| std_parallel | 3573865ns | 2152155ns | 2072491ns | 2130962ns | 6488905ns | +81.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 1965857ns | 1669343ns | 2346105ns | base | 0.533 |
| head_tail_dyn | 1833400ns | 1701004ns | 1995251ns | -6.74% | 0.572 |
| kernel_nway | 2112005ns | 1920761ns | 2250838ns | +7.43% | 0.496 |
| kernel_seq | 3155199ns | 3127991ns | 3179834ns | +60.50% | 0.332 |
| nway | 2345316ns | 1099860ns | 3774628ns | +19.30% | 0.447 |
| sequential | 3034145ns | 3008733ns | 3052448ns | +54.34% | 0.346 |
| std_parallel | 3570649ns | 2071445ns | 6485182ns | +81.63% | 0.294 |

## Performance model

- Peak throughput: **0.953 Gops/s** (nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.567 | 59.5% |
| head_tail_dyn | 0.590 | 61.9% |
| kernel_nway | 0.497 | 52.2% |
| kernel_seq | 0.332 | 34.8% |
| nway | 0.496 | 52.0% |
| sequential | 0.346 | 36.3% |
| std_parallel | 0.488 | 51.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 1970235ns | 1970235ns | base |
| head_tail_dyn | 1839017ns | 1839017ns | -6.66% |
| kernel_nway | 2117191ns | 2117191ns | +7.46% |
| kernel_seq | 3160086ns | 3160086ns | +60.39% |
| nway | 2346639ns | 2346639ns | +19.10% |
| sequential | 3034557ns | 3034557ns | +54.02% |
| std_parallel | 3573865ns | 3573865ns | +81.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 1848408ns | base | --- | [1703058, 2346105] | --- | --- | --- | --- |
| head_tail_dyn | 1777449ns | no significant difference | [-528924, +226348]ns | [1727499, 1995251] | no | 0.6875 | 0.6875 | 0 |
| kernel_nway | 2108239ns | no significant difference | [-198133, +417431]ns | [1976938, 2250838] | no | 0.3281 | 0.2188 | 0 |
| kernel_seq | 3156434ns | +1307611.8ns (+70.7%) | [+828970, +1431445]ns | [3129330, 3179834] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| nway | 2114028ns | no significant difference | [-646383, +1631350]ns | [1147291, 3774628] | no | 0.6875 | 0.6875 | 0 |
| sequential | 3032586ns | +1195331.6ns (+64.7%) | [+685017, +1324516]ns | [3017403, 3052448] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| std_parallel | 2147152ns | no significant difference | [-159221, +4651776]ns | [2079614, 6485182] | no | 0.3281 | 0.2188 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2215757ns | -12.3% | -5.1% | +42.5% | +70.4% | +36.2% | -3.3% |
| 1 | 2 | 1793674ns | -1.6% | +17.7% | +76.0% | -36.0% | +69.1% | +175.3% |
| 1 | 3 | 1888140ns | -5.0% | +12.4% | +66.9% | +12.0% | +61.7% | +92.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.302 | moderate- |
| head_tail_dyn | -0.260 | moderate- |
| kernel_nway | -0.010 | ok |
| kernel_seq | -0.263 | moderate- |
| nway | -0.406 | moderate- |
| sequential | -0.004 | ok |
| std_parallel | -0.477 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 2118340.0ns | 1965856.9ns | 107.8% | HIGH |
| head_tail_dyn | 2289330.9ns | 1833399.6ns | 124.9% | HIGH |
| kernel_nway | 2133581.6ns | 2112005.0ns | 101.0% | HIGH |
| kernel_seq | 3176838.9ns | 3155199.4ns | 100.7% | HIGH |
| nway | 2376272.6ns | 2345315.6ns | 101.3% | HIGH |
| sequential | 3032472.6ns | 3034145.4ns | 99.9% | HIGH |
| std_parallel | 2517625.4ns | 3570649.2ns | 70.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 1669343.3-2346105.0 ns)
  1669343.3 |########################################
  1703181.4 |########################################
  1737019.5 |
  1770857.6 |########################################
  1804695.6 |
  1838533.7 |
  1872371.8 |
  1906209.9 |########################################
  1940048.0 |
  1973886.1 |########################################
  2007724.1 |
  2041562.2 |
  2075400.3 |
  2109238.4 |
  2143076.5 |
  2176914.6 |
  2210752.7 |
  2244590.7 |
  2278428.8 |
  2312266.9 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 1701003.8-1995251.5 ns)
  1701003.8 |####################
  1715716.2 |
  1730428.6 |
  1745140.9 |####################
  1759853.3 |
  1774565.7 |########################################
  1789278.1 |
  1803990.5 |
  1818702.9 |
  1833415.2 |
  1848127.6 |
  1862840.0 |
  1877552.4 |####################
  1892264.8 |
  1906977.2 |
  1921689.5 |
  1936401.9 |
  1951114.3 |
  1965826.7 |
  1980539.1 |
  (0 below, 1 above range)

kernel_nway (n=6, range 1920761.0-2250837.6 ns)
  1920761.0 |####################
  1937264.8 |
  1953768.7 |
  1970272.5 |
  1986776.3 |
  2003280.1 |
  2019784.0 |####################
  2036287.8 |####################
  2052791.6 |
  2069295.5 |
  2085799.3 |
  2102303.1 |
  2118807.0 |
  2135310.8 |
  2151814.6 |
  2168318.4 |########################################
  2184822.3 |
  2201326.1 |
  2217829.9 |
  2234333.8 |
  (0 below, 1 above range)

kernel_seq (n=6, range 3127991.3-3179833.5 ns)
  3127991.3 |########################################
  3130583.4 |########################################
  3133175.5 |
  3135767.6 |########################################
  3138359.8 |
  3140951.9 |
  3143544.0 |
  3146136.1 |
  3148728.2 |
  3151320.3 |
  3153912.4 |
  3156504.5 |
  3159096.6 |
  3161688.8 |
  3164280.9 |
  3166873.0 |
  3169465.1 |
  3172057.2 |########################################
  3174649.3 |########################################
  3177241.4 |
  (0 below, 1 above range)

nway (n=6, range 1099859.8-3774628.0 ns)
  1099859.8 |########################################
  1233598.2 |
  1367336.6 |
  1501075.0 |
  1634813.4 |
  1768551.8 |
  1902290.2 |####################
  2036028.7 |
  2169767.1 |####################
  2303505.5 |
  2437243.9 |
  2570982.3 |
  2704720.7 |####################
  2838459.1 |
  2972197.5 |
  3105935.9 |
  3239674.3 |
  3373412.7 |
  3507151.1 |
  3640889.5 |
  (0 below, 1 above range)

sequential (n=6, range 3008732.8-3052447.8 ns)
  3008732.8 |########################################
  3010918.5 |
  3013104.3 |
  3015290.0 |
  3017475.8 |
  3019661.5 |
  3021847.3 |
  3024033.0 |########################################
  3026218.8 |
  3028404.5 |########################################
  3030590.3 |
  3032776.0 |
  3034961.8 |########################################
  3037147.5 |
  3039333.3 |
  3041519.0 |
  3043704.8 |
  3045890.5 |
  3048076.3 |
  3050262.0 |########################################
  (0 below, 1 above range)

std_parallel (n=6, range 2071445.4-6485182.1 ns)
  2071445.4 |########################################
  2292132.2 |
  2512819.1 |
  2733505.9 |
  2954192.7 |
  3174879.6 |
  3395566.4 |
  3616253.2 |
  3836940.1 |
  4057626.9 |
  4278313.7 |
  4499000.6 |
  4719687.4 |
  4940374.2 |
  5161061.1 |##########
  5381747.9 |
  5602434.7 |
  5823121.6 |
  6043808.4 |
  6264495.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=107.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=99.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=103.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.5% of algo (FFI overhead may distort results)
- **nway**: CV=52.4% (high variance, measurements may be unstable)
- **nway**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **nway**: bridge=109.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.9% of algo (FFI overhead may distort results)
- **std_parallel**: CV=61.4% (high variance, measurements may be unstable)
- **std_parallel**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **std_parallel**: bridge=109.2% of algo (FFI overhead may distort results)

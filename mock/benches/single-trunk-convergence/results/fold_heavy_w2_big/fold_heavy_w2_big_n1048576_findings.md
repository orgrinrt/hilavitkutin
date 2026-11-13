# fold_heavy, frames of tens of milliseconds, W = 2

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail shows alternating (throttle bounce) (autocorr -0.66)

head_tail's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {std_parallel, head_tail, head_tail_dyn, kernel_nway, nway} vs {sequential, kernel_seq} (68% apart)

The field splits into a fast tier {std_parallel, head_tail, head_tail_dyn, kernel_nway, nway} and a slow tier {sequential, kernel_seq} with a 68% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader std_parallel vs stability leader head_tail_dyn (+8% speed for 1.7x steadier)

std_parallel is fastest (2.16 ms, CV 3.9%); head_tail_dyn gives up 7.9% median for 1.7x lower variance (CV 2.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### kernel_nway is inconsistent: worst-20% is 1.6x its best-20%

kernel_nway's best 20% of batches run at 2.06 ms but its worst 20% at 3.29 ms (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_parallel** at 2163554.2 ns median (-6.2% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.93x (fastest 2163554.2 ns, slowest 4175169.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, frames of tens of milliseconds, W = 2 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100342298 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 2263216ns | 2308331ns | 2055414ns | 2244728ns | 2394850ns | base |
| head_tail_dyn | 2325730ns | 2334816ns | 2237120ns | 2314891ns | 2386295ns | +2.76% |
| kernel_nway | 2579267ns | 2370089ns | 2059626ns | 2279722ns | 3288406ns | +13.96% |
| kernel_seq | 4386876ns | 4177883ns | 4007721ns | 4122294ns | 4973327ns | +93.83% |
| nway | 2496142ns | 2415590ns | 2065287ns | 2306574ns | 2995921ns | +10.29% |
| sequential | 4080081ns | 4039473ns | 3937774ns | 4008120ns | 4259176ns | +80.28% |
| std_parallel | 2199451ns | 2166730ns | 2088608ns | 2162085ns | 2310922ns | -2.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2260548ns | 2054319ns | 2390329ns | base | 0.464 |
| head_tail_dyn | 2322805ns | 2234125ns | 2381817ns | +2.75% | 0.451 |
| kernel_nway | 2576858ns | 2058873ns | 3285408ns | +13.99% | 0.407 |
| kernel_seq | 4383876ns | 4005857ns | 4969133ns | +93.93% | 0.239 |
| nway | 2490256ns | 2063275ns | 2994028ns | +10.16% | 0.421 |
| sequential | 4078384ns | 3936500ns | 4257539ns | +80.42% | 0.257 |
| std_parallel | 2196734ns | 2087546ns | 2307898ns | -2.82% | 0.477 |

## Performance model

- Peak throughput: **0.510 Gops/s** (head_tail; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.455 | 89.1% |
| head_tail_dyn | 0.449 | 88.0% |
| kernel_nway | 0.443 | 86.8% |
| kernel_seq | 0.251 | 49.2% |
| nway | 0.437 | 85.6% |
| sequential | 0.260 | 50.9% |
| std_parallel | 0.485 | 95.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 2263216ns | 2263216ns | base |
| head_tail_dyn | 2325730ns | 2325730ns | +2.76% |
| kernel_nway | 2579267ns | 2579267ns | +13.96% |
| kernel_seq | 4386876ns | 4386876ns | +93.83% |
| nway | 2496142ns | 2496142ns | +10.29% |
| sequential | 4080081ns | 4080081ns | +80.28% |
| std_parallel | 2199451ns | 2199451ns | -2.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2305673ns | base | --- | [2085642, 2390329] | --- | --- | --- | --- |
| head_tail_dyn | 2333494ns | no significant difference | [-137226, +272678]ns | [2253103, 2381817] | no | 0.6875 | 0.6875 | 0 |
| kernel_nway | 2368071ns | no significant difference | [-236998, +1045956]ns | [2077094, 3285408] | no | 0.6875 | 0.6875 | 0 |
| kernel_seq | 4175170ns | +1946707.8ns (+84.4%) | [+1715178, +2708099]ns | [4007327, 4969133] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| nway | 2401233ns | no significant difference | [-223782, +765928]ns | [2075507, 2994028] | no | 0.6875 | 0.6875 | 0 |
| sequential | 4037536ns | +1712548.4ns (+74.3%) | [+1569062, +2171897]ns | [3940077, 4257539] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| std_parallel | 2163554ns | no significant difference | [-234404, +144683]ns | [2118750, 2307898] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2228100ns | +6.9% | +6.3% | +80.5% | +34.4% | +87.0% | -4.6% |
| 1 | 2 | 2314092ns | -0.3% | -10.2% | +80.7% | +3.8% | +70.3% | -3.1% |
| 1 | 3 | 2239452ns | +1.8% | +46.7% | +120.9% | -7.3% | +84.4% | -0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.657 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.048 | ok |
| kernel_nway | -0.314 | moderate- |
| kernel_seq | -0.083 | ok |
| nway | -0.018 | ok |
| sequential | -0.651 | HIGH- (thermal bounce) |
| std_parallel | -0.273 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 2255198.4ns | 2260548.1ns | 99.8% | HIGH |
| head_tail_dyn | 2312521.4ns | 2322804.6ns | 99.6% | HIGH |
| kernel_nway | 2580230.7ns | 2576857.9ns | 100.1% | HIGH |
| kernel_seq | 4380264.8ns | 4383876.5ns | 99.9% | HIGH |
| nway | 2442422.6ns | 2490256.1ns | 98.1% | HIGH |
| sequential | 4046501.8ns | 4078384.0ns | 99.2% | HIGH |
| std_parallel | 2246061.9ns | 2196734.1ns | 102.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 2054319.1-2390329.5 ns)
  2054319.1 |########################################
  2071119.6 |
  2087920.1 |
  2104720.7 |########################################
  2121521.2 |
  2138321.7 |
  2155122.2 |
  2171922.7 |
  2188723.2 |
  2205523.8 |
  2222324.3 |
  2239124.8 |
  2255925.3 |########################################
  2272725.8 |
  2289526.3 |
  2306326.9 |
  2323127.4 |########################################
  2339927.9 |########################################
  2356728.4 |
  2373528.9 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 2234125.3-2381816.6 ns)
  2234125.3 |########################################
  2241509.9 |
  2248894.4 |
  2256279.0 |
  2263663.6 |
  2271048.1 |########################################
  2278432.7 |
  2285817.3 |
  2293201.8 |
  2300586.4 |
  2307971.0 |
  2315355.5 |
  2322740.1 |########################################
  2330124.7 |
  2337509.2 |########################################
  2344893.8 |
  2352278.4 |
  2359662.9 |
  2367047.5 |########################################
  2374432.1 |
  (0 below, 1 above range)

kernel_nway (n=6, range 2058873.3-3285408.1 ns)
  2058873.3 |########################################
  2120200.0 |
  2181526.8 |
  2242853.5 |
  2304180.3 |####################
  2365507.0 |####################
  2426833.7 |
  2488160.5 |
  2549487.2 |
  2610814.0 |
  2672140.7 |
  2733467.4 |
  2794794.2 |
  2856120.9 |
  2917447.7 |
  2978774.4 |
  3040101.1 |
  3101427.9 |####################
  3162754.6 |
  3224081.4 |
  (0 below, 1 above range)

kernel_seq (n=6, range 4005856.6-4969132.5 ns)
  4005856.6 |########################################
  4054020.4 |
  4102184.2 |
  4150348.0 |
  4198511.8 |
  4246675.6 |
  4294839.4 |#############
  4343003.2 |#############
  4391167.0 |
  4439330.8 |
  4487494.6 |
  4535658.4 |
  4583822.2 |
  4631986.0 |
  4680149.8 |
  4728313.6 |
  4776477.4 |
  4824641.2 |
  4872805.0 |
  4920968.8 |
  (0 below, 1 above range)

nway (n=6, range 2063275.0-2994028.0 ns)
  2063275.0 |########################################
  2109812.7 |
  2156350.3 |####################
  2202888.0 |
  2249425.6 |
  2295963.3 |
  2342500.9 |
  2389038.6 |
  2435576.2 |
  2482113.9 |
  2528651.5 |
  2575189.2 |####################
  2621726.8 |
  2668264.5 |
  2714802.1 |
  2761339.8 |
  2807877.4 |
  2854415.1 |
  2900952.7 |####################
  2947490.4 |
  (0 below, 1 above range)

sequential (n=6, range 3936499.6-4257539.1 ns)
  3936499.6 |########################################
  3952551.6 |
  3968603.5 |####################
  3984655.5 |
  4000707.5 |
  4016759.5 |
  4032811.4 |
  4048863.4 |
  4064915.4 |
  4080967.4 |
  4097019.3 |####################
  4113071.3 |
  4129123.3 |
  4145175.2 |
  4161227.2 |
  4177279.2 |
  4193331.2 |
  4209383.1 |
  4225435.1 |####################
  4241487.1 |
  (0 below, 1 above range)

std_parallel (n=6, range 2087546.4-2307897.8 ns)
  2087546.4 |########################################
  2098564.0 |
  2109581.5 |
  2120599.1 |
  2131616.7 |
  2142634.2 |########################################
  2153651.8 |########################################
  2164669.4 |########################################
  2175686.9 |
  2186704.5 |
  2197722.1 |
  2208739.6 |
  2219757.2 |
  2230774.8 |
  2241792.3 |
  2252809.9 |
  2263827.5 |
  2274845.0 |########################################
  2285862.6 |
  2296880.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=96.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=99.3% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=20.3% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=97.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.0% of algo (FFI overhead may distort results)
- **nway**: bridge=94.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=98.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=105.0% of algo (FFI overhead may distort results)

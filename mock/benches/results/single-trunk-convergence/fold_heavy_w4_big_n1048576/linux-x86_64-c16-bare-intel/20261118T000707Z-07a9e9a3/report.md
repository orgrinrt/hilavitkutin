# fold_heavy, frames of tens of milliseconds, W = 4

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq shows alternating (throttle bounce) (autocorr -0.51)

kernel_seq's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn, head_tail, kernel_nway, std_parallel, nway} vs {sequential, kernel_seq} (42% apart)

The field splits into a fast tier {head_tail_dyn, head_tail, kernel_nway, std_parallel, nway} and a slow tier {sequential, kernel_seq} with a 42% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### head_tail is inconsistent: worst-20% is 2.2x its best-20%

head_tail's best 20% of batches run at 1.69 ms but its worst 20% at 3.69 ms (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 1760645.7 ns median (-15.6% vs baseline)
- Spread: 1.79x (fastest 1760645.7 ns, slowest 3152704.8 ns)

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
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 2497391ns | 2094466ns | 1694573ns | 1964692ns | 3697849ns | base |
| head_tail_dyn | 1803273ns | 1766038ns | 1716637ns | 1750629ns | 1925556ns | -27.79% |
| kernel_nway | 2185822ns | 2124720ns | 1934294ns | 2116243ns | 2415954ns | -12.48% |
| kernel_seq | 3156599ns | 3159382ns | 3085270ns | 3148145ns | 3204944ns | +26.40% |
| nway | 2195429ns | 2158404ns | 1970758ns | 2125442ns | 2412745ns | -12.09% |
| sequential | 3054027ns | 3054442ns | 3047234ns | 3052277ns | 3060048ns | +22.29% |
| std_parallel | 2371352ns | 2130181ns | 2075640ns | 2115029ns | 2903692ns | -5.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2491754ns | 1691386ns | 3691955ns | base | 0.421 |
| head_tail_dyn | 1799017ns | 1715807ns | 1919111ns | -27.80% | 0.583 |
| kernel_nway | 2182081ns | 1930916ns | 2410131ns | -12.43% | 0.481 |
| kernel_seq | 3150859ns | 3084520ns | 3195140ns | +26.45% | 0.333 |
| nway | 2194500ns | 1968796ns | 2411952ns | -11.93% | 0.478 |
| sequential | 3053630ns | 3046889ns | 3059570ns | +22.55% | 0.343 |
| std_parallel | 2369272ns | 2074918ns | 2901239ns | -4.92% | 0.443 |

## Performance model

- Peak throughput: **0.620 Gops/s** (head_tail; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.503 | 81.1% |
| head_tail_dyn | 0.596 | 96.1% |
| kernel_nway | 0.494 | 79.7% |
| kernel_seq | 0.333 | 53.6% |
| nway | 0.486 | 78.4% |
| sequential | 0.343 | 55.4% |
| std_parallel | 0.493 | 79.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 2497391ns | 2497391ns | base |
| head_tail_dyn | 1803273ns | 1803273ns | -27.79% |
| kernel_nway | 2185822ns | 2185822ns | -12.48% |
| kernel_seq | 3156599ns | 3156599ns | +26.40% |
| nway | 2195429ns | 2195429ns | -12.09% |
| sequential | 3054027ns | 3054027ns | +22.29% |
| std_parallel | 2371352ns | 2371352ns | -5.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2085465ns | base | --- | [1697841, 3691955] | --- | --- | --- | --- |
| head_tail_dyn | 1760646ns | no significant difference | [-1841410, +29381]ns | [1717295, 1919111] | no | 0.8250 | 0.6875 | 0 |
| kernel_nway | 2123096ns | no significant difference | [-1485049, +382368]ns | [2013016, 2410131] | no | 0.8250 | 0.6875 | 0 |
| kernel_seq | 3152705ns | no significant difference | [-503466, +1406892]ns | [3104733, 3195140] | no | 0.8250 | 0.6875 | 0 |
| nway | 2157673ns | no significant difference | [-1298920, +316034]ns | [2013876, 2411952] | no | 0.8250 | 0.6875 | 0 |
| sequential | 3054108ns | no significant difference | [-638584, +1356267]ns | [3047212, 3059570] | no | 0.8250 | 0.6875 | 0 |
| std_parallel | 2127357ns | no significant difference | [-1585664, +1148953]ns | [2079221, 2901239] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 3691955ns | -49.9% | -40.2% | -13.6% | -35.2% | -17.3% | -42.9% |
| 1 | 2 | 1937497ns | -7.8% | +14.8% | +60.6% | +11.7% | +57.4% | +8.4% |
| 1 | 3 | 1845810ns | -4.6% | +14.6% | +70.8% | +9.8% | +65.7% | +57.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.142 | ok |
| head_tail_dyn | -0.040 | ok |
| kernel_nway | -0.261 | moderate- |
| kernel_seq | -0.513 | HIGH- (thermal bounce) |
| nway | -0.014 | ok |
| sequential | -0.471 | moderate- |
| std_parallel | -0.170 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 2727633.0ns | 2491753.8ns | 109.5% | HIGH |
| head_tail_dyn | 1808155.1ns | 1799017.4ns | 100.5% | HIGH |
| kernel_nway | 2842153.8ns | 2182081.1ns | 130.2% | HIGH |
| kernel_seq | 3200117.1ns | 3150859.3ns | 101.6% | HIGH |
| nway | 3205734.5ns | 2194500.2ns | 146.1% | HIGH |
| sequential | 3052157.2ns | 3053630.0ns | 100.0% | HIGH |
| std_parallel | 2501387.6ns | 2369272.1ns | 105.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 1691385.5-3691955.0 ns)
  1691385.5 |########################################
  1791414.0 |
  1891442.4 |####################
  1991470.9 |
  2091499.4 |####################
  2191527.9 |
  2291556.4 |
  2391584.8 |
  2491613.3 |
  2591641.8 |
  2691670.2 |
  2791698.7 |
  2891727.2 |
  2991755.7 |
  3091784.2 |
  3191812.6 |
  3291841.1 |
  3391869.6 |
  3491898.0 |
  3591926.5 |####################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 1715806.6-1919111.3 ns)
  1715806.6 |########################################
  1725971.8 |
  1736137.1 |####################
  1746302.3 |
  1756467.5 |
  1766632.8 |
  1776798.0 |####################
  1786963.2 |
  1797128.5 |
  1807293.7 |
  1817458.9 |
  1827624.2 |
  1837789.4 |
  1847954.7 |####################
  1858119.9 |
  1868285.1 |
  1878450.4 |
  1888615.6 |
  1898780.8 |
  1908946.1 |
  (0 below, 1 above range)

kernel_nway (n=6, range 1930915.8-2410131.0 ns)
  1930915.8 |########################################
  1954876.6 |
  1978837.3 |
  2002798.1 |
  2026758.8 |
  2050719.6 |
  2074680.3 |########################################
  2098641.1 |########################################
  2122601.9 |########################################
  2146562.6 |
  2170523.4 |
  2194484.1 |
  2218444.9 |
  2242405.6 |
  2266366.4 |
  2290327.2 |########################################
  2314287.9 |
  2338248.7 |
  2362209.4 |
  2386170.2 |
  (0 below, 1 above range)

kernel_seq (n=6, range 3084520.2-3195139.9 ns)
  3084520.2 |########################################
  3090051.2 |
  3095582.2 |
  3101113.1 |
  3106644.1 |
  3112175.1 |
  3117706.1 |
  3123237.1 |########################################
  3128768.1 |
  3134299.0 |
  3139830.0 |########################################
  3145361.0 |
  3150892.0 |
  3156423.0 |
  3161954.0 |########################################
  3167484.9 |
  3173015.9 |
  3178546.9 |########################################
  3184077.9 |
  3189608.9 |
  (0 below, 1 above range)

nway (n=6, range 1968796.2-2411951.9 ns)
  1968796.2 |########################################
  1990954.0 |
  2013111.8 |
  2035269.6 |
  2057427.3 |########################################
  2079585.1 |########################################
  2101742.9 |
  2123900.7 |
  2146058.5 |
  2168216.3 |
  2190374.0 |
  2212531.8 |########################################
  2234689.6 |
  2256847.4 |########################################
  2279005.2 |
  2301163.0 |
  2323320.8 |
  2345478.5 |
  2367636.3 |
  2389794.1 |
  (0 below, 1 above range)

sequential (n=6, range 3046889.2-3059570.2 ns)
  3046889.2 |########################################
  3047523.3 |########################################
  3048157.3 |
  3048791.4 |
  3049425.4 |
  3050059.5 |
  3050693.5 |########################################
  3051327.6 |
  3051961.6 |
  3052595.7 |
  3053229.7 |
  3053863.8 |
  3054497.8 |
  3055131.9 |
  3055765.9 |
  3056400.0 |
  3057034.0 |########################################
  3057668.1 |
  3058302.1 |
  3058936.2 |########################################
  (0 below, 1 above range)

std_parallel (n=6, range 2074918.1-2901238.6 ns)
  2074918.1 |########################################
  2116234.1 |########################################
  2157550.2 |####################
  2198866.2 |
  2240182.2 |
  2281498.2 |
  2322814.3 |
  2364130.3 |
  2405446.3 |
  2446762.3 |
  2488078.4 |
  2529394.4 |
  2570710.4 |
  2612026.5 |
  2653342.5 |
  2694658.5 |
  2735974.5 |
  2777290.6 |
  2818606.6 |
  2859922.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=34.8% (high variance, measurements may be unstable)
- **head_tail**: bridge=108.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=99.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=111.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=101.1% of algo (FFI overhead may distort results)
- **nway**: bridge=105.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.9% of algo (FFI overhead may distort results)
- **std_parallel**: CV=23.6% (high variance, measurements may be unstable)
- **std_parallel**: bridge=99.3% of algo (FFI overhead may distort results)

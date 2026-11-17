# append, far above cache, W = 8

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential is an outlier: 5.1x slower than the field

sequential (11.66 ms) is 5.1x the fastest (2.27 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (head_tail, head_tail_dyn) are a dead heat (<1%)

head_tail (2.27 ms) and head_tail_dyn (2.29 ms) differ by 0.88%, inside the noise, even though the wider field spreads 414.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### std_parallel shows alternating (throttle bounce) (autocorr -0.59)

std_parallel's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (2.27 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, head_tail_dyn, kernel_seq, nway, kernel_nway, std_parallel} vs {sequential} (297% apart)

The field splits into a fast tier {head_tail, head_tail_dyn, kernel_seq, nway, kernel_nway, std_parallel} and a slow tier {sequential} with a 297% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.1x the fastest

Fastest head_tail (2.27 ms) to slowest sequential (11.66 ms): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (head_tail) is the fastest** at 2265401.0 ns median
- 4 variants significantly slower than baseline
- Spread: 5.15x (fastest 2265401.0 ns, slowest 11661099.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, far above cache, W = 8 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 2326759ns | 2269370ns | 2257490ns | 2266635ns | 2451578ns | base |
| head_tail_dyn | 2417431ns | 2288729ns | 2249885ns | 2283677ns | 2701833ns | +3.90% |
| kernel_nway | 2927698ns | 2924682ns | 2842219ns | 2902458ns | 3008297ns | +25.83% |
| kernel_seq | 2338955ns | 2331188ns | 2251500ns | 2312844ns | 2421849ns | +0.52% |
| nway | 2943833ns | 2920844ns | 2864594ns | 2903924ns | 3043318ns | +26.52% |
| sequential | 11683995ns | 11668547ns | 11585667ns | 11656278ns | 11774734ns | +402.16% |
| std_parallel | 2989823ns | 2936891ns | 2899552ns | 2932837ns | 3120438ns | +28.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2323010ns | 2254406ns | 2447807ns | base | 7.222 |
| head_tail_dyn | 2414231ns | 2246740ns | 2699083ns | +3.93% | 6.949 |
| kernel_nway | 2923010ns | 2837198ns | 3002974ns | +25.83% | 5.740 |
| kernel_seq | 2335708ns | 2248083ns | 2418188ns | +0.55% | 7.183 |
| nway | 2939097ns | 2860260ns | 3038594ns | +26.52% | 5.708 |
| sequential | 11676276ns | 11572167ns | 11769724ns | +402.64% | 1.437 |
| std_parallel | 2986359ns | 2896396ns | 3116490ns | +28.56% | 5.618 |

## Performance model

- Peak throughput: **7.467 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 7.406 | 99.2% |
| head_tail_dyn | 7.341 | 98.3% |
| kernel_nway | 5.744 | 76.9% |
| kernel_seq | 7.206 | 96.5% |
| nway | 5.752 | 77.0% |
| sequential | 1.439 | 19.3% |
| std_parallel | 5.719 | 76.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 2326759ns | 2326759ns | base |
| head_tail_dyn | 2417431ns | 2417431ns | +3.90% |
| kernel_nway | 2927698ns | 2927698ns | +25.83% |
| kernel_seq | 2338955ns | 2338955ns | +0.52% |
| nway | 2943833ns | 2943833ns | +26.52% |
| sequential | 11683995ns | 11683995ns | +402.16% |
| std_parallel | 2989823ns | 2989823ns | +28.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2265401ns | base | --- | [2255823, 2447807] | --- | --- | --- | --- |
| head_tail_dyn | 2285344ns | no significant difference | [-139302, +383443]ns | [2258266, 2699083] | no | 0.6875 | 0.6875 | 0 |
| kernel_nway | 2920755ns | +604338.5ns (+26.7%) | [+465234, +730427]ns | [2845302, 3002974] | YES | 0.0469 | 0.0313 | 0 |
| kernel_seq | 2328156ns | no significant difference | [-109990, +107198]ns | [2260781, 2418188] | no | 0.6875 | 0.6875 | 0 |
| nway | 2916615ns | +596682.2ns (+26.3%) | [+542276, +709302]ns | [2862083, 3038594] | YES | 0.0469 | 0.0313 | 0 |
| sequential | 11661099ns | +9334489.7ns (+412.0%) | [+9252266, +9473042]ns | [11598005, 11769724] | YES | 0.0469 | 0.0313 | 0 |
| std_parallel | 2933729ns | +663692.7ns (+29.3%) | [+640891, +685464]ns | [2908859, 3116490] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2315172ns | +7.1% | +26.4% | -1.3% | +26.0% | +406.8% | +29.4% |
| 1 | 2 | 2265401ns | +10.0% | +26.7% | +4.7% | +26.3% | +412.7% | +29.5% |
| 1 | 3 | 2388458ns | -4.9% | +24.4% | -1.6% | +27.2% | +389.0% | +26.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.433 | moderate- |
| head_tail_dyn | -0.198 | ok |
| kernel_nway | -0.538 | HIGH- (thermal bounce) |
| kernel_seq | -0.320 | moderate- |
| nway | -0.363 | moderate- |
| sequential | -0.533 | HIGH- (thermal bounce) |
| std_parallel | -0.588 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 91903414.9ns | 2323010.4ns | 3956.2% | HIGH |
| head_tail_dyn | 92025421.9ns | 2414230.9ns | 3811.8% | HIGH |
| kernel_nway | 92834019.1ns | 2923010.4ns | 3176.0% | HIGH |
| kernel_seq | 92024135.4ns | 2335708.3ns | 3939.9% | HIGH |
| nway | 92637411.4ns | 2939097.2ns | 3151.9% | HIGH |
| sequential | 101809602.4ns | 11676276.1ns | 871.9% | HIGH |
| std_parallel | 92509777.8ns | 2986359.4ns | 3097.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 2254406.2-2447807.3 ns)
  2254406.2 |########################################
  2264076.3 |#############
  2273746.3 |
  2283416.4 |
  2293086.4 |
  2302756.5 |
  2312426.5 |
  2322096.6 |
  2331766.6 |
  2341436.7 |
  2351106.8 |
  2360776.8 |
  2370446.9 |#############
  2380116.9 |
  2389787.0 |
  2399457.0 |
  2409127.1 |
  2418797.1 |
  2428467.2 |
  2438137.2 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 2246739.6-2699083.4 ns)
  2246739.6 |####################
  2269356.8 |########################################
  2291974.0 |####################
  2314591.2 |
  2337208.4 |
  2359825.5 |
  2382442.7 |
  2405059.9 |
  2427677.1 |
  2450294.3 |
  2472911.5 |
  2495528.7 |
  2518145.9 |
  2540763.0 |
  2563380.2 |
  2585997.4 |
  2608614.6 |
  2631231.8 |
  2653849.0 |####################
  2676466.2 |
  (0 below, 1 above range)

kernel_nway (n=6, range 2837197.9-3002974.0 ns)
  2837197.9 |########################################
  2845486.7 |########################################
  2853775.5 |
  2862064.3 |
  2870353.1 |
  2878641.9 |########################################
  2886930.7 |
  2895219.5 |
  2903508.3 |
  2911797.1 |
  2920085.9 |
  2928374.7 |
  2936663.5 |
  2944952.3 |
  2953241.1 |########################################
  2961529.9 |
  2969818.7 |
  2978107.5 |
  2986396.3 |########################################
  2994685.1 |
  (0 below, 1 above range)

kernel_seq (n=6, range 2248083.3-2418187.5 ns)
  2248083.3 |########################################
  2256588.5 |
  2265093.7 |########################################
  2273598.9 |
  2282104.1 |
  2290609.3 |
  2299114.6 |
  2307619.8 |
  2316125.0 |########################################
  2324630.2 |
  2333135.4 |########################################
  2341640.6 |
  2350145.8 |
  2358651.0 |
  2367156.2 |
  2375661.5 |
  2384166.7 |
  2392671.9 |
  2401177.1 |########################################
  2409682.3 |
  (0 below, 1 above range)

nway (n=6, range 2860260.4-3038593.8 ns)
  2860260.4 |########################################
  2869177.1 |####################
  2878093.7 |
  2887010.4 |
  2895927.1 |
  2904843.7 |
  2913760.4 |
  2922677.1 |
  2931593.7 |
  2940510.4 |
  2949427.1 |####################
  2958343.7 |
  2967260.4 |####################
  2976177.1 |
  2985093.7 |
  2994010.4 |
  3002927.1 |
  3011843.7 |
  3020760.4 |
  3029677.1 |
  (0 below, 1 above range)

sequential (n=6, range 11572166.7-11769723.9 ns)
  11572166.7 |########################################
  11582044.6 |
  11591922.4 |
  11601800.3 |
  11611678.1 |
  11621556.0 |########################################
  11631433.9 |
  11641311.7 |
  11651189.6 |########################################
  11661067.5 |########################################
  11670945.3 |
  11680823.2 |
  11690701.0 |
  11700578.9 |
  11710456.8 |
  11720334.6 |
  11730212.5 |########################################
  11740090.4 |
  11749968.2 |
  11759846.1 |
  (0 below, 1 above range)

std_parallel (n=6, range 2896395.8-3116489.6 ns)
  2896395.8 |####################
  2907400.5 |
  2918405.2 |####################
  2929409.9 |########################################
  2940414.6 |
  2951419.2 |
  2962423.9 |
  2973428.6 |
  2984433.3 |
  2995438.0 |
  3006442.7 |
  3017447.4 |
  3028452.1 |
  3039456.8 |
  3050461.5 |
  3061466.1 |####################
  3072470.8 |
  3083475.5 |
  3094480.2 |
  3105484.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=4055.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=4014.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=3175.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3949.4% of algo (FFI overhead may distort results)
- **nway**: bridge=3169.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=871.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=3147.8% of algo (FFI overhead may distort results)

# append, far above cache, W = 4

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential is an outlier: 5.2x slower than the field

sequential (11.67 ms) is 5.2x the fastest (2.27 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.56)

kernel_nway's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn, kernel_seq, kernel_nway, head_tail, std_parallel, nway} vs {sequential} (258% apart)

The field splits into a fast tier {head_tail_dyn, kernel_seq, kernel_nway, head_tail, std_parallel, nway} and a slow tier {sequential} with a 258% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.2x the fastest

Fastest head_tail_dyn (2.27 ms) to slowest sequential (11.67 ms): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: head_tail_dyn** at 2265177.1 ns median (-6.8% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 5.15x (fastest 2265177.1 ns, slowest 11673276.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, far above cache, W = 4 |
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
| head_tail | 2478741ns | 2433734ns | 2250000ns | 2373910ns | 2750359ns | base |
| head_tail_dyn | 2362990ns | 2268286ns | 2256146ns | 2264337ns | 2564391ns | -4.67% |
| kernel_nway | 2547372ns | 2429938ns | 2294823ns | 2392681ns | 2905682ns | +2.77% |
| kernel_seq | 2434319ns | 2300354ns | 2249698ns | 2284264ns | 2751714ns | -1.79% |
| nway | 3407181ns | 3269292ns | 3149885ns | 3237833ns | 3789849ns | +37.46% |
| sequential | 11710012ns | 11678875ns | 11482656ns | 11637698ns | 11932162ns | +372.42% |
| std_parallel | 2505332ns | 2482026ns | 2304854ns | 2423892ns | 2727729ns | +1.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2475340ns | 2246719ns | 2747125ns | base | 6.778 |
| head_tail_dyn | 2359783ns | 2252635ns | 2561448ns | -4.67% | 7.110 |
| kernel_nway | 2543062ns | 2290708ns | 2901198ns | +2.74% | 6.597 |
| kernel_seq | 2431247ns | 2246219ns | 2748859ns | -1.78% | 6.901 |
| nway | 3400035ns | 3141573ns | 3780578ns | +37.36% | 4.934 |
| sequential | 11703807ns | 11475865ns | 11924943ns | +372.82% | 1.433 |
| std_parallel | 2501576ns | 2300948ns | 2724448ns | +1.06% | 6.707 |

## Performance model

- Peak throughput: **7.469 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 6.904 | 92.4% |
| head_tail_dyn | 7.407 | 99.2% |
| kernel_nway | 6.915 | 92.6% |
| kernel_seq | 7.303 | 97.8% |
| nway | 5.142 | 68.8% |
| sequential | 1.437 | 19.2% |
| std_parallel | 6.770 | 90.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 2478741ns | 2478741ns | base |
| head_tail_dyn | 2362990ns | 2362990ns | -4.67% |
| kernel_nway | 2547372ns | 2547372ns | +2.77% |
| kernel_seq | 2434319ns | 2434319ns | -1.79% |
| nway | 3407181ns | 3407181ns | +37.46% |
| sequential | 11710012ns | 11710012ns | +372.42% |
| std_parallel | 2505332ns | 2505332ns | +1.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2429901ns | base | --- | [2248995, 2747125] | --- | --- | --- | --- |
| head_tail_dyn | 2265177ns | no significant difference | [-277115, +16182]ns | [2252724, 2561448] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 2426302ns | no significant difference | [-130552, +253250]ns | [2301688, 2901198] | no | 0.8250 | 0.6875 | 0 |
| kernel_seq | 2297312ns | no significant difference | [-132589, +21240]ns | [2247568, 2748859] | no | 0.4375 | 0.2188 | 0 |
| nway | 3263010ns | +823323.0ns (+33.9%) | [+609161, +1341599]ns | [3156516, 3780578] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| sequential | 11673276ns | +9227057.3ns (+379.7%) | [+9144198, +9314146]ns | [11513203, 11924943] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| std_parallel | 2478172ns | no significant difference | [-145630, +163990]ns | [2302109, 2724448] | no | 0.8250 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2747125ns | -6.8% | +0.3% | +0.1% | +32.4% | +333.7% | -0.8% |
| 1 | 2 | 2339094ns | -3.2% | -1.6% | -1.9% | +34.9% | +395.0% | -1.2% |
| 1 | 3 | 2339802ns | -3.7% | +9.9% | -3.9% | +45.6% | +396.6% | +5.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.168 | ok |
| head_tail_dyn | -0.151 | ok |
| kernel_nway | -0.560 | HIGH- (thermal bounce) |
| kernel_seq | -0.121 | ok |
| nway | -0.513 | HIGH- (thermal bounce) |
| sequential | -0.234 | moderate- |
| std_parallel | -0.419 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 2/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 2/3, lost 1/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 92479828.1ns | 2475340.3ns | 3736.0% | HIGH |
| head_tail_dyn | 93435539.9ns | 2359783.0ns | 3959.5% | HIGH |
| kernel_nway | 92781197.9ns | 2543062.5ns | 3648.4% | HIGH |
| kernel_seq | 92397227.5ns | 2431246.6ns | 3800.4% | HIGH |
| nway | 93404673.6ns | 3400034.7ns | 2747.2% | HIGH |
| sequential | 101960064.2ns | 11703807.3ns | 871.2% | HIGH |
| std_parallel | 92252534.7ns | 2501576.4ns | 3687.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 2246718.8-2747125.0 ns)
  2246718.8 |########################################
  2271739.1 |
  2296759.4 |
  2321779.7 |
  2346800.0 |
  2371820.3 |
  2396840.7 |
  2421861.0 |########################################
  2446881.3 |
  2471901.6 |
  2496921.9 |
  2521942.2 |
  2546962.5 |
  2571982.8 |
  2597003.1 |
  2622023.5 |####################
  2647043.8 |
  2672064.1 |
  2697084.4 |
  2722104.7 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 2252635.4-2561447.9 ns)
  2252635.4 |########################################
  2268076.0 |#############
  2283516.6 |
  2298957.3 |
  2314397.9 |
  2329838.5 |
  2345279.1 |
  2360719.8 |
  2376160.4 |
  2391601.0 |
  2407041.7 |
  2422482.3 |
  2437922.9 |
  2453363.5 |
  2468804.2 |
  2484244.8 |#############
  2499685.4 |
  2515126.0 |
  2530566.7 |
  2546007.3 |
  (0 below, 1 above range)

kernel_nway (n=6, range 2290708.3-2901197.9 ns)
  2290708.3 |########################################
  2321232.8 |
  2351757.3 |####################
  2382281.7 |
  2412806.2 |
  2443330.7 |
  2473855.2 |####################
  2504379.7 |
  2534904.1 |
  2565428.6 |
  2595953.1 |
  2626477.6 |
  2657002.1 |
  2687526.5 |
  2718051.0 |
  2748575.5 |####################
  2779100.0 |
  2809624.5 |
  2840148.9 |
  2870673.4 |
  (0 below, 1 above range)

kernel_seq (n=6, range 2246218.8-2748859.4 ns)
  2246218.8 |########################################
  2271350.8 |
  2296482.9 |
  2321614.9 |#############
  2346746.9 |
  2371878.9 |
  2397011.0 |
  2422143.0 |
  2447275.0 |
  2472407.1 |
  2497539.1 |
  2522671.1 |
  2547803.2 |
  2572935.2 |
  2598067.2 |
  2623199.2 |
  2648331.3 |
  2673463.3 |#############
  2698595.3 |
  2723727.4 |
  (0 below, 1 above range)

nway (n=6, range 3141572.9-3780578.1 ns)
  3141572.9 |########################################
  3173523.2 |####################
  3205473.4 |
  3237423.7 |
  3269373.9 |
  3301324.2 |
  3333274.5 |####################
  3365224.7 |
  3397175.0 |
  3429125.2 |
  3461075.5 |
  3493025.8 |
  3524976.0 |
  3556926.3 |
  3588876.5 |
  3620826.8 |####################
  3652777.1 |
  3684727.3 |
  3716677.6 |
  3748627.8 |
  (0 below, 1 above range)

sequential (n=6, range 11475864.6-11924942.7 ns)
  11475864.6 |########################################
  11498318.5 |
  11520772.4 |
  11543226.3 |########################################
  11565680.2 |
  11588134.1 |########################################
  11610588.0 |
  11633041.9 |
  11655495.8 |
  11677949.7 |
  11700403.6 |
  11722857.6 |########################################
  11745311.5 |########################################
  11767765.4 |
  11790219.3 |
  11812673.2 |
  11835127.1 |
  11857581.0 |
  11880034.9 |
  11902488.8 |
  (0 below, 1 above range)

std_parallel (n=6, range 2300947.9-2724447.9 ns)
  2300947.9 |########################################
  2322122.9 |
  2343297.9 |
  2364472.9 |
  2385647.9 |
  2406822.9 |
  2427997.9 |
  2449172.9 |
  2470347.9 |
  2491522.9 |
  2512697.9 |
  2533872.9 |
  2555047.9 |
  2576222.9 |
  2597397.9 |
  2618572.9 |#############
  2639747.9 |
  2660922.9 |
  2682097.9 |#############
  2703272.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=3798.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=4076.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=3821.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=4042.4% of algo (FFI overhead may distort results)
- **nway**: bridge=2864.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=873.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=3726.7% of algo (FFI overhead may distort results)

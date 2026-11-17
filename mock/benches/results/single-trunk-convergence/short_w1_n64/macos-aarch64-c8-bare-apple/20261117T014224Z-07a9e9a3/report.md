# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 7.7x slower than the field

std_parallel (19.05 us) is 7.7x the fastest (2.48 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (head_tail_dyn, kernel_seq) are a dead heat (<1%)

head_tail_dyn (2.48 us) and kernel_seq (2.49 us) differ by 0.24%, inside the noise, even though the wider field spreads 667.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail shows alternating (throttle bounce) (autocorr -0.67)

head_tail's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn, kernel_seq, kernel_nway, head_tail} vs {sequential, nway, std_parallel} (308% apart)

The field splits into a fast tier {head_tail_dyn, kernel_seq, kernel_nway, head_tail} and a slow tier {sequential, nway, std_parallel} with a 308% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.7x the fastest

Fastest head_tail_dyn (2.48 us) to slowest std_parallel (19.05 us): 7.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader head_tail_dyn vs stability leader kernel_nway (+1% speed for 2.3x steadier)

head_tail_dyn is fastest (2.48 us, CV 3.6%); kernel_nway gives up 1.4% median for 2.3x lower variance (CV 1.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### kernel_seq's edge over baseline is significant but tiny (-38 ns, 1.51%)

kernel_seq differs from baseline head_tail by -38 ns (1.51%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: head_tail_dyn** at 2483.3 ns median (-2.0% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 7.67x (fastest 2483.3 ns, slowest 19049.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 1 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 3042ns | 2809ns | 2742ns | 2859ns | 3466ns | base |
| head_tail_dyn | 2863ns | 2811ns | 2653ns | 2799ns | 3064ns | -5.88% |
| kernel_nway | 2812ns | 2778ns | 2676ns | 2769ns | 2943ns | -7.56% |
| kernel_seq | 2815ns | 2787ns | 2711ns | 2776ns | 2924ns | -7.46% |
| nway | 12467ns | 12368ns | 12180ns | 12325ns | 12822ns | +309.87% |
| sequential | 10501ns | 10488ns | 9855ns | 10344ns | 11060ns | +245.25% |
| std_parallel | 19553ns | 19395ns | 18036ns | 19185ns | 20862ns | +542.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2632ns | 2462ns | 2876ns | base | 0.024 |
| head_tail_dyn | 2504ns | 2430ns | 2584ns | -4.87% | 0.026 |
| kernel_nway | 2511ns | 2440ns | 2551ns | -4.61% | 0.025 |
| kernel_seq | 2496ns | 2436ns | 2548ns | -5.18% | 0.026 |
| nway | 12170ns | 11947ns | 12468ns | +362.29% | 0.005 |
| sequential | 10363ns | 9723ns | 10920ns | +293.67% | 0.006 |
| std_parallel | 19200ns | 17686ns | 20489ns | +629.36% | 0.003 |

## Performance model

- Peak throughput: **0.026 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.025 | 95.9% |
| head_tail_dyn | 0.026 | 97.9% |
| kernel_nway | 0.025 | 96.5% |
| kernel_seq | 0.026 | 97.6% |
| nway | 0.005 | 20.1% |
| sequential | 0.006 | 23.5% |
| std_parallel | 0.003 | 12.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 3042ns | 3042ns | base |
| head_tail_dyn | 2863ns | 2863ns | -5.88% |
| kernel_nway | 2812ns | 2812ns | -7.56% |
| kernel_seq | 2815ns | 2815ns | -7.46% |
| nway | 12467ns | 12467ns | +309.87% |
| sequential | 10501ns | 10501ns | +245.25% |
| std_parallel | 19553ns | 19553ns | +542.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2534ns | base | --- | [2499, 2740] | --- | --- | --- | --- |
| head_tail_dyn | 2483ns | no significant difference | [-232, +2]ns | [2457, 2510] | no | 0.1722 | 0.1435 | 1 |
| kernel_nway | 2519ns | no significant difference | [-237, +24]ns | [2498, 2532] | no | 0.3323 | 0.3323 | 1 |
| kernel_seq | 2489ns | -38.3ns (-1.5%) | [-212, -11]ns | [2467, 2521] | YES | 0.0191 | 0.0127 | 1 |
| nway | 12074ns | +9494.9ns (+374.7%) | [+9327, +9603]ns | [12009, 12136] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10344ns | +7563.3ns (+298.5%) | [+7342, +7978]ns | [10048, 10478] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 19049ns | +16472.6ns (+650.0%) | [+15858, +17059]ns | [18468, 19869] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2508ns | +0.1% | +0.9% | -1.0% | +384.5% | +294.6% | +652.6% |
| 1 | 2 | 2781ns | -10.3% | -9.6% | -9.9% | +333.4% | +268.2% | +595.5% |
| 1 | 3 | 2514ns | -2.3% | -0.4% | -0.9% | +379.4% | +327.1% | +655.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.666 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.044 | ok |
| kernel_nway | -0.027 | ok |
| kernel_seq | -0.563 | HIGH- (thermal bounce) |
| nway | -0.167 | ok |
| sequential | -0.008 | ok |
| std_parallel | -0.626 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 0/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 93002.5ns | 2632.5ns | 3532.9% | HIGH |
| head_tail_dyn | 90861.9ns | 2504.3ns | 3628.3% | HIGH |
| kernel_nway | 89376.8ns | 2511.1ns | 3559.3% | HIGH |
| kernel_seq | 90536.8ns | 2496.0ns | 3627.3% | HIGH |
| nway | 92826.3ns | 12169.6ns | 762.8% | HIGH |
| sequential | 90500.8ns | 10363.3ns | 873.3% | HIGH |
| std_parallel | 96949.8ns | 19200.3ns | 504.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 2462.2-2876.4 ns)
   2462.2 |########################################
   2482.9 |##########################
   2503.6 |#############
   2524.3 |########################################
   2545.0 |#############
   2565.7 |
   2586.5 |#############
   2607.2 |
   2627.9 |
   2648.6 |
   2669.3 |
   2690.0 |
   2710.7 |
   2731.4 |##########################
   2752.1 |
   2772.8 |
   2793.5 |#############
   2814.3 |
   2835.0 |
   2855.7 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 2430.0-2584.2 ns)
   2430.0 |
   2437.7 |
   2445.4 |##########
   2453.1 |########################################
   2460.8 |
   2468.5 |
   2476.2 |####################
   2484.0 |##########
   2491.7 |
   2499.4 |##############################
   2507.1 |##########
   2514.8 |
   2522.5 |
   2530.2 |####################
   2537.9 |
   2545.6 |
   2553.3 |
   2561.0 |
   2568.8 |
   2576.5 |##########
  (2 below, 1 above range)

kernel_nway (n=18, range 2440.0-2551.4 ns)
   2440.0 |
   2445.6 |
   2451.2 |#############
   2456.7 |
   2462.3 |
   2467.9 |
   2473.4 |#############
   2479.0 |
   2484.6 |
   2490.1 |#############
   2495.7 |#############
   2501.3 |#############
   2506.8 |#############
   2512.4 |
   2518.0 |########################################
   2523.5 |##########################
   2529.1 |
   2534.7 |#############
   2540.2 |#############
   2545.8 |#############
  (2 below, 2 above range)

kernel_seq (n=18, range 2435.6-2547.8 ns)
   2435.6 |
   2441.2 |
   2446.8 |####################
   2452.4 |####################
   2458.0 |####################
   2463.6 |
   2469.2 |####################
   2474.8 |####################
   2480.4 |####################
   2486.1 |####################
   2491.7 |####################
   2497.3 |####################
   2502.9 |
   2508.5 |####################
   2514.1 |
   2519.7 |########################################
   2525.3 |
   2530.9 |####################
   2536.5 |########################################
   2542.2 |
  (2 below, 1 above range)

nway (n=18, range 11947.2-12468.0 ns)
  11947.2 |##########################
  11973.2 |#############
  11999.3 |##########################
  12025.3 |##########################
  12051.4 |#############
  12077.4 |########################################
  12103.5 |
  12129.5 |##########################
  12155.5 |
  12181.6 |
  12207.6 |#############
  12233.7 |#############
  12259.7 |
  12285.8 |
  12311.8 |
  12337.8 |
  12363.9 |
  12389.9 |#############
  12416.0 |
  12442.0 |
  (1 below, 1 above range)

sequential (n=18, range 9722.8-10920.3 ns)
   9722.8 |
   9782.6 |#############
   9842.5 |#############
   9902.4 |#############
   9962.3 |
  10022.1 |
  10082.0 |#############
  10141.9 |##########################
  10201.8 |
  10261.6 |#############
  10321.5 |
  10381.4 |########################################
  10441.3 |##########################
  10501.1 |
  10561.0 |
  10620.9 |
  10680.8 |
  10740.6 |
  10800.5 |
  10860.4 |
  (2 below, 4 above range)

std_parallel (n=18, range 17685.6-20489.2 ns)
  17685.6 |####################
  17825.7 |
  17965.9 |####################
  18106.1 |####################
  18246.3 |####################
  18386.5 |
  18526.6 |########################################
  18666.8 |
  18807.0 |####################
  18947.2 |########################################
  19087.4 |####################
  19227.5 |
  19367.7 |
  19507.9 |
  19648.1 |########################################
  19788.3 |
  19928.4 |####################
  20068.6 |####################
  20208.8 |
  20349.0 |####################
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=3558.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3647.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=3561.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3614.9% of algo (FFI overhead may distort results)
- **nway**: bridge=767.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=877.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=504.9% of algo (FFI overhead may distort results)

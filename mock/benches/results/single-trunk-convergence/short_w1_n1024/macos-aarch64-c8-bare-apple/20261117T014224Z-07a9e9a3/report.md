# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 7.6x slower than the field

std_parallel (19.98 us) is 7.6x the fastest (2.63 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (kernel_nway, head_tail_dyn) are a dead heat (<1%)

kernel_nway (2.63 us) and head_tail_dyn (2.63 us) differ by 0.06%, inside the noise, even though the wider field spreads 660.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### nway shows alternating (throttle bounce) (autocorr -0.66)

nway's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_nway, head_tail_dyn, kernel_seq, head_tail} vs {sequential, nway, std_parallel} (251% apart)

The field splits into a fast tier {kernel_nway, head_tail_dyn, kernel_seq, head_tail} and a slow tier {sequential, nway, std_parallel} with a 251% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.6x the fastest

Fastest kernel_nway (2.63 us) to slowest std_parallel (19.98 us): 7.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_nway** at 2625.8 ns median (-8.7% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 7.61x (fastest 2625.8 ns, slowest 19979.2 ns)

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
| head_tail | 3384ns | 3359ns | 2895ns | 3246ns | 3834ns | base |
| head_tail_dyn | 3160ns | 3178ns | 2763ns | 3083ns | 3474ns | -6.61% |
| kernel_nway | 3215ns | 3203ns | 3031ns | 3191ns | 3344ns | -4.97% |
| kernel_seq | 3218ns | 3257ns | 2998ns | 3207ns | 3346ns | -4.88% |
| nway | 13237ns | 12912ns | 12418ns | 12828ns | 14259ns | +291.22% |
| sequential | 10254ns | 10226ns | 9824ns | 10174ns | 10591ns | +203.07% |
| std_parallel | 20170ns | 20548ns | 18198ns | 19753ns | 21780ns | +496.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2902ns | 2502ns | 3306ns | base | 0.353 |
| head_tail_dyn | 2692ns | 2510ns | 2920ns | -7.23% | 0.380 |
| kernel_nway | 2658ns | 2576ns | 2754ns | -8.38% | 0.385 |
| kernel_seq | 2701ns | 2515ns | 2887ns | -6.90% | 0.379 |
| nway | 12870ns | 12146ns | 13831ns | +343.55% | 0.080 |
| sequential | 10112ns | 9679ns | 10441ns | +248.50% | 0.101 |
| std_parallel | 19764ns | 17871ns | 21396ns | +581.15% | 0.052 |

## Performance model

- Peak throughput: **0.409 Gops/s** (head_tail; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.356 | 87.0% |
| head_tail_dyn | 0.390 | 95.2% |
| kernel_nway | 0.390 | 95.3% |
| kernel_seq | 0.385 | 94.2% |
| nway | 0.082 | 19.9% |
| sequential | 0.101 | 24.8% |
| std_parallel | 0.051 | 12.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 3384ns | 3384ns | base |
| head_tail_dyn | 3160ns | 3160ns | -6.61% |
| kernel_nway | 3215ns | 3215ns | -4.97% |
| kernel_seq | 3218ns | 3218ns | -4.88% |
| nway | 13237ns | 13237ns | +291.22% |
| sequential | 10254ns | 10254ns | +203.07% |
| std_parallel | 20170ns | 20170ns | +496.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2876ns | base | --- | [2555, 3090] | --- | --- | --- | --- |
| head_tail_dyn | 2628ns | no significant difference | [-370, +28]ns | [2545, 2784] | no | 0.2379 | 0.2379 | 0 |
| kernel_nway | 2626ns | no significant difference | [-460, +76]ns | [2606, 2680] | no | 0.2379 | 0.2379 | 0 |
| kernel_seq | 2657ns | no significant difference | [-369, +16]ns | [2596, 2821] | no | 0.2379 | 0.2379 | 0 |
| nway | 12548ns | +9699.2ns (+337.3%) | [+9572, +10182]ns | [12377, 13064] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10098ns | +7197.5ns (+250.3%) | [+7125, +7398]ns | [9922, 10353] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 19979ns | +16891.7ns (+587.4%) | [+15372, +18472]ns | [18102, 21106] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2524ns | +0.6% | +5.7% | +1.1% | +386.7% | +287.9% | +737.0% |
| 1 | 2 | 3030ns | -13.3% | -13.8% | -12.3% | +351.0% | +232.1% | +494.1% |
| 1 | 3 | 3088ns | -7.3% | -14.5% | -6.1% | +303.8% | +236.2% | +568.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.116 | ok |
| head_tail_dyn | -0.042 | ok |
| kernel_nway | -0.500 | moderate- |
| kernel_seq | -0.033 | ok |
| nway | -0.657 | HIGH- (thermal bounce) |
| sequential | -0.001 | ok |
| std_parallel | -0.653 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 93875.5ns | 2901.6ns | 3235.3% | HIGH |
| head_tail_dyn | 92480.2ns | 2691.7ns | 3435.8% | HIGH |
| kernel_nway | 92726.9ns | 2658.4ns | 3488.1% | HIGH |
| kernel_seq | 93412.9ns | 2701.4ns | 3458.0% | HIGH |
| nway | 92428.3ns | 12870.0ns | 718.2% | HIGH |
| sequential | 89578.7ns | 10111.9ns | 885.9% | HIGH |
| std_parallel | 98369.6ns | 19764.2ns | 497.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 2501.7-3305.5 ns)
   2501.7 |########################################
   2541.9 |##########
   2582.1 |
   2622.2 |
   2662.4 |
   2702.6 |##########
   2742.8 |##########
   2783.0 |##########
   2823.2 |
   2863.4 |
   2903.6 |
   2943.8 |##########
   2984.0 |##########
   3024.2 |####################
   3064.4 |
   3104.6 |####################
   3144.8 |
   3185.0 |##########
   3225.2 |
   3265.4 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 2510.0-2920.0 ns)
   2510.0 |########################################
   2530.5 |
   2551.0 |####################
   2571.5 |##########
   2592.0 |##########
   2612.5 |
   2633.0 |##########
   2653.5 |##########
   2674.0 |
   2694.5 |
   2715.0 |##########
   2735.5 |
   2756.0 |
   2776.5 |####################
   2797.0 |
   2817.5 |
   2838.0 |
   2858.5 |
   2879.0 |
   2899.5 |
  (1 below, 4 above range)

kernel_nway (n=18, range 2576.1-2753.6 ns)
   2576.1 |##########################
   2585.0 |
   2593.9 |
   2602.8 |########################################
   2611.6 |##########################
   2620.5 |#############
   2629.4 |#############
   2638.3 |#############
   2647.1 |
   2656.0 |#############
   2664.9 |#############
   2673.7 |
   2682.6 |
   2691.5 |#############
   2700.4 |
   2709.2 |
   2718.1 |
   2727.0 |#############
   2735.9 |#############
   2744.7 |
  (1 below, 2 above range)

kernel_seq (n=18, range 2515.0-2886.9 ns)
   2515.0 |########################################
   2533.6 |
   2552.2 |
   2570.8 |####################
   2589.4 |########################################
   2608.0 |########################################
   2626.6 |
   2645.2 |########################################
   2663.8 |
   2682.4 |
   2701.0 |####################
   2719.6 |####################
   2738.2 |
   2756.8 |####################
   2775.4 |
   2793.9 |
   2812.5 |
   2831.1 |
   2849.7 |####################
   2868.3 |####################
  (1 below, 3 above range)

nway (n=18, range 12145.6-13831.4 ns)
  12145.6 |
  12229.9 |##########################
  12314.1 |##########################
  12398.4 |#############
  12482.7 |########################################
  12567.0 |##########################
  12651.3 |
  12735.6 |#############
  12819.9 |
  12904.2 |
  12988.5 |
  13072.8 |
  13157.1 |
  13241.3 |
  13325.6 |
  13409.9 |#############
  13494.2 |#############
  13578.5 |
  13662.8 |
  13747.1 |#############
  (2 below, 2 above range)

sequential (n=18, range 9678.9-10440.8 ns)
   9678.9 |
   9717.0 |####################
   9755.1 |
   9793.2 |
   9831.3 |####################
   9869.4 |####################
   9907.5 |
   9945.6 |########################################
   9983.7 |####################
  10021.8 |####################
  10059.9 |
  10098.0 |
  10136.1 |####################
  10174.2 |####################
  10212.3 |
  10250.4 |
  10288.4 |####################
  10326.5 |####################
  10364.6 |########################################
  10402.7 |####################
  (2 below, 2 above range)

std_parallel (n=18, range 17871.1-21396.1 ns)
  17871.1 |########################
  18047.4 |########
  18223.6 |
  18399.9 |########
  18576.1 |
  18752.4 |
  18928.6 |
  19104.9 |
  19281.1 |########
  19457.4 |
  19633.6 |########
  19809.9 |
  19986.1 |
  20162.4 |########
  20338.6 |
  20514.9 |########
  20691.1 |
  20867.4 |
  21043.6 |########################################
  21219.9 |########
  (2 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=3264.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3513.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=3515.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3530.1% of algo (FFI overhead may distort results)
- **nway**: bridge=735.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=886.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=494.7% of algo (FFI overhead may distort results)

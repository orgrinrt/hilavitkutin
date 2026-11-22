# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 7.3x slower than the field

std_parallel (19.10 us) is 7.3x the fastest (2.62 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_parallel shows alternating (throttle bounce) (autocorr -0.65)

std_parallel's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn, head_tail, kernel_seq, kernel_nway} vs {sequential, nway, std_parallel} (284% apart)

The field splits into a fast tier {head_tail_dyn, head_tail, kernel_seq, kernel_nway} and a slow tier {sequential, nway, std_parallel} with a 284% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.3x the fastest

Fastest head_tail_dyn (2.62 us) to slowest std_parallel (19.10 us): 7.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: head_tail_dyn** at 2621.7 ns median (-1.7% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 7.29x (fastest 2621.7 ns, slowest 19102.5 ns)

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
| head_tail | 3242ns | 3205ns | 2931ns | 3162ns | 3517ns | base |
| head_tail_dyn | 3193ns | 3216ns | 2850ns | 3184ns | 3378ns | -1.51% |
| kernel_nway | 3353ns | 3309ns | 3042ns | 3240ns | 3678ns | +3.42% |
| kernel_seq | 3336ns | 3264ns | 3115ns | 3259ns | 3562ns | +2.91% |
| nway | 12947ns | 12929ns | 12417ns | 12822ns | 13399ns | +299.39% |
| sequential | 10677ns | 10579ns | 10302ns | 10507ns | 11121ns | +229.38% |
| std_parallel | 19843ns | 19462ns | 17370ns | 19101ns | 22194ns | +512.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2716ns | 2514ns | 2944ns | base | 0.094 |
| head_tail_dyn | 2663ns | 2481ns | 2827ns | -1.98% | 0.096 |
| kernel_nway | 2801ns | 2549ns | 3093ns | +3.13% | 0.091 |
| kernel_seq | 2777ns | 2591ns | 2996ns | +2.24% | 0.092 |
| nway | 12631ns | 12086ns | 13104ns | +364.99% | 0.020 |
| sequential | 10526ns | 10162ns | 10961ns | +287.49% | 0.024 |
| std_parallel | 19481ns | 17029ns | 21796ns | +617.13% | 0.013 |

## Performance model

- Peak throughput: **0.103 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.096 | 93.1% |
| head_tail_dyn | 0.098 | 94.6% |
| kernel_nway | 0.094 | 91.3% |
| kernel_seq | 0.095 | 91.7% |
| nway | 0.020 | 19.8% |
| sequential | 0.025 | 23.8% |
| std_parallel | 0.013 | 13.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 3242ns | 3242ns | base |
| head_tail_dyn | 3193ns | 3193ns | -1.51% |
| kernel_nway | 3353ns | 3353ns | +3.42% |
| kernel_seq | 3336ns | 3336ns | +2.91% |
| nway | 12947ns | 12947ns | +299.39% |
| sequential | 10677ns | 10677ns | +229.38% |
| std_parallel | 19843ns | 19843ns | +512.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2666ns | base | --- | [2569, 2782] | --- | --- | --- | --- |
| head_tail_dyn | 2622ns | no significant difference | [-177, +108]ns | [2577, 2757] | no | 0.5768 | 0.4807 | 0 |
| kernel_nway | 2717ns | no significant difference | [-113, +237]ns | [2640, 2874] | no | 0.3568 | 0.2379 | 0 |
| kernel_seq | 2706ns | no significant difference | [-109, +149]ns | [2663, 2818] | no | 0.8145 | 0.8145 | 0 |
| nway | 12558ns | +9834.9ns (+368.9%) | [+9657, +10072]ns | [12360, 12852] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10431ns | +7738.4ns (+290.3%) | [+7540, +7978]ns | [10214, 10632] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 19102ns | +16580.0ns (+621.9%) | [+15332, +17870]ns | [17956, 20573] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2654ns | -2.4% | +5.3% | +1.8% | +363.1% | +284.4% | +575.1% |
| 1 | 2 | 2672ns | -2.2% | -0.2% | +2.8% | +376.5% | +304.3% | +673.3% |
| 1 | 3 | 2782ns | -2.7% | -2.7% | -3.5% | +362.9% | +279.8% | +562.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.097 | ok |
| head_tail_dyn | -0.074 | ok |
| kernel_nway | -0.365 | moderate- |
| kernel_seq | -0.629 | HIGH- (thermal bounce) |
| nway | -0.052 | ok |
| sequential | -0.423 | moderate- |
| std_parallel | -0.645 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 96275.6ns | 2716.5ns | 3544.1% | HIGH |
| head_tail_dyn | 95227.2ns | 2662.8ns | 3576.2% | HIGH |
| kernel_nway | 92600.8ns | 2801.5ns | 3305.4% | HIGH |
| kernel_seq | 96243.3ns | 2777.2ns | 3465.4% | HIGH |
| nway | 92298.2ns | 12631.4ns | 730.7% | HIGH |
| sequential | 91797.5ns | 10526.2ns | 872.1% | HIGH |
| std_parallel | 97819.6ns | 19480.7ns | 502.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 2514.4-2943.6 ns)
   2514.4 |#############
   2535.9 |##########################
   2557.4 |
   2578.8 |#############
   2600.3 |
   2621.7 |
   2643.2 |########################################
   2664.6 |##########################
   2686.1 |#############
   2707.6 |
   2729.0 |
   2750.5 |#############
   2771.9 |
   2793.4 |#############
   2814.9 |#############
   2836.3 |
   2857.8 |
   2879.2 |
   2900.7 |
   2922.2 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 2481.1-2827.2 ns)
   2481.1 |####################
   2498.4 |
   2515.7 |
   2533.0 |
   2550.3 |####################
   2567.6 |########################################
   2584.9 |####################
   2602.2 |########################################
   2619.5 |####################
   2636.9 |
   2654.2 |####################
   2671.5 |
   2688.8 |####################
   2706.1 |
   2723.4 |
   2740.7 |####################
   2758.0 |####################
   2775.3 |
   2792.6 |
   2809.9 |########################################
  (2 below, 2 above range)

kernel_nway (n=18, range 2549.5-3092.5 ns)
   2549.5 |########################################
   2576.6 |####################
   2603.8 |####################
   2630.9 |
   2658.1 |########################################
   2685.2 |########################################
   2712.4 |####################
   2739.5 |####################
   2766.7 |
   2793.8 |####################
   2821.0 |
   2848.1 |####################
   2875.3 |####################
   2902.4 |
   2929.6 |####################
   2956.8 |
   2983.9 |
   3011.1 |####################
   3038.2 |
   3065.4 |
  (1 below, 2 above range)

kernel_seq (n=18, range 2590.6-2995.8 ns)
   2590.6 |#############
   2610.8 |#############
   2631.1 |#############
   2651.4 |########################################
   2671.6 |#############
   2691.9 |#############
   2712.1 |##########################
   2732.4 |
   2752.7 |
   2772.9 |
   2793.2 |##########################
   2813.5 |#############
   2833.7 |
   2854.0 |
   2874.3 |
   2894.5 |
   2914.8 |
   2935.0 |
   2955.3 |
   2975.6 |#############
  (1 below, 3 above range)

nway (n=18, range 12085.6-13103.6 ns)
  12085.6 |
  12136.5 |
  12187.4 |
  12238.3 |#############
  12289.2 |#############
  12340.1 |########################################
  12391.0 |
  12441.9 |#############
  12492.8 |#############
  12543.7 |#############
  12594.6 |
  12645.5 |#############
  12696.4 |
  12747.3 |#############
  12798.2 |
  12849.1 |#############
  12900.0 |##########################
  12950.9 |#############
  13001.8 |
  13052.7 |
  (2 below, 2 above range)

sequential (n=18, range 10161.7-10960.6 ns)
  10161.7 |#############
  10201.6 |########################################
  10241.6 |#############
  10281.5 |
  10321.4 |#############
  10361.4 |#############
  10401.3 |
  10441.3 |
  10481.2 |#############
  10521.2 |
  10561.1 |########################################
  10601.1 |
  10641.0 |
  10680.9 |#############
  10720.9 |#############
  10760.8 |
  10800.8 |
  10840.7 |
  10880.7 |
  10920.6 |
  (2 below, 3 above range)

std_parallel (n=18, range 17029.4-21796.4 ns)
  17029.4 |
  17267.8 |####################
  17506.1 |####################
  17744.5 |####################
  17982.8 |####################
  18221.2 |####################
  18459.5 |
  18697.9 |########################################
  18936.2 |
  19174.6 |####################
  19412.9 |
  19651.3 |####################
  19889.6 |
  20128.0 |########################################
  20366.3 |
  20604.7 |####################
  20843.0 |####################
  21081.4 |####################
  21319.7 |
  21558.1 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=3535.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3622.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=3428.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3494.9% of algo (FFI overhead may distort results)
- **nway**: bridge=738.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=879.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=503.8% of algo (FFI overhead may distort results)

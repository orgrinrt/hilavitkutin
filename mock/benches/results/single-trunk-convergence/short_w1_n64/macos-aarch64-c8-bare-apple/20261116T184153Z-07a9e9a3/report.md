# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 7.1x slower than the field

std_parallel (18.30 us) is 7.1x the fastest (2.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_parallel shows alternating (throttle bounce) (autocorr -0.65)

std_parallel's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn, kernel_seq, head_tail, kernel_nway} vs {sequential, nway, std_parallel} (263% apart)

The field splits into a fast tier {head_tail_dyn, kernel_seq, head_tail, kernel_nway} and a slow tier {sequential, nway, std_parallel} with a 263% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.1x the fastest

Fastest head_tail_dyn (2.58 us) to slowest std_parallel (18.30 us): 7.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: head_tail_dyn** at 2578.3 ns median (-2.2% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 7.10x (fastest 2578.3 ns, slowest 18303.3 ns)

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
| head_tail | 3378ns | 3200ns | 2896ns | 3142ns | 3971ns | base |
| head_tail_dyn | 3090ns | 3080ns | 2812ns | 3013ns | 3346ns | -8.50% |
| kernel_nway | 3482ns | 3458ns | 3136ns | 3373ns | 3821ns | +3.10% |
| kernel_seq | 3175ns | 3188ns | 2829ns | 3099ns | 3461ns | -6.01% |
| nway | 13271ns | 12942ns | 12715ns | 12903ns | 14101ns | +292.91% |
| sequential | 10693ns | 10716ns | 10124ns | 10598ns | 11121ns | +216.59% |
| std_parallel | 18842ns | 18795ns | 17929ns | 18520ns | 19780ns | +457.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2853ns | 2508ns | 3369ns | base | 0.022 |
| head_tail_dyn | 2632ns | 2492ns | 2799ns | -7.73% | 0.024 |
| kernel_nway | 2915ns | 2566ns | 3221ns | +2.18% | 0.022 |
| kernel_seq | 2658ns | 2463ns | 2863ns | -6.85% | 0.024 |
| nway | 12881ns | 12394ns | 13604ns | +351.47% | 0.005 |
| sequential | 10546ns | 9984ns | 10971ns | +269.63% | 0.006 |
| std_parallel | 18466ns | 17589ns | 19424ns | +547.23% | 0.003 |

## Performance model

- Peak throughput: **0.026 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.024 | 93.5% |
| head_tail_dyn | 0.025 | 95.5% |
| kernel_nway | 0.022 | 84.6% |
| kernel_seq | 0.025 | 94.4% |
| nway | 0.005 | 19.5% |
| sequential | 0.006 | 23.3% |
| std_parallel | 0.003 | 13.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 3378ns | 3378ns | base |
| head_tail_dyn | 3090ns | 3090ns | -8.50% |
| kernel_nway | 3482ns | 3482ns | +3.10% |
| kernel_seq | 3175ns | 3175ns | -6.01% |
| nway | 13271ns | 13271ns | +292.91% |
| sequential | 10693ns | 10693ns | +216.59% |
| std_parallel | 18842ns | 18842ns | +457.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2635ns | base | --- | [2583, 2812] | --- | --- | --- | --- |
| head_tail_dyn | 2578ns | no significant difference | [-195, +128]ns | [2524, 2718] | no | 0.2855 | 0.2379 | 0 |
| kernel_nway | 2910ns | +220.8ns (+8.4%) | [+73, +348]ns | [2722, 3069] | YES | 0.0463 | 0.0309 | 0 |
| kernel_seq | 2608ns | no significant difference | [-181, +87]ns | [2542, 2737] | no | 0.4807 | 0.4807 | 0 |
| nway | 12637ns | +9905.0ns (+375.9%) | [+9827, +10308]ns | [12449, 13070] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10572ns | +7829.2ns (+297.1%) | [+7563, +8242]ns | [10287, 10827] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 18303ns | +15493.4ns (+588.0%) | [+15155, +16107]ns | [17826, 18803] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2605ns | -3.2% | +15.1% | +6.8% | +425.9% | +307.7% | +597.1% |
| 1 | 2 | 2658ns | -1.9% | +4.6% | -5.3% | +371.5% | +299.0% | +593.1% |
| 1 | 3 | 2728ns | -1.7% | +2.4% | -4.4% | +361.5% | +282.6% | +563.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.005 | ok |
| head_tail_dyn | -0.001 | ok |
| kernel_nway | -0.205 | moderate- |
| kernel_seq | -0.392 | moderate- |
| nway | -0.195 | ok |
| sequential | -0.117 | ok |
| std_parallel | -0.649 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 94908.8ns | 2853.1ns | 3326.6% | HIGH |
| head_tail_dyn | 94873.7ns | 2632.5ns | 3603.9% | HIGH |
| kernel_nway | 95980.2ns | 2915.4ns | 3292.2% | HIGH |
| kernel_seq | 94636.2ns | 2657.6ns | 3561.0% | HIGH |
| nway | 93269.4ns | 12880.5ns | 724.1% | HIGH |
| sequential | 89834.4ns | 10545.8ns | 851.8% | HIGH |
| std_parallel | 95008.0ns | 18465.9ns | 514.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 2508.3-3368.9 ns)
   2508.3 |########
   2551.4 |########################################
   2594.4 |################
   2637.4 |########
   2680.4 |################
   2723.5 |
   2766.5 |
   2809.5 |########################
   2852.6 |########
   2895.6 |
   2938.6 |
   2981.6 |########
   3024.7 |
   3067.7 |
   3110.7 |
   3153.7 |
   3196.8 |
   3239.8 |
   3282.8 |
   3325.9 |
  (1 below, 1 above range)

head_tail_dyn (n=18, range 2491.7-2798.9 ns)
   2491.7 |##########
   2507.0 |####################
   2522.4 |########################################
   2537.7 |##########
   2553.1 |
   2568.5 |
   2583.8 |
   2599.2 |##########
   2614.6 |
   2629.9 |
   2645.3 |##########
   2660.6 |
   2676.0 |##########
   2691.4 |
   2706.7 |####################
   2722.1 |##########
   2737.4 |
   2752.8 |
   2768.2 |
   2783.5 |
  (1 below, 3 above range)

kernel_nway (n=18, range 2565.5-3221.4 ns)
   2565.5 |#############
   2598.3 |
   2631.1 |#############
   2663.9 |#############
   2696.7 |
   2729.5 |#############
   2762.3 |
   2795.1 |#############
   2827.9 |#############
   2860.7 |#############
   2893.5 |
   2926.3 |########################################
   2959.1 |
   2991.8 |
   3024.6 |
   3057.4 |#############
   3090.2 |
   3123.0 |#############
   3155.8 |
   3188.6 |#############
  (2 below, 3 above range)

kernel_seq (n=18, range 2462.8-2863.1 ns)
   2462.8 |
   2482.8 |########################################
   2502.8 |
   2522.8 |####################
   2542.8 |########################################
   2562.8 |########################################
   2582.9 |
   2602.9 |
   2622.9 |########################################
   2642.9 |
   2662.9 |
   2682.9 |
   2702.9 |####################
   2723.0 |########################################
   2743.0 |
   2763.0 |
   2783.0 |
   2803.0 |####################
   2823.0 |
   2843.1 |
  (2 below, 3 above range)

nway (n=18, range 12394.5-13604.2 ns)
  12394.5 |########################################
  12455.0 |##########
  12515.4 |##########
  12575.9 |##########
  12636.4 |########################################
  12696.9 |##########
  12757.4 |
  12817.9 |
  12878.3 |
  12938.8 |
  12999.3 |
  13059.8 |
  13120.3 |
  13180.8 |
  13241.2 |
  13301.7 |
  13362.2 |##########
  13422.7 |
  13483.2 |##########
  13543.7 |
  (1 below, 3 above range)

sequential (n=18, range 9983.9-10971.1 ns)
   9983.9 |
  10033.3 |####################
  10082.6 |####################
  10132.0 |
  10181.3 |####################
  10230.7 |####################
  10280.1 |####################
  10329.4 |
  10378.8 |########################################
  10428.1 |
  10477.5 |
  10526.9 |####################
  10576.2 |####################
  10625.6 |####################
  10675.0 |####################
  10724.3 |
  10773.7 |
  10823.0 |########################################
  10872.4 |####################
  10921.8 |####################
  (1 below, 2 above range)

std_parallel (n=18, range 17588.9-19424.5 ns)
  17588.9 |########################################
  17680.7 |########################################
  17772.5 |
  17864.2 |####################
  17956.0 |####################
  18047.8 |####################
  18139.6 |####################
  18231.3 |
  18323.1 |####################
  18414.9 |####################
  18506.7 |####################
  18598.5 |
  18690.2 |####################
  18782.0 |####################
  18873.8 |########################################
  18965.6 |
  19057.3 |
  19149.1 |
  19240.9 |
  19332.7 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=26.5% (high variance, measurements may be unstable)
- **head_tail**: bridge=3543.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3669.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=3294.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3628.0% of algo (FFI overhead may distort results)
- **nway**: bridge=737.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=855.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=519.4% of algo (FFI overhead may distort results)

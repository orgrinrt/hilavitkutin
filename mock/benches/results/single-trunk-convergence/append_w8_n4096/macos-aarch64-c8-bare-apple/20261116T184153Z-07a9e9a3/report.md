# append, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 27.8x slower than the field

std_parallel (79.47 us) is 27.8x the fastest (2.85 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (head_tail, head_tail_dyn) are a dead heat (<1%)

head_tail (2.85 us) and head_tail_dyn (2.86 us) differ by 0.36%, inside the noise, even though the wider field spreads 2684.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.67)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (2.85 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, head_tail_dyn, kernel_seq} vs {sequential, kernel_nway, nway, std_parallel} (346% apart)

The field splits into a fast tier {head_tail, head_tail_dyn, kernel_seq} and a slow tier {sequential, kernel_nway, nway, std_parallel} with a 346% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 27.8x the fastest

Fastest head_tail (2.85 us) to slowest std_parallel (79.47 us): 27.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 1.7x its best-20%

kernel_nway's best 20% of batches run at 14.06 us but its worst 20% at 23.79 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 2854.2 ns median
- 4 variants significantly slower than baseline
- Spread: 27.84x (fastest 2854.2 ns, slowest 79470.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 8 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 3525ns | 3469ns | 3368ns | 3452ns | 3711ns | base |
| head_tail_dyn | 3616ns | 3479ns | 3338ns | 3458ns | 3992ns | +2.59% |
| kernel_nway | 21462ns | 22352ns | 14978ns | 21537ns | 24592ns | +508.94% |
| kernel_seq | 3567ns | 3531ns | 3390ns | 3504ns | 3751ns | +1.21% |
| nway | 53799ns | 53910ns | 50899ns | 53248ns | 56076ns | +1426.43% |
| sequential | 13169ns | 13148ns | 12857ns | 13093ns | 13440ns | +273.64% |
| std_parallel | 79912ns | 79885ns | 76993ns | 79393ns | 82149ns | +2167.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2932ns | 2799ns | 3116ns | base | 1.397 |
| head_tail_dyn | 3020ns | 2776ns | 3383ns | +3.00% | 1.356 |
| kernel_nway | 20528ns | 14060ns | 23788ns | +600.20% | 0.200 |
| kernel_seq | 2960ns | 2810ns | 3149ns | +0.96% | 1.384 |
| nway | 53223ns | 50271ns | 55542ns | +1715.42% | 0.077 |
| sequential | 12990ns | 12676ns | 13251ns | +343.07% | 0.315 |
| std_parallel | 79526ns | 76633ns | 81766ns | +2612.62% | 0.052 |

## Performance model

- Peak throughput: **1.475 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.435 | 97.3% |
| head_tail_dyn | 1.430 | 96.9% |
| kernel_nway | 0.193 | 13.1% |
| kernel_seq | 1.407 | 95.4% |
| nway | 0.077 | 5.2% |
| sequential | 0.316 | 21.4% |
| std_parallel | 0.052 | 3.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 3525ns | 3525ns | base |
| head_tail_dyn | 3616ns | 3616ns | +2.59% |
| kernel_nway | 21462ns | 21462ns | +508.94% |
| kernel_seq | 3567ns | 3567ns | +1.21% |
| nway | 53799ns | 53799ns | +1426.43% |
| sequential | 13169ns | 13169ns | +273.64% |
| std_parallel | 79912ns | 79912ns | +2167.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2854ns | base | --- | [2829, 3035] | --- | --- | --- | --- |
| head_tail_dyn | 2865ns | no significant difference | [-102, +117]ns | [2806, 3044] | no | 0.5768 | 0.4807 | 0 |
| kernel_nway | 21200ns | +18262.5ns (+639.8%) | [+17158, +19752]ns | [20017, 22467] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2910ns | no significant difference | [-23, +150]ns | [2833, 3012] | no | 0.8145 | 0.8145 | 0 |
| nway | 53312ns | +50520.8ns (+1770.1%) | [+48877, +51135]ns | [51802, 54169] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 12977ns | +9985.4ns (+349.8%) | [+9883, +10146]ns | [12860, 12994] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 79471ns | +76514.6ns (+2680.8%) | [+75612, +77381]ns | [78479, 80321] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2848ns | -1.0% | +645.5% | +0.4% | +1798.4% | +345.9% | +2691.5% |
| 1 | 2 | 2842ns | +13.0% | +617.0% | +0.8% | +1753.9% | +357.3% | +2684.4% |
| 1 | 3 | 3090ns | -8.7% | +596.2% | +2.0% | +1599.9% | +320.2% | +2474.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.179 | ok |
| head_tail_dyn | -0.667 | HIGH- (thermal bounce) |
| kernel_nway | -0.630 | HIGH- (thermal bounce) |
| kernel_seq | -0.156 | ok |
| nway | -0.115 | ok |
| sequential | -0.189 | ok |
| std_parallel | -0.661 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 110289.3ns | 2931.7ns | 3761.9% | HIGH |
| head_tail_dyn | 109048.1ns | 3019.7ns | 3611.3% | HIGH |
| kernel_nway | 127113.4ns | 20528.0ns | 619.2% | HIGH |
| kernel_seq | 109851.4ns | 2959.7ns | 3711.5% | HIGH |
| nway | 142804.9ns | 53222.9ns | 268.3% | HIGH |
| sequential | 112985.9ns | 12989.6ns | 869.8% | HIGH |
| std_parallel | 170135.4ns | 79526.4ns | 213.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 2798.6-3116.0 ns)
   2798.6 |#############
   2814.5 |##########################
   2830.3 |########################################
   2846.2 |##########################
   2862.1 |#############
   2877.9 |
   2893.8 |
   2909.7 |
   2925.5 |#############
   2941.4 |
   2957.3 |
   2973.2 |
   2989.0 |
   3004.9 |
   3020.8 |#############
   3036.6 |#############
   3052.5 |
   3068.4 |
   3084.2 |
   3100.1 |
  (2 below, 4 above range)

head_tail_dyn (n=18, range 2776.4-3383.3 ns)
   2776.4 |########################################
   2806.7 |########################################
   2837.1 |#############
   2867.4 |#############
   2897.8 |
   2928.1 |#############
   2958.5 |
   2988.8 |#############
   3019.2 |#############
   3049.5 |#############
   3079.9 |#############
   3110.2 |
   3140.5 |
   3170.9 |
   3201.2 |
   3231.6 |
   3261.9 |
   3292.3 |#############
   3322.6 |
   3353.0 |
  (2 below, 2 above range)

kernel_nway (n=18, range 14059.7-23788.2 ns)
  14059.7 |####################
  14546.2 |
  15032.6 |
  15519.0 |
  16005.4 |
  16491.8 |
  16978.3 |####################
  17464.7 |
  17951.1 |
  18437.5 |
  18924.0 |
  19410.4 |####################
  19896.8 |####################
  20383.2 |########################################
  20869.6 |########################################
  21356.1 |####################
  21842.5 |####################
  22328.9 |########################################
  22815.3 |####################
  23301.8 |
  (2 below, 3 above range)

kernel_seq (n=18, range 2809.7-3148.6 ns)
   2809.7 |########################################
   2826.7 |########################################
   2843.6 |
   2860.6 |####################
   2877.5 |####################
   2894.5 |####################
   2911.4 |####################
   2928.3 |####################
   2945.3 |####################
   2962.2 |
   2979.2 |####################
   2996.1 |
   3013.1 |
   3030.0 |####################
   3047.0 |
   3063.9 |####################
   3080.8 |
   3097.8 |
   3114.7 |
   3131.7 |
  (2 below, 3 above range)

nway (n=18, range 50270.8-55541.7 ns)
  50270.8 |#############
  50534.4 |
  50797.9 |#############
  51061.5 |#############
  51325.0 |#############
  51588.5 |
  51852.1 |
  52115.6 |#############
  52379.2 |##########################
  52642.7 |
  52906.2 |
  53169.8 |#############
  53433.3 |#############
  53696.9 |########################################
  53960.4 |
  54224.0 |#############
  54487.5 |#############
  54751.0 |
  55014.6 |
  55278.1 |
  (1 below, 3 above range)

sequential (n=18, range 12676.4-13250.7 ns)
  12676.4 |######
  12705.1 |######
  12733.8 |
  12762.5 |
  12791.2 |######
  12820.0 |
  12848.7 |
  12877.4 |
  12906.1 |######
  12934.8 |######
  12963.5 |########################################
  12992.2 |######
  13021.0 |
  13049.7 |#############
  13078.4 |
  13107.1 |
  13135.8 |
  13164.5 |
  13193.3 |
  13222.0 |
  (2 below, 2 above range)

std_parallel (n=18, range 76633.3-81766.0 ns)
  76633.3 |
  76889.9 |
  77146.6 |
  77403.2 |
  77659.8 |
  77916.5 |
  78173.1 |####################
  78429.7 |########################################
  78686.4 |
  78943.0 |
  79199.6 |##########
  79456.3 |##########
  79712.9 |####################
  79969.5 |##########
  80226.2 |
  80482.8 |####################
  80739.4 |
  80996.1 |
  81252.7 |
  81509.3 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=3846.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3853.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=599.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3781.6% of algo (FFI overhead may distort results)
- **nway**: bridge=262.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=873.0% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=214.0% of algo (FFI overhead may distort results)

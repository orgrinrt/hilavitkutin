# append, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 5.6x slower than the field

std_parallel (16.92 us) is 5.6x the fastest (3.01 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (head_tail, head_tail_dyn) are a dead heat (<1%)

head_tail (3.01 us) and head_tail_dyn (3.03 us) differ by 0.62%, inside the noise, even though the wider field spreads 461.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### std_parallel shows alternating (throttle bounce) (autocorr -0.66)

std_parallel's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (3.01 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, head_tail_dyn, kernel_nway, kernel_seq} vs {sequential, nway, std_parallel} (310% apart)

The field splits into a fast tier {head_tail, head_tail_dyn, kernel_nway, kernel_seq} and a slow tier {sequential, nway, std_parallel} with a 310% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.6x the fastest

Fastest head_tail (3.01 us) to slowest std_parallel (16.92 us): 5.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 1.7x its best-20%

kernel_nway's best 20% of batches run at 2.84 us but its worst 20% at 4.86 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 3012.5 ns median
- 3 variants significantly slower than baseline
- Spread: 5.62x (fastest 3012.5 ns, slowest 16920.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 1 |
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
| head_tail | 3669ns | 3610ns | 3458ns | 3602ns | 3876ns | base |
| head_tail_dyn | 3731ns | 3654ns | 3374ns | 3586ns | 4128ns | +1.69% |
| kernel_nway | 4257ns | 3729ns | 3415ns | 3692ns | 5525ns | +16.01% |
| kernel_seq | 3915ns | 3812ns | 3469ns | 3785ns | 4333ns | +6.69% |
| nway | 16255ns | 16110ns | 15722ns | 16006ns | 16894ns | +342.97% |
| sequential | 13377ns | 13215ns | 12833ns | 13144ns | 13999ns | +264.55% |
| std_parallel | 17474ns | 17298ns | 16540ns | 17159ns | 18413ns | +376.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 3068ns | 2889ns | 3260ns | base | 1.335 |
| head_tail_dyn | 3134ns | 2839ns | 3501ns | +2.18% | 1.307 |
| kernel_nway | 3640ns | 2842ns | 4856ns | +18.65% | 1.125 |
| kernel_seq | 3303ns | 2946ns | 3697ns | +7.67% | 1.240 |
| nway | 15862ns | 15399ns | 16481ns | +417.10% | 0.258 |
| sequential | 13175ns | 12653ns | 13765ns | +329.50% | 0.311 |
| std_parallel | 17067ns | 16164ns | 17996ns | +456.38% | 0.240 |

## Performance model

- Peak throughput: **1.443 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.360 | 94.2% |
| head_tail_dyn | 1.351 | 93.7% |
| kernel_nway | 1.306 | 90.5% |
| kernel_seq | 1.290 | 89.4% |
| nway | 0.261 | 18.1% |
| sequential | 0.315 | 21.8% |
| std_parallel | 0.242 | 16.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 3669ns | 3669ns | base |
| head_tail_dyn | 3731ns | 3731ns | +1.69% |
| kernel_nway | 4257ns | 4257ns | +16.01% |
| kernel_seq | 3915ns | 3915ns | +6.69% |
| nway | 16255ns | 16255ns | +342.97% |
| sequential | 13377ns | 13377ns | +264.55% |
| std_parallel | 17474ns | 17474ns | +376.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 3012ns | base | --- | [2956, 3131] | --- | --- | --- | --- |
| head_tail_dyn | 3031ns | no significant difference | [-112, +129]ns | [2919, 3171] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 3135ns | no significant difference | [-23, +431]ns | [2975, 3410] | no | 0.4985 | 0.3323 | 1 |
| kernel_seq | 3175ns | no significant difference | [-58, +492]ns | [3054, 3502] | no | 0.9774 | 0.8145 | 0 |
| nway | 15696ns | +12654.1ns (+420.1%) | [+12577, +12921]ns | [15504, 16017] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 13019ns | +10029.2ns (+332.9%) | [+9788, +10285]ns | [12850, 13273] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 16921ns | +13835.4ns (+459.3%) | [+13398, +14217]ns | [16469, 17375] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 3121ns | +1.6% | -5.0% | +3.7% | +413.9% | +308.3% | +441.7% |
| 1 | 2 | 2958ns | +3.0% | +17.8% | +13.1% | +424.3% | +340.1% | +478.1% |
| 1 | 3 | 3073ns | -5.2% | +1.4% | +1.0% | +408.5% | +342.0% | +449.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.612 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.000 | ok |
| kernel_nway | -0.613 | HIGH- (thermal bounce) |
| kernel_seq | -0.470 | moderate- |
| nway | -0.300 | moderate- |
| sequential | -0.025 | ok |
| std_parallel | -0.657 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 107767.8ns | 3067.6ns | 3513.1% | HIGH |
| head_tail_dyn | 107374.5ns | 3134.5ns | 3425.6% | HIGH |
| kernel_nway | 108624.5ns | 3639.6ns | 2984.5% | HIGH |
| kernel_seq | 109735.2ns | 3303.0ns | 3322.3% | HIGH |
| nway | 115900.9ns | 15862.5ns | 730.7% | HIGH |
| sequential | 113924.5ns | 13175.5ns | 864.7% | HIGH |
| std_parallel | 116300.5ns | 17067.4ns | 681.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 2888.9-3260.4 ns)
   2888.9 |####################
   2907.5 |####################
   2926.1 |########################################
   2944.6 |
   2963.2 |####################
   2981.8 |########################################
   3000.4 |####################
   3018.9 |########################################
   3037.5 |
   3056.1 |
   3074.7 |
   3093.2 |####################
   3111.8 |####################
   3130.4 |####################
   3149.0 |####################
   3167.5 |####################
   3186.1 |
   3204.7 |####################
   3223.3 |
   3241.8 |
  (1 below, 1 above range)

head_tail_dyn (n=18, range 2838.9-3501.4 ns)
   2838.9 |########################################
   2872.0 |########################################
   2905.1 |####################
   2938.2 |########################################
   2971.4 |
   3004.5 |########################################
   3037.6 |
   3070.8 |####################
   3103.9 |####################
   3137.0 |####################
   3170.1 |####################
   3203.3 |####################
   3236.4 |
   3269.5 |
   3302.6 |####################
   3335.8 |
   3368.9 |
   3402.0 |
   3435.1 |
   3468.3 |
  (1 below, 2 above range)

kernel_nway (n=18, range 2841.7-4855.5 ns)
   2841.7 |#############
   2942.4 |########################################
   3043.1 |########################################
   3143.7 |##########################
   3244.4 |#############
   3345.1 |##########################
   3445.8 |
   3546.5 |##########################
   3647.2 |
   3747.9 |
   3848.6 |
   3949.3 |
   4050.0 |
   4150.7 |
   4251.4 |
   4352.1 |
   4452.8 |
   4553.5 |
   4654.1 |
   4754.8 |
  (2 below, 2 above range)

kernel_seq (n=18, range 2945.8-3696.5 ns)
   2945.8 |########################################
   2983.4 |####################
   3020.9 |########################################
   3058.4 |
   3096.0 |########################################
   3133.5 |####################
   3171.0 |####################
   3208.6 |
   3246.1 |
   3283.7 |
   3321.2 |
   3358.7 |########################################
   3396.3 |
   3433.8 |
   3471.3 |####################
   3508.9 |####################
   3546.4 |
   3583.9 |####################
   3621.5 |####################
   3659.0 |
  (1 below, 2 above range)

nway (n=18, range 15398.6-16481.2 ns)
  15398.6 |
  15452.7 |########################################
  15506.9 |##############################
  15561.0 |
  15615.1 |
  15669.3 |####################
  15723.4 |##########
  15777.5 |##########
  15831.7 |
  15885.8 |
  15939.9 |
  15994.1 |##########
  16048.2 |##########
  16102.3 |##########
  16156.5 |
  16210.6 |##########
  16264.7 |
  16318.9 |
  16373.0 |
  16427.1 |##########
  (1 below, 1 above range)

sequential (n=18, range 12652.8-13764.6 ns)
  12652.8 |
  12708.4 |########################################
  12763.9 |
  12819.5 |
  12875.1 |
  12930.7 |##########################
  12986.3 |##########################
  13041.9 |##########################
  13097.5 |##########################
  13153.1 |
  13208.7 |
  13264.3 |
  13319.9 |
  13375.4 |
  13431.0 |#############
  13486.6 |
  13542.2 |#############
  13597.8 |
  13653.4 |
  13709.0 |##########################
  (2 below, 1 above range)

std_parallel (n=18, range 16163.9-17995.8 ns)
  16163.9 |#############
  16255.5 |#############
  16347.1 |##########################
  16438.7 |
  16530.3 |#############
  16621.9 |#############
  16713.5 |#############
  16805.1 |#############
  16896.7 |
  16988.3 |########################################
  17079.9 |
  17171.5 |
  17263.1 |
  17354.7 |########################################
  17446.3 |
  17537.8 |
  17629.4 |
  17721.0 |
  17812.6 |
  17904.2 |#############
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=3553.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3500.5% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=42.7% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=3519.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3442.5% of algo (FFI overhead may distort results)
- **nway**: bridge=735.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=870.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=697.4% of algo (FFI overhead may distort results)

# append, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 17.1x slower than the field

std_parallel (48.70 us) is 17.1x the fastest (2.85 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (head_tail_dyn, head_tail) are a dead heat (<1%)

head_tail_dyn (2.85 us) and head_tail (2.88 us) differ by 0.95%, inside the noise, even though the wider field spreads 1607.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.65)

kernel_seq's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn, head_tail, kernel_seq} vs {sequential, kernel_nway, nway, std_parallel} (349% apart)

The field splits into a fast tier {head_tail_dyn, head_tail, kernel_seq} and a slow tier {sequential, kernel_nway, nway, std_parallel} with a 349% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 17.1x the fastest

Fastest head_tail_dyn (2.85 us) to slowest std_parallel (48.70 us): 17.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 1.6x its best-20%

kernel_nway's best 20% of batches run at 9.92 us but its worst 20% at 15.68 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 2852.1 ns median (-0.9% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 17.07x (fastest 2852.1 ns, slowest 48697.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 4 |
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
| head_tail | 3590ns | 3508ns | 3360ns | 3481ns | 3869ns | base |
| head_tail_dyn | 3491ns | 3462ns | 3308ns | 3438ns | 3663ns | -2.75% |
| kernel_nway | 14735ns | 15442ns | 10640ns | 14839ns | 16626ns | +310.43% |
| kernel_seq | 3550ns | 3494ns | 3386ns | 3485ns | 3728ns | -1.13% |
| nway | 38849ns | 38685ns | 34538ns | 37492ns | 43040ns | +982.13% |
| sequential | 13180ns | 13133ns | 12746ns | 13033ns | 13617ns | +267.12% |
| std_parallel | 49570ns | 49100ns | 46961ns | 48879ns | 51912ns | +1280.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2976ns | 2800ns | 3223ns | base | 1.376 |
| head_tail_dyn | 2887ns | 2799ns | 2990ns | -3.00% | 1.419 |
| kernel_nway | 13806ns | 9921ns | 15683ns | +363.91% | 0.297 |
| kernel_seq | 2915ns | 2801ns | 3025ns | -2.05% | 1.405 |
| nway | 38312ns | 33971ns | 42487ns | +1187.38% | 0.107 |
| sequential | 12995ns | 12569ns | 13435ns | +336.68% | 0.315 |
| std_parallel | 49114ns | 46462ns | 51401ns | +1550.36% | 0.083 |

## Performance model

- Peak throughput: **1.464 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.423 | 97.2% |
| head_tail_dyn | 1.436 | 98.1% |
| kernel_nway | 0.285 | 19.5% |
| kernel_seq | 1.421 | 97.1% |
| nway | 0.107 | 7.3% |
| sequential | 0.316 | 21.6% |
| std_parallel | 0.084 | 5.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 3590ns | 3590ns | base |
| head_tail_dyn | 3491ns | 3491ns | -2.75% |
| kernel_nway | 14735ns | 14735ns | +310.43% |
| kernel_seq | 3550ns | 3550ns | -1.13% |
| nway | 38849ns | 38849ns | +982.13% |
| sequential | 13180ns | 13180ns | +267.12% |
| std_parallel | 49570ns | 49570ns | +1280.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2879ns | base | --- | [2846, 2938] | --- | --- | --- | --- |
| head_tail_dyn | 2852ns | no significant difference | [-88, +15]ns | [2838, 2888] | no | 0.1722 | 0.1435 | 1 |
| kernel_nway | 14385ns | +11529.1ns (+400.4%) | [+10454, +12304]ns | [13494, 15212] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2883ns | no significant difference | [-50, +73]ns | [2852, 2958] | no | 0.8145 | 0.8145 | 0 |
| nway | 38167ns | +35275.0ns (+1225.2%) | [+32340, +37108]ns | [35312, 40040] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 12944ns | +10062.5ns (+349.5%) | [+9771, +10269]ns | [12654, 13148] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 48698ns | +45804.2ns (+1590.9%) | [+45121, +46954]ns | [47952, 49831] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2975ns | -3.2% | +405.6% | -4.3% | +1082.1% | +324.4% | +1556.6% |
| 1 | 2 | 2867ns | -0.6% | +402.7% | +2.5% | +1293.4% | +351.2% | +1595.5% |
| 1 | 3 | 2892ns | -1.6% | +386.3% | -1.1% | +1240.3% | +362.1% | +1576.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.307 | moderate- |
| head_tail_dyn | -0.108 | ok |
| kernel_nway | -0.018 | ok |
| kernel_seq | -0.653 | HIGH- (thermal bounce) |
| nway | -0.319 | moderate- |
| sequential | -0.006 | ok |
| std_parallel | -0.088 | ok |

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
| head_tail | 109945.1ns | 2975.9ns | 3694.5% | HIGH |
| head_tail_dyn | 108370.6ns | 2886.6ns | 3754.3% | HIGH |
| kernel_nway | 165869.7ns | 13805.6ns | 1201.5% | HIGH |
| kernel_seq | 109115.5ns | 2914.8ns | 3743.5% | HIGH |
| nway | 129369.2ns | 38311.6ns | 337.7% | HIGH |
| sequential | 112123.8ns | 12995.4ns | 862.8% | HIGH |
| std_parallel | 137441.2ns | 49113.7ns | 279.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 2800.0-3222.9 ns)
   2800.0 |
   2821.1 |##############################
   2842.3 |##########
   2863.4 |########################################
   2884.6 |####################
   2905.7 |##########
   2926.9 |
   2948.0 |####################
   2969.2 |
   2990.3 |##########
   3011.5 |
   3032.6 |
   3053.8 |
   3074.9 |
   3096.0 |
   3117.2 |
   3138.3 |
   3159.5 |
   3180.6 |
   3201.8 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 2798.6-2989.6 ns)
   2798.6 |
   2808.1 |#############
   2817.7 |#############
   2827.2 |#############
   2836.8 |##########################
   2846.3 |########################################
   2855.9 |#############
   2865.4 |#############
   2875.0 |#############
   2884.5 |#############
   2894.1 |
   2903.6 |#############
   2913.2 |
   2922.7 |
   2932.3 |
   2941.8 |
   2951.4 |
   2960.9 |
   2970.5 |
   2980.0 |
  (2 below, 3 above range)

kernel_nway (n=18, range 9920.8-15683.3 ns)
   9920.8 |##########################
  10209.0 |
  10497.1 |
  10785.2 |#############
  11073.3 |
  11361.5 |
  11649.6 |
  11937.7 |
  12225.8 |
  12514.0 |
  12802.1 |
  13090.2 |#############
  13378.3 |
  13666.5 |#############
  13954.6 |#############
  14242.7 |########################################
  14530.8 |#############
  14819.0 |#############
  15107.1 |##########################
  15395.2 |
  (1 below, 4 above range)

kernel_seq (n=18, range 2801.4-3025.0 ns)
   2801.4 |
   2812.6 |
   2823.8 |
   2834.9 |########################################
   2846.1 |########################################
   2857.3 |########################################
   2868.5 |
   2879.7 |########################################
   2890.8 |
   2902.0 |
   2913.2 |
   2924.4 |####################
   2935.6 |
   2946.7 |########################################
   2957.9 |####################
   2969.1 |####################
   2980.3 |
   2991.5 |
   3002.6 |
   3013.8 |####################
  (2 below, 2 above range)

nway (n=18, range 33970.8-42486.8 ns)
  33970.8 |########################################
  34396.6 |
  34822.4 |########################################
  35248.2 |####################
  35674.0 |####################
  36099.8 |####################
  36525.6 |
  36951.4 |
  37377.2 |####################
  37803.0 |
  38228.8 |
  38654.6 |########################################
  39080.4 |
  39506.2 |####################
  39932.0 |########################################
  40357.8 |####################
  40783.6 |
  41209.4 |####################
  41635.2 |
  42061.0 |
  (1 below, 2 above range)

sequential (n=18, range 12569.4-13435.4 ns)
  12569.4 |##########################
  12612.7 |##########################
  12656.0 |#############
  12699.3 |
  12742.6 |
  12785.9 |
  12829.2 |
  12872.5 |########################################
  12915.8 |
  12959.1 |##########################
  13002.4 |#############
  13045.7 |#############
  13089.0 |
  13132.3 |
  13175.6 |
  13218.9 |#############
  13262.2 |#############
  13305.5 |
  13348.8 |
  13392.1 |
  (1 below, 3 above range)

std_parallel (n=18, range 46462.5-51401.4 ns)
  46462.5 |########################################
  46709.4 |
  46956.4 |
  47203.3 |
  47450.3 |####################
  47697.2 |####################
  47944.2 |####################
  48191.1 |####################
  48438.1 |########################################
  48685.0 |####################
  48931.9 |####################
  49178.9 |
  49425.8 |########################################
  49672.8 |
  49919.7 |####################
  50166.7 |
  50413.6 |
  50660.6 |########################################
  50907.5 |
  51154.4 |####################
  (1 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=3865.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3802.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1194.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3773.5% of algo (FFI overhead may distort results)
- **nway**: bridge=336.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=869.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=279.9% of algo (FFI overhead may distort results)

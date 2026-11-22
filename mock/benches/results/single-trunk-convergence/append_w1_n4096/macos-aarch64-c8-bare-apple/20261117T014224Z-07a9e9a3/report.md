# append, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 6.2x slower than the field

std_parallel (17.74 us) is 6.2x the fastest (2.86 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.66)

kernel_nway's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_nway, head_tail, kernel_seq, head_tail_dyn} vs {sequential, nway, std_parallel} (303% apart)

The field splits into a fast tier {kernel_nway, head_tail, kernel_seq, head_tail_dyn} and a slow tier {sequential, nway, std_parallel} with a 303% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.2x the fastest

Fastest kernel_nway (2.86 us) to slowest std_parallel (17.74 us): 6.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_nway** at 2860.4 ns median (-4.1% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 6.20x (fastest 2860.4 ns, slowest 17739.5 ns)

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
| head_tail | 3748ns | 3702ns | 3368ns | 3576ns | 4195ns | base |
| head_tail_dyn | 4032ns | 3885ns | 3651ns | 3839ns | 4512ns | +7.59% |
| kernel_nway | 3678ns | 3502ns | 3383ns | 3483ns | 4117ns | -1.87% |
| kernel_seq | 4011ns | 3844ns | 3312ns | 3726ns | 4787ns | +7.02% |
| nway | 15583ns | 15573ns | 15201ns | 15484ns | 15923ns | +315.78% |
| sequential | 13402ns | 13365ns | 12775ns | 13278ns | 13903ns | +257.59% |
| std_parallel | 18295ns | 18108ns | 16672ns | 17895ns | 19706ns | +388.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 3087ns | 2742ns | 3497ns | base | 1.327 |
| head_tail_dyn | 3405ns | 3010ns | 3884ns | +10.30% | 1.203 |
| kernel_nway | 3044ns | 2750ns | 3490ns | -1.37% | 1.345 |
| kernel_seq | 3253ns | 2754ns | 3830ns | +5.39% | 1.259 |
| nway | 15200ns | 14910ns | 15484ns | +392.40% | 0.269 |
| sequential | 13199ns | 12593ns | 13713ns | +327.59% | 0.310 |
| std_parallel | 17893ns | 16294ns | 19251ns | +479.65% | 0.229 |

## Performance model

- Peak throughput: **1.494 Gops/s** (head_tail; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.374 | 92.0% |
| head_tail_dyn | 1.259 | 84.3% |
| kernel_nway | 1.432 | 95.8% |
| kernel_seq | 1.288 | 86.2% |
| nway | 0.270 | 18.1% |
| sequential | 0.312 | 20.9% |
| std_parallel | 0.231 | 15.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 3748ns | 3748ns | base |
| head_tail_dyn | 4032ns | 4032ns | +7.59% |
| kernel_nway | 3678ns | 3678ns | -1.87% |
| kernel_seq | 4011ns | 4011ns | +7.02% |
| nway | 15583ns | 15583ns | +315.78% |
| sequential | 13402ns | 13402ns | +257.59% |
| std_parallel | 18295ns | 18295ns | +388.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2981ns | base | --- | [2808, 3200] | --- | --- | --- | --- |
| head_tail_dyn | 3254ns | +279.2ns (+9.4%) | [+129, +402]ns | [3167, 3412] | YES | 0.0113 | 0.0075 | 0 |
| kernel_nway | 2860ns | no significant difference | [-354, +212]ns | [2790, 3221] | no | 0.8145 | 0.8145 | 0 |
| kernel_seq | 3181ns | no significant difference | [-52, +235]ns | [2875, 3315] | no | 0.5768 | 0.4807 | 0 |
| nway | 15144ns | +12052.1ns (+404.3%) | [+11994, +12171]ns | [15033, 15310] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 13127ns | +10054.1ns (+337.2%) | [+9888, +10396]ns | [12894, 13504] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 17740ns | +14716.7ns (+493.6%) | [+14129, +15206]ns | [17156, 18256] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 3294ns | -1.2% | -15.8% | +2.2% | +360.7% | +308.1% | +485.8% |
| 1 | 2 | 3010ns | +19.9% | +14.6% | +7.5% | +400.8% | +341.7% | +482.5% |
| 1 | 3 | 2752ns | +14.5% | +3.3% | +0.7% | +455.0% | +361.1% | +503.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.000 | ok |
| head_tail_dyn | -0.635 | HIGH- (thermal bounce) |
| kernel_nway | -0.661 | HIGH- (thermal bounce) |
| kernel_seq | -0.064 | ok |
| nway | -0.493 | moderate- |
| sequential | -0.075 | ok |
| std_parallel | -0.021 | ok |

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
| head_tail | 111922.7ns | 3086.8ns | 3625.8% | HIGH |
| head_tail_dyn | 112380.1ns | 3404.9ns | 3300.6% | HIGH |
| kernel_nway | 109901.6ns | 3044.5ns | 3609.9% | HIGH |
| kernel_seq | 114406.9ns | 3253.2ns | 3516.7% | HIGH |
| nway | 113720.4ns | 15199.5ns | 748.2% | HIGH |
| sequential | 113519.7ns | 13198.8ns | 860.1% | HIGH |
| std_parallel | 114082.2ns | 17892.6ns | 637.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 2741.7-3496.5 ns)
   2741.7 |########################################
   2779.4 |
   2817.2 |##########
   2854.9 |##########
   2892.6 |
   2930.4 |##########
   2968.1 |####################
   3005.9 |##########
   3043.6 |
   3081.3 |
   3119.1 |
   3156.8 |####################
   3194.6 |##########
   3232.3 |
   3270.1 |
   3307.8 |
   3345.5 |##########
   3383.3 |
   3421.0 |
   3458.8 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 3009.7-3884.0 ns)
   3009.7 |########################################
   3053.4 |
   3097.1 |####################
   3140.9 |########################################
   3184.6 |########################################
   3228.3 |########################################
   3272.0 |########################################
   3315.7 |
   3359.4 |####################
   3403.2 |####################
   3446.9 |
   3490.6 |
   3534.3 |
   3578.0 |
   3621.7 |
   3665.4 |####################
   3709.2 |
   3752.9 |
   3796.6 |####################
   3840.3 |
  (1 below, 2 above range)

kernel_nway (n=18, range 2750.0-3489.6 ns)
   2750.0 |########################################
   2787.0 |########################################
   2824.0 |##########################
   2861.0 |#############
   2897.9 |
   2934.9 |##########################
   2971.9 |
   3008.9 |
   3045.9 |
   3082.8 |
   3119.8 |
   3156.8 |
   3193.8 |#############
   3230.7 |
   3267.7 |
   3304.7 |#############
   3341.7 |
   3378.7 |#############
   3415.6 |
   3452.6 |
  (1 below, 3 above range)

kernel_seq (n=18, range 2754.2-3829.8 ns)
   2754.2 |##########################
   2808.0 |
   2861.7 |##########################
   2915.5 |#############
   2969.3 |
   3023.1 |
   3076.9 |#############
   3130.7 |#############
   3184.4 |########################################
   3238.2 |
   3292.0 |##########################
   3345.8 |
   3399.6 |
   3453.4 |
   3507.1 |#############
   3560.9 |
   3614.7 |
   3668.5 |
   3722.3 |
   3776.1 |
  (2 below, 3 above range)

nway (n=18, range 14909.7-15484.0 ns)
  14909.7 |
  14938.4 |####################
  14967.2 |########################################
  14995.9 |####################
  15024.6 |####################
  15053.3 |
  15082.0 |####################
  15110.7 |########################################
  15139.4 |####################
  15168.2 |####################
  15196.9 |####################
  15225.6 |
  15254.3 |####################
  15283.0 |
  15311.7 |
  15340.4 |####################
  15369.1 |
  15397.9 |####################
  15426.6 |####################
  15455.3 |
  (1 below, 2 above range)

sequential (n=18, range 12593.0-13713.2 ns)
  12593.0 |##########
  12649.0 |####################
  12705.0 |##########
  12761.1 |
  12817.1 |
  12873.1 |
  12929.1 |
  12985.1 |
  13041.1 |########################################
  13097.1 |
  13153.1 |##########
  13209.1 |
  13265.1 |##########
  13321.1 |##########
  13377.1 |
  13433.2 |
  13489.2 |####################
  13545.2 |
  13601.2 |##########
  13657.2 |##########
  (1 below, 2 above range)

std_parallel (n=18, range 16294.5-19250.7 ns)
  16294.5 |#############
  16442.3 |
  16590.1 |
  16737.9 |#############
  16885.7 |
  17033.5 |#############
  17181.3 |#############
  17329.1 |##########################
  17477.0 |#############
  17624.8 |
  17772.6 |#############
  17920.4 |
  18068.2 |########################################
  18216.0 |#############
  18363.8 |
  18511.6 |
  18659.4 |
  18807.3 |
  18955.1 |#############
  19102.9 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=3734.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3471.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=3890.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3565.9% of algo (FFI overhead may distort results)
- **nway**: bridge=751.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=874.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=648.6% of algo (FFI overhead may distort results)

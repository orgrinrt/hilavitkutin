# append, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 9.4x slower than the field

std_parallel (28.98 us) is 9.4x the fastest (3.07 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (kernel_seq, head_tail_dyn) are a dead heat (<1%)

kernel_seq (3.07 us) and head_tail_dyn (3.10 us) differ by 0.88%, inside the noise, even though the wider field spreads 843.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### nway shows alternating (throttle bounce) (autocorr -0.66)

nway's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_seq, head_tail_dyn, head_tail, kernel_nway} vs {sequential, nway, std_parallel} (169% apart)

The field splits into a fast tier {kernel_seq, head_tail_dyn, head_tail, kernel_nway} and a slow tier {sequential, nway, std_parallel} with a 169% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 9.4x the fastest

Fastest kernel_seq (3.07 us) to slowest std_parallel (28.98 us): 9.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 3070.8 ns median (-7.8% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 9.44x (fastest 3070.8 ns, slowest 28979.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 2 |
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
| head_tail | 4074ns | 3892ns | 3522ns | 3854ns | 4680ns | base |
| head_tail_dyn | 3769ns | 3685ns | 3504ns | 3663ns | 4062ns | -7.48% |
| kernel_nway | 5470ns | 5408ns | 4938ns | 5311ns | 5976ns | +34.27% |
| kernel_seq | 3767ns | 3690ns | 3511ns | 3664ns | 4049ns | -7.54% |
| nway | 21568ns | 21700ns | 19769ns | 21403ns | 22715ns | +429.40% |
| sequential | 13384ns | 13190ns | 12835ns | 13177ns | 13969ns | +228.52% |
| std_parallel | 29975ns | 29410ns | 27354ns | 29117ns | 32572ns | +635.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 3457ns | 2896ns | 4042ns | base | 1.185 |
| head_tail_dyn | 3171ns | 2917ns | 3460ns | -8.28% | 1.292 |
| kernel_nway | 4876ns | 4357ns | 5368ns | +41.04% | 0.840 |
| kernel_seq | 3153ns | 2890ns | 3456ns | -8.81% | 1.299 |
| nway | 21056ns | 19292ns | 22201ns | +509.03% | 0.195 |
| sequential | 13199ns | 12654ns | 13770ns | +281.77% | 0.310 |
| std_parallel | 29574ns | 26986ns | 32169ns | +755.38% | 0.139 |

## Performance model

- Peak throughput: **1.417 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.230 | 86.8% |
| head_tail_dyn | 1.322 | 93.3% |
| kernel_nway | 0.848 | 59.8% |
| kernel_seq | 1.334 | 94.1% |
| nway | 0.194 | 13.7% |
| sequential | 0.315 | 22.2% |
| std_parallel | 0.141 | 10.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4074ns | 4074ns | base |
| head_tail_dyn | 3769ns | 3769ns | -7.48% |
| kernel_nway | 5470ns | 5470ns | +34.27% |
| kernel_seq | 3767ns | 3767ns | -7.54% |
| nway | 21568ns | 21568ns | +429.40% |
| sequential | 13384ns | 13384ns | +228.52% |
| std_parallel | 29975ns | 29975ns | +635.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 3331ns | base | --- | [3075, 3692] | --- | --- | --- | --- |
| head_tail_dyn | 3098ns | no significant difference | [-588, +240]ns | [3017, 3181] | no | 0.4807 | 0.4807 | 0 |
| kernel_nway | 4831ns | +1510.4ns (+45.3%) | [+975, +1862]ns | [4438, 5227] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 3071ns | no significant difference | [-427, +0]ns | [2977, 3150] | no | 0.1722 | 0.1435 | 1 |
| nway | 21148ns | +17854.2ns (+536.0%) | [+16890, +17990]ns | [20644, 21648] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 13015ns | +9879.2ns (+296.6%) | [+9633, +10146]ns | [12971, 13475] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 28979ns | +25379.2ns (+761.9%) | [+24662, +27396]ns | [28140, 30627] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 3760ns | -20.1% | +32.2% | -18.5% | +471.0% | +239.4% | +696.0% |
| 1 | 2 | 3138ns | +7.3% | +66.6% | -2.9% | +524.8% | +314.8% | +834.7% |
| 1 | 3 | 3419ns | -9.4% | +29.7% | -5.0% | +536.4% | +286.4% | +732.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.467 | moderate- |
| head_tail_dyn | -0.625 | HIGH- (thermal bounce) |
| kernel_nway | -0.371 | moderate- |
| kernel_seq | -0.212 | moderate- |
| nway | -0.657 | HIGH- (thermal bounce) |
| sequential | -0.004 | ok |
| std_parallel | -0.007 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 109011.8ns | 3457.4ns | 3153.0% | HIGH |
| head_tail_dyn | 109396.8ns | 3171.1ns | 3449.8% | HIGH |
| kernel_nway | 115888.9ns | 4876.2ns | 2376.6% | HIGH |
| kernel_seq | 110363.7ns | 3152.8ns | 3500.5% | HIGH |
| nway | 116427.3ns | 21056.5ns | 552.9% | HIGH |
| sequential | 112675.7ns | 13199.3ns | 853.6% | HIGH |
| std_parallel | 124721.5ns | 29573.9ns | 421.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 2895.8-4041.7 ns)
   2895.8 |########################################
   2953.1 |
   3010.4 |########################################
   3067.7 |####################
   3125.0 |
   3182.3 |########################################
   3239.6 |
   3296.9 |########################################
   3354.2 |####################
   3411.5 |
   3468.8 |
   3526.0 |
   3583.3 |####################
   3640.6 |####################
   3697.9 |####################
   3755.2 |####################
   3812.5 |####################
   3869.8 |
   3927.1 |
   3984.4 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 2916.7-3459.7 ns)
   2916.7 |
   2943.8 |
   2971.0 |########################################
   2998.1 |########################################
   3025.3 |########################################
   3052.4 |
   3079.6 |########################################
   3106.7 |####################
   3133.9 |
   3161.0 |########################################
   3188.2 |####################
   3215.3 |
   3242.5 |####################
   3269.6 |
   3296.8 |
   3324.0 |
   3351.1 |
   3378.3 |
   3405.4 |
   3432.6 |
  (2 below, 3 above range)

kernel_nway (n=18, range 4357.0-5368.0 ns)
   4357.0 |#############
   4407.5 |########################################
   4458.1 |
   4508.6 |
   4559.2 |
   4609.7 |
   4660.3 |##########################
   4710.8 |#############
   4761.4 |
   4811.9 |
   4862.5 |
   4913.1 |##########################
   4963.6 |
   5014.2 |
   5064.7 |
   5115.3 |
   5165.8 |##########################
   5216.4 |##########################
   5266.9 |##########################
   5317.5 |
  (2 below, 1 above range)

kernel_seq (n=18, range 2890.3-3456.2 ns)
   2890.3 |
   2918.6 |##########################
   2946.9 |##########################
   2975.2 |##########################
   3003.5 |
   3031.8 |
   3060.1 |########################################
   3088.4 |##########################
   3116.7 |#############
   3145.0 |
   3173.3 |##########################
   3201.6 |
   3229.9 |
   3258.2 |
   3286.5 |
   3314.8 |#############
   3343.1 |
   3371.4 |
   3399.7 |
   3428.0 |
  (1 below, 2 above range)

nway (n=18, range 19291.7-22200.7 ns)
  19291.7 |####################
  19437.1 |
  19582.6 |########################################
  19728.0 |
  19873.5 |
  20018.9 |
  20164.4 |
  20309.8 |
  20455.3 |####################
  20600.7 |
  20746.2 |########################################
  20891.6 |########################################
  21037.1 |
  21182.5 |####################
  21328.0 |####################
  21473.4 |####################
  21618.9 |########################################
  21764.3 |########################################
  21909.8 |####################
  22055.2 |
  (1 below, 1 above range)

sequential (n=18, range 12654.2-13770.1 ns)
  12654.2 |
  12710.0 |#############
  12765.8 |#############
  12821.6 |
  12877.4 |
  12933.2 |########################################
  12989.0 |########################################
  13044.8 |#############
  13100.6 |
  13156.4 |#############
  13212.1 |
  13267.9 |
  13323.7 |#############
  13379.5 |
  13435.3 |
  13491.1 |
  13546.9 |
  13602.7 |
  13658.5 |
  13714.3 |##########################
  (2 below, 3 above range)

std_parallel (n=18, range 26986.1-32169.4 ns)
  26986.1 |
  27245.3 |
  27504.5 |#############
  27763.6 |########################################
  28022.8 |
  28282.0 |##########################
  28541.1 |#############
  28800.3 |##########################
  29059.5 |#############
  29318.6 |
  29577.8 |
  29836.9 |#############
  30096.1 |
  30355.3 |#############
  30614.4 |#############
  30873.6 |
  31132.8 |#############
  31391.9 |
  31651.1 |
  31910.3 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=3253.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3539.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=2370.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3569.8% of algo (FFI overhead may distort results)
- **nway**: bridge=554.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=870.0% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=429.3% of algo (FFI overhead may distort results)

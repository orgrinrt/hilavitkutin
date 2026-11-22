# append, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 10.4x slower than the field

std_parallel (29.07 us) is 10.4x the fastest (2.80 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {head_tail_dyn, kernel_seq, head_tail, kernel_nway} vs {sequential, nway, std_parallel} (213% apart)

The field splits into a fast tier {head_tail_dyn, kernel_seq, head_tail, kernel_nway} and a slow tier {sequential, nway, std_parallel} with a 213% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 10.4x the fastest

Fastest head_tail_dyn (2.80 us) to slowest std_parallel (29.07 us): 10.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: head_tail_dyn** at 2800.0 ns median (-6.1% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 10.38x (fastest 2800.0 ns, slowest 29070.8 ns)

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
| head_tail | 3722ns | 3581ns | 3350ns | 3576ns | 4128ns | base |
| head_tail_dyn | 3486ns | 3417ns | 3329ns | 3410ns | 3678ns | -6.36% |
| kernel_nway | 5224ns | 4781ns | 4676ns | 4778ns | 6166ns | +40.34% |
| kernel_seq | 3726ns | 3546ns | 3317ns | 3525ns | 4232ns | +0.10% |
| nway | 22562ns | 21917ns | 21017ns | 21825ns | 24440ns | +506.13% |
| sequential | 13394ns | 13229ns | 12626ns | 13045ns | 14302ns | +259.85% |
| std_parallel | 29170ns | 29427ns | 27625ns | 29072ns | 30090ns | +683.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 3107ns | 2767ns | 3510ns | base | 1.318 |
| head_tail_dyn | 2869ns | 2743ns | 3044ns | -7.67% | 1.428 |
| kernel_nway | 4547ns | 4067ns | 5375ns | +46.33% | 0.901 |
| kernel_seq | 3116ns | 2753ns | 3619ns | +0.28% | 1.315 |
| nway | 22076ns | 20683ns | 23810ns | +610.43% | 0.186 |
| sequential | 13200ns | 12453ns | 14075ns | +324.79% | 0.310 |
| std_parallel | 28787ns | 27263ns | 29685ns | +826.39% | 0.142 |

## Performance model

- Peak throughput: **1.493 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.374 | 92.0% |
| head_tail_dyn | 1.463 | 98.0% |
| kernel_nway | 0.983 | 65.8% |
| kernel_seq | 1.387 | 92.9% |
| nway | 0.191 | 12.8% |
| sequential | 0.314 | 21.0% |
| std_parallel | 0.141 | 9.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 3722ns | 3722ns | base |
| head_tail_dyn | 3486ns | 3486ns | -6.36% |
| kernel_nway | 5224ns | 5224ns | +40.34% |
| kernel_seq | 3726ns | 3726ns | +0.10% |
| nway | 22562ns | 22562ns | +506.13% |
| sequential | 13394ns | 13394ns | +259.85% |
| std_parallel | 29170ns | 29170ns | +683.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2981ns | base | --- | [2883, 3229] | --- | --- | --- | --- |
| head_tail_dyn | 2800ns | no significant difference | [-471, +21]ns | [2771, 2908] | no | 0.2855 | 0.2379 | 0 |
| kernel_nway | 4169ns | +1112.5ns (+37.3%) | [+917, +1806]ns | [4123, 4623] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2952ns | no significant difference | [-415, +646]ns | [2779, 3479] | no | 1.0000 | 1.0000 | 0 |
| nway | 21483ns | +18404.2ns (+617.3%) | [+18079, +19638]ns | [20981, 23015] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 13062ns | +9829.2ns (+329.7%) | [+9358, +11129]ns | [12665, 14008] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 29071ns | +25691.7ns (+861.8%) | [+25438, +26335]ns | [28208, 29452] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 3527ns | -21.0% | +17.8% | -21.4% | +562.4% | +259.1% | +724.7% |
| 1 | 2 | 2981ns | -4.1% | +37.8% | -1.8% | +644.0% | +332.8% | +891.2% |
| 1 | 3 | 2827ns | +0.4% | +91.2% | +28.7% | +638.7% | +396.0% | +897.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.063 | ok |
| head_tail_dyn | -0.340 | moderate- |
| kernel_nway | -0.185 | ok |
| kernel_seq | -0.081 | ok |
| nway | -0.000 | ok |
| sequential | -0.082 | ok |
| std_parallel | -0.389 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 112742.8ns | 3107.4ns | 3628.2% | HIGH |
| head_tail_dyn | 111527.8ns | 2869.2ns | 3887.1% | HIGH |
| kernel_nway | 114781.7ns | 4547.2ns | 2524.2% | HIGH |
| kernel_seq | 113182.6ns | 3116.0ns | 3632.3% | HIGH |
| nway | 123333.3ns | 22075.9ns | 558.7% | HIGH |
| sequential | 112947.9ns | 13200.0ns | 855.7% | HIGH |
| std_parallel | 119212.0ns | 28786.8ns | 414.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 2766.6-3509.7 ns)
   2766.6 |#############
   2803.8 |
   2840.9 |##########################
   2878.1 |########################################
   2915.3 |
   2952.4 |#############
   2989.6 |#############
   3026.7 |#############
   3063.9 |
   3101.0 |#############
   3138.2 |
   3175.3 |#############
   3212.5 |
   3249.6 |#############
   3286.8 |
   3324.0 |
   3361.1 |
   3398.3 |
   3435.4 |
   3472.6 |
  (2 below, 4 above range)

head_tail_dyn (n=18, range 2743.1-3043.8 ns)
   2743.1 |##########################
   2758.1 |#############
   2773.1 |########################################
   2788.2 |#############
   2803.2 |##########################
   2818.2 |
   2833.3 |
   2848.3 |
   2863.3 |
   2878.4 |#############
   2893.4 |#############
   2908.4 |#############
   2923.5 |#############
   2938.5 |
   2953.5 |
   2968.6 |
   2983.6 |
   2998.6 |
   3013.7 |
   3028.7 |#############
  (2 below, 2 above range)

kernel_nway (n=18, range 4066.7-5375.0 ns)
   4066.7 |##########################
   4132.1 |########################################
   4197.5 |######
   4262.9 |
   4328.4 |
   4393.8 |
   4459.2 |######
   4524.6 |
   4590.0 |
   4655.4 |
   4720.8 |######
   4786.3 |
   4851.7 |
   4917.1 |
   4982.5 |
   5047.9 |
   5113.3 |
   5178.8 |
   5244.2 |
   5309.6 |######
  (1 below, 3 above range)

kernel_seq (n=18, range 2752.8-3619.5 ns)
   2752.8 |########################################
   2796.1 |################
   2839.5 |########
   2882.8 |
   2926.1 |
   2969.5 |
   3012.8 |########
   3056.1 |
   3099.5 |################
   3142.8 |
   3186.1 |
   3229.5 |
   3272.8 |
   3316.1 |
   3359.5 |
   3402.8 |########
   3446.1 |
   3489.5 |########
   3532.8 |
   3576.1 |########
  (1 below, 3 above range)

nway (n=18, range 20683.3-23809.7 ns)
  20683.3 |########################################
  20839.6 |##########################
  20995.9 |#############
  21152.3 |
  21308.6 |##########################
  21464.9 |#############
  21621.2 |
  21777.6 |#############
  21933.9 |
  22090.2 |
  22246.5 |
  22402.8 |#############
  22559.2 |
  22715.5 |#############
  22871.8 |
  23028.1 |
  23184.4 |#############
  23340.8 |
  23497.1 |#############
  23653.4 |
  (1 below, 3 above range)

sequential (n=18, range 12452.8-14075.0 ns)
  12452.8 |##############################
  12533.9 |##########
  12615.0 |
  12696.1 |####################
  12777.2 |##########
  12858.3 |
  12939.4 |
  13020.6 |##############################
  13101.7 |
  13182.8 |##########
  13263.9 |
  13345.0 |
  13426.1 |
  13507.2 |
  13588.3 |
  13669.5 |
  13750.6 |
  13831.7 |
  13912.8 |
  13993.9 |########################################
  (1 below, 2 above range)

std_parallel (n=18, range 27262.5-29684.7 ns)
  27262.5 |#############
  27383.6 |
  27504.8 |#############
  27625.9 |#############
  27747.0 |
  27868.1 |
  27989.2 |#############
  28110.3 |
  28231.4 |
  28352.5 |#############
  28473.6 |
  28594.7 |#############
  28715.8 |#############
  28837.0 |#############
  28958.1 |
  29079.2 |
  29200.3 |########################################
  29321.4 |#############
  29442.5 |##########################
  29563.6 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=3759.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3995.1% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=2753.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3828.3% of algo (FFI overhead may distort results)
- **nway**: bridge=572.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=861.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=411.0% of algo (FFI overhead may distort results)

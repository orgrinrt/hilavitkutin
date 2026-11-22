# fold_light, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 6.5x slower than the field

std_parallel (18.85 us) is 6.5x the fastest (2.88 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (head_tail_dyn, head_tail) are a dead heat (<1%)

head_tail_dyn (2.88 us) and head_tail (2.88 us) differ by 0.15%, inside the noise, even though the wider field spreads 554.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail shows alternating (throttle bounce) (autocorr -0.67)

head_tail's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn, head_tail, kernel_seq, kernel_nway} vs {sequential, nway, std_parallel} (246% apart)

The field splits into a fast tier {head_tail_dyn, head_tail, kernel_seq, kernel_nway} and a slow tier {sequential, nway, std_parallel} with a 246% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.5x the fastest

Fastest head_tail_dyn (2.88 us) to slowest std_parallel (18.85 us): 6.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader head_tail_dyn vs stability leader head_tail (+0% speed for 1.9x steadier)

head_tail_dyn is fastest (2.88 us, CV 5.1%); head_tail gives up 0.1% median for 1.9x lower variance (CV 2.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: head_tail_dyn** at 2879.2 ns median (-0.1% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 6.55x (fastest 2879.2 ns, slowest 18854.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 1 |
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
| head_tail | 3440ns | 3469ns | 3190ns | 3422ns | 3592ns | base |
| head_tail_dyn | 3443ns | 3419ns | 3190ns | 3386ns | 3655ns | +0.08% |
| kernel_nway | 3697ns | 3669ns | 3444ns | 3627ns | 3929ns | +7.48% |
| kernel_seq | 3624ns | 3629ns | 3390ns | 3569ns | 3822ns | +5.34% |
| nway | 13344ns | 13167ns | 12712ns | 13085ns | 14050ns | +287.91% |
| sequential | 10875ns | 10569ns | 10233ns | 10488ns | 11777ns | +216.14% |
| std_parallel | 19605ns | 19200ns | 18229ns | 19117ns | 21024ns | +469.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2875ns | 2746ns | 2960ns | base | 1.425 |
| head_tail_dyn | 2919ns | 2775ns | 3083ns | +1.55% | 1.403 |
| kernel_nway | 3040ns | 2868ns | 3205ns | +5.75% | 1.347 |
| kernel_seq | 3004ns | 2783ns | 3197ns | +4.49% | 1.364 |
| nway | 12973ns | 12406ns | 13679ns | +351.32% | 0.316 |
| sequential | 10725ns | 10100ns | 11635ns | +273.10% | 0.382 |
| std_parallel | 19163ns | 17890ns | 20406ns | +566.63% | 0.214 |

## Performance model

- Peak throughput: **1.492 Gops/s** (head_tail; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.421 | 95.2% |
| head_tail_dyn | 1.423 | 95.4% |
| kernel_nway | 1.364 | 91.5% |
| kernel_seq | 1.369 | 91.8% |
| nway | 0.322 | 21.6% |
| sequential | 0.394 | 26.4% |
| std_parallel | 0.217 | 14.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 3440ns | 3440ns | base |
| head_tail_dyn | 3443ns | 3443ns | +0.08% |
| kernel_nway | 3697ns | 3697ns | +7.48% |
| kernel_seq | 3624ns | 3624ns | +5.34% |
| nway | 13344ns | 13344ns | +287.91% |
| sequential | 10875ns | 10875ns | +216.14% |
| std_parallel | 19605ns | 19605ns | +469.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2883ns | base | --- | [2821, 2931] | --- | --- | --- | --- |
| head_tail_dyn | 2879ns | no significant difference | [-35, +85]ns | [2827, 2933] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 3002ns | +147.9ns (+5.1%) | [+56, +242]ns | [2969, 3052] | YES | 0.0020 | 0.0013 | 0 |
| kernel_seq | 2992ns | no significant difference | [-13, +256]ns | [2869, 3148] | no | 0.1155 | 0.0963 | 0 |
| nway | 12723ns | +9881.2ns (+342.7%) | [+9683, +10288]ns | [12523, 13183] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10383ns | +7487.5ns (+259.7%) | [+7292, +8546]ns | [10154, 11296] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 18854ns | +15887.5ns (+551.0%) | [+15600, +16706]ns | [18442, 19625] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2883ns | -1.7% | +3.0% | +1.9% | +352.7% | +280.1% | +561.1% |
| 1 | 2 | 2888ns | -0.9% | +5.0% | +5.1% | +348.2% | +278.5% | +538.4% |
| 1 | 3 | 2883ns | +3.0% | +4.1% | +4.3% | +335.5% | +251.3% | +591.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.667 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.071 | ok |
| kernel_nway | -0.477 | moderate- |
| kernel_seq | -0.328 | moderate- |
| nway | -0.062 | ok |
| sequential | -0.148 | ok |
| std_parallel | -0.444 | moderate- |

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
| head_tail | 91189.8ns | 2874.5ns | 3172.3% | HIGH |
| head_tail_dyn | 91714.6ns | 2919.2ns | 3141.8% | HIGH |
| kernel_nway | 93254.4ns | 3039.8ns | 3067.8% | HIGH |
| kernel_seq | 89603.7ns | 3003.7ns | 2983.1% | HIGH |
| nway | 89464.6ns | 12973.4ns | 689.6% | HIGH |
| sequential | 91884.9ns | 10724.8ns | 856.8% | HIGH |
| std_parallel | 94947.5ns | 19162.5ns | 495.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 2745.8-2959.7 ns)
   2745.8 |####################
   2756.5 |
   2767.2 |
   2777.9 |
   2788.6 |
   2799.3 |####################
   2810.0 |####################
   2820.7 |####################
   2831.4 |
   2842.1 |
   2852.8 |####################
   2863.5 |
   2874.2 |########################################
   2884.9 |########################################
   2895.6 |####################
   2906.2 |####################
   2916.9 |
   2927.6 |
   2938.3 |####################
   2949.0 |####################
  (2 below, 3 above range)

head_tail_dyn (n=18, range 2775.0-3083.3 ns)
   2775.0 |##########################
   2790.4 |#############
   2805.9 |#############
   2821.3 |#############
   2836.7 |##########################
   2852.1 |
   2867.5 |#############
   2882.9 |#############
   2898.3 |#############
   2913.8 |
   2929.2 |########################################
   2944.6 |
   2960.0 |
   2975.4 |
   2990.8 |#############
   3006.2 |
   3021.7 |
   3037.1 |
   3052.5 |
   3067.9 |
  (1 below, 3 above range)

kernel_nway (n=18, range 2868.1-3204.8 ns)
   2868.1 |#############
   2884.9 |#############
   2901.7 |#############
   2918.6 |
   2935.4 |
   2952.3 |#############
   2969.1 |#############
   2985.9 |########################################
   3002.8 |##########################
   3019.6 |
   3036.5 |##########################
   3053.3 |#############
   3070.1 |
   3087.0 |
   3103.8 |#############
   3120.7 |
   3137.5 |
   3154.3 |
   3171.2 |
   3188.0 |
  (1 below, 3 above range)

kernel_seq (n=18, range 2783.3-3196.5 ns)
   2783.3 |#############
   2804.0 |#############
   2824.7 |#############
   2845.3 |#############
   2866.0 |
   2886.6 |########################################
   2907.3 |
   2928.0 |
   2948.6 |
   2969.3 |#############
   2989.9 |#############
   3010.6 |
   3031.3 |
   3051.9 |#############
   3072.6 |
   3093.2 |
   3113.9 |#############
   3134.6 |#############
   3155.2 |########################################
   3175.9 |
  (1 below, 2 above range)

nway (n=18, range 12405.5-13679.2 ns)
  12405.5 |#############
  12469.2 |########################################
  12532.9 |##########################
  12596.6 |#############
  12660.3 |#############
  12723.9 |#############
  12787.6 |
  12851.3 |
  12915.0 |#############
  12978.7 |
  13042.4 |
  13106.0 |#############
  13169.7 |##########################
  13233.4 |
  13297.1 |#############
  13360.8 |
  13424.5 |
  13488.1 |
  13551.8 |
  13615.5 |
  (1 below, 3 above range)

sequential (n=18, range 10100.0-11635.4 ns)
  10100.0 |########################################
  10176.8 |################
  10253.5 |
  10330.3 |################
  10407.1 |
  10483.9 |
  10560.6 |########
  10637.4 |
  10714.2 |########
  10790.9 |
  10867.7 |
  10944.5 |
  11021.2 |
  11098.0 |
  11174.8 |
  11251.5 |########
  11328.3 |
  11405.1 |
  11481.9 |########
  11558.6 |
  (1 below, 4 above range)

std_parallel (n=18, range 17890.3-20405.5 ns)
  17890.3 |####################
  18016.1 |########################################
  18141.8 |
  18267.6 |####################
  18393.3 |
  18519.1 |########################################
  18644.9 |####################
  18770.6 |########################################
  18896.4 |
  19022.2 |
  19147.9 |
  19273.7 |####################
  19399.5 |####################
  19525.2 |####################
  19651.0 |####################
  19776.7 |
  19902.5 |
  20028.3 |
  20154.0 |####################
  20279.8 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=3150.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3177.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=3088.6% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3034.3% of algo (FFI overhead may distort results)
- **nway**: bridge=698.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=879.6% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=503.6% of algo (FFI overhead may distort results)

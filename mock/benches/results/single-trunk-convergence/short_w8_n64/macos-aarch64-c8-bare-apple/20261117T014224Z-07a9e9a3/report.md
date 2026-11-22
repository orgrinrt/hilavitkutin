# fold_light, short fiber (1 to 4 morsels), W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 64% faster than the next best (head_tail)

kernel_seq (2.49 us) leads head_tail (4.08 us) by 64%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 40% (significant)

kernel_seq is -1.61 us (40%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 34.2x slower than the field

std_parallel (85.15 us) is 34.2x the fastest (2.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.67)

head_tail's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 34.2x the fastest

Fastest kernel_seq (2.49 us) to slowest std_parallel (85.15 us): 34.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 1.5x its best-20%

kernel_nway's best 20% of batches run at 14.40 us but its worst 20% at 21.66 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 2493.3 ns median (-38.9% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 34.15x (fastest 2493.3 ns, slowest 85153.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 8 |
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
| head_tail | 4913ns | 4720ns | 4574ns | 4684ns | 5427ns | base |
| head_tail_dyn | 5021ns | 4800ns | 4641ns | 4777ns | 5576ns | +2.18% |
| kernel_nway | 17878ns | 15934ns | 15029ns | 15881ns | 22296ns | +263.85% |
| kernel_seq | 2833ns | 2829ns | 2661ns | 2805ns | 2960ns | -42.35% |
| nway | 50030ns | 50355ns | 46569ns | 49584ns | 52430ns | +918.23% |
| sequential | 10505ns | 10400ns | 10096ns | 10325ns | 10978ns | +113.79% |
| std_parallel | 85503ns | 85488ns | 77651ns | 84411ns | 91067ns | +1640.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4295ns | 3956ns | 4810ns | base | 0.015 |
| head_tail_dyn | 4404ns | 4046ns | 4933ns | +2.52% | 0.015 |
| kernel_nway | 17251ns | 14401ns | 21659ns | +301.61% | 0.004 |
| kernel_seq | 2502ns | 2448ns | 2557ns | -41.74% | 0.026 |
| nway | 49470ns | 46004ns | 51867ns | +1051.67% | 0.001 |
| sequential | 10359ns | 9963ns | 10828ns | +141.16% | 0.006 |
| std_parallel | 85144ns | 77293ns | 90705ns | +1882.19% | 0.001 |

## Performance model

- Peak throughput: **0.026 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.016 | 60.0% |
| head_tail_dyn | 0.015 | 58.6% |
| kernel_nway | 0.004 | 16.0% |
| kernel_seq | 0.026 | 98.2% |
| nway | 0.001 | 4.9% |
| sequential | 0.006 | 23.9% |
| std_parallel | 0.001 | 2.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4913ns | 4913ns | base |
| head_tail_dyn | 5021ns | 5021ns | +2.18% |
| kernel_nway | 17878ns | 17878ns | +263.85% |
| kernel_seq | 2833ns | 2833ns | -42.35% |
| nway | 50030ns | 50030ns | +918.23% |
| sequential | 10505ns | 10505ns | +113.79% |
| std_parallel | 85503ns | 85503ns | +1640.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4082ns | base | --- | [4007, 4321] | --- | --- | --- | --- |
| head_tail_dyn | 4180ns | no significant difference | [-193, +500]ns | [4127, 4507] | no | 0.4807 | 0.4807 | 0 |
| kernel_nway | 15333ns | +11169.1ns (+273.6%) | [+10690, +14366]ns | [14851, 18878] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2493ns | -1613.3ns (-39.5%) | [-1806, -1508]ns | [2468, 2518] | YES | 0.0000 | 0.0000 | 0 |
| nway | 49804ns | +45740.8ns (+1120.6%) | [+43748, +46132]ns | [47962, 50536] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10262ns | +6203.3ns (+152.0%) | [+5758, +6467]ns | [9983, 10482] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 85153ns | +80650.8ns (+1975.9%) | [+78227, +83521]ns | [83025, 87809] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4017ns | +3.0% | +289.1% | -37.7% | +1129.4% | +158.9% | +2146.4% |
| 1 | 2 | 4739ns | -12.8% | +220.6% | -47.3% | +964.9% | +110.6% | +1658.9% |
| 1 | 3 | 4024ns | +16.8% | +288.9% | -38.8% | +1126.0% | +158.2% | +1972.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.667 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.169 | ok |
| kernel_nway | -0.666 | HIGH- (thermal bounce) |
| kernel_seq | -0.077 | ok |
| nway | -0.666 | HIGH- (thermal bounce) |
| sequential | -0.666 | HIGH- (thermal bounce) |
| std_parallel | -0.170 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 97294.1ns | 4295.5ns | 2265.0% | HIGH |
| head_tail_dyn | 100143.5ns | 4403.9ns | 2274.0% | HIGH |
| kernel_nway | 111913.6ns | 17251.1ns | 648.7% | HIGH |
| kernel_seq | 89803.0ns | 2502.5ns | 3588.5% | HIGH |
| nway | 109557.2ns | 49469.8ns | 221.5% | HIGH |
| sequential | 90399.2ns | 10358.9ns | 872.7% | HIGH |
| std_parallel | 131834.9ns | 85144.4ns | 154.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 3955.6-4810.3 ns)
   3955.6 |########
   3998.3 |########################################
   4041.0 |########
   4083.8 |########
   4126.5 |
   4169.2 |########################
   4212.0 |
   4254.7 |
   4297.5 |
   4340.2 |
   4382.9 |
   4425.7 |########
   4468.4 |
   4511.1 |
   4553.9 |
   4596.6 |
   4639.3 |
   4682.1 |########
   4724.8 |
   4767.5 |########
  (2 below, 2 above range)

head_tail_dyn (n=18, range 4046.1-4932.8 ns)
   4046.1 |########################################
   4090.5 |##########################
   4134.8 |########################################
   4179.1 |#############
   4223.5 |
   4267.8 |##########################
   4312.1 |
   4356.5 |#############
   4400.8 |
   4445.1 |
   4489.5 |
   4533.8 |
   4578.1 |
   4622.5 |#############
   4666.8 |
   4711.1 |
   4755.5 |#############
   4799.8 |
   4844.1 |
   4888.5 |
  (1 below, 3 above range)

kernel_nway (n=18, range 14400.6-21659.2 ns)
  14400.6 |########################################
  14763.5 |##############################
  15126.4 |####################
  15489.4 |
  15852.3 |##############################
  16215.2 |
  16578.1 |
  16941.1 |
  17304.0 |
  17666.9 |
  18029.9 |
  18392.8 |
  18755.7 |
  19118.7 |
  19481.6 |
  19844.5 |
  20207.4 |
  20570.4 |
  20933.3 |
  21296.2 |##########
  (1 below, 4 above range)

kernel_seq (n=18, range 2447.8-2557.2 ns)
   2447.8 |
   2453.2 |#############
   2458.7 |##########################
   2464.2 |##########################
   2469.7 |
   2475.1 |
   2480.6 |#############
   2486.1 |#############
   2491.6 |########################################
   2497.0 |
   2502.5 |#############
   2508.0 |#############
   2513.4 |
   2518.9 |#############
   2524.4 |
   2529.9 |
   2535.3 |
   2540.8 |
   2546.3 |#############
   2551.8 |#############
  (1 below, 2 above range)

nway (n=18, range 46004.5-51866.7 ns)
  46004.5 |
  46297.6 |
  46590.7 |#############
  46883.8 |#############
  47176.9 |
  47470.0 |
  47763.1 |########################################
  48056.2 |
  48349.3 |#############
  48642.5 |
  48935.6 |#############
  49228.7 |
  49521.8 |#############
  49814.9 |#############
  50108.0 |########################################
  50401.1 |
  50694.2 |##########################
  50987.3 |
  51280.4 |
  51573.6 |
  (1 below, 3 above range)

sequential (n=18, range 9963.3-10828.3 ns)
   9963.3 |########################################
  10006.6 |
  10049.8 |
  10093.1 |
  10136.3 |##########
  10179.6 |####################
  10222.8 |
  10266.1 |
  10309.3 |##########
  10352.6 |##########
  10395.8 |##########
  10439.1 |##########
  10482.3 |##########
  10525.6 |
  10568.8 |
  10612.1 |
  10655.3 |
  10698.6 |
  10741.8 |
  10785.1 |##########
  (2 below, 3 above range)

std_parallel (n=18, range 77292.8-90705.0 ns)
  77292.8 |####################
  77963.4 |########################################
  78634.0 |
  79304.6 |
  79975.2 |
  80645.9 |
  81316.5 |
  81987.1 |
  82657.7 |########################################
  83328.3 |########################################
  83998.9 |
  84669.5 |####################
  85340.1 |####################
  86010.7 |
  86681.3 |########################################
  87351.9 |########################################
  88022.6 |
  88693.2 |####################
  89363.8 |
  90034.4 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=2394.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2386.6% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=20.3% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=739.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3597.9% of algo (FFI overhead may distort results)
- **nway**: bridge=219.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=879.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=156.7% of algo (FFI overhead may distort results)

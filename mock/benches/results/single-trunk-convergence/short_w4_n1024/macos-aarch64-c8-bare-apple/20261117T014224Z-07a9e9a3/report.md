# fold_light, short fiber (1 to 4 morsels), W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 60% faster than the next best (head_tail_dyn)

kernel_seq (2.59 us) leads head_tail_dyn (4.14 us) by 60%, a clear separation rather than a photo finish. CV 4.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 39% (significant)

kernel_seq is -1.61 us (39%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 20.9x slower than the field

std_parallel (54.12 us) is 20.9x the fastest (2.59 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.67)

head_tail's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 20.9x the fastest

Fastest kernel_seq (2.59 us) to slowest std_parallel (54.12 us): 20.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2588.3 ns median (-37.7% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 20.91x (fastest 2588.3 ns, slowest 54115.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 4 |
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
| head_tail | 4989ns | 4753ns | 4648ns | 4731ns | 5546ns | base |
| head_tail_dyn | 4735ns | 4726ns | 4644ns | 4701ns | 4831ns | -5.08% |
| kernel_nway | 10530ns | 10198ns | 9296ns | 10006ns | 11934ns | +111.08% |
| kernel_seq | 2989ns | 2886ns | 2723ns | 2862ns | 3313ns | -40.09% |
| nway | 33074ns | 33583ns | 29584ns | 32963ns | 34985ns | +562.97% |
| sequential | 10365ns | 10326ns | 10093ns | 10254ns | 10667ns | +107.76% |
| std_parallel | 55034ns | 54502ns | 51027ns | 54022ns | 58554ns | +1003.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4383ns | 4037ns | 4919ns | base | 0.234 |
| head_tail_dyn | 4156ns | 4034ns | 4264ns | -5.17% | 0.246 |
| kernel_nway | 9915ns | 8682ns | 11308ns | +126.23% | 0.103 |
| kernel_seq | 2629ns | 2515ns | 2773ns | -40.02% | 0.390 |
| nway | 32524ns | 29033ns | 34446ns | +642.11% | 0.031 |
| sequential | 10222ns | 9958ns | 10515ns | +133.23% | 0.100 |
| std_parallel | 54640ns | 50634ns | 58160ns | +1146.73% | 0.019 |

## Performance model

- Peak throughput: **0.407 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.247 | 60.6% |
| head_tail_dyn | 0.248 | 60.8% |
| kernel_nway | 0.107 | 26.2% |
| kernel_seq | 0.396 | 97.2% |
| nway | 0.031 | 7.6% |
| sequential | 0.101 | 24.7% |
| std_parallel | 0.019 | 4.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4989ns | 4989ns | base |
| head_tail_dyn | 4735ns | 4735ns | -5.08% |
| kernel_nway | 10530ns | 10530ns | +111.08% |
| kernel_seq | 2989ns | 2989ns | -40.09% |
| nway | 33074ns | 33074ns | +562.97% |
| sequential | 10365ns | 10365ns | +107.76% |
| std_parallel | 55034ns | 55034ns | +1003.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4153ns | base | --- | [4102, 4642] | --- | --- | --- | --- |
| head_tail_dyn | 4136ns | no significant difference | [-456, +75]ns | [4099, 4197] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 9598ns | +5310.9ns (+127.9%) | [+4864, +6682]ns | [9130, 10717] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2588ns | -1612.5ns (-38.8%) | [-1826, -1562]ns | [2528, 2667] | YES | 0.0000 | 0.0000 | 0 |
| nway | 33034ns | +28615.0ns (+689.0%) | [+26900, +29643]ns | [31812, 33902] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10186ns | +5878.3ns (+141.5%) | [+5592, +5963]ns | [10034, 10239] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 54116ns | +49968.3ns (+1203.1%) | [+48758, +52838]ns | [53444, 56928] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4122ns | +1.0% | +151.4% | -38.8% | +714.8% | +143.7% | +1249.6% |
| 1 | 2 | 4727ns | -12.7% | +97.8% | -41.8% | +579.6% | +116.4% | +1030.7% |
| 1 | 3 | 4102ns | +0.4% | +122.9% | -37.3% | +714.4% | +147.9% | +1258.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.666 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.101 | ok |
| kernel_nway | -0.086 | ok |
| kernel_seq | -0.641 | HIGH- (thermal bounce) |
| nway | -0.658 | HIGH- (thermal bounce) |
| sequential | -0.370 | moderate- |
| std_parallel | -0.666 | HIGH- (thermal bounce) |

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
| head_tail | 96298.8ns | 4382.7ns | 2197.3% | HIGH |
| head_tail_dyn | 95604.7ns | 4155.9ns | 2300.4% | HIGH |
| kernel_nway | 123686.4ns | 9914.8ns | 1247.5% | HIGH |
| kernel_seq | 91864.4ns | 2628.7ns | 3494.7% | HIGH |
| nway | 112690.7ns | 32524.4ns | 346.5% | HIGH |
| sequential | 91337.2ns | 10221.7ns | 893.6% | HIGH |
| std_parallel | 116678.7ns | 54640.5ns | 213.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 4036.7-4919.4 ns)
   4036.7 |##############################
   4080.8 |####################
   4124.9 |########################################
   4169.1 |##########
   4213.2 |##########
   4257.4 |
   4301.5 |
   4345.6 |
   4389.8 |
   4433.9 |
   4478.1 |
   4522.2 |
   4566.3 |
   4610.5 |##########
   4654.6 |##########
   4698.8 |####################
   4742.9 |
   4787.0 |
   4831.2 |
   4875.3 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 4033.9-4263.6 ns)
   4033.9 |####################
   4045.4 |####################
   4056.9 |
   4068.4 |
   4079.8 |####################
   4091.3 |####################
   4102.8 |####################
   4114.3 |########################################
   4125.8 |####################
   4137.3 |####################
   4148.8 |####################
   4160.2 |
   4171.7 |########################################
   4183.2 |
   4194.7 |
   4206.2 |####################
   4217.7 |
   4229.1 |
   4240.6 |####################
   4252.1 |####################
  (1 below, 2 above range)

kernel_nway (n=18, range 8681.7-11308.1 ns)
   8681.7 |
   8813.0 |################
   8944.3 |
   9075.7 |########################################
   9207.0 |
   9338.3 |
   9469.6 |########
   9600.9 |################
   9732.2 |
   9863.6 |
   9994.9 |########
  10126.2 |
  10257.5 |
  10388.8 |
  10520.2 |
  10651.5 |########
  10782.8 |
  10914.1 |################
  11045.4 |
  11176.7 |
  (1 below, 3 above range)

kernel_seq (n=18, range 2515.0-2772.8 ns)
   2515.0 |########################################
   2527.9 |##########
   2540.8 |##########
   2553.7 |
   2566.6 |##########
   2579.4 |##########
   2592.3 |##########
   2605.2 |
   2618.1 |##########
   2631.0 |##########
   2643.9 |##########
   2656.8 |
   2669.7 |##########
   2682.6 |
   2695.4 |
   2708.3 |
   2721.2 |
   2734.1 |##########
   2747.0 |
   2759.9 |##########
  (1 below, 2 above range)

nway (n=18, range 29032.8-34445.8 ns)
  29032.8 |
  29303.4 |#############
  29574.1 |#############
  29844.7 |
  30115.4 |
  30386.0 |#############
  30656.7 |
  30927.3 |
  31198.0 |
  31468.6 |
  31739.3 |########################################
  32010.0 |
  32280.6 |#############
  32551.3 |
  32821.9 |#############
  33092.6 |#############
  33363.2 |##########################
  33633.9 |#############
  33904.5 |
  34175.2 |########################################
  (1 below, 2 above range)

sequential (n=18, range 9958.3-10515.0 ns)
   9958.3 |#############
   9986.1 |
  10014.0 |########################################
  10041.8 |##########################
  10069.6 |
  10097.5 |
  10125.3 |
  10153.1 |#############
  10181.0 |#############
  10208.8 |##########################
  10236.6 |##########################
  10264.5 |#############
  10292.3 |
  10320.2 |
  10348.0 |#############
  10375.8 |
  10403.7 |
  10431.5 |
  10459.3 |
  10487.2 |
  (2 below, 2 above range)

std_parallel (n=18, range 50633.9-58159.7 ns)
  50633.9 |########################################
  51010.2 |
  51386.5 |
  51762.8 |
  52139.1 |
  52515.4 |
  52891.6 |
  53267.9 |##########################
  53644.2 |##########################
  54020.5 |########################################
  54396.8 |#############
  54773.1 |
  55149.4 |
  55525.7 |
  55902.0 |
  56278.3 |
  56654.6 |########################################
  57030.8 |#############
  57407.1 |#############
  57783.4 |
  (1 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=2319.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2322.6% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1270.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3498.9% of algo (FFI overhead may distort results)
- **nway**: bridge=336.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=897.2% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=215.5% of algo (FFI overhead may distort results)

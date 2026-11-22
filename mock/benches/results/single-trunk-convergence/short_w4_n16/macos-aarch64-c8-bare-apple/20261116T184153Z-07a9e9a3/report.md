# fold_light, short fiber (1 to 4 morsels), W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 61% faster than the next best (head_tail)

kernel_seq (2.57 us) leads head_tail (4.15 us) by 61%, a clear separation rather than a photo finish. CV 3.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 38% (significant)

kernel_seq is -1.59 us (38%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 20.7x slower than the field

std_parallel (53.24 us) is 20.7x the fastest (2.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.64)

kernel_nway's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 20.7x the fastest

Fastest kernel_seq (2.57 us) to slowest std_parallel (53.24 us): 20.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 1.6x its best-20%

kernel_nway's best 20% of batches run at 8.60 us but its worst 20% at 13.73 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 2570.0 ns median (-38.0% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 20.71x (fastest 2570.0 ns, slowest 53236.7 ns)

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
| head_tail | 4821ns | 4770ns | 4563ns | 4741ns | 5070ns | base |
| head_tail_dyn | 4803ns | 4822ns | 4642ns | 4775ns | 4925ns | -0.38% |
| kernel_nway | 11538ns | 10608ns | 9212ns | 10418ns | 14381ns | +139.32% |
| kernel_seq | 3018ns | 2996ns | 2817ns | 2973ns | 3187ns | -37.40% |
| nway | 33807ns | 33644ns | 31776ns | 33436ns | 35380ns | +601.22% |
| sequential | 10432ns | 10388ns | 9929ns | 10322ns | 10847ns | +116.37% |
| std_parallel | 53522ns | 53641ns | 50137ns | 52837ns | 56242ns | +1010.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4219ns | 3972ns | 4479ns | base | 0.004 |
| head_tail_dyn | 4196ns | 4059ns | 4297ns | -0.54% | 0.004 |
| kernel_nway | 10902ns | 8603ns | 13733ns | +158.42% | 0.001 |
| kernel_seq | 2574ns | 2456ns | 2668ns | -38.98% | 0.006 |
| nway | 33254ns | 31223ns | 34832ns | +688.24% | 0.000 |
| sequential | 10289ns | 9796ns | 10705ns | +143.90% | 0.002 |
| std_parallel | 53112ns | 49717ns | 55831ns | +1158.93% | 0.000 |

## Performance model

- Peak throughput: **0.007 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.004 | 59.2% |
| head_tail_dyn | 0.004 | 58.4% |
| kernel_nway | 0.002 | 24.6% |
| kernel_seq | 0.006 | 95.5% |
| nway | 0.000 | 7.4% |
| sequential | 0.002 | 24.0% |
| std_parallel | 0.000 | 4.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4821ns | 4821ns | base |
| head_tail_dyn | 4803ns | 4803ns | -0.38% |
| kernel_nway | 11538ns | 11538ns | +139.32% |
| kernel_seq | 3018ns | 3018ns | -37.40% |
| nway | 33807ns | 33807ns | +601.22% |
| sequential | 10432ns | 10432ns | +116.37% |
| std_parallel | 53522ns | 53522ns | +1010.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4146ns | base | --- | [4078, 4218] | --- | --- | --- | --- |
| head_tail_dyn | 4202ns | no significant difference | [-29, +144]ns | [4125, 4262] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 9978ns | +5702.5ns (+137.5%) | [+5134, +8372]ns | [9412, 12207] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2570ns | -1590.0ns (-38.4%) | [-1658, -1482]ns | [2533, 2591] | YES | 0.0000 | 0.0000 | 0 |
| nway | 33109ns | +28905.0ns (+697.2%) | [+28163, +29719]ns | [32405, 34108] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10242ns | +6156.6ns (+148.5%) | [+5817, +6194]ns | [10152, 10293] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 53237ns | +48875.0ns (+1178.9%) | [+47268, +50303]ns | [51428, 54669] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4150ns | -0.2% | +138.4% | -38.5% | +683.9% | +147.4% | +1210.8% |
| 1 | 2 | 4203ns | +1.3% | +142.3% | -38.4% | +690.8% | +141.8% | +1186.7% |
| 1 | 3 | 4132ns | +2.1% | +137.5% | -37.5% | +728.1% | +143.4% | +1138.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.626 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.379 | moderate- |
| kernel_nway | -0.638 | HIGH- (thermal bounce) |
| kernel_seq | -0.295 | moderate- |
| nway | -0.005 | ok |
| sequential | 0.000 | ok |
| std_parallel | -0.118 | ok |

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
| head_tail | 98779.2ns | 4218.8ns | 2341.4% | HIGH |
| head_tail_dyn | 98408.0ns | 4195.8ns | 2345.4% | HIGH |
| kernel_nway | 126275.5ns | 10902.4ns | 1158.2% | HIGH |
| kernel_seq | 92265.1ns | 2574.3ns | 3584.1% | HIGH |
| nway | 113471.7ns | 33254.4ns | 341.2% | HIGH |
| sequential | 91034.6ns | 10289.4ns | 884.7% | HIGH |
| std_parallel | 114014.9ns | 53111.7ns | 214.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 3971.7-4479.4 ns)
   3971.7 |
   3997.1 |
   4022.4 |
   4047.8 |####################
   4073.2 |####################
   4098.6 |
   4124.0 |########################################
   4149.4 |
   4174.8 |####################
   4200.2 |####################
   4225.6 |
   4250.9 |##########
   4276.3 |
   4301.7 |
   4327.1 |
   4352.5 |
   4377.9 |
   4403.3 |
   4428.7 |
   4454.0 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 4058.9-4296.9 ns)
   4058.9 |
   4070.8 |
   4082.7 |
   4094.6 |########################################
   4106.5 |####################
   4118.4 |########################################
   4130.3 |
   4142.2 |
   4154.1 |####################
   4166.0 |
   4177.9 |
   4189.8 |########################################
   4201.7 |########################################
   4213.6 |
   4225.5 |
   4237.4 |########################################
   4249.3 |
   4261.2 |
   4273.1 |########################################
   4285.0 |####################
  (1 below, 2 above range)

kernel_nway (n=18, range 8603.3-13732.5 ns)
   8603.3 |
   8859.8 |##############################
   9116.3 |##########
   9372.7 |
   9629.2 |####################
   9885.6 |########################################
  10142.1 |##########
  10398.5 |
  10655.0 |
  10911.5 |
  11167.9 |
  11424.4 |
  11680.8 |
  11937.3 |
  12193.8 |##########
  12450.2 |
  12706.7 |####################
  12963.1 |
  13219.6 |
  13476.1 |
  (1 below, 3 above range)

kernel_seq (n=18, range 2455.5-2667.5 ns)
   2455.5 |
   2466.1 |##########################
   2476.7 |#############
   2487.3 |
   2497.9 |
   2508.5 |
   2519.1 |#############
   2529.7 |#############
   2540.3 |#############
   2550.9 |#############
   2561.5 |#############
   2572.1 |##########################
   2582.7 |########################################
   2593.3 |
   2603.9 |
   2614.5 |
   2625.1 |
   2635.7 |#############
   2646.3 |##########################
   2656.9 |
  (1 below, 1 above range)

nway (n=18, range 31222.8-34831.7 ns)
  31222.8 |####################
  31403.2 |
  31583.7 |####################
  31764.1 |
  31944.6 |####################
  32125.0 |####################
  32305.5 |
  32485.9 |####################
  32666.3 |########################################
  32846.8 |####################
  33027.2 |
  33207.7 |####################
  33388.1 |####################
  33568.6 |
  33749.0 |####################
  33929.4 |####################
  34109.9 |####################
  34290.3 |####################
  34470.8 |
  34651.2 |####################
  (1 below, 2 above range)

sequential (n=18, range 9796.1-10705.3 ns)
   9796.1 |
   9841.6 |##########################
   9887.0 |
   9932.5 |
   9978.0 |
  10023.4 |
  10068.9 |
  10114.3 |##########################
  10159.8 |#############
  10205.2 |########################################
  10250.7 |########################################
  10296.2 |#############
  10341.6 |
  10387.1 |#############
  10432.5 |
  10478.0 |#############
  10523.4 |
  10568.9 |
  10614.4 |
  10659.8 |
  (2 below, 2 above range)

std_parallel (n=18, range 49716.7-55831.4 ns)
  49716.7 |
  50022.4 |####################
  50328.1 |
  50633.9 |
  50939.6 |####################
  51245.3 |########################################
  51551.1 |####################
  51856.8 |####################
  52162.6 |
  52468.3 |
  52774.0 |####################
  53079.8 |
  53385.5 |########################################
  53691.2 |####################
  53997.0 |
  54302.7 |####################
  54608.5 |####################
  54914.2 |####################
  55219.9 |
  55525.7 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=2367.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2338.6% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=20.0% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=1229.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3597.7% of algo (FFI overhead may distort results)
- **nway**: bridge=344.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=894.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=216.9% of algo (FFI overhead may distort results)

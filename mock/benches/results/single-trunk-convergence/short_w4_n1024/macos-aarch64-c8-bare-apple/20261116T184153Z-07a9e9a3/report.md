# fold_light, short fiber (1 to 4 morsels), W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 51% faster than the next best (head_tail_dyn)

kernel_seq (2.86 us) leads head_tail_dyn (4.32 us) by 51%, a clear separation rather than a photo finish. CV 8.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 35% (significant)

kernel_seq is -1.53 us (35%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 19.1x slower than the field

std_parallel (54.72 us) is 19.1x the fastest (2.86 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_parallel shows alternating (throttle bounce) (autocorr -0.65)

std_parallel's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 19.1x the fastest

Fastest kernel_seq (2.86 us) to slowest std_parallel (54.72 us): 19.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2858.3 ns median (-34.7% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 19.14x (fastest 2858.3 ns, slowest 54720.0 ns)

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
| head_tail | 5108ns | 5011ns | 4736ns | 4947ns | 5536ns | base |
| head_tail_dyn | 5088ns | 4913ns | 4729ns | 4894ns | 5558ns | -0.40% |
| kernel_nway | 9874ns | 10163ns | 7831ns | 9616ns | 11282ns | +93.30% |
| kernel_seq | 3395ns | 3435ns | 2818ns | 3370ns | 3722ns | -33.53% |
| nway | 33917ns | 33447ns | 30272ns | 33145ns | 36897ns | +564.00% |
| sequential | 10736ns | 10534ns | 10107ns | 10490ns | 11420ns | +110.19% |
| std_parallel | 56345ns | 55125ns | 51258ns | 54622ns | 61474ns | +1003.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4483ns | 4125ns | 4895ns | base | 0.228 |
| head_tail_dyn | 4500ns | 4139ns | 4962ns | +0.37% | 0.228 |
| kernel_nway | 9296ns | 7238ns | 10639ns | +107.36% | 0.110 |
| kernel_seq | 2864ns | 2544ns | 3131ns | -36.12% | 0.358 |
| nway | 33361ns | 29691ns | 36340ns | +644.16% | 0.031 |
| sequential | 10581ns | 9957ns | 11260ns | +136.02% | 0.097 |
| std_parallel | 55927ns | 50842ns | 61059ns | +1147.52% | 0.018 |

## Performance model

- Peak throughput: **0.403 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.234 | 58.1% |
| head_tail_dyn | 0.237 | 58.8% |
| kernel_nway | 0.107 | 26.6% |
| kernel_seq | 0.358 | 89.0% |
| nway | 0.031 | 7.7% |
| sequential | 0.099 | 24.5% |
| std_parallel | 0.019 | 4.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 5108ns | 5108ns | base |
| head_tail_dyn | 5088ns | 5088ns | -0.40% |
| kernel_nway | 9874ns | 9874ns | +93.30% |
| kernel_seq | 3395ns | 3395ns | -33.53% |
| nway | 33917ns | 33917ns | +564.00% |
| sequential | 10736ns | 10736ns | +110.19% |
| std_parallel | 56345ns | 56345ns | +1003.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4375ns | base | --- | [4260, 4545] | --- | --- | --- | --- |
| head_tail_dyn | 4323ns | no significant difference | [-203, +186]ns | [4275, 4554] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 9572ns | +4675.0ns (+106.9%) | [+3922, +5718]ns | [8525, 9864] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2858ns | -1531.7ns (-35.0%) | [-1701, -1432]ns | [2698, 2981] | YES | 0.0000 | 0.0000 | 0 |
| nway | 32908ns | +28562.5ns (+652.9%) | [+27425, +29562]ns | [31678, 33923] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10380ns | +6096.7ns (+139.4%) | [+5568, +6342]ns | [10248, 10913] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 54720ns | +50339.2ns (+1150.6%) | [+48478, +53708]ns | [52894, 58148] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4358ns | +0.3% | +116.3% | -32.0% | +653.5% | +161.6% | +1184.9% |
| 1 | 2 | 4647ns | -7.3% | +79.8% | -36.6% | +592.3% | +123.3% | +1055.5% |
| 1 | 3 | 4260ns | +1.2% | +131.4% | -36.3% | +695.6% | +141.7% | +1204.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.627 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.188 | ok |
| kernel_nway | -0.615 | HIGH- (thermal bounce) |
| kernel_seq | -0.134 | ok |
| nway | -0.420 | moderate- |
| sequential | -0.131 | ok |
| std_parallel | -0.647 | HIGH- (thermal bounce) |

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
| head_tail | 96034.3ns | 4483.1ns | 2142.2% | HIGH |
| head_tail_dyn | 97149.7ns | 4499.5ns | 2159.1% | HIGH |
| kernel_nway | 112769.4ns | 9295.9ns | 1213.1% | HIGH |
| kernel_seq | 94243.2ns | 2863.8ns | 3290.9% | HIGH |
| nway | 110429.2ns | 33361.1ns | 331.0% | HIGH |
| sequential | 91174.5ns | 10580.9ns | 861.7% | HIGH |
| std_parallel | 116045.1ns | 55927.1ns | 207.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 4125.0-4894.7 ns)
   4125.0 |
   4163.5 |#############
   4202.0 |
   4240.5 |########################################
   4278.9 |##########################
   4317.4 |
   4355.9 |##########################
   4394.4 |##########################
   4432.9 |
   4471.4 |
   4509.9 |#############
   4548.3 |#############
   4586.8 |
   4625.3 |
   4663.8 |
   4702.3 |
   4740.8 |#############
   4779.3 |
   4817.7 |
   4856.2 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 4138.9-4962.0 ns)
   4138.9 |########
   4180.1 |########
   4221.2 |########
   4262.4 |################
   4303.5 |########################################
   4344.7 |
   4385.8 |########
   4427.0 |
   4468.1 |
   4509.3 |########
   4550.4 |########
   4591.6 |################
   4632.7 |
   4673.9 |
   4715.0 |
   4756.2 |
   4797.4 |
   4838.5 |
   4879.7 |
   4920.8 |
  (1 below, 2 above range)

kernel_nway (n=18, range 7238.4-10638.6 ns)
   7238.4 |
   7408.4 |
   7578.4 |
   7748.4 |##########
   7918.4 |##########
   8088.4 |##########
   8258.4 |
   8428.4 |
   8598.5 |
   8768.5 |##########
   8938.5 |####################
   9108.5 |
   9278.5 |##########
   9448.5 |
   9618.5 |########################################
   9788.5 |##########
   9958.6 |
  10128.6 |##########
  10298.6 |##########
  10468.6 |
  (2 below, 2 above range)

kernel_seq (n=18, range 2543.9-3130.8 ns)
   2543.9 |
   2573.2 |####################
   2602.6 |####################
   2631.9 |####################
   2661.3 |
   2690.6 |
   2720.0 |
   2749.3 |####################
   2778.7 |####################
   2808.0 |####################
   2837.4 |########################################
   2866.7 |####################
   2896.1 |
   2925.4 |####################
   2954.8 |####################
   2984.1 |####################
   3013.4 |
   3042.8 |
   3072.1 |####################
   3101.5 |####################
  (2 below, 2 above range)

nway (n=18, range 29690.6-36340.3 ns)
  29690.6 |####################
  30023.1 |
  30355.5 |
  30688.0 |
  31020.5 |
  31353.0 |####################
  31685.5 |##########
  32018.0 |##########
  32350.5 |
  32682.9 |##############################
  33015.4 |
  33347.9 |
  33680.4 |########################################
  34012.9 |
  34345.4 |
  34677.9 |##########
  35010.3 |
  35342.8 |
  35675.3 |
  36007.8 |##########
  (1 below, 2 above range)

sequential (n=18, range 9957.2-11260.0 ns)
   9957.2 |##########
  10022.4 |
  10087.5 |##########
  10152.6 |##########
  10217.8 |
  10282.9 |####################
  10348.1 |########################################
  10413.2 |
  10478.3 |
  10543.5 |##########
  10608.6 |##########
  10673.7 |
  10738.9 |
  10804.0 |
  10869.2 |
  10934.3 |
  10999.4 |
  11064.6 |
  11129.7 |
  11194.8 |
  (2 below, 5 above range)

std_parallel (n=18, range 50841.7-61058.6 ns)
  50841.7 |
  51352.5 |
  51863.4 |#############
  52374.2 |##########################
  52885.1 |##########################
  53395.9 |#############
  53906.8 |#############
  54417.6 |
  54928.4 |
  55439.3 |#############
  55950.1 |#############
  56461.0 |#############
  56971.8 |
  57482.7 |
  57993.5 |########################################
  58504.4 |
  59015.2 |#############
  59526.1 |
  60036.9 |
  60547.8 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=2191.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2229.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1195.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3301.8% of algo (FFI overhead may distort results)
- **nway**: bridge=336.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=877.6% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=209.7% of algo (FFI overhead may distort results)

# fold_light, short fiber (1 to 4 morsels), W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 63% faster than the next best (head_tail_dyn)

kernel_seq (2.51 us) leads head_tail_dyn (4.09 us) by 63%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 39% (significant)

kernel_seq is -1.59 us (39%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 32.9x slower than the field

std_parallel (82.49 us) is 32.9x the fastest (2.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.65)

head_tail's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 32.9x the fastest

Fastest kernel_seq (2.51 us) to slowest std_parallel (82.49 us): 32.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2510.8 ns median (-39.0% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 32.85x (fastest 2510.8 ns, slowest 82491.7 ns)

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
| head_tail | 4818ns | 4716ns | 4554ns | 4685ns | 5149ns | base |
| head_tail_dyn | 4697ns | 4669ns | 4623ns | 4671ns | 4771ns | -2.52% |
| kernel_nway | 16735ns | 15622ns | 14892ns | 15562ns | 19414ns | +247.34% |
| kernel_seq | 2831ns | 2792ns | 2723ns | 2781ns | 2959ns | -41.25% |
| nway | 51233ns | 51104ns | 47948ns | 50888ns | 53392ns | +963.37% |
| sequential | 10790ns | 10779ns | 10072ns | 10650ns | 11360ns | +123.96% |
| std_parallel | 81785ns | 82887ns | 74733ns | 80785ns | 86811ns | +1597.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4218ns | 3970ns | 4547ns | base | 0.061 |
| head_tail_dyn | 4089ns | 3970ns | 4178ns | -3.07% | 0.063 |
| kernel_nway | 16103ns | 14265ns | 18774ns | +281.73% | 0.016 |
| kernel_seq | 2528ns | 2452ns | 2615ns | -40.07% | 0.101 |
| nway | 50668ns | 47388ns | 52827ns | +1101.11% | 0.005 |
| sequential | 10640ns | 9932ns | 11213ns | +152.22% | 0.024 |
| std_parallel | 81403ns | 74311ns | 86449ns | +1829.70% | 0.003 |

## Performance model

- Peak throughput: **0.104 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.062 | 59.6% |
| head_tail_dyn | 0.063 | 60.0% |
| kernel_nway | 0.017 | 16.3% |
| kernel_seq | 0.102 | 97.6% |
| nway | 0.005 | 4.9% |
| sequential | 0.024 | 23.1% |
| std_parallel | 0.003 | 3.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4818ns | 4818ns | base |
| head_tail_dyn | 4697ns | 4697ns | -2.52% |
| kernel_nway | 16735ns | 16735ns | +247.34% |
| kernel_seq | 2831ns | 2831ns | -41.25% |
| nway | 51233ns | 51233ns | +963.37% |
| sequential | 10790ns | 10790ns | +123.96% |
| std_parallel | 81785ns | 81785ns | +1597.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4113ns | base | --- | [4035, 4227] | --- | --- | --- | --- |
| head_tail_dyn | 4087ns | no significant difference | [-187, +62]ns | [4045, 4147] | no | 0.2379 | 0.2379 | 0 |
| kernel_nway | 15012ns | +10973.3ns (+266.8%) | [+10352, +12045]ns | [14572, 16075] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2511ns | -1590.0ns (-38.7%) | [-1653, -1515]ns | [2472, 2534] | YES | 0.0000 | 0.0000 | 0 |
| nway | 50529ns | +46310.8ns (+1125.9%) | [+45390, +47944]ns | [49605, 52052] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10615ns | +6383.4ns (+155.2%) | [+6108, +6778]ns | [10310, 10866] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 82492ns | +78397.5ns (+1905.9%) | [+73065, +80926]ns | [77262, 84952] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4101ns | -1.1% | +260.0% | -38.9% | +1111.2% | +176.5% | +1918.9% |
| 1 | 2 | 4483ns | -7.8% | +242.2% | -44.1% | +1019.3% | +136.8% | +1682.4% |
| 1 | 3 | 4030ns | +2.0% | +287.8% | -37.6% | +1181.6% | +153.8% | +1969.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.653 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.375 | moderate- |
| kernel_nway | -0.024 | ok |
| kernel_seq | -0.032 | ok |
| nway | -0.048 | ok |
| sequential | -0.020 | ok |
| std_parallel | -0.648 | HIGH- (thermal bounce) |

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
| head_tail | 97089.9ns | 4218.4ns | 2301.6% | HIGH |
| head_tail_dyn | 97880.5ns | 4089.0ns | 2393.8% | HIGH |
| kernel_nway | 109064.6ns | 16103.0ns | 677.3% | HIGH |
| kernel_seq | 90502.9ns | 2528.1ns | 3579.9% | HIGH |
| nway | 111957.4ns | 50667.8ns | 221.0% | HIGH |
| sequential | 90930.1ns | 10639.6ns | 854.6% | HIGH |
| std_parallel | 131944.4ns | 81403.1ns | 162.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 3970.0-4546.6 ns)
   3970.0 |
   3998.9 |##########################
   4027.7 |########################################
   4056.5 |
   4085.4 |##########################
   4114.2 |#############
   4143.0 |##########################
   4171.8 |#############
   4200.7 |
   4229.5 |#############
   4258.3 |
   4287.2 |
   4316.0 |
   4344.8 |
   4373.7 |
   4402.5 |
   4431.3 |
   4460.2 |#############
   4489.0 |#############
   4517.8 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 3970.0-4178.1 ns)
   3970.0 |##########################
   3980.4 |
   3990.8 |
   4001.2 |
   4011.6 |
   4022.0 |
   4032.4 |##########################
   4042.8 |
   4053.2 |########################################
   4063.6 |
   4074.0 |#############
   4084.4 |#############
   4094.8 |#############
   4105.2 |#############
   4115.6 |
   4126.0 |#############
   4136.4 |
   4146.8 |
   4157.2 |#############
   4167.6 |##########################
  (1 below, 2 above range)

kernel_nway (n=18, range 14265.0-18773.9 ns)
  14265.0 |#############
  14490.4 |########################################
  14715.9 |########################################
  14941.3 |
  15166.8 |##########################
  15392.2 |
  15617.7 |
  15843.1 |##########################
  16068.6 |#############
  16294.0 |#############
  16519.4 |
  16744.9 |
  16970.3 |
  17195.8 |
  17421.2 |
  17646.7 |
  17872.1 |
  18097.6 |
  18323.0 |
  18548.5 |
  (2 below, 3 above range)

kernel_seq (n=18, range 2451.7-2614.7 ns)
   2451.7 |########################################
   2459.8 |########################################
   2468.0 |
   2476.1 |####################
   2484.3 |
   2492.4 |########################################
   2500.6 |####################
   2508.7 |####################
   2516.9 |########################################
   2525.0 |####################
   2533.2 |
   2541.4 |####################
   2549.5 |
   2557.7 |####################
   2565.8 |
   2574.0 |
   2582.1 |
   2590.3 |
   2598.4 |####################
   2606.6 |
  (1 below, 2 above range)

nway (n=18, range 47387.8-52827.2 ns)
  47387.8 |
  47659.7 |
  47931.7 |
  48203.7 |
  48475.7 |#############
  48747.6 |
  49019.6 |
  49291.6 |##########################
  49563.5 |
  49835.5 |########################################
  50107.5 |#############
  50379.5 |
  50651.4 |#############
  50923.4 |
  51195.4 |
  51467.4 |#############
  51739.3 |#############
  52011.3 |##########################
  52283.3 |#############
  52555.2 |
  (2 below, 3 above range)

sequential (n=18, range 9931.7-11213.1 ns)
   9931.7 |
   9995.7 |
  10059.8 |
  10123.9 |##########################
  10187.9 |
  10252.0 |#############
  10316.1 |#############
  10380.2 |
  10444.2 |#############
  10508.3 |
  10572.4 |########################################
  10636.4 |#############
  10700.5 |#############
  10764.6 |#############
  10828.6 |
  10892.7 |#############
  10956.8 |
  11020.9 |
  11084.9 |
  11149.0 |
  (2 below, 4 above range)

std_parallel (n=18, range 74310.6-86449.4 ns)
  74310.6 |##########################
  74917.5 |
  75524.5 |
  76131.4 |#############
  76738.3 |#############
  77345.3 |##########################
  77952.2 |
  78559.2 |
  79166.1 |
  79773.1 |
  80380.0 |#############
  80987.0 |
  81593.9 |#############
  82200.8 |
  82807.8 |#############
  83414.7 |
  84021.7 |#############
  84628.6 |########################################
  85235.6 |#############
  85842.5 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=2372.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2400.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=708.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3578.6% of algo (FFI overhead may distort results)
- **nway**: bridge=220.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=858.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=159.5% of algo (FFI overhead may distort results)

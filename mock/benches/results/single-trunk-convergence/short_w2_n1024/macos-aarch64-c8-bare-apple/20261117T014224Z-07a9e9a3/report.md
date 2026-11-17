# fold_light, short fiber (1 to 4 morsels), W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 59% faster than the next best (kernel_nway)

kernel_seq (2.57 us) leads kernel_nway (4.09 us) by 59%, a clear separation rather than a photo finish. CV 7.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 38% (significant)

kernel_seq is -1.57 us (38%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 12.4x slower than the field

std_parallel (32.04 us) is 12.4x the fastest (2.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.65)

head_tail's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 12.4x the fastest

Fastest kernel_seq (2.57 us) to slowest std_parallel (32.04 us): 12.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2574.2 ns median (-37.7% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 12.45x (fastest 2574.2 ns, slowest 32040.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 2 |
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
| head_tail | 4838ns | 4746ns | 4592ns | 4697ns | 5171ns | base |
| head_tail_dyn | 4765ns | 4767ns | 4658ns | 4736ns | 4861ns | -1.51% |
| kernel_nway | 4724ns | 4697ns | 4548ns | 4679ns | 4880ns | -2.34% |
| kernel_seq | 2987ns | 2848ns | 2719ns | 2837ns | 3347ns | -38.25% |
| nway | 18231ns | 18182ns | 18044ns | 18173ns | 18411ns | +276.84% |
| sequential | 10270ns | 10216ns | 9934ns | 10183ns | 10568ns | +112.29% |
| std_parallel | 33425ns | 32437ns | 30847ns | 32119ns | 36672ns | +590.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4218ns | 3950ns | 4557ns | base | 0.243 |
| head_tail_dyn | 4179ns | 4073ns | 4250ns | -0.91% | 0.245 |
| kernel_nway | 4123ns | 3985ns | 4257ns | -2.24% | 0.248 |
| kernel_seq | 2641ns | 2497ns | 2839ns | -37.37% | 0.388 |
| nway | 17742ns | 17545ns | 17922ns | +320.66% | 0.058 |
| sequential | 10131ns | 9798ns | 10427ns | +140.21% | 0.101 |
| std_parallel | 33054ns | 30477ns | 36301ns | +683.71% | 0.031 |

## Performance model

- Peak throughput: **0.410 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.248 | 60.4% |
| head_tail_dyn | 0.245 | 59.6% |
| kernel_nway | 0.250 | 61.0% |
| kernel_seq | 0.398 | 97.0% |
| nway | 0.058 | 14.1% |
| sequential | 0.102 | 24.8% |
| std_parallel | 0.032 | 7.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4838ns | 4838ns | base |
| head_tail_dyn | 4765ns | 4765ns | -1.51% |
| kernel_nway | 4724ns | 4724ns | -2.34% |
| kernel_seq | 2987ns | 2987ns | -38.25% |
| nway | 18231ns | 18231ns | +276.84% |
| sequential | 10270ns | 10270ns | +112.29% |
| std_parallel | 33425ns | 33425ns | +590.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4132ns | base | --- | [4016, 4328] | --- | --- | --- | --- |
| head_tail_dyn | 4187ns | no significant difference | [-207, +178]ns | [4133, 4232] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 4093ns | no significant difference | [-282, +97]ns | [4056, 4192] | no | 0.5768 | 0.4807 | 0 |
| kernel_seq | 2574ns | -1575.0ns (-38.1%) | [-1677, -1435]ns | [2542, 2631] | YES | 0.0000 | 0.0000 | 0 |
| nway | 17697ns | +13574.2ns (+328.5%) | [+13416, +13667]ns | [17642, 17854] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10074ns | +5967.4ns (+144.4%) | [+5757, +6076]ns | [10032, 10228] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 32041ns | +27545.0ns (+666.7%) | [+27032, +29252]ns | [31202, 33614] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 3980ns | +5.3% | +6.4% | -29.7% | +346.7% | +152.2% | +766.2% |
| 1 | 2 | 4458ns | -6.2% | -8.9% | -42.4% | +300.5% | +129.9% | +600.7% |
| 1 | 3 | 4068ns | +2.7% | +0.6% | -37.1% | +333.9% | +145.4% | +704.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.647 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.023 | ok |
| kernel_nway | -0.272 | moderate- |
| kernel_seq | -0.142 | ok |
| nway | -0.407 | moderate- |
| sequential | -0.639 | HIGH- (thermal bounce) |
| std_parallel | -0.475 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 96934.6ns | 4217.6ns | 2298.3% | HIGH |
| head_tail_dyn | 95322.5ns | 4179.2ns | 2280.9% | HIGH |
| kernel_nway | 96866.0ns | 4123.2ns | 2349.3% | HIGH |
| kernel_seq | 91026.0ns | 2641.5ns | 3446.0% | HIGH |
| nway | 91978.4ns | 17741.7ns | 518.4% | HIGH |
| sequential | 90472.9ns | 10130.9ns | 893.0% | HIGH |
| std_parallel | 105429.0ns | 33053.8ns | 319.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 3950.0-4556.7 ns)
   3950.0 |########################################
   3980.3 |####################
   4010.7 |
   4041.0 |########################################
   4071.3 |####################
   4101.7 |####################
   4132.0 |########################################
   4162.3 |####################
   4192.7 |
   4223.0 |
   4253.3 |
   4283.7 |####################
   4314.0 |
   4344.3 |####################
   4374.7 |
   4405.0 |
   4435.3 |
   4465.7 |####################
   4496.0 |
   4526.3 |####################
  (2 below, 2 above range)

head_tail_dyn (n=18, range 4072.8-4250.0 ns)
   4072.8 |####################
   4081.6 |
   4090.5 |
   4099.3 |########################################
   4108.2 |
   4117.1 |
   4125.9 |####################
   4134.8 |####################
   4143.7 |
   4152.5 |####################
   4161.4 |
   4170.2 |
   4179.1 |########################################
   4188.0 |####################
   4196.8 |####################
   4205.7 |
   4214.5 |
   4223.4 |########################################
   4232.3 |####################
   4241.1 |####################
  (1 below, 3 above range)

kernel_nway (n=18, range 3985.0-4256.9 ns)
   3985.0 |
   3998.6 |#############
   4012.2 |#############
   4025.8 |
   4039.4 |#############
   4053.0 |
   4066.6 |########################################
   4080.2 |#############
   4093.8 |#############
   4107.4 |#############
   4121.0 |#############
   4134.6 |
   4148.2 |
   4161.8 |
   4175.4 |#############
   4189.0 |
   4202.6 |#############
   4216.1 |
   4229.7 |
   4243.3 |
  (2 below, 4 above range)

kernel_seq (n=18, range 2497.2-2839.2 ns)
   2497.2 |########################################
   2514.3 |
   2531.4 |########################################
   2548.5 |#############
   2565.6 |########################################
   2582.7 |##########################
   2599.8 |
   2616.9 |
   2634.0 |
   2651.1 |
   2668.2 |#############
   2685.3 |
   2702.4 |
   2719.5 |
   2736.6 |#############
   2753.7 |
   2770.8 |
   2787.9 |
   2805.0 |
   2822.1 |
  (1 below, 3 above range)

nway (n=18, range 17545.0-17922.2 ns)
  17545.0 |
  17563.8 |
  17582.7 |
  17601.6 |
  17620.4 |########################################
  17639.3 |##########################
  17658.1 |##########################
  17677.0 |
  17695.9 |
  17714.7 |#############
  17733.6 |
  17752.5 |#############
  17771.3 |
  17790.2 |#############
  17809.1 |
  17827.9 |#############
  17846.8 |#############
  17865.6 |
  17884.5 |
  17903.4 |#############
  (2 below, 3 above range)

sequential (n=18, range 9798.3-10426.7 ns)
   9798.3 |####################
   9829.8 |
   9861.2 |
   9892.6 |
   9924.0 |
   9955.4 |
   9986.8 |
  10018.2 |########################################
  10049.7 |##########
  10081.1 |##########
  10112.5 |
  10143.9 |####################
  10175.3 |
  10206.8 |####################
  10238.2 |##########
  10269.6 |
  10301.0 |##########
  10332.4 |
  10363.8 |
  10395.2 |
  (2 below, 2 above range)

std_parallel (n=18, range 30477.2-36300.6 ns)
  30477.2 |##########################
  30768.4 |#############
  31059.6 |########################################
  31350.7 |##########################
  31641.9 |
  31933.1 |
  32224.2 |
  32515.4 |##########################
  32806.6 |#############
  33097.7 |
  33388.9 |#############
  33680.1 |#############
  33971.2 |#############
  34262.4 |
  34553.6 |
  34844.7 |
  35135.9 |
  35427.1 |
  35718.2 |
  36009.4 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=2331.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2262.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=2372.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3494.8% of algo (FFI overhead may distort results)
- **nway**: bridge=519.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=899.2% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=329.2% of algo (FFI overhead may distort results)

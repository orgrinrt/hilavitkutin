# fold_light, short fiber (1 to 4 morsels), W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 69% faster than the next best (head_tail_dyn)

kernel_seq (2.48 us) leads head_tail_dyn (4.20 us) by 69%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 44% (significant)

kernel_seq is -1.94 us (44%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 33.1x slower than the field

std_parallel (82.03 us) is 33.1x the fastest (2.48 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_parallel shows alternating (throttle bounce) (autocorr -0.53)

std_parallel's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 33.1x the fastest

Fastest kernel_seq (2.48 us) to slowest std_parallel (82.03 us): 33.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2480.8 ns median (-43.9% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 33.07x (fastest 2480.8 ns, slowest 82033.4 ns)

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
| head_tail | 5010ns | 5021ns | 4573ns | 4913ns | 5374ns | base |
| head_tail_dyn | 4863ns | 4796ns | 4615ns | 4771ns | 5124ns | -2.94% |
| kernel_nway | 16991ns | 15339ns | 14739ns | 15284ns | 20678ns | +239.13% |
| kernel_seq | 2805ns | 2797ns | 2687ns | 2761ns | 2931ns | -44.01% |
| nway | 49191ns | 49046ns | 46804ns | 48788ns | 50988ns | +881.81% |
| sequential | 10507ns | 10313ns | 10119ns | 10308ns | 11000ns | +109.71% |
| std_parallel | 81571ns | 82390ns | 73956ns | 80169ns | 87482ns | +1528.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4412ns | 4021ns | 4767ns | base | 0.004 |
| head_tail_dyn | 4257ns | 4011ns | 4520ns | -3.51% | 0.004 |
| kernel_nway | 16350ns | 14097ns | 20025ns | +270.63% | 0.001 |
| kernel_seq | 2481ns | 2441ns | 2510ns | -43.77% | 0.006 |
| nway | 48640ns | 46256ns | 50428ns | +1002.56% | 0.000 |
| sequential | 10353ns | 9979ns | 10827ns | +134.68% | 0.002 |
| std_parallel | 81180ns | 73533ns | 87102ns | +1740.15% | 0.000 |

## Performance model

- Peak throughput: **0.007 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.004 | 55.2% |
| head_tail_dyn | 0.004 | 58.1% |
| kernel_nway | 0.001 | 16.6% |
| kernel_seq | 0.006 | 98.4% |
| nway | 0.000 | 5.0% |
| sequential | 0.002 | 24.0% |
| std_parallel | 0.000 | 3.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 5010ns | 5010ns | base |
| head_tail_dyn | 4863ns | 4863ns | -2.94% |
| kernel_nway | 16991ns | 16991ns | +239.13% |
| kernel_seq | 2805ns | 2805ns | -44.01% |
| nway | 49191ns | 49191ns | +881.81% |
| sequential | 10507ns | 10507ns | +109.71% |
| std_parallel | 81571ns | 81571ns | +1528.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4421ns | base | --- | [4123, 4529] | --- | --- | --- | --- |
| head_tail_dyn | 4204ns | no significant difference | [-452, +79]ns | [4128, 4402] | no | 0.0963 | 0.0963 | 0 |
| kernel_nway | 14714ns | +10261.7ns (+232.1%) | [+10083, +11571]ns | [14408, 15655] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2481ns | -1941.7ns (-43.9%) | [-2011, -1693]ns | [2464, 2502] | YES | 0.0000 | 0.0000 | 0 |
| nway | 48520ns | +44453.3ns (+1005.5%) | [+43241, +45249]ns | [47768, 49681] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10174ns | +6025.0ns (+136.3%) | [+5675, +6180]ns | [10138, 10456] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 82033ns | +77379.1ns (+1750.3%) | [+72832, +80625]ns | [77012, 85312] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4505ns | -7.3% | +230.9% | -44.6% | +1002.7% | +133.6% | +1720.9% |
| 1 | 2 | 4570ns | -8.4% | +225.4% | -45.7% | +938.9% | +120.8% | +1627.4% |
| 1 | 3 | 4107ns | +7.2% | +248.4% | -39.9% | +1081.5% | +147.3% | +1959.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.246 | moderate- |
| head_tail_dyn | -0.143 | ok |
| kernel_nway | -0.134 | ok |
| kernel_seq | -0.002 | ok |
| nway | -0.483 | moderate- |
| sequential | -0.252 | moderate- |
| std_parallel | -0.531 | HIGH- (thermal bounce) |

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
| head_tail | 101291.1ns | 4411.6ns | 2296.0% | HIGH |
| head_tail_dyn | 96566.9ns | 4256.7ns | 2268.6% | HIGH |
| kernel_nway | 111183.5ns | 16350.5ns | 680.0% | HIGH |
| kernel_seq | 89730.7ns | 2480.5ns | 3617.4% | HIGH |
| nway | 108538.0ns | 48640.5ns | 223.1% | HIGH |
| sequential | 91152.0ns | 10353.1ns | 880.4% | HIGH |
| std_parallel | 134233.8ns | 81179.7ns | 165.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 4021.1-4766.9 ns)
   4021.1 |
   4058.4 |
   4095.7 |########################################
   4133.0 |
   4170.3 |
   4207.6 |##########
   4244.9 |
   4282.1 |
   4319.4 |##########
   4356.7 |
   4394.0 |##########
   4431.3 |##########
   4468.6 |####################
   4505.9 |##########
   4543.2 |##########
   4580.5 |
   4617.8 |
   4655.1 |##########
   4692.4 |
   4729.7 |##########
  (2 below, 2 above range)

head_tail_dyn (n=18, range 4010.6-4520.0 ns)
   4010.6 |##########################
   4036.0 |#############
   4061.5 |
   4087.0 |
   4112.4 |##########################
   4137.9 |#############
   4163.4 |#############
   4188.9 |########################################
   4214.3 |#############
   4239.8 |
   4265.3 |
   4290.7 |
   4316.2 |
   4341.7 |
   4367.2 |#############
   4392.6 |#############
   4418.1 |#############
   4443.6 |#############
   4469.0 |
   4494.5 |
  (1 below, 2 above range)

kernel_nway (n=18, range 14096.7-20024.7 ns)
  14096.7 |########################################
  14393.1 |########################################
  14689.5 |##########################
  14985.9 |##########################
  15282.3 |#############
  15578.7 |
  15875.1 |#############
  16171.5 |
  16467.9 |
  16764.3 |
  17060.7 |
  17357.1 |
  17653.5 |
  17949.9 |
  18246.3 |
  18542.7 |
  18839.1 |
  19135.5 |
  19431.9 |
  19728.3 |
  (2 below, 4 above range)

kernel_seq (n=18, range 2440.6-2510.0 ns)
   2440.6 |
   2444.0 |
   2447.5 |#############
   2451.0 |
   2454.5 |#############
   2457.9 |##########################
   2461.4 |
   2464.9 |
   2468.3 |#############
   2471.8 |
   2475.3 |########################################
   2478.8 |
   2482.2 |########################################
   2485.7 |
   2489.2 |
   2492.6 |
   2496.1 |#############
   2499.6 |
   2503.1 |##########################
   2506.5 |
  (1 below, 3 above range)

nway (n=18, range 46256.1-50428.3 ns)
  46256.1 |####################
  46464.7 |####################
  46673.3 |####################
  46881.9 |
  47090.5 |
  47299.2 |
  47507.8 |####################
  47716.4 |####################
  47925.0 |
  48133.6 |########################################
  48342.2 |####################
  48550.8 |####################
  48759.4 |
  48968.0 |####################
  49176.7 |####################
  49385.3 |####################
  49593.9 |
  49802.5 |####################
  50011.1 |
  50219.7 |########################################
  (1 below, 2 above range)

sequential (n=18, range 9979.4-10826.7 ns)
   9979.4 |#############
  10021.8 |#############
  10064.2 |
  10106.5 |########################################
  10148.9 |########################################
  10191.2 |#############
  10233.6 |
  10276.0 |#############
  10318.3 |
  10360.7 |
  10403.0 |
  10445.4 |#############
  10487.8 |
  10530.1 |
  10572.5 |##########################
  10614.9 |
  10657.2 |
  10699.6 |#############
  10741.9 |
  10784.3 |
  (2 below, 2 above range)

std_parallel (n=18, range 73532.8-87101.9 ns)
  73532.8 |####################
  74211.2 |####################
  74889.7 |####################
  75568.1 |
  76246.6 |####################
  76925.1 |####################
  77603.5 |####################
  78282.0 |####################
  78960.4 |
  79638.9 |
  80317.4 |####################
  80995.8 |
  81674.3 |
  82352.7 |
  83031.2 |####################
  83709.6 |########################################
  84388.1 |####################
  85066.6 |
  85745.0 |########################################
  86423.5 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=2283.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2296.1% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=757.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3619.5% of algo (FFI overhead may distort results)
- **nway**: bridge=222.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=897.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=161.4% of algo (FFI overhead may distort results)

# fold_light, short fiber (1 to 4 morsels), W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 60% faster than the next best (head_tail)

kernel_seq (2.58 us) leads head_tail (4.14 us) by 60%, a clear separation rather than a photo finish. CV 7.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 38% (significant)

kernel_seq is -1.57 us (38%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 12.0x slower than the field

std_parallel (30.87 us) is 12.0x the fastest (2.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.54)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 12.0x the fastest

Fastest kernel_seq (2.58 us) to slowest std_parallel (30.87 us): 12.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2580.8 ns median (-37.6% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 11.96x (fastest 2580.8 ns, slowest 30867.5 ns)

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
| head_tail | 4753ns | 4718ns | 4599ns | 4712ns | 4891ns | base |
| head_tail_dyn | 4886ns | 4838ns | 4668ns | 4810ns | 5109ns | +2.80% |
| kernel_nway | 4810ns | 4754ns | 4627ns | 4735ns | 5013ns | +1.19% |
| kernel_seq | 3088ns | 3011ns | 2858ns | 2970ns | 3378ns | -35.04% |
| nway | 18461ns | 18392ns | 18186ns | 18346ns | 18771ns | +288.39% |
| sequential | 10631ns | 10433ns | 9969ns | 10406ns | 11300ns | +123.66% |
| std_parallel | 32426ns | 31258ns | 30267ns | 31101ns | 35494ns | +582.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4162ns | 3990ns | 4297ns | base | 0.246 |
| head_tail_dyn | 4306ns | 4096ns | 4524ns | +3.46% | 0.238 |
| kernel_nway | 4190ns | 4017ns | 4359ns | +0.69% | 0.244 |
| kernel_seq | 2660ns | 2511ns | 2868ns | -36.08% | 0.385 |
| nway | 17953ns | 17672ns | 18237ns | +331.38% | 0.057 |
| sequential | 10492ns | 9835ns | 11161ns | +152.12% | 0.098 |
| std_parallel | 32048ns | 29900ns | 35111ns | +670.07% | 0.032 |

## Performance model

- Peak throughput: **0.408 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.248 | 60.7% |
| head_tail_dyn | 0.240 | 58.8% |
| kernel_nway | 0.247 | 60.5% |
| kernel_seq | 0.397 | 97.3% |
| nway | 0.057 | 14.0% |
| sequential | 0.100 | 24.4% |
| std_parallel | 0.033 | 8.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4753ns | 4753ns | base |
| head_tail_dyn | 4886ns | 4886ns | +2.80% |
| kernel_nway | 4810ns | 4810ns | +1.19% |
| kernel_seq | 3088ns | 3088ns | -35.04% |
| nway | 18461ns | 18461ns | +288.39% |
| sequential | 10631ns | 10631ns | +123.66% |
| std_parallel | 32426ns | 32426ns | +582.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4136ns | base | --- | [4090, 4210] | --- | --- | --- | --- |
| head_tail_dyn | 4271ns | no significant difference | [-29, +233]ns | [4188, 4311] | no | 0.2855 | 0.2379 | 0 |
| kernel_nway | 4153ns | no significant difference | [-115, +91]ns | [4105, 4194] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 2581ns | -1567.5ns (-37.9%) | [-1630, -1454]ns | [2537, 2658] | YES | 0.0000 | 0.0000 | 0 |
| nway | 17890ns | +13817.6ns (+334.1%) | [+13558, +13942]ns | [17759, 18003] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10285ns | +6256.6ns (+151.3%) | [+5972, +6376]ns | [10211, 10584] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 30868ns | +26739.2ns (+646.5%) | [+26207, +28485]ns | [30413, 32792] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4112ns | +3.2% | +1.4% | -36.7% | +336.0% | +149.8% | +652.7% |
| 1 | 2 | 4162ns | +3.3% | +0.5% | -37.3% | +329.8% | +142.2% | +637.3% |
| 1 | 3 | 4197ns | +0.1% | -1.1% | -39.2% | +326.2% | +152.2% | +664.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.008 | ok |
| head_tail_dyn | -0.542 | HIGH- (thermal bounce) |
| kernel_nway | -0.388 | moderate- |
| kernel_seq | -0.236 | moderate- |
| nway | -0.110 | ok |
| sequential | -0.409 | moderate- |
| std_parallel | -0.276 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 96084.7ns | 4161.8ns | 2308.8% | HIGH |
| head_tail_dyn | 96743.5ns | 4305.7ns | 2246.9% | HIGH |
| kernel_nway | 96612.2ns | 4190.4ns | 2305.6% | HIGH |
| kernel_seq | 92325.2ns | 2660.1ns | 3470.8% | HIGH |
| nway | 92372.1ns | 17952.9ns | 514.5% | HIGH |
| sequential | 91630.5ns | 10492.4ns | 873.3% | HIGH |
| std_parallel | 104701.5ns | 32048.3ns | 326.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 3990.0-4296.7 ns)
   3990.0 |####################
   4005.3 |
   4020.7 |
   4036.0 |####################
   4051.3 |
   4066.7 |####################
   4082.0 |########################################
   4097.3 |
   4112.7 |########################################
   4128.0 |########################################
   4143.3 |
   4158.7 |
   4174.0 |####################
   4189.3 |####################
   4204.7 |########################################
   4220.0 |
   4235.3 |
   4250.7 |
   4266.0 |####################
   4281.3 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 4096.1-4524.2 ns)
   4096.1 |####################
   4117.5 |
   4138.9 |##########
   4160.3 |##########
   4181.7 |##########
   4203.1 |####################
   4224.5 |
   4245.9 |
   4267.3 |########################################
   4288.7 |##########
   4310.1 |##########
   4331.5 |##########
   4352.9 |
   4374.4 |
   4395.8 |##########
   4417.2 |
   4438.6 |
   4460.0 |
   4481.4 |
   4502.8 |
  (1 below, 2 above range)

kernel_nway (n=18, range 4017.2-4359.5 ns)
   4017.2 |
   4034.3 |
   4051.5 |##########################
   4068.6 |
   4085.7 |#############
   4102.8 |#############
   4119.9 |
   4137.0 |########################################
   4154.1 |##########################
   4171.2 |##########################
   4188.4 |
   4205.5 |##########################
   4222.6 |
   4239.7 |
   4256.8 |##########################
   4273.9 |
   4291.0 |
   4308.1 |
   4325.2 |
   4342.4 |
  (2 below, 1 above range)

kernel_seq (n=18, range 2511.1-2868.0 ns)
   2511.1 |####################
   2528.9 |########################################
   2546.8 |##########
   2564.6 |####################
   2582.5 |
   2600.3 |
   2618.2 |##########
   2636.0 |####################
   2653.9 |
   2671.7 |####################
   2689.6 |
   2707.4 |
   2725.3 |
   2743.1 |
   2761.0 |
   2778.8 |
   2796.7 |
   2814.5 |
   2832.4 |##########
   2850.2 |
  (1 below, 2 above range)

nway (n=18, range 17671.7-18236.7 ns)
  17671.7 |#############
  17699.9 |
  17728.2 |
  17756.4 |########################################
  17784.7 |
  17812.9 |#############
  17841.2 |#############
  17869.4 |##########################
  17897.7 |
  17925.9 |
  17954.2 |#############
  17982.4 |########################################
  18010.7 |
  18038.9 |
  18067.2 |
  18095.4 |#############
  18123.7 |
  18151.9 |
  18180.2 |
  18208.4 |
  (2 below, 3 above range)

sequential (n=18, range 9835.0-11160.6 ns)
   9835.0 |#############
   9901.3 |
   9967.6 |
  10033.9 |#############
  10100.1 |
  10166.4 |########################################
  10232.7 |########################################
  10299.0 |
  10365.2 |#############
  10431.5 |#############
  10497.8 |
  10564.1 |##########################
  10630.3 |
  10696.6 |
  10762.9 |
  10829.2 |
  10895.4 |
  10961.7 |
  11028.0 |
  11094.3 |
  (2 below, 4 above range)

std_parallel (n=18, range 29900.0-35111.1 ns)
  29900.0 |#############
  30160.6 |##########################
  30421.1 |########################################
  30681.7 |##########################
  30942.2 |#############
  31202.8 |
  31463.3 |#############
  31723.9 |
  31984.4 |#############
  32245.0 |
  32505.6 |
  32766.1 |
  33026.7 |
  33287.2 |#############
  33547.8 |#############
  33808.3 |
  34068.9 |
  34329.4 |
  34590.0 |
  34850.6 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=2316.0% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2262.1% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=2313.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3552.5% of algo (FFI overhead may distort results)
- **nway**: bridge=517.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=895.2% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=332.8% of algo (FFI overhead may distort results)

# fold_light, short fiber (1 to 4 morsels), W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 60% faster than the next best (head_tail_dyn)

kernel_seq (2.58 us) leads head_tail_dyn (4.13 us) by 60%, a clear separation rather than a photo finish. CV 9.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 37% (significant)

kernel_seq is -1.54 us (37%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 12.2x slower than the field

std_parallel (31.46 us) is 12.2x the fastest (2.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq is fastest but the noisiest (CV 9.0%)

kernel_seq wins on median (2.58 us) yet has the highest variance (CV 9.0%), while head_tail_dyn is the steadiest (CV 2.5%, 4.13 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.63)

kernel_seq's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 12.2x the fastest

Fastest kernel_seq (2.58 us) to slowest std_parallel (31.46 us): 12.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2580.8 ns median (-37.7% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 12.19x (fastest 2580.8 ns, slowest 31460.0 ns)

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
| head_tail | 4792ns | 4767ns | 4613ns | 4741ns | 4959ns | base |
| head_tail_dyn | 4741ns | 4762ns | 4579ns | 4707ns | 4873ns | -1.07% |
| kernel_nway | 4754ns | 4750ns | 4579ns | 4711ns | 4905ns | -0.81% |
| kernel_seq | 3134ns | 3047ns | 2762ns | 2988ns | 3541ns | -34.60% |
| nway | 18596ns | 18621ns | 18022ns | 18462ns | 19084ns | +288.03% |
| sequential | 10634ns | 10442ns | 9970ns | 10347ns | 11394ns | +121.89% |
| std_parallel | 32461ns | 31841ns | 30145ns | 31565ns | 34962ns | +577.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4172ns | 4019ns | 4318ns | base | 0.061 |
| head_tail_dyn | 4131ns | 3993ns | 4245ns | -0.97% | 0.062 |
| kernel_nway | 4150ns | 3978ns | 4295ns | -0.52% | 0.062 |
| kernel_seq | 2684ns | 2478ns | 2965ns | -35.66% | 0.095 |
| nway | 18089ns | 17518ns | 18582ns | +333.59% | 0.014 |
| sequential | 10491ns | 9834ns | 11255ns | +151.46% | 0.024 |
| std_parallel | 32087ns | 29779ns | 34578ns | +669.12% | 0.008 |

## Performance model

- Peak throughput: **0.103 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.062 | 59.8% |
| head_tail_dyn | 0.062 | 60.0% |
| kernel_nway | 0.062 | 59.8% |
| kernel_seq | 0.099 | 96.0% |
| nway | 0.014 | 13.7% |
| sequential | 0.025 | 24.1% |
| std_parallel | 0.008 | 7.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4792ns | 4792ns | base |
| head_tail_dyn | 4741ns | 4741ns | -1.07% |
| kernel_nway | 4754ns | 4754ns | -0.81% |
| kernel_seq | 3134ns | 3134ns | -34.60% |
| nway | 18596ns | 18596ns | +288.03% |
| sequential | 10634ns | 10634ns | +121.89% |
| std_parallel | 32461ns | 32461ns | +577.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4143ns | base | --- | [4104, 4191] | --- | --- | --- | --- |
| head_tail_dyn | 4128ns | no significant difference | [-102, +78]ns | [4063, 4202] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 4144ns | no significant difference | [-83, +82]ns | [4045, 4209] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 2581ns | -1540.8ns (-37.2%) | [-1597, -1431]ns | [2529, 2748] | YES | 0.0000 | 0.0000 | 0 |
| nway | 18099ns | +13980.0ns (+337.4%) | [+13632, +14135]ns | [17723, 18265] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10299ns | +6120.0ns (+147.7%) | [+5863, +6741]ns | [10099, 10861] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 31460ns | +27313.3ns (+659.2%) | [+26335, +28619]ns | [30586, 32813] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4142ns | +0.7% | +0.1% | -36.6% | +337.6% | +174.3% | +670.1% |
| 1 | 2 | 4143ns | +0.1% | +0.3% | -38.9% | +341.0% | +138.5% | +655.2% |
| 1 | 3 | 4116ns | -1.3% | +0.7% | -36.8% | +331.8% | +148.5% | +669.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.198 | ok |
| head_tail_dyn | -0.066 | ok |
| kernel_nway | -0.628 | HIGH- (thermal bounce) |
| kernel_seq | -0.628 | HIGH- (thermal bounce) |
| nway | -0.354 | moderate- |
| sequential | -0.309 | moderate- |
| std_parallel | -0.565 | HIGH- (thermal bounce) |

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
| head_tail | 97789.0ns | 4171.9ns | 2344.0% | HIGH |
| head_tail_dyn | 97859.2ns | 4131.4ns | 2368.7% | HIGH |
| kernel_nway | 97113.9ns | 4150.3ns | 2339.9% | HIGH |
| kernel_seq | 93590.7ns | 2684.1ns | 3486.9% | HIGH |
| nway | 92885.1ns | 18089.3ns | 513.5% | HIGH |
| sequential | 91205.0ns | 10490.9ns | 869.4% | HIGH |
| std_parallel | 103721.0ns | 32087.1ns | 323.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 4018.9-4317.5 ns)
   4018.9 |
   4033.8 |#############
   4048.7 |##########################
   4063.7 |
   4078.6 |
   4093.5 |#############
   4108.5 |########################################
   4123.4 |
   4138.3 |##########################
   4153.3 |
   4168.2 |##########################
   4183.1 |##########################
   4198.1 |#############
   4213.0 |
   4227.9 |
   4242.9 |
   4257.8 |
   4272.7 |
   4287.7 |#############
   4302.6 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 3992.8-4245.0 ns)
   3992.8 |##########
   4005.4 |##########
   4018.0 |
   4030.6 |
   4043.2 |####################
   4055.8 |##########
   4068.4 |##########
   4081.1 |
   4093.7 |
   4106.3 |##########
   4118.9 |##########
   4131.5 |####################
   4144.1 |
   4156.7 |##########
   4169.3 |
   4182.0 |
   4194.6 |########################################
   4207.2 |
   4219.8 |
   4232.4 |
  (1 below, 2 above range)

kernel_nway (n=18, range 3978.3-4295.3 ns)
   3978.3 |
   3994.2 |##########################
   4010.0 |
   4025.9 |#############
   4041.7 |#############
   4057.6 |
   4073.4 |
   4089.3 |
   4105.1 |#############
   4121.0 |
   4136.8 |########################################
   4152.7 |#############
   4168.5 |
   4184.4 |#############
   4200.2 |##########################
   4216.1 |
   4231.9 |##########################
   4247.8 |
   4263.6 |
   4279.5 |
  (2 below, 2 above range)

kernel_seq (n=18, range 2478.3-2964.7 ns)
   2478.3 |####################
   2502.7 |
   2527.0 |########################################
   2551.3 |##########
   2575.6 |
   2599.9 |##########
   2624.3 |####################
   2648.6 |##########
   2672.9 |
   2697.2 |
   2721.5 |
   2745.9 |
   2770.2 |
   2794.5 |
   2818.8 |##########
   2843.1 |
   2867.5 |##########
   2891.8 |
   2916.1 |
   2940.4 |
  (2 below, 3 above range)

nway (n=18, range 17517.8-18582.2 ns)
  17517.8 |########################################
  17571.0 |
  17624.2 |####################
  17677.4 |####################
  17730.7 |####################
  17783.9 |
  17837.1 |####################
  17890.3 |
  17943.5 |####################
  17996.8 |
  18050.0 |####################
  18103.2 |####################
  18156.4 |####################
  18209.7 |########################################
  18262.9 |########################################
  18316.1 |####################
  18369.3 |####################
  18422.5 |
  18475.8 |
  18529.0 |
  (1 below, 1 above range)

sequential (n=18, range 9833.9-11255.0 ns)
   9833.9 |##########################
   9905.0 |
   9976.0 |#############
  10047.1 |
  10118.1 |##########################
  10189.2 |##########################
  10260.2 |
  10331.3 |########################################
  10402.3 |#############
  10473.4 |
  10544.5 |
  10615.5 |
  10686.6 |
  10757.6 |
  10828.7 |
  10899.7 |
  10970.8 |
  11041.8 |
  11112.9 |
  11183.9 |
  (2 below, 5 above range)

std_parallel (n=18, range 29778.9-34578.1 ns)
  29778.9 |########################################
  30018.8 |####################
  30258.8 |####################
  30498.7 |
  30738.7 |########################################
  30978.7 |########################################
  31218.6 |
  31458.6 |
  31698.5 |####################
  31938.5 |####################
  32178.5 |
  32418.4 |####################
  32658.4 |########################################
  32898.3 |
  33138.3 |####################
  33378.3 |
  33618.2 |
  33858.2 |
  34098.1 |####################
  34338.1 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=2359.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2376.4% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=2337.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3615.8% of algo (FFI overhead may distort results)
- **nway**: bridge=512.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=887.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=327.3% of algo (FFI overhead may distort results)

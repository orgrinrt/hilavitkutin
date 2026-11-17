# fold_light, short fiber (1 to 4 morsels), W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 65% faster than the next best (head_tail_dyn)

kernel_seq (2.51 us) leads head_tail_dyn (4.13 us) by 65%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 38% (significant)

kernel_seq is -1.59 us (38%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 21.9x slower than the field

std_parallel (54.82 us) is 21.9x the fastest (2.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.67)

head_tail's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 21.9x the fastest

Fastest kernel_seq (2.51 us) to slowest std_parallel (54.82 us): 21.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2506.7 ns median (-39.4% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 21.87x (fastest 2506.7 ns, slowest 54818.3 ns)

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
| head_tail | 4959ns | 4755ns | 4620ns | 4753ns | 5437ns | base |
| head_tail_dyn | 4745ns | 4723ns | 4606ns | 4712ns | 4864ns | -4.30% |
| kernel_nway | 10165ns | 9802ns | 9269ns | 9772ns | 11203ns | +105.00% |
| kernel_seq | 2824ns | 2782ns | 2684ns | 2769ns | 2976ns | -43.05% |
| nway | 33103ns | 33363ns | 31025ns | 32794ns | 34605ns | +567.58% |
| sequential | 10282ns | 10298ns | 9741ns | 10188ns | 10694ns | +107.35% |
| std_parallel | 55176ns | 55208ns | 51572ns | 54724ns | 57657ns | +1012.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4322ns | 4017ns | 4744ns | base | 0.059 |
| head_tail_dyn | 4141ns | 4014ns | 4243ns | -4.20% | 0.062 |
| kernel_nway | 9545ns | 8664ns | 10572ns | +120.83% | 0.027 |
| kernel_seq | 2530ns | 2462ns | 2607ns | -41.47% | 0.101 |
| nway | 32559ns | 30490ns | 34063ns | +653.29% | 0.008 |
| sequential | 10141ns | 9611ns | 10540ns | +134.63% | 0.025 |
| std_parallel | 54780ns | 51186ns | 57259ns | +1167.41% | 0.005 |

## Performance model

- Peak throughput: **0.104 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.062 | 59.5% |
| head_tail_dyn | 0.062 | 59.6% |
| kernel_nway | 0.028 | 26.8% |
| kernel_seq | 0.102 | 98.2% |
| nway | 0.008 | 7.5% |
| sequential | 0.025 | 24.2% |
| std_parallel | 0.005 | 4.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4959ns | 4959ns | base |
| head_tail_dyn | 4745ns | 4745ns | -4.30% |
| kernel_nway | 10165ns | 10165ns | +105.00% |
| kernel_seq | 2824ns | 2824ns | -43.05% |
| nway | 33103ns | 33103ns | +567.58% |
| sequential | 10282ns | 10282ns | +107.35% |
| std_parallel | 55176ns | 55176ns | +1012.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4138ns | base | --- | [4072, 4434] | --- | --- | --- | --- |
| head_tail_dyn | 4131ns | no significant difference | [-228, +47]ns | [4081, 4194] | no | 0.4807 | 0.4807 | 0 |
| kernel_nway | 9171ns | +4964.1ns (+120.0%) | [+4725, +5395]ns | [9061, 9572] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2507ns | -1591.7ns (-38.5%) | [-1942, -1537]ns | [2488, 2536] | YES | 0.0000 | 0.0000 | 0 |
| nway | 32803ns | +28430.0ns (+687.0%) | [+27287, +29004]ns | [31609, 33461] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10158ns | +5679.1ns (+137.2%) | [+5538, +6040]ns | [9948, 10190] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 54818ns | +50647.5ns (+1223.9%) | [+49123, +51947]ns | [53335, 56305] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4097ns | +0.8% | +123.3% | -38.3% | +688.3% | +134.6% | +1219.9% |
| 1 | 2 | 4555ns | -8.1% | +100.0% | -45.3% | +624.8% | +123.0% | +1117.5% |
| 1 | 3 | 4098ns | +0.1% | +129.6% | -38.1% | +695.1% | +148.7% | +1255.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.667 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.607 | HIGH- (thermal bounce) |
| kernel_nway | -0.240 | moderate- |
| kernel_seq | -0.636 | HIGH- (thermal bounce) |
| nway | -0.561 | HIGH- (thermal bounce) |
| sequential | -0.137 | ok |
| std_parallel | -0.139 | ok |

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
| head_tail | 99964.1ns | 4322.2ns | 2312.8% | HIGH |
| head_tail_dyn | 97329.7ns | 4140.7ns | 2350.5% | HIGH |
| kernel_nway | 118935.7ns | 9544.7ns | 1246.1% | HIGH |
| kernel_seq | 90901.5ns | 2529.8ns | 3593.2% | HIGH |
| nway | 113190.8ns | 32558.8ns | 347.7% | HIGH |
| sequential | 90188.1ns | 10141.1ns | 889.3% | HIGH |
| std_parallel | 117566.8ns | 54780.4ns | 214.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 4017.2-4743.9 ns)
   4017.2 |########################################
   4053.6 |##########################
   4089.9 |##########################
   4126.2 |##########################
   4162.6 |
   4198.9 |
   4235.2 |
   4271.6 |#############
   4307.9 |#############
   4344.2 |#############
   4380.6 |
   4416.9 |
   4453.2 |
   4489.6 |#############
   4525.9 |
   4562.2 |#############
   4598.6 |
   4634.9 |
   4671.2 |
   4707.6 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 4013.9-4242.5 ns)
   4013.9 |
   4025.3 |
   4036.7 |#############
   4048.2 |########################################
   4059.6 |
   4071.0 |
   4082.5 |#############
   4093.9 |
   4105.3 |########################################
   4116.8 |
   4128.2 |
   4139.6 |#############
   4151.1 |
   4162.5 |#############
   4173.9 |#############
   4185.4 |##########################
   4196.8 |#############
   4208.2 |
   4219.7 |
   4231.1 |#############
  (1 below, 2 above range)

kernel_nway (n=18, range 8664.4-10571.7 ns)
   8664.4 |
   8759.8 |
   8855.2 |##########################
   8950.5 |
   9045.9 |########################################
   9141.2 |########################################
   9236.6 |#############
   9332.0 |
   9427.3 |#############
   9522.7 |##########################
   9618.0 |
   9713.4 |
   9808.8 |
   9904.1 |
   9999.5 |#############
  10094.9 |
  10190.2 |
  10285.6 |
  10380.9 |
  10476.3 |
  (2 below, 3 above range)

kernel_seq (n=18, range 2461.7-2607.2 ns)
   2461.7 |
   2468.9 |
   2476.2 |#############
   2483.5 |########################################
   2490.8 |#############
   2498.1 |##########################
   2505.3 |#############
   2512.6 |#############
   2519.9 |#############
   2527.2 |#############
   2534.4 |#############
   2541.7 |
   2549.0 |
   2556.3 |
   2563.6 |##########################
   2570.8 |
   2578.1 |
   2585.4 |
   2592.7 |
   2599.9 |
  (2 below, 2 above range)

nway (n=18, range 30490.0-34063.1 ns)
  30490.0 |#############
  30668.6 |#############
  30847.3 |
  31025.9 |#############
  31204.6 |
  31383.2 |#############
  31561.9 |
  31740.6 |##########################
  31919.2 |
  32097.9 |#############
  32276.5 |
  32455.2 |
  32633.8 |#############
  32812.5 |#############
  32991.1 |##########################
  33169.8 |#############
  33348.4 |
  33527.1 |
  33705.8 |########################################
  33884.4 |
  (1 below, 2 above range)

sequential (n=18, range 9611.1-10540.3 ns)
   9611.1 |########################
   9657.6 |
   9704.0 |
   9750.5 |
   9797.0 |########
   9843.4 |
   9889.9 |
   9936.3 |
   9982.8 |
  10029.2 |
  10075.7 |########
  10122.2 |################################
  10168.6 |########################################
  10215.1 |
  10261.5 |
  10308.0 |
  10354.4 |
  10400.9 |########
  10447.4 |
  10493.8 |
  (1 below, 2 above range)

std_parallel (n=18, range 51186.1-57258.9 ns)
  51186.1 |####################
  51489.7 |
  51793.4 |####################
  52097.0 |
  52400.7 |####################
  52704.3 |
  53007.9 |####################
  53311.6 |####################
  53615.2 |
  53918.9 |####################
  54222.5 |
  54526.1 |########################################
  54829.8 |####################
  55133.4 |####################
  55437.0 |
  55740.7 |####################
  56044.3 |########################################
  56348.0 |########################################
  56651.6 |
  56955.2 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=2404.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2359.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1297.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3574.0% of algo (FFI overhead may distort results)
- **nway**: bridge=344.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=901.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=214.0% of algo (FFI overhead may distort results)

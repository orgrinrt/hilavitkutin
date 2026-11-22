# fold_light, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 41% faster than the next best (kernel_nway)

kernel_seq (3.10 us) leads kernel_nway (4.37 us) by 41%, a clear separation rather than a photo finish. CV 9.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 30% (significant)

kernel_seq is -1.33 us (30%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 10.3x slower than the field

std_parallel (31.90 us) is 10.3x the fastest (3.10 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.58)

kernel_seq's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 10.3x the fastest

Fastest kernel_seq (3.10 us) to slowest std_parallel (31.90 us): 10.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 3102.1 ns median (-31.1% vs baseline)
- 2 variants significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 10.28x (fastest 3102.1 ns, slowest 31895.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 2 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 5144ns | 5106ns | 4771ns | 5070ns | 5442ns | base |
| head_tail_dyn | 5465ns | 5440ns | 4961ns | 5339ns | 5906ns | +6.24% |
| kernel_nway | 5012ns | 4998ns | 4788ns | 4968ns | 5192ns | -2.56% |
| kernel_seq | 3788ns | 3694ns | 3375ns | 3623ns | 4242ns | -26.36% |
| nway | 19097ns | 18838ns | 18567ns | 18764ns | 19862ns | +271.24% |
| sequential | 10917ns | 10888ns | 10240ns | 10741ns | 11521ns | +112.23% |
| std_parallel | 33757ns | 32340ns | 29915ns | 32054ns | 38232ns | +556.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4543ns | 4226ns | 4849ns | base | 0.902 |
| head_tail_dyn | 4860ns | 4394ns | 5292ns | +6.97% | 0.843 |
| kernel_nway | 4391ns | 4181ns | 4570ns | -3.34% | 0.933 |
| kernel_seq | 3141ns | 2800ns | 3490ns | -30.87% | 1.304 |
| nway | 18564ns | 18004ns | 19317ns | +308.63% | 0.221 |
| sequential | 10763ns | 10088ns | 11357ns | +136.92% | 0.381 |
| std_parallel | 33339ns | 29528ns | 37811ns | +633.85% | 0.123 |

## Performance model

- Peak throughput: **1.463 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.910 | 62.2% |
| head_tail_dyn | 0.851 | 58.2% |
| kernel_nway | 0.937 | 64.1% |
| kernel_seq | 1.320 | 90.3% |
| nway | 0.224 | 15.3% |
| sequential | 0.381 | 26.0% |
| std_parallel | 0.128 | 8.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 5144ns | 5144ns | base |
| head_tail_dyn | 5465ns | 5465ns | +6.24% |
| kernel_nway | 5012ns | 5012ns | -2.56% |
| kernel_seq | 3788ns | 3788ns | -26.36% |
| nway | 19097ns | 19097ns | +271.24% |
| sequential | 10917ns | 10917ns | +112.23% |
| std_parallel | 33757ns | 33757ns | +556.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4502ns | base | --- | [4375, 4731] | --- | --- | --- | --- |
| head_tail_dyn | 4810ns | +175.0ns (+3.9%) | [+96, +562]ns | [4567, 5090] | YES | 0.0309 | 0.0309 | 0 |
| kernel_nway | 4371ns | -160.4ns (-3.6%) | [-298, -48]ns | [4319, 4425] | YES | 0.0090 | 0.0075 | 0 |
| kernel_seq | 3102ns | -1329.2ns (-29.5%) | [-1694, -1154]ns | [2908, 3338] | YES | 0.0000 | 0.0000 | 0 |
| nway | 18277ns | +13829.2ns (+307.2%) | [+13444, +14150]ns | [18177, 18569] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10752ns | +5954.1ns (+132.3%) | [+5775, +6500]ns | [10410, 11017] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 31896ns | +27272.8ns (+605.8%) | [+26840, +28179]ns | [31290, 32567] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4265ns | +5.8% | +0.6% | -23.6% | +336.5% | +151.9% | +640.5% |
| 1 | 2 | 4829ns | -0.4% | -9.6% | -40.6% | +277.0% | +122.6% | +560.5% |
| 1 | 3 | 4481ns | +15.6% | -1.9% | -30.4% | +308.9% | +137.0% | +626.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.570 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.002 | ok |
| kernel_nway | -0.036 | ok |
| kernel_seq | -0.584 | HIGH- (thermal bounce) |
| nway | -0.352 | moderate- |
| sequential | -0.208 | moderate- |
| std_parallel | -0.026 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 91653.5ns | 4543.1ns | 2017.4% | HIGH |
| head_tail_dyn | 94498.8ns | 4859.7ns | 1944.5% | HIGH |
| kernel_nway | 92906.0ns | 4391.2ns | 2115.7% | HIGH |
| kernel_seq | 93420.1ns | 3140.7ns | 2974.5% | HIGH |
| nway | 97276.9ns | 18564.1ns | 524.0% | HIGH |
| sequential | 92247.0ns | 10763.4ns | 857.0% | HIGH |
| std_parallel | 103965.0ns | 33339.3ns | 311.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 4226.4-4849.3 ns)
   4226.4 |#############
   4257.5 |##########################
   4288.7 |
   4319.8 |
   4351.0 |#############
   4382.1 |#############
   4413.3 |#############
   4444.4 |#############
   4475.6 |#############
   4506.7 |########################################
   4537.9 |
   4569.0 |
   4600.2 |
   4631.3 |
   4662.4 |
   4693.6 |
   4724.7 |##########################
   4755.9 |#############
   4787.0 |
   4818.2 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 4394.4-5292.4 ns)
   4394.4 |########################################
   4439.3 |####################
   4484.2 |
   4529.1 |####################
   4574.0 |####################
   4618.9 |####################
   4663.8 |
   4708.7 |########################################
   4753.6 |
   4798.5 |
   4843.4 |####################
   4888.3 |####################
   4933.2 |
   4978.1 |
   5023.0 |########################################
   5067.9 |
   5112.8 |####################
   5157.7 |
   5202.6 |####################
   5247.5 |
  (1 below, 3 above range)

kernel_nway (n=18, range 4180.6-4570.1 ns)
   4180.6 |
   4200.0 |##########
   4219.5 |##########
   4239.0 |##########
   4258.5 |
   4278.0 |
   4297.4 |##########
   4316.9 |##########
   4336.4 |##########
   4355.9 |########################################
   4375.4 |##########
   4394.8 |##########
   4414.3 |
   4433.8 |##########
   4453.3 |
   4472.7 |
   4492.2 |##########
   4511.7 |
   4531.2 |
   4550.7 |
  (1 below, 3 above range)

kernel_seq (n=18, range 2800.0-3490.3 ns)
   2800.0 |
   2834.5 |#############
   2869.0 |##########################
   2903.5 |
   2938.0 |########################################
   2972.5 |
   3007.1 |
   3041.6 |#############
   3076.1 |
   3110.6 |#############
   3145.1 |#############
   3179.6 |#############
   3214.2 |
   3248.7 |
   3283.2 |#############
   3317.7 |
   3352.2 |#############
   3386.7 |
   3421.3 |##########################
   3455.8 |
  (2 below, 2 above range)

nway (n=18, range 18004.2-19316.7 ns)
  18004.2 |
  18069.8 |##########################
  18135.4 |##########################
  18201.0 |########################################
  18266.7 |#############
  18332.3 |#############
  18397.9 |#############
  18463.5 |#############
  18529.2 |
  18594.8 |#############
  18660.4 |#############
  18726.0 |
  18791.7 |
  18857.3 |
  18922.9 |#############
  18988.5 |
  19054.2 |
  19119.8 |
  19185.4 |
  19251.0 |
  (2 below, 2 above range)

sequential (n=18, range 10087.5-11356.9 ns)
  10087.5 |########################################
  10151.0 |####################
  10214.5 |
  10277.9 |
  10341.4 |####################
  10404.9 |####################
  10468.4 |####################
  10531.8 |
  10595.3 |####################
  10658.8 |####################
  10722.2 |
  10785.7 |########################################
  10849.2 |####################
  10912.6 |
  10976.1 |########################################
  11039.6 |####################
  11103.1 |
  11166.5 |
  11230.0 |
  11293.5 |
  (1 below, 3 above range)

std_parallel (n=18, range 29527.8-37811.1 ns)
  29527.8 |##########
  29941.9 |
  30356.1 |####################
  30770.3 |
  31184.4 |##############################
  31598.6 |##############################
  32012.8 |##########
  32426.9 |########################################
  32841.1 |##########
  33255.3 |
  33669.4 |
  34083.6 |
  34497.8 |
  34911.9 |
  35326.1 |
  35740.3 |
  36154.4 |
  36568.6 |
  36982.8 |
  37396.9 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=2034.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=1971.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=2136.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=2991.5% of algo (FFI overhead may distort results)
- **nway**: bridge=534.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=854.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=321.1% of algo (FFI overhead may distort results)

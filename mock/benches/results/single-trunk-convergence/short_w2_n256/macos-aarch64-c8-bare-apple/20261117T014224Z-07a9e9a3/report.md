# fold_light, short fiber (1 to 4 morsels), W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 54% faster than the next best (head_tail_dyn)

kernel_seq (2.65 us) leads head_tail_dyn (4.08 us) by 54%, a clear separation rather than a photo finish. CV 9.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 37% (significant)

kernel_seq is -1.65 us (37%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 11.7x slower than the field

std_parallel (30.93 us) is 11.7x the fastest (2.65 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.67)

nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 11.7x the fastest

Fastest kernel_seq (2.65 us) to slowest std_parallel (30.93 us): 11.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2645.0 ns median (-40.6% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 11.69x (fastest 2645.0 ns, slowest 30931.7 ns)

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
| head_tail | 5056ns | 5118ns | 4557ns | 4938ns | 5481ns | base |
| head_tail_dyn | 4671ns | 4675ns | 4594ns | 4664ns | 4721ns | -7.60% |
| kernel_nway | 4944ns | 4785ns | 4653ns | 4763ns | 5362ns | -2.21% |
| kernel_seq | 3130ns | 3095ns | 2744ns | 3031ns | 3470ns | -38.10% |
| nway | 18079ns | 18130ns | 17354ns | 18054ns | 18479ns | +257.59% |
| sequential | 10476ns | 10128ns | 9873ns | 10135ns | 11289ns | +107.21% |
| std_parallel | 32261ns | 31300ns | 30118ns | 31270ns | 34818ns | +538.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4428ns | 3972ns | 4807ns | base | 0.058 |
| head_tail_dyn | 4082ns | 4019ns | 4138ns | -7.81% | 0.063 |
| kernel_nway | 4320ns | 4016ns | 4719ns | -2.45% | 0.059 |
| kernel_seq | 2696ns | 2484ns | 2948ns | -39.11% | 0.095 |
| nway | 17536ns | 16679ns | 17981ns | +296.00% | 0.015 |
| sequential | 10335ns | 9733ns | 11142ns | +133.40% | 0.025 |
| std_parallel | 31888ns | 29719ns | 34445ns | +620.11% | 0.008 |

## Performance model

- Peak throughput: **0.103 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.058 | 55.8% |
| head_tail_dyn | 0.063 | 60.9% |
| kernel_nway | 0.061 | 59.4% |
| kernel_seq | 0.097 | 93.9% |
| nway | 0.015 | 14.1% |
| sequential | 0.026 | 24.9% |
| std_parallel | 0.008 | 8.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 5056ns | 5056ns | base |
| head_tail_dyn | 4671ns | 4671ns | -7.60% |
| kernel_nway | 4944ns | 4944ns | -2.21% |
| kernel_seq | 3130ns | 3130ns | -38.10% |
| nway | 18079ns | 18079ns | +257.59% |
| sequential | 10476ns | 10476ns | +107.21% |
| std_parallel | 32261ns | 32261ns | +538.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4452ns | base | --- | [4097, 4716] | --- | --- | --- | --- |
| head_tail_dyn | 4082ns | -370.0ns (-8.3%) | [-621, -42]ns | [4052, 4100] | YES | 0.0371 | 0.0309 | 0 |
| kernel_nway | 4181ns | no significant difference | [-568, +300]ns | [4092, 4318] | no | 0.4807 | 0.4807 | 0 |
| kernel_seq | 2645ns | -1653.3ns (-37.1%) | [-2038, -1517]ns | [2540, 2702] | YES | 0.0000 | 0.0000 | 0 |
| nway | 17628ns | +13330.0ns (+299.4%) | [+12707, +13555]ns | [17512, 17668] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 9992ns | +5656.6ns (+127.1%) | [+5189, +6411]ns | [9960, 10626] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 30932ns | +26714.2ns (+600.1%) | [+26118, +27881]ns | [30767, 32237] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4676ns | -13.3% | -11.5% | -41.7% | +277.1% | +112.4% | +614.9% |
| 1 | 2 | 4688ns | -12.0% | -11.9% | -45.0% | +260.9% | +112.8% | +553.9% |
| 1 | 3 | 4052ns | +0.5% | +8.5% | -36.5% | +335.4% | +179.1% | +665.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.176 | ok |
| head_tail_dyn | -0.612 | HIGH- (thermal bounce) |
| kernel_nway | -0.181 | ok |
| kernel_seq | -0.159 | ok |
| nway | -0.666 | HIGH- (thermal bounce) |
| sequential | -0.151 | ok |
| std_parallel | -0.239 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 100440.0ns | 4428.2ns | 2268.2% | HIGH |
| head_tail_dyn | 98532.8ns | 4082.5ns | 2413.5% | HIGH |
| kernel_nway | 97130.9ns | 4319.5ns | 2248.6% | HIGH |
| kernel_seq | 93201.4ns | 2696.5ns | 3456.4% | HIGH |
| nway | 92763.9ns | 17535.8ns | 529.0% | HIGH |
| sequential | 89960.9ns | 10335.4ns | 870.4% | HIGH |
| std_parallel | 103476.3ns | 31888.0ns | 324.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 3972.2-4807.2 ns)
   3972.2 |
   4014.0 |####################
   4055.7 |########################################
   4097.5 |####################
   4139.2 |
   4181.0 |####################
   4222.7 |
   4264.5 |
   4306.2 |########################################
   4348.0 |
   4389.7 |
   4431.5 |
   4473.2 |
   4515.0 |
   4556.7 |####################
   4598.5 |####################
   4640.2 |########################################
   4682.0 |
   4723.7 |####################
   4765.5 |
  (2 below, 4 above range)

head_tail_dyn (n=18, range 4019.5-4138.3 ns)
   4019.5 |##########
   4025.4 |##########
   4031.4 |
   4037.3 |
   4043.2 |##########
   4049.2 |
   4055.1 |####################
   4061.1 |
   4067.0 |##########
   4073.0 |##########
   4078.9 |
   4084.8 |########################################
   4090.8 |
   4096.7 |
   4102.7 |
   4108.6 |##########
   4114.6 |
   4120.5 |
   4126.4 |
   4132.4 |####################
  (2 below, 2 above range)

kernel_nway (n=18, range 4016.1-4718.6 ns)
   4016.1 |##########
   4051.2 |##########
   4086.4 |####################
   4121.5 |
   4156.6 |########################################
   4191.7 |##########
   4226.9 |####################
   4262.0 |
   4297.1 |
   4332.2 |
   4367.4 |####################
   4402.5 |
   4437.6 |
   4472.7 |
   4507.9 |##########
   4543.0 |
   4578.1 |
   4613.2 |
   4648.4 |
   4683.5 |
  (2 below, 2 above range)

kernel_seq (n=18, range 2484.4-2948.1 ns)
   2484.4 |########################################
   2507.6 |########################################
   2530.8 |
   2554.0 |########################################
   2577.2 |####################
   2600.3 |
   2623.5 |####################
   2646.7 |####################
   2669.9 |########################################
   2693.1 |########################################
   2716.2 |
   2739.4 |
   2762.6 |####################
   2785.8 |
   2809.0 |
   2832.2 |
   2855.3 |
   2878.5 |
   2901.7 |
   2924.9 |####################
  (1 below, 2 above range)

nway (n=18, range 16678.9-17980.5 ns)
  16678.9 |
  16744.0 |
  16809.1 |########
  16874.1 |
  16939.2 |########
  17004.3 |
  17069.4 |
  17134.5 |
  17199.6 |
  17264.6 |
  17329.7 |
  17394.8 |
  17459.9 |########
  17525.0 |################
  17590.0 |########################################
  17655.1 |########################
  17720.2 |
  17785.3 |
  17850.4 |
  17915.5 |
  (2 below, 3 above range)

sequential (n=18, range 9733.3-11142.5 ns)
   9733.3 |
   9803.8 |
   9874.2 |#####
   9944.7 |########################################
  10015.1 |#####
  10085.6 |#####
  10156.1 |
  10226.5 |#####
  10297.0 |
  10367.4 |
  10437.9 |
  10508.4 |
  10578.8 |
  10649.3 |
  10719.7 |
  10790.2 |
  10860.7 |
  10931.1 |
  11001.6 |#####
  11072.0 |
  (2 below, 4 above range)

std_parallel (n=18, range 29719.4-34445.3 ns)
  29719.4 |
  29955.7 |##########
  30192.0 |
  30428.3 |####################
  30664.6 |########################################
  30900.9 |####################
  31137.2 |####################
  31373.5 |
  31609.8 |
  31846.1 |##########
  32082.3 |
  32318.6 |##########
  32554.9 |##########
  32791.2 |
  33027.5 |
  33263.8 |
  33500.1 |
  33736.4 |
  33972.7 |
  34209.0 |##########
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=2254.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2401.4% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=2344.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3507.7% of algo (FFI overhead may distort results)
- **nway**: bridge=525.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=900.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=335.5% of algo (FFI overhead may distort results)

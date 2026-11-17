# fold_light, short fiber (1 to 4 morsels), W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 58% faster than the next best (head_tail)

kernel_seq (2.56 us) leads head_tail (4.05 us) by 58%, a clear separation rather than a photo finish. CV 9.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 37% (significant)

kernel_seq is -1.50 us (37%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 12.4x slower than the field

std_parallel (31.61 us) is 12.4x the fastest (2.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.59)

sequential's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 12.4x the fastest

Fastest kernel_seq (2.56 us) to slowest std_parallel (31.61 us): 12.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2559.2 ns median (-36.8% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 12.35x (fastest 2559.2 ns, slowest 31606.7 ns)

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
| head_tail | 4708ns | 4654ns | 4560ns | 4637ns | 4888ns | base |
| head_tail_dyn | 4880ns | 4875ns | 4616ns | 4832ns | 5084ns | +3.66% |
| kernel_nway | 4860ns | 4789ns | 4606ns | 4757ns | 5141ns | +3.23% |
| kernel_seq | 3108ns | 3078ns | 2792ns | 3024ns | 3392ns | -33.98% |
| nway | 20055ns | 19842ns | 18016ns | 19270ns | 22252ns | +326.00% |
| sequential | 10370ns | 10309ns | 9933ns | 10249ns | 10771ns | +120.28% |
| std_parallel | 33214ns | 31968ns | 29442ns | 31869ns | 37119ns | +605.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4105ns | 3967ns | 4266ns | base | 0.016 |
| head_tail_dyn | 4250ns | 4011ns | 4460ns | +3.53% | 0.015 |
| kernel_nway | 4252ns | 4021ns | 4523ns | +3.59% | 0.015 |
| kernel_seq | 2648ns | 2458ns | 2898ns | -35.50% | 0.024 |
| nway | 19522ns | 17488ns | 21684ns | +375.55% | 0.003 |
| sequential | 10225ns | 9786ns | 10621ns | +149.06% | 0.006 |
| std_parallel | 32822ns | 29070ns | 36728ns | +699.52% | 0.002 |

## Performance model

- Peak throughput: **0.026 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.016 | 60.7% |
| head_tail_dyn | 0.015 | 58.1% |
| kernel_nway | 0.015 | 58.8% |
| kernel_seq | 0.025 | 96.0% |
| nway | 0.003 | 12.7% |
| sequential | 0.006 | 24.2% |
| std_parallel | 0.002 | 7.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4708ns | 4708ns | base |
| head_tail_dyn | 4880ns | 4880ns | +3.66% |
| kernel_nway | 4860ns | 4860ns | +3.23% |
| kernel_seq | 3108ns | 3108ns | -33.98% |
| nway | 20055ns | 20055ns | +326.00% |
| sequential | 10370ns | 10370ns | +120.28% |
| std_parallel | 33214ns | 33214ns | +605.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4050ns | base | --- | [4026, 4152] | --- | --- | --- | --- |
| head_tail_dyn | 4229ns | no significant difference | [-22, +282]ns | [4117, 4377] | no | 0.2379 | 0.2379 | 0 |
| kernel_nway | 4182ns | +124.2ns (+3.1%) | [+31, +292]ns | [4082, 4292] | YES | 0.0090 | 0.0075 | 0 |
| kernel_seq | 2559ns | -1503.3ns (-37.1%) | [-1630, -1376]ns | [2512, 2675] | YES | 0.0000 | 0.0000 | 0 |
| nway | 19328ns | +15309.1ns (+378.0%) | [+13787, +17038]ns | [17794, 21077] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10172ns | +6107.5ns (+150.8%) | [+5968, +6180]ns | [10038, 10242] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 31607ns | +27493.3ns (+678.8%) | [+27015, +29584]ns | [31015, 33588] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4050ns | +0.8% | +1.0% | -31.9% | +388.8% | +150.9% | +701.7% |
| 1 | 2 | 4050ns | +4.4% | +1.3% | -37.2% | +427.0% | +148.9% | +737.7% |
| 1 | 3 | 4108ns | +7.1% | +4.3% | -38.8% | +329.3% | +148.4% | +650.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.167 | ok |
| head_tail_dyn | -0.001 | ok |
| kernel_nway | -0.133 | ok |
| kernel_seq | -0.104 | ok |
| nway | -0.442 | moderate- |
| sequential | -0.593 | HIGH- (thermal bounce) |
| std_parallel | -0.477 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 97860.6ns | 4105.2ns | 2383.8% | HIGH |
| head_tail_dyn | 94125.0ns | 4250.2ns | 2214.6% | HIGH |
| kernel_nway | 96132.0ns | 4252.4ns | 2260.7% | HIGH |
| kernel_seq | 93294.0ns | 2647.8ns | 3523.5% | HIGH |
| nway | 97662.2ns | 19522.2ns | 500.3% | HIGH |
| sequential | 90689.9ns | 10224.5ns | 887.0% | HIGH |
| std_parallel | 105932.2ns | 32821.9ns | 322.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 3967.2-4266.4 ns)
   3967.2 |
   3982.2 |#############
   3997.1 |#############
   4012.1 |#############
   4027.1 |########################################
   4042.0 |##########################
   4057.0 |#############
   4071.9 |
   4086.9 |#############
   4101.9 |
   4116.8 |#############
   4131.8 |
   4146.7 |
   4161.7 |#############
   4176.6 |#############
   4191.6 |
   4206.6 |
   4221.5 |#############
   4236.5 |
   4251.4 |#############
  (2 below, 1 above range)

head_tail_dyn (n=18, range 4010.6-4459.7 ns)
   4010.6 |
   4033.0 |####################
   4055.5 |
   4077.9 |####################
   4100.4 |########################################
   4122.9 |########################################
   4145.3 |
   4167.8 |####################
   4190.2 |
   4212.7 |
   4235.1 |
   4257.6 |
   4280.0 |####################
   4302.5 |####################
   4325.0 |
   4347.4 |####################
   4369.9 |########################################
   4392.3 |
   4414.8 |########################################
   4437.2 |
  (2 below, 2 above range)

kernel_nway (n=18, range 4020.6-4523.1 ns)
   4020.6 |
   4045.7 |##########################
   4070.8 |##########################
   4095.9 |##########################
   4121.1 |
   4146.2 |
   4171.3 |##########################
   4196.4 |
   4221.6 |
   4246.7 |#############
   4271.8 |########################################
   4296.9 |
   4322.1 |
   4347.2 |
   4372.3 |
   4397.4 |
   4422.6 |
   4447.7 |
   4472.8 |
   4497.9 |
  (2 below, 4 above range)

kernel_seq (n=18, range 2457.8-2897.5 ns)
   2457.8 |
   2479.8 |##########################
   2501.7 |########################################
   2523.7 |#############
   2545.7 |##########################
   2567.7 |#############
   2589.7 |
   2611.7 |#############
   2633.7 |
   2655.6 |#############
   2677.6 |##########################
   2699.6 |
   2721.6 |
   2743.6 |
   2765.6 |
   2787.6 |
   2809.6 |#############
   2831.5 |
   2853.5 |
   2875.5 |
  (2 below, 2 above range)

nway (n=18, range 17487.8-21684.2 ns)
  17487.8 |##########################
  17697.6 |########################################
  17907.4 |
  18117.2 |#############
  18327.0 |
  18536.9 |
  18746.7 |#############
  18956.5 |
  19166.3 |#############
  19376.1 |##########################
  19586.0 |
  19795.8 |
  20005.6 |#############
  20215.4 |
  20425.2 |
  20635.1 |
  20844.9 |
  21054.7 |#############
  21264.5 |#############
  21474.3 |#############
  (1 below, 3 above range)

sequential (n=18, range 9786.1-10621.1 ns)
   9786.1 |########################################
   9827.9 |
   9869.6 |
   9911.4 |#############
   9953.1 |
   9994.9 |
  10036.6 |
  10078.4 |
  10120.1 |########################################
  10161.9 |########################################
  10203.6 |##########################
  10245.4 |##########################
  10287.1 |
  10328.9 |
  10370.6 |
  10412.4 |
  10454.1 |
  10495.9 |
  10537.6 |
  10579.4 |
  (1 below, 3 above range)

std_parallel (n=18, range 29070.0-36727.5 ns)
  29070.0 |
  29452.9 |#############
  29835.8 |#############
  30218.6 |#############
  30601.5 |#############
  30984.4 |##########################
  31367.3 |########################################
  31750.1 |#############
  32133.0 |
  32515.9 |
  32898.8 |#############
  33281.6 |##########################
  33664.5 |
  34047.4 |##########################
  34430.3 |
  34813.1 |
  35196.0 |
  35578.9 |
  35961.8 |
  36344.6 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=2435.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2229.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=2276.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3651.1% of algo (FFI overhead may distort results)
- **nway**: bridge=495.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=892.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=332.9% of algo (FFI overhead may distort results)

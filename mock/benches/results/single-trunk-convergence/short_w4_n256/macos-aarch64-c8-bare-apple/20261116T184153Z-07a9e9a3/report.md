# fold_light, short fiber (1 to 4 morsels), W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 31% faster than the next best (head_tail)

kernel_seq (3.39 us) leads head_tail (4.45 us) by 31%, a clear separation rather than a photo finish. CV 69.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 26% (significant)

kernel_seq is -1.16 us (26%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 12.6x slower than the field

std_parallel (42.67 us) is 12.6x the fastest (3.39 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq is fastest but the noisiest (CV 69.9%)

kernel_seq wins on median (3.39 us) yet has the highest variance (CV 69.9%), while sequential is the steadiest (CV 3.3%, 10.74 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 12.6x the fastest

Fastest kernel_seq (3.39 us) to slowest std_parallel (42.67 us): 12.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_seq is inconsistent: worst-20% is 2.5x its best-20%

kernel_seq's best 20% of batches run at 2.68 us but its worst 20% at 6.66 us (2.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 3392.5 ns median (-23.8% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 12.58x (fastest 3392.5 ns, slowest 42673.3 ns)

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
| head_tail | 4923ns | 4828ns | 4603ns | 4815ns | 5245ns | base |
| head_tail_dyn | 5109ns | 5068ns | 4631ns | 4927ns | 5622ns | +3.79% |
| kernel_nway | 9848ns | 9690ns | 8134ns | 9341ns | 11464ns | +100.04% |
| kernel_seq | 4737ns | 3631ns | 3155ns | 3604ns | 7226ns | -3.78% |
| nway | 27989ns | 26222ns | 20783ns | 25557ns | 35239ns | +468.56% |
| sequential | 10926ns | 10884ns | 10413ns | 10814ns | 11352ns | +121.96% |
| std_parallel | 45141ns | 42914ns | 34989ns | 41795ns | 55235ns | +816.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4537ns | 4193ns | 4883ns | base | 0.056 |
| head_tail_dyn | 4701ns | 4161ns | 5208ns | +3.62% | 0.054 |
| kernel_nway | 9433ns | 7830ns | 10933ns | +107.91% | 0.027 |
| kernel_seq | 4283ns | 2684ns | 6661ns | -5.59% | 0.060 |
| nway | 27602ns | 20497ns | 34752ns | +508.39% | 0.009 |
| sequential | 10759ns | 10271ns | 11160ns | +137.15% | 0.024 |
| std_parallel | 44833ns | 34741ns | 54817ns | +888.17% | 0.006 |

## Performance model

- Peak throughput: **0.095 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.058 | 60.3% |
| head_tail_dyn | 0.055 | 57.9% |
| kernel_nway | 0.028 | 29.5% |
| kernel_seq | 0.075 | 79.1% |
| nway | 0.010 | 10.4% |
| sequential | 0.024 | 25.0% |
| std_parallel | 0.006 | 6.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4923ns | 4923ns | base |
| head_tail_dyn | 5109ns | 5109ns | +3.79% |
| kernel_nway | 9848ns | 9848ns | +100.04% |
| kernel_seq | 4737ns | 4737ns | -3.78% |
| nway | 27989ns | 27989ns | +468.56% |
| sequential | 10926ns | 10926ns | +121.96% |
| std_parallel | 45141ns | 45141ns | +816.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4449ns | base | --- | [4370, 4582] | --- | --- | --- | --- |
| head_tail_dyn | 4634ns | no significant difference | [-96, +431]ns | [4391, 4859] | no | 0.4807 | 0.4807 | 0 |
| kernel_nway | 9108ns | +4685.9ns (+105.3%) | [+3869, +5522]ns | [8600, 9949] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 3392ns | -1160.0ns (-26.1%) | [-1406, -608]ns | [2955, 3870] | YES | 0.0371 | 0.0309 | 0 |
| nway | 25909ns | +20828.3ns (+468.1%) | [+18428, +28122]ns | [23062, 32347] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10736ns | +6130.9ns (+137.8%) | [+5945, +6362]ns | [10548, 10842] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 42673ns | +38167.5ns (+857.9%) | [+35043, +48293]ns | [39517, 52714] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4305ns | +1.3% | +137.2% | -34.7% | +655.3% | +146.9% | +1176.0% |
| 1 | 2 | 4496ns | +7.4% | +93.8% | -24.5% | +373.5% | +137.7% | +785.7% |
| 1 | 3 | 4762ns | -3.2% | +74.6% | +19.6% | +444.0% | +131.7% | +757.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.006 | ok |
| head_tail_dyn | -0.478 | moderate- |
| kernel_nway | -0.067 | ok |
| kernel_seq | -0.071 | ok |
| nway | -0.438 | moderate- |
| sequential | -0.096 | ok |
| std_parallel | -0.202 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 95229.4ns | 4536.9ns | 2099.0% | HIGH |
| head_tail_dyn | 95589.5ns | 4701.4ns | 2033.2% | HIGH |
| kernel_nway | 114955.6ns | 9432.6ns | 1218.7% | HIGH |
| kernel_seq | 108620.4ns | 4283.2ns | 2535.9% | HIGH |
| nway | 106081.0ns | 27602.3ns | 384.3% | HIGH |
| sequential | 91135.2ns | 10759.4ns | 847.0% | HIGH |
| std_parallel | 113843.5ns | 44832.7ns | 253.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 4193.3-4883.3 ns)
   4193.3 |########################################
   4227.8 |
   4262.3 |
   4296.8 |####################
   4331.3 |####################
   4365.8 |########################################
   4400.3 |####################
   4434.8 |########################################
   4469.3 |####################
   4503.8 |
   4538.3 |########################################
   4572.8 |####################
   4607.3 |
   4641.8 |
   4676.3 |
   4710.8 |
   4745.3 |
   4779.8 |
   4814.3 |
   4848.8 |####################
  (1 below, 3 above range)

head_tail_dyn (n=18, range 4161.1-5208.3 ns)
   4161.1 |
   4213.5 |#############
   4265.9 |#############
   4318.2 |
   4370.6 |##########################
   4422.9 |########################################
   4475.3 |
   4527.7 |
   4580.0 |
   4632.4 |
   4684.7 |
   4737.1 |
   4789.5 |########################################
   4841.8 |##########################
   4894.2 |
   4946.5 |
   4998.9 |
   5051.3 |
   5103.6 |#############
   5156.0 |
  (2 below, 3 above range)

kernel_nway (n=18, range 7830.0-10933.3 ns)
   7830.0 |##########
   7985.2 |####################
   8140.3 |
   8295.5 |
   8450.7 |##########
   8605.8 |####################
   8761.0 |####################
   8916.2 |
   9071.3 |
   9226.5 |
   9381.7 |##########
   9536.8 |
   9692.0 |
   9847.2 |########################################
  10002.3 |
  10157.5 |
  10312.7 |##########
  10467.8 |##########
  10623.0 |##########
  10778.2 |
  (1 below, 1 above range)

kernel_seq (n=18, range 2684.4-6661.1 ns)
   2684.4 |########################################
   2883.3 |##########################
   3082.1 |#############
   3280.9 |########################################
   3479.8 |########################################
   3678.6 |
   3877.4 |
   4076.3 |#############
   4275.1 |
   4473.9 |
   4672.8 |
   4871.6 |
   5070.4 |
   5269.3 |#############
   5468.1 |
   5666.9 |
   5865.8 |#############
   6064.6 |
   6263.4 |
   6462.3 |
  (1 below, 2 above range)

nway (n=18, range 20496.7-34752.0 ns)
  20496.7 |##########################
  21209.4 |#############
  21922.2 |#############
  22635.0 |
  23347.7 |########################################
  24060.5 |
  24773.3 |#############
  25486.0 |
  26198.8 |#############
  26911.6 |
  27624.3 |
  28337.1 |
  29049.8 |#############
  29762.6 |#############
  30475.4 |
  31188.1 |
  31900.9 |#############
  32613.7 |##########################
  33326.4 |#############
  34039.2 |
  (1 below, 2 above range)

sequential (n=18, range 10271.1-11160.3 ns)
  10271.1 |
  10315.6 |
  10360.0 |##########################
  10404.5 |
  10448.9 |#############
  10493.4 |
  10537.9 |
  10582.3 |#############
  10626.8 |
  10671.2 |########################################
  10715.7 |
  10760.2 |##########################
  10804.6 |##########################
  10849.1 |#############
  10893.5 |
  10938.0 |
  10982.5 |
  11026.9 |
  11071.4 |
  11115.8 |
  (2 below, 4 above range)

std_parallel (n=18, range 34741.1-54817.2 ns)
  34741.1 |#############
  35744.9 |
  36748.7 |##########################
  37752.5 |
  38756.3 |##########################
  39760.1 |#############
  40763.9 |
  41767.7 |########################################
  42771.5 |#############
  43775.4 |
  44779.2 |
  45783.0 |#############
  46786.8 |
  47790.6 |
  48794.4 |
  49798.2 |
  50802.0 |
  51805.8 |#############
  52809.6 |#############
  53813.4 |#############
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=2119.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2051.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1258.4% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=55.4% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=2825.3% of algo (FFI overhead may distort results)
- **nway**: CV=21.6% (high variance, measurements may be unstable)
- **nway**: bridge=400.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=860.2% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=266.1% of algo (FFI overhead may distort results)

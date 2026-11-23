# fold_light, far above cache, W = 1

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (head_tail, sequential) are a dead heat (<1%)

head_tail (4.14 ms) and sequential (4.14 ms) differ by 0.10%, inside the noise, even though the wider field spreads 18.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.67)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (4.14 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (head_tail) is the fastest** at 4136855.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.19x (fastest 4136855.5 ns, slowest 4911673.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 1 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112823136 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 4158113ns | 4143239ns | 4077908ns | 4139199ns | 4226587ns | base |
| head_tail_dyn | 4154989ns | 4149652ns | 4128216ns | 4144510ns | 4184093ns | -0.08% |
| kernel_nway | 4516810ns | 4188912ns | 4153634ns | 4180652ns | 5202636ns | +8.63% |
| kernel_seq | 4161336ns | 4163686ns | 4123786ns | 4150814ns | 4195893ns | +0.08% |
| nway | 4207442ns | 4197309ns | 4133636ns | 4184952ns | 4278081ns | +1.19% |
| sequential | 3947581ns | 4142129ns | 3454861ns | 3937139ns | 4209605ns | -5.06% |
| std_parallel | 4909515ns | 4915010ns | 4783206ns | 4903744ns | 4981328ns | +18.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4151368ns | 4075442ns | 4214719ns | base | 4.041 |
| head_tail_dyn | 4149798ns | 4126134ns | 4175227ns | -0.04% | 4.043 |
| kernel_nway | 4509279ns | 4151480ns | 5191574ns | +8.62% | 3.721 |
| kernel_seq | 4155773ns | 4121752ns | 4186990ns | +0.11% | 4.037 |
| nway | 4205985ns | 4132470ns | 4276778ns | +1.32% | 3.989 |
| sequential | 3946490ns | 3453754ns | 4208515ns | -4.94% | 4.251 |
| std_parallel | 4905983ns | 4781039ns | 4976315ns | +18.18% | 3.420 |

## Performance model

- Peak throughput: **4.858 Gops/s** (sequential; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 4.056 | 83.5% |
| head_tail_dyn | 4.048 | 83.3% |
| kernel_nway | 4.013 | 82.6% |
| kernel_seq | 4.035 | 83.1% |
| nway | 3.999 | 82.3% |
| sequential | 4.051 | 83.4% |
| std_parallel | 3.416 | 70.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4158113ns | 4158113ns | base |
| head_tail_dyn | 4154989ns | 4154989ns | -0.08% |
| kernel_nway | 4516810ns | 4516810ns | +8.63% |
| kernel_seq | 4161336ns | 4161336ns | +0.08% |
| nway | 4207442ns | 4207442ns | +1.19% |
| sequential | 3947581ns | 3947581ns | -5.06% |
| std_parallel | 4909515ns | 4909515ns | +18.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4136855ns | base | --- | [4102528, 4214719] | --- | --- | --- | --- |
| head_tail_dyn | 4144786ns | no significant difference | [-55540, +51724]ns | [4129380, 4175227] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 4180252ns | no significant difference | [-34467, +1055379]ns | [4156013, 5191574] | no | 1.0000 | 0.6875 | 0 |
| kernel_seq | 4157745ns | no significant difference | [-27730, +44808]ns | [4122585, 4186990] | no | 1.0000 | 1.0000 | 0 |
| nway | 4195627ns | no significant difference | [-19092, +167669]ns | [4145551, 4276778] | no | 1.0000 | 0.6875 | 0 |
| sequential | 4141099ns | no significant difference | [-619251, +10824]ns | [3489857, 4208515] | no | 1.0000 | 1.0000 | 0 |
| std_parallel | 4911674ns | +752282.4ns (+18.2%) | [+691342, +820223]ns | [4829961, 4976315] | YES (adj: no) | 0.1875 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4167996ns | -0.6% | +24.1% | -0.2% | -0.2% | -0.0% | +18.6% |
| 1 | 2 | 4109108ns | +1.3% | +2.0% | +1.1% | +4.1% | -15.1% | +18.9% |
| 1 | 3 | 4176998ns | -0.8% | -0.3% | -0.5% | +0.1% | +0.1% | +17.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.657 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.667 | HIGH- (thermal bounce) |
| kernel_nway | -0.153 | ok |
| kernel_seq | -0.183 | ok |
| nway | -0.651 | HIGH- (thermal bounce) |
| sequential | -0.666 | HIGH- (thermal bounce) |
| std_parallel | -0.205 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 1/3, lost 1/3
- **sequential**: won 1/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 4161935.0ns | 4151367.5ns | 100.3% | HIGH |
| head_tail_dyn | 4169349.9ns | 4149797.6ns | 100.5% | HIGH |
| kernel_nway | 4411265.6ns | 4509279.5ns | 97.8% | HIGH |
| kernel_seq | 4172197.5ns | 4155773.0ns | 100.4% | HIGH |
| nway | 4217781.2ns | 4205985.4ns | 100.3% | HIGH |
| sequential | 3947672.4ns | 3946490.4ns | 100.0% | HIGH |
| std_parallel | 4954929.8ns | 4905983.2ns | 101.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 4075441.5-4214719.4 ns)
  4075441.5 |####################
  4082405.4 |
  4089369.3 |
  4096333.2 |
  4103297.1 |
  4110261.0 |
  4117224.9 |
  4124188.8 |########################################
  4131152.7 |
  4138116.6 |####################
  4145080.5 |
  4152044.3 |
  4159008.2 |
  4165972.1 |
  4172936.0 |
  4179899.9 |
  4186863.8 |
  4193827.7 |
  4200791.6 |####################
  4207755.5 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 4126134.2-4175226.7 ns)
  4126134.2 |####################
  4128588.8 |
  4131043.5 |########################################
  4133498.1 |
  4135952.7 |
  4138407.3 |
  4140862.0 |
  4143316.6 |
  4145771.2 |
  4148225.8 |
  4150680.5 |
  4153135.1 |
  4155589.7 |####################
  4158044.3 |
  4160499.0 |####################
  4162953.6 |
  4165408.2 |
  4167862.8 |
  4170317.5 |
  4172772.1 |
  (0 below, 1 above range)

kernel_nway (n=6, range 4151479.8-5191573.6 ns)
  4151479.8 |########################################
  4203484.5 |##########
  4255489.2 |
  4307493.9 |
  4359498.6 |
  4411503.2 |
  4463507.9 |
  4515512.6 |
  4567517.3 |
  4619522.0 |
  4671526.7 |
  4723531.4 |
  4775536.1 |
  4827540.8 |
  4879545.5 |
  4931550.1 |
  4983554.8 |
  5035559.5 |
  5087564.2 |
  5139568.9 |
  (0 below, 1 above range)

kernel_seq (n=6, range 4121751.5-4186989.5 ns)
  4121751.5 |########################################
  4125013.4 |
  4128275.3 |####################
  4131537.2 |
  4134799.1 |
  4138061.0 |
  4141322.9 |
  4144584.8 |
  4147846.7 |
  4151108.6 |
  4154370.5 |
  4157632.4 |
  4160894.3 |
  4164156.2 |
  4167418.1 |
  4170680.0 |
  4173941.9 |
  4177203.8 |
  4180465.7 |
  4183727.6 |########################################
  (0 below, 1 above range)

nway (n=6, range 4132470.0-4276777.7 ns)
  4132470.0 |########################################
  4139685.4 |
  4146900.8 |
  4154116.1 |########################################
  4161331.5 |
  4168546.9 |
  4175762.3 |
  4182977.7 |########################################
  4190193.1 |
  4197408.4 |########################################
  4204623.8 |
  4211839.2 |
  4219054.6 |
  4226270.0 |
  4233485.4 |
  4240700.7 |
  4247916.1 |########################################
  4255131.5 |
  4262346.9 |
  4269562.3 |
  (0 below, 1 above range)

sequential (n=6, range 3453753.7-4208514.8 ns)
  3453753.7 |####################
  3491491.8 |####################
  3529229.8 |
  3566967.9 |
  3604705.9 |
  3642444.0 |
  3680182.0 |
  3717920.1 |
  3755658.1 |
  3793396.2 |
  3831134.2 |
  3868872.3 |
  3906610.3 |
  3944348.4 |
  3982086.4 |
  4019824.5 |
  4057562.5 |
  4095300.6 |
  4133038.6 |########################################
  4170776.7 |####################
  (0 below, 1 above range)

std_parallel (n=6, range 4781039.1-4976314.6 ns)
  4781039.1 |########################################
  4790802.9 |
  4800566.6 |
  4810330.4 |
  4820094.2 |
  4829858.0 |
  4839621.8 |
  4849385.5 |
  4859149.3 |
  4868913.1 |
  4878676.8 |########################################
  4888440.6 |
  4898204.4 |########################################
  4907968.2 |
  4917731.9 |########################################
  4927495.7 |
  4937259.5 |
  4947023.3 |
  4956787.0 |########################################
  4966550.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=100.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=101.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.0% of algo (FFI overhead may distort results)
- **nway**: bridge=100.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=99.7% of algo (FFI overhead may distort results)

# fold_heavy, frames of tens of milliseconds, W = 1

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel shows alternating (throttle bounce) (autocorr -0.61)

std_parallel's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader std_parallel vs stability leader sequential (+8% speed for 3.2x steadier)

std_parallel is fastest (4.25 ms, CV 6.2%); sequential gives up 7.9% median for 3.2x lower variance (CV 1.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### head_tail is inconsistent: worst-20% is 1.7x its best-20%

head_tail's best 20% of batches run at 4.47 ms but its worst 20% at 7.82 ms (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_parallel** at 4254557.1 ns median (-11.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.13x (fastest 4254557.1 ns, slowest 4819600.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, frames of tens of milliseconds, W = 1 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100350026 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 5722843ns | 4785559ns | 4474944ns | 4738489ns | 7823323ns | base |
| head_tail_dyn | 4597497ns | 4339049ns | 4055168ns | 4314265ns | 5293509ns | -19.66% |
| kernel_nway | 4855097ns | 4534014ns | 4164100ns | 4442098ns | 5820094ns | -15.16% |
| kernel_seq | 5055175ns | 4823978ns | 4053311ns | 4593106ns | 6249209ns | -11.67% |
| nway | 4676665ns | 4497943ns | 4174740ns | 4409258ns | 5328739ns | -18.28% |
| sequential | 4614880ns | 4594478ns | 4509374ns | 4579590ns | 4720567ns | -19.36% |
| std_parallel | 4387538ns | 4257589ns | 4150738ns | 4236689ns | 4732210ns | -23.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 5718803ns | 4472212ns | 7818692ns | base | 0.183 |
| head_tail_dyn | 4593672ns | 4052563ns | 5289382ns | -19.67% | 0.228 |
| kernel_nway | 4851457ns | 4159726ns | 5816197ns | -15.17% | 0.216 |
| kernel_seq | 5051153ns | 4049021ns | 6244746ns | -11.67% | 0.208 |
| nway | 4674608ns | 4173026ns | 5326927ns | -18.26% | 0.224 |
| sequential | 4612968ns | 4507915ns | 4718926ns | -19.34% | 0.227 |
| std_parallel | 4384198ns | 4145641ns | 4728578ns | -23.34% | 0.239 |

## Performance model

- Peak throughput: **0.259 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.219 | 84.7% |
| head_tail_dyn | 0.242 | 93.4% |
| kernel_nway | 0.231 | 89.4% |
| kernel_seq | 0.218 | 84.0% |
| nway | 0.233 | 90.1% |
| sequential | 0.228 | 88.2% |
| std_parallel | 0.246 | 95.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 5722843ns | 5722843ns | base |
| head_tail_dyn | 4597497ns | 4597497ns | -19.66% |
| kernel_nway | 4855097ns | 4855097ns | -15.16% |
| kernel_seq | 5055175ns | 5055175ns | -11.67% |
| nway | 4676665ns | 4676665ns | -18.28% |
| sequential | 4614880ns | 4614880ns | -19.36% |
| std_parallel | 4387538ns | 4387538ns | -23.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4781784ns | base | --- | [4555932, 7818692] | --- | --- | --- | --- |
| head_tail_dyn | 4334028ns | -452006.0ns (-9.5%) | [-2885016, -38372]ns | [4157605, 5289382] | YES (adj: no) | 0.6563 | 0.2188 | 0 |
| kernel_nway | 4530440ns | no significant difference | [-3581757, +1034413]ns | [4207734, 5816197] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 4819600ns | no significant difference | [-2999092, +1549196]ns | [4089111, 6244746] | no | 0.8250 | 0.6875 | 0 |
| nway | 4495777ns | no significant difference | [-3617572, +768480]ns | [4201120, 5326927] | no | 0.8250 | 0.6875 | 0 |
| sequential | 4591998ns | no significant difference | [-3173045, +63157]ns | [4527979, 4718926] | no | 0.8250 | 0.6875 | 0 |
| std_parallel | 4254557ns | no significant difference | [-3393277, +1788]ns | [4169458, 4728578] | no | 0.6563 | 0.2188 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 7818692ns | -36.9% | -45.8% | -38.4% | -46.3% | -40.6% | -43.4% |
| 1 | 2 | 4642166ns | -8.1% | +21.4% | -11.9% | +10.0% | -0.9% | -9.3% |
| 1 | 3 | 4695550ns | -2.5% | -0.3% | +33.0% | +0.4% | -2.2% | -3.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.175 | ok |
| head_tail_dyn | -0.480 | moderate- |
| kernel_nway | -0.602 | HIGH- (thermal bounce) |
| kernel_seq | -0.385 | moderate- |
| nway | -0.452 | moderate- |
| sequential | -0.083 | ok |
| std_parallel | -0.610 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 5756120.8ns | 5718802.8ns | 100.7% | HIGH |
| head_tail_dyn | 4830283.1ns | 4593671.5ns | 105.2% | HIGH |
| kernel_nway | 4891253.0ns | 4851456.9ns | 100.8% | HIGH |
| kernel_seq | 5072044.3ns | 5051152.6ns | 100.4% | HIGH |
| nway | 4618913.4ns | 4674608.1ns | 98.8% | HIGH |
| sequential | 4637242.9ns | 4612967.7ns | 100.5% | HIGH |
| std_parallel | 6411594.9ns | 4384197.9ns | 146.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 4472212.2-7818692.3 ns)
  4472212.2 |####################
  4639536.2 |########################################
  4806860.2 |####################
  4974184.2 |
  5141508.2 |
  5308832.2 |
  5476156.2 |
  5643480.2 |
  5810804.2 |
  5978128.2 |
  6145452.2 |
  6312776.3 |
  6480100.3 |
  6647424.3 |
  6814748.3 |
  6982072.3 |
  7149396.3 |
  7316720.3 |
  7484044.3 |####################
  7651368.3 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 4052562.8-5289381.7 ns)
  4052562.8 |####################
  4114403.7 |
  4176244.7 |
  4238085.6 |########################################
  4299926.6 |
  4361767.5 |####################
  4423608.5 |
  4485449.4 |
  4547290.4 |
  4609131.3 |
  4670972.2 |
  4732813.2 |####################
  4794654.1 |
  4856495.1 |
  4918336.0 |
  4980177.0 |
  5042017.9 |
  5103858.9 |
  5165699.8 |
  5227540.8 |
  (0 below, 1 above range)

kernel_nway (n=6, range 4159726.3-5816197.0 ns)
  4159726.3 |####################
  4242549.8 |########################################
  4325373.4 |
  4408196.9 |
  4491020.4 |
  4573844.0 |
  4656667.5 |
  4739491.0 |####################
  4822314.6 |
  4905138.1 |
  4987961.7 |
  5070785.2 |####################
  5153608.7 |
  5236432.3 |
  5319255.8 |
  5402079.3 |
  5484902.9 |
  5567726.4 |
  5650549.9 |
  5733373.5 |
  (0 below, 1 above range)

kernel_seq (n=6, range 4049020.8-6244746.2 ns)
  4049020.8 |########################################
  4158807.1 |
  4268593.3 |####################
  4378379.6 |
  4488165.9 |
  4597952.1 |
  4707738.4 |
  4817524.7 |
  4927310.9 |
  5037097.2 |
  5146883.5 |
  5256669.7 |####################
  5366456.0 |
  5476242.3 |
  5586028.5 |
  5695814.8 |
  5805601.1 |
  5915387.3 |
  6025173.6 |
  6134959.9 |####################
  (0 below, 1 above range)

nway (n=6, range 4173025.6-5326926.6 ns)
  4173025.6 |########################################
  4230720.7 |
  4288415.7 |####################
  4346110.8 |
  4403805.8 |
  4461500.8 |
  4519195.9 |
  4576891.0 |
  4634586.0 |####################
  4692281.0 |
  4749976.1 |####################
  4807671.1 |
  4865366.2 |
  4923061.2 |
  4980756.3 |
  5038451.3 |
  5096146.4 |
  5153841.4 |
  5211536.5 |
  5269231.5 |
  (0 below, 1 above range)

sequential (n=6, range 4507915.2-4718926.0 ns)
  4507915.2 |########################################
  4518465.7 |
  4529016.3 |
  4539566.8 |########################################
  4550117.4 |
  4560667.9 |
  4571218.4 |
  4581769.0 |########################################
  4592319.5 |########################################
  4602870.1 |
  4613420.6 |
  4623971.1 |
  4634521.7 |
  4645072.2 |########################################
  4655622.8 |
  4666173.3 |
  4676723.8 |
  4687274.4 |
  4697824.9 |
  4708375.5 |
  (0 below, 1 above range)

std_parallel (n=6, range 4145640.7-4728578.2 ns)
  4145640.7 |########################################
  4174787.6 |########################################
  4203934.5 |########################################
  4233081.3 |
  4262228.2 |########################################
  4291375.1 |
  4320522.0 |
  4349668.8 |
  4378815.7 |
  4407962.6 |
  4437109.5 |
  4466256.3 |
  4495403.2 |
  4524550.1 |
  4553697.0 |########################################
  4582843.8 |
  4611990.7 |
  4641137.6 |
  4670284.5 |
  4699431.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=26.2% (high variance, measurements may be unstable)
- **head_tail**: bridge=102.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=104.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=102.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=97.7% of algo (FFI overhead may distort results)
- **nway**: bridge=99.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=101.7% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=171.1% of algo (FFI overhead may distort results)

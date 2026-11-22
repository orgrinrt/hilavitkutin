# fold_light, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 40% faster than the next best (head_tail_dyn)

kernel_seq (3.20 us) leads head_tail_dyn (4.47 us) by 40%, a clear separation rather than a photo finish. CV 13.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 29% (significant)

kernel_seq is -1.29 us (29%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 27.2x slower than the field

std_parallel (87.02 us) is 27.2x the fastest (3.20 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.64)

kernel_seq's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 27.2x the fastest

Fastest kernel_seq (3.20 us) to slowest std_parallel (87.02 us): 27.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 1.5x its best-20%

kernel_nway's best 20% of batches run at 15.18 us but its worst 20% at 23.11 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 3195.8 ns median (-29.3% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 27.23x (fastest 3195.8 ns, slowest 87016.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 8 |
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
| head_tail | 5203ns | 5169ns | 4868ns | 5062ns | 5581ns | base |
| head_tail_dyn | 5089ns | 5042ns | 4928ns | 5025ns | 5265ns | -2.18% |
| kernel_nway | 20451ns | 21188ns | 15781ns | 19809ns | 23750ns | +293.09% |
| kernel_seq | 3867ns | 3848ns | 3399ns | 3742ns | 4288ns | -25.68% |
| nway | 53709ns | 54090ns | 46469ns | 53655ns | 57412ns | +932.32% |
| sequential | 10871ns | 10650ns | 10231ns | 10523ns | 11713ns | +108.94% |
| std_parallel | 87697ns | 87346ns | 81251ns | 86285ns | 93040ns | +1585.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4573ns | 4231ns | 4924ns | base | 0.896 |
| head_tail_dyn | 4491ns | 4307ns | 4654ns | -1.81% | 0.912 |
| kernel_nway | 19805ns | 15185ns | 23107ns | +333.07% | 0.207 |
| kernel_seq | 3255ns | 2886ns | 3642ns | -28.83% | 1.258 |
| nway | 53133ns | 45918ns | 56817ns | +1061.85% | 0.077 |
| sequential | 10722ns | 10086ns | 11566ns | +134.45% | 0.382 |
| std_parallel | 87306ns | 80899ns | 92616ns | +1809.11% | 0.047 |

## Performance model

- Peak throughput: **1.419 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.906 | 63.8% |
| head_tail_dyn | 0.917 | 64.6% |
| kernel_nway | 0.199 | 14.0% |
| kernel_seq | 1.282 | 90.3% |
| nway | 0.076 | 5.4% |
| sequential | 0.391 | 27.5% |
| std_parallel | 0.047 | 3.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 5203ns | 5203ns | base |
| head_tail_dyn | 5089ns | 5089ns | -2.18% |
| kernel_nway | 20451ns | 20451ns | +293.09% |
| kernel_seq | 3867ns | 3867ns | -25.68% |
| nway | 53709ns | 53709ns | +932.32% |
| sequential | 10871ns | 10871ns | +108.94% |
| std_parallel | 87697ns | 87697ns | +1585.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4523ns | base | --- | [4360, 4677] | --- | --- | --- | --- |
| head_tail_dyn | 4467ns | no significant difference | [-246, +252]ns | [4412, 4540] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 20569ns | +15700.1ns (+347.1%) | [+12421, +17546]ns | [16865, 21842] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 3196ns | -1289.6ns (-28.5%) | [-1748, -1065]ns | [2965, 3327] | YES | 0.0002 | 0.0001 | 0 |
| nway | 53562ns | +48912.5ns (+1081.4%) | [+47842, +50619]ns | [52329, 55138] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10483ns | +6127.1ns (+135.5%) | [+5496, +6719]ns | [10108, 11110] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 87017ns | +82635.4ns (+1827.0%) | [+79654, +85152]ns | [83706, 89660] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4360ns | +2.5% | +352.6% | -26.2% | +1125.5% | +162.9% | +1923.5% |
| 1 | 2 | 4685ns | -5.4% | +256.9% | -36.9% | +1052.6% | +115.6% | +1824.6% |
| 1 | 3 | 4490ns | +0.0% | +413.3% | -27.0% | +1081.7% | +134.5% | +1734.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.563 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.601 | HIGH- (thermal bounce) |
| kernel_nway | -0.484 | moderate- |
| kernel_seq | -0.644 | HIGH- (thermal bounce) |
| nway | -0.559 | HIGH- (thermal bounce) |
| sequential | -0.367 | moderate- |
| std_parallel | -0.319 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 94061.3ns | 4573.1ns | 2056.8% | HIGH |
| head_tail_dyn | 93534.0ns | 4490.5ns | 2082.9% | HIGH |
| kernel_nway | 111363.0ns | 19804.9ns | 562.3% | HIGH |
| kernel_seq | 94134.3ns | 3254.9ns | 2892.1% | HIGH |
| nway | 110545.4ns | 53132.9ns | 208.1% | HIGH |
| sequential | 91312.7ns | 10721.8ns | 851.7% | HIGH |
| std_parallel | 128202.5ns | 87306.5ns | 146.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 4230.5-4924.3 ns)
   4230.5 |########################################
   4265.2 |####################
   4299.9 |
   4334.6 |########################################
   4369.3 |
   4404.0 |########################################
   4438.7 |
   4473.3 |####################
   4508.0 |####################
   4542.7 |
   4577.4 |########################################
   4612.1 |####################
   4646.8 |
   4681.5 |
   4716.2 |########################################
   4750.8 |####################
   4785.5 |
   4820.2 |
   4854.9 |
   4889.6 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 4307.0-4654.2 ns)
   4307.0 |
   4324.3 |
   4341.7 |####################
   4359.0 |
   4376.4 |
   4393.8 |####################
   4411.1 |##########
   4428.5 |##########
   4445.8 |##########
   4463.2 |########################################
   4480.6 |
   4497.9 |
   4515.3 |##########
   4532.6 |##########
   4550.0 |##########
   4567.4 |
   4584.7 |
   4602.1 |
   4619.4 |
   4636.8 |
  (1 below, 3 above range)

kernel_nway (n=18, range 15184.7-23106.9 ns)
  15184.7 |
  15580.8 |#############
  15977.0 |
  16373.1 |##########################
  16769.2 |#############
  17165.3 |
  17561.4 |
  17957.5 |
  18353.6 |
  18749.7 |
  19145.8 |
  19541.9 |##########################
  19938.1 |
  20334.2 |########################################
  20730.3 |##########################
  21126.4 |
  21522.5 |
  21918.6 |
  22314.7 |#############
  22710.8 |
  (2 below, 4 above range)

kernel_seq (n=18, range 2886.1-3641.7 ns)
   2886.1 |
   2923.9 |########################################
   2961.7 |########################################
   2999.5 |
   3037.2 |
   3075.0 |
   3112.8 |####################
   3150.6 |########################################
   3188.3 |####################
   3226.1 |####################
   3263.9 |####################
   3301.7 |####################
   3339.5 |########################################
   3377.2 |
   3415.0 |####################
   3452.8 |
   3490.6 |####################
   3528.3 |
   3566.1 |
   3603.9 |
  (2 below, 1 above range)

nway (n=18, range 45918.0-56816.7 ns)
  45918.0 |
  46463.0 |
  47007.9 |
  47552.8 |
  48097.8 |#############
  48642.7 |
  49187.6 |
  49732.6 |
  50277.5 |
  50822.4 |
  51367.4 |
  51912.3 |##########################
  52457.2 |########################################
  53002.2 |#############
  53547.1 |#############
  54092.0 |##########################
  54637.0 |#############
  55181.9 |
  55726.8 |##########################
  56271.8 |#############
  (2 below, 2 above range)

sequential (n=18, range 10086.1-11566.0 ns)
  10086.1 |########################################
  10160.1 |
  10234.1 |
  10308.1 |######
  10382.1 |
  10456.1 |#############
  10530.1 |
  10604.1 |
  10678.1 |######
  10752.1 |
  10826.0 |######
  10900.0 |
  10974.0 |######
  11048.0 |
  11122.0 |######
  11196.0 |
  11270.0 |
  11344.0 |
  11418.0 |
  11492.0 |
  (1 below, 4 above range)

std_parallel (n=18, range 80898.6-92616.0 ns)
  80898.6 |####################
  81484.5 |####################
  82070.3 |
  82656.2 |####################
  83242.1 |####################
  83827.9 |####################
  84413.8 |
  84999.7 |####################
  85585.5 |
  86171.4 |####################
  86757.3 |########################################
  87343.2 |####################
  87929.0 |
  88514.9 |
  89100.8 |########################################
  89686.6 |####################
  90272.5 |####################
  90858.4 |
  91444.2 |
  92030.1 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=2069.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2075.6% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=536.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=2987.7% of algo (FFI overhead may distort results)
- **nway**: bridge=205.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=870.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=149.3% of algo (FFI overhead may distort results)

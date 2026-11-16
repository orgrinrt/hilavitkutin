# append, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 8.3x slower than the field

std_parallel (78.69 us) is 8.3x the fastest (9.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.63)

kernel_nway's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (9.50 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 8.3x the fastest

Fastest head_tail (9.50 us) to slowest std_parallel (78.69 us): 8.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 1.6x its best-20%

kernel_nway's best 20% of batches run at 19.29 us but its worst 20% at 31.19 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 9500.0 ns median
- 5 variants significantly slower than baseline
- Spread: 8.28x (fastest 9500.0 ns, slowest 78685.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 8 |
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
| head_tail | 10231ns | 10248ns | 9867ns | 10160ns | 10518ns | base |
| head_tail_dyn | 10592ns | 10521ns | 10053ns | 10479ns | 11031ns | +3.53% |
| kernel_nway | 26887ns | 26906ns | 20997ns | 25195ns | 32369ns | +162.81% |
| kernel_seq | 10319ns | 10200ns | 9806ns | 10150ns | 10827ns | +0.86% |
| nway | 64760ns | 64504ns | 63033ns | 64335ns | 66262ns | +533.01% |
| sequential | 56445ns | 56417ns | 56019ns | 56347ns | 56804ns | +451.73% |
| std_parallel | 80563ns | 80077ns | 78176ns | 79887ns | 82771ns | +687.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 9570ns | 9372ns | 9798ns | base | 6.848 |
| head_tail_dyn | 9871ns | 9433ns | 10344ns | +3.15% | 6.639 |
| kernel_nway | 25479ns | 19293ns | 31192ns | +166.24% | 2.572 |
| kernel_seq | 9793ns | 9460ns | 10182ns | +2.33% | 6.692 |
| nway | 63665ns | 62104ns | 64950ns | +565.26% | 1.029 |
| sequential | 55909ns | 55533ns | 56265ns | +484.22% | 1.172 |
| std_parallel | 79085ns | 77203ns | 81078ns | +726.39% | 0.829 |

## Performance model

- Peak throughput: **6.993 Gops/s** (head_tail; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 6.899 | 98.7% |
| head_tail_dyn | 6.679 | 95.5% |
| kernel_nway | 2.570 | 36.8% |
| kernel_seq | 6.735 | 96.3% |
| nway | 1.030 | 14.7% |
| sequential | 1.173 | 16.8% |
| std_parallel | 0.833 | 11.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 10231ns | 10231ns | base |
| head_tail_dyn | 10592ns | 10592ns | +3.53% |
| kernel_nway | 26887ns | 26887ns | +162.81% |
| kernel_seq | 10319ns | 10319ns | +0.86% |
| nway | 64760ns | 64760ns | +533.01% |
| sequential | 56445ns | 56445ns | +451.73% |
| std_parallel | 80563ns | 80563ns | +687.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 9500ns | base | --- | [9450, 9590] | --- | --- | --- | --- |
| head_tail_dyn | 9812ns | no significant difference | [-15, +510]ns | [9558, 10106] | no | 0.1155 | 0.0963 | 0 |
| kernel_nway | 25496ns | +15943.8ns (+167.8%) | [+11542, +19150]ns | [20696, 28085] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 9731ns | no significant difference | [-15, +350]ns | [9517, 9827] | no | 0.2379 | 0.2379 | 0 |
| nway | 63610ns | +54097.9ns (+569.5%) | [+53383, +54538]ns | [62917, 64117] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 55854ns | +46295.8ns (+487.3%) | [+46108, +46571]ns | [55677, 56071] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 78685ns | +69193.8ns (+728.4%) | [+68594, +69644]ns | [77973, 79179] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 9500ns | +1.9% | +168.0% | +2.9% | +564.2% | +485.4% | +724.8% |
| 1 | 2 | 9508ns | +2.7% | +160.0% | +1.0% | +565.0% | +487.9% | +720.0% |
| 1 | 3 | 9521ns | +3.9% | +170.1% | +2.1% | +570.4% | +489.1% | +744.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.009 | ok |
| head_tail_dyn | -0.007 | ok |
| kernel_nway | -0.627 | HIGH- (thermal bounce) |
| kernel_seq | -0.599 | HIGH- (thermal bounce) |
| nway | -0.080 | ok |
| sequential | -0.010 | ok |
| std_parallel | -0.258 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 442768.1ns | 9569.9ns | 4626.7% | HIGH |
| head_tail_dyn | 444393.3ns | 9871.1ns | 4502.0% | HIGH |
| kernel_nway | 457143.7ns | 25478.9ns | 1794.2% | HIGH |
| kernel_seq | 443805.1ns | 9793.1ns | 4531.8% | HIGH |
| nway | 490989.4ns | 63664.6ns | 771.2% | HIGH |
| sequential | 467374.1ns | 55909.5ns | 835.9% | HIGH |
| std_parallel | 502803.0ns | 79084.7ns | 635.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 9372.2-9797.9 ns)
   9372.2 |####################
   9393.5 |
   9414.8 |########################################
   9436.1 |####################
   9457.3 |########################################
   9478.6 |########################################
   9499.9 |####################
   9521.2 |####################
   9542.5 |####################
   9563.8 |####################
   9585.1 |####################
   9606.4 |
   9627.6 |
   9648.9 |####################
   9670.2 |
   9691.5 |
   9712.8 |####################
   9734.1 |
   9755.4 |
   9776.6 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 9433.3-10343.8 ns)
   9433.3 |########################################
   9478.9 |
   9524.4 |####################
   9569.9 |########################################
   9615.4 |
   9660.9 |
   9706.5 |
   9752.0 |########################################
   9797.5 |
   9843.0 |########################################
   9888.5 |########################################
   9934.1 |
   9979.6 |
  10025.1 |
  10070.6 |
  10116.1 |
  10161.7 |
  10207.2 |
  10252.7 |####################
  10298.2 |
  (2 below, 4 above range)

kernel_nway (n=18, range 19293.1-31191.7 ns)
  19293.1 |########################################
  19888.0 |####################
  20482.9 |########################################
  21077.9 |
  21672.8 |
  22267.7 |
  22862.6 |####################
  23457.6 |
  24052.5 |
  24647.4 |########################################
  25242.4 |
  25837.3 |####################
  26432.2 |########################################
  27027.2 |
  27622.1 |####################
  28217.0 |
  28811.9 |
  29406.9 |########################################
  30001.8 |####################
  30596.7 |
  (1 below, 2 above range)

kernel_seq (n=18, range 9459.7-10182.0 ns)
   9459.7 |########################################
   9495.8 |####################
   9531.9 |####################
   9568.0 |####################
   9604.2 |
   9640.3 |####################
   9676.4 |
   9712.5 |####################
   9748.6 |########################################
   9784.7 |########################################
   9820.8 |####################
   9856.9 |
   9893.1 |
   9929.2 |####################
   9965.3 |
  10001.4 |
  10037.5 |
  10073.6 |
  10109.7 |
  10145.9 |
  (2 below, 3 above range)

nway (n=18, range 62104.2-64950.0 ns)
  62104.2 |
  62246.5 |####################
  62388.8 |
  62531.0 |####################
  62673.3 |########################################
  62815.6 |
  62957.9 |
  63100.2 |########################################
  63242.5 |####################
  63384.8 |####################
  63527.1 |
  63669.4 |########################################
  63811.7 |########################################
  63954.0 |
  64096.2 |
  64238.5 |####################
  64380.8 |
  64523.1 |
  64665.4 |####################
  64807.7 |
  (1 below, 3 above range)

sequential (n=18, range 55533.3-56265.3 ns)
  55533.3 |##########################
  55569.9 |
  55606.5 |
  55643.1 |##########################
  55679.7 |##########################
  55716.3 |
  55752.9 |
  55789.5 |
  55826.1 |########################################
  55862.7 |
  55899.3 |
  55935.9 |#############
  55972.5 |#############
  56009.1 |#############
  56045.7 |
  56082.3 |
  56118.9 |#############
  56155.5 |#############
  56192.1 |
  56228.7 |
  (1 below, 3 above range)

std_parallel (n=18, range 77202.8-81077.8 ns)
  77202.8 |#############
  77396.5 |#############
  77590.3 |#############
  77784.0 |#############
  77977.8 |#############
  78171.5 |#############
  78365.3 |##########################
  78559.0 |
  78752.8 |########################################
  78946.5 |#############
  79140.3 |
  79334.0 |#############
  79527.8 |
  79721.5 |
  79915.3 |
  80109.0 |
  80302.8 |
  80496.5 |
  80690.3 |#############
  80884.0 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=4666.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=4526.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1787.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=4554.2% of algo (FFI overhead may distort results)
- **nway**: bridge=774.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=836.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=641.2% of algo (FFI overhead may distort results)

# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 120% faster than the next best (kernel_seq)

sequential (10.74 us) leads kernel_seq (23.68 us) by 120%, a clear separation rather than a photo finish. CV 19.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 55% (significant)

sequential is -13.37 us (55%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 4.4x slower than the field

std_parallel (47.42 us) is 4.4x the fastest (10.74 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.66)

kernel_nway's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential} vs {kernel_seq, head_tail, kernel_nway, head_tail_dyn, nway, std_parallel} (120% apart)

The field splits into a fast tier {sequential} and a slow tier {kernel_seq, head_tail, kernel_nway, head_tail_dyn, nway, std_parallel} with a 120% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.4x the fastest

Fastest sequential (10.74 us) to slowest std_parallel (47.42 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### head_tail_dyn is inconsistent: worst-20% is 5.6x its best-20%

head_tail_dyn's best 20% of batches run at 9.14 us but its worst 20% at 51.21 us (5.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 10744.9 ns median (-55.8% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 4.41x (fastest 10744.9 ns, slowest 47421.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 1 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100350026 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 24822ns | 25139ns | 20081ns | 23950ns | 28499ns | base |
| head_tail_dyn | 31387ns | 26068ns | 9609ns | 25010ns | 51840ns | +26.45% |
| kernel_nway | 25549ns | 25790ns | 23548ns | 25103ns | 27220ns | +2.93% |
| kernel_seq | 25814ns | 24564ns | 22509ns | 24196ns | 29894ns | +4.00% |
| nway | 36023ns | 35399ns | 31786ns | 34975ns | 39713ns | +45.12% |
| sequential | 10297ns | 11157ns | 7228ns | 9996ns | 12284ns | -58.51% |
| std_parallel | 48758ns | 48119ns | 37567ns | 46487ns | 57760ns | +96.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 24332ns | 19729ns | 28000ns | base | 0.001 |
| head_tail_dyn | 30887ns | 9138ns | 51209ns | +26.94% | 0.001 |
| kernel_nway | 25043ns | 23046ns | 26695ns | +2.92% | 0.001 |
| kernel_seq | 25250ns | 21985ns | 29297ns | +3.77% | 0.001 |
| nway | 35564ns | 31418ns | 39215ns | +46.16% | 0.000 |
| sequential | 9946ns | 7013ns | 11935ns | -59.12% | 0.002 |
| std_parallel | 48201ns | 37019ns | 57232ns | +98.09% | 0.000 |

## Performance model

- Peak throughput: **0.002 Gops/s** (sequential; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.001 | 28.8% |
| head_tail_dyn | 0.001 | 27.3% |
| kernel_nway | 0.001 | 27.7% |
| kernel_seq | 0.001 | 29.6% |
| nway | 0.000 | 20.1% |
| sequential | 0.001 | 65.3% |
| std_parallel | 0.000 | 14.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 24822ns | 24822ns | base |
| head_tail_dyn | 31387ns | 31387ns | +26.45% |
| kernel_nway | 25549ns | 25549ns | +2.93% |
| kernel_seq | 25814ns | 25814ns | +4.00% |
| nway | 36023ns | 36023ns | +45.12% |
| sequential | 10297ns | 10297ns | -58.51% |
| std_parallel | 48758ns | 48758ns | +96.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 24329ns | base | --- | [22457, 25285] | --- | --- | --- | --- |
| head_tail_dyn | 25652ns | +3758.6ns (+15.4%) | [+526, +4959]ns | [22764, 28991] | YES | 0.0463 | 0.0309 | 0 |
| kernel_nway | 25331ns | no significant difference | [-376, +3240]ns | [23668, 25683] | no | 0.2855 | 0.2379 | 0 |
| kernel_seq | 23675ns | no significant difference | [-2305, +4316]ns | [23021, 27399] | no | 0.4807 | 0.4807 | 0 |
| nway | 34964ns | +10434.0ns (+42.9%) | [+8458, +13239]ns | [33840, 36804] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10745ns | -13367.9ns (-54.9%) | [-14434, -12580]ns | [7667, 11394] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 47422ns | +23082.0ns (+94.9%) | [+19826, +26267]ns | [43060, 52375] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 25224ns | +31.5% | -3.1% | -7.8% | +42.9% | -53.8% | +93.2% |
| 1 | 2 | 23826ns | +17.2% | +10.8% | -3.4% | +37.6% | -54.6% | +105.4% |
| 1 | 3 | 20573ns | +10.6% | +20.1% | +34.6% | +83.5% | -65.7% | +112.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.034 | ok |
| head_tail_dyn | -0.000 | ok |
| kernel_nway | -0.657 | HIGH- (thermal bounce) |
| kernel_seq | -0.194 | ok |
| nway | -0.591 | HIGH- (thermal bounce) |
| sequential | -0.079 | ok |
| std_parallel | -0.187 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 23592.3ns | 24332.4ns | 97.0% | HIGH |
| head_tail_dyn | 36474.4ns | 30886.8ns | 118.1% | HIGH |
| kernel_nway | 26082.7ns | 25042.8ns | 104.2% | HIGH |
| kernel_seq | 25652.3ns | 25250.2ns | 101.6% | HIGH |
| nway | 35859.6ns | 35564.3ns | 100.8% | HIGH |
| sequential | 10528.2ns | 9946.1ns | 105.9% | HIGH |
| std_parallel | 48022.4ns | 48200.8ns | 99.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 19728.9-27999.9 ns)
  19728.9 |####################
  20142.5 |
  20556.0 |
  20969.6 |##########
  21383.1 |
  21796.7 |
  22210.2 |####################
  22623.8 |##########
  23037.3 |
  23450.9 |##########
  23864.4 |##########
  24278.0 |##########
  24691.5 |##########
  25105.1 |########################################
  25518.6 |
  25932.2 |
  26345.7 |
  26759.3 |
  27172.8 |##########
  27586.4 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 9137.7-51209.1 ns)
   9137.7 |
  11241.3 |
  13344.9 |
  15448.4 |
  17552.0 |
  19655.6 |
  21759.1 |########################################
  23862.7 |################
  25966.3 |########################
  28069.8 |################
  30173.4 |########
  32277.0 |
  34380.6 |
  36484.1 |
  38587.7 |
  40691.3 |########
  42794.8 |########
  44898.4 |
  47002.0 |
  49105.5 |
  (2 below, 1 above range)

kernel_nway (n=18, range 23045.7-26694.9 ns)
  23045.7 |##########
  23228.2 |##########
  23410.6 |####################
  23593.1 |
  23775.6 |####################
  23958.0 |
  24140.5 |
  24322.9 |
  24505.4 |
  24687.8 |
  24870.3 |##########
  25052.7 |##########
  25235.2 |
  25417.7 |####################
  25600.1 |########################################
  25782.6 |
  25965.0 |
  26147.5 |
  26329.9 |
  26512.4 |
  (1 below, 3 above range)

kernel_seq (n=18, range 21985.0-29297.2 ns)
  21985.0 |#############
  22350.6 |#############
  22716.2 |########################################
  23081.8 |########################################
  23447.4 |
  23813.0 |#############
  24178.7 |
  24544.3 |#############
  24909.9 |
  25275.5 |#############
  25641.1 |
  26006.7 |
  26372.3 |
  26737.9 |
  27103.5 |#############
  27469.1 |########################################
  27834.7 |
  28200.3 |
  28566.0 |
  28931.6 |
  (1 below, 2 above range)

nway (n=18, range 31417.6-39215.5 ns)
  31417.6 |##########################
  31807.5 |
  32197.4 |
  32587.2 |
  32977.1 |#############
  33367.0 |
  33756.9 |########################################
  34146.8 |#############
  34536.7 |#############
  34926.6 |#############
  35316.5 |
  35706.4 |##########################
  36096.3 |#############
  36486.2 |
  36876.1 |
  37266.0 |#############
  37655.9 |
  38045.8 |
  38435.7 |#############
  38825.6 |
  (1 below, 3 above range)

sequential (n=18, range 7013.4-11935.5 ns)
   7013.4 |####################
   7259.5 |
   7505.6 |####################
   7751.7 |
   7997.8 |
   8243.9 |
   8490.0 |
   8736.1 |
   8982.2 |
   9228.3 |
   9474.4 |
   9720.6 |##########
   9966.7 |
  10212.8 |####################
  10458.9 |
  10705.0 |
  10951.1 |####################
  11197.2 |########################################
  11443.3 |
  11689.4 |##########
  (2 below, 2 above range)

std_parallel (n=18, range 37019.1-57232.1 ns)
  37019.1 |
  38029.8 |
  39040.4 |
  40051.1 |
  41061.7 |#############
  42072.4 |##########################
  43083.0 |##########################
  44093.7 |
  45104.3 |#############
  46115.0 |
  47125.6 |##########################
  48136.3 |
  49146.9 |
  50157.6 |########################################
  51168.2 |
  52178.9 |
  53189.5 |
  54200.2 |#############
  55210.8 |#############
  56221.5 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=96.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=86.0% (high variance, measurements may be unstable)
- **head_tail_dyn**: worst_20/best_20 = 5.6x (possible bimodal distribution)
- **head_tail_dyn**: bridge=107.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=100.6% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=105.5% of algo (FFI overhead may distort results)
- **nway**: bridge=99.6% of algo (FFI overhead may distort results)
- **sequential**: CV=20.6% (high variance, measurements may be unstable)
- **sequential**: bridge=100.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=101.0% of algo (FFI overhead may distort results)

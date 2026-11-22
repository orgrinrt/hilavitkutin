# fold_light, short fiber (1 to 4 morsels), W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 16% faster than the next best (head_tail)

kernel_seq (9.86 us) leads head_tail (11.43 us) by 16%, a clear separation rather than a photo finish. CV 19.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 29% (significant)

kernel_seq is -3.32 us (29%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 5.7x slower than the field

std_parallel (56.10 us) is 5.7x the fastest (9.86 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.67)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_seq, head_tail, sequential, head_tail_dyn, kernel_nway} vs {nway, std_parallel} (173% apart)

The field splits into a fast tier {kernel_seq, head_tail, sequential, head_tail_dyn, kernel_nway} and a slow tier {nway, std_parallel} with a 173% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.7x the fastest

Fastest kernel_seq (9.86 us) to slowest std_parallel (56.10 us): 5.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.4x its best-20%

sequential's best 20% of batches run at 6.98 us but its worst 20% at 23.44 us (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 9856.5 ns median (-13.8% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 5.69x (fastest 9856.5 ns, slowest 56102.2 ns)

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
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 13652ns | 12096ns | 11749ns | 12109ns | 16917ns | base |
| head_tail_dyn | 13992ns | 13938ns | 11506ns | 13431ns | 16078ns | +2.49% |
| kernel_nway | 16848ns | 17040ns | 13997ns | 16516ns | 18772ns | +23.42% |
| kernel_seq | 10858ns | 10558ns | 7920ns | 10172ns | 13356ns | -20.47% |
| nway | 45310ns | 44097ns | 36804ns | 42317ns | 54052ns | +231.90% |
| sequential | 14914ns | 13155ns | 7118ns | 11499ns | 23935ns | +9.25% |
| std_parallel | 60560ns | 56512ns | 39023ns | 51844ns | 84402ns | +343.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 12937ns | 11142ns | 16119ns | base | 0.020 |
| head_tail_dyn | 13241ns | 10906ns | 15254ns | +2.35% | 0.019 |
| kernel_nway | 15938ns | 13226ns | 17805ns | +23.19% | 0.016 |
| kernel_seq | 10040ns | 7383ns | 12216ns | -22.39% | 0.025 |
| nway | 44688ns | 36312ns | 53252ns | +245.42% | 0.006 |
| sequential | 14615ns | 6978ns | 23441ns | +12.97% | 0.018 |
| std_parallel | 60103ns | 38800ns | 83607ns | +364.58% | 0.004 |

## Performance model

- Peak throughput: **0.037 Gops/s** (sequential; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.022 | 61.0% |
| head_tail_dyn | 0.019 | 52.6% |
| kernel_nway | 0.016 | 43.8% |
| kernel_seq | 0.026 | 70.8% |
| nway | 0.006 | 16.0% |
| sequential | 0.020 | 54.1% |
| std_parallel | 0.005 | 12.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 13652ns | 13652ns | base |
| head_tail_dyn | 13992ns | 13992ns | +2.49% |
| kernel_nway | 16848ns | 16848ns | +23.42% |
| kernel_seq | 10858ns | 10858ns | -20.47% |
| nway | 45310ns | 45310ns | +231.90% |
| sequential | 14914ns | 14914ns | +9.25% |
| std_parallel | 60560ns | 60560ns | +343.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 11432ns | base | --- | [11273, 15816] | --- | --- | --- | --- |
| head_tail_dyn | 13265ns | no significant difference | [-971, +2046]ns | [11940, 13514] | no | 0.1155 | 0.0963 | 0 |
| kernel_nway | 15935ns | +2823.0ns (+24.7%) | [+993, +5167]ns | [14955, 16883] | YES | 0.0113 | 0.0075 | 0 |
| kernel_seq | 9857ns | -3316.6ns (-29.0%) | [-5176, -1011]ns | [8910, 11084] | YES | 0.0113 | 0.0075 | 0 |
| nway | 43525ns | +31604.2ns (+276.5%) | [+23353, +36721]ns | [38238, 48518] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 12896ns | no significant difference | [-4108, +6919]ns | [7171, 20974] | no | 0.8145 | 0.8145 | 0 |
| std_parallel | 56102ns | +42689.0ns (+373.4%) | [+32000, +64509]ns | [43443, 76642] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 11206ns | +7.3% | +24.7% | -30.1% | +359.9% | -35.9% | +523.4% |
| 1 | 2 | 11432ns | +26.6% | +51.8% | +1.4% | +305.1% | +39.8% | +396.7% |
| 1 | 3 | 15866ns | -24.7% | -1.4% | -37.9% | +136.1% | +7.5% | +182.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.142 | ok |
| head_tail_dyn | -0.666 | HIGH- (thermal bounce) |
| kernel_nway | -0.504 | HIGH- (thermal bounce) |
| kernel_seq | -0.474 | moderate- |
| nway | -0.015 | ok |
| sequential | -0.113 | ok |
| std_parallel | -0.000 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 13671.1ns | 12937.1ns | 105.7% | HIGH |
| head_tail_dyn | 13548.2ns | 13241.1ns | 102.3% | HIGH |
| kernel_nway | 16624.5ns | 15937.9ns | 104.3% | HIGH |
| kernel_seq | 10416.2ns | 10040.0ns | 103.7% | HIGH |
| nway | 45595.4ns | 44687.9ns | 102.0% | HIGH |
| sequential | 15144.5ns | 14615.1ns | 103.6% | HIGH |
| std_parallel | 60423.2ns | 60102.8ns | 100.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 11142.2-16118.7 ns)
  11142.2 |########################################
  11391.1 |#################
  11639.9 |#####
  11888.7 |
  12137.5 |
  12386.4 |
  12635.2 |
  12884.0 |
  13132.8 |
  13381.7 |
  13630.5 |
  13879.3 |
  14128.1 |
  14377.0 |
  14625.8 |
  14874.6 |
  15123.4 |
  15372.3 |
  15621.1 |#################
  15869.9 |###########
  (1 below, 1 above range)

head_tail_dyn (n=18, range 10906.5-15254.2 ns)
  10906.5 |##########################
  11123.9 |
  11341.3 |
  11558.7 |
  11776.0 |########################################
  11993.4 |#############
  12210.8 |
  12428.2 |
  12645.6 |
  12863.0 |#############
  13080.3 |##########################
  13297.7 |########################################
  13515.1 |#############
  13732.5 |
  13949.9 |
  14167.3 |
  14384.7 |
  14602.0 |
  14819.4 |
  15036.8 |
  (1 below, 4 above range)

kernel_nway (n=18, range 13225.8-17805.3 ns)
  13225.8 |########################################
  13454.7 |
  13683.7 |
  13912.7 |
  14141.7 |
  14370.7 |
  14599.6 |####################
  14828.6 |########################################
  15057.6 |####################
  15286.6 |####################
  15515.5 |
  15744.5 |########################################
  15973.5 |
  16202.5 |
  16431.5 |####################
  16660.4 |########################################
  16889.4 |####################
  17118.4 |####################
  17347.4 |####################
  17576.4 |
  (1 below, 2 above range)

kernel_seq (n=18, range 7383.1-12216.1 ns)
   7383.1 |########################################
   7624.8 |
   7866.4 |
   8108.1 |####################
   8349.7 |
   8591.4 |####################
   8833.0 |####################
   9074.7 |########################################
   9316.3 |
   9558.0 |####################
   9799.6 |####################
  10041.3 |
  10282.9 |
  10524.6 |########################################
  10766.2 |####################
  11007.9 |
  11249.5 |####################
  11491.2 |####################
  11732.8 |
  11974.5 |
  (1 below, 3 above range)

nway (n=18, range 36312.4-53252.4 ns)
  36312.4 |#############
  37159.4 |########################################
  38006.4 |
  38853.4 |#############
  39700.4 |
  40547.4 |##########################
  41394.4 |
  42241.4 |#############
  43088.4 |
  43935.4 |#############
  44782.4 |
  45629.4 |##########################
  46476.4 |
  47323.4 |#############
  48170.4 |#############
  49017.4 |
  49864.4 |
  50711.4 |
  51558.4 |#############
  52405.4 |
  (1 below, 3 above range)

sequential (n=18, range 6977.7-23441.4 ns)
   6977.7 |########################################
   7800.9 |
   8624.0 |
   9447.2 |
  10270.4 |#####
  11093.6 |
  11916.8 |
  12740.0 |
  13563.1 |
  14386.3 |#####
  15209.5 |
  16032.7 |
  16855.9 |
  17679.1 |
  18502.2 |#####
  19325.4 |
  20148.6 |#####
  20971.8 |#################
  21795.0 |
  22618.2 |
  (1 below, 3 above range)

std_parallel (n=18, range 38800.2-83606.7 ns)
  38800.2 |########################################
  41040.5 |
  43280.9 |
  45521.2 |#############
  47761.5 |##########################
  50001.8 |#############
  52242.2 |
  54482.5 |
  56722.8 |
  58963.1 |#############
  61203.5 |#############
  63443.8 |#############
  65684.1 |
  67924.4 |
  70164.8 |
  72405.1 |
  74645.4 |#############
  76885.7 |##########################
  79126.1 |
  81366.4 |#############
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=104.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=102.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=102.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=103.4% of algo (FFI overhead may distort results)
- **nway**: bridge=101.8% of algo (FFI overhead may distort results)
- **sequential**: CV=51.6% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **sequential**: bridge=105.9% of algo (FFI overhead may distort results)
- **std_parallel**: CV=31.4% (high variance, measurements may be unstable)
- **std_parallel**: bridge=99.7% of algo (FFI overhead may distort results)

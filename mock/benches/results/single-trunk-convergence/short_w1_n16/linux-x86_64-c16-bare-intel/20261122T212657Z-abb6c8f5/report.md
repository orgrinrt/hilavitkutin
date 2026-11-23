# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 3.6x slower than the field

std_parallel (32.84 us) is 3.6x the fastest (9.18 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.64)

nway's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (9.18 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, sequential, kernel_seq, head_tail_dyn, kernel_nway} vs {nway, std_parallel} (170% apart)

The field splits into a fast tier {head_tail, sequential, kernel_seq, head_tail_dyn, kernel_nway} and a slow tier {nway, std_parallel} with a 170% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.6x the fastest

Fastest head_tail (9.18 us) to slowest std_parallel (32.84 us): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.8x its best-20%

sequential's best 20% of batches run at 6.74 us but its worst 20% at 25.56 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 9175.8 ns median
- 4 variants significantly slower than baseline
- Spread: 3.58x (fastest 9175.8 ns, slowest 32838.2 ns)

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
| Counter frequency | 2112789863 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 9983ns | 9718ns | 8137ns | 9303ns | 11927ns | base |
| head_tail_dyn | 12174ns | 12340ns | 8272ns | 11740ns | 14776ns | +21.95% |
| kernel_nway | 13412ns | 12533ns | 9990ns | 12100ns | 17092ns | +34.35% |
| kernel_seq | 11426ns | 11344ns | 9490ns | 10890ns | 13199ns | +14.45% |
| nway | 32547ns | 32025ns | 28955ns | 31479ns | 35945ns | +226.02% |
| sequential | 14841ns | 9901ns | 6878ns | 10001ns | 26083ns | +48.66% |
| std_parallel | 36541ns | 33348ns | 29587ns | 32223ns | 46494ns | +266.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 9257ns | 7679ns | 10777ns | base | 0.002 |
| head_tail_dyn | 11456ns | 7830ns | 13808ns | +23.76% | 0.001 |
| kernel_nway | 12589ns | 9500ns | 15893ns | +35.99% | 0.001 |
| kernel_seq | 10699ns | 8997ns | 12261ns | +15.57% | 0.001 |
| nway | 32102ns | 28567ns | 35432ns | +246.78% | 0.000 |
| sequential | 14545ns | 6744ns | 25557ns | +57.12% | 0.001 |
| std_parallel | 35854ns | 29109ns | 45408ns | +287.31% | 0.000 |

## Performance model

- Peak throughput: **0.002 Gops/s** (sequential; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.002 | 73.5% |
| head_tail_dyn | 0.001 | 58.5% |
| kernel_nway | 0.001 | 57.7% |
| kernel_seq | 0.002 | 64.1% |
| nway | 0.001 | 21.3% |
| sequential | 0.002 | 69.5% |
| std_parallel | 0.000 | 20.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 9983ns | 9983ns | base |
| head_tail_dyn | 12174ns | 12174ns | +21.95% |
| kernel_nway | 13412ns | 13412ns | +34.35% |
| kernel_seq | 11426ns | 11426ns | +14.45% |
| nway | 32547ns | 32547ns | +226.02% |
| sequential | 14841ns | 14841ns | +48.66% |
| std_parallel | 36541ns | 36541ns | +266.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 9176ns | base | --- | [8225, 9522] | --- | --- | --- | --- |
| head_tail_dyn | 11532ns | +3046.6ns (+33.2%) | [+104, +3644]ns | [10272, 12808] | YES (adj: no) | 0.1444 | 0.0963 | 0 |
| kernel_nway | 11684ns | +3244.0ns (+35.4%) | [+1040, +5956]ns | [10442, 14599] | YES | 0.0151 | 0.0075 | 0 |
| kernel_seq | 10526ns | no significant difference | [-227, +2678]ns | [9684, 11487] | no | 0.2855 | 0.2379 | 0 |
| nway | 31604ns | +21991.0ns (+239.7%) | [+21567, +23761]ns | [30167, 32947] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 9706ns | no significant difference | [-1222, +13574]ns | [6831, 22882] | no | 1.0000 | 1.0000 | 0 |
| std_parallel | 32838ns | +23652.5ns (+257.8%) | [+22070, +25607]ns | [30107, 35940] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 8440ns | +32.5% | +38.4% | +46.3% | +274.3% | +57.6% | +270.2% |
| 1 | 2 | 8725ns | +50.6% | +77.0% | +12.3% | +245.8% | +24.8% | +283.4% |
| 1 | 3 | 10269ns | -14.8% | +0.9% | -4.6% | +211.5% | -5.5% | +222.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.091 | ok |
| head_tail_dyn | -0.463 | moderate- |
| kernel_nway | -0.625 | HIGH- (thermal bounce) |
| kernel_seq | -0.166 | ok |
| nway | -0.638 | HIGH- (thermal bounce) |
| sequential | -0.025 | ok |
| std_parallel | -0.259 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 9861.8ns | 9257.2ns | 106.5% | HIGH |
| head_tail_dyn | 11700.7ns | 11456.5ns | 102.1% | HIGH |
| kernel_nway | 13163.9ns | 12588.7ns | 104.6% | HIGH |
| kernel_seq | 10978.8ns | 10698.8ns | 102.6% | HIGH |
| nway | 33686.6ns | 32102.1ns | 104.9% | HIGH |
| sequential | 14956.3ns | 14544.8ns | 102.8% | HIGH |
| std_parallel | 37766.6ns | 35854.3ns | 105.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 7679.1-10776.8 ns)
   7679.1 |#############
   7834.0 |#############
   7988.9 |#############
   8143.8 |##########################
   8298.6 |
   8453.5 |#############
   8608.4 |#############
   8763.3 |
   8918.2 |
   9073.1 |##########################
   9228.0 |########################################
   9382.8 |
   9537.7 |#############
   9692.6 |
   9847.5 |
  10002.4 |
  10157.3 |
  10312.2 |
  10467.0 |
  10621.9 |
  (1 below, 4 above range)

head_tail_dyn (n=18, range 7830.3-13808.1 ns)
   7830.3 |
   8129.2 |####################
   8428.1 |
   8726.9 |
   9025.8 |####################
   9324.7 |
   9623.6 |####################
   9922.5 |
  10221.4 |
  10520.3 |
  10819.2 |########################################
  11118.1 |########################################
  11417.0 |
  11715.9 |####################
  12014.8 |
  12313.7 |########################################
  12612.6 |########################################
  12911.5 |####################
  13210.3 |####################
  13509.2 |
  (2 below, 2 above range)

kernel_nway (n=18, range 9500.3-15892.8 ns)
   9500.3 |#############
   9819.9 |
  10139.5 |########################################
  10459.2 |########################################
  10778.8 |#############
  11098.4 |
  11418.0 |
  11737.7 |
  12057.3 |
  12376.9 |#############
  12696.5 |
  13016.1 |#############
  13335.8 |
  13655.4 |#############
  13975.0 |
  14294.6 |#############
  14614.3 |
  14933.9 |##########################
  15253.5 |
  15573.1 |#############
  (1 below, 2 above range)

kernel_seq (n=18, range 8997.3-12261.3 ns)
   8997.3 |########################################
   9160.5 |
   9323.7 |
   9486.9 |#############
   9650.1 |#############
   9813.3 |#############
   9976.5 |
  10139.7 |
  10302.9 |
  10466.1 |########################################
  10629.3 |
  10792.5 |
  10955.7 |##########################
  11118.9 |
  11282.1 |#############
  11445.3 |
  11608.5 |#############
  11771.7 |
  11934.9 |
  12098.1 |
  (1 below, 4 above range)

nway (n=18, range 28567.1-35431.6 ns)
  28567.1 |
  28910.3 |
  29253.5 |####################
  29596.7 |####################
  29940.0 |########################################
  30283.2 |
  30626.4 |####################
  30969.7 |########################################
  31312.9 |
  31656.1 |########################################
  31999.3 |####################
  32342.6 |
  32685.8 |####################
  33029.0 |
  33372.3 |####################
  33715.5 |####################
  34058.7 |
  34401.9 |
  34745.2 |
  35088.4 |
  (2 below, 3 above range)

sequential (n=18, range 6744.2-25557.3 ns)
   6744.2 |########################################
   7684.8 |
   8625.5 |
   9566.1 |
  10506.8 |
  11447.4 |
  12388.1 |#####
  13328.8 |
  14269.4 |#####
  15210.1 |
  16150.7 |
  17091.4 |
  18032.0 |
  18972.7 |#####
  19913.3 |
  20854.0 |
  21794.6 |
  22735.3 |#####
  23676.0 |#####
  24616.6 |##########
  (1 below, 2 above range)

std_parallel (n=18, range 29109.1-45408.3 ns)
  29109.1 |########################################
  29924.1 |########################################
  30739.0 |#############
  31554.0 |
  32368.9 |#############
  33183.9 |##########################
  33998.9 |#############
  34813.8 |#############
  35628.8 |
  36443.7 |#############
  37258.7 |
  38073.7 |#############
  38888.6 |
  39703.6 |
  40518.5 |
  41333.5 |
  42148.5 |
  42963.4 |
  43778.4 |
  44593.3 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=102.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.8% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=21.6% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=108.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=104.0% of algo (FFI overhead may distort results)
- **nway**: bridge=103.4% of algo (FFI overhead may distort results)
- **sequential**: CV=58.9% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **sequential**: bridge=105.5% of algo (FFI overhead may distort results)
- **std_parallel**: CV=25.0% (high variance, measurements may be unstable)
- **std_parallel**: bridge=107.8% of algo (FFI overhead may distort results)

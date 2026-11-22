# fold_heavy, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 4.1x slower than the field

std_parallel (23.36 us) is 4.1x the fastest (5.72 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.66)

kernel_nway's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (5.72 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, kernel_nway, kernel_seq, head_tail_dyn} vs {sequential, nway, std_parallel} (128% apart)

The field splits into a fast tier {head_tail, kernel_nway, kernel_seq, head_tail_dyn} and a slow tier {sequential, nway, std_parallel} with a 128% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.1x the fastest

Fastest head_tail (5.72 us) to slowest std_parallel (23.36 us): 4.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader head_tail vs stability leader head_tail_dyn (+3% speed for 1.4x steadier)

head_tail is fastest (5.72 us, CV 5.6%); head_tail_dyn gives up 3.5% median for 1.4x lower variance (CV 3.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (head_tail) is the fastest** at 5720.9 ns median
- 3 variants significantly slower than baseline
- Spread: 4.08x (fastest 5720.9 ns, slowest 23364.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 1 |
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
| head_tail | 6352ns | 6150ns | 6018ns | 6162ns | 6806ns | base |
| head_tail_dyn | 6400ns | 6385ns | 6053ns | 6318ns | 6697ns | +0.75% |
| kernel_nway | 6388ns | 6356ns | 6019ns | 6289ns | 6721ns | +0.57% |
| kernel_seq | 6357ns | 6300ns | 5911ns | 6225ns | 6780ns | +0.08% |
| nway | 16522ns | 16292ns | 15671ns | 16133ns | 17530ns | +160.09% |
| sequential | 13762ns | 13700ns | 12897ns | 13554ns | 14508ns | +116.65% |
| std_parallel | 23887ns | 23712ns | 21890ns | 23351ns | 25689ns | +276.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 5851ns | 5536ns | 6255ns | base | 0.700 |
| head_tail_dyn | 5913ns | 5615ns | 6169ns | +1.05% | 0.693 |
| kernel_nway | 5886ns | 5574ns | 6196ns | +0.60% | 0.696 |
| kernel_seq | 5885ns | 5456ns | 6295ns | +0.58% | 0.696 |
| nway | 16131ns | 15357ns | 17098ns | +175.67% | 0.254 |
| sequential | 13593ns | 12750ns | 14364ns | +132.30% | 0.301 |
| std_parallel | 23502ns | 21525ns | 25309ns | +301.65% | 0.174 |

## Performance model

- Peak throughput: **0.751 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.716 | 95.4% |
| head_tail_dyn | 0.692 | 92.2% |
| kernel_nway | 0.703 | 93.7% |
| kernel_seq | 0.700 | 93.3% |
| nway | 0.257 | 34.3% |
| sequential | 0.303 | 40.4% |
| std_parallel | 0.175 | 23.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 6352ns | 6352ns | base |
| head_tail_dyn | 6400ns | 6400ns | +0.75% |
| kernel_nway | 6388ns | 6388ns | +0.57% |
| kernel_seq | 6357ns | 6357ns | +0.08% |
| nway | 16522ns | 16522ns | +160.09% |
| sequential | 13762ns | 13762ns | +116.65% |
| std_parallel | 23887ns | 23887ns | +276.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 5721ns | base | --- | [5567, 6077] | --- | --- | --- | --- |
| head_tail_dyn | 5919ns | no significant difference | [-329, +327]ns | [5733, 5990] | no | 0.5768 | 0.4807 | 0 |
| kernel_nway | 5823ns | no significant difference | [-238, +315]ns | [5692, 6025] | no | 0.5768 | 0.4807 | 0 |
| kernel_seq | 5848ns | no significant difference | [-352, +371]ns | [5644, 6060] | no | 1.0000 | 1.0000 | 1 |
| nway | 15912ns | +9977.1ns (+174.4%) | [+9827, +10485]ns | [15515, 16383] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 13508ns | +7635.4ns (+133.5%) | [+7106, +8313]ns | [13067, 14250] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 23365ns | +17633.3ns (+308.2%) | [+16646, +18533]ns | [22388, 24290] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 5556ns | +7.8% | +6.7% | +1.6% | +180.5% | +156.5% | +312.0% |
| 1 | 2 | 6235ns | -4.7% | -9.3% | -4.5% | +155.9% | +111.1% | +282.8% |
| 1 | 3 | 5721ns | +0.1% | +3.1% | +1.3% | +189.8% | +126.4% | +312.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.631 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.079 | ok |
| kernel_nway | -0.658 | HIGH- (thermal bounce) |
| kernel_seq | -0.507 | HIGH- (thermal bounce) |
| nway | -0.013 | ok |
| sequential | -0.089 | ok |
| std_parallel | -0.337 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 90161.3ns | 5851.4ns | 1540.9% | HIGH |
| head_tail_dyn | 89466.4ns | 5913.0ns | 1513.1% | HIGH |
| kernel_nway | 90048.1ns | 5886.3ns | 1529.8% | HIGH |
| kernel_seq | 90081.0ns | 5885.2ns | 1530.6% | HIGH |
| nway | 93238.2ns | 16130.6ns | 578.0% | HIGH |
| sequential | 89137.7ns | 13592.6ns | 655.8% | HIGH |
| std_parallel | 98932.6ns | 23502.3ns | 420.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 5536.1-6254.9 ns)
   5536.1 |########################################
   5572.0 |##########
   5608.0 |##########
   5643.9 |
   5679.9 |####################
   5715.8 |##########
   5751.7 |##########
   5787.7 |
   5823.6 |
   5859.5 |
   5895.5 |
   5931.4 |
   5967.4 |##########
   6003.3 |
   6039.2 |##########
   6075.2 |##########
   6111.1 |
   6147.1 |
   6183.0 |##########
   6218.9 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 5615.3-6169.4 ns)
   5615.3 |
   5643.0 |
   5670.7 |
   5698.4 |##########################
   5726.1 |##########################
   5753.8 |#############
   5781.5 |
   5809.2 |
   5836.9 |#############
   5864.6 |
   5892.4 |#############
   5920.1 |##########################
   5947.8 |
   5975.5 |########################################
   6003.2 |
   6030.9 |
   6058.6 |#############
   6086.3 |
   6114.0 |
   6141.7 |
  (2 below, 3 above range)

kernel_nway (n=18, range 5573.6-6195.8 ns)
   5573.6 |##########################
   5604.7 |#############
   5635.8 |
   5666.9 |#############
   5698.0 |#############
   5729.2 |
   5760.3 |
   5791.4 |########################################
   5822.5 |#############
   5853.6 |#############
   5884.7 |#############
   5915.8 |
   5946.9 |
   5978.1 |
   6009.2 |##########################
   6040.3 |#############
   6071.4 |
   6102.5 |
   6133.6 |
   6164.7 |#############
  (1 below, 2 above range)

kernel_seq (n=18, range 5455.6-6295.1 ns)
   5455.6 |
   5497.5 |########################################
   5539.5 |
   5581.5 |####################
   5623.5 |
   5665.5 |########################################
   5707.4 |
   5749.4 |####################
   5791.4 |####################
   5833.4 |
   5875.4 |########################################
   5917.3 |####################
   5959.3 |
   6001.3 |####################
   6043.3 |
   6085.2 |########################################
   6127.2 |
   6169.2 |
   6211.2 |
   6253.2 |
  (2 below, 3 above range)

nway (n=18, range 15356.9-17097.9 ns)
  15356.9 |##########################
  15444.0 |########################################
  15531.0 |
  15618.1 |#############
  15705.1 |
  15792.2 |##########################
  15879.2 |##########################
  15966.3 |#############
  16053.3 |
  16140.4 |
  16227.4 |
  16314.5 |#############
  16401.5 |
  16488.6 |
  16575.6 |#############
  16662.7 |
  16749.7 |#############
  16836.8 |#############
  16923.8 |#############
  17010.9 |
  (1 below, 1 above range)

sequential (n=18, range 12750.0-14363.9 ns)
  12750.0 |####################
  12830.7 |
  12911.4 |####################
  12992.1 |####################
  13072.8 |########################################
  13153.5 |########################################
  13234.2 |
  13314.9 |
  13395.6 |
  13476.3 |
  13557.0 |
  13637.6 |
  13718.3 |
  13799.0 |########################################
  13879.7 |
  13960.4 |####################
  14041.1 |
  14121.8 |
  14202.5 |########################################
  14283.2 |
  (2 below, 4 above range)

std_parallel (n=18, range 21525.0-25309.0 ns)
  21525.0 |####################
  21714.2 |
  21903.4 |########################################
  22092.6 |####################
  22281.8 |
  22471.0 |####################
  22660.2 |####################
  22849.4 |####################
  23038.6 |####################
  23227.8 |
  23417.0 |####################
  23606.2 |########################################
  23795.4 |####################
  23984.6 |
  24173.8 |
  24363.0 |
  24552.2 |########################################
  24741.4 |
  24930.6 |####################
  25119.8 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=1557.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=1499.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1536.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=1527.3% of algo (FFI overhead may distort results)
- **nway**: bridge=585.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=663.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=421.0% of algo (FFI overhead may distort results)

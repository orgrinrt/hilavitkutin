# fold_heavy, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 25% faster than the next best (head_tail_dyn)

kernel_seq (5.58 us) leads head_tail_dyn (6.96 us) by 25%, a clear separation rather than a photo finish. CV 7.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 20% (significant)

kernel_seq is -1.42 us (20%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 15.3x slower than the field

std_parallel (85.58 us) is 15.3x the fastest (5.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.67)

kernel_nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 15.3x the fastest

Fastest kernel_seq (5.58 us) to slowest std_parallel (85.58 us): 15.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 5579.1 ns median (-20.6% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 15.34x (fastest 5579.1 ns, slowest 85583.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 8 |
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
| head_tail | 7660ns | 7581ns | 7003ns | 7599ns | 8081ns | base |
| head_tail_dyn | 7996ns | 7592ns | 7236ns | 7575ns | 9007ns | +4.38% |
| kernel_nway | 26607ns | 26506ns | 21143ns | 25748ns | 30629ns | +247.35% |
| kernel_seq | 6179ns | 6085ns | 5853ns | 6050ns | 6536ns | -19.33% |
| nway | 51821ns | 51244ns | 46640ns | 51055ns | 55561ns | +576.50% |
| sequential | 13330ns | 13290ns | 12836ns | 13105ns | 13916ns | +74.02% |
| std_parallel | 89240ns | 85944ns | 79854ns | 85426ns | 99654ns | +1064.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 7019ns | 6393ns | 7470ns | base | 0.584 |
| head_tail_dyn | 7365ns | 6706ns | 8339ns | +4.92% | 0.556 |
| kernel_nway | 25940ns | 20503ns | 29938ns | +269.54% | 0.158 |
| kernel_seq | 5704ns | 5475ns | 6023ns | -18.74% | 0.718 |
| nway | 51253ns | 46088ns | 54983ns | +630.16% | 0.080 |
| sequential | 13170ns | 12692ns | 13727ns | +87.62% | 0.311 |
| std_parallel | 88859ns | 79482ns | 99236ns | +1165.90% | 0.046 |

## Performance model

- Peak throughput: **0.748 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.583 | 77.9% |
| head_tail_dyn | 0.588 | 78.7% |
| kernel_nway | 0.158 | 21.2% |
| kernel_seq | 0.734 | 98.1% |
| nway | 0.081 | 10.8% |
| sequential | 0.312 | 41.7% |
| std_parallel | 0.048 | 6.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 7660ns | 7660ns | base |
| head_tail_dyn | 7996ns | 7996ns | +4.38% |
| kernel_nway | 26607ns | 26607ns | +247.35% |
| kernel_seq | 6179ns | 6179ns | -19.33% |
| nway | 51821ns | 51821ns | +576.50% |
| sequential | 13330ns | 13330ns | +74.02% |
| std_parallel | 89240ns | 89240ns | +1064.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 7025ns | base | --- | [6810, 7229] | --- | --- | --- | --- |
| head_tail_dyn | 6960ns | no significant difference | [-383, +369]ns | [6888, 7117] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 25865ns | +18935.4ns (+269.5%) | [+16713, +20302]ns | [23717, 27431] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 5579ns | -1416.7ns (-20.2%) | [-1602, -958]ns | [5535, 5646] | YES | 0.0002 | 0.0001 | 0 |
| nway | 50696ns | +43979.2ns (+626.0%) | [+42531, +45396]ns | [49746, 52429] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 13127ns | +6279.2ns (+89.4%) | [+5725, +6477]ns | [12740, 13446] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 85583ns | +78693.8ns (+1120.2%) | [+76090, +86798]ns | [82792, 94060] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 7204ns | -3.7% | +246.8% | -22.1% | +594.8% | +88.3% | +1108.0% |
| 1 | 2 | 6844ns | +6.8% | +288.8% | -18.9% | +649.9% | +86.8% | +1188.2% |
| 1 | 3 | 7081ns | -2.8% | +252.2% | -20.9% | +623.9% | +82.1% | +1076.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.592 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.657 | HIGH- (thermal bounce) |
| kernel_nway | -0.666 | HIGH- (thermal bounce) |
| kernel_seq | -0.656 | HIGH- (thermal bounce) |
| nway | -0.193 | ok |
| sequential | -0.248 | moderate- |
| std_parallel | -0.309 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 87400.9ns | 7019.4ns | 1245.1% | HIGH |
| head_tail_dyn | 87810.4ns | 7365.0ns | 1192.3% | HIGH |
| kernel_nway | 121456.2ns | 25939.8ns | 468.2% | HIGH |
| kernel_seq | 89359.0ns | 5703.9ns | 1566.6% | HIGH |
| nway | 106481.7ns | 51253.5ns | 207.8% | HIGH |
| sequential | 89297.9ns | 13170.1ns | 678.0% | HIGH |
| std_parallel | 133133.1ns | 88859.5ns | 149.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 6393.0-7470.1 ns)
   6393.0 |####################
   6446.9 |####################
   6500.7 |####################
   6554.6 |
   6608.5 |
   6662.3 |
   6716.2 |
   6770.0 |########################################
   6823.9 |
   6877.7 |####################
   6931.6 |####################
   6985.4 |########################################
   7039.3 |
   7093.2 |####################
   7147.0 |####################
   7200.9 |########################################
   7254.7 |
   7308.6 |
   7362.4 |
   7416.3 |####################
  (1 below, 3 above range)

head_tail_dyn (n=18, range 6705.5-8338.9 ns)
   6705.5 |
   6787.2 |##############################
   6868.9 |########################################
   6950.5 |##############################
   7032.2 |##########
   7113.9 |##########
   7195.5 |##########
   7277.2 |
   7358.9 |##########
   7440.5 |
   7522.2 |##########
   7603.9 |
   7685.5 |
   7767.2 |
   7848.9 |
   7930.5 |
   8012.2 |
   8093.9 |
   8175.5 |
   8257.2 |
  (2 below, 1 above range)

kernel_nway (n=18, range 20502.8-29937.5 ns)
  20502.8 |
  20974.5 |
  21446.2 |#############
  21918.0 |#############
  22389.7 |#############
  22861.5 |#############
  23333.2 |
  23804.9 |
  24276.7 |##########################
  24748.4 |
  25220.1 |##########################
  25691.9 |
  26163.6 |#############
  26635.3 |#############
  27107.1 |########################################
  27578.8 |
  28050.6 |#############
  28522.3 |
  28994.0 |#############
  29465.8 |
  (1 below, 2 above range)

kernel_seq (n=18, range 5475.0-6022.9 ns)
   5475.0 |
   5502.4 |##########################
   5529.8 |########################################
   5557.2 |########################################
   5584.6 |
   5612.0 |##########################
   5639.4 |##########################
   5666.8 |
   5694.2 |
   5721.6 |
   5749.0 |
   5776.4 |
   5803.8 |##########################
   5831.1 |#############
   5858.5 |
   5885.9 |
   5913.3 |
   5940.7 |
   5968.1 |
   5995.5 |
  (2 below, 1 above range)

nway (n=18, range 46087.5-54983.3 ns)
  46087.5 |
  46532.3 |#############
  46977.1 |
  47421.9 |
  47866.7 |
  48311.5 |
  48756.2 |#############
  49201.0 |#############
  49645.8 |#############
  50090.6 |########################################
  50535.4 |#############
  50980.2 |
  51425.0 |#############
  51869.8 |#############
  52314.6 |##########################
  52759.4 |
  53204.2 |#############
  53649.0 |
  54093.8 |
  54538.5 |
  (2 below, 3 above range)

sequential (n=18, range 12691.7-13727.1 ns)
  12691.7 |########################################
  12743.4 |########
  12795.2 |########
  12847.0 |
  12898.8 |
  12950.5 |
  13002.3 |
  13054.1 |########
  13105.8 |
  13157.6 |########
  13209.4 |
  13261.1 |################
  13312.9 |
  13364.7 |
  13416.5 |########
  13468.2 |################
  13520.0 |
  13571.8 |
  13623.5 |########
  13675.3 |
  (1 below, 2 above range)

std_parallel (n=18, range 79481.9-99236.1 ns)
  79481.9 |####################
  80469.6 |####################
  81457.4 |########################################
  82445.1 |########################################
  83432.8 |####################
  84420.5 |####################
  85408.2 |
  86395.9 |####################
  87383.6 |
  88371.3 |
  89359.0 |########################################
  90346.7 |
  91334.4 |
  92322.2 |####################
  93309.9 |
  94297.6 |
  95285.3 |####################
  96273.0 |
  97260.7 |
  98248.4 |####################
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=1236.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=20.9% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=1246.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=472.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=1593.6% of algo (FFI overhead may distort results)
- **nway**: bridge=213.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=680.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=157.1% of algo (FFI overhead may distort results)

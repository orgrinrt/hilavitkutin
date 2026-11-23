# fold_light, short fiber (1 to 4 morsels), W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 4.3x slower than the field

std_parallel (47.41 us) is 4.3x the fastest (11.10 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 72.0%)

sequential wins on median (11.10 us) yet has the highest variance (CV 72.0%), while head_tail is the steadiest (CV 11.2%, 13.20 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### sequential shows alternating (throttle bounce) (autocorr -0.58)

sequential's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential, head_tail_dyn, kernel_seq, kernel_nway, head_tail} vs {nway, std_parallel} (153% apart)

The field splits into a fast tier {sequential, head_tail_dyn, kernel_seq, kernel_nway, head_tail} and a slow tier {nway, std_parallel} with a 153% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.3x the fastest

Fastest sequential (11.10 us) to slowest std_parallel (47.41 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.5x its best-20%

sequential's best 20% of batches run at 6.84 us but its worst 20% at 23.68 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 11098.8 ns median (-15.9% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 4.27x (fastest 11098.8 ns, slowest 47411.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 2 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112817100 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 14139ns | 13802ns | 12171ns | 13481ns | 16111ns | base |
| head_tail_dyn | 12660ns | 12133ns | 11384ns | 12052ns | 14211ns | -10.46% |
| kernel_nway | 12705ns | 12578ns | 10465ns | 12262ns | 14491ns | -10.14% |
| kernel_seq | 12853ns | 12686ns | 9493ns | 11950ns | 15886ns | -9.10% |
| nway | 35314ns | 33920ns | 30791ns | 33450ns | 40371ns | +149.76% |
| sequential | 14512ns | 11317ns | 6978ns | 10578ns | 24180ns | +2.63% |
| std_parallel | 46561ns | 47885ns | 37254ns | 46075ns | 51944ns | +229.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 13377ns | 11595ns | 15147ns | base | 0.077 |
| head_tail_dyn | 11983ns | 10782ns | 13362ns | -10.42% | 0.085 |
| kernel_nway | 11957ns | 9852ns | 13605ns | -10.62% | 0.086 |
| kernel_seq | 12018ns | 8819ns | 14891ns | -10.16% | 0.085 |
| nway | 34815ns | 30384ns | 39763ns | +160.27% | 0.029 |
| sequential | 14220ns | 6841ns | 23685ns | +6.30% | 0.072 |
| std_parallel | 45989ns | 36887ns | 51159ns | +243.80% | 0.022 |

## Performance model

- Peak throughput: **0.150 Gops/s** (sequential; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.078 | 51.8% |
| head_tail_dyn | 0.088 | 59.1% |
| kernel_nway | 0.086 | 57.3% |
| kernel_seq | 0.087 | 58.1% |
| nway | 0.031 | 20.5% |
| sequential | 0.092 | 61.6% |
| std_parallel | 0.022 | 14.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 14139ns | 14139ns | base |
| head_tail_dyn | 12660ns | 12660ns | -10.46% |
| kernel_nway | 12705ns | 12705ns | -10.14% |
| kernel_seq | 12853ns | 12853ns | -9.10% |
| nway | 35314ns | 35314ns | +149.76% |
| sequential | 14512ns | 14512ns | +2.63% |
| std_parallel | 46561ns | 46561ns | +229.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 13198ns | base | --- | [11962, 14135] | --- | --- | --- | --- |
| head_tail_dyn | 11577ns | -1235.8ns (-9.4%) | [-2501, -532]ns | [11356, 12274] | YES | 0.0020 | 0.0013 | 0 |
| kernel_nway | 11944ns | -1235.0ns (-9.4%) | [-1862, -787]ns | [10895, 12847] | YES | 0.0003 | 0.0001 | 0 |
| kernel_seq | 11776ns | -1422.5ns (-10.8%) | [-1931, -245]ns | [10139, 14014] | YES (adj: no) | 0.1155 | 0.0963 | 0 |
| nway | 33429ns | +20463.0ns (+155.1%) | [+19071, +21759]ns | [32222, 36051] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 11099ns | no significant difference | [-5593, +6104]ns | [6963, 20244] | no | 1.0000 | 1.0000 | 0 |
| std_parallel | 47411ns | +33896.3ns (+256.8%) | [+29756, +36101]ns | [43149, 49423] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 14647ns | -16.2% | -12.3% | -2.9% | +146.1% | +18.6% | +231.7% |
| 1 | 2 | 12595ns | -8.7% | -9.7% | -6.5% | +157.1% | -44.7% | +274.3% |
| 1 | 3 | 12228ns | -8.4% | -16.0% | -24.2% | +163.5% | +11.0% | +277.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.093 | ok |
| head_tail_dyn | -0.043 | ok |
| kernel_nway | -0.005 | ok |
| kernel_seq | -0.000 | ok |
| nway | -0.146 | ok |
| sequential | -0.580 | HIGH- (thermal bounce) |
| std_parallel | -0.010 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 13748.4ns | 13376.8ns | 102.8% | HIGH |
| head_tail_dyn | 12174.0ns | 11983.5ns | 101.6% | HIGH |
| kernel_nway | 12187.3ns | 11956.8ns | 101.9% | HIGH |
| kernel_seq | 12409.4ns | 12017.9ns | 103.3% | HIGH |
| nway | 35480.3ns | 34815.3ns | 101.9% | HIGH |
| sequential | 14754.2ns | 14219.7ns | 103.8% | HIGH |
| std_parallel | 48004.9ns | 45988.9ns | 104.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 11595.1-15147.3 ns)
  11595.1 |#############
  11772.7 |
  11950.3 |########################################
  12128.0 |
  12305.6 |
  12483.2 |#############
  12660.8 |
  12838.4 |
  13016.0 |##########################
  13193.6 |#############
  13371.2 |#############
  13548.8 |
  13726.4 |#############
  13904.0 |#############
  14081.6 |
  14259.2 |#############
  14436.8 |
  14614.4 |
  14792.1 |
  14969.7 |
  (2 below, 4 above range)

head_tail_dyn (n=18, range 10782.4-13362.3 ns)
  10782.4 |#############
  10911.4 |##########################
  11040.4 |
  11169.4 |
  11298.4 |##########################
  11427.4 |########################################
  11556.4 |##########################
  11685.4 |#############
  11814.4 |
  11943.4 |#############
  12072.4 |
  12201.3 |
  12330.3 |
  12459.3 |##########################
  12588.3 |#############
  12717.3 |
  12846.3 |#############
  12975.3 |
  13104.3 |
  13233.3 |
  (1 below, 1 above range)

kernel_nway (n=18, range 9851.9-13605.1 ns)
   9851.9 |
  10039.5 |####################
  10227.2 |####################
  10414.9 |
  10602.5 |
  10790.2 |####################
  10977.8 |####################
  11165.5 |########################################
  11353.2 |####################
  11540.8 |
  11728.5 |
  11916.1 |
  12103.8 |
  12291.5 |####################
  12479.1 |####################
  12666.8 |########################################
  12854.5 |####################
  13042.1 |####################
  13229.8 |
  13417.4 |####################
  (2 below, 2 above range)

kernel_seq (n=18, range 8818.7-14891.0 ns)
   8818.7 |##########################
   9122.3 |
   9425.9 |#############
   9729.5 |#############
  10033.2 |
  10336.8 |##########################
  10640.4 |#############
  10944.0 |
  11247.6 |#############
  11551.2 |
  11854.9 |
  12158.5 |#############
  12462.1 |#############
  12765.7 |#############
  13069.3 |
  13373.0 |
  13676.6 |#############
  13980.2 |########################################
  14283.8 |
  14587.4 |
  (1 below, 2 above range)

nway (n=18, range 30383.5-39763.0 ns)
  30383.5 |########################################
  30852.5 |
  31321.4 |
  31790.4 |#############
  32259.4 |########################################
  32728.4 |
  33197.3 |##########################
  33666.3 |
  34135.3 |
  34604.3 |#############
  35073.2 |
  35542.2 |##########################
  36011.2 |#############
  36480.2 |#############
  36949.1 |
  37418.1 |
  37887.1 |
  38356.1 |
  38825.0 |
  39294.0 |
  (1 below, 3 above range)

sequential (n=18, range 6841.5-23684.5 ns)
   6841.5 |########################################
   7683.6 |
   8525.8 |
   9367.9 |
  10210.1 |
  11052.2 |
  11894.4 |
  12736.5 |
  13578.7 |
  14420.8 |#####
  15263.0 |
  16105.2 |#####
  16947.3 |
  17789.5 |
  18631.6 |
  19473.8 |##########
  20315.9 |#####
  21158.1 |#####
  22000.2 |
  22842.4 |
  (1 below, 3 above range)

std_parallel (n=18, range 36887.4-51159.0 ns)
  36887.4 |
  37601.0 |
  38314.5 |
  39028.1 |
  39741.7 |##########################
  40455.3 |
  41168.9 |#############
  41882.5 |
  42596.0 |
  43309.6 |
  44023.2 |
  44736.8 |#############
  45450.4 |##########################
  46164.0 |#############
  46877.5 |
  47591.1 |
  48304.7 |########################################
  49018.3 |########################################
  49731.9 |
  50445.5 |#############
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=102.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=102.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=103.4% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=20.1% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=102.0% of algo (FFI overhead may distort results)
- **nway**: bridge=102.3% of algo (FFI overhead may distort results)
- **sequential**: CV=56.2% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **sequential**: bridge=106.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=104.3% of algo (FFI overhead may distort results)

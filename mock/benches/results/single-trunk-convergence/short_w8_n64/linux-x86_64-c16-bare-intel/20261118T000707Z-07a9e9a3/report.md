# fold_light, short fiber (1 to 4 morsels), W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 6.2x slower than the field

std_parallel (67.06 us) is 6.2x the fastest (10.77 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.67)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (10.77 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, kernel_seq, head_tail_dyn, sequential, kernel_nway} vs {nway, std_parallel} (172% apart)

The field splits into a fast tier {head_tail, kernel_seq, head_tail_dyn, sequential, kernel_nway} and a slow tier {nway, std_parallel} with a 172% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.2x the fastest

Fastest head_tail (10.77 us) to slowest std_parallel (67.06 us): 6.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.6x its best-20%

sequential's best 20% of batches run at 6.73 us but its worst 20% at 24.06 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 10773.8 ns median
- 4 variants significantly slower than baseline
- Spread: 6.22x (fastest 10773.8 ns, slowest 67057.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 8 |
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
| head_tail | 10732ns | 11277ns | 8318ns | 10467ns | 12337ns | base |
| head_tail_dyn | 13384ns | 12304ns | 11016ns | 12379ns | 16075ns | +24.71% |
| kernel_nway | 18853ns | 18183ns | 15028ns | 17750ns | 22421ns | +75.67% |
| kernel_seq | 12182ns | 12181ns | 8119ns | 11152ns | 15759ns | +13.51% |
| nway | 49117ns | 48068ns | 42721ns | 46087ns | 56859ns | +357.66% |
| sequential | 15190ns | 14733ns | 6868ns | 11718ns | 24559ns | +41.54% |
| std_parallel | 88006ns | 67371ns | 62924ns | 67004ns | 132050ns | +720.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 10116ns | 7887ns | 11558ns | base | 0.006 |
| head_tail_dyn | 12692ns | 10513ns | 15323ns | +25.47% | 0.005 |
| kernel_nway | 18008ns | 14470ns | 21240ns | +78.02% | 0.004 |
| kernel_seq | 11367ns | 7609ns | 14764ns | +12.37% | 0.006 |
| nway | 48536ns | 42214ns | 56270ns | +379.81% | 0.001 |
| sequential | 14890ns | 6734ns | 24064ns | +47.20% | 0.004 |
| std_parallel | 87598ns | 62644ns | 131352ns | +765.96% | 0.001 |

## Performance model

- Peak throughput: **0.010 Gops/s** (sequential; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.006 | 62.5% |
| head_tail_dyn | 0.006 | 58.9% |
| kernel_nway | 0.004 | 38.6% |
| kernel_seq | 0.006 | 60.6% |
| nway | 0.001 | 14.2% |
| sequential | 0.004 | 46.6% |
| std_parallel | 0.001 | 10.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 10732ns | 10732ns | base |
| head_tail_dyn | 13384ns | 13384ns | +24.71% |
| kernel_nway | 18853ns | 18853ns | +75.67% |
| kernel_seq | 12182ns | 12182ns | +13.51% |
| nway | 49117ns | 49117ns | +357.66% |
| sequential | 15190ns | 15190ns | +41.54% |
| std_parallel | 88006ns | 88006ns | +720.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 10774ns | base | --- | [8363, 10978] | --- | --- | --- | --- |
| head_tail_dyn | 11427ns | +449.2ns (+4.2%) | [+191, +6080]ns | [11218, 14080] | YES | 0.0463 | 0.0309 | 0 |
| kernel_nway | 17446ns | +7876.0ns (+73.1%) | [+6904, +9077]ns | [15809, 20396] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 11120ns | no significant difference | [-2379, +5100]ns | [8955, 13403] | no | 1.0000 | 1.0000 | 0 |
| nway | 47374ns | +36405.6ns (+337.9%) | [+35599, +40088]ns | [43493, 50614] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 14443ns | no significant difference | [-3231, +12292]ns | [6910, 21976] | no | 0.9774 | 0.8145 | 0 |
| std_parallel | 67057ns | +58694.0ns (+544.8%) | [+55604, +69020]ns | [65180, 79793] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 11251ns | -1.5% | +55.1% | -20.4% | +385.6% | -1.9% | +832.8% |
| 1 | 2 | 8044ns | +90.4% | +95.4% | +76.1% | +442.1% | +105.4% | +720.7% |
| 1 | 3 | 10931ns | +3.4% | +88.2% | -10.2% | +338.0% | +26.3% | +496.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.661 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.665 | HIGH- (thermal bounce) |
| kernel_nway | -0.397 | moderate- |
| kernel_seq | -0.651 | HIGH- (thermal bounce) |
| nway | -0.420 | moderate- |
| sequential | -0.497 | moderate- |
| std_parallel | -0.156 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 10358.6ns | 10115.7ns | 102.4% | HIGH |
| head_tail_dyn | 12994.2ns | 12692.2ns | 102.4% | HIGH |
| kernel_nway | 18938.7ns | 18007.6ns | 105.2% | HIGH |
| kernel_seq | 11759.5ns | 11367.2ns | 103.5% | HIGH |
| nway | 49090.9ns | 48535.9ns | 101.1% | HIGH |
| sequential | 15226.8ns | 14890.3ns | 102.3% | HIGH |
| std_parallel | 91474.9ns | 87598.2ns | 104.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 7887.3-11558.4 ns)
   7887.3 |##########################
   8070.8 |#############
   8254.4 |##########################
   8437.9 |
   8621.5 |
   8805.1 |
   8988.6 |
   9172.2 |
   9355.7 |
   9539.3 |
   9722.9 |
   9906.4 |
  10090.0 |
  10273.5 |#############
  10457.1 |#############
  10640.6 |##########################
  10824.2 |########################################
  11007.8 |#############
  11191.3 |
  11374.9 |#############
  (1 below, 3 above range)

head_tail_dyn (n=18, range 10513.2-15322.8 ns)
  10513.2 |#############
  10753.6 |#############
  10994.1 |########################################
  11234.6 |########################################
  11475.1 |#############
  11715.6 |
  11956.1 |#############
  12196.5 |
  12437.0 |
  12677.5 |
  12918.0 |
  13158.5 |
  13398.9 |
  13639.4 |
  13879.9 |##########################
  14120.4 |#############
  14360.9 |#############
  14601.4 |
  14841.8 |
  15082.3 |
  (1 below, 3 above range)

kernel_nway (n=18, range 14470.1-21240.2 ns)
  14470.1 |########################################
  14808.6 |
  15147.1 |####################
  15485.6 |####################
  15824.1 |########################################
  16162.6 |
  16501.1 |####################
  16839.6 |
  17178.1 |####################
  17516.6 |####################
  17855.1 |
  18193.6 |
  18532.1 |
  18870.6 |####################
  19209.1 |
  19547.7 |####################
  19886.2 |
  20224.7 |########################################
  20563.2 |####################
  20901.7 |
  (1 below, 3 above range)

kernel_seq (n=18, range 7608.8-14763.6 ns)
   7608.8 |########################################
   7966.5 |####################
   8324.2 |####################
   8682.0 |
   9039.7 |########################################
   9397.5 |
   9755.2 |
  10113.0 |####################
  10470.7 |####################
  10828.4 |
  11186.2 |
  11543.9 |####################
  11901.7 |
  12259.4 |####################
  12617.1 |####################
  12974.9 |
  13332.6 |########################################
  13690.4 |
  14048.1 |####################
  14405.9 |
  (1 below, 3 above range)

nway (n=18, range 42214.0-56269.5 ns)
  42214.0 |##########
  42916.7 |########################################
  43619.5 |####################
  44322.3 |
  45025.1 |
  45727.8 |
  46430.6 |
  47133.4 |####################
  47836.2 |##########
  48539.0 |####################
  49241.7 |
  49944.5 |
  50647.3 |
  51350.1 |
  52052.8 |##########
  52755.6 |
  53458.4 |
  54161.2 |##########
  54863.9 |##########
  55566.7 |
  (1 below, 2 above range)

sequential (n=18, range 6734.2-24064.0 ns)
   6734.2 |########################################
   7600.7 |
   8467.2 |
   9333.7 |
  10200.2 |
  11066.7 |
  11933.2 |
  12799.6 |
  13666.1 |######
  14532.6 |######
  15399.1 |
  16265.6 |
  17132.1 |
  17998.6 |
  18865.0 |######
  19731.5 |######
  20598.0 |
  21464.5 |######
  22331.0 |
  23197.5 |####################
  (2 below, 2 above range)

std_parallel (n=18, range 62644.2-131351.9 ns)
  62644.2 |########################################
  66079.6 |################################
  69514.9 |########
  72950.3 |
  76385.7 |########
  79821.1 |########
  83256.5 |########
  86691.9 |
  90127.3 |
  93562.7 |
  96998.0 |
  100433.4 |
  103868.8 |
  107304.2 |
  110739.6 |
  114175.0 |
  117610.4 |
  121045.8 |########
  124481.1 |
  127916.5 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=100.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=105.6% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=105.3% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=25.7% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=96.6% of algo (FFI overhead may distort results)
- **nway**: bridge=102.8% of algo (FFI overhead may distort results)
- **sequential**: CV=52.3% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **sequential**: bridge=105.5% of algo (FFI overhead may distort results)
- **std_parallel**: CV=55.2% (high variance, measurements may be unstable)
- **std_parallel**: bridge=98.8% of algo (FFI overhead may distort results)

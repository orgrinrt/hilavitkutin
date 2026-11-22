# append, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway is an outlier: 6.7x slower than the field

nway (64.67 us) is 6.7x the fastest (9.63 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (head_tail, kernel_seq) are a dead heat (<1%)

head_tail (9.63 us) and kernel_seq (9.65 us) differ by 0.24%, inside the noise, even though the wider field spreads 571.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail shows alternating (throttle bounce) (autocorr -0.66)

head_tail's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (9.63 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, kernel_seq, head_tail_dyn, kernel_nway} vs {std_parallel, sequential, nway} (250% apart)

The field splits into a fast tier {head_tail, kernel_seq, head_tail_dyn, kernel_nway} and a slow tier {std_parallel, sequential, nway} with a 250% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.7x the fastest

Fastest head_tail (9.63 us) to slowest nway (64.67 us): 6.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (head_tail) is the fastest** at 9631.2 ns median
- 4 variants significantly slower than baseline
- Spread: 6.71x (fastest 9631.2 ns, slowest 64672.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 2 |
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
| head_tail | 10253ns | 10144ns | 9904ns | 10123ns | 10623ns | base |
| head_tail_dyn | 10597ns | 10317ns | 10032ns | 10289ns | 11342ns | +3.35% |
| kernel_nway | 11051ns | 10910ns | 10422ns | 10859ns | 11653ns | +7.78% |
| kernel_seq | 10453ns | 10369ns | 10050ns | 10354ns | 10803ns | +1.95% |
| nway | 63981ns | 65340ns | 59017ns | 64424ns | 65797ns | +524.00% |
| sequential | 56695ns | 56431ns | 55068ns | 56247ns | 58179ns | +452.94% |
| std_parallel | 37831ns | 36065ns | 33488ns | 35740ns | 43138ns | +268.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 9742ns | 9503ns | 10050ns | base | 6.727 |
| head_tail_dyn | 10029ns | 9507ns | 10767ns | +2.95% | 6.535 |
| kernel_nway | 10273ns | 9564ns | 10880ns | +5.45% | 6.380 |
| kernel_seq | 9908ns | 9535ns | 10377ns | +1.71% | 6.614 |
| nway | 63343ns | 58453ns | 65120ns | +550.21% | 1.035 |
| sequential | 56270ns | 54708ns | 57716ns | +477.60% | 1.165 |
| std_parallel | 36744ns | 31890ns | 42445ns | +277.18% | 1.784 |

## Performance model

- Peak throughput: **6.897 Gops/s** (head_tail; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 6.805 | 98.7% |
| head_tail_dyn | 6.777 | 98.3% |
| kernel_nway | 6.477 | 93.9% |
| kernel_seq | 6.788 | 98.4% |
| nway | 1.013 | 14.7% |
| sequential | 1.172 | 17.0% |
| std_parallel | 1.851 | 26.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 10253ns | 10253ns | base |
| head_tail_dyn | 10597ns | 10597ns | +3.35% |
| kernel_nway | 11051ns | 11051ns | +7.78% |
| kernel_seq | 10453ns | 10453ns | +1.95% |
| nway | 63981ns | 63981ns | +524.00% |
| sequential | 56695ns | 56695ns | +452.94% |
| std_parallel | 37831ns | 37831ns | +268.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 9631ns | base | --- | [9581, 9735] | --- | --- | --- | --- |
| head_tail_dyn | 9671ns | no significant difference | [-40, +635]ns | [9596, 10517] | no | 0.2855 | 0.2379 | 0 |
| kernel_nway | 10119ns | +556.2ns (+5.8%) | [+50, +721]ns | [9992, 10402] | YES (adj: no) | 0.1444 | 0.0963 | 0 |
| kernel_seq | 9654ns | no significant difference | [-146, +496]ns | [9602, 10206] | no | 0.4807 | 0.4807 | 0 |
| nway | 64673ns | +54697.9ns (+567.9%) | [+53519, +55217]ns | [63171, 64967] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 55923ns | +46295.8ns (+480.7%) | [+45975, +47090]ns | [55502, 57006] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 35410ns | +25450.0ns (+264.2%) | [+23200, +28754]ns | [32704, 38425] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 9588ns | -0.3% | +8.4% | -0.1% | +562.3% | +477.8% | +300.8% |
| 1 | 2 | 9917ns | +6.2% | +1.3% | -2.8% | +541.6% | +482.3% | +263.3% |
| 1 | 3 | 9612ns | +1.3% | +5.9% | +6.9% | +577.0% | +481.2% | +235.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.664 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.647 | HIGH- (thermal bounce) |
| kernel_nway | -0.418 | moderate- |
| kernel_seq | -0.124 | ok |
| nway | -0.127 | ok |
| sequential | -0.643 | HIGH- (thermal bounce) |
| std_parallel | -0.010 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 441092.1ns | 9741.9ns | 4527.8% | HIGH |
| head_tail_dyn | 445371.5ns | 10028.9ns | 4440.9% | HIGH |
| kernel_nway | 440484.7ns | 10272.7ns | 4287.9% | HIGH |
| kernel_seq | 444419.7ns | 9908.1ns | 4485.4% | HIGH |
| nway | 481091.7ns | 63343.3ns | 759.5% | HIGH |
| sequential | 467447.0ns | 56269.7ns | 830.7% | HIGH |
| std_parallel | 459604.4ns | 36744.4ns | 1250.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 9502.8-10050.0 ns)
   9502.8 |########################################
   9530.1 |####################
   9557.5 |########################################
   9584.9 |########################################
   9612.2 |####################
   9639.6 |########################################
   9666.9 |
   9694.3 |########################################
   9721.7 |
   9749.0 |####################
   9776.4 |
   9803.7 |####################
   9831.1 |
   9858.5 |
   9885.8 |
   9913.2 |
   9940.6 |
   9967.9 |
   9995.3 |
  10022.6 |####################
  (1 below, 2 above range)

head_tail_dyn (n=18, range 9506.9-10767.4 ns)
   9506.9 |##########
   9570.0 |########################################
   9633.0 |##############################
   9696.0 |
   9759.0 |##########
   9822.0 |
   9885.1 |
   9948.1 |
  10011.1 |
  10074.1 |
  10137.1 |
  10200.2 |##########
  10263.2 |
  10326.2 |
  10389.2 |##########
  10452.3 |
  10515.3 |
  10578.3 |##########
  10641.3 |##########
  10704.3 |##########
  (2 below, 2 above range)

kernel_nway (n=18, range 9563.9-10879.9 ns)
   9563.9 |
   9629.7 |
   9695.5 |
   9761.3 |####################
   9827.1 |
   9892.9 |####################
   9958.7 |########################################
  10024.5 |########################################
  10090.3 |########################################
  10156.1 |
  10221.9 |####################
  10287.7 |########################################
  10353.5 |
  10419.3 |####################
  10485.1 |####################
  10550.9 |
  10616.7 |
  10682.5 |
  10748.3 |
  10814.1 |
  (2 below, 3 above range)

kernel_seq (n=18, range 9534.7-10377.1 ns)
   9534.7 |##########
   9576.8 |####################
   9618.9 |########################################
   9661.1 |##########
   9703.2 |
   9745.3 |
   9787.4 |
   9829.5 |
   9871.6 |
   9913.8 |
   9955.9 |
   9998.0 |##########
  10040.1 |
  10082.2 |##########
  10124.4 |
  10166.5 |##########
  10208.6 |##########
  10250.7 |####################
  10292.8 |
  10334.9 |
  (2 below, 2 above range)

nway (n=18, range 58452.8-65120.2 ns)
  58452.8 |
  58786.1 |
  59119.5 |
  59452.9 |########
  59786.2 |
  60119.6 |########
  60453.0 |########
  60786.4 |
  61119.7 |
  61453.1 |
  61786.5 |
  62119.8 |
  62453.2 |
  62786.6 |########
  63119.9 |########
  63453.3 |
  63786.7 |################
  64120.0 |
  64453.4 |################
  64786.8 |########################################
  (1 below, 3 above range)

sequential (n=18, range 54708.3-57716.0 ns)
  54708.3 |
  54858.7 |####################
  55009.1 |
  55159.5 |####################
  55309.9 |
  55460.2 |########################################
  55610.6 |####################
  55761.0 |########################################
  55911.4 |####################
  56061.8 |
  56212.2 |####################
  56362.5 |####################
  56512.9 |####################
  56663.3 |
  56813.7 |
  56964.1 |
  57114.4 |
  57264.8 |####################
  57415.2 |
  57565.6 |####################
  (2 below, 3 above range)

std_parallel (n=18, range 31890.3-42445.1 ns)
  31890.3 |########################################
  32418.0 |##########################
  32945.8 |
  33473.5 |
  34001.2 |#############
  34529.0 |#############
  35056.7 |##########################
  35584.5 |
  36112.2 |#############
  36640.0 |#############
  37167.7 |
  37695.4 |#############
  38223.2 |#############
  38750.9 |#############
  39278.7 |
  39806.4 |
  40334.2 |
  40861.9 |
  41389.6 |
  41917.4 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=4578.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=4571.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=4337.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=4576.6% of algo (FFI overhead may distort results)
- **nway**: bridge=737.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=829.6% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=1284.8% of algo (FFI overhead may distort results)

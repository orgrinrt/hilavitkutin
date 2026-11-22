# append, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway is an outlier: 6.2x slower than the field

nway (60.70 us) is 6.2x the fastest (9.82 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.63)

nway's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 6.2x the fastest

Fastest kernel_seq (9.82 us) to slowest nway (60.70 us): 6.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 9818.8 ns median (-1.3% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 6.18x (fastest 9818.8 ns, slowest 60695.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 1 |
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
| head_tail | 10574ns | 10500ns | 10131ns | 10446ns | 10989ns | base |
| head_tail_dyn | 10999ns | 10848ns | 10029ns | 10585ns | 12105ns | +4.02% |
| kernel_nway | 10626ns | 10446ns | 9942ns | 10357ns | 11370ns | +0.49% |
| kernel_seq | 10624ns | 10450ns | 10012ns | 10361ns | 11325ns | +0.47% |
| nway | 61690ns | 61329ns | 59682ns | 60847ns | 63958ns | +483.39% |
| sequential | 57447ns | 56535ns | 55950ns | 56530ns | 59570ns | +443.27% |
| std_parallel | 25494ns | 25408ns | 24414ns | 25257ns | 26391ns | +141.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 10007ns | 9636ns | 10340ns | base | 6.549 |
| head_tail_dyn | 10323ns | 9551ns | 11340ns | +3.15% | 6.349 |
| kernel_nway | 10114ns | 9539ns | 10750ns | +1.07% | 6.479 |
| kernel_seq | 10013ns | 9622ns | 10581ns | +0.06% | 6.545 |
| nway | 61124ns | 59347ns | 63210ns | +510.78% | 1.072 |
| sequential | 56828ns | 55462ns | 58621ns | +467.86% | 1.153 |
| std_parallel | 24690ns | 23571ns | 25794ns | +146.71% | 2.654 |

## Performance model

- Peak throughput: **6.870 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 6.587 | 95.9% |
| head_tail_dyn | 6.596 | 96.0% |
| kernel_nway | 6.580 | 95.8% |
| kernel_seq | 6.675 | 97.1% |
| nway | 1.080 | 15.7% |
| sequential | 1.167 | 17.0% |
| std_parallel | 2.677 | 39.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 10574ns | 10574ns | base |
| head_tail_dyn | 10999ns | 10999ns | +4.02% |
| kernel_nway | 10626ns | 10626ns | +0.49% |
| kernel_seq | 10624ns | 10624ns | +0.47% |
| nway | 61690ns | 61690ns | +483.39% |
| sequential | 57447ns | 57447ns | +443.27% |
| std_parallel | 25494ns | 25494ns | +141.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 9950ns | base | --- | [9854, 10056] | --- | --- | --- | --- |
| head_tail_dyn | 9935ns | no significant difference | [-279, +494]ns | [9748, 10471] | no | 0.9774 | 0.8145 | 0 |
| kernel_nway | 9960ns | no significant difference | [-242, +452]ns | [9729, 10283] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 9819ns | no significant difference | [-331, +65]ns | [9688, 9952] | no | 0.7210 | 0.4807 | 0 |
| nway | 60696ns | +50589.6ns (+508.4%) | [+49754, +51306]ns | [59702, 61315] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 56148ns | +46210.4ns (+464.4%) | [+46029, +46825]ns | [55823, 56883] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 24485ns | +14404.1ns (+144.8%) | [+14125, +14965]ns | [24054, 24923] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 10004ns | +0.3% | +1.7% | -1.3% | +510.0% | +461.2% | +142.5% |
| 1 | 2 | 10050ns | +0.5% | -3.2% | -2.3% | +494.1% | +456.7% | +144.1% |
| 1 | 3 | 9879ns | -1.3% | -0.9% | -1.2% | +522.1% | +473.1% | +155.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.334 | moderate- |
| head_tail_dyn | -0.277 | moderate- |
| kernel_nway | -0.246 | moderate- |
| kernel_seq | -0.000 | ok |
| nway | -0.630 | HIGH- (thermal bounce) |
| sequential | -0.353 | moderate- |
| std_parallel | -0.044 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 444482.2ns | 10007.4ns | 4441.5% | HIGH |
| head_tail_dyn | 448528.9ns | 10322.7ns | 4345.1% | HIGH |
| kernel_nway | 447401.8ns | 10114.4ns | 4423.4% | HIGH |
| kernel_seq | 446650.0ns | 10013.4ns | 4460.5% | HIGH |
| nway | 481933.8ns | 61123.6ns | 788.5% | HIGH |
| sequential | 474102.6ns | 56828.5ns | 834.3% | HIGH |
| std_parallel | 456191.0ns | 24689.6ns | 1847.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 9636.1-10340.3 ns)
   9636.1 |#############
   9671.3 |#############
   9706.5 |
   9741.7 |
   9776.9 |#############
   9812.1 |#############
   9847.4 |
   9882.6 |##########################
   9917.8 |##########################
   9953.0 |########################################
   9988.2 |
  10023.4 |#############
  10058.6 |#############
  10093.8 |
  10129.0 |
  10164.2 |#############
  10199.4 |
  10234.7 |
  10269.9 |
  10305.1 |#############
  (1 below, 2 above range)

head_tail_dyn (n=18, range 9551.4-11340.3 ns)
   9551.4 |#############
   9640.8 |##########################
   9730.3 |########################################
   9819.7 |#############
   9909.2 |#############
   9998.6 |
  10088.1 |
  10177.5 |##########################
  10267.0 |
  10356.4 |
  10445.8 |########################################
  10535.3 |
  10624.7 |
  10714.2 |#############
  10803.6 |#############
  10893.1 |
  10982.5 |
  11072.0 |
  11161.4 |
  11250.8 |
  (2 below, 1 above range)

kernel_nway (n=18, range 9538.9-10750.0 ns)
   9538.9 |########################################
   9599.5 |
   9660.0 |########################################
   9720.6 |########################################
   9781.1 |########################################
   9841.7 |
   9902.2 |
   9962.8 |
  10023.3 |
  10083.9 |########################################
  10144.5 |####################
  10205.0 |####################
  10265.6 |
  10326.1 |####################
  10386.7 |####################
  10447.2 |
  10507.8 |
  10568.3 |
  10628.9 |
  10689.5 |########################################
  (1 below, 1 above range)

kernel_seq (n=18, range 9622.2-10581.2 ns)
   9622.2 |########################################
   9670.2 |##########
   9718.1 |
   9766.1 |##############################
   9814.0 |##############################
   9862.0 |
   9909.9 |##########
   9957.9 |##########
  10005.8 |
  10053.8 |####################
  10101.7 |
  10149.7 |
  10197.6 |
  10245.6 |
  10293.5 |
  10341.5 |
  10389.4 |
  10437.4 |##########
  10485.3 |
  10533.3 |
  (1 below, 1 above range)

nway (n=18, range 59347.2-63209.7 ns)
  59347.2 |#############
  59540.3 |##########################
  59733.5 |#############
  59926.6 |#############
  60119.7 |
  60312.8 |##########################
  60506.0 |
  60699.1 |
  60892.2 |##########################
  61085.3 |##########################
  61278.5 |#############
  61471.6 |
  61664.7 |########################################
  61857.8 |
  62051.0 |
  62244.1 |
  62437.2 |
  62630.4 |
  62823.5 |
  63016.6 |
  (2 below, 1 above range)

sequential (n=18, range 55462.5-58620.8 ns)
  55462.5 |
  55620.4 |##########################
  55778.3 |##########################
  55936.2 |########################################
  56094.2 |#############
  56252.1 |
  56410.0 |#############
  56567.9 |#############
  56725.8 |#############
  56883.8 |
  57041.7 |##########################
  57199.6 |
  57357.5 |
  57515.4 |
  57673.3 |
  57831.2 |
  57989.2 |
  58147.1 |
  58305.0 |#############
  58462.9 |
  (2 below, 2 above range)

std_parallel (n=18, range 23570.8-25794.5 ns)
  23570.8 |####################
  23682.0 |
  23793.2 |####################
  23904.4 |####################
  24015.6 |
  24126.7 |####################
  24237.9 |########################################
  24349.1 |####################
  24460.3 |####################
  24571.5 |####################
  24682.6 |####################
  24793.8 |
  24905.0 |########################################
  25016.2 |
  25127.4 |
  25238.5 |
  25349.7 |
  25460.9 |
  25572.1 |
  25683.3 |####################
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=4451.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=4479.6% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=4475.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=4520.7% of algo (FFI overhead may distort results)
- **nway**: bridge=793.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=837.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=1856.9% of algo (FFI overhead may distort results)

# fold_light, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 24% faster than the next best (head_tail)

kernel_nway (58.47 us) leads head_tail (72.48 us) by 24%, a clear separation rather than a photo finish. CV 3.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### head_tail_dyn is an outlier: 2.4x slower than the field

head_tail_dyn (137.71 us) is 2.4x the fastest (58.47 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: kernel_nway** at 58472.9 ns median (-19.3% vs baseline)
- 1 variant significantly faster than baseline
- 5 variants significantly slower than baseline
- Spread: 2.36x (fastest 58472.9 ns, slowest 137708.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 4 |
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
| head_tail | 72482ns | 73081ns | 68811ns | 72526ns | 74252ns | base |
| head_tail_dyn | 138250ns | 138088ns | 136371ns | 137875ns | 139751ns | +90.74% |
| kernel_nway | 59085ns | 59327ns | 55825ns | 58613ns | 61425ns | -18.48% |
| kernel_seq | 96182ns | 95588ns | 94833ns | 95419ns | 98000ns | +32.70% |
| nway | 74605ns | 74638ns | 70704ns | 74136ns | 77260ns | +2.93% |
| sequential | 101484ns | 100256ns | 98783ns | 99960ns | 105119ns | +40.01% |
| std_parallel | 87307ns | 85648ns | 76651ns | 84703ns | 96540ns | +20.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 71845ns | 68140ns | 73622ns | base | 14.595 |
| head_tail_dyn | 137920ns | 136085ns | 139417ns | +91.97% | 7.603 |
| kernel_nway | 58363ns | 55101ns | 60702ns | -18.76% | 17.966 |
| kernel_seq | 95950ns | 94642ns | 97726ns | +33.55% | 10.928 |
| nway | 73989ns | 70158ns | 76644ns | +2.98% | 14.172 |
| sequential | 101268ns | 98579ns | 104883ns | +40.95% | 10.355 |
| std_parallel | 86882ns | 76236ns | 96099ns | +20.93% | 12.069 |

## Performance model

- Peak throughput: **19.030 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 14.467 | 76.0% |
| head_tail_dyn | 7.614 | 40.0% |
| kernel_nway | 17.933 | 94.2% |
| kernel_seq | 10.991 | 57.8% |
| nway | 14.152 | 74.4% |
| sequential | 10.480 | 55.1% |
| std_parallel | 12.306 | 64.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 72482ns | 72482ns | base |
| head_tail_dyn | 138250ns | 138250ns | +90.74% |
| kernel_nway | 59085ns | 59085ns | -18.48% |
| kernel_seq | 96182ns | 96182ns | +32.70% |
| nway | 74605ns | 74605ns | +2.93% |
| sequential | 101484ns | 101484ns | +40.01% |
| std_parallel | 87307ns | 87307ns | +20.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 72481ns | base | --- | [71058, 73106] | --- | --- | --- | --- |
| head_tail_dyn | 137708ns | +66550.0ns (+91.8%) | [+64498, +67058]ns | [137292, 138704] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 58473ns | -13797.9ns (-19.0%) | [-14942, -12006]ns | [56833, 59988] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 95406ns | +23204.2ns (+32.0%) | [+21994, +25310]ns | [94958, 95856] | YES | 0.0000 | 0.0000 | 0 |
| nway | 74092ns | +1452.1ns (+2.0%) | [+144, +4531]ns | [72760, 75465] | YES (adj: no) | 0.0963 | 0.0963 | 0 |
| sequential | 100056ns | +27729.2ns (+38.3%) | [+26744, +29910]ns | [99235, 100952] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 85208ns | +13095.8ns (+18.1%) | [+11008, +20062]ns | [83454, 90860] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 70767ns | +94.6% | -17.4% | +34.8% | +3.1% | +42.7% | +23.2% |
| 1 | 2 | 72267ns | +90.5% | -21.4% | +31.9% | +4.4% | +37.8% | +16.1% |
| 1 | 3 | 73010ns | +88.5% | -16.9% | +30.6% | +1.8% | +36.0% | +16.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.024 | ok |
| head_tail_dyn | -0.077 | ok |
| kernel_nway | -0.440 | moderate- |
| kernel_seq | -0.395 | moderate- |
| nway | -0.468 | moderate- |
| sequential | -0.071 | ok |
| std_parallel | -0.318 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 140001.4ns | 71844.9ns | 194.9% | HIGH |
| head_tail_dyn | 137315.7ns | 137919.7ns | 99.6% | HIGH |
| kernel_nway | 119277.1ns | 58363.4ns | 204.4% | HIGH |
| kernel_seq | 95408.8ns | 95949.8ns | 99.4% | HIGH |
| nway | 140022.0ns | 73989.1ns | 189.2% | HIGH |
| sequential | 101259.5ns | 101267.6ns | 100.0% | HIGH |
| std_parallel | 130558.3ns | 86882.4ns | 150.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 68140.3-73622.2 ns)
  68140.3 |
  68414.4 |#############
  68688.5 |#############
  68962.6 |
  69236.7 |
  69510.8 |
  69784.9 |
  70059.0 |#############
  70333.1 |
  70607.2 |
  70881.2 |#############
  71155.3 |#############
  71429.4 |
  71703.5 |#############
  71977.6 |#############
  72251.7 |#############
  72525.8 |#############
  72799.9 |########################################
  73074.0 |########################################
  73348.1 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 136084.7-139416.7 ns)
  136084.7 |
  136251.3 |
  136417.9 |##########
  136584.5 |##########
  136751.1 |
  136917.7 |
  137084.3 |##########
  137250.9 |########################################
  137417.5 |##########
  137584.1 |
  137750.7 |
  137917.3 |##############################
  138083.9 |
  138250.5 |
  138417.1 |##########
  138583.7 |
  138750.3 |##########
  138916.9 |
  139083.5 |
  139250.1 |
  (1 below, 4 above range)

kernel_nway (n=18, range 55101.4-60702.1 ns)
  55101.4 |
  55381.4 |
  55661.5 |####################
  55941.5 |
  56221.5 |####################
  56501.6 |########################################
  56781.6 |########################################
  57061.6 |
  57341.7 |
  57621.7 |
  57901.7 |
  58181.8 |####################
  58461.8 |########################################
  58741.8 |####################
  59021.9 |
  59301.9 |########################################
  59581.9 |
  59862.0 |
  60142.0 |
  60422.0 |########################################
  (1 below, 3 above range)

kernel_seq (n=18, range 94641.7-97726.4 ns)
  94641.7 |########################################
  94795.9 |#############
  94950.1 |########################################
  95104.4 |
  95258.6 |#############
  95412.9 |##########################
  95567.1 |#############
  95721.3 |#############
  95875.6 |##########################
  96029.8 |
  96184.0 |
  96338.3 |
  96492.5 |
  96646.7 |
  96801.0 |
  96955.2 |
  97109.5 |
  97263.7 |
  97417.9 |
  97572.2 |
  (1 below, 3 above range)

nway (n=18, range 70158.3-76643.8 ns)
  70158.3 |####################
  70482.6 |
  70806.9 |
  71131.1 |
  71455.4 |########################################
  71779.7 |
  72104.0 |
  72428.2 |####################
  72752.5 |####################
  73076.8 |####################
  73401.0 |####################
  73725.3 |####################
  74049.6 |########################################
  74373.9 |
  74698.1 |########################################
  75022.4 |
  75346.7 |
  75670.9 |####################
  75995.2 |
  76319.5 |
  (1 below, 4 above range)

sequential (n=18, range 98579.2-104883.3 ns)
  98579.2 |####################
  98894.4 |########################################
  99209.6 |########################################
  99524.8 |
  99840.0 |########################################
  100155.2 |########################################
  100470.4 |####################
  100785.6 |########################################
  101100.8 |
  101416.0 |####################
  101731.2 |
  102046.5 |
  102361.7 |####################
  102676.9 |
  102992.1 |
  103307.3 |
  103622.5 |
  103937.7 |
  104252.9 |
  104568.1 |
  (2 below, 2 above range)

std_parallel (n=18, range 76236.1-96098.6 ns)
  76236.1 |#############
  77229.2 |#############
  78222.3 |#############
  79215.5 |
  80208.6 |
  81201.7 |
  82194.8 |
  83188.0 |########################################
  84181.1 |##########################
  85174.2 |#############
  86167.3 |#############
  87160.5 |
  88153.6 |#############
  89146.7 |#############
  90139.8 |
  91133.0 |#############
  92126.1 |#############
  93119.2 |
  94112.3 |#############
  95105.5 |#############
  (1 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=197.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=99.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=204.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.9% of algo (FFI overhead may distort results)
- **nway**: bridge=189.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=153.8% of algo (FFI overhead may distort results)

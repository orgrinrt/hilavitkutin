# append, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 30% faster than the next best (std_parallel)

kernel_nway (69.45 us) leads std_parallel (90.61 us) by 30%, a clear separation rather than a photo finish. CV 4.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_nway beats baseline by 39% (significant)

kernel_nway is -43.62 us (39%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 10.9x slower than the field

sequential (756.58 us) is 10.9x the fastest (69.45 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.66)

sequential's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 10.9x the fastest

Fastest kernel_nway (69.45 us) to slowest sequential (756.58 us): 10.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_nway** at 69447.9 ns median (-37.9% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 10.89x (fastest 69447.9 ns, slowest 756583.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 4 |
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
| head_tail | 115190ns | 114838ns | 111974ns | 113951ns | 118657ns | base |
| head_tail_dyn | 114632ns | 113638ns | 112529ns | 113560ns | 117292ns | -0.48% |
| kernel_nway | 72457ns | 72148ns | 68775ns | 71245ns | 76116ns | -37.10% |
| kernel_seq | 113633ns | 113181ns | 112539ns | 113068ns | 115027ns | -1.35% |
| nway | 252712ns | 246567ns | 238324ns | 244620ns | 272042ns | +119.39% |
| sequential | 759751ns | 758571ns | 747854ns | 757305ns | 769368ns | +559.56% |
| std_parallel | 92026ns | 92004ns | 88619ns | 91406ns | 94658ns | -20.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 112787ns | 110833ns | 115494ns | base | 9.297 |
| head_tail_dyn | 111436ns | 110935ns | 111995ns | -1.20% | 9.410 |
| kernel_nway | 69778ns | 66592ns | 72747ns | -38.13% | 15.027 |
| kernel_seq | 111481ns | 110932ns | 112031ns | -1.16% | 9.406 |
| nway | 249762ns | 235214ns | 269331ns | +121.44% | 4.198 |
| sequential | 758011ns | 746676ns | 766853ns | +572.07% | 1.383 |
| std_parallel | 90521ns | 87143ns | 93290ns | -19.74% | 11.584 |

## Performance model

- Peak throughput: **15.746 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 9.376 | 59.5% |
| head_tail_dyn | 9.425 | 59.9% |
| kernel_nway | 15.099 | 95.9% |
| kernel_seq | 9.408 | 59.7% |
| nway | 4.304 | 27.3% |
| sequential | 1.386 | 8.8% |
| std_parallel | 11.572 | 73.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 115190ns | 115190ns | base |
| head_tail_dyn | 114632ns | 114632ns | -0.48% |
| kernel_nway | 72457ns | 72457ns | -37.10% |
| kernel_seq | 113633ns | 113633ns | -1.35% |
| nway | 252712ns | 252712ns | +119.39% |
| sequential | 759751ns | 759751ns | +559.56% |
| std_parallel | 92026ns | 92026ns | -20.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 111838ns | base | --- | [111158, 113427] | --- | --- | --- | --- |
| head_tail_dyn | 111258ns | no significant difference | [-2171, +185]ns | [111046, 111698] | no | 0.4807 | 0.4807 | 0 |
| kernel_nway | 69448ns | -43620.8ns (-39.0%) | [-44269, -41696]ns | [67967, 70496] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 111458ns | no significant difference | [-1669, +248]ns | [111040, 111665] | no | 0.4807 | 0.4807 | 0 |
| nway | 243625ns | +129914.6ns (+116.2%) | [+127665, +137269]ns | [238858, 249860] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 756583ns | +644466.7ns (+576.3%) | [+641279, +648158]ns | [754031, 760815] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 90612ns | -21600.0ns (-19.3%) | [-22967, -20017]ns | [88948, 91929] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 113427ns | -1.9% | -38.5% | -1.8% | +114.2% | +566.8% | -20.7% |
| 1 | 2 | 110967ns | +0.3% | -37.1% | +0.4% | +126.1% | +583.5% | -17.1% |
| 1 | 3 | 111350ns | +0.1% | -38.5% | +0.1% | +114.8% | +579.0% | -20.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.256 | moderate- |
| head_tail_dyn | -0.322 | moderate- |
| kernel_nway | -0.153 | ok |
| kernel_seq | -0.443 | moderate- |
| nway | -0.600 | HIGH- (thermal bounce) |
| sequential | -0.658 | HIGH- (thermal bounce) |
| std_parallel | -0.578 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 5807281.3ns | 112787.5ns | 5148.9% | HIGH |
| head_tail_dyn | 5808981.7ns | 111435.6ns | 5212.9% | HIGH |
| kernel_nway | 5810205.6ns | 69777.5ns | 8326.8% | HIGH |
| kernel_seq | 5810127.8ns | 111480.8ns | 5211.8% | HIGH |
| nway | 5975169.2ns | 249762.0ns | 2392.3% | HIGH |
| sequential | 6450080.6ns | 758010.6ns | 850.9% | HIGH |
| std_parallel | 5799663.2ns | 90521.1ns | 6407.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 110833.3-115493.8 ns)
  110833.3 |########################################
  111066.3 |##########
  111299.3 |##############################
  111532.4 |
  111765.4 |
  111998.4 |
  112231.4 |##########
  112464.5 |####################
  112697.5 |
  112930.5 |
  113163.5 |##########
  113396.5 |##########
  113629.6 |
  113862.6 |####################
  114095.6 |##########
  114328.6 |
  114561.7 |
  114794.7 |
  115027.7 |
  115260.7 |
  (1 below, 1 above range)

head_tail_dyn (n=18, range 110934.7-111995.2 ns)
  110934.7 |#############
  110987.7 |########################################
  111040.7 |#############
  111093.8 |#############
  111146.8 |
  111199.8 |##########################
  111252.8 |#############
  111305.9 |
  111358.9 |
  111411.9 |
  111464.9 |
  111517.9 |#############
  111571.0 |#############
  111624.0 |#############
  111677.0 |#############
  111730.0 |
  111783.1 |#############
  111836.1 |
  111889.1 |
  111942.1 |
  (1 below, 3 above range)

kernel_nway (n=18, range 66591.7-72747.2 ns)
  66591.7 |####################
  66899.4 |########################################
  67207.2 |
  67515.0 |
  67822.8 |########################################
  68130.6 |
  68438.3 |####################
  68746.1 |####################
  69053.9 |####################
  69361.7 |####################
  69669.5 |####################
  69977.2 |########################################
  70285.0 |
  70592.8 |########################################
  70900.6 |
  71208.3 |
  71516.1 |####################
  71823.9 |
  72131.7 |
  72439.5 |
  (1 below, 2 above range)

kernel_seq (n=18, range 110931.9-112030.5 ns)
  110931.9 |##########################
  110986.9 |##########################
  111041.8 |#############
  111096.7 |
  111151.7 |#############
  111206.6 |#############
  111261.5 |
  111316.4 |
  111371.4 |#############
  111426.3 |
  111481.2 |#############
  111536.2 |
  111591.1 |########################################
  111646.0 |#############
  111701.0 |#############
  111755.9 |
  111810.8 |
  111865.8 |
  111920.7 |
  111975.6 |#############
  (1 below, 2 above range)

nway (n=18, range 235213.9-269330.6 ns)
  235213.9 |#############
  236919.7 |##########################
  238625.5 |##########################
  240331.4 |#############
  242037.2 |#############
  243743.0 |########################################
  245448.9 |#############
  247154.7 |
  248860.6 |
  250566.4 |
  252272.2 |#############
  253978.1 |##########################
  255683.9 |
  257389.7 |
  259095.6 |
  260801.4 |
  262507.2 |
  264213.1 |
  265918.9 |
  267624.7 |
  (2 below, 2 above range)

sequential (n=18, range 746676.4-766852.8 ns)
  746676.4 |
  747685.2 |
  748694.0 |
  749702.8 |##########################
  750711.6 |
  751720.5 |#############
  752729.3 |#############
  753738.1 |
  754746.9 |#############
  755755.7 |########################################
  756764.6 |#############
  757773.4 |##########################
  758782.2 |
  759791.0 |#############
  760799.8 |#############
  761808.7 |
  762817.5 |
  763826.3 |
  764835.1 |
  765843.9 |#############
  (1 below, 3 above range)

std_parallel (n=18, range 87143.1-93289.6 ns)
  87143.1 |
  87450.4 |
  87757.7 |#############
  88065.0 |#############
  88372.4 |#############
  88679.7 |#############
  88987.0 |##########################
  89294.3 |#############
  89601.7 |
  89909.0 |
  90216.3 |
  90523.7 |########################################
  90831.0 |#############
  91138.3 |
  91445.6 |
  91753.0 |##########################
  92060.3 |
  92367.6 |
  92674.9 |
  92982.3 |#############
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=5192.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=5220.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=8352.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=5211.5% of algo (FFI overhead may distort results)
- **nway**: bridge=2446.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=851.7% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=6400.0% of algo (FFI overhead may distort results)

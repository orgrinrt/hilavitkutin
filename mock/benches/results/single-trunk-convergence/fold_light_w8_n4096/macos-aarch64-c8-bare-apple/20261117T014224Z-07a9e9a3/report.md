# fold_light, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 61% faster than the next best (head_tail)

kernel_seq (2.85 us) leads head_tail (4.60 us) by 61%, a clear separation rather than a photo finish. CV 5.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 36% (significant)

kernel_seq is -1.65 us (36%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 29.8x slower than the field

std_parallel (84.83 us) is 29.8x the fastest (2.85 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.52)

kernel_seq's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 29.8x the fastest

Fastest kernel_seq (2.85 us) to slowest std_parallel (84.83 us): 29.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 2.4x its best-20%

kernel_nway's best 20% of batches run at 11.30 us but its worst 20% at 27.49 us (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 2850.0 ns median (-38.0% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 29.76x (fastest 2850.0 ns, slowest 84827.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 8 |
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
| head_tail | 5395ns | 5200ns | 4735ns | 5092ns | 6181ns | base |
| head_tail_dyn | 5301ns | 5262ns | 4943ns | 5199ns | 5634ns | -1.74% |
| kernel_nway | 19761ns | 18160ns | 11935ns | 16784ns | 28141ns | +266.28% |
| kernel_seq | 3434ns | 3360ns | 3132ns | 3332ns | 3739ns | -36.35% |
| nway | 50390ns | 50506ns | 46229ns | 49266ns | 54157ns | +833.99% |
| sequential | 10821ns | 10729ns | 10014ns | 10482ns | 11734ns | +100.58% |
| std_parallel | 87209ns | 85202ns | 76726ns | 84319ns | 96785ns | +1516.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4784ns | 4157ns | 5545ns | base | 0.856 |
| head_tail_dyn | 4699ns | 4301ns | 5035ns | -1.78% | 0.872 |
| kernel_nway | 19099ns | 11300ns | 27492ns | +299.20% | 0.214 |
| kernel_seq | 2912ns | 2719ns | 3119ns | -39.13% | 1.407 |
| nway | 49811ns | 45662ns | 53578ns | +941.14% | 0.082 |
| sequential | 10630ns | 9878ns | 11460ns | +122.19% | 0.385 |
| std_parallel | 86838ns | 76367ns | 96392ns | +1715.07% | 0.047 |

## Performance model

- Peak throughput: **1.506 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.891 | 59.1% |
| head_tail_dyn | 0.876 | 58.2% |
| kernel_nway | 0.234 | 15.5% |
| kernel_seq | 1.437 | 95.4% |
| nway | 0.082 | 5.4% |
| sequential | 0.388 | 25.8% |
| std_parallel | 0.048 | 3.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 5395ns | 5395ns | base |
| head_tail_dyn | 5301ns | 5301ns | -1.74% |
| kernel_nway | 19761ns | 19761ns | +266.28% |
| kernel_seq | 3434ns | 3434ns | -36.35% |
| nway | 50390ns | 50390ns | +833.99% |
| sequential | 10821ns | 10821ns | +100.58% |
| std_parallel | 87209ns | 87209ns | +1516.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4598ns | base | --- | [4298, 5100] | --- | --- | --- | --- |
| head_tail_dyn | 4675ns | no significant difference | [-373, +312]ns | [4494, 4883] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 17515ns | +12145.9ns (+264.2%) | [+9221, +20479]ns | [14083, 24671] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2850ns | -1645.8ns (-35.8%) | [-2173, -1396]ns | [2783, 3050] | YES | 0.0000 | 0.0000 | 0 |
| nway | 49965ns | +45258.3ns (+984.3%) | [+42327, +47571]ns | [47260, 52165] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10556ns | +5843.8ns (+127.1%) | [+5600, +6075]ns | [9929, 10850] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 84827ns | +80435.4ns (+1749.4%) | [+77912, +88029]ns | [82142, 93575] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4256ns | +3.2% | +557.9% | -34.7% | +1162.9% | +132.3% | +1891.8% |
| 1 | 2 | 4502ns | +9.9% | +163.3% | -34.1% | +996.6% | +135.1% | +1644.2% |
| 1 | 3 | 5515ns | -15.2% | +217.6% | -48.0% | +745.2% | +104.0% | +1636.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.073 | ok |
| head_tail_dyn | -0.494 | moderate- |
| kernel_nway | -0.394 | moderate- |
| kernel_seq | -0.518 | HIGH- (thermal bounce) |
| nway | -0.011 | ok |
| sequential | -0.000 | ok |
| std_parallel | -0.403 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 95656.9ns | 4784.3ns | 1999.4% | HIGH |
| head_tail_dyn | 92838.0ns | 4699.1ns | 1975.7% | HIGH |
| kernel_nway | 109129.6ns | 19098.8ns | 571.4% | HIGH |
| kernel_seq | 90320.1ns | 2912.0ns | 3101.6% | HIGH |
| nway | 106500.2ns | 49810.9ns | 213.8% | HIGH |
| sequential | 88054.9ns | 10630.3ns | 828.3% | HIGH |
| std_parallel | 128662.5ns | 86837.7ns | 148.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 4157.0-5545.1 ns)
   4157.0 |#############
   4226.4 |#############
   4295.8 |########################################
   4365.2 |
   4434.6 |#############
   4504.0 |#############
   4573.4 |
   4642.8 |##########################
   4712.2 |#############
   4781.6 |
   4851.1 |
   4920.5 |
   4989.9 |
   5059.3 |##########################
   5128.7 |
   5198.1 |
   5267.5 |
   5336.9 |
   5406.3 |
   5475.7 |#############
  (2 below, 3 above range)

head_tail_dyn (n=18, range 4301.4-5034.7 ns)
   4301.4 |####################
   4338.0 |####################
   4374.7 |
   4411.4 |####################
   4448.0 |####################
   4484.7 |####################
   4521.4 |
   4558.0 |####################
   4594.7 |########################################
   4631.4 |
   4668.0 |
   4704.7 |####################
   4741.4 |####################
   4778.1 |####################
   4814.7 |
   4851.4 |####################
   4888.1 |####################
   4924.7 |####################
   4961.4 |####################
   4998.1 |
  (1 below, 2 above range)

kernel_nway (n=18, range 11300.0-27492.4 ns)
  11300.0 |########################################
  12109.6 |
  12919.2 |
  13728.9 |
  14538.5 |
  15348.1 |#############
  16157.7 |##########################
  16967.3 |##########################
  17776.9 |#############
  18586.6 |#############
  19396.2 |
  20205.8 |
  21015.4 |
  21825.0 |
  22634.6 |
  23444.3 |
  24253.9 |#############
  25063.5 |#############
  25873.1 |
  26682.7 |
  (2 below, 4 above range)

kernel_seq (n=18, range 2719.4-3118.8 ns)
   2719.4 |##########
   2739.4 |##############################
   2759.4 |
   2779.3 |
   2799.3 |##########
   2819.3 |
   2839.2 |########################################
   2859.2 |
   2879.2 |##########
   2899.1 |
   2919.1 |
   2939.1 |##########
   2959.0 |
   2979.0 |
   2999.0 |##########
   3018.9 |
   3038.9 |
   3058.9 |
   3078.8 |##########
   3098.8 |
  (1 below, 4 above range)

nway (n=18, range 45662.5-53578.5 ns)
  45662.5 |########################################
  46058.3 |####################
  46454.1 |
  46849.9 |####################
  47245.7 |####################
  47641.5 |
  48037.3 |########################################
  48433.1 |
  48828.9 |
  49224.7 |####################
  49620.5 |
  50016.3 |####################
  50412.1 |########################################
  50807.9 |
  51203.7 |
  51599.5 |####################
  51995.3 |
  52391.1 |
  52786.9 |
  53182.7 |####################
  (1 below, 4 above range)

sequential (n=18, range 9877.8-11459.7 ns)
   9877.8 |########################################
   9956.9 |#############
  10036.0 |
  10115.1 |
  10194.2 |
  10273.3 |
  10352.4 |#############
  10431.4 |#############
  10510.5 |##########################
  10589.6 |##########################
  10668.7 |#############
  10747.8 |
  10826.9 |
  10906.0 |
  10985.1 |#############
  11064.2 |#############
  11143.3 |
  11222.4 |
  11301.5 |
  11380.6 |
  (2 below, 3 above range)

std_parallel (n=18, range 76366.7-96391.6 ns)
  76366.7 |########################################
  77367.9 |
  78369.2 |
  79370.4 |####################
  80371.7 |
  81372.9 |########################################
  82374.2 |####################
  83375.4 |########################################
  84376.7 |
  85377.9 |####################
  86379.2 |
  87380.4 |########################################
  88381.7 |
  89382.9 |
  90384.2 |
  91385.4 |
  92386.7 |
  93387.9 |########################################
  94389.2 |
  95390.4 |########################################
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=2091.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=1980.3% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=33.9% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=641.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3154.9% of algo (FFI overhead may distort results)
- **nway**: bridge=213.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=842.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=152.4% of algo (FFI overhead may distort results)

# fold_light, short fiber (1 to 4 morsels), W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 57% faster than the next best (head_tail)

kernel_seq (2.65 us) leads head_tail (4.15 us) by 57%, a clear separation rather than a photo finish. CV 5.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 37% (significant)

kernel_seq is -1.52 us (37%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 31.9x slower than the field

std_parallel (84.54 us) is 31.9x the fastest (2.65 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.66)

head_tail's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 31.9x the fastest

Fastest kernel_seq (2.65 us) to slowest std_parallel (84.54 us): 31.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2651.7 ns median (-36.1% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 31.88x (fastest 2651.7 ns, slowest 84537.5 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 5045ns | 4772ns | 4586ns | 4726ns | 5754ns | base |
| head_tail_dyn | 4821ns | 4747ns | 4652ns | 4738ns | 5029ns | -4.45% |
| kernel_nway | 16587ns | 15935ns | 14533ns | 15632ns | 19046ns | +228.77% |
| kernel_seq | 3163ns | 3140ns | 2821ns | 3073ns | 3470ns | -37.30% |
| nway | 51189ns | 51065ns | 47361ns | 50683ns | 53863ns | +914.62% |
| sequential | 10308ns | 10125ns | 9771ns | 10026ns | 10998ns | +104.30% |
| std_parallel | 84858ns | 84888ns | 74038ns | 82489ns | 93820ns | +1581.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4431ns | 3968ns | 5119ns | base | 0.231 |
| head_tail_dyn | 4254ns | 4094ns | 4441ns | -3.98% | 0.241 |
| kernel_nway | 15966ns | 13948ns | 18417ns | +260.37% | 0.064 |
| kernel_seq | 2707ns | 2572ns | 2875ns | -38.91% | 0.378 |
| nway | 50632ns | 46806ns | 53302ns | +1042.80% | 0.020 |
| sequential | 10165ns | 9634ns | 10843ns | +129.43% | 0.101 |
| std_parallel | 84474ns | 73724ns | 93374ns | +1806.62% | 0.012 |

## Performance model

- Peak throughput: **0.398 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.247 | 61.9% |
| head_tail_dyn | 0.243 | 61.0% |
| kernel_nway | 0.067 | 16.8% |
| kernel_seq | 0.386 | 97.0% |
| nway | 0.020 | 5.1% |
| sequential | 0.102 | 25.7% |
| std_parallel | 0.012 | 3.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 5045ns | 5045ns | base |
| head_tail_dyn | 4821ns | 4821ns | -4.45% |
| kernel_nway | 16587ns | 16587ns | +228.77% |
| kernel_seq | 3163ns | 3163ns | -37.30% |
| nway | 51189ns | 51189ns | +914.62% |
| sequential | 10308ns | 10308ns | +104.30% |
| std_parallel | 84858ns | 84858ns | +1581.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4152ns | base | --- | [4075, 4480] | --- | --- | --- | --- |
| head_tail_dyn | 4217ns | no significant difference | [-304, +159]ns | [4141, 4239] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 15310ns | +10899.2ns (+262.5%) | [+10460, +11963]ns | [14660, 16053] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2652ns | -1515.8ns (-36.5%) | [-1806, -1438]ns | [2608, 2780] | YES | 0.0000 | 0.0000 | 0 |
| nway | 50501ns | +46213.3ns (+1112.9%) | [+44845, +47617]ns | [49296, 52291] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 9991ns | +5645.9ns (+136.0%) | [+5457, +5927]ns | [9652, 10409] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 84538ns | +80544.1ns (+1939.7%) | [+74620, +83768]ns | [78925, 87805] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4127ns | +3.8% | +256.8% | -37.1% | +1135.2% | +133.6% | +2022.1% |
| 1 | 2 | 4936ns | -16.1% | +215.9% | -41.9% | +917.5% | +103.4% | +1565.0% |
| 1 | 3 | 4075ns | +3.5% | +284.2% | -35.4% | +1139.3% | +157.8% | +1974.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.665 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.521 | HIGH- (thermal bounce) |
| kernel_nway | -0.132 | ok |
| kernel_seq | -0.655 | HIGH- (thermal bounce) |
| nway | -0.407 | moderate- |
| sequential | -0.001 | ok |
| std_parallel | -0.456 | moderate- |

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
| head_tail | 97313.2ns | 4430.6ns | 2196.4% | HIGH |
| head_tail_dyn | 95490.9ns | 4254.4ns | 2244.5% | HIGH |
| kernel_nway | 112941.6ns | 15966.4ns | 707.4% | HIGH |
| kernel_seq | 93034.2ns | 2706.7ns | 3437.2% | HIGH |
| nway | 111101.2ns | 50632.1ns | 219.4% | HIGH |
| sequential | 89376.9ns | 10165.0ns | 879.3% | HIGH |
| std_parallel | 131563.3ns | 84473.6ns | 155.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 3967.8-5119.1 ns)
   3967.8 |##########################
   4025.3 |########################################
   4082.9 |########################################
   4140.5 |##########################
   4198.0 |##########################
   4255.6 |
   4313.2 |
   4370.8 |
   4428.3 |
   4485.9 |
   4543.5 |
   4601.0 |
   4658.6 |
   4716.2 |#############
   4773.7 |
   4831.3 |
   4888.9 |##########################
   4946.4 |
   5004.0 |
   5061.6 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 4093.9-4440.6 ns)
   4093.9 |########
   4111.2 |########################
   4128.6 |########
   4145.9 |########
   4163.2 |
   4180.6 |########
   4197.9 |########
   4215.2 |########
   4232.6 |########################################
   4249.9 |
   4267.2 |
   4284.6 |
   4301.9 |
   4319.2 |########
   4336.6 |
   4353.9 |
   4371.2 |########
   4388.6 |
   4405.9 |
   4423.2 |
  (1 below, 1 above range)

kernel_nway (n=18, range 13947.8-18417.0 ns)
  13947.8 |########################################
  14171.2 |
  14394.7 |####################
  14618.1 |########################################
  14841.6 |####################
  15065.1 |####################
  15288.5 |####################
  15512.0 |####################
  15735.4 |########################################
  15958.9 |####################
  16182.4 |
  16405.8 |
  16629.3 |
  16852.7 |####################
  17076.2 |
  17299.7 |
  17523.1 |
  17746.6 |
  17970.0 |
  18193.5 |
  (2 below, 3 above range)

kernel_seq (n=18, range 2571.7-2875.0 ns)
   2571.7 |########################################
   2586.8 |########################################
   2602.0 |########################################
   2617.2 |####################
   2632.3 |
   2647.5 |########################################
   2662.7 |
   2677.8 |####################
   2693.0 |
   2708.2 |
   2723.3 |####################
   2738.5 |
   2753.7 |####################
   2768.8 |
   2784.0 |
   2799.2 |####################
   2814.3 |
   2829.5 |
   2844.7 |####################
   2859.8 |
  (1 below, 3 above range)

nway (n=18, range 46806.1-53301.7 ns)
  46806.1 |
  47130.9 |
  47455.7 |
  47780.4 |####################
  48105.2 |####################
  48430.0 |
  48754.8 |####################
  49079.5 |####################
  49404.3 |########################################
  49729.1 |
  50053.9 |########################################
  50378.7 |####################
  50703.4 |
  51028.2 |
  51353.0 |####################
  51677.8 |########################################
  52002.6 |
  52327.3 |
  52652.1 |####################
  52976.9 |
  (1 below, 4 above range)

sequential (n=18, range 9633.9-10842.8 ns)
   9633.9 |########################################
   9694.3 |
   9754.8 |
   9815.2 |
   9875.7 |
   9936.1 |########################
   9996.6 |########
  10057.0 |################
  10117.4 |########
  10177.9 |
  10238.3 |
  10298.8 |
  10359.2 |
  10419.7 |
  10480.1 |
  10540.5 |
  10601.0 |
  10661.4 |########
  10721.9 |########
  10782.3 |
  (1 below, 3 above range)

std_parallel (n=18, range 73723.9-93374.4 ns)
  73723.9 |########################################
  74706.4 |####################
  75688.9 |
  76671.5 |
  77654.0 |
  78636.5 |########################################
  79619.0 |
  80601.6 |
  81584.1 |########################################
  82566.6 |
  83549.2 |####################
  84531.7 |####################
  85514.2 |####################
  86496.7 |####################
  87479.3 |########################################
  88461.8 |
  89444.3 |####################
  90426.9 |
  91409.4 |
  92391.9 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=2335.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2275.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=734.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3440.6% of algo (FFI overhead may distort results)
- **nway**: bridge=220.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=902.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=153.2% of algo (FFI overhead may distort results)

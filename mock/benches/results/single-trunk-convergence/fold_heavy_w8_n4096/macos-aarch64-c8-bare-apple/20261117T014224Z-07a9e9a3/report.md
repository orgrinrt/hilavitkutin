# fold_heavy, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 16% faster than the next best (head_tail_dyn)

kernel_seq (5.50 us) leads head_tail_dyn (6.38 us) by 16%, a clear separation rather than a photo finish. CV 6.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 22% (significant)

kernel_seq is -1.52 us (22%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 15.7x slower than the field

std_parallel (86.69 us) is 15.7x the fastest (5.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.61)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 15.7x the fastest

Fastest kernel_seq (5.50 us) to slowest std_parallel (86.69 us): 15.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 5504.1 ns median (-21.8% vs baseline)
- 2 variants significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 15.75x (fastest 5504.1 ns, slowest 86689.6 ns)

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
| head_tail | 7963ns | 7725ns | 6967ns | 7549ns | 9081ns | base |
| head_tail_dyn | 6977ns | 6967ns | 6788ns | 6940ns | 7128ns | -12.37% |
| kernel_nway | 23304ns | 24050ns | 19612ns | 23235ns | 25253ns | +192.67% |
| kernel_seq | 6120ns | 6033ns | 5754ns | 5960ns | 6544ns | -23.13% |
| nway | 52165ns | 52171ns | 48860ns | 51526ns | 54774ns | +555.13% |
| sequential | 13082ns | 12985ns | 12682ns | 12928ns | 13515ns | +64.30% |
| std_parallel | 87276ns | 87104ns | 78968ns | 85994ns | 93355ns | +996.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 7297ns | 6362ns | 8398ns | base | 0.561 |
| head_tail_dyn | 6379ns | 6267ns | 6478ns | -12.58% | 0.642 |
| kernel_nway | 22659ns | 19008ns | 24624ns | +210.54% | 0.181 |
| kernel_seq | 5646ns | 5375ns | 6023ns | -22.63% | 0.726 |
| nway | 51587ns | 48290ns | 54172ns | +606.98% | 0.079 |
| sequential | 12932ns | 12542ns | 13360ns | +77.23% | 0.317 |
| std_parallel | 86904ns | 78640ns | 92956ns | +1090.99% | 0.047 |

## Performance model

- Peak throughput: **0.762 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.582 | 76.3% |
| head_tail_dyn | 0.642 | 84.3% |
| kernel_nway | 0.175 | 23.0% |
| kernel_seq | 0.744 | 97.7% |
| nway | 0.079 | 10.4% |
| sequential | 0.319 | 41.9% |
| std_parallel | 0.047 | 6.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 7963ns | 7963ns | base |
| head_tail_dyn | 6977ns | 6977ns | -12.37% |
| kernel_nway | 23304ns | 23304ns | +192.67% |
| kernel_seq | 6120ns | 6120ns | -23.13% |
| nway | 52165ns | 52165ns | +555.13% |
| sequential | 13082ns | 13082ns | +64.30% |
| std_parallel | 87276ns | 87276ns | +996.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 7042ns | base | --- | [6473, 7756] | --- | --- | --- | --- |
| head_tail_dyn | 6379ns | -754.2ns (-10.7%) | [-1462, -29]ns | [6308, 6425] | YES | 0.0023 | 0.0023 | 1 |
| kernel_nway | 23415ns | +15814.6ns (+224.6%) | [+13767, +16819]ns | [21471, 23792] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 5504ns | -1516.7ns (-21.5%) | [-2221, -1046]ns | [5435, 5683] | YES | 0.0000 | 0.0000 | 0 |
| nway | 51617ns | +45143.8ns (+641.1%) | [+42129, +45492]ns | [50033, 52519] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 12838ns | +5793.8ns (+82.3%) | [+4985, +6729]ns | [12579, 13121] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 86690ns | +79410.4ns (+1127.7%) | [+76958, +84267]ns | [83748, 91023] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 8223ns | -22.4% | +161.1% | -28.4% | +508.5% | +52.5% | +907.5% |
| 1 | 2 | 7042ns | -10.4% | +234.4% | -22.7% | +653.6% | +82.3% | +1110.0% |
| 1 | 3 | 6419ns | -0.1% | +264.5% | -15.4% | +704.2% | +108.1% | +1318.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.021 | ok |
| head_tail_dyn | -0.607 | HIGH- (thermal bounce) |
| kernel_nway | -0.206 | moderate- |
| kernel_seq | -0.149 | ok |
| nway | -0.485 | moderate- |
| sequential | -0.016 | ok |
| std_parallel | -0.038 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 93302.1ns | 7296.8ns | 1278.7% | HIGH |
| head_tail_dyn | 85411.3ns | 6378.9ns | 1339.0% | HIGH |
| kernel_nway | 112439.4ns | 22659.5ns | 496.2% | HIGH |
| kernel_seq | 87397.0ns | 5645.6ns | 1548.1% | HIGH |
| nway | 109595.8ns | 51586.8ns | 212.4% | HIGH |
| sequential | 90548.6ns | 12931.7ns | 700.2% | HIGH |
| std_parallel | 127152.3ns | 86903.7ns | 146.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 6362.5-8397.9 ns)
   6362.5 |########################################
   6464.3 |########################################
   6566.0 |
   6667.8 |
   6769.6 |
   6871.4 |#############
   6973.1 |##########################
   7074.9 |
   7176.7 |
   7278.4 |
   7380.2 |#############
   7482.0 |#############
   7583.8 |
   7685.5 |##########################
   7787.3 |
   7889.1 |#############
   7990.8 |
   8092.6 |
   8194.4 |
   8296.1 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 6266.7-6477.8 ns)
   6266.7 |
   6277.2 |##########################
   6287.8 |#############
   6298.3 |#############
   6308.9 |#############
   6319.4 |#############
   6330.0 |
   6340.6 |
   6351.1 |
   6361.7 |#############
   6372.2 |##########################
   6382.8 |
   6393.3 |#############
   6403.9 |##########################
   6414.4 |
   6425.0 |
   6435.6 |########################################
   6446.1 |
   6456.7 |
   6467.2 |
  (1 below, 2 above range)

kernel_nway (n=18, range 19008.3-24624.3 ns)
  19008.3 |
  19289.1 |
  19569.9 |
  19850.7 |##########
  20131.5 |##########
  20412.3 |##########
  20693.1 |
  20973.9 |
  21254.7 |##########
  21535.5 |##########
  21816.3 |
  22097.1 |
  22377.9 |
  22658.7 |##########
  22939.5 |##########
  23220.3 |####################
  23501.1 |####################
  23781.9 |########################################
  24062.7 |
  24343.5 |
  (1 below, 2 above range)

kernel_seq (n=18, range 5375.0-6022.9 ns)
   5375.0 |####################
   5407.4 |########################################
   5439.8 |##########
   5472.2 |##########
   5504.6 |##########
   5537.0 |##########
   5569.4 |
   5601.8 |##########
   5634.2 |##########
   5666.5 |
   5698.9 |##########
   5731.3 |##########
   5763.7 |
   5796.1 |
   5828.5 |
   5860.9 |
   5893.3 |
   5925.7 |
   5958.1 |
   5990.5 |##########
  (1 below, 2 above range)

nway (n=18, range 48290.3-54172.2 ns)
  48290.3 |
  48584.4 |
  48878.5 |
  49172.6 |####################
  49466.7 |########################################
  49760.8 |
  50054.9 |
  50348.9 |####################
  50643.0 |####################
  50937.1 |
  51231.2 |########################################
  51525.3 |########################################
  51819.4 |####################
  52113.5 |####################
  52407.6 |
  52701.7 |####################
  52995.8 |
  53289.9 |########################################
  53584.0 |
  53878.1 |
  (2 below, 2 above range)

sequential (n=18, range 12541.7-13359.7 ns)
  12541.7 |########
  12582.6 |########
  12623.5 |
  12664.4 |
  12705.3 |
  12746.2 |
  12787.1 |
  12828.0 |########################################
  12868.9 |
  12909.8 |
  12950.7 |
  12991.6 |
  13032.5 |
  13073.4 |################
  13114.3 |########
  13155.2 |
  13196.1 |
  13237.0 |
  13277.9 |########
  13318.8 |
  (4 below, 3 above range)

std_parallel (n=18, range 78640.3-92955.6 ns)
  78640.3 |########################################
  79356.0 |####################
  80071.8 |
  80787.6 |
  81503.3 |
  82219.1 |
  82934.9 |####################
  83650.6 |####################
  84366.4 |
  85082.2 |
  85797.9 |########################################
  86513.7 |########################################
  87229.4 |
  87945.2 |####################
  88661.0 |####################
  89376.7 |
  90092.5 |####################
  90808.3 |####################
  91524.0 |####################
  92239.8 |####################
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=1312.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=1331.4% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=470.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=1585.5% of algo (FFI overhead may distort results)
- **nway**: bridge=209.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=702.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=145.2% of algo (FFI overhead may distort results)

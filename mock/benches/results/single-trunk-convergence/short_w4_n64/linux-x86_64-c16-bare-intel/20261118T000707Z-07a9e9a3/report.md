# fold_light, short fiber (1 to 4 morsels), W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 4.0x slower than the field

std_parallel (44.40 us) is 4.0x the fastest (10.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.65)

sequential's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn, kernel_seq, head_tail, sequential, kernel_nway} vs {nway, std_parallel} (166% apart)

The field splits into a fast tier {head_tail_dyn, kernel_seq, head_tail, sequential, kernel_nway} and a slow tier {nway, std_parallel} with a 166% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.0x the fastest

Fastest head_tail_dyn (10.98 us) to slowest std_parallel (44.40 us): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.5x its best-20%

sequential's best 20% of batches run at 6.73 us but its worst 20% at 23.23 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 10983.4 ns median (-10.0% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 4.04x (fastest 10983.4 ns, slowest 44398.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 4 |
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
| head_tail | 13474ns | 12761ns | 10538ns | 12441ns | 16491ns | base |
| head_tail_dyn | 11778ns | 11618ns | 10713ns | 11520ns | 12696ns | -12.59% |
| kernel_nway | 17640ns | 16863ns | 13670ns | 16648ns | 21113ns | +30.92% |
| kernel_seq | 13510ns | 12521ns | 11111ns | 12266ns | 16576ns | +0.27% |
| nway | 42735ns | 42687ns | 37181ns | 41277ns | 47698ns | +217.17% |
| sequential | 14606ns | 12829ns | 6870ns | 11107ns | 23724ns | +8.40% |
| std_parallel | 49986ns | 44656ns | 36759ns | 42789ns | 67393ns | +270.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 12790ns | 10047ns | 15591ns | base | 0.005 |
| head_tail_dyn | 11112ns | 10085ns | 11887ns | -13.12% | 0.006 |
| kernel_nway | 16709ns | 13049ns | 20032ns | +30.64% | 0.004 |
| kernel_seq | 12713ns | 10441ns | 15578ns | -0.60% | 0.005 |
| nway | 42145ns | 36712ns | 46965ns | +229.52% | 0.002 |
| sequential | 14310ns | 6732ns | 23231ns | +11.89% | 0.004 |
| std_parallel | 49646ns | 36484ns | 66940ns | +288.17% | 0.001 |

## Performance model

- Peak throughput: **0.010 Gops/s** (sequential; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.005 | 55.2% |
| head_tail_dyn | 0.006 | 61.3% |
| kernel_nway | 0.004 | 42.5% |
| kernel_seq | 0.005 | 57.2% |
| nway | 0.002 | 16.0% |
| sequential | 0.005 | 53.5% |
| std_parallel | 0.001 | 15.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 13474ns | 13474ns | base |
| head_tail_dyn | 11778ns | 11778ns | -12.59% |
| kernel_nway | 17640ns | 17640ns | +30.92% |
| kernel_seq | 13510ns | 13510ns | +0.27% |
| nway | 42735ns | 42735ns | +217.17% |
| sequential | 14606ns | 14606ns | +8.40% |
| std_parallel | 49986ns | 49986ns | +270.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 12205ns | base | --- | [10955, 14070] | --- | --- | --- | --- |
| head_tail_dyn | 10983ns | no significant difference | [-2818, +187]ns | [10809, 11431] | no | 0.1444 | 0.0963 | 0 |
| kernel_nway | 15837ns | +3609.8ns (+29.6%) | [+1014, +5776]ns | [15205, 17278] | YES (adj: no) | 0.1444 | 0.0963 | 0 |
| kernel_seq | 11767ns | no significant difference | [-1319, +1418]ns | [11234, 13064] | no | 0.9774 | 0.8145 | 0 |
| nway | 42118ns | +28610.5ns (+234.4%) | [+26848, +31507]ns | [38737, 44406] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 12580ns | no significant difference | [-4631, +6315]ns | [6923, 20560] | no | 1.0000 | 1.0000 | 0 |
| std_parallel | 44398ns | +32220.7ns (+264.0%) | [+28357, +41112]ns | [40057, 54537] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 15393ns | -28.8% | -5.7% | -25.2% | +184.1% | -3.1% | +246.7% |
| 1 | 2 | 11946ns | -10.9% | +30.3% | +14.3% | +262.5% | -41.4% | +271.4% |
| 1 | 3 | 10600ns | +11.7% | +70.0% | +13.2% | +252.6% | +57.3% | +303.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.040 | ok |
| head_tail_dyn | -0.333 | moderate- |
| kernel_nway | -0.033 | ok |
| kernel_seq | -0.636 | HIGH- (thermal bounce) |
| nway | -0.133 | ok |
| sequential | -0.647 | HIGH- (thermal bounce) |
| std_parallel | -0.092 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 13235.4ns | 12789.9ns | 103.5% | HIGH |
| head_tail_dyn | 11499.2ns | 11112.1ns | 103.5% | HIGH |
| kernel_nway | 16895.2ns | 16708.8ns | 101.1% | HIGH |
| kernel_seq | 12656.4ns | 12712.9ns | 99.6% | HIGH |
| nway | 43833.6ns | 42145.0ns | 104.0% | HIGH |
| sequential | 14752.8ns | 14310.3ns | 103.1% | HIGH |
| std_parallel | 48855.9ns | 49646.0ns | 98.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 10047.5-15590.9 ns)
  10047.5 |
  10324.6 |####################
  10601.8 |####################
  10879.0 |########################################
  11156.2 |
  11433.3 |####################
  11710.5 |####################
  11987.7 |########################################
  12264.9 |
  12542.0 |####################
  12819.2 |
  13096.4 |
  13373.6 |####################
  13650.7 |####################
  13927.9 |
  14205.1 |####################
  14482.3 |
  14759.4 |####################
  15036.6 |
  15313.8 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 10085.1-11886.9 ns)
  10085.1 |
  10175.2 |
  10265.3 |
  10355.3 |
  10445.4 |
  10535.5 |########
  10625.6 |################
  10715.7 |
  10805.8 |
  10895.9 |########################################
  10986.0 |
  11076.1 |########
  11166.2 |########
  11256.3 |########
  11346.4 |
  11436.5 |
  11526.5 |########
  11616.6 |########
  11706.7 |
  11796.8 |
  (2 below, 3 above range)

kernel_nway (n=18, range 13049.2-20031.7 ns)
  13049.2 |
  13398.4 |##########
  13747.5 |
  14096.6 |
  14445.7 |
  14794.9 |##########
  15144.0 |########################################
  15493.1 |##########
  15842.2 |##########
  16191.4 |
  16540.5 |####################
  16889.6 |##########
  17238.7 |##########
  17587.9 |
  17937.0 |
  18286.1 |
  18635.2 |##########
  18984.4 |
  19333.5 |##########
  19682.6 |
  (2 below, 2 above range)

kernel_seq (n=18, range 10440.5-15578.3 ns)
  10440.5 |#############
  10697.4 |#############
  10954.3 |#############
  11211.2 |#############
  11468.1 |########################################
  11725.0 |##########################
  11981.9 |#############
  12238.7 |#############
  12495.6 |
  12752.5 |
  13009.4 |
  13266.3 |
  13523.2 |
  13780.1 |#############
  14036.9 |
  14293.8 |
  14550.7 |
  14807.6 |
  15064.5 |
  15321.4 |
  (2 below, 4 above range)

nway (n=18, range 36712.3-46964.7 ns)
  36712.3 |########################################
  37224.9 |####################
  37737.5 |####################
  38250.2 |
  38762.8 |
  39275.4 |####################
  39788.0 |
  40300.6 |########################################
  40813.2 |
  41325.9 |
  41838.5 |########################################
  42351.1 |
  42863.7 |########################################
  43376.3 |
  43888.9 |
  44401.6 |####################
  44914.2 |########################################
  45426.8 |####################
  45939.4 |####################
  46452.0 |
  (1 below, 1 above range)

sequential (n=18, range 6732.5-23231.0 ns)
   6732.5 |########################################
   7557.4 |
   8382.4 |
   9207.3 |
  10032.2 |#####
  10857.1 |
  11682.0 |
  12507.0 |
  13331.9 |
  14156.8 |#####
  14981.8 |
  15806.7 |
  16631.6 |
  17456.5 |
  18281.4 |#####
  19106.4 |###########
  19931.3 |
  20756.2 |#####
  21581.2 |#####
  22406.1 |
  (1 below, 3 above range)

std_parallel (n=18, range 36484.3-66940.0 ns)
  36484.3 |####################
  38007.1 |####################
  39529.9 |##########
  41052.7 |##########
  42575.5 |##########
  44098.2 |########################################
  45621.0 |
  47143.8 |
  48666.6 |
  50189.4 |##########
  51712.1 |
  53234.9 |
  54757.7 |
  56280.5 |
  57803.3 |##########
  59326.1 |
  60848.8 |
  62371.6 |##########
  63894.4 |
  65417.2 |##########
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=104.0% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=105.3% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=102.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=103.4% of algo (FFI overhead may distort results)
- **nway**: bridge=102.1% of algo (FFI overhead may distort results)
- **sequential**: CV=52.9% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **sequential**: bridge=103.8% of algo (FFI overhead may distort results)
- **std_parallel**: CV=28.7% (high variance, measurements may be unstable)
- **std_parallel**: bridge=96.3% of algo (FFI overhead may distort results)

# append, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway is an outlier: 3.0x slower than the field

nway (42.42 us) is 3.0x the fastest (14.23 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (head_tail, kernel_seq) are a dead heat (<1%)

head_tail (14.23 us) and kernel_seq (14.34 us) differ by 0.79%, inside the noise, even though the wider field spreads 198.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### nway shows alternating (throttle bounce) (autocorr -0.67)

nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (14.23 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, kernel_seq, kernel_nway, head_tail_dyn, sequential} vs {std_parallel, nway} (111% apart)

The field splits into a fast tier {head_tail, kernel_seq, kernel_nway, head_tail_dyn, sequential} and a slow tier {std_parallel, nway} with a 111% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### sequential is inconsistent: worst-20% is 3.6x its best-20%

sequential's best 20% of batches run at 9.76 us but its worst 20% at 35.57 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 14229.0 ns median
- 3 variants significantly slower than baseline
- Spread: 2.98x (fastest 14229.0 ns, slowest 42422.8 ns)

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
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 16326ns | 14667ns | 13022ns | 14436ns | 20813ns | base |
| head_tail_dyn | 17382ns | 17011ns | 14071ns | 16213ns | 20790ns | +6.47% |
| kernel_nway | 16795ns | 15480ns | 12389ns | 14873ns | 21880ns | +2.87% |
| kernel_seq | 16144ns | 14833ns | 12311ns | 14560ns | 20435ns | -1.12% |
| nway | 47927ns | 42957ns | 36338ns | 41795ns | 62918ns | +193.56% |
| sequential | 22510ns | 19756ns | 9901ns | 17615ns | 36158ns | +37.88% |
| std_parallel | 44165ns | 42144ns | 35819ns | 39990ns | 54600ns | +170.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 15480ns | 12579ns | 19210ns | base | 0.265 |
| head_tail_dyn | 16458ns | 13550ns | 19165ns | +6.32% | 0.249 |
| kernel_nway | 15925ns | 11939ns | 20345ns | +2.88% | 0.257 |
| kernel_seq | 15287ns | 11789ns | 18890ns | -1.25% | 0.268 |
| nway | 47223ns | 35904ns | 61812ns | +205.06% | 0.087 |
| sequential | 22163ns | 9755ns | 35572ns | +43.17% | 0.185 |
| std_parallel | 43308ns | 35279ns | 53417ns | +179.77% | 0.095 |

## Performance model

- Peak throughput: **0.420 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.288 | 68.6% |
| head_tail_dyn | 0.250 | 59.6% |
| kernel_nway | 0.274 | 65.3% |
| kernel_seq | 0.286 | 68.0% |
| nway | 0.097 | 23.0% |
| sequential | 0.210 | 50.1% |
| std_parallel | 0.100 | 23.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 16326ns | 16326ns | base |
| head_tail_dyn | 17382ns | 17382ns | +6.47% |
| kernel_nway | 16795ns | 16795ns | +2.87% |
| kernel_seq | 16144ns | 16144ns | -1.12% |
| nway | 47927ns | 47927ns | +193.56% |
| sequential | 22510ns | 22510ns | +37.88% |
| std_parallel | 44165ns | 44165ns | +170.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 14229ns | base | --- | [13549, 15729] | --- | --- | --- | --- |
| head_tail_dyn | 16379ns | +1727.8ns (+12.1%) | [+795, +2461]ns | [14548, 17884] | YES | 0.0003 | 0.0001 | 0 |
| kernel_nway | 14946ns | no significant difference | [-621, +1393]ns | [13028, 16618] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 14341ns | no significant difference | [-1026, +1572]ns | [13170, 16936] | no | 0.9774 | 0.8145 | 0 |
| nway | 42423ns | +27703.8ns (+194.7%) | [+24658, +34514]ns | [39384, 49455] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 19470ns | no significant difference | [-2613, +17702]ns | [10026, 33924] | no | 0.9774 | 0.8145 | 0 |
| std_parallel | 41104ns | +24592.6ns (+172.8%) | [+22751, +29325]ns | [36518, 43446] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 14066ns | +2.9% | -12.0% | +4.8% | +230.4% | -1.5% | +215.4% |
| 1 | 2 | 14984ns | +16.9% | -0.2% | -12.3% | +157.7% | +5.4% | +166.5% |
| 1 | 3 | 13808ns | +18.6% | +16.7% | +11.1% | +237.9% | +138.1% | +164.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.638 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.412 | moderate- |
| kernel_nway | -0.030 | ok |
| kernel_seq | -0.620 | HIGH- (thermal bounce) |
| nway | -0.666 | HIGH- (thermal bounce) |
| sequential | -0.116 | ok |
| std_parallel | -0.004 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 68231.6ns | 15479.8ns | 440.8% | HIGH |
| head_tail_dyn | 79625.5ns | 16458.5ns | 483.8% | HIGH |
| kernel_nway | 77092.2ns | 15925.4ns | 484.1% | HIGH |
| kernel_seq | 74225.7ns | 15287.0ns | 485.5% | HIGH |
| nway | 98210.6ns | 47223.2ns | 208.0% | HIGH |
| sequential | 55766.3ns | 22163.0ns | 251.6% | HIGH |
| std_parallel | 94954.2ns | 43308.5ns | 219.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 12578.8-19209.7 ns)
  12578.8 |
  12910.3 |########################################
  13241.9 |########################################
  13573.4 |####################
  13905.0 |########################################
  14236.5 |########################################
  14568.1 |####################
  14899.6 |
  15231.2 |####################
  15562.7 |
  15894.3 |########################################
  16225.8 |####################
  16557.4 |####################
  16888.9 |
  17220.5 |
  17552.0 |
  17883.5 |
  18215.1 |
  18546.6 |
  18878.2 |
  (2 below, 1 above range)

head_tail_dyn (n=18, range 13549.7-19164.9 ns)
  13549.7 |####################
  13830.4 |
  14111.2 |####################
  14391.9 |########################################
  14672.7 |
  14953.5 |
  15234.2 |########################################
  15515.0 |
  15795.7 |
  16076.5 |
  16357.3 |########################################
  16638.0 |####################
  16918.8 |
  17199.5 |########################################
  17480.3 |
  17761.1 |
  18041.8 |
  18322.6 |########################################
  18603.3 |
  18884.1 |
  (2 below, 3 above range)

kernel_nway (n=18, range 11939.1-20344.9 ns)
  11939.1 |########################################
  12359.4 |####################
  12779.7 |####################
  13200.0 |####################
  13620.3 |####################
  14040.6 |####################
  14460.9 |####################
  14881.1 |####################
  15301.4 |####################
  15721.7 |
  16142.0 |####################
  16562.3 |########################################
  16982.6 |
  17402.9 |########################################
  17823.2 |####################
  18243.4 |
  18663.7 |
  19084.0 |
  19504.3 |
  19924.6 |
  (1 below, 1 above range)

kernel_seq (n=18, range 11789.2-18889.9 ns)
  11789.2 |####################
  12144.2 |
  12499.2 |########################################
  12854.3 |####################
  13209.3 |########################################
  13564.4 |####################
  13919.4 |####################
  14274.4 |
  14629.5 |####################
  14984.5 |
  15339.5 |####################
  15694.6 |####################
  16049.6 |####################
  16404.7 |
  16759.7 |
  17114.7 |
  17469.8 |####################
  17824.8 |
  18179.9 |
  18534.9 |
  (1 below, 4 above range)

nway (n=18, range 35904.2-61811.6 ns)
  35904.2 |########################################
  37199.6 |
  38494.9 |########################################
  39790.3 |####################
  41085.7 |########################################
  42381.1 |########################################
  43676.4 |
  44971.8 |####################
  46267.2 |
  47562.5 |
  48857.9 |########################################
  50153.3 |####################
  51448.7 |
  52744.0 |####################
  54039.4 |
  55334.8 |
  56630.1 |
  57925.5 |
  59220.9 |
  60516.2 |
  (2 below, 2 above range)

sequential (n=18, range 9755.1-35572.0 ns)
   9755.1 |########################################
  11045.9 |
  12336.8 |######
  13627.6 |
  14918.4 |
  16209.3 |
  17500.1 |######
  18791.0 |
  20081.8 |######
  21372.7 |
  22663.5 |
  23954.4 |
  25245.2 |
  26536.1 |
  27826.9 |
  29117.8 |
  30408.6 |
  31699.4 |#############
  32990.3 |######
  34281.1 |##########################
  (1 below, 1 above range)

std_parallel (n=18, range 35279.1-53417.2 ns)
  35279.1 |########################################
  36186.0 |
  37092.9 |####################
  37999.8 |
  38906.7 |
  39813.6 |##########
  40720.6 |####################
  41627.5 |####################
  42534.4 |##########
  43441.3 |##########
  44348.2 |
  45255.1 |
  46162.0 |##########
  47068.9 |
  47975.8 |
  48882.7 |
  49789.6 |
  50696.5 |
  51603.4 |####################
  52510.3 |
  (1 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=30.8% (high variance, measurements may be unstable)
- **head_tail**: bridge=465.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=505.0% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=32.1% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=480.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=489.1% of algo (FFI overhead may distort results)
- **nway**: CV=33.0% (high variance, measurements may be unstable)
- **nway**: bridge=217.6% of algo (FFI overhead may distort results)
- **sequential**: CV=52.3% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **sequential**: bridge=252.8% of algo (FFI overhead may distort results)
- **std_parallel**: CV=25.2% (high variance, measurements may be unstable)
- **std_parallel**: bridge=229.1% of algo (FFI overhead may distort results)

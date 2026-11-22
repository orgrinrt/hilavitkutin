# fold_light, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 11% faster than the next best (head_tail_dyn)

kernel_seq (13.17 us) leads head_tail_dyn (14.63 us) by 11%, a clear separation rather than a photo finish. CV 36.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### nway is an outlier: 3.4x slower than the field

nway (45.05 us) is 3.4x the fastest (13.17 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.58)

kernel_nway's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.4x the fastest

Fastest kernel_seq (13.17 us) to slowest nway (45.05 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_seq is inconsistent: worst-20% is 1.9x its best-20%

kernel_seq's best 20% of batches run at 11.57 us but its worst 20% at 22.35 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 13174.0 ns median (-15.8% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 3.42x (fastest 13174.0 ns, slowest 45052.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 1 |
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
| head_tail | 18294ns | 17123ns | 13466ns | 15695ns | 24607ns | base |
| head_tail_dyn | 15873ns | 15319ns | 13212ns | 15069ns | 18409ns | -13.23% |
| kernel_nway | 21399ns | 20679ns | 18790ns | 20249ns | 24430ns | +16.97% |
| kernel_seq | 16999ns | 14023ns | 12195ns | 13809ns | 24186ns | -7.08% |
| nway | 47377ns | 45774ns | 38497ns | 44425ns | 56245ns | +158.98% |
| sequential | 28137ns | 27866ns | 21977ns | 26869ns | 33119ns | +53.80% |
| std_parallel | 42656ns | 43749ns | 34257ns | 40885ns | 49512ns | +133.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 17199ns | 12690ns | 23061ns | base | 0.238 |
| head_tail_dyn | 14798ns | 12552ns | 17184ns | -13.96% | 0.277 |
| kernel_nway | 20100ns | 17951ns | 22505ns | +16.87% | 0.204 |
| kernel_seq | 15769ns | 11570ns | 22345ns | -8.31% | 0.260 |
| nway | 46654ns | 37968ns | 55343ns | +171.27% | 0.088 |
| sequential | 27594ns | 21545ns | 32477ns | +60.44% | 0.148 |
| std_parallel | 41827ns | 33690ns | 48368ns | +143.20% | 0.098 |

## Performance model

- Peak throughput: **0.354 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.262 | 73.9% |
| head_tail_dyn | 0.280 | 79.1% |
| kernel_nway | 0.209 | 59.2% |
| kernel_seq | 0.311 | 87.8% |
| nway | 0.091 | 25.7% |
| sequential | 0.150 | 42.3% |
| std_parallel | 0.096 | 27.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 18294ns | 18294ns | base |
| head_tail_dyn | 15873ns | 15873ns | -13.23% |
| kernel_nway | 21399ns | 21399ns | +16.97% |
| kernel_seq | 16999ns | 16999ns | -7.08% |
| nway | 47377ns | 47377ns | +158.98% |
| sequential | 28137ns | 28137ns | +53.80% |
| std_parallel | 42656ns | 42656ns | +133.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 15647ns | base | --- | [13583, 17777] | --- | --- | --- | --- |
| head_tail_dyn | 14631ns | no significant difference | [-2336, +349]ns | [13271, 16906] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 19556ns | +5017.9ns (+32.1%) | [+1423, +6848]ns | [18362, 21168] | YES | 0.0113 | 0.0075 | 0 |
| kernel_seq | 13174ns | no significant difference | [-5414, +7599]ns | [12002, 20973] | no | 0.2855 | 0.2379 | 0 |
| nway | 45053ns | +28471.8ns (+182.0%) | [+27018, +29760]ns | [41564, 46965] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 27333ns | +11687.1ns (+74.7%) | [+9446, +12603]ns | [24778, 30840] | YES | 0.0003 | 0.0001 | 0 |
| std_parallel | 42817ns | +25507.0ns (+163.0%) | [+22315, +27087]ns | [36981, 45051] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 12792ns | +14.4% | +52.9% | +70.3% | +218.8% | +72.0% | +188.1% |
| 1 | 2 | 15922ns | -18.3% | +37.3% | -26.9% | +184.5% | +83.3% | +162.5% |
| 1 | 3 | 18135ns | -6.4% | +0.5% | -27.4% | +160.6% | +70.9% | +147.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.006 | ok |
| head_tail_dyn | -0.437 | moderate- |
| kernel_nway | -0.579 | HIGH- (thermal bounce) |
| kernel_seq | -0.253 | moderate- |
| nway | -0.033 | ok |
| sequential | -0.071 | ok |
| std_parallel | -0.013 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 16542.9ns | 17198.6ns | 96.2% | HIGH |
| head_tail_dyn | 15610.8ns | 14798.2ns | 105.5% | HIGH |
| kernel_nway | 21990.1ns | 20100.4ns | 109.4% | HIGH |
| kernel_seq | 16376.6ns | 15769.4ns | 103.9% | HIGH |
| nway | 46441.9ns | 46654.3ns | 99.5% | HIGH |
| sequential | 28580.6ns | 27593.6ns | 103.6% | HIGH |
| std_parallel | 41138.4ns | 41826.6ns | 98.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 12690.5-23060.9 ns)
  12690.5 |########################################
  13209.0 |#############
  13727.5 |########################################
  14246.1 |#############
  14764.6 |
  15283.1 |
  15801.6 |
  16320.1 |##########################
  16838.6 |##########################
  17357.2 |
  17875.7 |#############
  18394.2 |
  18912.7 |#############
  19431.2 |#############
  19949.7 |
  20468.3 |
  20986.8 |
  21505.3 |#############
  22023.8 |
  22542.3 |
  (1 below, 1 above range)

head_tail_dyn (n=18, range 12551.7-17184.0 ns)
  12551.7 |##########################
  12783.3 |#############
  13014.9 |
  13246.6 |#############
  13478.2 |
  13709.8 |##########################
  13941.4 |
  14173.0 |
  14404.6 |#############
  14636.3 |##########################
  14867.9 |#############
  15099.5 |
  15331.1 |
  15562.7 |
  15794.3 |
  16026.0 |
  16257.6 |
  16489.2 |
  16720.8 |########################################
  16952.4 |##########################
  (2 below, 1 above range)

kernel_nway (n=18, range 17951.2-22505.5 ns)
  17951.2 |########################################
  18178.9 |#############
  18406.6 |#############
  18634.3 |
  18862.0 |
  19089.7 |##########################
  19317.5 |#############
  19545.2 |#############
  19772.9 |
  20000.6 |##########################
  20228.3 |
  20456.0 |
  20683.7 |
  20911.5 |
  21139.2 |########################################
  21366.9 |
  21594.6 |
  21822.3 |
  22050.0 |
  22277.8 |#############
  (1 below, 2 above range)

kernel_seq (n=18, range 11570.3-22345.3 ns)
  11570.3 |########################################
  12109.1 |##########
  12647.8 |##############################
  13186.6 |##############################
  13725.3 |
  14264.1 |
  14802.8 |
  15341.6 |
  15880.3 |
  16419.1 |
  16957.8 |
  17496.6 |
  18035.3 |
  18574.1 |
  19112.8 |
  19651.6 |
  20190.3 |
  20729.1 |##############################
  21267.8 |
  21806.6 |
  (1 below, 3 above range)

nway (n=18, range 37967.7-55343.3 ns)
  37967.7 |#############
  38836.4 |
  39705.2 |#############
  40574.0 |##########################
  41442.8 |#############
  42311.6 |#############
  43180.3 |
  44049.1 |#############
  44917.9 |########################################
  45786.7 |##########################
  46655.5 |#############
  47524.2 |#############
  48393.0 |#############
  49261.8 |
  50130.6 |
  50999.4 |
  51868.1 |
  52736.9 |
  53605.7 |#############
  54474.5 |
  (1 below, 1 above range)

sequential (n=18, range 21544.8-32476.6 ns)
  21544.8 |########################################
  22091.4 |
  22637.9 |
  23184.5 |
  23731.1 |#############
  24277.7 |
  24824.3 |
  25370.9 |#############
  25917.5 |##########################
  26464.1 |#############
  27010.7 |
  27557.3 |
  28103.9 |##########################
  28650.4 |
  29197.0 |
  29743.6 |#############
  30290.2 |
  30836.8 |##########################
  31383.4 |##########################
  31930.0 |
  (1 below, 2 above range)

std_parallel (n=18, range 33690.0-48368.5 ns)
  33690.0 |
  34423.9 |#############
  35157.8 |##########################
  35891.8 |
  36625.7 |
  37359.6 |
  38093.5 |#############
  38827.5 |
  39561.4 |##########################
  40295.3 |
  41029.2 |
  41763.2 |
  42497.1 |########################################
  43231.0 |##########################
  43964.9 |
  44698.9 |
  45432.8 |
  46166.7 |#############
  46900.6 |#############
  47634.6 |#############
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=38.4% (high variance, measurements may be unstable)
- **head_tail**: bridge=104.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=102.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=102.9% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=30.3% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=104.1% of algo (FFI overhead may distort results)
- **nway**: CV=22.9% (high variance, measurements may be unstable)
- **nway**: bridge=102.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=104.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=97.8% of algo (FFI overhead may distort results)

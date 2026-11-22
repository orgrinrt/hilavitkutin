# fold_light, far above cache, W = 8

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (head_tail) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline head_tail has the worst median (6.05 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest std_parallel at 1.86 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### head_tail is an outlier: 3.3x slower than the field

head_tail (6.05 ms) is 3.3x the fastest (1.86 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_parallel is fastest but the noisiest (CV 62.9%)

std_parallel wins on median (1.86 ms) yet has the highest variance (CV 62.9%), while kernel_seq is the steadiest (CV 1.3%, 4.16 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### std_parallel shows alternating (throttle bounce) (autocorr -0.67)

std_parallel's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {std_parallel} vs {head_tail_dyn, kernel_nway, sequential, kernel_seq, nway, head_tail} (82% apart)

The field splits into a fast tier {std_parallel} and a slow tier {head_tail_dyn, kernel_nway, sequential, kernel_seq, nway, head_tail} with a 82% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.3x the fastest

Fastest std_parallel (1.86 ms) to slowest head_tail (6.05 ms): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 2.8x its best-20%

kernel_nway's best 20% of batches run at 1.74 ms but its worst 20% at 4.87 ms (2.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_parallel** at 1858074.7 ns median (-69.3% vs baseline)
- Spread: 3.25x (fastest 1858074.7 ns, slowest 6046322.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 8 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 5642994ns | 6051477ns | 2841140ns | 5099515ns | 7859140ns | base |
| head_tail_dyn | 3376462ns | 3393873ns | 3063700ns | 3341950ns | 3584613ns | -40.17% |
| kernel_nway | 3504003ns | 3463964ns | 1744527ns | 3169396ns | 4885652ns | -37.91% |
| kernel_seq | 4173679ns | 4166836ns | 4103176ns | 4151168ns | 4242696ns | -26.04% |
| nway | 4675143ns | 4434956ns | 3414095ns | 4360276ns | 5777969ns | -17.15% |
| sequential | 4080109ns | 4139766ns | 3738837ns | 4107635ns | 4209455ns | -27.70% |
| std_parallel | 2570087ns | 1860996ns | 1808802ns | 1858092ns | 4018721ns | -54.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 5632960ns | 2828448ns | 7845364ns | base | 2.978 |
| head_tail_dyn | 3369337ns | 3061115ns | 3576842ns | -40.19% | 4.979 |
| kernel_nway | 3495205ns | 1738004ns | 4873221ns | -37.95% | 4.800 |
| kernel_seq | 4168144ns | 4101286ns | 4237031ns | -26.00% | 4.025 |
| nway | 4669854ns | 3411337ns | 5771599ns | -17.10% | 3.593 |
| sequential | 4078701ns | 3737455ns | 4208236ns | -27.59% | 4.113 |
| std_parallel | 2565057ns | 1806232ns | 4009790ns | -54.46% | 6.541 |

## Performance model

- Peak throughput: **9.653 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 2.775 | 28.7% |
| head_tail_dyn | 4.959 | 51.4% |
| kernel_nway | 4.854 | 50.3% |
| kernel_seq | 4.035 | 41.8% |
| nway | 3.788 | 39.2% |
| sequential | 4.054 | 42.0% |
| std_parallel | 9.029 | 93.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 5642994ns | 5642994ns | base |
| head_tail_dyn | 3376462ns | 3376462ns | -40.17% |
| kernel_nway | 3504003ns | 3504003ns | -37.91% |
| kernel_seq | 4173679ns | 4173679ns | -26.04% |
| nway | 4675143ns | 4675143ns | -17.15% |
| sequential | 4080109ns | 4080109ns | -27.70% |
| std_parallel | 2570087ns | 2570087ns | -54.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 6046323ns | base | --- | [3007194, 7845364] | --- | --- | --- | --- |
| head_tail_dyn | 3382894ns | no significant difference | [-4360497, +375700]ns | [3148275, 3576842] | no | 0.6875 | 0.6875 | 0 |
| kernel_nway | 3456232ns | no significant difference | [-4398801, +635349]ns | [2156162, 4873221] | no | 0.6563 | 0.2188 | 0 |
| kernel_seq | 4157639ns | no significant difference | [-3707828, +1204110]ns | [4109761, 4237031] | no | 0.6875 | 0.6875 | 0 |
| nway | 4429546ns | no significant difference | [-3877477, +2764405]ns | [3808417, 5771599] | no | 0.6875 | 0.6875 | 0 |
| sequential | 4138573ns | no significant difference | [-3676800, +1119011]ns | [3889295, 4208236] | no | 0.6875 | 0.6875 | 0 |
| std_parallel | 1858075ns | no significant difference | [-5993753, +418569]ns | [1827307, 4009790] | no | 0.6563 | 0.2188 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 7845364ns | -54.4% | -55.9% | -47.3% | -49.4% | -46.9% | -76.4% |
| 1 | 2 | 4248465ns | -25.3% | -49.2% | -0.3% | +16.2% | -8.5% | -5.6% |
| 1 | 3 | 4805052ns | -30.1% | +1.4% | -14.1% | +6.2% | -13.0% | -61.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.256 | moderate- |
| head_tail_dyn | -0.471 | moderate- |
| kernel_nway | -0.485 | moderate- |
| kernel_seq | -0.664 | HIGH- (thermal bounce) |
| nway | -0.096 | ok |
| sequential | -0.666 | HIGH- (thermal bounce) |
| std_parallel | -0.667 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 5303160.8ns | 5632960.4ns | 94.1% | HIGH |
| head_tail_dyn | 3410541.3ns | 3369336.9ns | 101.2% | HIGH |
| kernel_nway | 3477997.2ns | 3495205.2ns | 99.5% | HIGH |
| kernel_seq | 4186882.9ns | 4168143.8ns | 100.4% | HIGH |
| nway | 4837477.7ns | 4669854.1ns | 103.6% | HIGH |
| sequential | 4140555.7ns | 4078701.4ns | 101.5% | HIGH |
| std_parallel | 2521499.1ns | 2565057.2ns | 98.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 2828448.0-7845364.3 ns)
  2828448.0 |####################
  3079293.8 |####################
  3330139.6 |
  3580985.5 |
  3831831.3 |
  4082677.1 |
  4333522.9 |
  4584368.7 |
  4835214.5 |
  5086060.4 |
  5336906.2 |
  5587752.0 |####################
  5838597.8 |
  6089443.6 |
  6340289.4 |########################################
  6591135.3 |
  6841981.1 |
  7092826.9 |
  7343672.7 |
  7594518.5 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 3061114.8-3576842.0 ns)
  3061114.8 |########################################
  3086901.2 |
  3112687.5 |
  3138473.9 |
  3164260.2 |
  3190046.6 |
  3215833.0 |########################################
  3241619.3 |
  3267405.7 |########################################
  3293192.0 |
  3318978.4 |
  3344764.8 |
  3370551.1 |
  3396337.5 |
  3422123.8 |
  3447910.2 |
  3473696.6 |########################################
  3499482.9 |########################################
  3525269.3 |
  3551055.6 |
  (0 below, 1 above range)

kernel_nway (n=6, range 1738004.5-4873221.3 ns)
  1738004.5 |####################
  1894765.3 |
  2051526.2 |
  2208287.0 |
  2365047.9 |
  2521808.7 |####################
  2678569.5 |
  2835330.4 |
  2992091.2 |
  3148852.1 |
  3305612.9 |########################################
  3462373.7 |
  3619134.6 |
  3775895.4 |
  3932656.3 |
  4089417.1 |####################
  4246177.9 |
  4402938.8 |
  4559699.6 |
  4716460.5 |
  (0 below, 1 above range)

kernel_seq (n=6, range 4101286.5-4237031.2 ns)
  4101286.5 |####################
  4108073.7 |
  4114861.0 |####################
  4121648.2 |
  4128435.4 |
  4135222.7 |
  4142009.9 |
  4148797.1 |
  4155584.4 |########################################
  4162371.6 |
  4169158.8 |
  4175946.1 |
  4182733.3 |
  4189520.5 |
  4196307.8 |
  4203095.0 |
  4209882.2 |####################
  4216669.5 |
  4223456.7 |
  4230243.9 |
  (0 below, 1 above range)

nway (n=6, range 3411337.2-5771599.4 ns)
  3411337.2 |########################################
  3529350.3 |
  3647363.4 |
  3765376.5 |
  3883389.6 |
  4001402.8 |
  4119415.9 |########################################
  4237429.0 |########################################
  4355442.1 |
  4473455.2 |########################################
  4591468.3 |
  4709481.4 |
  4827494.5 |
  4945507.6 |
  5063520.7 |
  5181533.9 |
  5299547.0 |
  5417560.1 |
  5535573.2 |
  5653586.3 |########################################
  (0 below, 1 above range)

sequential (n=6, range 3737454.8-4208235.9 ns)
  3737454.8 |########################################
  3760993.9 |
  3784532.9 |
  3808072.0 |
  3831611.0 |
  3855150.1 |
  3878689.1 |
  3902228.2 |
  3925767.2 |
  3949306.3 |
  3972845.4 |
  3996384.4 |
  4019923.5 |########################################
  4043462.5 |
  4067001.6 |
  4090540.6 |
  4114079.7 |########################################
  4137618.7 |########################################
  4161157.8 |
  4184696.8 |########################################
  (0 below, 1 above range)

std_parallel (n=6, range 1806232.0-4009789.8 ns)
  1806232.0 |########################################
  1916409.9 |
  2026587.8 |
  2136765.7 |
  2246943.5 |
  2357121.4 |
  2467299.3 |
  2577477.2 |
  2687655.1 |
  2797833.0 |
  2908010.9 |
  3018188.8 |##########
  3128366.6 |
  3238544.5 |
  3348722.4 |
  3458900.3 |
  3569078.2 |
  3679256.1 |
  3789434.0 |
  3899611.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=38.2% (high variance, measurements may be unstable)
- **head_tail**: bridge=93.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.9% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=34.3% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=99.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.8% of algo (FFI overhead may distort results)
- **nway**: bridge=109.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=101.3% of algo (FFI overhead may distort results)
- **std_parallel**: CV=45.5% (high variance, measurements may be unstable)
- **std_parallel**: bridge=101.2% of algo (FFI overhead may distort results)

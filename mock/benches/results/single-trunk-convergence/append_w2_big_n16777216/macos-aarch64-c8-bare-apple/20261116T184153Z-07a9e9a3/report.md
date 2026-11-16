# append, far above cache, W = 2

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential is an outlier: 4.3x slower than the field

sequential (12.15 ms) is 4.3x the fastest (2.85 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.64)

kernel_nway's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.3x the fastest

Fastest head_tail_dyn (2.85 ms) to slowest sequential (12.15 ms): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### std_parallel is inconsistent: worst-20% is 1.5x its best-20%

std_parallel's best 20% of batches run at 2.41 ms but its worst 20% at 3.70 ms (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 2852073.0 ns median (-8.8% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 4.26x (fastest 2852073.0 ns, slowest 12148859.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, far above cache, W = 2 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 3159203ns | 3131594ns | 2686490ns | 2987184ns | 3653588ns | base |
| head_tail_dyn | 2889464ns | 2857214ns | 2295771ns | 2819045ns | 3291938ns | -8.54% |
| kernel_nway | 3171128ns | 3176870ns | 2771281ns | 3129538ns | 3433438ns | +0.38% |
| kernel_seq | 2947057ns | 3003427ns | 2482729ns | 2946003ns | 3180802ns | -6.72% |
| nway | 5929589ns | 5891656ns | 5801875ns | 5870378ns | 6082260ns | +87.69% |
| sequential | 12170582ns | 12152896ns | 12034812ns | 12143260ns | 12279448ns | +285.24% |
| std_parallel | 3158931ns | 3118792ns | 2412667ns | 3047500ns | 3699208ns | -0.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 3155325ns | 2680969ns | 3650510ns | base | 5.317 |
| head_tail_dyn | 2885733ns | 2292927ns | 3289026ns | -8.54% | 5.814 |
| kernel_nway | 3167634ns | 2766896ns | 3429875ns | +0.39% | 5.296 |
| kernel_seq | 2943623ns | 2480333ns | 3177411ns | -6.71% | 5.700 |
| nway | 5925816ns | 5798031ns | 6078693ns | +87.80% | 2.831 |
| sequential | 12166530ns | 12031219ns | 12275719ns | +285.59% | 1.379 |
| std_parallel | 3155519ns | 2409802ns | 3696068ns | +0.01% | 5.317 |

## Performance model

- Peak throughput: **7.317 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 5.365 | 73.3% |
| head_tail_dyn | 5.882 | 80.4% |
| kernel_nway | 5.286 | 72.2% |
| kernel_seq | 5.593 | 76.4% |
| nway | 2.849 | 38.9% |
| sequential | 1.381 | 18.9% |
| std_parallel | 5.385 | 73.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 3159203ns | 3159203ns | base |
| head_tail_dyn | 2889464ns | 2889464ns | -8.54% |
| kernel_nway | 3171128ns | 3171128ns | +0.38% |
| kernel_seq | 2947057ns | 2947057ns | -6.72% |
| nway | 5929589ns | 5929589ns | +87.69% |
| sequential | 12170582ns | 12170582ns | +285.24% |
| std_parallel | 3158931ns | 3158931ns | -0.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 3127443ns | base | --- | [2688021, 3650510] | --- | --- | --- | --- |
| head_tail_dyn | 2852073ns | no significant difference | [-816115, +371464]ns | [2516099, 3289026] | no | 0.8250 | 0.6875 | 0 |
| kernel_nway | 3173630ns | no significant difference | [-472474, +540156]ns | [2899396, 3429875] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 2999714ns | no significant difference | [-640641, +311693]ns | [2653745, 3177411] | no | 0.8250 | 0.6875 | 0 |
| nway | 5888333ns | +2682979.2ns (+85.8%) | [+2406521, +3221974]ns | [5810422, 6078693] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| sequential | 12148859ns | +9148276.1ns (+292.5%) | [+8441427, +9443911]ns | [12075010, 12275719] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| std_parallel | 3115594ns | no significant difference | [-560297, +820115]ns | [2654896, 3696068] | no | 0.8250 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 3018344ns | -6.2% | +0.0% | +2.1% | +96.7% | +302.1% | +3.2% |
| 1 | 2 | 3320188ns | -0.9% | +2.0% | -6.8% | +81.6% | +264.1% | +11.3% |
| 1 | 3 | 3127443ns | -18.9% | -1.0% | -15.1% | +85.8% | +292.5% | -15.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.582 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.566 | HIGH- (thermal bounce) |
| kernel_nway | -0.640 | HIGH- (thermal bounce) |
| kernel_seq | -0.181 | ok |
| nway | -0.447 | moderate- |
| sequential | -0.323 | moderate- |
| std_parallel | -0.537 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 1/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 95515817.7ns | 3155324.7ns | 3027.1% | HIGH |
| head_tail_dyn | 95105263.9ns | 2885732.6ns | 3295.7% | HIGH |
| kernel_nway | 94910067.7ns | 3167633.7ns | 2996.2% | HIGH |
| kernel_seq | 94712151.0ns | 2943623.2ns | 3217.5% | HIGH |
| nway | 97727665.0ns | 5925816.0ns | 1649.2% | HIGH |
| sequential | 104069057.3ns | 12166529.5ns | 855.4% | HIGH |
| std_parallel | 96274286.5ns | 3155519.1ns | 3051.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 2680968.8-3650510.5 ns)
  2680968.8 |########################################
  2729445.9 |
  2777923.0 |
  2826400.0 |
  2874877.1 |
  2923354.2 |
  2971831.3 |
  3020308.4 |
  3068785.5 |####################
  3117262.5 |
  3165739.6 |####################
  3214216.7 |
  3262693.8 |
  3311170.9 |####################
  3359648.0 |
  3408125.0 |
  3456602.1 |
  3505079.2 |
  3553556.3 |
  3602033.4 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 2292927.1-3289026.0 ns)
  2292927.1 |########################################
  2342732.0 |
  2392537.0 |
  2442341.9 |
  2492146.9 |
  2541951.8 |
  2591756.8 |
  2641561.7 |
  2691366.7 |########################################
  2741171.6 |########################################
  2790976.6 |
  2840781.5 |
  2890586.5 |########################################
  2940391.4 |
  2990196.4 |
  3040001.3 |
  3089806.3 |
  3139611.2 |
  3189416.2 |########################################
  3239221.1 |
  (0 below, 1 above range)

kernel_nway (n=6, range 2766895.8-3429875.0 ns)
  2766895.8 |########################################
  2800044.8 |
  2833193.7 |
  2866342.7 |
  2899491.6 |
  2932640.6 |
  2965789.6 |
  2998938.5 |########################################
  3032087.5 |
  3065236.4 |
  3098385.4 |
  3131534.4 |########################################
  3164683.3 |########################################
  3197832.3 |
  3230981.2 |
  3264130.2 |########################################
  3297279.2 |
  3330428.1 |
  3363577.1 |
  3396726.0 |
  (0 below, 1 above range)

kernel_seq (n=6, range 2480333.3-3177411.5 ns)
  2480333.3 |########################################
  2515187.2 |
  2550041.1 |
  2584895.0 |
  2619748.9 |
  2654602.8 |
  2689456.7 |
  2724310.7 |
  2759164.6 |
  2794018.5 |########################################
  2828872.4 |
  2863726.3 |
  2898580.2 |########################################
  2933434.1 |
  2968288.0 |
  3003141.9 |
  3037995.8 |########################################
  3072849.7 |########################################
  3107703.6 |
  3142557.5 |
  (0 below, 1 above range)

nway (n=6, range 5798031.2-6078692.7 ns)
  5798031.2 |########################################
  5812064.3 |########################################
  5826097.4 |
  5840130.4 |
  5854163.5 |########################################
  5868196.6 |
  5882229.7 |
  5896262.7 |
  5910295.8 |########################################
  5924328.9 |
  5938362.0 |
  5952395.0 |########################################
  5966428.1 |
  5980461.2 |
  5994494.2 |
  6008527.3 |
  6022560.4 |
  6036593.5 |
  6050626.5 |
  6064659.6 |
  (0 below, 1 above range)

sequential (n=6, range 12031218.8-12275718.8 ns)
  12031218.8 |####################
  12043443.8 |
  12055668.8 |
  12067893.8 |
  12080118.8 |
  12092343.8 |
  12104568.8 |
  12116793.8 |####################
  12129018.8 |
  12141243.8 |########################################
  12153468.8 |
  12165693.8 |
  12177918.8 |
  12190143.8 |
  12202368.8 |
  12214593.8 |
  12226818.8 |
  12239043.8 |
  12251268.8 |####################
  12263493.8 |
  (0 below, 1 above range)

std_parallel (n=6, range 2409802.1-3696067.7 ns)
  2409802.1 |########################################
  2474115.4 |
  2538428.7 |
  2602741.9 |
  2667055.2 |
  2731368.5 |
  2795681.8 |
  2859995.1 |########################################
  2924308.3 |
  2988621.6 |########################################
  3052934.9 |
  3117248.2 |
  3181561.5 |########################################
  3245874.7 |
  3310188.0 |
  3374501.3 |
  3438814.6 |
  3503127.9 |
  3567441.1 |########################################
  3631754.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=3040.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3341.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=2978.6% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3153.3% of algo (FFI overhead may distort results)
- **nway**: bridge=1658.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=856.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=3037.8% of algo (FFI overhead may distort results)

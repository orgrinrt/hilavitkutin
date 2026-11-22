# fold_heavy, frames of tens of milliseconds, W = 1

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 2.2x slower than the field

std_parallel (6.81 ms) is 2.2x the fastest (3.05 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_parallel shows alternating (throttle bounce) (autocorr -0.66)

std_parallel's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential, kernel_seq, head_tail, kernel_nway, head_tail_dyn, nway} vs {std_parallel} (113% apart)

The field splits into a fast tier {sequential, kernel_seq, head_tail, kernel_nway, head_tail_dyn, nway} and a slow tier {std_parallel} with a 113% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: sequential** at 3054120.6 ns median (-3.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.23x (fastest 3054120.6 ns, slowest 6807525.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, frames of tens of milliseconds, W = 1 |
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
| head_tail | 3173258ns | 3159615ns | 3125160ns | 3149276ns | 3233280ns | base |
| head_tail_dyn | 3167504ns | 3197017ns | 3016574ns | 3136917ns | 3288850ns | -0.18% |
| kernel_nway | 3192203ns | 3189447ns | 3122044ns | 3170587ns | 3259706ns | +0.60% |
| kernel_seq | 3186144ns | 3141155ns | 3126719ns | 3138144ns | 3287857ns | +0.41% |
| nway | 3191862ns | 3196431ns | 3172438ns | 3188882ns | 3206046ns | +0.59% |
| sequential | 3045332ns | 3054474ns | 3006547ns | 3048238ns | 3060364ns | -4.03% |
| std_parallel | 6953661ns | 6810074ns | 5597760ns | 6657082ns | 8076480ns | +119.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 3167892ns | 3124200ns | 3223945ns | base | 0.331 |
| head_tail_dyn | 3162136ns | 3014364ns | 3281231ns | -0.18% | 0.332 |
| kernel_nway | 3187983ns | 3121309ns | 3256324ns | +0.63% | 0.329 |
| kernel_seq | 3181436ns | 3125816ns | 3281132ns | +0.43% | 0.330 |
| nway | 3191187ns | 3171913ns | 3205351ns | +0.74% | 0.329 |
| sequential | 3044899ns | 3006086ns | 3059851ns | -3.88% | 0.344 |
| std_parallel | 6951704ns | 5595691ns | 8074524ns | +119.44% | 0.151 |

## Performance model

- Peak throughput: **0.349 Gops/s** (sequential; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.332 | 95.3% |
| head_tail_dyn | 0.329 | 94.2% |
| kernel_nway | 0.330 | 94.5% |
| kernel_seq | 0.334 | 95.9% |
| nway | 0.328 | 94.1% |
| sequential | 0.343 | 98.4% |
| std_parallel | 0.154 | 44.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 3173258ns | 3173258ns | base |
| head_tail_dyn | 3167504ns | 3167504ns | -0.18% |
| kernel_nway | 3192203ns | 3192203ns | +0.60% |
| kernel_seq | 3186144ns | 3186144ns | +0.41% |
| nway | 3191862ns | 3191862ns | +0.59% |
| sequential | 3045332ns | 3045332ns | -4.03% |
| std_parallel | 6953661ns | 6953661ns | +119.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 3153703ns | base | --- | [3126030, 3223945] | --- | --- | --- | --- |
| head_tail_dyn | 3189955ns | no significant difference | [-178599, +123233]ns | [3015221, 3281231] | no | 1.0000 | 0.6875 | 0 |
| kernel_nway | 3180965ns | no significant difference | [-42139, +101600]ns | [3126661, 3256324] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 3135348ns | no significant difference | [-48487, +88827]ns | [3127827, 3281132] | no | 1.0000 | 1.0000 | 0 |
| nway | 3195754ns | no significant difference | [-24720, +57967]ns | [3172458, 3205351] | no | 0.4375 | 0.2188 | 0 |
| sequential | 3054121ns | -101827.0ns (-3.2%) | [-191572, -75582]ns | [3020724, 3059851] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| std_parallel | 6807526ns | +3627371.4ns (+115.0%) | [+2847031, +4877031]ns | [5973061, 8074524] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 3157998ns | +3.9% | -0.1% | +2.5% | +0.9% | -3.2% | +132.1% |
| 1 | 2 | 3151860ns | +1.2% | +3.1% | -0.5% | +1.6% | -3.1% | +93.8% |
| 1 | 3 | 3193820ns | -5.6% | -1.0% | -0.7% | -0.3% | -5.4% | +132.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.250 | moderate- |
| head_tail_dyn | -0.021 | ok |
| kernel_nway | -0.664 | HIGH- (thermal bounce) |
| kernel_seq | -0.397 | moderate- |
| nway | -0.658 | HIGH- (thermal bounce) |
| sequential | -0.133 | ok |
| std_parallel | -0.664 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 3337933.3ns | 3167892.3ns | 105.4% | HIGH |
| head_tail_dyn | 3194802.1ns | 3162135.7ns | 101.0% | HIGH |
| kernel_nway | 3200349.9ns | 3187983.3ns | 100.4% | HIGH |
| kernel_seq | 3165963.8ns | 3181435.8ns | 99.5% | HIGH |
| nway | 3227654.4ns | 3191187.4ns | 101.1% | HIGH |
| sequential | 3045408.1ns | 3044898.6ns | 100.0% | HIGH |
| std_parallel | 5836341.2ns | 6951703.6ns | 84.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 3124199.6-3223944.5 ns)
  3124199.6 |########################################
  3129186.8 |####################
  3134174.1 |
  3139161.3 |
  3144148.6 |
  3149135.8 |
  3154123.1 |
  3159110.3 |
  3164097.6 |
  3169084.8 |
  3174072.1 |####################
  3179059.3 |
  3184046.6 |####################
  3189033.8 |
  3194021.1 |
  3199008.3 |
  3203995.6 |
  3208982.8 |
  3213970.1 |
  3218957.3 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 3014364.3-3281230.6 ns)
  3014364.3 |########################################
  3027707.6 |
  3041050.9 |
  3054394.2 |
  3067737.6 |
  3081080.9 |
  3094424.2 |
  3107767.5 |
  3121110.8 |
  3134454.1 |
  3147797.5 |####################
  3161140.8 |
  3174484.1 |
  3187827.4 |
  3201170.7 |
  3214514.0 |####################
  3227857.3 |
  3241200.7 |####################
  3254544.0 |
  3267887.3 |
  (0 below, 1 above range)

kernel_nway (n=6, range 3121309.3-3256324.0 ns)
  3121309.3 |########################################
  3128060.0 |########################################
  3134810.8 |
  3141561.5 |
  3148312.2 |
  3155063.0 |
  3161813.7 |
  3168564.4 |########################################
  3175315.2 |
  3182065.9 |########################################
  3188816.6 |########################################
  3195567.4 |
  3202318.1 |
  3209068.8 |
  3215819.6 |
  3222570.3 |
  3229321.0 |
  3236071.8 |
  3242822.5 |
  3249573.2 |
  (0 below, 1 above range)

kernel_seq (n=6, range 3125816.1-3281132.4 ns)
  3125816.1 |########################################
  3133581.9 |#############
  3141347.7 |
  3149113.5 |
  3156879.4 |
  3164645.2 |
  3172411.0 |
  3180176.8 |
  3187942.6 |
  3195708.4 |
  3203474.2 |
  3211240.0 |#############
  3219005.9 |
  3226771.7 |
  3234537.5 |
  3242303.3 |
  3250069.1 |
  3257834.9 |
  3265600.7 |
  3273366.5 |
  (0 below, 1 above range)

nway (n=6, range 3171912.6-3205351.0 ns)
  3171912.6 |########################################
  3173584.5 |
  3175256.4 |
  3176928.4 |
  3178600.3 |
  3180272.2 |
  3181944.1 |
  3183616.1 |
  3185288.0 |
  3186959.9 |
  3188631.8 |
  3190303.7 |
  3191975.7 |
  3193647.6 |####################
  3195319.5 |####################
  3196991.4 |
  3198663.4 |
  3200335.3 |####################
  3202007.2 |
  3203679.1 |
  (0 below, 1 above range)

sequential (n=6, range 3006085.8-3059851.0 ns)
  3006085.8 |####################
  3008774.1 |
  3011462.3 |
  3014150.6 |
  3016838.8 |
  3019527.1 |
  3022215.4 |
  3024903.6 |
  3027591.9 |
  3030280.2 |
  3032968.4 |####################
  3035656.7 |
  3038344.9 |
  3041033.2 |
  3043721.5 |
  3046409.7 |
  3049098.0 |####################
  3051786.3 |
  3054474.5 |
  3057162.8 |########################################
  (0 below, 1 above range)

std_parallel (n=6, range 5595691.4-8074524.3 ns)
  5595691.4 |########################################
  5719633.0 |
  5843574.7 |
  5967516.3 |
  6091458.0 |
  6215399.6 |
  6339341.3 |########################################
  6463282.9 |
  6587224.6 |########################################
  6711166.2 |
  6835107.9 |
  6959049.5 |########################################
  7082991.1 |
  7206932.8 |
  7330874.4 |
  7454816.1 |
  7578757.7 |########################################
  7702699.4 |
  7826641.0 |
  7950582.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=100.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.4% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=101.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.4% of algo (FFI overhead may distort results)
- **nway**: bridge=101.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=89.1% of algo (FFI overhead may distort results)

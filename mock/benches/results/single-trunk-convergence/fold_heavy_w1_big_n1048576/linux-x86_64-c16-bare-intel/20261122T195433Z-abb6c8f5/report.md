# fold_heavy, frames of tens of milliseconds, W = 1

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 2.0x slower than the field

std_parallel (6.26 ms) is 2.0x the fastest (3.06 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_parallel shows alternating (throttle bounce) (autocorr -0.67)

std_parallel's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential, kernel_seq, kernel_nway, head_tail_dyn, head_tail, nway} vs {std_parallel} (96% apart)

The field splits into a fast tier {sequential, kernel_seq, kernel_nway, head_tail_dyn, head_tail, nway} and a slow tier {std_parallel} with a 96% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### std_parallel is inconsistent: worst-20% is 1.6x its best-20%

std_parallel's best 20% of batches run at 4.74 ms but its worst 20% at 7.61 ms (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 3060408.3 ns median (-3.9% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.04x (fastest 3060408.3 ns, slowest 6256882.3 ns)

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
| Counter frequency | 2112788144 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 3176852ns | 3193074ns | 3134808ns | 3175279ns | 3200233ns | base |
| head_tail_dyn | 3178218ns | 3170202ns | 3083822ns | 3153409ns | 3262628ns | +0.04% |
| kernel_nway | 3177713ns | 3165602ns | 3130198ns | 3154737ns | 3235934ns | +0.03% |
| kernel_seq | 3155080ns | 3162045ns | 3091256ns | 3150957ns | 3193177ns | -0.69% |
| nway | 3196537ns | 3192302ns | 3173376ns | 3186285ns | 3223497ns | +0.62% |
| sequential | 3057829ns | 3060753ns | 3038369ns | 3058691ns | 3066265ns | -3.75% |
| std_parallel | 6412490ns | 6259938ns | 4737573ns | 6170636ns | 7612731ns | +101.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 3171093ns | 3134030ns | 3192951ns | base | 0.331 |
| head_tail_dyn | 3170655ns | 3083097ns | 3249222ns | -0.01% | 0.331 |
| kernel_nway | 3173851ns | 3129426ns | 3229473ns | +0.09% | 0.330 |
| kernel_seq | 3149794ns | 3090463ns | 3182465ns | -0.67% | 0.333 |
| nway | 3195929ns | 3172814ns | 3222818ns | +0.78% | 0.328 |
| sequential | 3057395ns | 3038015ns | 3065763ns | -3.59% | 0.343 |
| std_parallel | 6409978ns | 4735765ns | 7609519ns | +102.14% | 0.164 |

## Performance model

- Peak throughput: **0.345 Gops/s** (sequential; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.329 | 95.4% |
| head_tail_dyn | 0.332 | 96.1% |
| kernel_nway | 0.332 | 96.1% |
| kernel_seq | 0.332 | 96.2% |
| nway | 0.329 | 95.2% |
| sequential | 0.343 | 99.3% |
| std_parallel | 0.168 | 48.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 3176852ns | 3176852ns | base |
| head_tail_dyn | 3178218ns | 3178218ns | +0.04% |
| kernel_nway | 3177713ns | 3177713ns | +0.03% |
| kernel_seq | 3155080ns | 3155080ns | -0.69% |
| nway | 3196537ns | 3196537ns | +0.62% |
| sequential | 3057829ns | 3057829ns | -3.75% |
| std_parallel | 6412490ns | 6412490ns | +101.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 3183905ns | base | --- | [3136424, 3192951] | --- | --- | --- | --- |
| head_tail_dyn | 3161664ns | no significant difference | [-84155, +90238]ns | [3101079, 3249222] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 3161356ns | no significant difference | [-33336, +45567]ns | [3130725, 3229473] | no | 0.8250 | 0.6875 | 0 |
| kernel_seq | 3157740ns | -10627.2ns (-0.3%) | [-51829, -1440]ns | [3109178, 3182465] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| nway | 3191713ns | +33592.4ns (+1.1%) | [+4082, +36832]ns | [3173256, 3222818] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| sequential | 3060408ns | -118710.0ns (-3.7%) | [-146369, -76015]ns | [3046014, 3065763] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| std_parallel | 6256882ns | +3089419.9ns (+97.0%) | [+2201622, +4425614]ns | [5363534, 7609519] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 3161344ns | -0.2% | +1.2% | -0.7% | +1.2% | -3.3% | +117.6% |
| 1 | 2 | 3192951ns | -2.6% | -1.0% | -1.1% | +0.4% | -4.4% | +73.4% |
| 1 | 3 | 3158984ns | +2.9% | +0.1% | -0.2% | +0.8% | -3.1% | +115.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.664 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.372 | moderate- |
| kernel_nway | -0.175 | ok |
| kernel_seq | -0.281 | moderate- |
| nway | -0.408 | moderate- |
| sequential | -0.603 | HIGH- (thermal bounce) |
| std_parallel | -0.665 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 1/3, lost 1/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 3192501.7ns | 3171093.3ns | 100.7% | HIGH |
| head_tail_dyn | 3214205.3ns | 3170655.3ns | 101.4% | HIGH |
| kernel_nway | 3187230.7ns | 3173851.3ns | 100.4% | HIGH |
| kernel_seq | 3175260.2ns | 3149794.4ns | 100.8% | HIGH |
| nway | 3206559.8ns | 3195929.0ns | 100.3% | HIGH |
| sequential | 3056295.4ns | 3057395.1ns | 100.0% | HIGH |
| std_parallel | 6160991.3ns | 6409978.3ns | 96.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 3134030.5-3192951.0 ns)
  3134030.5 |####################
  3136976.5 |####################
  3139922.5 |
  3142868.6 |
  3145814.6 |
  3148760.6 |
  3151706.6 |
  3154652.7 |
  3157598.7 |
  3160544.7 |
  3163490.8 |
  3166436.8 |
  3169382.8 |
  3172328.8 |
  3175274.9 |
  3178220.9 |####################
  3181166.9 |
  3184112.9 |
  3187059.0 |########################################
  3190005.0 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 3083097.3-3249222.5 ns)
  3083097.3 |########################################
  3091403.6 |
  3099709.8 |
  3108016.1 |
  3116322.3 |########################################
  3124628.6 |
  3132934.9 |########################################
  3141241.1 |
  3149547.4 |
  3157853.6 |
  3166159.9 |
  3174466.2 |
  3182772.4 |########################################
  3191078.7 |
  3199384.9 |
  3207691.2 |
  3215997.5 |
  3224303.7 |
  3232610.0 |
  3240916.2 |########################################
  (0 below, 1 above range)

kernel_nway (n=6, range 3129426.3-3229472.8 ns)
  3129426.3 |########################################
  3134428.6 |####################
  3139430.9 |
  3144433.3 |
  3149435.6 |
  3154437.9 |
  3159440.2 |
  3164442.6 |
  3169444.9 |
  3174447.2 |
  3179449.5 |
  3184451.8 |####################
  3189454.2 |####################
  3194456.5 |
  3199458.8 |
  3204461.1 |
  3209463.5 |
  3214465.8 |
  3219468.1 |
  3224470.4 |
  (0 below, 1 above range)

kernel_seq (n=6, range 3090462.9-3182464.9 ns)
  3090462.9 |####################
  3095063.0 |
  3099663.1 |
  3104263.2 |
  3108863.3 |
  3113463.4 |
  3118063.5 |
  3122663.6 |
  3127263.7 |####################
  3131863.8 |####################
  3136463.9 |
  3141064.0 |
  3145664.1 |
  3150264.2 |
  3154864.3 |
  3159464.4 |
  3164064.5 |
  3168664.6 |
  3173264.7 |
  3177864.8 |########################################
  (0 below, 1 above range)

nway (n=6, range 3172814.4-3222818.5 ns)
  3172814.4 |########################################
  3175314.6 |
  3177814.8 |
  3180315.0 |
  3182815.2 |
  3185315.4 |
  3187815.6 |
  3190315.8 |####################
  3192816.0 |####################
  3195316.2 |
  3197816.5 |
  3200316.7 |
  3202816.9 |
  3205317.1 |
  3207817.3 |
  3210317.5 |
  3212817.7 |
  3215317.9 |
  3217818.1 |
  3220318.3 |####################
  (0 below, 1 above range)

sequential (n=6, range 3038014.9-3065762.9 ns)
  3038014.9 |####################
  3039402.3 |
  3040789.7 |
  3042177.1 |
  3043564.5 |
  3044951.9 |
  3046339.3 |
  3047726.7 |
  3049114.1 |
  3050501.5 |
  3051888.9 |
  3053276.3 |####################
  3054663.7 |
  3056051.1 |
  3057438.5 |####################
  3058825.9 |
  3060213.3 |
  3061600.7 |########################################
  3062988.1 |
  3064375.5 |
  (0 below, 1 above range)

std_parallel (n=6, range 4735764.8-7609518.8 ns)
  4735764.8 |########################################
  4879452.5 |
  5023140.2 |
  5166827.9 |
  5310515.6 |
  5454203.3 |
  5597891.0 |
  5741578.7 |
  5885266.4 |########################################
  6028954.1 |
  6172641.8 |########################################
  6316329.5 |########################################
  6460017.2 |
  6603704.9 |
  6747392.6 |
  6891080.3 |
  7034768.0 |
  7178455.7 |
  7322143.4 |########################################
  7465831.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=100.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=100.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.2% of algo (FFI overhead may distort results)
- **nway**: bridge=100.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=94.5% of algo (FFI overhead may distort results)

# append, far above cache, W = 1

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential is an outlier: 4.5x slower than the field

sequential (12.21 ms) is 4.5x the fastest (2.71 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn is fastest but the noisiest (CV 23.1%)

head_tail_dyn wins on median (2.71 ms) yet has the highest variance (CV 23.1%), while nway is the steadiest (CV 0.6%, 11.51 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### nway shows alternating (throttle bounce) (autocorr -0.66)

nway's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn, std_parallel, kernel_nway, head_tail, kernel_seq} vs {nway, sequential} (281% apart)

The field splits into a fast tier {head_tail_dyn, std_parallel, kernel_nway, head_tail, kernel_seq} and a slow tier {nway, sequential} with a 281% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.5x the fastest

Fastest head_tail_dyn (2.71 ms) to slowest sequential (12.21 ms): 4.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### head_tail_dyn is inconsistent: worst-20% is 1.6x its best-20%

head_tail_dyn's best 20% of batches run at 2.53 ms but its worst 20% at 3.95 ms (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 2708911.5 ns median (-5.3% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 4.51x (fastest 2708911.5 ns, slowest 12208802.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, far above cache, W = 1 |
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
| head_tail | 2911509ns | 2862844ns | 2537781ns | 2801000ns | 3264135ns | base |
| head_tail_dyn | 3073387ns | 2712698ns | 2534938ns | 2668764ns | 3949547ns | +5.56% |
| kernel_nway | 2892811ns | 2834010ns | 2524552ns | 2796493ns | 3221417ns | -0.64% |
| kernel_seq | 3107139ns | 3028870ns | 2892292ns | 2986993ns | 3394781ns | +6.72% |
| nway | 11510068ns | 11518594ns | 11420302ns | 11491101ns | 11583401ns | +295.33% |
| sequential | 12255233ns | 12212641ns | 12003823ns | 12168139ns | 12511578ns | +320.92% |
| std_parallel | 3044321ns | 2755172ns | 2529938ns | 2744156ns | 3751760ns | +4.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2907944ns | 2534010ns | 3260568ns | base | 5.769 |
| head_tail_dyn | 3070012ns | 2532000ns | 3946083ns | +5.57% | 5.465 |
| kernel_nway | 2889307ns | 2521281ns | 3217672ns | -0.64% | 5.807 |
| kernel_seq | 3102611ns | 2887729ns | 3390755ns | +6.69% | 5.407 |
| nway | 11506222ns | 11416417ns | 11579724ns | +295.68% | 1.458 |
| sequential | 12250554ns | 12000385ns | 12504620ns | +321.28% | 1.370 |
| std_parallel | 3040639ns | 2526688ns | 3747604ns | +4.56% | 5.518 |

## Performance model

- Peak throughput: **6.654 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 5.868 | 88.2% |
| head_tail_dyn | 6.193 | 93.1% |
| kernel_nway | 5.927 | 89.1% |
| kernel_seq | 5.548 | 83.4% |
| nway | 1.457 | 21.9% |
| sequential | 1.374 | 20.7% |
| std_parallel | 6.098 | 91.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 2911509ns | 2911509ns | base |
| head_tail_dyn | 3073387ns | 3073387ns | +5.56% |
| kernel_nway | 2892811ns | 2892811ns | -0.64% |
| kernel_seq | 3107139ns | 3107139ns | +6.72% |
| nway | 11510068ns | 11510068ns | +295.33% |
| sequential | 12255233ns | 12255233ns | +320.92% |
| std_parallel | 3044321ns | 3044321ns | +4.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2859120ns | base | --- | [2604146, 3260568] | --- | --- | --- | --- |
| head_tail_dyn | 2708911ns | no significant difference | [-352219, +733656]ns | [2555042, 3946083] | no | 0.6875 | 0.6875 | 0 |
| kernel_nway | 2830469ns | no significant difference | [-369552, +356162]ns | [2619781, 3217672] | no | 0.6875 | 0.6875 | 0 |
| kernel_seq | 3024130ns | no significant difference | [-78125, +502323]ns | [2892948, 3390755] | no | 0.6875 | 0.6875 | 0 |
| nway | 11514578ns | +8657640.6ns (+302.8%) | [+8264609, +8872583]ns | [11424365, 11579724] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| sequential | 12208802ns | +9330156.3ns (+326.3%) | [+8972021, +9725651]ns | [12038240, 12504620] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| std_parallel | 2751312ns | no significant difference | [-530620, +1016776]ns | [2623000, 3747604] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2604146ns | +4.0% | +3.1% | +15.9% | +340.7% | +373.1% | +27.5% |
| 1 | 2 | 2975786ns | +9.8% | +5.5% | -2.6% | +288.9% | +313.9% | +2.5% |
| 1 | 3 | 3143901ns | +2.9% | -9.5% | +7.9% | +264.8% | +285.4% | -12.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.030 | ok |
| head_tail_dyn | -0.198 | ok |
| kernel_nway | -0.587 | HIGH- (thermal bounce) |
| kernel_seq | -0.318 | moderate- |
| nway | -0.664 | HIGH- (thermal bounce) |
| sequential | -0.161 | ok |
| std_parallel | -0.001 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 95078736.1ns | 2907944.4ns | 3269.6% | HIGH |
| head_tail_dyn | 94640055.5ns | 3070012.1ns | 3082.7% | HIGH |
| kernel_nway | 95269454.8ns | 2889307.3ns | 3297.3% | HIGH |
| kernel_seq | 95020007.0ns | 3102611.1ns | 3062.6% | HIGH |
| nway | 103251720.5ns | 11506222.2ns | 897.4% | HIGH |
| sequential | 104295111.1ns | 12250553.8ns | 851.4% | HIGH |
| std_parallel | 94936897.6ns | 3040638.9ns | 3122.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 2534010.4-3260567.7 ns)
  2534010.4 |########################################
  2570338.3 |
  2606666.1 |
  2642994.0 |########################################
  2679321.9 |
  2715649.7 |
  2751977.6 |
  2788305.5 |########################################
  2824633.3 |
  2860961.2 |
  2897289.0 |########################################
  2933616.9 |
  2969944.8 |
  3006272.6 |########################################
  3042600.5 |
  3078928.4 |
  3115256.2 |
  3151584.1 |
  3187912.0 |
  3224239.8 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 2532000.0-3946083.3 ns)
  2532000.0 |########################################
  2602704.2 |####################
  2673408.3 |
  2744112.5 |####################
  2814816.7 |
  2885520.8 |
  2956225.0 |
  3026929.2 |
  3097633.3 |
  3168337.5 |
  3239041.6 |
  3309745.8 |
  3380450.0 |
  3451154.1 |
  3521858.3 |
  3592562.5 |
  3663266.6 |
  3733970.8 |
  3804675.0 |
  3875379.1 |####################
  (0 below, 1 above range)

kernel_nway (n=6, range 2521281.2-3217671.9 ns)
  2521281.2 |########################################
  2556100.7 |
  2590920.3 |
  2625739.8 |
  2660559.3 |
  2695378.9 |########################################
  2730198.4 |
  2765017.9 |
  2799837.5 |########################################
  2834657.0 |########################################
  2869476.5 |
  2904296.1 |
  2939115.6 |########################################
  2973935.1 |
  3008754.7 |
  3043574.2 |
  3078393.7 |
  3113213.3 |
  3148032.8 |
  3182852.3 |
  (0 below, 1 above range)

kernel_seq (n=6, range 2887729.2-3390755.2 ns)
  2887729.2 |########################################
  2912880.5 |
  2938031.8 |
  2963183.1 |
  2988334.4 |
  3013485.7 |
  3038637.0 |
  3063788.3 |
  3088939.6 |
  3114090.9 |
  3139242.2 |#############
  3164393.5 |#############
  3189544.8 |
  3214696.1 |
  3239847.4 |
  3264998.7 |
  3290150.0 |
  3315301.3 |
  3340452.6 |
  3365603.9 |
  (0 below, 1 above range)

nway (n=6, range 11416416.7-11579723.9 ns)
  11416416.7 |########################################
  11424582.1 |########################################
  11432747.4 |
  11440912.8 |
  11449078.1 |
  11457243.5 |
  11465408.9 |
  11473574.2 |
  11481739.6 |
  11489905.0 |
  11498070.3 |
  11506235.7 |########################################
  11514401.0 |########################################
  11522566.4 |
  11530731.8 |########################################
  11538897.1 |
  11547062.5 |
  11555227.9 |
  11563393.2 |
  11571558.6 |
  (0 below, 1 above range)

sequential (n=6, range 12000385.4-12504619.8 ns)
  12000385.4 |########################################
  12025597.1 |
  12050808.8 |
  12076020.6 |########################################
  12101232.3 |
  12126444.0 |
  12151655.7 |
  12176867.4 |########################################
  12202079.2 |
  12227290.9 |########################################
  12252502.6 |
  12277714.3 |
  12302926.0 |
  12328137.8 |
  12353349.5 |
  12378561.2 |
  12403772.9 |
  12428984.6 |########################################
  12454196.4 |
  12479408.1 |
  (0 below, 1 above range)

std_parallel (n=6, range 2526687.5-3747604.1 ns)
  2526687.5 |#############
  2587733.3 |
  2648779.2 |
  2709825.0 |########################################
  2770870.8 |
  2831916.7 |
  2892962.5 |
  2954008.3 |
  3015054.2 |
  3076100.0 |
  3137145.8 |
  3198191.7 |
  3259237.5 |
  3320283.3 |
  3381329.2 |#############
  3442375.0 |
  3503420.8 |
  3564466.7 |
  3625512.5 |
  3686558.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=3324.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=20.4% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=3493.4% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=3365.6% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3143.5% of algo (FFI overhead may distort results)
- **nway**: bridge=897.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=853.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=3447.0% of algo (FFI overhead may distort results)

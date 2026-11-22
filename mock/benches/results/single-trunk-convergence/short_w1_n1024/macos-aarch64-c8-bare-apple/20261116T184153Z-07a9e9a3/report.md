# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 6.9x slower than the field

std_parallel (18.61 us) is 6.9x the fastest (2.69 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (head_tail, kernel_seq) are a dead heat (<1%)

head_tail (2.69 us) and kernel_seq (2.71 us) differ by 0.99%, inside the noise, even though the wider field spreads 592.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.67)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (2.69 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, kernel_seq, head_tail_dyn, kernel_nway} vs {sequential, nway, std_parallel} (290% apart)

The field splits into a fast tier {head_tail, kernel_seq, head_tail_dyn, kernel_nway} and a slow tier {sequential, nway, std_parallel} with a 290% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.9x the fastest

Fastest head_tail (2.69 us) to slowest std_parallel (18.61 us): 6.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (head_tail) is the fastest** at 2685.8 ns median
- 4 variants significantly slower than baseline
- Spread: 6.93x (fastest 2685.8 ns, slowest 18609.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 1 |
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
| head_tail | 3203ns | 3175ns | 2947ns | 3125ns | 3449ns | base |
| head_tail_dyn | 3363ns | 3366ns | 3082ns | 3301ns | 3596ns | +4.97% |
| kernel_nway | 3365ns | 3318ns | 3155ns | 3314ns | 3547ns | +5.04% |
| kernel_seq | 3258ns | 3262ns | 2792ns | 3156ns | 3644ns | +1.71% |
| nway | 13798ns | 13122ns | 12412ns | 13018ns | 15660ns | +330.71% |
| sequential | 10896ns | 10932ns | 10008ns | 10641ns | 11722ns | +240.13% |
| std_parallel | 19056ns | 18952ns | 17503ns | 18589ns | 20534ns | +494.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2710ns | 2558ns | 2882ns | base | 0.378 |
| head_tail_dyn | 2780ns | 2527ns | 3009ns | +2.60% | 0.368 |
| kernel_nway | 2832ns | 2663ns | 3036ns | +4.50% | 0.362 |
| kernel_seq | 2756ns | 2526ns | 3005ns | +1.72% | 0.372 |
| nway | 13356ns | 12143ns | 14987ns | +392.84% | 0.077 |
| sequential | 10714ns | 9872ns | 11504ns | +295.38% | 0.096 |
| std_parallel | 18706ns | 17157ns | 20163ns | +590.27% | 0.055 |

## Performance model

- Peak throughput: **0.405 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.381 | 94.0% |
| head_tail_dyn | 0.372 | 91.8% |
| kernel_nway | 0.371 | 91.4% |
| kernel_seq | 0.378 | 93.1% |
| nway | 0.080 | 19.8% |
| sequential | 0.095 | 23.4% |
| std_parallel | 0.055 | 13.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 3203ns | 3203ns | base |
| head_tail_dyn | 3363ns | 3363ns | +4.97% |
| kernel_nway | 3365ns | 3365ns | +5.04% |
| kernel_seq | 3258ns | 3258ns | +1.71% |
| nway | 13798ns | 13798ns | +330.71% |
| sequential | 10896ns | 10896ns | +240.13% |
| std_parallel | 19056ns | 19056ns | +494.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2686ns | base | --- | [2588, 2807] | --- | --- | --- | --- |
| head_tail_dyn | 2750ns | no significant difference | [-85, +209]ns | [2632, 2878] | no | 0.2855 | 0.2379 | 0 |
| kernel_nway | 2762ns | +110.8ns (+4.1%) | [+33, +155]ns | [2719, 2836] | YES | 0.0463 | 0.0309 | 0 |
| kernel_seq | 2712ns | no significant difference | [-59, +150]ns | [2588, 2827] | no | 0.8145 | 0.8145 | 0 |
| nway | 12754ns | +9997.4ns (+372.2%) | [+9692, +11263]ns | [12388, 13928] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10774ns | +7995.0ns (+297.7%) | [+7342, +8256]ns | [10033, 11030] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 18609ns | +15954.1ns (+594.0%) | [+15193, +16783]ns | [17756, 19473] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2572ns | +5.0% | +5.8% | -0.3% | +466.0% | +329.1% | +603.4% |
| 1 | 2 | 2807ns | +2.4% | +0.9% | +0.7% | +359.9% | +289.4% | +567.2% |
| 1 | 3 | 2690ns | +0.3% | +5.1% | +2.5% | +357.5% | +269.5% | +609.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.499 | moderate- |
| head_tail_dyn | -0.666 | HIGH- (thermal bounce) |
| kernel_nway | -0.190 | ok |
| kernel_seq | -0.328 | moderate- |
| nway | -0.045 | ok |
| sequential | -0.117 | ok |
| std_parallel | -0.016 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 93319.2ns | 2709.9ns | 3443.6% | HIGH |
| head_tail_dyn | 94225.4ns | 2780.3ns | 3389.1% | HIGH |
| kernel_nway | 93540.0ns | 2832.0ns | 3303.0% | HIGH |
| kernel_seq | 92629.3ns | 2756.4ns | 3360.5% | HIGH |
| nway | 95723.8ns | 13355.6ns | 716.7% | HIGH |
| sequential | 91522.7ns | 10714.4ns | 854.2% | HIGH |
| std_parallel | 95491.2ns | 18705.6ns | 510.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 2557.8-2881.9 ns)
   2557.8 |########################################
   2574.0 |
   2590.2 |#############
   2606.4 |#############
   2622.6 |
   2638.8 |
   2655.0 |#############
   2671.2 |#############
   2687.4 |##########################
   2703.6 |#############
   2719.8 |
   2736.1 |
   2752.3 |
   2768.5 |#############
   2784.7 |
   2800.9 |
   2817.1 |#############
   2833.3 |#############
   2849.5 |
   2865.7 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 2527.2-3008.9 ns)
   2527.2 |########################################
   2551.3 |
   2575.4 |
   2599.5 |####################
   2623.5 |########################################
   2647.6 |####################
   2671.7 |
   2695.8 |########################################
   2719.9 |
   2744.0 |
   2768.0 |
   2792.1 |####################
   2816.2 |####################
   2840.3 |########################################
   2864.4 |
   2888.5 |####################
   2912.5 |
   2936.6 |########################################
   2960.7 |
   2984.8 |
  (1 below, 2 above range)

kernel_nway (n=18, range 2663.3-3035.8 ns)
   2663.3 |
   2681.9 |##########################
   2700.6 |##########################
   2719.2 |########################################
   2737.8 |#############
   2756.4 |
   2775.1 |#############
   2793.7 |
   2812.3 |##########################
   2830.9 |##########################
   2849.6 |
   2868.2 |
   2886.8 |
   2905.4 |
   2924.1 |
   2942.7 |
   2961.3 |
   2980.0 |
   2998.6 |
   3017.2 |
  (1 below, 4 above range)

kernel_seq (n=18, range 2525.5-3005.3 ns)
   2525.5 |########################################
   2549.5 |####################
   2573.5 |####################
   2597.5 |####################
   2621.5 |
   2645.5 |########################################
   2669.5 |
   2693.4 |####################
   2717.4 |####################
   2741.4 |
   2765.4 |########################################
   2789.4 |
   2813.4 |########################################
   2837.4 |
   2861.3 |####################
   2885.3 |
   2909.3 |
   2933.3 |
   2957.3 |
   2981.3 |
  (1 below, 3 above range)

nway (n=18, range 12143.3-14987.2 ns)
  12143.3 |########################################
  12285.5 |##########################
  12427.7 |#############
  12569.9 |##########################
  12712.1 |#############
  12854.3 |#############
  12996.5 |
  13138.7 |
  13280.9 |
  13423.1 |#############
  13565.3 |
  13707.5 |
  13849.7 |##########################
  13991.9 |
  14134.1 |
  14276.3 |
  14418.5 |#############
  14560.6 |#############
  14702.8 |
  14845.0 |#############
  (1 below, 1 above range)

sequential (n=18, range 9872.2-11503.9 ns)
   9872.2 |##########################
   9953.8 |##########################
  10035.4 |#############
  10117.0 |
  10198.5 |
  10280.1 |#############
  10361.7 |#############
  10443.3 |
  10524.9 |
  10606.5 |
  10688.0 |#############
  10769.6 |
  10851.2 |##########################
  10932.8 |#############
  11014.4 |########################################
  11096.0 |#############
  11177.5 |
  11259.1 |
  11340.7 |
  11422.3 |
  (1 below, 2 above range)

std_parallel (n=18, range 17156.7-20163.1 ns)
  17156.7 |########################################
  17307.0 |####################
  17457.3 |####################
  17607.6 |
  17757.9 |
  17908.3 |####################
  18058.6 |####################
  18208.9 |####################
  18359.2 |####################
  18509.5 |
  18659.9 |########################################
  18810.2 |####################
  18960.5 |
  19110.8 |
  19261.1 |####################
  19411.5 |
  19561.8 |####################
  19712.1 |
  19862.4 |
  20012.7 |
  (1 below, 4 above range)

```

## Diagnostics

- **head_tail**: bridge=3474.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3413.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=3389.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3386.9% of algo (FFI overhead may distort results)
- **nway**: bridge=744.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=839.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=515.4% of algo (FFI overhead may distort results)

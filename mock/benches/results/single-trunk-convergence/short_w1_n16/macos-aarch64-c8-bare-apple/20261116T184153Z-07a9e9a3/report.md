# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 7.1x slower than the field

std_parallel (18.41 us) is 7.1x the fastest (2.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.65)

head_tail's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn, head_tail, kernel_nway, kernel_seq} vs {sequential, nway, std_parallel} (284% apart)

The field splits into a fast tier {head_tail_dyn, head_tail, kernel_nway, kernel_seq} and a slow tier {sequential, nway, std_parallel} with a 284% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.1x the fastest

Fastest head_tail_dyn (2.58 us) to slowest std_parallel (18.41 us): 7.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: head_tail_dyn** at 2583.3 ns median (-2.0% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 7.13x (fastest 2583.3 ns, slowest 18406.7 ns)

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
| head_tail | 3173ns | 3132ns | 2840ns | 3092ns | 3461ns | base |
| head_tail_dyn | 3161ns | 3120ns | 2928ns | 3091ns | 3383ns | -0.36% |
| kernel_nway | 3189ns | 3199ns | 2954ns | 3147ns | 3370ns | +0.52% |
| kernel_seq | 3297ns | 3287ns | 2946ns | 3201ns | 3617ns | +3.92% |
| nway | 12789ns | 12798ns | 12493ns | 12740ns | 13012ns | +303.09% |
| sequential | 10625ns | 10408ns | 9908ns | 10316ns | 11447ns | +234.88% |
| std_parallel | 18825ns | 18755ns | 17562ns | 18580ns | 19822ns | +493.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2668ns | 2463ns | 2842ns | base | 0.006 |
| head_tail_dyn | 2660ns | 2502ns | 2851ns | -0.31% | 0.006 |
| kernel_nway | 2671ns | 2516ns | 2816ns | +0.09% | 0.006 |
| kernel_seq | 2768ns | 2541ns | 3061ns | +3.75% | 0.006 |
| nway | 12451ns | 12209ns | 12671ns | +366.61% | 0.001 |
| sequential | 10457ns | 9773ns | 11225ns | +291.90% | 0.002 |
| std_parallel | 18472ns | 17205ns | 19473ns | +592.28% | 0.001 |

## Performance model

- Peak throughput: **0.006 Gops/s** (head_tail; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.006 | 93.5% |
| head_tail_dyn | 0.006 | 95.4% |
| kernel_nway | 0.006 | 92.5% |
| kernel_seq | 0.006 | 92.1% |
| nway | 0.001 | 19.8% |
| sequential | 0.002 | 24.0% |
| std_parallel | 0.001 | 13.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 3173ns | 3173ns | base |
| head_tail_dyn | 3161ns | 3161ns | -0.36% |
| kernel_nway | 3189ns | 3189ns | +0.52% |
| kernel_seq | 3297ns | 3297ns | +3.92% |
| nway | 12789ns | 12789ns | +303.09% |
| sequential | 10625ns | 10625ns | +234.88% |
| std_parallel | 18825ns | 18825ns | +493.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2636ns | base | --- | [2593, 2733] | --- | --- | --- | --- |
| head_tail_dyn | 2583ns | no significant difference | [-154, +51]ns | [2567, 2688] | no | 0.5768 | 0.4807 | 0 |
| kernel_nway | 2663ns | no significant difference | [-38, +82]ns | [2587, 2710] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 2675ns | no significant difference | [-60, +209]ns | [2581, 2907] | no | 0.3568 | 0.2379 | 0 |
| nway | 12418ns | +9788.4ns (+371.4%) | [+9637, +9872]ns | [12322, 12548] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10268ns | +7707.5ns (+292.4%) | [+7318, +7941]ns | [10030, 10525] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 18407ns | +15790.0ns (+599.0%) | [+15427, +16321]ns | [17958, 18985] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2613ns | +0.4% | +2.5% | -1.0% | +378.2% | +285.6% | +599.6% |
| 1 | 2 | 2733ns | -5.2% | -2.6% | +4.8% | +357.1% | +272.3% | +543.8% |
| 1 | 3 | 2632ns | -3.2% | +0.0% | +8.6% | +369.0% | +309.9% | +622.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.653 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.004 | ok |
| kernel_nway | -0.029 | ok |
| kernel_seq | -0.182 | ok |
| nway | -0.153 | ok |
| sequential | -0.097 | ok |
| std_parallel | -0.490 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 1/3, lost 1/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 92937.7ns | 2668.3ns | 3483.0% | HIGH |
| head_tail_dyn | 94098.4ns | 2660.0ns | 3537.5% | HIGH |
| kernel_nway | 94019.6ns | 2670.7ns | 3520.4% | HIGH |
| kernel_seq | 96180.8ns | 2768.4ns | 3474.2% | HIGH |
| nway | 93980.2ns | 12450.8ns | 754.8% | HIGH |
| sequential | 91381.3ns | 10457.1ns | 873.9% | HIGH |
| std_parallel | 95584.0ns | 18472.3ns | 517.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 2463.3-2842.5 ns)
   2463.3 |########################################
   2482.3 |
   2501.2 |
   2520.2 |
   2539.2 |
   2558.1 |####################
   2577.1 |####################
   2596.0 |########################################
   2615.0 |########################################
   2634.0 |########################################
   2652.9 |
   2671.9 |####################
   2690.8 |
   2709.8 |
   2728.8 |########################################
   2747.7 |
   2766.7 |
   2785.6 |
   2804.6 |
   2823.5 |####################
  (1 below, 3 above range)

head_tail_dyn (n=18, range 2501.7-2851.4 ns)
   2501.7 |##########
   2519.2 |
   2536.6 |
   2554.1 |##############################
   2571.6 |########################################
   2589.1 |##########
   2606.6 |
   2624.1 |
   2641.6 |
   2659.0 |##########
   2676.5 |##########
   2694.0 |##########
   2711.5 |
   2729.0 |
   2746.5 |
   2764.0 |
   2781.5 |
   2798.9 |
   2816.4 |
   2833.9 |##########
  (2 below, 3 above range)

kernel_nway (n=18, range 2515.5-2816.1 ns)
   2515.5 |########################################
   2530.6 |####################
   2545.6 |
   2560.6 |
   2575.7 |####################
   2590.7 |########################################
   2605.7 |
   2620.7 |
   2635.8 |####################
   2650.8 |####################
   2665.8 |########################################
   2680.9 |####################
   2695.9 |####################
   2710.9 |####################
   2725.9 |####################
   2741.0 |
   2756.0 |
   2771.0 |
   2786.1 |
   2801.1 |####################
  (1 below, 2 above range)

kernel_seq (n=18, range 2541.1-3061.1 ns)
   2541.1 |########################################
   2567.1 |
   2593.1 |##########################
   2619.1 |#############
   2645.1 |#############
   2671.1 |#############
   2697.1 |#############
   2723.1 |
   2749.1 |
   2775.1 |
   2801.1 |#############
   2827.1 |
   2853.1 |
   2879.1 |#############
   2905.1 |#############
   2931.1 |
   2957.1 |
   2983.1 |
   3009.1 |#############
   3035.1 |#############
  (2 below, 2 above range)

nway (n=18, range 12209.4-12671.1 ns)
  12209.4 |
  12232.5 |
  12255.6 |
  12278.7 |####################
  12301.8 |########################################
  12324.9 |####################
  12347.9 |####################
  12371.0 |####################
  12394.1 |####################
  12417.2 |####################
  12440.3 |####################
  12463.4 |####################
  12486.4 |
  12509.5 |
  12532.6 |####################
  12555.7 |####################
  12578.8 |
  12601.9 |####################
  12624.9 |
  12648.0 |
  (2 below, 3 above range)

sequential (n=18, range 9772.8-11225.3 ns)
   9772.8 |########################################
   9845.4 |
   9918.0 |#############
   9990.6 |
  10063.3 |#############
  10135.9 |
  10208.5 |########################################
  10281.1 |##########################
  10353.8 |##########################
  10426.4 |
  10499.0 |
  10571.7 |
  10644.3 |#############
  10716.9 |
  10789.5 |
  10862.2 |#############
  10934.8 |
  11007.4 |
  11080.0 |#############
  11152.7 |
  (1 below, 2 above range)

std_parallel (n=18, range 17205.0-19472.8 ns)
  17205.0 |
  17318.4 |########################################
  17431.7 |
  17545.1 |
  17658.5 |####################
  17771.9 |
  17885.3 |####################
  17998.7 |####################
  18112.1 |########################################
  18225.5 |
  18338.9 |########################################
  18452.3 |####################
  18565.6 |####################
  18679.0 |
  18792.4 |
  18905.8 |####################
  19019.2 |########################################
  19132.6 |
  19246.0 |
  19359.4 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=3536.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3637.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=3514.6% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3574.1% of algo (FFI overhead may distort results)
- **nway**: bridge=756.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=888.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=520.8% of algo (FFI overhead may distort results)

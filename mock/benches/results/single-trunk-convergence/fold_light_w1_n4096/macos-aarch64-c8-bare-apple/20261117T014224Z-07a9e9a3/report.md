# fold_light, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 7.4x slower than the field

std_parallel (20.13 us) is 7.4x the fastest (2.72 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.67)

head_tail's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_nway, head_tail, kernel_seq, head_tail_dyn} vs {sequential, nway, std_parallel} (262% apart)

The field splits into a fast tier {kernel_nway, head_tail, kernel_seq, head_tail_dyn} and a slow tier {sequential, nway, std_parallel} with a 262% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.4x the fastest

Fastest kernel_nway (2.72 us) to slowest std_parallel (20.13 us): 7.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway's edge over baseline is significant but tiny (-35 ns, 1.27%)

kernel_nway differs from baseline head_tail by -35 ns (1.27%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: kernel_nway** at 2716.7 ns median (-2.5% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 7.41x (fastest 2716.7 ns, slowest 20129.2 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 3466ns | 3369ns | 3065ns | 3303ns | 3909ns | base |
| head_tail_dyn | 3476ns | 3290ns | 3058ns | 3275ns | 3986ns | +0.31% |
| kernel_nway | 3288ns | 3292ns | 3104ns | 3268ns | 3412ns | -5.11% |
| kernel_seq | 3405ns | 3338ns | 3036ns | 3288ns | 3765ns | -1.76% |
| nway | 13074ns | 12769ns | 12597ns | 12757ns | 13788ns | +277.26% |
| sequential | 10805ns | 10435ns | 10307ns | 10425ns | 11625ns | +211.79% |
| std_parallel | 20291ns | 20502ns | 18886ns | 20211ns | 21112ns | +485.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2921ns | 2699ns | 3226ns | base | 1.402 |
| head_tail_dyn | 3002ns | 2718ns | 3409ns | +2.78% | 1.364 |
| kernel_nway | 2739ns | 2692ns | 2797ns | -6.22% | 1.495 |
| kernel_seq | 2893ns | 2710ns | 3145ns | -0.95% | 1.416 |
| nway | 12736ns | 12326ns | 13418ns | +336.06% | 0.322 |
| sequential | 10656ns | 10164ns | 11470ns | +264.84% | 0.384 |
| std_parallel | 19934ns | 18524ns | 20794ns | +582.51% | 0.205 |

## Performance model

- Peak throughput: **1.522 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.469 | 96.6% |
| head_tail_dyn | 1.441 | 94.7% |
| kernel_nway | 1.508 | 99.1% |
| kernel_seq | 1.466 | 96.3% |
| nway | 0.330 | 21.7% |
| sequential | 0.399 | 26.2% |
| std_parallel | 0.203 | 13.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 3466ns | 3466ns | base |
| head_tail_dyn | 3476ns | 3476ns | +0.31% |
| kernel_nway | 3288ns | 3288ns | -5.11% |
| kernel_seq | 3405ns | 3405ns | -1.76% |
| nway | 13074ns | 13074ns | +277.26% |
| sequential | 10805ns | 10805ns | +211.79% |
| std_parallel | 20291ns | 20291ns | +485.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2788ns | base | --- | [2729, 3088] | --- | --- | --- | --- |
| head_tail_dyn | 2842ns | no significant difference | [-340, +271]ns | [2783, 3056] | no | 0.5768 | 0.4807 | 0 |
| kernel_nway | 2717ns | -35.4ns (-1.3%) | [-350, -17]ns | [2706, 2748] | YES | 0.0113 | 0.0075 | 0 |
| kernel_seq | 2794ns | no significant difference | [-96, +183]ns | [2735, 2942] | no | 1.0000 | 1.0000 | 0 |
| nway | 12419ns | +9608.4ns (+344.7%) | [+9375, +9775]ns | [12358, 12621] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10275ns | +7493.8ns (+268.8%) | [+7200, +8165]ns | [10256, 10933] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 20129ns | +17027.1ns (+610.8%) | [+16350, +17571]ns | [19481, 20406] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2752ns | +3.0% | -1.1% | +3.5% | +380.6% | +273.1% | +588.6% |
| 1 | 2 | 3165ns | -12.3% | -14.0% | -8.9% | +291.0% | +224.0% | +533.9% |
| 1 | 3 | 2738ns | +13.0% | -0.8% | +0.4% | +352.1% | +324.8% | +658.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.666 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.279 | moderate- |
| kernel_nway | -0.025 | ok |
| kernel_seq | -0.328 | moderate- |
| nway | -0.169 | ok |
| sequential | -0.172 | ok |
| std_parallel | -0.011 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 92950.2ns | 2920.6ns | 3182.6% | HIGH |
| head_tail_dyn | 92145.4ns | 3001.8ns | 3069.6% | HIGH |
| kernel_nway | 90395.1ns | 2738.9ns | 3300.4% | HIGH |
| kernel_seq | 91822.5ns | 2892.8ns | 3174.1% | HIGH |
| nway | 89051.6ns | 12735.6ns | 699.2% | HIGH |
| sequential | 92686.1ns | 10655.6ns | 869.8% | HIGH |
| std_parallel | 96445.4ns | 19933.6ns | 483.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 2698.6-3226.4 ns)
   2698.6 |########################################
   2725.0 |########################################
   2751.4 |#############
   2777.8 |########################################
   2804.2 |
   2830.6 |
   2856.9 |
   2883.3 |
   2909.7 |
   2936.1 |
   2962.5 |
   2988.9 |
   3015.3 |
   3041.7 |##########################
   3068.1 |
   3094.4 |#############
   3120.8 |
   3147.2 |#############
   3173.6 |
   3200.0 |#############
  (1 below, 2 above range)

head_tail_dyn (n=18, range 2718.1-3409.0 ns)
   2718.1 |##########
   2752.6 |##############################
   2787.2 |##########
   2821.7 |########################################
   2856.3 |
   2890.8 |##########
   2925.4 |
   2959.9 |
   2994.5 |
   3029.0 |####################
   3063.6 |##########
   3098.1 |##########
   3132.6 |
   3167.2 |
   3201.7 |
   3236.3 |
   3270.8 |
   3305.4 |
   3339.9 |
   3374.5 |
  (2 below, 2 above range)

kernel_nway (n=18, range 2691.7-2796.5 ns)
   2691.7 |##########
   2696.9 |####################
   2702.2 |##########
   2707.4 |########################################
   2712.6 |
   2717.9 |##########
   2723.1 |
   2728.4 |##########
   2733.6 |
   2738.8 |
   2744.1 |####################
   2749.3 |##########
   2754.6 |
   2759.8 |
   2765.1 |
   2770.3 |
   2775.5 |
   2780.8 |
   2786.0 |##########
   2791.3 |
  (1 below, 3 above range)

kernel_seq (n=18, range 2709.7-3145.1 ns)
   2709.7 |########################################
   2731.5 |#############
   2753.3 |##########################
   2775.0 |#############
   2796.8 |#############
   2818.6 |
   2840.4 |
   2862.1 |#############
   2883.9 |##########################
   2905.7 |
   2927.4 |
   2949.2 |
   2971.0 |#############
   2992.7 |
   3014.5 |#############
   3036.3 |
   3058.1 |
   3079.8 |
   3101.6 |
   3123.4 |
  (2 below, 3 above range)

nway (n=18, range 12326.4-13418.1 ns)
  12326.4 |########################################
  12381.0 |########################
  12435.6 |########
  12490.1 |########
  12544.7 |################
  12599.3 |
  12653.9 |########
  12708.5 |
  12763.1 |
  12817.6 |
  12872.2 |########
  12926.8 |
  12981.4 |
  13036.0 |
  13090.6 |
  13145.2 |
  13199.7 |
  13254.3 |
  13308.9 |
  13363.5 |
  (1 below, 3 above range)

sequential (n=18, range 10163.9-11470.1 ns)
  10163.9 |
  10229.2 |########################################
  10294.5 |########
  10359.8 |
  10425.1 |####
  10490.5 |
  10555.8 |
  10621.1 |
  10686.4 |
  10751.7 |
  10817.0 |
  10882.3 |
  10947.6 |
  11013.0 |
  11078.3 |
  11143.6 |
  11208.9 |
  11274.2 |
  11339.5 |
  11404.8 |
  (1 below, 5 above range)

std_parallel (n=18, range 18523.6-20794.5 ns)
  18523.6 |#############
  18637.2 |
  18750.7 |#############
  18864.3 |
  18977.8 |
  19091.3 |#############
  19204.9 |#############
  19318.4 |
  19432.0 |
  19545.5 |
  19659.0 |##########################
  19772.6 |
  19886.1 |#############
  19999.7 |
  20113.2 |########################################
  20226.7 |#############
  20340.3 |##########################
  20453.8 |
  20567.4 |
  20680.9 |##########################
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=3278.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3231.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=3302.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3291.3% of algo (FFI overhead may distort results)
- **nway**: bridge=708.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=903.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=478.8% of algo (FFI overhead may distort results)

# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail dominates: 15% faster than the next best (head_tail_dyn)

head_tail (1.85 us) leads head_tail_dyn (2.12 us) by 15%, a clear separation rather than a photo finish. CV 30.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### nway is an outlier: 9.2x slower than the field

nway (16.96 us) is 9.2x the fastest (1.85 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.67)

head_tail's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (1.85 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 9.2x the fastest

Fastest head_tail (1.85 us) to slowest nway (16.96 us): 9.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.3x its best-20%

sequential's best 20% of batches run at 7.10 us but its worst 20% at 23.47 us (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 1850.8 ns median
- 3 variants significantly slower than baseline
- Spread: 9.16x (fastest 1850.8 ns, slowest 16955.2 ns)

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
| Counter frequency | 2100306664 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 2574ns | 2179ns | 2022ns | 2210ns | 3397ns | base |
| head_tail_dyn | 2739ns | 2375ns | 2081ns | 2384ns | 3602ns | +6.42% |
| kernel_nway | 3278ns | 3403ns | 2102ns | 3126ns | 4095ns | +27.34% |
| kernel_seq | 3053ns | 3146ns | 2339ns | 2965ns | 3541ns | +18.59% |
| nway | 16069ns | 17409ns | 11527ns | 15925ns | 18556ns | +524.22% |
| sequential | 13051ns | 7954ns | 7320ns | 7807ns | 23783ns | +406.99% |
| std_parallel | 13871ns | 14649ns | 10138ns | 13401ns | 16445ns | +438.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2276ns | 1810ns | 3017ns | base | 0.450 |
| head_tail_dyn | 2417ns | 1861ns | 3223ns | +6.21% | 0.424 |
| kernel_nway | 2798ns | 1856ns | 3493ns | +22.95% | 0.366 |
| kernel_seq | 2722ns | 2081ns | 3143ns | +19.62% | 0.376 |
| nway | 15720ns | 11249ns | 18185ns | +590.81% | 0.065 |
| sequential | 12782ns | 7100ns | 23467ns | +461.70% | 0.080 |
| std_parallel | 13427ns | 9867ns | 16046ns | +490.07% | 0.076 |

## Performance model

- Peak throughput: **0.566 Gops/s** (head_tail; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.553 | 97.8% |
| head_tail_dyn | 0.483 | 85.4% |
| kernel_nway | 0.335 | 59.3% |
| kernel_seq | 0.360 | 63.7% |
| nway | 0.060 | 10.7% |
| sequential | 0.133 | 23.5% |
| std_parallel | 0.073 | 12.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 2574ns | 2574ns | base |
| head_tail_dyn | 2739ns | 2739ns | +6.42% |
| kernel_nway | 3278ns | 3278ns | +27.34% |
| kernel_seq | 3053ns | 3053ns | +18.59% |
| nway | 16069ns | 16069ns | +524.22% |
| sequential | 13051ns | 13051ns | +406.99% |
| std_parallel | 13871ns | 13871ns | +438.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 1851ns | base | --- | [1827, 2887] | --- | --- | --- | --- |
| head_tail_dyn | 2120ns | no significant difference | [-1006, +1352]ns | [1918, 3203] | no | 0.4807 | 0.4807 | 0 |
| kernel_nway | 3053ns | no significant difference | [-945, +1412]ns | [1882, 3248] | no | 0.2855 | 0.2379 | 0 |
| kernel_seq | 2844ns | no significant difference | [-594, +1054]ns | [2357, 2892] | no | 0.2855 | 0.2379 | 0 |
| nway | 16955ns | +14575.8ns (+787.6%) | [+11792, +15039]ns | [13201, 17856] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 7711ns | +5627.0ns (+304.0%) | [+5248, +6299]ns | [7157, 8407] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 14111ns | +10977.4ns (+593.1%) | [+9124, +13183]ns | [10617, 15057] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 1841ns | +74.7% | +76.9% | +56.9% | +853.6% | +286.0% | +731.4% |
| 1 | 2 | 3020ns | -37.5% | -38.2% | -30.1% | +488.9% | +157.8% | +367.3% |
| 1 | 3 | 1827ns | +16.0% | +71.7% | +57.5% | +527.0% | +360.9% | +460.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.667 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.268 | moderate- |
| kernel_nway | -0.663 | HIGH- (thermal bounce) |
| kernel_seq | -0.667 | HIGH- (thermal bounce) |
| nway | -0.185 | ok |
| sequential | -0.000 | ok |
| std_parallel | -0.057 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 2477.5ns | 2275.5ns | 108.9% | HIGH |
| head_tail_dyn | 2956.0ns | 2416.8ns | 122.3% | HIGH |
| kernel_nway | 3011.7ns | 2797.7ns | 107.6% | HIGH |
| kernel_seq | 2961.6ns | 2722.1ns | 108.8% | HIGH |
| nway | 16229.4ns | 15719.8ns | 103.2% | HIGH |
| sequential | 10613.2ns | 12781.7ns | 83.0% | HIGH |
| std_parallel | 13938.1ns | 13427.3ns | 103.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 1810.5-3017.1 ns)
   1810.5 |########################################
   1870.8 |
   1931.1 |
   1991.5 |
   2051.8 |
   2112.1 |
   2172.5 |
   2232.8 |
   2293.1 |
   2353.5 |
   2413.8 |
   2474.1 |
   2534.5 |
   2594.8 |
   2655.1 |####
   2715.5 |
   2775.8 |####
   2836.1 |
   2896.5 |####
   2956.8 |####
  (1 below, 3 above range)

head_tail_dyn (n=18, range 1860.9-3223.2 ns)
   1860.9 |########################################
   1929.0 |
   1997.2 |
   2065.3 |########################################
   2133.4 |##########
   2201.5 |
   2269.6 |##########
   2337.7 |
   2405.9 |
   2474.0 |
   2542.1 |
   2610.2 |
   2678.3 |
   2746.4 |
   2814.5 |
   2882.7 |
   2950.8 |
   3018.9 |
   3087.0 |
   3155.1 |########################################
  (2 below, 2 above range)

kernel_nway (n=18, range 1856.1-3492.7 ns)
   1856.1 |########################################
   1938.0 |
   2019.8 |
   2101.6 |
   2183.5 |
   2265.3 |
   2347.1 |
   2428.9 |
   2510.8 |
   2592.6 |
   2674.4 |
   2756.3 |
   2838.1 |##########
   2919.9 |##########
   3001.8 |##############################
   3083.6 |
   3165.4 |##########
   3247.2 |##############################
   3329.1 |##########
   3410.9 |##########
  (2 below, 1 above range)

kernel_seq (n=18, range 2081.4-3142.9 ns)
   2081.4 |################
   2134.4 |
   2187.5 |
   2240.6 |
   2293.7 |########
   2346.8 |########
   2399.8 |
   2452.9 |
   2506.0 |
   2559.1 |
   2612.1 |
   2665.2 |
   2718.3 |
   2771.4 |########
   2824.4 |################################
   2877.5 |########################################
   2930.6 |
   2983.7 |
   3036.7 |
   3089.8 |########
  (2 below, 1 above range)

nway (n=18, range 11249.0-18184.9 ns)
  11249.0 |##########
  11595.8 |####################
  11942.6 |
  12289.4 |
  12636.2 |
  12983.0 |
  13329.8 |
  13676.6 |
  14023.4 |
  14370.2 |##########
  14716.9 |
  15063.7 |
  15410.5 |
  15757.3 |
  16104.1 |##########
  16450.9 |
  16797.7 |########################################
  17144.5 |
  17491.3 |##########
  17838.1 |##############################
  (2 below, 3 above range)

sequential (n=18, range 7099.6-23466.8 ns)
   7099.6 |########################################
   7918.0 |#########################
   8736.3 |
   9554.7 |#####
  10373.0 |
  11191.4 |
  12009.8 |
  12828.1 |
  13646.5 |
  14464.8 |#####
  15283.2 |
  16101.6 |
  16919.9 |
  17738.3 |
  18556.6 |
  19375.0 |
  20193.4 |
  21011.7 |
  21830.1 |
  22648.4 |
  (2 below, 1 above range)

std_parallel (n=18, range 9867.3-16046.1 ns)
   9867.3 |
  10176.3 |########################################
  10485.2 |
  10794.2 |#############
  11103.1 |
  11412.0 |
  11721.0 |
  12029.9 |
  12338.8 |
  12647.8 |
  12956.7 |#############
  13265.7 |#############
  13574.6 |
  13883.5 |##########################
  14192.5 |
  14501.4 |#############
  14810.4 |########################################
  15119.3 |
  15428.2 |#############
  15737.2 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=24.8% (high variance, measurements may be unstable)
- **head_tail**: bridge=106.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=24.1% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=123.9% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=25.8% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=101.6% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=106.6% of algo (FFI overhead may distort results)
- **nway**: bridge=104.1% of algo (FFI overhead may distort results)
- **sequential**: CV=147.2% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **sequential**: bridge=100.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=99.0% of algo (FFI overhead may distort results)

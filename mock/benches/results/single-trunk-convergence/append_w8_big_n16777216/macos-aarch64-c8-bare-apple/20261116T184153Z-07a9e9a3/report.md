# append, far above cache, W = 8

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential is an outlier: 4.5x slower than the field

sequential (12.16 ms) is 4.5x the fastest (2.72 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.65)

kernel_nway's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn, head_tail, kernel_seq, kernel_nway, nway, std_parallel} vs {sequential} (212% apart)

The field splits into a fast tier {head_tail_dyn, head_tail, kernel_seq, kernel_nway, nway, std_parallel} and a slow tier {sequential} with a 212% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.5x the fastest

Fastest head_tail_dyn (2.72 ms) to slowest sequential (12.16 ms): 4.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: head_tail_dyn** at 2723385.4 ns median (-1.7% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 4.47x (fastest 2723385.4 ns, slowest 12163963.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, far above cache, W = 8 |
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
| head_tail | 2909611ns | 2774714ns | 2628240ns | 2729837ns | 3319958ns | base |
| head_tail_dyn | 2775370ns | 2726766ns | 2627552ns | 2707274ns | 2951422ns | -4.61% |
| kernel_nway | 3409858ns | 3394620ns | 3128938ns | 3325406ns | 3676995ns | +17.19% |
| kernel_seq | 3184767ns | 3116016ns | 2569427ns | 3009913ns | 3754719ns | +9.46% |
| nway | 3625872ns | 3680281ns | 3219490ns | 3553806ns | 3937161ns | +24.62% |
| sequential | 12156677ns | 12168469ns | 12060135ns | 12158115ns | 12202792ns | +317.81% |
| std_parallel | 3857035ns | 3898198ns | 3503219ns | 3847024ns | 4048958ns | +32.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2905109ns | 2623771ns | 3314318ns | base | 5.775 |
| head_tail_dyn | 2771830ns | 2624885ns | 2948089ns | -4.59% | 6.053 |
| kernel_nway | 3405727ns | 3125510ns | 3672875ns | +17.23% | 4.926 |
| kernel_seq | 3181366ns | 2566031ns | 3751453ns | +9.51% | 5.274 |
| nway | 3621573ns | 3215281ns | 3933214ns | +24.66% | 4.633 |
| sequential | 12152639ns | 12055948ns | 12199260ns | +318.32% | 1.381 |
| std_parallel | 3852986ns | 3498635ns | 4044521ns | +32.63% | 4.354 |

## Performance model

- Peak throughput: **6.538 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 6.055 | 92.6% |
| head_tail_dyn | 6.160 | 94.2% |
| kernel_nway | 4.950 | 75.7% |
| kernel_seq | 5.390 | 82.4% |
| nway | 4.564 | 69.8% |
| sequential | 1.379 | 21.1% |
| std_parallel | 4.308 | 65.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 2909611ns | 2909611ns | base |
| head_tail_dyn | 2775370ns | 2775370ns | -4.61% |
| kernel_nway | 3409858ns | 3409858ns | +17.19% |
| kernel_seq | 3184767ns | 3184767ns | +9.46% |
| nway | 3625872ns | 3625872ns | +24.62% |
| sequential | 12156677ns | 12156677ns | +317.81% |
| std_parallel | 3857035ns | 3857035ns | +32.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2770906ns | base | --- | [2630104, 3314318] | --- | --- | --- | --- |
| head_tail_dyn | 2723385ns | no significant difference | [-545125, +230740]ns | [2644016, 2948089] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 3389568ns | +504171.9ns (+18.2%) | [+66906, +930776]ns | [3154740, 3672875] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| kernel_seq | 3112521ns | no significant difference | [-160349, +855901]ns | [2680125, 3751453] | no | 0.8250 | 0.6875 | 0 |
| nway | 3676219ns | +644192.7ns (+23.2%) | [+241479, +1263719]ns | [3255286, 3933214] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| sequential | 12163964ns | +9352593.8ns (+337.5%) | [+8865302, +9524693]ns | [12094693, 12199260] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| std_parallel | 3894297ns | +1103921.8ns (+39.8%) | [+475516, +1264193]ns | [3620141, 4044521] | YES (adj: no) | 0.3281 | 0.2188 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2770906ns | +3.0% | +21.5% | +7.2% | +26.8% | +337.6% | +39.8% |
| 1 | 2 | 2630104ns | +2.3% | +32.0% | +26.1% | +39.3% | +362.0% | +48.1% |
| 1 | 3 | 3314318ns | -16.4% | +2.0% | -1.6% | +11.2% | +267.5% | +14.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.290 | moderate- |
| head_tail_dyn | -0.484 | moderate- |
| kernel_nway | -0.654 | HIGH- (thermal bounce) |
| kernel_seq | -0.260 | moderate- |
| nway | -0.100 | ok |
| sequential | -0.001 | ok |
| std_parallel | -0.277 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 94962043.4ns | 2905109.4ns | 3268.8% | HIGH |
| head_tail_dyn | 95137885.4ns | 2771829.9ns | 3432.3% | HIGH |
| kernel_nway | 95093066.0ns | 3405727.4ns | 2792.2% | HIGH |
| kernel_seq | 94905805.6ns | 3181366.3ns | 2983.2% | HIGH |
| nway | 95578923.6ns | 3621572.9ns | 2639.2% | HIGH |
| sequential | 103832545.2ns | 12152638.9ns | 854.4% | HIGH |
| std_parallel | 95250324.6ns | 3852986.1ns | 2472.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 2623770.8-3314317.7 ns)
  2623770.8 |########################################
  2658298.1 |####################
  2692825.5 |
  2727352.8 |
  2761880.2 |
  2796407.5 |
  2830934.9 |####################
  2865462.2 |
  2899989.6 |
  2934516.9 |
  2969044.2 |
  3003571.6 |
  3038098.9 |####################
  3072626.3 |
  3107153.6 |
  3141681.0 |
  3176208.3 |
  3210735.7 |
  3245263.0 |
  3279790.4 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 2624885.4-2948088.5 ns)
  2624885.4 |########################################
  2641045.6 |
  2657205.7 |########################################
  2673365.9 |########################################
  2689526.0 |
  2705686.2 |
  2721846.3 |
  2738006.5 |
  2754166.7 |########################################
  2770326.8 |
  2786487.0 |
  2802647.1 |
  2818807.3 |
  2834967.4 |
  2851127.6 |
  2867287.8 |########################################
  2883447.9 |
  2899608.1 |
  2915768.2 |
  2931928.4 |
  (0 below, 1 above range)

kernel_nway (n=6, range 3125510.4-3672875.0 ns)
  3125510.4 |####################
  3152878.6 |
  3180246.9 |########################################
  3207615.1 |
  3234983.3 |
  3262351.5 |
  3289719.8 |
  3317088.0 |
  3344456.2 |
  3371824.5 |
  3399192.7 |
  3426560.9 |
  3453929.2 |
  3481297.4 |
  3508665.6 |
  3536033.9 |
  3563402.1 |####################
  3590770.3 |####################
  3618138.5 |
  3645506.8 |
  (0 below, 1 above range)

kernel_seq (n=6, range 2566031.2-3751453.1 ns)
  2566031.2 |########################################
  2625302.3 |
  2684573.4 |
  2743844.5 |########################################
  2803115.6 |########################################
  2862386.7 |
  2921657.8 |
  2980928.9 |
  3040200.0 |
  3099471.1 |
  3158742.2 |
  3218013.3 |
  3277284.4 |
  3336555.5 |########################################
  3395826.6 |
  3455097.7 |
  3514368.8 |
  3573639.9 |
  3632911.0 |########################################
  3692182.1 |
  (0 below, 1 above range)

nway (n=6, range 3215281.2-3933213.5 ns)
  3215281.2 |########################################
  3251177.8 |
  3287074.4 |########################################
  3322971.1 |
  3358867.7 |
  3394764.3 |
  3430660.9 |
  3466557.5 |
  3502454.1 |
  3538350.8 |
  3574247.4 |
  3610144.0 |########################################
  3646040.6 |
  3681937.2 |
  3717833.8 |########################################
  3753730.5 |########################################
  3789627.1 |
  3825523.7 |
  3861420.3 |
  3897316.9 |
  (0 below, 1 above range)

sequential (n=6, range 12055947.9-12199260.4 ns)
  12055947.9 |########################################
  12063113.5 |
  12070279.2 |
  12077444.8 |
  12084610.4 |
  12091776.0 |
  12098941.7 |
  12106107.3 |
  12113272.9 |
  12120438.5 |
  12127604.1 |########################################
  12134769.8 |
  12141935.4 |
  12149101.0 |
  12156266.6 |########################################
  12163432.3 |########################################
  12170597.9 |
  12177763.5 |
  12184929.1 |
  12192094.8 |########################################
  (0 below, 1 above range)

std_parallel (n=6, range 3498635.4-4044520.8 ns)
  3498635.4 |########################################
  3525929.7 |
  3553223.9 |
  3580518.2 |
  3607812.5 |
  3635106.8 |
  3662401.0 |
  3689695.3 |
  3716989.6 |########################################
  3744283.8 |
  3771578.1 |
  3798872.4 |
  3826166.6 |
  3853460.9 |########################################
  3880755.2 |
  3908049.4 |########################################
  3935343.7 |
  3962638.0 |
  3989932.3 |########################################
  4017226.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=3430.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3488.1% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=2804.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3052.5% of algo (FFI overhead may distort results)
- **nway**: bridge=2599.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=853.7% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=2446.6% of algo (FFI overhead may distort results)

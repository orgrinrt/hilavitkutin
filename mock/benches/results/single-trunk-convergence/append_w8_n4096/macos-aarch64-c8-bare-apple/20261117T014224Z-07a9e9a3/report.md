# append, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 28.1x slower than the field

std_parallel (78.34 us) is 28.1x the fastest (2.78 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (kernel_seq, head_tail_dyn) are a dead heat (<1%)

kernel_seq (2.78 us) and head_tail_dyn (2.81 us) differ by 0.90%, inside the noise, even though the wider field spreads 2714.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail shows alternating (throttle bounce) (autocorr -0.67)

head_tail's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_seq, head_tail_dyn, head_tail} vs {sequential, kernel_nway, nway, std_parallel} (369% apart)

The field splits into a fast tier {kernel_seq, head_tail_dyn, head_tail} and a slow tier {sequential, kernel_nway, nway, std_parallel} with a 369% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 28.1x the fastest

Fastest kernel_seq (2.78 us) to slowest std_parallel (78.34 us): 28.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2783.3 ns median (-1.5% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 28.15x (fastest 2783.3 ns, slowest 78339.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 8 |
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
| head_tail | 3685ns | 3475ns | 3374ns | 3450ns | 4193ns | base |
| head_tail_dyn | 3457ns | 3427ns | 3335ns | 3407ns | 3592ns | -6.18% |
| kernel_nway | 24207ns | 23317ns | 19490ns | 22753ns | 28747ns | +556.97% |
| kernel_seq | 3419ns | 3431ns | 3307ns | 3410ns | 3488ns | -7.22% |
| nway | 54775ns | 53867ns | 52036ns | 53911ns | 57440ns | +1386.54% |
| sequential | 13450ns | 13435ns | 12607ns | 13181ns | 14274ns | +265.02% |
| std_parallel | 80021ns | 78908ns | 76704ns | 78690ns | 83675ns | +2071.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 3053ns | 2742ns | 3561ns | base | 1.342 |
| head_tail_dyn | 2847ns | 2754ns | 2972ns | -6.76% | 1.439 |
| kernel_nway | 23475ns | 18882ns | 28067ns | +668.91% | 0.174 |
| kernel_seq | 2799ns | 2733ns | 2860ns | -8.33% | 1.463 |
| nway | 54206ns | 51474ns | 56842ns | +1675.48% | 0.076 |
| sequential | 13249ns | 12418ns | 14095ns | +333.95% | 0.309 |
| std_parallel | 79575ns | 76297ns | 83287ns | +2506.43% | 0.051 |

## Performance model

- Peak throughput: **1.499 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.450 | 96.8% |
| head_tail_dyn | 1.459 | 97.3% |
| kernel_nway | 0.184 | 12.2% |
| kernel_seq | 1.472 | 98.2% |
| nway | 0.077 | 5.1% |
| sequential | 0.309 | 20.6% |
| std_parallel | 0.052 | 3.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 3685ns | 3685ns | base |
| head_tail_dyn | 3457ns | 3457ns | -6.18% |
| kernel_nway | 24207ns | 24207ns | +556.97% |
| kernel_seq | 3419ns | 3419ns | -7.22% |
| nway | 54775ns | 54775ns | +1386.54% |
| sequential | 13450ns | 13450ns | +265.02% |
| std_parallel | 80021ns | 80021ns | +2071.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2825ns | base | --- | [2767, 3419] | --- | --- | --- | --- |
| head_tail_dyn | 2808ns | no significant difference | [-431, +6]ns | [2767, 2902] | no | 0.1435 | 0.1435 | 1 |
| kernel_nway | 22319ns | +19193.8ns (+679.4%) | [+17750, +22540]ns | [20385, 25929] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2783ns | no significant difference | [-606, +6]ns | [2765, 2823] | no | 0.1435 | 0.1435 | 1 |
| nway | 53329ns | +50291.7ns (+1780.2%) | [+49840, +52598]ns | [52765, 55962] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 13240ns | +10202.0ns (+361.1%) | [+9377, +11131]ns | [12446, 13898] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 78340ns | +75427.1ns (+2670.0%) | [+74867, +77462]ns | [77629, 80506] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2781ns | -0.1% | +866.3% | +0.4% | +1832.7% | +399.6% | +2797.3% |
| 1 | 2 | 3571ns | -20.6% | +488.0% | -22.2% | +1431.4% | +248.4% | +2053.4% |
| 1 | 3 | 2785ns | +1.9% | +658.8% | +0.7% | +1796.7% | +396.1% | +2701.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.667 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.132 | ok |
| kernel_nway | -0.179 | ok |
| kernel_seq | -0.499 | moderate- |
| nway | -0.501 | HIGH- (thermal bounce) |
| sequential | -0.665 | HIGH- (thermal bounce) |
| std_parallel | -0.364 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 113981.7ns | 3053.0ns | 3733.4% | HIGH |
| head_tail_dyn | 109595.1ns | 2846.8ns | 3849.8% | HIGH |
| kernel_nway | 123898.1ns | 23475.0ns | 527.8% | HIGH |
| kernel_seq | 110548.1ns | 2798.8ns | 3949.8% | HIGH |
| nway | 147282.9ns | 54205.8ns | 271.7% | HIGH |
| sequential | 113275.5ns | 13248.6ns | 855.0% | HIGH |
| std_parallel | 168216.4ns | 79574.8ns | 211.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 2741.7-3561.1 ns)
   2741.7 |########################################
   2782.6 |##############################
   2823.6 |##########
   2864.6 |##########
   2905.6 |##########
   2946.5 |
   2987.5 |
   3028.5 |
   3069.4 |
   3110.4 |
   3151.4 |
   3192.4 |
   3233.3 |
   3274.3 |
   3315.3 |##########
   3356.3 |
   3397.2 |
   3438.2 |
   3479.2 |##########
   3520.1 |##########
  (2 below, 3 above range)

head_tail_dyn (n=18, range 2754.2-2971.5 ns)
   2754.2 |########################################
   2765.0 |##########################
   2775.9 |#############
   2786.8 |##########################
   2797.6 |
   2808.5 |
   2819.4 |##########################
   2830.2 |
   2841.1 |#############
   2852.0 |
   2862.8 |
   2873.7 |
   2884.6 |
   2895.5 |##########################
   2906.3 |
   2917.2 |#############
   2928.1 |
   2938.9 |#############
   2949.8 |
   2960.7 |
  (1 below, 2 above range)

kernel_nway (n=18, range 18882.0-28066.7 ns)
  18882.0 |####################
  19341.2 |########################################
  19800.4 |####################
  20259.7 |
  20718.9 |####################
  21178.1 |####################
  21637.4 |####################
  22096.6 |########################################
  22555.8 |
  23015.1 |
  23474.3 |
  23933.6 |####################
  24392.8 |
  24852.0 |####################
  25311.3 |
  25770.5 |########################################
  26229.7 |
  26689.0 |########################################
  27148.2 |
  27607.4 |
  (1 below, 2 above range)

kernel_seq (n=18, range 2733.3-2860.4 ns)
   2733.3 |####################
   2739.7 |
   2746.0 |
   2752.4 |########################################
   2758.7 |####################
   2765.1 |####################
   2771.5 |####################
   2777.8 |########################################
   2784.2 |####################
   2790.5 |
   2796.9 |
   2803.2 |########################################
   2809.6 |
   2815.9 |####################
   2822.3 |####################
   2828.6 |
   2835.0 |####################
   2841.3 |
   2847.7 |
   2854.0 |
  (1 below, 3 above range)

nway (n=18, range 51473.6-56842.4 ns)
  51473.6 |
  51742.1 |#############
  52010.5 |#############
  52278.9 |
  52547.4 |########################################
  52815.8 |##########################
  53084.3 |##########################
  53352.7 |
  53621.1 |
  53889.6 |
  54158.0 |#############
  54426.4 |
  54694.9 |
  54963.3 |
  55231.8 |#############
  55500.2 |
  55768.6 |#############
  56037.1 |########################################
  56305.5 |
  56573.9 |
  (1 below, 2 above range)

sequential (n=18, range 12418.1-14095.1 ns)
  12418.1 |########################################
  12501.9 |######
  12585.8 |
  12669.6 |
  12753.5 |
  12837.3 |
  12921.2 |
  13005.0 |######
  13088.9 |
  13172.7 |
  13256.6 |
  13340.5 |######
  13424.3 |
  13508.2 |
  13592.0 |
  13675.9 |
  13759.7 |
  13843.6 |##########################
  13927.4 |######
  14011.3 |
  (1 below, 3 above range)

std_parallel (n=18, range 76297.2-83286.8 ns)
  76297.2 |
  76646.7 |##########################
  76996.2 |
  77345.7 |########################################
  77695.2 |#############
  78044.6 |##########################
  78394.1 |
  78743.6 |
  79093.1 |
  79442.5 |##########################
  79792.0 |
  80141.5 |#############
  80491.0 |#############
  80840.5 |
  81189.9 |#############
  81539.4 |
  81888.9 |
  82238.4 |
  82587.9 |
  82937.3 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=4027.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3931.4% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=543.6% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3969.3% of algo (FFI overhead may distort results)
- **nway**: bridge=275.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=845.0% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=213.9% of algo (FFI overhead may distort results)

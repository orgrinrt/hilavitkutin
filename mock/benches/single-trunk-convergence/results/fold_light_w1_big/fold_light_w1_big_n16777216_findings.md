# fold_light, far above cache, W = 1

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail dominates: 24% faster than the next best (nway)

head_tail (3.89 ms) leads nway (4.82 ms) by 24%, a clear separation rather than a photo finish. CV 42.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 3.0x slower than the field

std_parallel (11.80 ms) is 3.0x the fastest (3.89 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.65)

head_tail's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (3.89 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, nway, sequential, kernel_seq, kernel_nway, head_tail_dyn} vs {std_parallel} (78% apart)

The field splits into a fast tier {head_tail, nway, sequential, kernel_seq, kernel_nway, head_tail_dyn} and a slow tier {std_parallel} with a 78% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.0x the fastest

Fastest head_tail (3.89 ms) to slowest std_parallel (11.80 ms): 3.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### head_tail_dyn is inconsistent: worst-20% is 3.7x its best-20%

head_tail_dyn's best 20% of batches run at 3.14 ms but its worst 20% at 11.63 ms (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 3887577.9 ns median
- 2 variants significantly slower than baseline
- Spread: 3.03x (fastest 3887577.9 ns, slowest 11795146.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 1 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100350026 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 4415364ns | 3891561ns | 3023901ns | 3665647ns | 6235671ns | base |
| head_tail_dyn | 7276752ns | 6663933ns | 3141712ns | 5745803ns | 11640696ns | +64.81% |
| kernel_nway | 6806024ns | 5835233ns | 3431826ns | 5172640ns | 10943198ns | +54.14% |
| kernel_seq | 5498643ns | 5333302ns | 3318453ns | 4665270ns | 7838798ns | +24.53% |
| nway | 6067497ns | 4821741ns | 3847678ns | 4549831ns | 9453905ns | +37.42% |
| sequential | 5945795ns | 4881255ns | 2997244ns | 4325273ns | 9850853ns | +34.66% |
| std_parallel | 11615435ns | 11799882ns | 10156434ns | 11412855ns | 12648807ns | +163.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4410889ns | 3021713ns | 6228817ns | base | 3.804 |
| head_tail_dyn | 7261076ns | 3139469ns | 11633332ns | +64.62% | 2.311 |
| kernel_nway | 6800335ns | 3428356ns | 10935503ns | +54.17% | 2.467 |
| kernel_seq | 5493691ns | 3315618ns | 7831692ns | +24.55% | 3.054 |
| nway | 6064699ns | 3845602ns | 9450357ns | +37.49% | 2.766 |
| sequential | 5943511ns | 2995960ns | 9847733ns | +34.75% | 2.823 |
| std_parallel | 11610939ns | 10153568ns | 12643958ns | +163.23% | 1.445 |

## Performance model

- Peak throughput: **5.600 Gops/s** (sequential; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 4.316 | 77.1% |
| head_tail_dyn | 2.532 | 45.2% |
| kernel_nway | 2.877 | 51.4% |
| kernel_seq | 3.149 | 56.2% |
| nway | 3.481 | 62.2% |
| sequential | 3.439 | 61.4% |
| std_parallel | 1.422 | 25.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4415364ns | 4415364ns | base |
| head_tail_dyn | 7276752ns | 7276752ns | +64.81% |
| kernel_nway | 6806024ns | 6806024ns | +54.14% |
| kernel_seq | 5498643ns | 5498643ns | +24.53% |
| nway | 6067497ns | 6067497ns | +37.42% |
| sequential | 5945795ns | 5945795ns | +34.66% |
| std_parallel | 11615435ns | 11615435ns | +163.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 3887578ns | base | --- | [3116272, 6228817] | --- | --- | --- | --- |
| head_tail_dyn | 6627365ns | no significant difference | [-4314, +7936094]ns | [3522531, 11633332] | no | 0.4375 | 0.2188 | 0 |
| kernel_nway | 5830842ns | no significant difference | [-425398, +7238265]ns | [3634658, 10935503] | no | 0.6875 | 0.6875 | 0 |
| kernel_seq | 5328300ns | no significant difference | [-531808, +3374930]ns | [3321081, 7831692] | no | 0.6875 | 0.6875 | 0 |
| nway | 4819005ns | +618123.1ns (+15.9%) | [+306657, +4036650]ns | [3924734, 9450357] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| sequential | 4878892ns | no significant difference | [-2090970, +5876157]ns | [3103908, 9847733] | no | 0.6875 | 0.6875 | 0 |
| std_parallel | 11795147ns | +7438939.4ns (+191.4%) | [+5391146, +8770066]ns | [10393713, 12643958] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 3971576ns | +3.5% | -8.5% | +48.1% | +13.0% | +148.0% | +216.5% |
| 1 | 2 | 5563852ns | +8.5% | +4.8% | -7.7% | +7.3% | -37.6% | +113.4% |
| 1 | 3 | 3697238ns | +214.6% | +195.8% | +47.9% | +109.2% | +22.0% | +181.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.654 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.049 | ok |
| kernel_nway | -0.033 | ok |
| kernel_seq | -0.464 | moderate- |
| nway | -0.002 | ok |
| sequential | -0.261 | moderate- |
| std_parallel | -0.027 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 4811366.3ns | 4410888.7ns | 109.1% | HIGH |
| head_tail_dyn | 7445416.8ns | 7261076.1ns | 102.5% | HIGH |
| kernel_nway | 7191735.2ns | 6800334.5ns | 105.8% | HIGH |
| kernel_seq | 5997137.7ns | 5493690.8ns | 109.2% | HIGH |
| nway | 6554224.6ns | 6064698.8ns | 108.1% | HIGH |
| sequential | 6180392.9ns | 5943511.1ns | 104.0% | HIGH |
| std_parallel | 11554002.4ns | 11610939.2ns | 99.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 3021713.1-6228816.6 ns)
  3021713.1 |########################################
  3182068.3 |########################################
  3342423.5 |########################################
  3502778.6 |
  3663133.8 |
  3823489.0 |
  3983844.1 |
  4144199.3 |
  4304554.5 |########################################
  4464909.7 |########################################
  4625264.8 |
  4785620.0 |
  4945975.2 |
  5106330.4 |
  5266685.5 |
  5427040.7 |
  5587395.9 |
  5747751.1 |
  5908106.2 |
  6068461.4 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 3139468.9-11633332.3 ns)
  3139468.9 |########################################
  3564162.1 |########################################
  3988855.2 |
  4413548.4 |
  4838241.6 |########################################
  5262934.8 |
  5687627.9 |
  6112321.1 |
  6537014.3 |
  6961707.5 |
  7386400.6 |
  7811093.8 |########################################
  8235787.0 |
  8660480.1 |
  9085173.3 |
  9509866.5 |
  9934559.7 |########################################
  10359252.8 |
  10783946.0 |
  11208639.2 |
  (0 below, 1 above range)

kernel_nway (n=6, range 3428355.9-10935503.2 ns)
  3428355.9 |####################
  3803713.3 |########################################
  4179070.6 |
  4554428.0 |
  4929785.4 |
  5305142.7 |
  5680500.1 |
  6055857.5 |
  6431214.8 |
  6806572.2 |
  7181929.5 |
  7557286.9 |####################
  7932644.3 |
  8308001.6 |
  8683359.0 |
  9058716.4 |
  9434073.7 |
  9809431.1 |####################
  10184788.5 |
  10560145.8 |
  (0 below, 1 above range)

kernel_seq (n=6, range 3315618.5-7831691.7 ns)
  3315618.5 |########################################
  3541422.2 |####################
  3767225.8 |
  3993029.5 |
  4218833.1 |
  4444636.8 |
  4670440.5 |
  4896244.1 |
  5122047.8 |
  5347851.4 |
  5573655.1 |
  5799458.8 |
  6025262.4 |
  6251066.1 |
  6476869.7 |
  6702673.4 |
  6928477.1 |####################
  7154280.7 |####################
  7380084.4 |
  7605888.0 |
  (0 below, 1 above range)

nway (n=6, range 3845601.9-9450357.1 ns)
  3845601.9 |########################################
  4125839.7 |
  4406077.4 |####################
  4686315.2 |
  4966552.9 |####################
  5246790.7 |
  5527028.4 |
  5807266.2 |
  6087504.0 |
  6367741.7 |
  6647979.5 |
  6928217.2 |
  7208455.0 |
  7488692.7 |
  7768930.5 |
  8049168.3 |####################
  8329406.0 |
  8609643.8 |
  8889881.5 |
  9170119.3 |
  (0 below, 1 above range)

sequential (n=6, range 2995960.0-9847732.9 ns)
  2995960.0 |########################################
  3338548.6 |
  3681137.3 |####################
  4023725.9 |
  4366314.6 |
  4708903.2 |
  5051491.9 |
  5394080.5 |
  5736669.2 |####################
  6079257.8 |
  6421846.5 |
  6764435.1 |
  7107023.8 |
  7449612.4 |
  7792201.1 |
  8134789.7 |
  8477378.4 |
  8819967.0 |
  9162555.7 |
  9505144.3 |####################
  (0 below, 1 above range)

std_parallel (n=6, range 10153568.1-12643958.2 ns)
  10153568.1 |########################################
  10278087.6 |
  10402607.1 |
  10527126.6 |########################################
  10651646.1 |
  10776165.6 |
  10900685.1 |
  11025204.6 |
  11149724.1 |
  11274243.6 |########################################
  11398763.1 |
  11523282.7 |
  11647802.2 |
  11772321.7 |
  11896841.2 |
  12021360.7 |
  12145880.2 |
  12270399.7 |########################################
  12394919.2 |########################################
  12519438.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=37.8% (high variance, measurements may be unstable)
- **head_tail**: bridge=106.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=48.9% (high variance, measurements may be unstable)
- **head_tail_dyn**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **head_tail_dyn**: bridge=113.4% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=48.4% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **kernel_nway**: bridge=115.4% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=38.2% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=105.3% of algo (FFI overhead may distort results)
- **nway**: CV=42.0% (high variance, measurements may be unstable)
- **nway**: bridge=113.8% of algo (FFI overhead may distort results)
- **sequential**: CV=49.0% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **sequential**: bridge=117.2% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=96.6% of algo (FFI overhead may distort results)

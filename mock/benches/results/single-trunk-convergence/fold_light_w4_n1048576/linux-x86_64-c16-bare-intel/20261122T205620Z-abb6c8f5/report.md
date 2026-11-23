# fold_light, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail_dyn is an outlier: 2.1x slower than the field

head_tail_dyn (203.50 us) is 2.1x the fastest (95.18 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail is fastest but the noisiest (CV 126.2%)

head_tail wins on median (95.18 us) yet has the highest variance (CV 126.2%), while sequential is the steadiest (CV 4.9%, 102.14 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### head_tail shows alternating (throttle bounce) (autocorr -0.65)

head_tail's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (108.32 us) is smaller than the fastest variant's own run-to-run std-dev (120.11 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (95.18 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, sequential, kernel_nway, kernel_seq} vs {std_parallel, nway, head_tail_dyn} (37% apart)

The field splits into a fast tier {head_tail, sequential, kernel_nway, kernel_seq} and a slow tier {std_parallel, nway, head_tail_dyn} with a 37% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader head_tail vs stability leader sequential (+7% speed for 25.7x steadier)

head_tail is fastest (95.18 us, CV 126.2%); sequential gives up 7.3% median for 25.7x lower variance (CV 4.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### kernel_nway is inconsistent: worst-20% is 5.4x its best-20%

kernel_nway's best 20% of batches run at 51.11 us but its worst 20% at 276.68 us (5.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 95175.3 ns median
- 4 variants significantly slower than baseline
- Spread: 2.14x (fastest 95175.3 ns, slowest 203495.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 4 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112780093 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 154235ns | 95336ns | 85359ns | 93447ns | 279856ns | base |
| head_tail_dyn | 237216ns | 204638ns | 178749ns | 201563ns | 319928ns | +53.80% |
| kernel_nway | 147323ns | 109107ns | 51323ns | 91372ns | 279248ns | -4.48% |
| kernel_seq | 133146ns | 113471ns | 102162ns | 111676ns | 180843ns | -13.67% |
| nway | 249258ns | 176162ns | 109638ns | 163734ns | 447355ns | +61.61% |
| sequential | 101736ns | 102405ns | 93366ns | 101013ns | 107007ns | -34.04% |
| std_parallel | 165770ns | 155546ns | 105376ns | 146887ns | 224292ns | +7.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 153583ns | 85183ns | 278305ns | base | 6.827 |
| head_tail_dyn | 236250ns | 178358ns | 318429ns | +53.83% | 4.438 |
| kernel_nway | 146189ns | 51112ns | 276678ns | -4.81% | 7.173 |
| kernel_seq | 132523ns | 101958ns | 179557ns | -13.71% | 7.912 |
| nway | 248467ns | 109370ns | 445890ns | +61.78% | 4.220 |
| sequential | 101465ns | 93104ns | 106744ns | -33.93% | 10.334 |
| std_parallel | 164762ns | 105102ns | 222529ns | +7.28% | 6.364 |

## Performance model

- Peak throughput: **20.515 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 11.017 | 53.7% |
| head_tail_dyn | 5.153 | 25.1% |
| kernel_nway | 9.655 | 47.1% |
| kernel_seq | 9.267 | 45.2% |
| nway | 5.973 | 29.1% |
| sequential | 10.266 | 50.0% |
| std_parallel | 6.770 | 33.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 154235ns | 154235ns | base |
| head_tail_dyn | 237216ns | 237216ns | +53.80% |
| kernel_nway | 147323ns | 147323ns | -4.48% |
| kernel_seq | 133146ns | 133146ns | -13.67% |
| nway | 249258ns | 249258ns | +61.61% |
| sequential | 101736ns | 101736ns | -34.04% |
| std_parallel | 165770ns | 165770ns | +7.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 95175ns | base | --- | [88386, 138806] | --- | --- | --- | --- |
| head_tail_dyn | 203495ns | +93592.9ns (+98.3%) | [+71639, +115163]ns | [181416, 280113] | YES | 0.0009 | 0.0001 | 0 |
| kernel_nway | 108602ns | no significant difference | [-30279, +24153]ns | [55832, 147184] | no | 0.4807 | 0.4807 | 0 |
| kernel_seq | 113156ns | +16217.9ns (+17.0%) | [+2546, +22411]ns | [104370, 137332] | YES | 0.0463 | 0.0309 | 0 |
| nway | 175551ns | +36968.4ns (+38.8%) | [+11021, +238230]ns | [137898, 333259] | YES | 0.0463 | 0.0309 | 0 |
| sequential | 102138ns | no significant difference | [-29164, +10388]ns | [98715, 106224] | no | 0.2855 | 0.2379 | 0 |
| std_parallel | 154894ns | +36165.8ns (+38.0%) | [+14826, +46937]ns | [126303, 207488] | YES | 0.0226 | 0.0075 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 97423ns | +84.5% | +4.9% | +7.4% | +215.3% | +9.6% | +34.0% |
| 1 | 2 | 86111ns | +113.1% | -35.2% | +21.5% | +66.7% | +9.9% | +116.7% |
| 1 | 3 | 95536ns | +198.9% | +105.8% | +43.7% | +49.4% | +6.9% | +66.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.651 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.149 | ok |
| kernel_nway | -0.378 | moderate- |
| kernel_seq | -0.166 | ok |
| nway | -0.164 | ok |
| sequential | -0.572 | HIGH- (thermal bounce) |
| std_parallel | -0.494 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 159384.6ns | 153582.9ns | 103.8% | HIGH |
| head_tail_dyn | 236456.2ns | 236249.5ns | 100.1% | HIGH |
| kernel_nway | 146818.8ns | 146188.5ns | 100.4% | HIGH |
| kernel_seq | 133429.6ns | 132523.5ns | 100.7% | HIGH |
| nway | 248419.7ns | 248467.2ns | 100.0% | HIGH |
| sequential | 101716.1ns | 101465.4ns | 100.2% | HIGH |
| std_parallel | 165285.6ns | 164761.6ns | 100.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 85182.8-278304.8 ns)
  85182.8 |########################################
  94838.9 |#################################
  104495.0 |
  114151.1 |
  123807.2 |######
  133463.3 |
  143119.4 |######
  152775.5 |
  162431.6 |
  172087.7 |
  181743.8 |
  191399.9 |
  201056.0 |
  210712.1 |
  220368.2 |#############
  230024.3 |
  239680.4 |
  249336.5 |
  258992.6 |
  268648.7 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 178358.3-318428.9 ns)
  178358.3 |########################################
  185361.8 |
  192365.3 |
  199368.9 |
  206372.4 |
  213375.9 |
  220379.5 |###########
  227383.0 |
  234386.5 |
  241390.0 |
  248393.6 |
  255397.1 |
  262400.6 |#####
  269404.2 |#####
  276407.7 |
  283411.2 |######################
  290414.8 |
  297418.3 |
  304421.8 |
  311425.4 |
  (2 below, 1 above range)

kernel_nway (n=18, range 51112.4-276677.8 ns)
  51112.4 |########################################
  62390.7 |
  73668.9 |
  84947.2 |####################
  96225.5 |##########
  107503.7 |##########
  118782.0 |####################
  130060.3 |##########
  141338.5 |
  152616.8 |
  163895.1 |##########
  175173.3 |
  186451.6 |
  197729.9 |
  209008.1 |
  220286.4 |##########
  231564.7 |
  242842.9 |
  254121.2 |
  265399.5 |
  (2 below, 3 above range)

kernel_seq (n=18, range 101958.4-179556.5 ns)
  101958.4 |########################################
  105838.3 |
  109718.2 |######
  113598.1 |######
  117478.0 |######
  121358.0 |
  125237.9 |
  129117.8 |
  132997.7 |######
  136877.6 |####################
  140757.5 |
  144637.4 |
  148517.3 |
  152397.2 |######
  156277.1 |
  160157.0 |
  164036.9 |
  167916.8 |
  171796.7 |
  175676.6 |
  (2 below, 2 above range)

nway (n=18, range 109370.2-445889.8 ns)
  109370.2 |##########################
  126196.2 |########################################
  143022.2 |##########################
  159848.1 |#############
  176674.1 |#############
  193500.1 |#############
  210326.1 |#############
  227152.1 |
  243978.0 |
  260804.0 |
  277630.0 |
  294456.0 |
  311282.0 |
  328107.9 |##########################
  344933.9 |
  361759.9 |
  378585.9 |#############
  395411.9 |
  412237.8 |
  429063.8 |
  (1 below, 3 above range)

sequential (n=18, range 93103.8-106743.8 ns)
  93103.8 |
  93785.8 |#############
  94467.8 |#############
  95149.8 |
  95831.8 |
  96513.8 |
  97195.8 |#############
  97877.8 |
  98559.8 |
  99241.8 |
  99923.8 |#############
  100605.8 |
  101287.8 |##########################
  101969.8 |##########################
  102651.8 |#############
  103333.8 |
  104015.8 |#############
  104697.8 |
  105379.8 |
  106061.8 |########################################
  (2 below, 3 above range)

std_parallel (n=18, range 105101.9-222529.3 ns)
  105101.9 |####################
  110973.2 |####################
  116844.6 |####################
  122716.0 |########################################
  128587.4 |####################
  134458.7 |####################
  140330.1 |####################
  146201.5 |
  152072.8 |
  157944.2 |
  163815.6 |####################
  169687.0 |
  175558.3 |####################
  181429.7 |####################
  187301.1 |
  193172.4 |
  199043.8 |
  204915.2 |########################################
  210786.6 |
  216657.9 |
  (1 below, 4 above range)

```

## Diagnostics

- **head_tail**: CV=78.2% (high variance, measurements may be unstable)
- **head_tail**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **head_tail**: bridge=100.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=32.2% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=101.6% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=79.4% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 5.4x (possible bimodal distribution)
- **kernel_nway**: bridge=100.5% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=37.3% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=101.1% of algo (FFI overhead may distort results)
- **nway**: CV=61.6% (high variance, measurements may be unstable)
- **nway**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **nway**: bridge=99.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.2% of algo (FFI overhead may distort results)
- **std_parallel**: CV=28.5% (high variance, measurements may be unstable)
- **std_parallel**: bridge=108.5% of algo (FFI overhead may distort results)

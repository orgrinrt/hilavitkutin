# fold_heavy, frames of tens of milliseconds, W = 2

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.67)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_nway, nway, std_parallel, head_tail_dyn, head_tail} vs {sequential, kernel_seq} (67% apart)

The field splits into a fast tier {kernel_nway, nway, std_parallel, head_tail_dyn, head_tail} and a slow tier {sequential, kernel_seq} with a 67% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: kernel_nway** at 412932.3 ns median (-8.2% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.83x (fastest 412932.3 ns, slowest 754963.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, frames of tens of milliseconds, W = 2 |
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
| head_tail | 452049ns | 450635ns | 437177ns | 446243ns | 468193ns | base |
| head_tail_dyn | 447491ns | 439740ns | 438125ns | 439372ns | 464354ns | -1.01% |
| kernel_nway | 415938ns | 413688ns | 408438ns | 412618ns | 424667ns | -7.99% |
| kernel_seq | 764470ns | 755708ns | 745719ns | 752483ns | 791828ns | +69.11% |
| nway | 419370ns | 419286ns | 406760ns | 416198ns | 430432ns | -7.23% |
| sequential | 758300ns | 752495ns | 747438ns | 751847ns | 773412ns | +67.75% |
| std_parallel | 435668ns | 437755ns | 422698ns | 432740ns | 446547ns | -3.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 451226ns | 436458ns | 467214ns | base | 2.324 |
| head_tail_dyn | 446750ns | 437510ns | 463338ns | -0.99% | 2.347 |
| kernel_nway | 414958ns | 407688ns | 423245ns | -8.04% | 2.527 |
| kernel_seq | 762997ns | 745510ns | 788448ns | +69.09% | 1.374 |
| nway | 418642ns | 406188ns | 429609ns | -7.22% | 2.505 |
| sequential | 757898ns | 747208ns | 772760ns | +67.96% | 1.384 |
| std_parallel | 435182ns | 422167ns | 445964ns | -3.56% | 2.410 |

## Performance model

- Peak throughput: **2.582 Gops/s** (nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 2.331 | 90.3% |
| head_tail_dyn | 2.388 | 92.5% |
| kernel_nway | 2.539 | 98.4% |
| kernel_seq | 1.389 | 53.8% |
| nway | 2.505 | 97.0% |
| sequential | 1.394 | 54.0% |
| std_parallel | 2.398 | 92.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 452049ns | 452049ns | base |
| head_tail_dyn | 447491ns | 447491ns | -1.01% |
| kernel_nway | 415938ns | 415938ns | -7.99% |
| kernel_seq | 764470ns | 764470ns | +69.11% |
| nway | 419370ns | 419370ns | -7.23% |
| sequential | 758300ns | 758300ns | +67.75% |
| std_parallel | 435668ns | 435668ns | -3.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 449786ns | base | --- | [436677, 467214] | --- | --- | --- | --- |
| head_tail_dyn | 439182ns | no significant difference | [-19875, +11036]ns | [437729, 463338] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 412932ns | -34541.6ns (-7.7%) | [-50120, -24141]ns | [408698, 423245] | YES | 0.0469 | 0.0313 | 0 |
| kernel_seq | 754964ns | +314015.7ns (+69.8%) | [+287094, +334203]ns | [745578, 788448] | YES | 0.0469 | 0.0313 | 0 |
| nway | 418589ns | -37833.3ns (-8.4%) | [-52849, -7068]ns | [407729, 429609] | YES | 0.0469 | 0.0313 | 0 |
| sequential | 752214ns | +307078.2ns (+68.3%) | [+291745, +321193]ns | [748719, 772760] | YES | 0.0469 | 0.0313 | 0 |
| std_parallel | 437344ns | no significant difference | [-36891, +2958]ns | [422240, 445964] | no | 0.8250 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 436677ns | +0.4% | -5.6% | +72.9% | -1.6% | +71.5% | -1.1% |
| 1 | 2 | 458484ns | +1.1% | -10.7% | +62.6% | -11.1% | +64.1% | -6.4% |
| 1 | 3 | 458516ns | -4.3% | -7.7% | +72.0% | -8.7% | +68.5% | -3.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.166 | ok |
| head_tail_dyn | -0.667 | HIGH- (thermal bounce) |
| kernel_nway | -0.302 | moderate- |
| kernel_seq | -0.299 | moderate- |
| nway | -0.498 | moderate- |
| sequential | -0.096 | ok |
| std_parallel | -0.278 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 441213.5ns | 451225.7ns | 97.8% | HIGH |
| head_tail_dyn | 449484.4ns | 446750.0ns | 100.6% | HIGH |
| kernel_nway | 413177.1ns | 414958.3ns | 99.6% | HIGH |
| kernel_seq | 767055.6ns | 762996.5ns | 100.5% | HIGH |
| nway | 416414.9ns | 418642.4ns | 99.5% | HIGH |
| sequential | 756203.1ns | 757897.6ns | 99.8% | HIGH |
| std_parallel | 433875.0ns | 435182.3ns | 99.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 436458.3-467213.6 ns)
  436458.3 |########################################
  437996.1 |
  439533.8 |
  441071.6 |
  442609.3 |
  444147.1 |
  445684.9 |
  447222.6 |
  448760.4 |########################################
  450298.2 |
  451835.9 |
  453373.7 |
  454911.5 |
  456449.2 |
  457987.0 |
  459524.7 |
  461062.5 |
  462600.3 |
  464138.0 |
  465675.8 |####################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 437510.4-463338.5 ns)
  437510.4 |########################################
  438801.8 |########################################
  440093.2 |
  441384.6 |
  442676.0 |
  443967.4 |
  445258.8 |
  446550.2 |
  447841.6 |
  449133.0 |
  450424.5 |
  451715.9 |
  453007.3 |
  454298.7 |
  455590.1 |
  456881.5 |####################
  458172.9 |
  459464.3 |
  460755.7 |
  462047.1 |
  (0 below, 1 above range)

kernel_nway (n=6, range 407687.5-423244.8 ns)
  407687.5 |########################################
  408465.4 |
  409243.2 |########################################
  410021.1 |
  410799.0 |########################################
  411576.8 |
  412354.7 |
  413132.6 |
  413910.4 |
  414688.3 |########################################
  415466.2 |
  416244.0 |
  417021.9 |
  417799.7 |
  418577.6 |
  419355.5 |
  420133.3 |
  420911.2 |
  421689.1 |
  422466.9 |########################################
  (0 below, 1 above range)

kernel_seq (n=6, range 745510.4-788447.9 ns)
  745510.4 |########################################
  747657.3 |####################
  749804.2 |
  751951.0 |
  754097.9 |
  756244.8 |
  758391.7 |
  760538.5 |####################
  762685.4 |
  764832.3 |
  766979.2 |
  769126.1 |
  771272.9 |
  773419.8 |
  775566.7 |
  777713.6 |
  779860.4 |
  782007.3 |####################
  784154.2 |
  786301.1 |
  (0 below, 1 above range)

nway (n=6, range 406187.5-429609.3 ns)
  406187.5 |########################################
  407358.6 |
  408529.7 |########################################
  409700.8 |
  410871.9 |
  412043.0 |
  413214.1 |
  414385.1 |
  415556.2 |
  416727.3 |########################################
  417898.4 |
  419069.5 |########################################
  420240.6 |
  421411.7 |
  422582.8 |########################################
  423753.9 |
  424925.0 |
  426096.1 |
  427267.2 |
  428438.3 |
  (0 below, 1 above range)

sequential (n=6, range 747208.3-772760.4 ns)
  747208.3 |####################
  748485.9 |
  749763.5 |########################################
  751041.1 |
  752318.7 |
  753596.3 |####################
  754873.9 |
  756151.6 |
  757429.2 |
  758706.8 |
  759984.4 |
  761262.0 |
  762539.6 |
  763817.2 |
  765094.8 |
  766372.4 |####################
  767650.0 |
  768927.6 |
  770205.2 |
  771482.8 |
  (0 below, 1 above range)

std_parallel (n=6, range 422166.7-445963.5 ns)
  422166.7 |########################################
  423356.5 |
  424546.4 |
  425736.2 |
  426926.1 |
  428115.9 |
  429305.7 |
  430495.6 |
  431685.4 |
  432875.3 |
  434065.1 |
  435254.9 |####################
  436444.8 |
  437634.6 |####################
  438824.5 |
  440014.3 |
  441204.1 |####################
  442394.0 |
  443583.8 |
  444773.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=97.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.3% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=99.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=101.3% of algo (FFI overhead may distort results)
- **nway**: bridge=99.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=98.9% of algo (FFI overhead may distort results)

# fold_heavy, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 2.3x slower than the field

std_parallel (100.83 us) is 2.3x the fastest (42.91 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.67)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### sequential is inconsistent: worst-20% is 7.7x its best-20%

sequential's best 20% of batches run at 18.43 us but its worst 20% at 141.50 us (7.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 42911.6 ns median (-1.8% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 2.35x (fastest 42911.6 ns, slowest 100832.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 1 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 46851ns | 44129ns | 39962ns | 43365ns | 55525ns | base |
| head_tail_dyn | 47820ns | 45315ns | 27124ns | 41841ns | 67138ns | +2.07% |
| kernel_nway | 46929ns | 47244ns | 22682ns | 44813ns | 62227ns | +0.17% |
| kernel_seq | 50638ns | 43450ns | 33089ns | 41848ns | 72598ns | +8.08% |
| nway | 79148ns | 75878ns | 63414ns | 73061ns | 96146ns | +68.93% |
| sequential | 74807ns | 54741ns | 18575ns | 48321ns | 142652ns | +59.67% |
| std_parallel | 105977ns | 101474ns | 89397ns | 99276ns | 124318ns | +126.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 46006ns | 39539ns | 53870ns | base | 0.089 |
| head_tail_dyn | 47147ns | 26905ns | 66027ns | +2.48% | 0.087 |
| kernel_nway | 46062ns | 22412ns | 60866ns | +0.12% | 0.089 |
| kernel_seq | 49940ns | 32922ns | 71528ns | +8.55% | 0.082 |
| nway | 78607ns | 62908ns | 95493ns | +70.86% | 0.052 |
| sequential | 74204ns | 18432ns | 141497ns | +61.29% | 0.055 |
| std_parallel | 105105ns | 88779ns | 123147ns | +128.46% | 0.039 |

## Performance model

- Peak throughput: **0.222 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.094 | 42.2% |
| head_tail_dyn | 0.091 | 41.1% |
| kernel_nway | 0.088 | 39.5% |
| kernel_seq | 0.095 | 43.0% |
| nway | 0.054 | 24.5% |
| sequential | 0.075 | 33.9% |
| std_parallel | 0.041 | 18.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 46851ns | 46851ns | base |
| head_tail_dyn | 47820ns | 47820ns | +2.07% |
| kernel_nway | 46929ns | 46929ns | +0.17% |
| kernel_seq | 50638ns | 50638ns | +8.08% |
| nway | 79148ns | 79148ns | +68.93% |
| sequential | 74807ns | 74807ns | +59.67% |
| std_parallel | 105977ns | 105977ns | +126.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 43696ns | base | --- | [41390, 47598] | --- | --- | --- | --- |
| head_tail_dyn | 44848ns | no significant difference | [-5975, +8616]ns | [36296, 53450] | no | 0.3568 | 0.2379 | 0 |
| kernel_nway | 46651ns | no significant difference | [-2668, +5844]ns | [40793, 53148] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 42912ns | no significant difference | [-6309, +5278]ns | [34559, 60030] | no | 0.9774 | 0.8145 | 0 |
| nway | 75234ns | +31069.3ns (+71.1%) | [+25865, +41622]ns | [67191, 86970] | YES | 0.0004 | 0.0001 | 0 |
| sequential | 54291ns | no significant difference | [-7585, +14031]ns | [40680, 64164] | no | 0.3568 | 0.2379 | 0 |
| std_parallel | 100832ns | +56397.7ns (+129.1%) | [+51273, +63312]ns | [94935, 107371] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 43043ns | -3.3% | -3.0% | -0.3% | +67.1% | +12.7% | +128.0% |
| 1 | 2 | 44834ns | +19.2% | +11.5% | +1.1% | +94.0% | +242.2% | +149.8% |
| 1 | 3 | 43397ns | -3.1% | +9.7% | +13.3% | +53.1% | -1.5% | +117.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.643 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.666 | HIGH- (thermal bounce) |
| kernel_nway | -0.346 | moderate- |
| kernel_seq | -0.011 | ok |
| nway | -0.622 | HIGH- (thermal bounce) |
| sequential | -0.665 | HIGH- (thermal bounce) |
| std_parallel | -0.639 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 45572.5ns | 46006.0ns | 99.1% | HIGH |
| head_tail_dyn | 47758.4ns | 47147.1ns | 101.3% | HIGH |
| kernel_nway | 47479.1ns | 46061.5ns | 103.1% | HIGH |
| kernel_seq | 50713.9ns | 49939.6ns | 101.6% | HIGH |
| nway | 79922.3ns | 78607.1ns | 101.7% | HIGH |
| sequential | 75316.4ns | 74204.0ns | 101.5% | HIGH |
| std_parallel | 104363.8ns | 105105.4ns | 99.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 39538.9-53869.6 ns)
  39538.9 |#############
  40255.4 |##########################
  40971.9 |########################################
  41688.5 |#############
  42405.0 |#############
  43121.5 |
  43838.1 |#############
  44554.6 |#############
  45271.1 |
  45987.7 |
  46704.2 |##########################
  47420.8 |
  48137.3 |#############
  48853.8 |
  49570.4 |
  50286.9 |#############
  51003.4 |#############
  51720.0 |#############
  52436.5 |
  53153.0 |
  (1 below, 1 above range)

head_tail_dyn (n=18, range 26904.9-66026.9 ns)
  26904.9 |
  28861.0 |
  30817.1 |#############
  32773.2 |########################################
  34729.3 |#############
  36685.4 |#############
  38641.5 |#############
  40597.6 |
  42553.7 |#############
  44509.8 |
  46465.9 |#############
  48422.0 |##########################
  50378.1 |
  52334.2 |#############
  54290.3 |#############
  56246.4 |
  58202.5 |
  60158.6 |#############
  62114.7 |
  64070.8 |
  (1 below, 3 above range)

kernel_nway (n=18, range 22412.1-60866.4 ns)
  22412.1 |
  24334.8 |
  26257.5 |
  28180.2 |
  30102.9 |
  32025.7 |
  33948.4 |
  35871.1 |####################
  37793.8 |####################
  39716.5 |########################################
  41639.2 |########################################
  43562.0 |
  45484.7 |########################################
  47407.4 |####################
  49330.1 |####################
  51252.8 |####################
  53175.5 |####################
  55098.3 |
  57021.0 |####################
  58943.7 |
  (2 below, 3 above range)

kernel_seq (n=18, range 32922.5-71528.3 ns)
  32922.5 |########################################
  34852.8 |
  36783.0 |
  38713.3 |########
  40643.6 |################
  42573.9 |########
  44504.2 |
  46434.5 |########
  48364.8 |
  50295.1 |
  52225.4 |########
  54155.7 |
  56085.9 |########
  58016.2 |
  59946.5 |
  61876.8 |########
  63807.1 |
  65737.4 |
  67667.7 |########
  69598.0 |
  (1 below, 3 above range)

nway (n=18, range 62907.5-95493.4 ns)
  62907.5 |
  64536.8 |########################################
  66166.1 |########################################
  67795.4 |####################
  69424.7 |####################
  71054.0 |
  72683.3 |
  74312.6 |########################################
  75941.9 |
  77571.2 |
  79200.5 |####################
  80829.8 |
  82459.1 |
  84088.4 |####################
  85717.7 |####################
  87346.9 |####################
  88976.2 |####################
  90605.5 |
  92234.8 |
  93864.1 |
  (2 below, 3 above range)

sequential (n=18, range 18431.5-141497.4 ns)
  18431.5 |##############################
  24584.8 |
  30738.1 |##########
  36891.4 |
  43044.7 |##########
  49198.0 |########################################
  55351.3 |####################
  61504.6 |####################
  67657.8 |
  73811.1 |
  79964.4 |
  86117.7 |
  92271.0 |
  98424.3 |##########
  104577.6 |
  110730.9 |
  116884.2 |
  123037.5 |
  129190.8 |
  135344.1 |
  (1 below, 3 above range)

std_parallel (n=18, range 88778.9-123147.4 ns)
  88778.9 |#############
  90497.4 |
  92215.8 |#############
  93934.2 |########################################
  95652.6 |#############
  97371.1 |#############
  99089.5 |#############
  100807.9 |#############
  102526.3 |##########################
  104244.8 |#############
  105963.2 |
  107681.6 |
  109400.0 |#############
  111118.5 |
  112836.9 |#############
  114555.3 |
  116273.7 |
  117992.2 |
  119710.6 |
  121429.0 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=101.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=35.1% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=99.5% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=32.0% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=102.1% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=34.2% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=102.5% of algo (FFI overhead may distort results)
- **nway**: bridge=102.0% of algo (FFI overhead may distort results)
- **sequential**: CV=84.3% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 7.7x (possible bimodal distribution)
- **sequential**: bridge=101.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=98.0% of algo (FFI overhead may distort results)

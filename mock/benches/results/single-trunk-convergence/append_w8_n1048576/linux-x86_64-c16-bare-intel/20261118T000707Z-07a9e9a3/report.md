# append, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel dominates: 36% faster than the next best (head_tail_dyn)

std_parallel (407.10 us) leads head_tail_dyn (553.66 us) by 36%, a clear separation rather than a photo finish. CV 20.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel beats baseline by 31% (significant)

std_parallel is -173.60 us (31%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### nway is an outlier: 3.8x slower than the field

nway (1.55 ms) is 3.8x the fastest (407.10 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.65)

kernel_nway's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {std_parallel, head_tail_dyn, head_tail, kernel_seq, kernel_nway, sequential} vs {nway} (91% apart)

The field splits into a fast tier {std_parallel, head_tail_dyn, head_tail, kernel_seq, kernel_nway, sequential} and a slow tier {nway} with a 91% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.8x the fastest

Fastest std_parallel (407.10 us) to slowest nway (1.55 ms): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 2.1x its best-20%

kernel_nway's best 20% of batches run at 428.42 us but its worst 20% at 879.68 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_parallel** at 407095.2 ns median (-27.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 3.80x (fastest 407095.2 ns, slowest 1546615.8 ns)

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
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 590292ns | 565624ns | 523914ns | 569280ns | 654997ns | base |
| head_tail_dyn | 573245ns | 555062ns | 505517ns | 550903ns | 640622ns | -2.89% |
| kernel_nway | 689894ns | 686301ns | 430354ns | 648550ns | 881680ns | +16.87% |
| kernel_seq | 598619ns | 608556ns | 508830ns | 576359ns | 676904ns | +1.41% |
| nway | 1550750ns | 1547819ns | 1079578ns | 1513234ns | 1842610ns | +162.71% |
| sequential | 812022ns | 810404ns | 806549ns | 809206ns | 818983ns | +37.56% |
| std_parallel | 421537ns | 408687ns | 296401ns | 399918ns | 516533ns | -28.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 588736ns | 522551ns | 653472ns | base | 1.781 |
| head_tail_dyn | 571715ns | 504161ns | 639020ns | -2.89% | 1.834 |
| kernel_nway | 688004ns | 428418ns | 879682ns | +16.86% | 1.524 |
| kernel_seq | 597267ns | 507620ns | 675326ns | +1.45% | 1.756 |
| nway | 1549494ns | 1078320ns | 1841232ns | +163.19% | 0.677 |
| sequential | 811564ns | 806152ns | 818476ns | +37.85% | 1.292 |
| std_parallel | 419899ns | 294981ns | 514663ns | -28.68% | 2.497 |

## Performance model

- Peak throughput: **3.555 Gops/s** (std_parallel; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.861 | 52.3% |
| head_tail_dyn | 1.894 | 53.3% |
| kernel_nway | 1.533 | 43.1% |
| kernel_seq | 1.726 | 48.6% |
| nway | 0.678 | 19.1% |
| sequential | 1.295 | 36.4% |
| std_parallel | 2.576 | 72.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 590292ns | 590292ns | base |
| head_tail_dyn | 573245ns | 573245ns | -2.89% |
| kernel_nway | 689894ns | 689894ns | +16.87% |
| kernel_seq | 598619ns | 598619ns | +1.41% |
| nway | 1550750ns | 1550750ns | +162.71% |
| sequential | 812022ns | 812022ns | +37.56% |
| std_parallel | 421537ns | 421537ns | -28.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 563496ns | base | --- | [535954, 644980] | --- | --- | --- | --- |
| head_tail_dyn | 553659ns | no significant difference | [-80599, +17488]ns | [541435, 581706] | no | 0.5768 | 0.4807 | 0 |
| kernel_nway | 684096ns | no significant difference | [-6711, +233056]ns | [572290, 787450] | no | 0.5768 | 0.4807 | 0 |
| kernel_seq | 607369ns | no significant difference | [-121294, +84538]ns | [517506, 623432] | no | 0.8145 | 0.8145 | 0 |
| nway | 1546616ns | +955512.2ns (+169.6%) | [+873138, +1215012]ns | [1485628, 1780699] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 809893ns | +249642.4ns (+44.3%) | [+172728, +269054]ns | [807292, 810629] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 407095ns | -173597.2ns (-30.8%) | [-224072, -132719]ns | [380075, 466352] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 653719ns | -13.8% | +2.5% | -21.6% | +109.3% | +24.0% | -28.6% |
| 1 | 2 | 546819ns | +14.3% | +42.4% | +14.3% | +200.7% | +47.6% | -28.6% |
| 1 | 3 | 545656ns | -2.8% | +26.1% | +11.3% | +224.8% | +48.2% | -25.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.161 | ok |
| head_tail_dyn | -0.586 | HIGH- (thermal bounce) |
| kernel_nway | -0.650 | HIGH- (thermal bounce) |
| kernel_seq | -0.257 | moderate- |
| nway | -0.028 | ok |
| sequential | -0.474 | moderate- |
| std_parallel | -0.297 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 4644274.5ns | 588736.0ns | 788.9% | HIGH |
| head_tail_dyn | 4615578.6ns | 571715.2ns | 807.3% | HIGH |
| kernel_nway | 12042208.2ns | 688003.7ns | 1750.3% | HIGH |
| kernel_seq | 4626292.6ns | 597266.8ns | 774.6% | HIGH |
| nway | 11971684.7ns | 1549493.8ns | 772.6% | HIGH |
| sequential | 4668490.7ns | 811564.1ns | 575.2% | HIGH |
| std_parallel | 7840855.6ns | 419899.4ns | 1867.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 522551.4-653472.2 ns)
  522551.4 |##########################
  529097.4 |##########################
  535643.5 |#############
  542189.5 |
  548735.5 |
  555281.6 |########################################
  561827.6 |#############
  568373.7 |
  574919.7 |
  581465.8 |
  588011.8 |
  594557.9 |
  601103.9 |
  607649.9 |
  614196.0 |
  620742.0 |
  627288.1 |#############
  633834.1 |#############
  640380.2 |##########################
  646926.2 |#############
  (1 below, 3 above range)

head_tail_dyn (n=18, range 504160.8-639019.6 ns)
  504160.8 |
  510903.7 |##########
  517646.7 |
  524389.6 |##########
  531132.6 |##########
  537875.5 |
  544618.4 |########################################
  551361.4 |####################
  558104.3 |
  564847.3 |##########
  571590.2 |##########
  578333.1 |
  585076.1 |##########
  591819.0 |
  598562.0 |##########
  605304.9 |
  612047.9 |
  618790.8 |
  625533.7 |
  632276.7 |
  (2 below, 3 above range)

kernel_nway (n=18, range 428417.8-879681.9 ns)
  428417.8 |####################
  450981.0 |####################
  473544.2 |
  496107.4 |####################
  518670.6 |####################
  541233.8 |
  563797.0 |
  586360.2 |
  608923.4 |####################
  631486.6 |####################
  654049.8 |####################
  676613.1 |########################################
  699176.3 |
  721739.5 |####################
  744302.7 |########################################
  766865.9 |
  789429.1 |
  811992.3 |####################
  834555.5 |
  857118.7 |####################
  (1 below, 3 above range)

kernel_seq (n=18, range 507620.0-675325.8 ns)
  507620.0 |##############################
  516005.3 |##########
  524390.6 |
  532775.8 |
  541161.1 |
  549546.4 |
  557931.7 |
  566317.0 |
  574702.3 |##########
  583087.6 |##########
  591472.9 |
  599858.2 |####################
  608243.5 |
  616628.8 |########################################
  625014.1 |##########
  633399.4 |##########
  641784.7 |
  650170.0 |##########
  658555.2 |
  666940.5 |
  (2 below, 1 above range)

nway (n=18, range 1078319.8-1841231.7 ns)
  1078319.8 |
  1116465.4 |#############
  1154611.0 |
  1192756.5 |#############
  1230902.1 |
  1269047.7 |
  1307193.3 |
  1345338.9 |
  1383484.5 |
  1421630.1 |
  1459775.7 |#############
  1497921.3 |########################################
  1536066.9 |##########################
  1574212.5 |#############
  1612358.1 |
  1650503.7 |
  1688649.3 |#############
  1726794.9 |#############
  1764940.5 |
  1803086.1 |#############
  (2 below, 4 above range)

sequential (n=18, range 806152.1-818476.3 ns)
  806152.1 |####################
  806768.3 |##########
  807384.5 |##########
  808000.7 |##########
  808616.9 |
  809233.1 |##########
  809849.4 |########################################
  810465.6 |####################
  811081.8 |
  811698.0 |
  812314.2 |
  812930.4 |
  813546.6 |
  814162.8 |
  814779.0 |
  815395.2 |
  816011.4 |##########
  816627.7 |
  817243.9 |
  817860.1 |
  (2 below, 3 above range)

std_parallel (n=18, range 294980.7-514662.8 ns)
  294980.7 |
  305964.8 |####################
  316948.9 |
  327933.0 |
  338917.1 |
  349901.2 |
  360885.3 |####################
  371869.4 |########################################
  382853.5 |####################
  393837.6 |########################################
  404821.7 |####################
  415805.8 |####################
  426789.9 |
  437774.0 |########################################
  448758.1 |
  459742.3 |
  470726.4 |
  481710.5 |########################################
  492694.6 |
  503678.7 |####################
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=809.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=821.1% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=25.2% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=1775.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=755.6% of algo (FFI overhead may distort results)
- **nway**: bridge=764.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=576.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=1963.9% of algo (FFI overhead may distort results)

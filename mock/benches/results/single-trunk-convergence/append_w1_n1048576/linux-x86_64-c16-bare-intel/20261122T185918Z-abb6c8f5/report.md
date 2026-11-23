# append, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential shows alternating (throttle bounce) (autocorr -0.67)

sequential's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn, kernel_seq, kernel_nway, head_tail} vs {sequential, std_parallel, nway} (42% apart)

The field splits into a fast tier {head_tail_dyn, kernel_seq, kernel_nway, head_tail} and a slow tier {sequential, std_parallel, nway} with a 42% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### nway is inconsistent: worst-20% is 1.6x its best-20%

nway's best 20% of batches run at 972.42 us but its worst 20% at 1.56 ms (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 536190.4 ns median (-7.4% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.86x (fastest 536190.4 ns, slowest 997184.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 1 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112772868 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 588310ns | 580340ns | 543254ns | 573704ns | 632747ns | base |
| head_tail_dyn | 542695ns | 537402ns | 506868ns | 535929ns | 570757ns | -7.75% |
| kernel_nway | 595291ns | 562537ns | 519694ns | 559123ns | 687340ns | +1.19% |
| kernel_seq | 580777ns | 555354ns | 519828ns | 557197ns | 646622ns | -1.28% |
| nway | 1180239ns | 998342ns | 973690ns | 994258ns | 1562483ns | +100.62% |
| sequential | 851596ns | 821496ns | 813328ns | 821631ns | 915677ns | +44.75% |
| std_parallel | 928694ns | 910299ns | 858905ns | 906309ns | 997167ns | +57.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 586868ns | 541763ns | 631250ns | base | 1.787 |
| head_tail_dyn | 541271ns | 505619ns | 569216ns | -7.77% | 1.937 |
| kernel_nway | 593978ns | 518552ns | 686058ns | +1.21% | 1.765 |
| kernel_seq | 579371ns | 518539ns | 645164ns | -1.28% | 1.810 |
| nway | 1179001ns | 972418ns | 1561176ns | +100.90% | 0.889 |
| sequential | 851089ns | 812912ns | 915109ns | +45.02% | 1.232 |
| std_parallel | 927087ns | 857615ns | 995480ns | +57.97% | 1.131 |

## Performance model

- Peak throughput: **2.074 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.811 | 87.3% |
| head_tail_dyn | 1.956 | 94.3% |
| kernel_nway | 1.868 | 90.1% |
| kernel_seq | 1.893 | 91.3% |
| nway | 1.052 | 50.7% |
| sequential | 1.277 | 61.6% |
| std_parallel | 1.154 | 55.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 588310ns | 588310ns | base |
| head_tail_dyn | 542695ns | 542695ns | -7.75% |
| kernel_nway | 595291ns | 595291ns | +1.19% |
| kernel_seq | 580777ns | 580777ns | -1.28% |
| nway | 1180239ns | 1180239ns | +100.62% |
| sequential | 851596ns | 851596ns | +44.75% |
| std_parallel | 928694ns | 928694ns | +57.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 579012ns | base | --- | [559939, 616826] | --- | --- | --- | --- |
| head_tail_dyn | 536190ns | -37311.3ns (-6.4%) | [-77425, -19294]ns | [529683, 552389] | YES | 0.0113 | 0.0075 | 0 |
| kernel_nway | 561186ns | no significant difference | [-60700, +41111]ns | [549168, 618442] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 553924ns | no significant difference | [-61014, +40961]ns | [542643, 627772] | no | 0.9774 | 0.8145 | 0 |
| nway | 997184ns | +431646.6ns (+74.5%) | [+391997, +505021]ns | [987681, 1120782] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 821031ns | +271944.8ns (+47.0%) | [+204515, +318610]ns | [818832, 899428] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 908589ns | +324683.7ns (+56.1%) | [+287381, +358143]ns | [899683, 916621] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 624219ns | -14.4% | -10.7% | -13.1% | +70.5% | +30.9% | +43.2% |
| 1 | 2 | 573957ns | -3.7% | +11.0% | +3.6% | +73.6% | +57.1% | +59.1% |
| 1 | 3 | 547256ns | -2.0% | -0.3% | +9.9% | +91.7% | +49.7% | +65.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.020 | ok |
| head_tail_dyn | -0.661 | HIGH- (thermal bounce) |
| kernel_nway | -0.657 | HIGH- (thermal bounce) |
| kernel_seq | -0.107 | ok |
| nway | -0.636 | HIGH- (thermal bounce) |
| sequential | -0.666 | HIGH- (thermal bounce) |
| std_parallel | -0.434 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 4566551.3ns | 586868.2ns | 778.1% | HIGH |
| head_tail_dyn | 4610114.0ns | 541270.9ns | 851.7% | HIGH |
| kernel_nway | 4634789.6ns | 593977.9ns | 780.3% | HIGH |
| kernel_seq | 4655355.7ns | 579371.5ns | 803.5% | HIGH |
| nway | 5129251.8ns | 1179001.5ns | 435.1% | HIGH |
| sequential | 4719332.5ns | 851089.4ns | 554.5% | HIGH |
| std_parallel | 5000393.7ns | 927087.3ns | 539.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 541762.7-631250.3 ns)
  541762.7 |
  546237.1 |########################################
  550711.5 |
  555185.9 |####################
  559660.2 |####################
  564134.6 |########################################
  568609.0 |
  573083.4 |
  577557.8 |########################################
  582032.1 |####################
  586506.5 |
  590980.9 |
  595455.3 |
  599929.6 |
  604404.0 |####################
  608878.4 |####################
  613352.8 |
  617827.2 |####################
  622301.5 |########################################
  626775.9 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 505619.1-569215.7 ns)
  505619.1 |
  508799.0 |
  511978.8 |
  515158.6 |#############
  518338.4 |
  521518.3 |
  524698.1 |##########################
  527877.9 |
  531057.7 |##########################
  534237.6 |########################################
  537417.4 |
  540597.2 |##########################
  543777.0 |
  546956.9 |
  550136.7 |#############
  553316.5 |#############
  556496.3 |
  559676.2 |
  562856.0 |##########################
  566035.8 |
  (2 below, 2 above range)

kernel_nway (n=18, range 518551.6-686057.5 ns)
  518551.6 |
  526926.9 |
  535302.2 |##########################
  543677.5 |##########################
  552052.8 |##########################
  560428.1 |########################################
  568803.4 |
  577178.7 |#############
  585554.0 |
  593929.3 |
  602304.6 |
  610679.9 |#############
  619055.2 |
  627430.5 |##########################
  635805.8 |#############
  644181.1 |#############
  652556.4 |
  660931.7 |
  669307.0 |
  677682.3 |
  (2 below, 1 above range)

kernel_seq (n=18, range 518538.8-645163.5 ns)
  518538.8 |
  524870.1 |#############
  531201.3 |#############
  537532.5 |#############
  543863.8 |##########################
  550195.0 |########################################
  556526.2 |
  562857.5 |
  569188.7 |
  575519.9 |#############
  581851.2 |
  588182.4 |
  594513.6 |#############
  600844.9 |
  607176.1 |
  613507.3 |
  619838.6 |
  626169.8 |##########################
  632501.0 |
  638832.3 |##########################
  (2 below, 2 above range)

nway (n=18, range 972418.4-1561176.1 ns)
  972418.4 |########################################
  1001856.3 |####
  1031294.2 |
  1060732.1 |
  1090169.9 |####
  1119607.8 |####
  1149045.7 |
  1178483.6 |
  1207921.5 |
  1237359.4 |
  1266797.3 |
  1296235.2 |
  1325673.0 |
  1355110.9 |
  1384548.8 |
  1413986.7 |
  1443424.6 |
  1472862.5 |
  1502300.4 |
  1531738.2 |
  (1 below, 4 above range)

sequential (n=18, range 812912.3-915109.1 ns)
  812912.3 |#################
  818022.1 |########################################
  823132.0 |
  828241.8 |#####
  833351.6 |
  838461.5 |
  843571.3 |
  848681.2 |
  853791.0 |
  858900.8 |
  864010.7 |
  869120.5 |
  874230.4 |
  879340.2 |
  884450.1 |
  889559.9 |
  894669.7 |#####
  899779.6 |#################
  904889.4 |
  909999.3 |
  (1 below, 2 above range)

std_parallel (n=18, range 857615.2-995480.4 ns)
  857615.2 |
  864508.5 |##########
  871401.7 |
  878295.0 |##########
  885188.2 |
  892081.5 |##########
  898974.8 |########################################
  905868.0 |##############################
  912761.3 |##############################
  919654.5 |##########
  926547.8 |##########
  933441.0 |
  940334.3 |
  947227.6 |##########
  954120.8 |
  961014.1 |
  967907.3 |
  974800.6 |
  981693.9 |
  988587.1 |
  (1 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=788.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=846.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=812.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=819.6% of algo (FFI overhead may distort results)
- **nway**: CV=27.6% (high variance, measurements may be unstable)
- **nway**: bridge=499.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=570.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=538.1% of algo (FFI overhead may distort results)

# append, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway is an outlier: 2.6x slower than the field

nway (1.49 ms) is 2.6x the fastest (575.47 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.64)

kernel_seq's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_seq, kernel_nway, head_tail_dyn, head_tail, std_parallel} vs {sequential, nway} (74% apart)

The field splits into a fast tier {kernel_seq, kernel_nway, head_tail_dyn, head_tail, std_parallel} and a slow tier {sequential, nway} with a 74% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### nway is inconsistent: worst-20% is 1.6x its best-20%

nway's best 20% of batches run at 1.13 ms but its worst 20% at 1.78 ms (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 575470.9 ns median (-6.4% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.59x (fastest 575470.9 ns, slowest 1489438.4 ns)

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
| Counter frequency | 2100350026 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 689274ns | 617778ns | 572306ns | 606793ns | 871479ns | base |
| head_tail_dyn | 636893ns | 606459ns | 559223ns | 598121ns | 733887ns | -7.60% |
| kernel_nway | 622214ns | 590512ns | 564175ns | 590944ns | 698138ns | -9.73% |
| kernel_seq | 583861ns | 577618ns | 538604ns | 570574ns | 626421ns | -15.29% |
| nway | 1481761ns | 1491240ns | 1127778ns | 1396377ns | 1786828ns | +114.97% |
| sequential | 1209568ns | 1179230ns | 1122053ns | 1167916ns | 1315803ns | +75.48% |
| std_parallel | 704484ns | 679320ns | 579350ns | 655574ns | 840415ns | +2.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 687102ns | 570046ns | 869181ns | base | 1.526 |
| head_tail_dyn | 634483ns | 557341ns | 731791ns | -7.66% | 1.653 |
| kernel_nway | 620174ns | 562373ns | 696030ns | -9.74% | 1.691 |
| kernel_seq | 581750ns | 536426ns | 624237ns | -15.33% | 1.802 |
| nway | 1479883ns | 1126080ns | 1784841ns | +115.38% | 0.709 |
| sequential | 1207859ns | 1120394ns | 1313973ns | +75.79% | 0.868 |
| std_parallel | 702370ns | 577365ns | 838171ns | +2.22% | 1.493 |

## Performance model

- Peak throughput: **1.955 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.705 | 87.2% |
| head_tail_dyn | 1.737 | 88.8% |
| kernel_nway | 1.781 | 91.1% |
| kernel_seq | 1.822 | 93.2% |
| nway | 0.704 | 36.0% |
| sequential | 0.890 | 45.6% |
| std_parallel | 1.548 | 79.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 689274ns | 689274ns | base |
| head_tail_dyn | 636893ns | 636893ns | -7.60% |
| kernel_nway | 622214ns | 622214ns | -9.73% |
| kernel_seq | 583861ns | 583861ns | -15.29% |
| nway | 1481761ns | 1481761ns | +114.97% |
| sequential | 1209568ns | 1209568ns | +75.48% |
| std_parallel | 704484ns | 704484ns | +2.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 615028ns | base | --- | [579676, 699671] | --- | --- | --- | --- |
| head_tail_dyn | 603770ns | no significant difference | [-49375, +13551]ns | [583000, 661450] | no | 0.5768 | 0.4807 | 0 |
| kernel_nway | 588698ns | no significant difference | [-46084, +5431]ns | [577536, 630120] | no | 0.3568 | 0.2379 | 0 |
| kernel_seq | 575471ns | -44933.7ns (-7.3%) | [-134722, -8170]ns | [556499, 586128] | YES (adj: no) | 0.1925 | 0.0963 | 0 |
| nway | 1489438ns | +781347.6ns (+127.0%) | [+549606, +994076]ns | [1256545, 1667813] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 1177621ns | +556962.1ns (+90.6%) | [+518275, +613167]ns | [1140854, 1243212] | YES | 0.0004 | 0.0001 | 0 |
| std_parallel | 677212ns | no significant difference | [-38870, +120159]ns | [617223, 724866] | no | 0.8145 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 679732ns | -2.7% | +0.8% | -15.1% | +120.9% | +81.1% | +9.8% |
| 1 | 2 | 595696ns | -3.7% | -3.8% | -4.9% | +141.6% | +100.9% | +17.7% |
| 1 | 3 | 637179ns | -4.4% | -5.1% | -9.8% | +139.8% | +81.3% | -8.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.496 | moderate- |
| head_tail_dyn | -0.436 | moderate- |
| kernel_nway | -0.342 | moderate- |
| kernel_seq | -0.642 | HIGH- (thermal bounce) |
| nway | -0.611 | HIGH- (thermal bounce) |
| sequential | -0.002 | ok |
| std_parallel | -0.040 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 6061737.4ns | 687101.9ns | 882.2% | HIGH |
| head_tail_dyn | 5966062.8ns | 634482.8ns | 940.3% | HIGH |
| kernel_nway | 6146938.5ns | 620174.1ns | 991.2% | HIGH |
| kernel_seq | 5794579.2ns | 581749.7ns | 996.1% | HIGH |
| nway | 6762925.8ns | 1479883.3ns | 457.0% | HIGH |
| sequential | 6478277.9ns | 1207859.2ns | 536.3% | HIGH |
| std_parallel | 6076263.0ns | 702370.1ns | 865.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 570046.0-869180.5 ns)
  570046.0 |########################################
  585002.7 |
  599959.5 |#############
  614916.2 |#############
  629872.9 |
  644829.6 |######
  659786.4 |######
  674743.1 |
  689699.8 |
  704656.5 |
  719613.3 |######
  734570.0 |######
  749526.7 |######
  764483.4 |
  779440.2 |
  794396.9 |######
  809353.6 |
  824310.3 |
  839267.1 |
  854223.8 |
  (1 below, 1 above range)

head_tail_dyn (n=18, range 557340.8-731791.4 ns)
  557340.8 |########################################
  566063.3 |
  574785.8 |########################################
  583508.4 |########################################
  592230.9 |####################
  600953.4 |
  609675.9 |####################
  618398.5 |########################################
  627121.0 |
  635843.5 |
  644566.1 |####################
  653288.6 |
  662011.1 |
  670733.7 |####################
  679456.2 |
  688178.7 |####################
  696901.2 |####################
  705623.8 |
  714346.3 |
  723068.8 |
  (2 below, 2 above range)

kernel_nway (n=18, range 562372.6-696030.4 ns)
  562372.6 |##########################
  569055.5 |#############
  575738.4 |########################################
  582421.2 |##########################
  589104.1 |#############
  595787.0 |
  602469.9 |
  609152.8 |#############
  615835.7 |##########################
  622518.6 |
  629201.5 |
  635884.4 |#############
  642567.3 |
  649250.2 |
  655933.1 |
  662616.0 |#############
  669298.8 |
  675981.7 |
  682664.6 |
  689347.5 |
  (1 below, 3 above range)

kernel_seq (n=18, range 536426.5-624237.4 ns)
  536426.5 |##########
  540817.0 |
  545207.6 |
  549598.1 |####################
  553988.7 |####################
  558379.2 |
  562769.8 |
  567160.3 |
  571550.9 |########################################
  575941.4 |####################
  580332.0 |##########
  584722.5 |##########
  589113.1 |
  593503.6 |####################
  597894.2 |
  602284.7 |
  606675.2 |
  611065.8 |##########
  615456.3 |
  619846.9 |
  (1 below, 1 above range)

nway (n=18, range 1126079.7-1784840.9 ns)
  1126079.7 |#############
  1159017.7 |
  1191955.8 |##########################
  1224893.9 |
  1257831.9 |#############
  1290770.0 |
  1323708.1 |#############
  1356646.1 |#############
  1389584.2 |
  1422522.2 |#############
  1455460.3 |
  1488398.4 |
  1521336.4 |#############
  1554274.5 |##########################
  1587212.6 |
  1620150.6 |
  1653088.7 |########################################
  1686026.8 |
  1718964.8 |##########################
  1751902.9 |
  (2 below, 1 above range)

sequential (n=18, range 1120393.8-1313973.0 ns)
  1120393.8 |####################
  1130072.7 |########################################
  1139751.7 |########################################
  1149430.6 |####################
  1159109.6 |
  1168788.6 |########################################
  1178467.5 |####################
  1188146.5 |####################
  1197825.5 |
  1207504.4 |####################
  1217183.4 |
  1226862.3 |
  1236541.3 |####################
  1246220.3 |####################
  1255899.2 |
  1265578.2 |####################
  1275257.1 |
  1284936.1 |
  1294615.1 |
  1304294.0 |
  (1 below, 3 above range)

std_parallel (n=18, range 577365.4-838171.0 ns)
  577365.4 |####################
  590405.6 |####################
  603445.9 |####################
  616486.2 |####################
  629526.5 |####################
  642566.8 |
  655607.1 |########################################
  668647.3 |
  681687.6 |####################
  694727.9 |########################################
  707768.2 |####################
  720808.5 |
  733848.7 |####################
  746889.0 |
  759929.3 |
  772969.6 |####################
  786009.9 |
  799050.1 |####################
  812090.4 |
  825130.7 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=31.2% (high variance, measurements may be unstable)
- **head_tail**: bridge=972.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=969.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1054.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=1004.5% of algo (FFI overhead may distort results)
- **nway**: bridge=454.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=542.2% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=894.3% of algo (FFI overhead may distort results)

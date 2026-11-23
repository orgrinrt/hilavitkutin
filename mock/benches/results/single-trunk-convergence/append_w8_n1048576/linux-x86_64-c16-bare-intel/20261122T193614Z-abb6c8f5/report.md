# append, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel dominates: 110% faster than the next best (head_tail)

std_parallel (259.54 us) leads head_tail (544.01 us) by 110%, a clear separation rather than a photo finish. CV 20.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel beats baseline by 53% (significant)

std_parallel is -290.30 us (53%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 3.2x slower than the field

sequential (818.81 us) is 3.2x the fastest (259.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.66)

head_tail's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {std_parallel} vs {head_tail, kernel_seq, nway, head_tail_dyn, kernel_nway, sequential} (110% apart)

The field splits into a fast tier {std_parallel} and a slow tier {head_tail, kernel_seq, nway, head_tail_dyn, kernel_nway, sequential} with a 110% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.2x the fastest

Fastest std_parallel (259.54 us) to slowest sequential (818.81 us): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### nway is inconsistent: worst-20% is 3.9x its best-20%

nway's best 20% of batches run at 383.89 us but its worst 20% at 1.49 ms (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_parallel** at 259541.8 ns median (-52.3% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 3.15x (fastest 259541.8 ns, slowest 818807.2 ns)

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
| Counter frequency | 2112862021 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 561701ns | 545101ns | 523055ns | 548378ns | 601009ns | base |
| head_tail_dyn | 580524ns | 585931ns | 530721ns | 575162ns | 613468ns | +3.35% |
| kernel_nway | 717599ns | 793657ns | 436084ns | 728016ns | 842730ns | +27.75% |
| kernel_seq | 570028ns | 572881ns | 522144ns | 560731ns | 607914ns | +1.48% |
| nway | 807543ns | 575233ns | 385092ns | 494024ns | 1489046ns | +43.77% |
| sequential | 818374ns | 819475ns | 807888ns | 815350ns | 828153ns | +45.70% |
| std_parallel | 281990ns | 260678ns | 232355ns | 255671ns | 346285ns | -49.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 560332ns | 521809ns | 599702ns | base | 1.871 |
| head_tail_dyn | 579228ns | 529399ns | 612213ns | +3.37% | 1.810 |
| kernel_nway | 716067ns | 434497ns | 841168ns | +27.79% | 1.464 |
| kernel_seq | 568678ns | 521011ns | 606476ns | +1.49% | 1.844 |
| nway | 806357ns | 383890ns | 1487916ns | +43.91% | 1.300 |
| sequential | 817845ns | 807488ns | 827621ns | +45.96% | 1.282 |
| std_parallel | 280656ns | 231325ns | 344842ns | -49.91% | 3.736 |

## Performance model

- Peak throughput: **4.533 Gops/s** (std_parallel; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.928 | 42.5% |
| head_tail_dyn | 1.794 | 39.6% |
| kernel_nway | 1.324 | 29.2% |
| kernel_seq | 1.834 | 40.5% |
| nway | 1.826 | 40.3% |
| sequential | 1.281 | 28.3% |
| std_parallel | 4.040 | 89.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 561701ns | 561701ns | base |
| head_tail_dyn | 580524ns | 580524ns | +3.35% |
| kernel_nway | 717599ns | 717599ns | +27.75% |
| kernel_seq | 570028ns | 570028ns | +1.48% |
| nway | 807543ns | 807543ns | +43.77% |
| sequential | 818374ns | 818374ns | +45.70% |
| std_parallel | 281990ns | 281990ns | -49.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 544007ns | base | --- | [534986, 584481] | --- | --- | --- | --- |
| head_tail_dyn | 584526ns | no significant difference | [-10180, +49665]ns | [558550, 604106] | no | 0.7210 | 0.4807 | 0 |
| kernel_nway | 792229ns | +194338.7ns (+35.7%) | [+102059, +257608]ns | [670873, 808419] | YES | 0.0151 | 0.0075 | 0 |
| kernel_seq | 571615ns | no significant difference | [-22941, +40733]ns | [543253, 597293] | no | 0.9774 | 0.8145 | 0 |
| nway | 574206ns | no significant difference | [-139936, +617254]ns | [392077, 1203515] | no | 1.0000 | 1.0000 | 0 |
| sequential | 818807ns | +263509.9ns (+48.4%) | [+237725, +278709]ns | [809239, 823730] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 259542ns | -290299.5ns (-53.4%) | [-339018, -240011]ns | [242735, 307375] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 542467ns | +11.3% | +14.0% | -0.1% | +90.3% | +48.9% | -46.7% |
| 1 | 2 | 597442ns | -0.9% | +34.3% | -4.3% | +55.3% | +38.6% | -58.8% |
| 1 | 3 | 534986ns | +2.3% | +41.1% | +13.8% | -27.0% | +53.1% | -49.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.659 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.069 | ok |
| kernel_nway | -0.326 | moderate- |
| kernel_seq | -0.003 | ok |
| nway | -0.088 | ok |
| sequential | -0.451 | moderate- |
| std_parallel | -0.528 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 4544804.5ns | 560331.6ns | 811.1% | HIGH |
| head_tail_dyn | 4578538.0ns | 579227.5ns | 790.5% | HIGH |
| kernel_nway | 12017080.7ns | 716066.5ns | 1678.2% | HIGH |
| kernel_seq | 4553200.0ns | 568677.6ns | 800.7% | HIGH |
| nway | 6879490.1ns | 806356.8ns | 853.2% | HIGH |
| sequential | 4677247.1ns | 817844.8ns | 571.9% | HIGH |
| std_parallel | 5496893.6ns | 280655.5ns | 1958.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 521809.0-599701.7 ns)
  521809.0 |
  525703.7 |
  529598.3 |#############
  533492.9 |########################################
  537387.6 |#############
  541282.2 |########################################
  545176.8 |
  549071.5 |
  552966.1 |
  556860.7 |
  560755.4 |
  564650.0 |
  568544.7 |
  572439.3 |##########################
  576333.9 |
  580228.6 |#############
  584123.2 |#############
  588017.8 |
  591912.5 |#############
  595807.1 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 529398.9-612213.4 ns)
  529398.9 |
  533539.6 |
  537680.4 |
  541821.1 |########################################
  545961.8 |########################################
  550102.5 |########################################
  554243.3 |
  558384.0 |
  562524.7 |########################################
  566665.4 |########################################
  570806.2 |
  574946.9 |########################################
  579087.6 |########################################
  583228.4 |########################################
  587369.1 |########################################
  591509.8 |########################################
  595650.5 |
  599791.3 |########################################
  603932.0 |########################################
  608072.7 |
  (2 below, 4 above range)

kernel_nway (n=18, range 434496.7-841168.2 ns)
  434496.7 |
  454830.3 |
  475163.8 |
  495497.4 |#############
  515831.0 |
  536164.6 |
  556498.2 |
  576831.7 |#############
  597165.3 |
  617498.9 |
  637832.4 |#############
  658166.0 |
  678499.6 |#############
  698833.2 |##########################
  719166.8 |
  739500.3 |
  759833.9 |
  780167.5 |########################################
  800501.1 |########################################
  820834.6 |##########################
  (2 below, 2 above range)

kernel_seq (n=18, range 521010.8-606476.4 ns)
  521010.8 |
  525284.1 |####################
  529557.4 |
  533830.7 |####################
  538103.9 |####################
  542377.2 |########################################
  546650.5 |####################
  550923.8 |####################
  555197.1 |
  559470.3 |
  563743.6 |
  568016.9 |########################################
  572290.2 |
  576563.5 |####################
  580836.7 |####################
  585110.0 |
  589383.3 |####################
  593656.6 |
  597929.9 |
  602203.1 |####################
  (1 below, 4 above range)

nway (n=18, range 383889.8-1487916.1 ns)
  383889.8 |########################################
  439091.1 |
  494292.4 |
  549493.7 |###########
  604695.1 |#####
  659896.4 |#####
  715097.7 |
  770299.0 |
  825500.3 |
  880701.6 |
  935903.0 |
  991104.3 |
  1046305.6 |
  1101506.9 |
  1156708.2 |#####
  1211909.5 |
  1267110.9 |
  1322312.2 |#####
  1377513.5 |#####
  1432714.8 |
  (1 below, 3 above range)

sequential (n=18, range 807487.7-827620.8 ns)
  807487.7 |########################################
  808494.4 |
  809501.0 |#############
  810507.7 |
  811514.3 |#############
  812521.0 |
  813527.6 |
  814534.3 |
  815540.9 |
  816547.6 |
  817554.2 |##########################
  818560.9 |##########################
  819567.5 |#############
  820574.2 |#############
  821580.8 |
  822587.5 |
  823594.1 |
  824600.8 |
  825607.4 |
  826614.1 |##########################
  (2 below, 3 above range)

std_parallel (n=18, range 231325.5-344842.4 ns)
  231325.5 |#############
  237001.3 |########################################
  242677.2 |##########################
  248353.0 |#############
  254028.9 |#############
  259704.7 |#############
  265380.5 |#############
  271056.4 |#############
  276732.2 |
  282408.1 |
  288083.9 |
  293759.8 |
  299435.6 |
  305111.5 |##########################
  310787.3 |#############
  316463.2 |
  322139.0 |
  327814.9 |
  333490.7 |
  339166.6 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=832.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=784.7% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=20.9% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=1558.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=797.4% of algo (FFI overhead may distort results)
- **nway**: CV=62.3% (high variance, measurements may be unstable)
- **nway**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **nway**: bridge=968.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=571.0% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=1879.2% of algo (FFI overhead may distort results)

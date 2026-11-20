# append, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway is an outlier: 4.9x slower than the field

nway (2.54 ms) is 4.9x the fastest (515.18 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.65)

sequential's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {std_parallel, head_tail, kernel_seq, head_tail_dyn, sequential, kernel_nway} vs {nway} (175% apart)

The field splits into a fast tier {std_parallel, head_tail, kernel_seq, head_tail_dyn, sequential, kernel_nway} and a slow tier {nway} with a 175% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.9x the fastest

Fastest std_parallel (515.18 us) to slowest nway (2.54 ms): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### nway is inconsistent: worst-20% is 2.9x its best-20%

nway's best 20% of batches run at 979.25 us but its worst 20% at 2.79 ms (2.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_parallel** at 515179.8 ns median (-6.6% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 4.93x (fastest 515179.8 ns, slowest 2541848.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 2 |
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
| head_tail | 611437ns | 552648ns | 535810ns | 549960ns | 741464ns | base |
| head_tail_dyn | 617805ns | 619352ns | 584725ns | 615961ns | 637110ns | +1.04% |
| kernel_nway | 976954ns | 927050ns | 535906ns | 811498ns | 1445663ns | +59.78% |
| kernel_seq | 589849ns | 590743ns | 532033ns | 582098ns | 630384ns | -3.53% |
| nway | 2267005ns | 2542966ns | 980552ns | 2345159ns | 2793002ns | +270.77% |
| sequential | 853108ns | 859350ns | 806374ns | 845748ns | 887514ns | +39.53% |
| std_parallel | 578677ns | 516591ns | 456062ns | 506828ns | 747758ns | -5.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 609998ns | 534532ns | 739795ns | base | 1.719 |
| head_tail_dyn | 616500ns | 583329ns | 635839ns | +1.07% | 1.701 |
| kernel_nway | 975467ns | 534566ns | 1443800ns | +59.91% | 1.075 |
| kernel_seq | 588419ns | 530982ns | 628667ns | -3.54% | 1.782 |
| nway | 2265775ns | 979253ns | 2791732ns | +271.44% | 0.463 |
| sequential | 852395ns | 805968ns | 886572ns | +39.74% | 1.230 |
| std_parallel | 577009ns | 454357ns | 746048ns | -5.41% | 1.817 |

## Performance model

- Peak throughput: **2.308 Gops/s** (std_parallel; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.902 | 82.4% |
| head_tail_dyn | 1.696 | 73.5% |
| kernel_nway | 1.133 | 49.1% |
| kernel_seq | 1.780 | 77.1% |
| nway | 0.413 | 17.9% |
| sequential | 1.221 | 52.9% |
| std_parallel | 2.035 | 88.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 611437ns | 611437ns | base |
| head_tail_dyn | 617805ns | 617805ns | +1.04% |
| kernel_nway | 976954ns | 976954ns | +59.78% |
| kernel_seq | 589849ns | 589849ns | -3.53% |
| nway | 2267005ns | 2267005ns | +270.77% |
| sequential | 853108ns | 853108ns | +39.53% |
| std_parallel | 578677ns | 578677ns | -5.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 551318ns | base | --- | [539207, 602901] | --- | --- | --- | --- |
| head_tail_dyn | 618118ns | no significant difference | [-10512, +84878]ns | [611040, 626996] | no | 0.1444 | 0.0963 | 0 |
| kernel_nway | 925660ns | +282561.3ns (+51.3%) | [+73662, +391568]ns | [684192, 1112731] | YES | 0.0151 | 0.0075 | 0 |
| kernel_seq | 589135ns | no significant difference | [-13454, +53436]ns | [566470, 610578] | no | 0.2855 | 0.2379 | 0 |
| nway | 2541849ns | +2000012.9ns (+362.8%) | [+1335896, +2026436]ns | [2172146, 2572024] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 858653ns | +282076.0ns (+51.2%) | [+239760, +316675]ns | [824301, 868679] | YES | 0.0039 | 0.0013 | 0 |
| std_parallel | 515180ns | no significant difference | [-79386, +13744]ns | [475808, 618908] | no | 0.8145 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 564865ns | +6.6% | +63.9% | +9.2% | +222.6% | +53.8% | -1.2% |
| 1 | 2 | 547135ns | +13.6% | +25.1% | +7.9% | +365.2% | +47.5% | -0.5% |
| 1 | 3 | 540081ns | +17.1% | +123.6% | +0.9% | +374.2% | +59.0% | -11.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.039 | ok |
| head_tail_dyn | -0.020 | ok |
| kernel_nway | -0.474 | moderate- |
| kernel_seq | -0.014 | ok |
| nway | -0.156 | ok |
| sequential | -0.651 | HIGH- (thermal bounce) |
| std_parallel | -0.086 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 4739939.8ns | 609998.5ns | 777.0% | HIGH |
| head_tail_dyn | 4594402.0ns | 616500.4ns | 745.2% | HIGH |
| kernel_nway | 8185800.3ns | 975467.0ns | 839.2% | HIGH |
| kernel_seq | 4635043.9ns | 588419.2ns | 787.7% | HIGH |
| nway | 8889730.9ns | 2265775.4ns | 392.3% | HIGH |
| sequential | 4719510.8ns | 852395.4ns | 553.7% | HIGH |
| std_parallel | 5390522.8ns | 577008.7ns | 934.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 534532.3-739795.4 ns)
  534532.3 |########################################
  544795.5 |########################################
  555058.6 |##########
  565321.8 |####################
  575585.0 |
  585848.1 |
  596111.3 |
  606374.4 |
  616637.6 |
  626900.7 |##########
  637163.9 |
  647427.0 |##########
  657690.2 |
  667953.3 |
  678216.5 |
  688479.6 |
  698742.8 |
  709006.0 |
  719269.1 |
  729532.3 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 583329.3-635838.6 ns)
  583329.3 |#############
  585954.7 |
  588580.2 |
  591205.7 |#############
  593831.1 |
  596456.6 |
  599082.1 |#############
  601707.5 |
  604333.0 |
  606958.5 |
  609583.9 |##########################
  612209.4 |##########################
  614834.9 |#############
  617460.3 |
  620085.8 |#############
  622711.3 |#############
  625336.7 |########################################
  627962.2 |#############
  630587.7 |
  633213.1 |#############
  (1 below, 2 above range)

kernel_nway (n=18, range 534566.1-1443799.9 ns)
  534566.1 |#############
  580027.8 |########################################
  625489.4 |
  670951.1 |
  716412.8 |#############
  761874.5 |##########################
  807336.2 |
  852797.9 |
  898259.6 |##########################
  943721.3 |##########################
  989183.0 |
  1034644.7 |
  1080106.4 |##########################
  1125568.0 |#############
  1171029.7 |
  1216491.4 |#############
  1261953.1 |
  1307414.8 |
  1352876.5 |
  1398338.2 |
  (1 below, 2 above range)

kernel_seq (n=18, range 530982.5-628666.8 ns)
  530982.5 |
  535866.7 |####################
  540750.9 |####################
  545635.1 |####################
  550519.3 |
  555403.6 |
  560287.8 |
  565172.0 |
  570056.2 |
  574940.4 |
  579824.7 |########################################
  584708.9 |########################################
  589593.1 |########################################
  594477.3 |
  599361.5 |
  604245.7 |########################################
  609130.0 |####################
  614014.2 |
  618898.4 |####################
  623782.6 |
  (2 below, 3 above range)

nway (n=18, range 979253.2-2791731.5 ns)
  979253.2 |
  1069877.1 |
  1160501.1 |
  1251125.0 |
  1341748.9 |
  1432372.8 |#####
  1522996.7 |
  1613620.6 |
  1704244.5 |
  1794868.5 |
  1885492.4 |
  1976116.3 |
  2066740.2 |
  2157364.1 |############################
  2247988.0 |
  2338611.9 |
  2429235.8 |
  2519859.8 |########################################
  2610483.7 |
  2701107.6 |
  (2 below, 3 above range)

sequential (n=18, range 805968.4-886572.1 ns)
  805968.4 |########################################
  809998.6 |
  814028.8 |
  818059.0 |
  822089.2 |
  826119.3 |
  830149.5 |
  834179.7 |
  838209.9 |##########
  842240.1 |
  846270.3 |
  850300.5 |####################
  854330.6 |##########
  858360.8 |####################
  862391.0 |####################
  866421.2 |
  870451.4 |##########
  874481.6 |####################
  878511.8 |
  882542.0 |
  (1 below, 2 above range)

std_parallel (n=18, range 454357.3-746048.4 ns)
  454357.3 |##########
  468941.9 |########################################
  483526.4 |####################
  498111.0 |
  512695.6 |
  527280.1 |
  541864.7 |####################
  556449.2 |
  571033.8 |##########
  585618.3 |
  600202.9 |##########
  614787.4 |
  629372.0 |##########
  643956.5 |
  658541.1 |
  673125.7 |
  687710.2 |
  702294.8 |
  716879.3 |
  731463.9 |####################
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=825.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=742.8% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=43.7% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=859.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=777.7% of algo (FFI overhead may distort results)
- **nway**: CV=28.8% (high variance, measurements may be unstable)
- **nway**: bridge=378.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=550.5% of algo (FFI overhead may distort results)
- **std_parallel**: CV=24.0% (high variance, measurements may be unstable)
- **std_parallel**: bridge=994.9% of algo (FFI overhead may distort results)

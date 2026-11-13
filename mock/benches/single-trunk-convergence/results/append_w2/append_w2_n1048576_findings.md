# append, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 16% faster than the next best (std_parallel)

kernel_nway (337.84 us) leads std_parallel (392.06 us) by 16%, a clear separation rather than a photo finish. CV 15.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_nway beats baseline by 41% (significant)

kernel_nway is -239.91 us (41%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 3.6x slower than the field

sequential (1.21 ms) is 3.6x the fastest (337.84 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.67)

kernel_seq's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.6x the fastest

Fastest kernel_nway (337.84 us) to slowest sequential (1.21 ms): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_nway** at 337839.0 ns median (-42.5% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 3.57x (fastest 337839.0 ns, slowest 1206408.3 ns)

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
| Counter frequency | 2100342298 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 630829ns | 589759ns | 550338ns | 577472ns | 751111ns | base |
| head_tail_dyn | 615927ns | 596465ns | 538330ns | 592974ns | 689156ns | -2.36% |
| kernel_nway | 350709ns | 339705ns | 306407ns | 332388ns | 400342ns | -44.41% |
| kernel_seq | 592477ns | 579829ns | 543920ns | 577469ns | 639268ns | -6.08% |
| nway | 743327ns | 732029ns | 590445ns | 700101ns | 884608ns | +17.83% |
| sequential | 1265109ns | 1207964ns | 1130130ns | 1195006ns | 1437753ns | +100.55% |
| std_parallel | 417825ns | 394994ns | 360156ns | 390074ns | 488287ns | -33.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 628640ns | 548512ns | 748880ns | base | 1.668 |
| head_tail_dyn | 613795ns | 536450ns | 686841ns | -2.36% | 1.708 |
| kernel_nway | 348814ns | 304835ns | 398288ns | -44.51% | 3.006 |
| kernel_seq | 590470ns | 541955ns | 637092ns | -6.07% | 1.776 |
| nway | 741602ns | 588862ns | 882771ns | +17.97% | 1.414 |
| sequential | 1263352ns | 1128523ns | 1435881ns | +100.97% | 0.830 |
| std_parallel | 414783ns | 357994ns | 483702ns | -34.02% | 2.528 |

## Performance model

- Peak throughput: **3.440 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.785 | 51.9% |
| head_tail_dyn | 1.764 | 51.3% |
| kernel_nway | 3.104 | 90.2% |
| kernel_seq | 1.813 | 52.7% |
| nway | 1.436 | 41.7% |
| sequential | 0.869 | 25.3% |
| std_parallel | 2.675 | 77.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 630829ns | 630829ns | base |
| head_tail_dyn | 615927ns | 615927ns | -2.36% |
| kernel_nway | 350709ns | 350709ns | -44.41% |
| kernel_seq | 592477ns | 592477ns | -6.08% |
| nway | 743327ns | 743327ns | +17.83% |
| sequential | 1265109ns | 1265109ns | +100.55% |
| std_parallel | 417825ns | 417825ns | -33.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 587279ns | base | --- | [557574, 668590] | --- | --- | --- | --- |
| head_tail_dyn | 594547ns | no significant difference | [-46118, +60421]ns | [556840, 672210] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 337839ns | -239913.6ns (-40.9%) | [-325333, -221303]ns | [319582, 353152] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 578287ns | no significant difference | [-91060, +24403]ns | [560487, 614221] | no | 0.8145 | 0.8145 | 0 |
| nway | 730238ns | +127560.2ns (+21.7%) | [+51482, +166048]ns | [654749, 784634] | YES | 0.0020 | 0.0013 | 0 |
| sequential | 1206408ns | +593001.2ns (+101.0%) | [+537320, +702552]ns | [1154463, 1335932] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 392056ns | -200528.5ns (-34.1%) | [-266792, -172598]ns | [374989, 421904] | YES | 0.0003 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 599734ns | +12.1% | -42.8% | -4.1% | +30.6% | +104.8% | -33.0% |
| 1 | 2 | 579502ns | -4.9% | -44.4% | +6.0% | +10.9% | +123.2% | -27.1% |
| 1 | 3 | 557272ns | +2.8% | -38.7% | +3.2% | +21.2% | +104.9% | -32.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.000 | ok |
| head_tail_dyn | -0.272 | moderate- |
| kernel_nway | -0.664 | HIGH- (thermal bounce) |
| kernel_seq | -0.667 | HIGH- (thermal bounce) |
| nway | -0.309 | moderate- |
| sequential | -0.453 | moderate- |
| std_parallel | -0.473 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 5996522.2ns | 628640.4ns | 953.9% | HIGH |
| head_tail_dyn | 6010626.7ns | 613795.0ns | 979.3% | HIGH |
| kernel_nway | 5535124.0ns | 348813.8ns | 1586.8% | HIGH |
| kernel_seq | 5969329.1ns | 590470.0ns | 1010.9% | HIGH |
| nway | 6033998.5ns | 741602.2ns | 813.6% | HIGH |
| sequential | 6709628.5ns | 1263352.4ns | 531.1% | HIGH |
| std_parallel | 5901005.4ns | 414783.2ns | 1422.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 548512.3-748879.7 ns)
  548512.3 |########################################
  558530.6 |########
  568549.0 |########
  578567.4 |################
  588585.7 |
  598604.1 |################
  608622.5 |
  618640.9 |
  628659.2 |
  638677.6 |
  648696.0 |
  658714.3 |########
  668732.7 |########
  678751.1 |
  688769.5 |########
  698787.8 |
  708806.2 |########
  718824.6 |
  728842.9 |
  738861.3 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 536449.9-686841.2 ns)
  536449.9 |
  543969.5 |########################################
  551489.0 |
  559008.6 |####################
  566528.2 |##########
  574047.7 |
  581567.3 |##########
  589086.8 |
  596606.4 |
  604126.0 |##########
  611645.5 |
  619165.1 |
  626684.7 |
  634204.2 |
  641723.8 |
  649243.3 |
  656762.9 |##########
  664282.5 |####################
  671802.0 |####################
  679321.6 |##########
  (1 below, 2 above range)

kernel_nway (n=18, range 304834.5-398287.7 ns)
  304834.5 |####################
  309507.2 |
  314179.8 |####################
  318852.5 |########################################
  323525.2 |########################################
  328197.8 |
  332870.5 |####################
  337543.1 |####################
  342215.8 |####################
  346888.4 |####################
  351561.1 |########################################
  356233.8 |
  360906.4 |####################
  365579.1 |
  370251.7 |
  374924.4 |
  379597.0 |
  384269.7 |
  388942.4 |########################################
  393615.0 |
  (2 below, 1 above range)

kernel_seq (n=18, range 541955.2-637092.2 ns)
  541955.2 |
  546712.1 |####################
  551468.9 |########################################
  556225.8 |
  560982.6 |####################
  565739.5 |####################
  570496.3 |####################
  575253.2 |########################################
  580010.0 |
  584766.9 |
  589523.7 |
  594280.6 |
  599037.4 |####################
  603794.3 |####################
  608551.1 |####################
  613308.0 |########################################
  618064.8 |########################################
  622821.7 |
  627578.5 |
  632335.4 |
  (2 below, 1 above range)

nway (n=18, range 588862.3-882771.3 ns)
  588862.3 |
  603557.7 |#############
  618253.2 |#############
  632948.6 |
  647644.1 |##########################
  662339.5 |#############
  677035.0 |#############
  691730.4 |
  706425.9 |
  721121.3 |#############
  735816.8 |##########################
  750512.2 |
  765207.7 |
  779903.1 |########################################
  794598.6 |
  809294.1 |
  823989.5 |
  838685.0 |
  853380.4 |#############
  868075.9 |
  (2 below, 3 above range)

sequential (n=18, range 1128523.3-1435881.0 ns)
  1128523.3 |########################################
  1143891.2 |#############
  1159259.0 |#############
  1174626.9 |#############
  1189994.8 |##########################
  1205362.7 |#############
  1220730.6 |
  1236098.5 |#############
  1251466.4 |#############
  1266834.3 |
  1282202.1 |
  1297570.0 |
  1312937.9 |
  1328305.8 |#############
  1343673.7 |#############
  1359041.6 |
  1374409.5 |#############
  1389777.4 |#############
  1405145.3 |
  1420513.1 |
  (1 below, 2 above range)

std_parallel (n=18, range 357993.6-483702.0 ns)
  357993.6 |########################################
  364279.0 |########################################
  370564.4 |
  376849.9 |####################
  383135.3 |########################################
  389420.7 |########################################
  395706.1 |####################
  401991.6 |
  408277.0 |########################################
  414562.4 |
  420847.8 |
  427133.2 |########################################
  433418.7 |
  439704.1 |
  445989.5 |
  452274.9 |
  458560.4 |
  464845.8 |
  471131.2 |
  477416.6 |####################
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=1003.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=987.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1626.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=1018.6% of algo (FFI overhead may distort results)
- **nway**: bridge=817.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=551.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=1476.5% of algo (FFI overhead may distort results)

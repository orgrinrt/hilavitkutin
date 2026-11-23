# append, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel dominates: 82% faster than the next best (head_tail)

std_parallel (301.99 us) leads head_tail (548.60 us) by 82%, a clear separation rather than a photo finish. CV 22.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel beats baseline by 45% (significant)

std_parallel is -245.54 us (45%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### nway is an outlier: 7.4x slower than the field

nway (2.24 ms) is 7.4x the fastest (301.99 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.67)

kernel_nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 7.4x the fastest

Fastest std_parallel (301.99 us) to slowest nway (2.24 ms): 7.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 5.4x its best-20%

kernel_nway's best 20% of batches run at 239.19 us but its worst 20% at 1.28 ms (5.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_parallel** at 301988.4 ns median (-45.0% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 7.43x (fastest 301988.4 ns, slowest 2244555.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 4 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112846685 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 549044ns | 549877ns | 512480ns | 545761ns | 572251ns | base |
| head_tail_dyn | 555222ns | 556619ns | 502725ns | 545428ns | 596162ns | +1.13% |
| kernel_nway | 939194ns | 1061808ns | 240284ns | 940774ns | 1286279ns | +71.06% |
| kernel_seq | 571538ns | 562681ns | 506258ns | 547803ns | 639780ns | +4.10% |
| nway | 2127594ns | 2246366ns | 1024181ns | 2193836ns | 2579937ns | +287.51% |
| sequential | 816515ns | 816012ns | 813594ns | 815714ns | 819176ns | +48.72% |
| std_parallel | 315666ns | 303408ns | 270989ns | 296001ns | 367502ns | -42.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 547543ns | 510978ns | 570773ns | base | 1.915 |
| head_tail_dyn | 553893ns | 501000ns | 595036ns | +1.16% | 1.893 |
| kernel_nway | 937873ns | 239189ns | 1284858ns | +71.29% | 1.118 |
| kernel_seq | 570096ns | 504796ns | 638305ns | +4.12% | 1.839 |
| nway | 2125962ns | 1022101ns | 2578377ns | +288.27% | 0.493 |
| sequential | 816082ns | 813182ns | 818716ns | +49.04% | 1.285 |
| std_parallel | 314212ns | 269284ns | 365995ns | -42.61% | 3.337 |

## Performance model

- Peak throughput: **4.384 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.911 | 43.6% |
| head_tail_dyn | 1.888 | 43.1% |
| kernel_nway | 0.989 | 22.6% |
| kernel_seq | 1.868 | 42.6% |
| nway | 0.467 | 10.7% |
| sequential | 1.286 | 29.3% |
| std_parallel | 3.472 | 79.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 549044ns | 549044ns | base |
| head_tail_dyn | 555222ns | 555222ns | +1.13% |
| kernel_nway | 939194ns | 939194ns | +71.06% |
| kernel_seq | 571538ns | 571538ns | +4.10% |
| nway | 2127594ns | 2127594ns | +287.51% |
| sequential | 816515ns | 816515ns | +48.72% |
| std_parallel | 315666ns | 315666ns | -42.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 548599ns | base | --- | [535997, 553709] | --- | --- | --- | --- |
| head_tail_dyn | 555470ns | no significant difference | [-14697, +30532]ns | [523114, 585232] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 1060592ns | +507489.3ns (+92.5%) | [+217262, +656204]ns | [775053, 1197387] | YES | 0.0113 | 0.0075 | 0 |
| kernel_seq | 561422ns | no significant difference | [-35737, +67803]ns | [520820, 624472] | no | 0.5768 | 0.4807 | 0 |
| nway | 2244556ns | +1700788.6ns (+310.0%) | [+1576019, +1898039]ns | [2147410, 2461552] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 815590ns | +265839.0ns (+48.5%) | [+262585, +280047]ns | [815053, 816405] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 301988ns | -245544.5ns (-44.8%) | [-260806, -232192]ns | [287804, 313346] | YES | 0.0003 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 560832ns | +6.6% | +98.6% | -8.9% | +260.2% | +45.4% | -45.5% |
| 1 | 2 | 545538ns | -5.6% | -24.6% | +2.9% | +319.9% | +49.1% | -42.6% |
| 1 | 3 | 541069ns | +2.7% | +110.1% | +16.8% | +323.2% | +50.9% | -47.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.061 | ok |
| head_tail_dyn | -0.491 | moderate- |
| kernel_nway | -0.666 | HIGH- (thermal bounce) |
| kernel_seq | -0.006 | ok |
| nway | -0.168 | ok |
| sequential | -0.580 | HIGH- (thermal bounce) |
| std_parallel | -0.329 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 4534096.7ns | 547543.1ns | 828.1% | HIGH |
| head_tail_dyn | 4539719.7ns | 553893.3ns | 819.6% | HIGH |
| kernel_nway | 10908116.9ns | 937873.0ns | 1163.1% | HIGH |
| kernel_seq | 4552839.6ns | 570096.1ns | 798.6% | HIGH |
| nway | 10054155.9ns | 2125962.4ns | 472.9% | HIGH |
| sequential | 4672027.7ns | 816081.8ns | 572.5% | HIGH |
| std_parallel | 4517217.3ns | 314211.7ns | 1437.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 510978.3-570773.0 ns)
  510978.3 |
  513968.0 |#############
  516957.7 |
  519947.5 |
  522937.2 |
  525927.0 |
  528916.7 |
  531906.4 |#############
  534896.2 |##########################
  537885.9 |
  540875.7 |
  543865.4 |##########################
  546855.1 |#############
  549844.9 |##########################
  552834.6 |########################################
  555824.3 |
  558814.1 |
  561803.8 |
  564793.6 |
  567783.3 |#############
  (2 below, 3 above range)

head_tail_dyn (n=18, range 501000.4-595036.1 ns)
  501000.4 |
  505702.2 |
  510404.0 |####################
  515105.8 |########################################
  519807.5 |
  524509.3 |####################
  529211.1 |####################
  533912.9 |
  538614.7 |
  543316.5 |
  548018.3 |########################################
  552720.0 |
  557421.8 |####################
  562123.6 |
  566825.4 |
  571527.2 |########################################
  576229.0 |
  580930.8 |####################
  585632.5 |####################
  590334.3 |
  (2 below, 4 above range)

kernel_nway (n=18, range 239189.3-1284857.6 ns)
  239189.3 |##########
  291472.7 |
  343756.2 |
  396039.6 |
  448323.0 |
  500606.4 |
  552889.8 |##########
  605173.2 |##########
  657456.6 |
  709740.0 |
  762023.5 |
  814306.9 |
  866590.3 |##########
  918873.7 |##########
  971157.1 |
  1023440.5 |########################################
  1075723.9 |
  1128007.3 |##########
  1180290.8 |####################
  1232574.2 |
  (2 below, 4 above range)

kernel_seq (n=18, range 504795.9-638305.4 ns)
  504795.9 |####################
  511471.4 |####################
  518146.9 |########################################
  524822.4 |####################
  531497.8 |
  538173.3 |
  544848.8 |
  551524.2 |####################
  558199.7 |########################################
  564875.2 |####################
  571550.7 |
  578226.1 |####################
  584901.6 |
  591577.1 |
  598252.6 |
  604928.0 |
  611603.5 |
  618279.0 |####################
  624954.5 |########################################
  631629.9 |####################
  (2 below, 2 above range)

nway (n=18, range 1022100.8-2578377.3 ns)
  1022100.8 |
  1099914.7 |
  1177728.5 |################
  1255542.3 |
  1333356.1 |
  1411169.9 |
  1488983.8 |
  1566797.6 |
  1644611.4 |
  1722425.2 |########
  1800239.0 |
  1878052.9 |
  1955866.7 |
  2033680.5 |########
  2111494.3 |
  2189308.2 |########################################
  2267122.0 |########
  2344935.8 |################
  2422749.6 |
  2500563.4 |################
  (1 below, 3 above range)

sequential (n=18, range 813181.6-818716.4 ns)
  813181.6 |####################
  813458.3 |
  813735.1 |##########
  814011.8 |
  814288.6 |
  814565.3 |
  814842.0 |##########
  815118.8 |####################
  815395.5 |########################################
  815672.2 |
  815949.0 |##########
  816225.7 |##########
  816502.5 |##########
  816779.2 |
  817055.9 |####################
  817332.7 |
  817609.4 |
  817886.2 |
  818162.9 |
  818439.6 |
  (1 below, 2 above range)

std_parallel (n=18, range 269284.4-365994.9 ns)
  269284.4 |####################
  274120.0 |########################################
  278955.5 |
  283791.0 |########################################
  288626.5 |########################################
  293462.0 |
  298297.6 |####################
  303133.1 |########################################
  307968.6 |########################################
  312804.1 |########################################
  317639.7 |
  322475.2 |
  327310.7 |
  332146.2 |####################
  336981.7 |####################
  341817.3 |
  346652.8 |
  351488.3 |
  356323.8 |
  361159.3 |
  (1 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=824.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=819.8% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=40.0% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 5.4x (possible bimodal distribution)
- **kernel_nway**: bridge=1140.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=811.0% of algo (FFI overhead may distort results)
- **nway**: CV=25.8% (high variance, measurements may be unstable)
- **nway**: bridge=469.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=572.8% of algo (FFI overhead may distort results)
- **std_parallel**: CV=21.5% (high variance, measurements may be unstable)
- **std_parallel**: bridge=1439.7% of algo (FFI overhead may distort results)

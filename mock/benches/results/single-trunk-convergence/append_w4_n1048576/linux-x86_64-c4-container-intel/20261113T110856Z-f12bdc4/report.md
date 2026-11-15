# append, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel beats baseline by 40% (significant)

std_parallel is -236.80 us (40%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 3.1x slower than the field

sequential (1.19 ms) is 3.1x the fastest (389.73 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.66)

nway's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.1x the fastest

Fastest kernel_nway (389.73 us) to slowest sequential (1.19 ms): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### std_parallel is inconsistent: worst-20% is 1.8x its best-20%

std_parallel's best 20% of batches run at 311.46 us but its worst 20% at 576.08 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 389733.0 ns median (-34.6% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 3.05x (fastest 389733.0 ns, slowest 1188855.6 ns)

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
| Counter frequency | 2100312120 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 633799ns | 597902ns | 536384ns | 593546ns | 742885ns | base |
| head_tail_dyn | 696905ns | 613892ns | 556300ns | 616515ns | 887792ns | +9.96% |
| kernel_nway | 397864ns | 391597ns | 334720ns | 383922ns | 450348ns | -37.23% |
| kernel_seq | 634796ns | 604246ns | 544577ns | 595156ns | 739364ns | +0.16% |
| nway | 828056ns | 826478ns | 602583ns | 779632ns | 1013428ns | +30.65% |
| sequential | 1205622ns | 1190442ns | 1118653ns | 1178932ns | 1289142ns | +90.22% |
| std_parallel | 437994ns | 409766ns | 314145ns | 386026ns | 577870ns | -30.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 631726ns | 534202ns | 740682ns | base | 1.660 |
| head_tail_dyn | 694881ns | 554265ns | 885632ns | +10.00% | 1.509 |
| kernel_nway | 395787ns | 333063ns | 448022ns | -37.35% | 2.649 |
| kernel_seq | 632663ns | 542166ns | 737390ns | +0.15% | 1.657 |
| nway | 826221ns | 600930ns | 1011596ns | +30.79% | 1.269 |
| sequential | 1203940ns | 1116718ns | 1287498ns | +90.58% | 0.871 |
| std_parallel | 436031ns | 311461ns | 576077ns | -30.98% | 2.405 |

## Performance model

- Peak throughput: **3.367 Gops/s** (std_parallel; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.759 | 52.3% |
| head_tail_dyn | 1.713 | 50.9% |
| kernel_nway | 2.690 | 79.9% |
| kernel_seq | 1.740 | 51.7% |
| nway | 1.271 | 37.8% |
| sequential | 0.882 | 26.2% |
| std_parallel | 2.570 | 76.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 633799ns | 633799ns | base |
| head_tail_dyn | 696905ns | 696905ns | +9.96% |
| kernel_nway | 397864ns | 397864ns | -37.23% |
| kernel_seq | 634796ns | 634796ns | +0.16% |
| nway | 828056ns | 828056ns | +30.65% |
| sequential | 1205622ns | 1205622ns | +90.22% |
| std_parallel | 437994ns | 437994ns | -30.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 596061ns | base | --- | [566385, 694108] | --- | --- | --- | --- |
| head_tail_dyn | 612303ns | no significant difference | [-79140, +195064]ns | [569714, 828211] | no | 0.9774 | 0.8145 | 0 |
| kernel_nway | 389733ns | -234384.5ns (-39.3%) | [-272897, -173105]ns | [368850, 406855] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 602499ns | no significant difference | [-94221, +56575]ns | [577181, 635759] | no | 1.0000 | 1.0000 | 0 |
| nway | 824734ns | +194861.0ns (+32.7%) | [+115799, +298914]ns | [713786, 943091] | YES | 0.0003 | 0.0001 | 0 |
| sequential | 1188856ns | +557867.6ns (+93.6%) | [+510103, +632307]ns | [1156937, 1245174] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 407987ns | -236803.6ns (-39.7%) | [-263766, -146000]ns | [347748, 420223] | YES | 0.0113 | 0.0075 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 581774ns | +47.0% | -29.9% | -3.1% | +58.1% | +108.6% | -29.2% |
| 1 | 2 | 566986ns | +8.0% | -31.4% | +11.2% | +9.7% | +110.6% | -35.5% |
| 1 | 3 | 705519ns | -19.5% | -48.1% | -15.5% | +27.3% | +66.8% | -41.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.226 | moderate- |
| head_tail_dyn | -0.091 | ok |
| kernel_nway | -0.002 | ok |
| kernel_seq | -0.505 | HIGH- (thermal bounce) |
| nway | -0.664 | HIGH- (thermal bounce) |
| sequential | -0.001 | ok |
| std_parallel | -0.664 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 6146443.6ns | 631725.6ns | 973.0% | HIGH |
| head_tail_dyn | 6121323.9ns | 694881.1ns | 880.9% | HIGH |
| kernel_nway | 5794733.4ns | 395786.7ns | 1464.1% | HIGH |
| kernel_seq | 6102575.5ns | 632662.9ns | 964.6% | HIGH |
| nway | 6298389.4ns | 826220.7ns | 762.3% | HIGH |
| sequential | 6482043.6ns | 1203940.2ns | 538.4% | HIGH |
| std_parallel | 5869082.8ns | 436031.0ns | 1346.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 534202.3-740681.8 ns)
  534202.3 |########################################
  544526.3 |
  554850.2 |########################################
  565174.2 |####################
  575498.2 |####################
  585822.2 |########################################
  596146.1 |####################
  606470.1 |
  616794.1 |
  627118.1 |####################
  637442.0 |####################
  647766.0 |
  658090.0 |
  668414.0 |
  678737.9 |####################
  689061.9 |
  699385.9 |####################
  709709.9 |
  720033.8 |########################################
  730357.8 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 554264.6-885631.6 ns)
  554264.6 |########################################
  570832.9 |##########
  587401.3 |####################
  603969.6 |
  620538.0 |##########
  637106.3 |
  653674.7 |
  670243.0 |##########
  686811.4 |
  703379.7 |
  719948.1 |
  736516.4 |
  753084.8 |##########
  769653.1 |
  786221.5 |
  802789.8 |##########
  819358.2 |
  835926.5 |##########
  852494.9 |##########
  869063.2 |
  (2 below, 3 above range)

kernel_nway (n=18, range 333063.4-448021.9 ns)
  333063.4 |
  338811.3 |##########################
  344559.2 |
  350307.1 |
  356055.1 |
  361803.0 |#############
  367550.9 |########################################
  373298.8 |
  379046.8 |
  384794.7 |########################################
  390542.6 |#############
  396290.5 |#############
  402038.5 |#############
  407786.4 |#############
  413534.3 |##########################
  419282.2 |
  425030.2 |#############
  430778.1 |
  436526.0 |
  442273.9 |
  (1 below, 1 above range)

kernel_seq (n=18, range 542165.9-737390.0 ns)
  542165.9 |########################################
  551927.1 |
  561688.3 |
  571449.5 |########################################
  581210.7 |
  590971.9 |##########################
  600733.1 |#############
  610494.3 |
  620255.5 |##########################
  630016.7 |##########################
  639778.0 |
  649539.2 |#############
  659300.4 |
  669061.6 |
  678822.8 |
  688584.0 |
  698345.2 |
  708106.4 |#############
  717867.6 |
  727628.8 |
  (1 below, 2 above range)

nway (n=18, range 600930.2-1011595.7 ns)
  600930.2 |####################
  621463.5 |####################
  641996.8 |
  662530.1 |
  683063.3 |####################
  703596.6 |
  724129.9 |########################################
  744663.2 |
  765196.4 |
  785729.7 |####################
  806263.0 |########################################
  826796.2 |
  847329.5 |####################
  867862.8 |
  888396.1 |####################
  908929.3 |
  929462.6 |########################################
  949995.9 |
  970529.2 |
  991062.4 |####################
  (2 below, 3 above range)

sequential (n=18, range 1116717.9-1287497.8 ns)
  1116717.9 |#############
  1125256.9 |
  1133795.9 |#############
  1142334.9 |
  1150873.9 |########################################
  1159412.9 |#############
  1167951.9 |
  1176490.9 |##########################
  1185029.9 |
  1193568.9 |#############
  1202107.9 |##########################
  1210646.9 |
  1219185.8 |
  1227724.8 |
  1236263.8 |
  1244802.8 |#############
  1253341.8 |
  1261880.8 |#############
  1270419.8 |
  1278958.8 |#############
  (1 below, 3 above range)

std_parallel (n=18, range 311461.5-576077.4 ns)
  311461.5 |################
  324692.3 |########
  337923.1 |################
  351153.9 |
  364384.7 |
  377615.5 |########
  390846.3 |
  404077.1 |########################################
  417307.9 |################
  430538.7 |
  443769.5 |########
  457000.2 |
  470231.0 |
  483461.8 |
  496692.6 |
  509923.4 |
  523154.2 |
  536385.0 |
  549615.8 |
  562846.6 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=1028.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=21.1% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=982.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1486.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=1003.8% of algo (FFI overhead may distort results)
- **nway**: bridge=749.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=543.7% of algo (FFI overhead may distort results)
- **std_parallel**: CV=33.0% (high variance, measurements may be unstable)
- **std_parallel**: bridge=1431.6% of algo (FFI overhead may distort results)

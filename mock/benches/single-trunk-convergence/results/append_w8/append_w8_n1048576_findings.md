# append, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway beats baseline by 40% (significant)

kernel_nway is -247.06 us (40%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 2.8x slower than the field

sequential (1.16 ms) is 2.8x the fastest (408.43 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.67)

nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### nway is inconsistent: worst-20% is 1.9x its best-20%

nway's best 20% of batches run at 527.11 us but its worst 20% at 995.26 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 408428.5 ns median (-33.1% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.83x (fastest 408428.5 ns, slowest 1157284.6 ns)

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
| Counter frequency | 2100242360 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 645585ns | 612564ns | 561073ns | 603281ns | 751296ns | base |
| head_tail_dyn | 601198ns | 580025ns | 538920ns | 572207ns | 675823ns | -6.88% |
| kernel_nway | 406044ns | 411560ns | 296915ns | 387035ns | 489121ns | -37.10% |
| kernel_seq | 594901ns | 575555ns | 541757ns | 568543ns | 661009ns | -7.85% |
| nway | 820090ns | 860790ns | 528906ns | 799074ns | 997205ns | +27.03% |
| sequential | 1185681ns | 1158930ns | 1103949ns | 1152324ns | 1276581ns | +83.66% |
| std_parallel | 424364ns | 414906ns | 351470ns | 406453ns | 487677ns | -34.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 643419ns | 558848ns | 748918ns | base | 1.630 |
| head_tail_dyn | 599062ns | 537100ns | 673588ns | -6.89% | 1.750 |
| kernel_nway | 403803ns | 294653ns | 487200ns | -37.24% | 2.597 |
| kernel_seq | 592737ns | 539963ns | 658651ns | -7.88% | 1.769 |
| nway | 818210ns | 527114ns | 995257ns | +27.17% | 1.282 |
| sequential | 1183773ns | 1102359ns | 1274355ns | +83.98% | 0.886 |
| std_parallel | 422230ns | 349255ns | 485807ns | -34.38% | 2.483 |

## Performance model

- Peak throughput: **3.559 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.717 | 48.3% |
| head_tail_dyn | 1.814 | 51.0% |
| kernel_nway | 2.567 | 72.1% |
| kernel_seq | 1.830 | 51.4% |
| nway | 1.221 | 34.3% |
| sequential | 0.906 | 25.5% |
| std_parallel | 2.540 | 71.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 645585ns | 645585ns | base |
| head_tail_dyn | 601198ns | 601198ns | -6.88% |
| kernel_nway | 406044ns | 406044ns | -37.10% |
| kernel_seq | 594901ns | 594901ns | -7.85% |
| nway | 820090ns | 820090ns | +27.03% |
| sequential | 1185681ns | 1185681ns | +83.66% |
| std_parallel | 424364ns | 424364ns | -34.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 610662ns | base | --- | [580602, 656432] | --- | --- | --- | --- |
| head_tail_dyn | 577935ns | no significant difference | [-77045, +19026]ns | [557103, 615010] | no | 0.4807 | 0.4807 | 0 |
| kernel_nway | 408429ns | -247059.1ns (-40.5%) | [-305174, -161592]ns | [350614, 431069] | YES | 0.0003 | 0.0001 | 0 |
| kernel_seq | 572917ns | no significant difference | [-108847, +11503]ns | [550248, 604285] | no | 0.2855 | 0.2379 | 0 |
| nway | 858837ns | +208060.4ns (+34.1%) | [+44951, +309472]ns | [713275, 896311] | YES | 0.0463 | 0.0309 | 0 |
| sequential | 1157285ns | +544060.3ns (+89.1%) | [+509244, +574635]ns | [1136185, 1211075] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 412847ns | -207177.5ns (-33.9%) | [-267772, -187680]ns | [383801, 448053] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 581739ns | -3.8% | -27.8% | -3.3% | +52.6% | +91.0% | -22.8% |
| 1 | 2 | 621072ns | -4.0% | -47.9% | -1.8% | +3.6% | +83.4% | -38.6% |
| 1 | 3 | 750094ns | -25.0% | -46.6% | -23.2% | +18.0% | +64.3% | -42.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.058 | ok |
| head_tail_dyn | -0.663 | HIGH- (thermal bounce) |
| kernel_nway | -0.642 | HIGH- (thermal bounce) |
| kernel_seq | -0.612 | HIGH- (thermal bounce) |
| nway | -0.667 | HIGH- (thermal bounce) |
| sequential | -0.058 | ok |
| std_parallel | -0.637 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 5924193.9ns | 643418.6ns | 920.7% | HIGH |
| head_tail_dyn | 5951715.7ns | 599061.6ns | 993.5% | HIGH |
| kernel_nway | 5782910.5ns | 403803.4ns | 1432.1% | HIGH |
| kernel_seq | 5974044.2ns | 592737.2ns | 1007.9% | HIGH |
| nway | 6260685.1ns | 818209.8ns | 765.2% | HIGH |
| sequential | 6583556.5ns | 1183773.0ns | 556.2% | HIGH |
| std_parallel | 5928590.7ns | 422230.1ns | 1404.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 558848.0-748918.4 ns)
  558848.0 |#############
  568351.5 |#############
  577855.0 |########################################
  587358.5 |#############
  596862.1 |#############
  606365.6 |
  615869.1 |#############
  625372.6 |
  634876.1 |##########################
  644379.7 |#############
  653883.2 |#############
  663386.7 |
  672890.2 |
  682393.7 |
  691897.3 |
  701400.8 |
  710904.3 |
  720407.8 |
  729911.4 |
  739414.9 |#############
  (2 below, 3 above range)

head_tail_dyn (n=18, range 537100.3-673588.2 ns)
  537100.3 |########################################
  543924.7 |########################################
  550749.1 |
  557573.5 |####################
  564397.9 |########################################
  571222.3 |####################
  578046.7 |########################################
  584871.1 |
  591695.5 |
  598519.9 |####################
  605344.2 |
  612168.6 |########################################
  618993.0 |
  625817.4 |
  632641.8 |
  639466.2 |
  646290.6 |########################################
  653115.0 |
  659939.4 |
  666763.8 |
  (1 below, 2 above range)

kernel_nway (n=18, range 294652.7-487199.7 ns)
  294652.7 |####################
  304280.0 |
  313907.4 |####################
  323534.7 |########################################
  333162.1 |
  342789.4 |
  352416.8 |
  362044.1 |########################################
  371671.5 |
  381298.8 |
  390926.2 |
  400553.5 |########################################
  410180.9 |########################################
  419808.2 |####################
  429435.6 |####################
  439062.9 |########################################
  448690.3 |
  458317.6 |
  467945.0 |
  477572.3 |
  (1 below, 3 above range)

kernel_seq (n=18, range 539963.4-658651.2 ns)
  539963.4 |##########################
  545897.8 |##########################
  551832.2 |########################################
  557766.6 |
  563701.0 |
  569635.3 |##########################
  575569.7 |
  581504.1 |
  587438.5 |
  593372.9 |##########################
  599307.3 |#############
  605241.7 |##########################
  611176.1 |
  617110.4 |
  623044.8 |
  628979.2 |#############
  634913.6 |
  640848.0 |
  646782.4 |
  652716.8 |
  (1 below, 2 above range)

nway (n=18, range 527114.4-995256.6 ns)
  527114.4 |##########
  550521.5 |
  573928.6 |##########
  597335.7 |
  620742.8 |
  644149.9 |
  667557.1 |
  690964.2 |####################
  714371.3 |####################
  737778.4 |
  761185.5 |
  784592.6 |
  807999.7 |##########
  831406.8 |##########
  854813.9 |##########
  878221.0 |########################################
  901628.1 |##########
  925035.3 |
  948442.4 |
  971849.5 |
  (1 below, 3 above range)

sequential (n=18, range 1102359.1-1274354.8 ns)
  1102359.1 |#############
  1110958.9 |#############
  1119558.7 |
  1128158.5 |#############
  1136758.2 |########################################
  1145358.0 |
  1153957.8 |##########################
  1162557.6 |
  1171157.4 |#############
  1179757.2 |
  1188357.0 |
  1196956.7 |#############
  1205556.5 |#############
  1214156.3 |
  1222756.1 |#############
  1231355.9 |
  1239955.7 |##########################
  1248555.5 |
  1257155.2 |
  1265755.0 |
  (2 below, 2 above range)

std_parallel (n=18, range 349255.3-485807.2 ns)
  349255.3 |
  356082.9 |
  362910.5 |
  369738.1 |########################################
  376565.7 |####################
  383393.2 |
  390220.8 |####################
  397048.4 |####################
  403876.0 |########################################
  410703.6 |####################
  417531.2 |
  424358.8 |####################
  431186.4 |
  438014.0 |####################
  444841.6 |########################################
  451669.2 |
  458496.8 |####################
  465324.4 |
  472152.0 |
  478979.6 |####################
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=974.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=1023.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1413.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=1035.0% of algo (FFI overhead may distort results)
- **nway**: CV=21.6% (high variance, measurements may be unstable)
- **nway**: bridge=725.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=557.7% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=1381.5% of algo (FFI overhead may distort results)

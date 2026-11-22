# append, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel dominates: 51% faster than the next best (head_tail)

std_parallel (364.46 us) leads head_tail (549.34 us) by 51%, a clear separation rather than a photo finish. CV 35.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel beats baseline by 41% (significant)

std_parallel is -226.85 us (41%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### nway is an outlier: 8.3x slower than the field

nway (3.02 ms) is 8.3x the fastest (364.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_parallel is fastest but the noisiest (CV 35.4%)

std_parallel wins on median (364.46 us) yet has the highest variance (CV 35.4%), while sequential is the steadiest (CV 1.1%, 845.35 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.66)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {std_parallel, head_tail, head_tail_dyn, kernel_seq, sequential, kernel_nway} vs {nway} (123% apart)

The field splits into a fast tier {std_parallel, head_tail, head_tail_dyn, kernel_seq, sequential, kernel_nway} and a slow tier {nway} with a 123% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 8.3x the fastest

Fastest std_parallel (364.46 us) to slowest nway (3.02 ms): 8.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### std_parallel is inconsistent: worst-20% is 2.0x its best-20%

std_parallel's best 20% of batches run at 286.42 us but its worst 20% at 568.45 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_parallel** at 364464.2 ns median (-33.7% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 8.27x (fastest 364464.2 ns, slowest 3015534.8 ns)

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
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 579180ns | 550338ns | 534130ns | 547160ns | 649735ns | base |
| head_tail_dyn | 568282ns | 555443ns | 522949ns | 549157ns | 619637ns | -1.88% |
| kernel_nway | 1330137ns | 1353612ns | 1013521ns | 1308828ns | 1520408ns | +129.66% |
| kernel_seq | 631073ns | 629777ns | 540225ns | 615414ns | 699985ns | +8.96% |
| nway | 3007181ns | 3016499ns | 2677062ns | 2987515ns | 3201739ns | +419.21% |
| sequential | 842867ns | 846155ns | 828684ns | 841819ns | 851530ns | +45.53% |
| std_parallel | 409653ns | 365772ns | 287538ns | 343410ns | 570074ns | -29.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 577830ns | 533102ns | 647962ns | base | 1.815 |
| head_tail_dyn | 566773ns | 521526ns | 617789ns | -1.91% | 1.850 |
| kernel_nway | 1328506ns | 1011901ns | 1519110ns | +129.91% | 0.789 |
| kernel_seq | 629748ns | 538812ns | 698613ns | +8.98% | 1.665 |
| nway | 3006151ns | 2675989ns | 3200702ns | +420.25% | 0.349 |
| sequential | 842263ns | 828191ns | 850967ns | +45.76% | 1.245 |
| std_parallel | 408240ns | 286423ns | 568451ns | -29.35% | 2.569 |

## Performance model

- Peak throughput: **3.661 Gops/s** (std_parallel; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.909 | 52.1% |
| head_tail_dyn | 1.893 | 51.7% |
| kernel_nway | 0.776 | 21.2% |
| kernel_seq | 1.668 | 45.6% |
| nway | 0.348 | 9.5% |
| sequential | 1.240 | 33.9% |
| std_parallel | 2.877 | 78.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 579180ns | 579180ns | base |
| head_tail_dyn | 568282ns | 568282ns | -1.88% |
| kernel_nway | 1330137ns | 1330137ns | +129.66% |
| kernel_seq | 631073ns | 631073ns | +8.96% |
| nway | 3007181ns | 3007181ns | +419.21% |
| sequential | 842867ns | 842867ns | +45.53% |
| std_parallel | 409653ns | 409653ns | -29.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 549337ns | base | --- | [538641, 633531] | --- | --- | --- | --- |
| head_tail_dyn | 553936ns | no significant difference | [-43355, +17231]ns | [541387, 579140] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 1351409ns | +755652.0ns (+137.6%) | [+658073, +881862]ns | [1213757, 1460990] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 628455ns | no significant difference | [-9925, +93622]ns | [592829, 640171] | no | 0.1155 | 0.0963 | 0 |
| nway | 3015535ns | +2413939.8ns (+439.4%) | [+2383717, +2484745]ns | [2949815, 3087154] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 845353ns | +288160.7ns (+52.5%) | [+216040, +300523]ns | [834359, 849607] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 364464ns | -226848.9ns (-41.3%) | [-240438, -104710]ns | [299086, 465837] | YES | 0.0113 | 0.0075 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 538641ns | +2.8% | +178.4% | +16.9% | +453.6% | +56.9% | -41.9% |
| 1 | 2 | 545875ns | +2.7% | +108.2% | +12.1% | +451.4% | +52.0% | -23.5% |
| 1 | 3 | 649242ns | -14.9% | +112.6% | -5.3% | +366.6% | +31.0% | -45.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.134 | ok |
| head_tail_dyn | -0.656 | HIGH- (thermal bounce) |
| kernel_nway | -0.598 | HIGH- (thermal bounce) |
| kernel_seq | -0.248 | moderate- |
| nway | -0.007 | ok |
| sequential | -0.626 | HIGH- (thermal bounce) |
| std_parallel | -0.562 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 4640852.9ns | 577830.2ns | 803.2% | HIGH |
| head_tail_dyn | 4574860.2ns | 566772.7ns | 807.2% | HIGH |
| kernel_nway | 14835301.3ns | 1328505.6ns | 1116.7% | HIGH |
| kernel_seq | 4653003.1ns | 629747.6ns | 738.9% | HIGH |
| nway | 14229528.6ns | 3006151.3ns | 473.3% | HIGH |
| sequential | 4708677.6ns | 842263.5ns | 559.1% | HIGH |
| std_parallel | 5925498.8ns | 408239.5ns | 1451.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 533102.3-647961.6 ns)
  533102.3 |########################################
  538845.3 |##########################
  544588.3 |########################################
  550331.2 |#############
  556074.2 |#############
  561817.2 |
  567560.1 |
  573303.1 |
  579046.1 |
  584789.0 |
  590532.0 |
  596275.0 |
  602017.9 |
  607760.9 |
  613503.9 |
  619246.8 |
  624989.8 |
  630732.8 |##########################
  636475.7 |#############
  642218.7 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 521525.6-617789.5 ns)
  521525.6 |#############
  526338.8 |##########################
  531152.0 |
  535965.2 |#############
  540778.4 |########################################
  545591.6 |
  550404.8 |#############
  555218.0 |#############
  560031.2 |########################################
  564844.3 |
  569657.5 |
  574470.7 |
  579283.9 |
  584097.1 |
  588910.3 |#############
  593723.5 |
  598536.7 |
  603349.9 |
  608163.1 |
  612976.3 |#############
  (1 below, 3 above range)

kernel_nway (n=18, range 1011900.7-1519110.4 ns)
  1011900.7 |
  1037261.2 |
  1062621.7 |
  1087982.2 |####################
  1113342.7 |
  1138703.2 |
  1164063.6 |####################
  1189424.1 |########################################
  1214784.6 |
  1240145.1 |
  1265505.6 |
  1290866.1 |####################
  1316226.5 |########################################
  1341587.0 |####################
  1366947.5 |
  1392308.0 |####################
  1417668.5 |####################
  1443029.0 |########################################
  1468389.4 |####################
  1493749.9 |
  (2 below, 3 above range)

kernel_seq (n=18, range 538812.2-698613.0 ns)
  538812.2 |
  546802.2 |
  554792.2 |
  562782.3 |#############
  570772.3 |#############
  578762.4 |
  586752.4 |#############
  594742.4 |#############
  602732.5 |
  610722.5 |#############
  618712.6 |#############
  626702.6 |########################################
  634692.6 |########################################
  642682.7 |#############
  650672.7 |##########################
  658662.8 |
  666652.8 |
  674642.8 |
  682632.9 |
  690622.9 |
  (2 below, 1 above range)

nway (n=18, range 2675989.2-3200702.1 ns)
  2675989.2 |
  2702224.8 |#############
  2728460.5 |
  2754696.1 |
  2780931.8 |
  2807167.4 |
  2833403.1 |
  2859638.7 |
  2885874.4 |
  2912110.0 |##########################
  2938345.7 |########################################
  2964581.3 |#############
  2990817.0 |#############
  3017052.6 |##########################
  3043288.2 |##########################
  3069523.9 |
  3095759.5 |#############
  3121995.2 |
  3148230.8 |#############
  3174466.5 |
  (1 below, 3 above range)

sequential (n=18, range 828191.1-850966.7 ns)
  828191.1 |
  829329.9 |########################################
  830468.7 |
  831607.5 |
  832746.3 |
  833885.0 |
  835023.8 |
  836162.6 |
  837301.4 |
  838440.2 |##########
  839578.9 |
  840717.7 |
  841856.5 |
  842995.3 |##########
  844134.1 |####################
  845272.8 |####################
  846411.6 |##########
  847550.4 |
  848689.2 |##########
  849828.0 |##############################
  (1 below, 2 above range)

std_parallel (n=18, range 286423.4-568450.5 ns)
  286423.4 |########################################
  300524.8 |##############################
  314626.1 |##########
  328727.5 |
  342828.8 |
  356930.2 |
  371031.5 |
  385132.9 |
  399234.3 |##########
  413335.6 |####################
  427437.0 |
  441538.3 |##########
  455639.7 |
  469741.0 |##########
  483842.4 |
  497943.8 |
  512045.1 |
  526146.5 |
  540247.8 |
  554349.2 |
  (1 below, 4 above range)

```

## Diagnostics

- **head_tail**: bridge=822.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=821.6% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1127.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=733.0% of algo (FFI overhead may distort results)
- **nway**: bridge=475.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=557.1% of algo (FFI overhead may distort results)
- **std_parallel**: CV=31.6% (high variance, measurements may be unstable)
- **std_parallel**: bridge=1551.1% of algo (FFI overhead may distort results)

# append, far above cache, W = 1

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.67)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_seq, head_tail_dyn, kernel_nway, head_tail, std_parallel} vs {sequential, nway} (32% apart)

The field splits into a fast tier {kernel_seq, head_tail_dyn, kernel_nway, head_tail, std_parallel} and a slow tier {sequential, nway} with a 32% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: kernel_seq** at 9495891.0 ns median (-4.2% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.56x (fastest 9495891.0 ns, slowest 14772597.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, far above cache, W = 1 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 9708383ns | 9912761ns | 9270522ns | 9703951ns | 9933961ns | base |
| head_tail_dyn | 9679438ns | 9904879ns | 8897686ns | 9749757ns | 9964837ns | -0.30% |
| kernel_nway | 9694188ns | 9906535ns | 9056135ns | 9703541ns | 9999185ns | -0.15% |
| kernel_seq | 9527506ns | 9498884ns | 9162662ns | 9420184ns | 9870912ns | -1.86% |
| nway | 14762931ns | 14774930ns | 14529633ns | 14762512ns | 14880207ns | +52.06% |
| sequential | 14351623ns | 14155573ns | 13908082ns | 14095426ns | 14957689ns | +47.83% |
| std_parallel | 10637313ns | 10712839ns | 10058850ns | 10646110ns | 10913349ns | +9.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 9704518ns | 9267589ns | 9930182ns | base | 1.729 |
| head_tail_dyn | 9675616ns | 8895000ns | 9959996ns | -0.30% | 1.734 |
| kernel_nway | 9690326ns | 9051314ns | 9994934ns | -0.15% | 1.731 |
| kernel_seq | 9523859ns | 9158548ns | 9866228ns | -1.86% | 1.762 |
| nway | 14760560ns | 14527374ns | 14877737ns | +52.10% | 1.137 |
| sequential | 14349920ns | 13906539ns | 14955888ns | +47.87% | 1.169 |
| std_parallel | 10633620ns | 10055885ns | 10908748ns | +9.57% | 1.578 |

## Performance model

- Peak throughput: **1.886 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.693 | 89.8% |
| head_tail_dyn | 1.694 | 89.8% |
| kernel_nway | 1.694 | 89.8% |
| kernel_seq | 1.767 | 93.7% |
| nway | 1.136 | 60.2% |
| sequential | 1.185 | 62.8% |
| std_parallel | 1.567 | 83.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 9708383ns | 9708383ns | base |
| head_tail_dyn | 9679438ns | 9679438ns | -0.30% |
| kernel_nway | 9694188ns | 9694188ns | -0.15% |
| kernel_seq | 9527506ns | 9527506ns | -1.86% |
| nway | 14762931ns | 14762931ns | +52.06% |
| sequential | 14351623ns | 14351623ns | +47.83% |
| std_parallel | 10637313ns | 10637313ns | +9.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 9909036ns | base | --- | [9274337, 9930182] | --- | --- | --- | --- |
| head_tail_dyn | 9901823ns | no significant difference | [-744008, +638454]ns | [9165028, 9959996] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 9903062ns | no significant difference | [-736055, +720598]ns | [9172981, 9994934] | no | 0.8250 | 0.6875 | 0 |
| kernel_seq | 9495891ns | no significant difference | [-699579, +469859]ns | [9209457, 9866228] | no | 0.8250 | 0.6875 | 0 |
| nway | 14772598ns | +4968701.0ns (+50.1%) | [+4701162, +5498261]ns | [14631345, 14877737] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| sequential | 14153959ns | +4761453.5ns (+48.1%) | [+4018883, +5155868]ns | [13939913, 14955888] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| std_parallel | 10709357ns | +935565.6ns (+9.4%) | [+402645, +1449095]ns | [10282756, 10908748] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 9274337ns | +6.9% | +7.8% | +5.1% | +59.3% | +52.6% | +15.6% |
| 1 | 2 | 9909036ns | -7.5% | -7.4% | -7.1% | +50.1% | +44.4% | +8.1% |
| 1 | 3 | 9930182ns | +0.2% | -0.3% | -3.1% | +47.3% | +46.9% | +5.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.151 | ok |
| head_tail_dyn | -0.666 | HIGH- (thermal bounce) |
| kernel_nway | -0.660 | HIGH- (thermal bounce) |
| kernel_seq | -0.633 | HIGH- (thermal bounce) |
| nway | -0.449 | moderate- |
| sequential | -0.016 | ok |
| std_parallel | -0.139 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 72467747.2ns | 9704518.4ns | 746.7% | HIGH |
| head_tail_dyn | 72507214.8ns | 9675615.6ns | 749.4% | HIGH |
| kernel_nway | 72376309.1ns | 9690325.8ns | 746.9% | HIGH |
| kernel_seq | 72493409.8ns | 9523858.8ns | 761.2% | HIGH |
| nway | 77517909.1ns | 14760559.9ns | 525.2% | HIGH |
| sequential | 76508946.1ns | 14349919.9ns | 533.2% | HIGH |
| std_parallel | 72615842.8ns | 10633620.4ns | 682.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 9267588.7-9930182.4 ns)
  9267588.7 |##########################
  9300718.4 |
  9333848.1 |
  9366977.8 |
  9400107.4 |
  9433237.1 |
  9466366.8 |
  9499496.5 |
  9532626.2 |
  9565755.9 |
  9598885.6 |
  9632015.3 |
  9665144.9 |
  9698274.6 |
  9731404.3 |
  9764534.0 |
  9797663.7 |
  9830793.4 |
  9863923.1 |
  9897052.8 |########################################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 8894999.9-9959996.1 ns)
  8894999.9 |####################
  8948249.7 |
  9001499.5 |
  9054749.3 |
  9107999.1 |
  9161248.9 |
  9214498.8 |
  9267748.6 |
  9320998.4 |
  9374248.2 |
  9427498.0 |####################
  9480747.8 |
  9533997.6 |
  9587247.4 |
  9640497.2 |
  9693747.1 |
  9746996.9 |
  9800246.7 |
  9853496.5 |####################
  9906746.3 |########################################
  (0 below, 1 above range)

kernel_nway (n=6, range 9051313.9-9994934.4 ns)
  9051313.9 |########################################
  9098494.9 |
  9145676.0 |
  9192857.0 |
  9240038.0 |
  9287219.0 |########################################
  9334400.1 |
  9381581.1 |
  9428762.1 |
  9475943.1 |
  9523124.2 |
  9570305.2 |
  9617486.2 |
  9664667.2 |
  9711848.2 |
  9759029.3 |
  9806210.3 |
  9853391.3 |########################################
  9900572.3 |########################################
  9947753.4 |########################################
  (0 below, 1 above range)

kernel_seq (n=6, range 9158547.5-9866228.1 ns)
  9158547.5 |########################################
  9193931.5 |
  9229315.6 |########################################
  9264699.6 |
  9300083.6 |########################################
  9335467.7 |
  9370851.7 |
  9406235.7 |
  9441619.7 |
  9477003.8 |
  9512387.8 |
  9547771.8 |
  9583155.9 |
  9618539.9 |
  9653923.9 |########################################
  9689308.0 |
  9724692.0 |
  9760076.0 |
  9795460.0 |########################################
  9830844.1 |
  (0 below, 1 above range)

nway (n=6, range 14527374.3-14877737.2 ns)
  14527374.3 |########################################
  14544892.4 |
  14562410.6 |
  14579928.7 |
  14597446.9 |
  14614965.0 |
  14632483.2 |
  14650001.3 |
  14667519.5 |
  14685037.6 |
  14702555.8 |
  14720073.9 |########################################
  14737592.0 |########################################
  14755110.2 |
  14772628.3 |
  14790146.5 |########################################
  14807664.6 |########################################
  14825182.8 |
  14842700.9 |
  14860219.1 |
  (0 below, 1 above range)

sequential (n=6, range 13906539.2-14955887.8 ns)
  13906539.2 |########################################
  13959006.6 |########################################
  14011474.1 |########################################
  14063941.5 |
  14116408.9 |
  14168876.3 |
  14221343.8 |########################################
  14273811.2 |
  14326278.6 |
  14378746.1 |
  14431213.5 |
  14483680.9 |
  14536148.4 |
  14588615.8 |
  14641083.2 |########################################
  14693550.7 |
  14746018.1 |
  14798485.5 |
  14850952.9 |
  14903420.4 |
  (0 below, 1 above range)

std_parallel (n=6, range 10055885.3-10908748.2 ns)
  10055885.3 |########################################
  10098528.4 |
  10141171.6 |
  10183814.7 |
  10226457.9 |
  10269101.0 |
  10311744.2 |
  10354387.3 |
  10397030.4 |
  10439673.6 |
  10482316.7 |########################################
  10524959.9 |
  10567603.0 |########################################
  10610246.2 |
  10652889.3 |
  10695532.4 |
  10738175.6 |
  10780818.7 |
  10823461.9 |########################################
  10866105.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=731.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=733.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=732.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=763.7% of algo (FFI overhead may distort results)
- **nway**: bridge=524.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=537.0% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=676.8% of algo (FFI overhead may distort results)

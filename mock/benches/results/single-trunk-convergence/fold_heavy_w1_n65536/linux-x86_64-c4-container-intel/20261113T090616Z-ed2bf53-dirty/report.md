# fold_heavy, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 11% faster than the next best (nway)

sequential (271.72 us) leads nway (301.33 us) by 11%, a clear separation rather than a photo finish. CV 7.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential shows alternating (throttle bounce) (autocorr -0.61)

sequential's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### kernel_seq is inconsistent: worst-20% is 1.7x its best-20%

kernel_seq's best 20% of batches run at 284.26 us but its worst 20% at 476.40 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 271719.7 ns median (-13.0% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.47x (fastest 271719.7 ns, slowest 400449.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 1 |
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
| head_tail | 348159ns | 313804ns | 285822ns | 305949ns | 442643ns | base |
| head_tail_dyn | 355500ns | 319162ns | 288386ns | 316439ns | 447647ns | +2.11% |
| kernel_nway | 373960ns | 350888ns | 289116ns | 340996ns | 465826ns | +7.41% |
| kernel_seq | 367748ns | 325983ns | 285097ns | 322107ns | 477534ns | +5.63% |
| nway | 309942ns | 302004ns | 295362ns | 301616ns | 329721ns | -10.98% |
| sequential | 277257ns | 272071ns | 254280ns | 269213ns | 300810ns | -20.37% |
| std_parallel | 401664ns | 401464ns | 332532ns | 393380ns | 448657ns | +15.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 346929ns | 285015ns | 441118ns | base | 0.189 |
| head_tail_dyn | 354102ns | 287287ns | 446360ns | +2.07% | 0.185 |
| kernel_nway | 372588ns | 288447ns | 464092ns | +7.40% | 0.176 |
| kernel_seq | 366738ns | 284264ns | 476401ns | +5.71% | 0.179 |
| nway | 309238ns | 294515ns | 329005ns | -10.86% | 0.212 |
| sequential | 276839ns | 253949ns | 300296ns | -20.20% | 0.237 |
| std_parallel | 400527ns | 331459ns | 447323ns | +15.45% | 0.164 |

## Performance model

- Peak throughput: **0.258 Gops/s** (sequential; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.210 | 81.3% |
| head_tail_dyn | 0.207 | 80.1% |
| kernel_nway | 0.188 | 72.7% |
| kernel_seq | 0.202 | 78.4% |
| nway | 0.217 | 84.3% |
| sequential | 0.241 | 93.5% |
| std_parallel | 0.164 | 63.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 348159ns | 348159ns | base |
| head_tail_dyn | 355500ns | 355500ns | +2.11% |
| kernel_nway | 373960ns | 373960ns | +7.41% |
| kernel_seq | 367748ns | 367748ns | +5.63% |
| nway | 309942ns | 309942ns | -10.98% |
| sequential | 277257ns | 277257ns | -20.37% |
| std_parallel | 401664ns | 401664ns | +15.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 312380ns | base | --- | [291344, 387901] | --- | --- | --- | --- |
| head_tail_dyn | 317071ns | no significant difference | [-82606, +113832]ns | [303222, 418529] | no | 0.9774 | 0.8145 | 0 |
| kernel_nway | 349324ns | +40425.6ns (+12.9%) | [+10353, +66061]ns | [320923, 369909] | YES (adj: no) | 0.0927 | 0.0309 | 0 |
| kernel_seq | 323919ns | no significant difference | [-67750, +92670]ns | [301096, 397795] | no | 0.9774 | 0.8145 | 0 |
| nway | 301331ns | no significant difference | [-75890, +14662]ns | [298292, 310376] | no | 1.0000 | 1.0000 | 0 |
| sequential | 271720ns | -35955.7ns (-11.5%) | [-91734, -25770]ns | [259414, 286407] | YES | 0.0009 | 0.0001 | 0 |
| std_parallel | 400449ns | +55614.5ns (+17.8%) | [+2284, +102861]ns | [378243, 422591] | YES (adj: no) | 0.1925 | 0.0963 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 309860ns | +23.1% | +1.1% | +4.5% | +1.0% | -7.6% | +17.9% |
| 1 | 2 | 287126ns | +24.6% | +24.0% | +38.5% | +4.8% | -11.5% | +40.9% |
| 1 | 3 | 445543ns | -30.2% | +7.1% | -35.0% | -33.3% | -37.9% | -5.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.248 | moderate- |
| head_tail_dyn | -0.023 | ok |
| kernel_nway | -0.047 | ok |
| kernel_seq | -0.603 | HIGH- (thermal bounce) |
| nway | -0.053 | ok |
| sequential | -0.611 | HIGH- (thermal bounce) |
| std_parallel | -0.029 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 345664.4ns | 346929.4ns | 99.6% | HIGH |
| head_tail_dyn | 372319.5ns | 354102.1ns | 105.1% | HIGH |
| kernel_nway | 373798.4ns | 372587.9ns | 100.3% | HIGH |
| kernel_seq | 330047.5ns | 366738.5ns | 90.0% | HIGH |
| nway | 310728.0ns | 309238.4ns | 100.5% | HIGH |
| sequential | 282949.7ns | 276838.8ns | 102.2% | HIGH |
| std_parallel | 384251.4ns | 400527.0ns | 95.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 285014.6-441117.7 ns)
  285014.6 |########################################
  292819.8 |##########################
  300624.9 |##########################
  308430.1 |
  316235.2 |########################################
  324040.4 |
  331845.5 |
  339650.7 |
  347455.9 |
  355261.0 |
  363066.2 |
  370871.3 |#############
  378676.5 |
  386481.6 |
  394286.8 |
  402091.9 |#############
  409897.1 |
  417702.2 |
  425507.4 |
  433312.5 |#############
  (2 below, 3 above range)

head_tail_dyn (n=18, range 287287.3-446360.2 ns)
  287287.3 |
  295240.9 |########################################
  303194.6 |##########################
  311148.2 |########################################
  319101.9 |
  327055.5 |
  335009.2 |##########################
  342962.8 |
  350916.5 |
  358870.1 |
  366823.8 |
  374777.4 |
  382731.1 |
  390684.7 |
  398638.4 |
  406592.0 |#############
  414545.6 |
  422499.3 |########################################
  430452.9 |
  438406.6 |
  (2 below, 2 above range)

kernel_nway (n=18, range 288446.5-464092.3 ns)
  288446.5 |
  297228.8 |#############
  306011.1 |#############
  314793.4 |##########################
  323575.7 |
  332358.0 |
  341140.3 |########################################
  349922.6 |#############
  358704.8 |##########################
  367487.1 |##########################
  376269.4 |
  385051.7 |
  393834.0 |
  402616.3 |
  411398.6 |#############
  420180.9 |
  428963.1 |
  437745.4 |
  446527.7 |
  455310.0 |
  (2 below, 3 above range)

kernel_seq (n=18, range 284263.6-476400.9 ns)
  284263.6 |########################################
  293870.4 |########################################
  303477.3 |####################
  313084.2 |########################################
  322691.0 |####################
  332297.9 |
  341904.8 |####################
  351511.6 |
  361118.5 |####################
  370725.4 |
  380332.2 |####################
  389939.1 |
  399546.0 |####################
  409152.8 |
  418759.7 |
  428366.6 |
  437973.4 |####################
  447580.3 |
  457187.2 |####################
  466794.0 |
  (2 below, 2 above range)

nway (n=18, range 294515.1-329005.2 ns)
  294515.1 |
  296239.6 |########################################
  297964.1 |#############
  299688.7 |########################################
  301413.2 |#############
  303137.7 |#############
  304862.2 |#############
  306586.7 |
  308311.2 |
  310035.7 |##########################
  311760.2 |
  313484.7 |
  315209.2 |##########################
  316933.7 |
  318658.2 |
  320382.7 |
  322107.2 |
  323831.7 |
  325556.2 |
  327280.7 |
  (2 below, 2 above range)

sequential (n=18, range 253949.2-300296.1 ns)
  253949.2 |########################################
  256266.6 |
  258583.9 |##########################
  260901.3 |
  263218.6 |
  265535.9 |
  267853.3 |##########################
  270170.6 |#############
  272488.0 |#############
  274805.3 |
  277122.7 |
  279440.0 |#############
  281757.3 |#############
  284074.7 |#############
  286392.0 |#############
  288709.4 |#############
  291026.7 |#############
  293344.1 |
  295661.4 |
  297978.7 |
  (1 below, 2 above range)

std_parallel (n=18, range 331458.6-447322.6 ns)
  331458.6 |
  337251.8 |
  343045.0 |
  348838.2 |
  354631.4 |
  360424.6 |########################################
  366217.8 |####################
  372011.0 |
  377804.2 |
  383597.4 |########################################
  389390.6 |
  395183.8 |########################################
  400977.0 |####################
  406770.2 |####################
  412563.4 |####################
  418356.6 |########################################
  424149.8 |
  429943.0 |
  435736.2 |####################
  441529.4 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=20.8% (high variance, measurements may be unstable)
- **head_tail**: bridge=100.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=104.6% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=22.2% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=101.2% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=26.4% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=98.2% of algo (FFI overhead may distort results)
- **nway**: bridge=100.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=95.3% of algo (FFI overhead may distort results)

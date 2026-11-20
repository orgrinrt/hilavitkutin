# append, far above cache, W = 4

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel dominates: 39% faster than the next best (kernel_seq)

std_parallel (6.78 ms) leads kernel_seq (9.43 ms) by 39%, a clear separation rather than a photo finish. CV 16.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel beats baseline by 33% (significant)

std_parallel is -3.28 ms (33%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### nway is an outlier: 2.4x slower than the field

nway (16.56 ms) is 2.4x the fastest (6.78 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.67)

nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### nway is inconsistent: worst-20% is 4.0x its best-20%

nway's best 20% of batches run at 6.66 ms but its worst 20% at 26.62 ms (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_parallel** at 6778930.5 ns median (-32.4% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.44x (fastest 6778930.5 ns, slowest 16562543.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, far above cache, W = 4 |
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
| head_tail | 9954881ns | 10024663ns | 9439107ns | 9999686ns | 10145560ns | base |
| head_tail_dyn | 9503075ns | 9650386ns | 8737429ns | 9482710ns | 9916445ns | -4.54% |
| kernel_nway | 11674040ns | 12055846ns | 8700997ns | 11030298ns | 14126176ns | +17.27% |
| kernel_seq | 9615148ns | 9431465ns | 9347525ns | 9409181ns | 10057910ns | -3.41% |
| nway | 17751376ns | 16565570ns | 6661651ns | 15536610ns | 26618387ns | +78.32% |
| sequential | 14387901ns | 14044006ns | 13909414ns | 14004015ns | 15202974ns | +44.53% |
| std_parallel | 7281479ns | 6783205ns | 6730571ns | 6772434ns | 8320500ns | -26.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 9951048ns | 9436632ns | 10140387ns | base | 1.686 |
| head_tail_dyn | 9499139ns | 8735040ns | 9912457ns | -4.54% | 1.766 |
| kernel_nway | 11669274ns | 8694697ns | 14122667ns | +17.27% | 1.438 |
| kernel_seq | 9611475ns | 9342969ns | 10054169ns | -3.41% | 1.746 |
| nway | 17748483ns | 6658928ns | 26615630ns | +78.36% | 0.945 |
| sequential | 14386241ns | 13908016ns | 15201150ns | +44.57% | 1.166 |
| std_parallel | 7277945ns | 6727822ns | 8317064ns | -26.86% | 2.305 |

## Performance model

- Peak throughput: **2.520 Gops/s** (nway; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.674 | 66.4% |
| head_tail_dyn | 1.739 | 69.0% |
| kernel_nway | 1.392 | 55.3% |
| kernel_seq | 1.779 | 70.6% |
| nway | 1.013 | 40.2% |
| sequential | 1.195 | 47.4% |
| std_parallel | 2.475 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 9954881ns | 9954881ns | base |
| head_tail_dyn | 9503075ns | 9503075ns | -4.54% |
| kernel_nway | 11674040ns | 11674040ns | +17.27% |
| kernel_seq | 9615148ns | 9615148ns | -3.41% |
| nway | 17751376ns | 17751376ns | +78.32% |
| sequential | 14387901ns | 14387901ns | +44.53% |
| std_parallel | 7281479ns | 7281479ns | -26.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 10021088ns | base | --- | [9691668, 10140387] | --- | --- | --- | --- |
| head_tail_dyn | 9646359ns | -166124.5ns (-1.7%) | [-1186404, -3198]ns | [8938602, 9912457] | YES (adj: no) | 0.2625 | 0.2188 | 0 |
| kernel_nway | 12049995ns | no significant difference | [-861342, +4106411]ns | [8835159, 14122667] | no | 0.6875 | 0.6875 | 0 |
| kernel_seq | 9428887ns | -301552.3ns (-3.0%) | [-685101, -32066]ns | [9351369, 10054169] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| nway | 16562543ns | +6785941.6ns (+67.7%) | [+46188, +16560175]ns | [10067276, 26615630] | YES (adj: no) | 0.2625 | 0.2188 | 0 |
| sequential | 14042350ns | +4152452.3ns (+41.4%) | [+3981067, +5172060]ns | [13915223, 15201150] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| std_parallel | 6778930ns | -3278415.8ns (-32.7%) | [-3361457, -1379437]ns | [6737840, 8317064] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 9696501ns | -0.5% | -8.9% | -3.3% | +69.1% | +44.0% | -14.2% |
| 1 | 2 | 10031638ns | -1.2% | +32.7% | -6.3% | +102.2% | +41.0% | -32.7% |
| 1 | 3 | 10125006ns | -11.7% | +27.0% | -0.7% | +63.6% | +48.7% | -33.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.064 | ok |
| head_tail_dyn | -0.337 | moderate- |
| kernel_nway | -0.221 | moderate- |
| kernel_seq | -0.153 | ok |
| nway | -0.666 | HIGH- (thermal bounce) |
| sequential | -0.086 | ok |
| std_parallel | -0.174 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 72910415.7ns | 9951048.0ns | 732.7% | HIGH |
| head_tail_dyn | 72062276.9ns | 9499139.2ns | 758.6% | HIGH |
| kernel_nway | 80857388.5ns | 11669273.8ns | 692.9% | HIGH |
| kernel_seq | 72211109.4ns | 9611474.7ns | 751.3% | HIGH |
| nway | 87648376.5ns | 17748482.8ns | 493.8% | HIGH |
| sequential | 76133688.7ns | 14386241.2ns | 529.2% | HIGH |
| std_parallel | 74929615.2ns | 7277944.8ns | 1029.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 9436632.2-10140387.2 ns)
  9436632.2 |####################
  9471820.0 |
  9507007.7 |
  9542195.5 |
  9577383.2 |
  9612571.0 |
  9647758.7 |
  9682946.5 |
  9718134.2 |
  9753322.0 |
  9788509.7 |
  9823697.5 |
  9858885.2 |
  9894073.0 |
  9929260.7 |########################################
  9964448.5 |
  9999636.2 |
  10034824.0 |
  10070011.7 |####################
  10105199.5 |####################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 8735039.5-9912456.6 ns)
  8735039.5 |########################################
  8793910.4 |
  8852781.2 |
  8911652.1 |
  8970522.9 |
  9029393.8 |
  9088264.6 |########################################
  9147135.5 |
  9206006.4 |
  9264877.2 |
  9323748.1 |
  9382618.9 |
  9441489.8 |########################################
  9500360.6 |
  9559231.5 |
  9618102.4 |
  9676973.2 |
  9735844.1 |
  9794714.9 |########################################
  9853585.8 |########################################
  (0 below, 1 above range)

kernel_nway (n=6, range 8694697.4-14122667.3 ns)
  8694697.4 |####################
  8966095.9 |####################
  9237494.4 |
  9508892.9 |
  9780291.4 |
  10051689.9 |
  10323088.4 |
  10594486.9 |
  10865885.4 |
  11137283.9 |
  11408682.4 |
  11680080.9 |
  11951479.4 |########################################
  12222877.9 |
  12494276.4 |
  12765674.9 |
  13037073.4 |
  13308471.9 |
  13579870.4 |####################
  13851268.9 |
  (0 below, 1 above range)

kernel_seq (n=6, range 9342969.2-10054168.6 ns)
  9342969.2 |########################################
  9378529.2 |####################
  9414089.1 |
  9449649.1 |####################
  9485209.1 |
  9520769.0 |
  9556329.0 |
  9591889.0 |
  9627449.0 |
  9663008.9 |
  9698568.9 |
  9734128.9 |
  9769688.8 |
  9805248.8 |
  9840808.8 |
  9876368.8 |
  9911928.7 |
  9947488.7 |####################
  9983048.7 |
  10018608.6 |
  (0 below, 1 above range)

nway (n=6, range 6658928.5-26615629.5 ns)
  6658928.5 |########################################
  7656763.5 |
  8654598.6 |
  9652433.7 |
  10650268.7 |
  11648103.8 |
  12645938.8 |########################################
  13643773.9 |########################################
  14641608.9 |
  15639444.0 |
  16637279.0 |
  17635114.1 |
  18632949.1 |########################################
  19630784.1 |
  20628619.2 |
  21626454.2 |
  22624289.3 |
  23622124.4 |
  24619959.4 |
  25617794.4 |########################################
  (0 below, 1 above range)

sequential (n=6, range 13908015.6-15201150.5 ns)
  13908015.6 |########################################
  13972672.3 |####################
  14037329.1 |####################
  14101985.8 |
  14166642.6 |
  14231299.3 |
  14295956.1 |
  14360612.8 |####################
  14425269.6 |
  14489926.3 |
  14554583.1 |
  14619239.8 |
  14683896.5 |
  14748553.3 |
  14813210.0 |
  14877866.8 |
  14942523.5 |
  15007180.3 |
  15071837.0 |
  15136493.8 |
  (0 below, 1 above range)

std_parallel (n=6, range 6727822.4-8317063.8 ns)
  6727822.4 |########################################
  6807284.5 |
  6886746.5 |##########
  6966208.6 |
  7045670.7 |
  7125132.8 |
  7204594.8 |
  7284056.9 |
  7363519.0 |
  7442981.0 |
  7522443.1 |
  7601905.2 |
  7681367.2 |
  7760829.3 |
  7840291.4 |
  7919753.5 |
  7999215.5 |
  8078677.6 |
  8158139.7 |
  8237601.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=727.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=747.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=686.6% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=766.8% of algo (FFI overhead may distort results)
- **nway**: CV=40.9% (high variance, measurements may be unstable)
- **nway**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **nway**: bridge=530.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=539.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=1112.6% of algo (FFI overhead may distort results)

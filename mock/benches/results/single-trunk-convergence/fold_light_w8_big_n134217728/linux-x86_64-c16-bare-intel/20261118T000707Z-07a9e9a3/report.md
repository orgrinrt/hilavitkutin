# fold_light, far above cache, W = 8

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway beats baseline by 33% (significant)

kernel_nway is -6.60 ms (33%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### head_tail_dyn is an outlier: 3.5x slower than the field

head_tail_dyn (46.30 ms) is 3.5x the fastest (13.09 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.57)

head_tail's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.5x the fastest

Fastest kernel_nway (13.09 ms) to slowest head_tail_dyn (46.30 ms): 3.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### head_tail_dyn is inconsistent: worst-20% is 4.0x its best-20%

head_tail_dyn's best 20% of batches run at 24.36 ms but its worst 20% at 96.37 ms (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 13088069.6 ns median (-35.3% vs baseline)
- 3 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 3.54x (fastest 13088069.6 ns, slowest 46301790.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 8 |
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
| head_tail | 20407032ns | 20242942ns | 19494439ns | 20011247ns | 21457005ns | base |
| head_tail_dyn | 55987104ns | 46310355ns | 24359137ns | 39600375ns | 96381182ns | +174.35% |
| kernel_nway | 14311283ns | 13092944ns | 12880691ns | 13032104ns | 16945347ns | -29.87% |
| kernel_seq | 30133709ns | 30419377ns | 27897287ns | 30359346ns | 30913465ns | +47.66% |
| nway | 16157541ns | 15652647ns | 12808856ns | 15529426ns | 18774057ns | -20.82% |
| sequential | 26625979ns | 26417914ns | 26171412ns | 26362674ns | 27248221ns | +30.47% |
| std_parallel | 14286625ns | 14239208ns | 13087066ns | 13862696ns | 15522299ns | -29.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 20398858ns | 19487852ns | 21449000ns | base | 6.580 |
| head_tail_dyn | 55978836ns | 24357100ns | 96367678ns | +174.42% | 2.398 |
| kernel_nway | 14304535ns | 12865724ns | 16939845ns | -29.88% | 9.383 |
| kernel_seq | 30129109ns | 27895918ns | 30906718ns | +47.70% | 4.455 |
| nway | 16155335ns | 12806800ns | 18771796ns | -20.80% | 8.308 |
| sequential | 26624211ns | 26169373ns | 27246583ns | +30.52% | 5.041 |
| std_parallel | 14283450ns | 13083124ns | 15519172ns | -29.98% | 9.397 |

## Performance model

- Peak throughput: **10.480 Gops/s** (nway; best 20% batches)
- Ops per call: 134217728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 6.632 | 63.3% |
| head_tail_dyn | 2.899 | 27.7% |
| kernel_nway | 10.255 | 97.9% |
| kernel_seq | 4.413 | 42.1% |
| nway | 8.576 | 81.8% |
| sequential | 5.081 | 48.5% |
| std_parallel | 9.428 | 90.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 20407032ns | 20407032ns | base |
| head_tail_dyn | 55987104ns | 55987104ns | +174.35% |
| kernel_nway | 14311283ns | 14311283ns | -29.87% |
| kernel_seq | 30133709ns | 30133709ns | +47.66% |
| nway | 16157541ns | 16157541ns | -20.82% |
| sequential | 26625979ns | 26625979ns | +30.47% |
| std_parallel | 14286625ns | 14286625ns | -29.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 20237643ns | base | --- | [19509932, 21449000] | --- | --- | --- | --- |
| head_tail_dyn | 46301790ns | +26010788.4ns (+128.5%) | [+4873434, +75855712]ns | [25267041, 96367678] | YES | 0.0313 | 0.0313 | 0 |
| kernel_nway | 13088070ns | -6599655.5ns (-32.6%) | [-7759739, -3923574]ns | [12885691, 16939845] | YES | 0.0313 | 0.0313 | 0 |
| kernel_seq | 30414133ns | +9550713.9ns (+47.2%) | [+8785221, +10854818]ns | [29066476, 30906718] | YES | 0.0313 | 0.0313 | 0 |
| nway | 15650472ns | -4253834.9ns (-21.0%) | [-6736563, -1740170]ns | [14043738, 18771796] | YES | 0.0313 | 0.0313 | 0 |
| sequential | 26415916ns | +6366013.6ns (+31.5%) | [+4884651, +7425394]ns | [26210135, 27246583] | YES | 0.0313 | 0.0313 | 0 |
| std_parallel | 14235922ns | -6274981.1ns (-31.0%) | [-7100528, -4970714]ns | [13095257, 15519172] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 19509932ns | +277.3% | -33.8% | +51.3% | -14.8% | +37.9% | -26.9% |
| 1 | 2 | 21449000ns | +129.9% | -27.9% | +42.7% | -23.7% | +22.8% | -28.0% |
| 1 | 3 | 20237643ns | +122.4% | -28.1% | +49.4% | -23.5% | +31.6% | -35.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.575 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.093 | ok |
| kernel_nway | -0.401 | moderate- |
| kernel_seq | -0.389 | moderate- |
| nway | -0.060 | ok |
| sequential | -0.512 | HIGH- (thermal bounce) |
| std_parallel | -0.505 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 29301669.5ns | 20398858.2ns | 143.6% | HIGH |
| head_tail_dyn | 50128710.7ns | 55978836.2ns | 89.5% | HIGH |
| kernel_nway | 16177935.2ns | 14304535.5ns | 113.1% | HIGH |
| kernel_seq | 30340642.9ns | 30129109.1ns | 100.7% | HIGH |
| nway | 14975749.0ns | 16155335.3ns | 92.7% | HIGH |
| sequential | 26522781.6ns | 26624211.3ns | 99.6% | HIGH |
| std_parallel | 13118404.6ns | 14283450.5ns | 91.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 19487852.0-21448999.6 ns)
  19487852.0 |########################################
  19585909.4 |
  19683966.8 |
  19782024.1 |
  19880081.5 |
  19978138.9 |
  20076196.3 |####################
  20174253.6 |
  20272311.0 |####################
  20370368.4 |
  20468425.8 |
  20566483.2 |
  20664540.5 |
  20762597.9 |
  20860655.3 |
  20958712.7 |
  21056770.0 |
  21154827.4 |
  21252884.8 |
  21350942.2 |####################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 24357100.4-96367678.2 ns)
  24357100.4 |########################################
  27957629.3 |
  31558158.2 |
  35158687.1 |
  38759216.0 |
  42359744.8 |
  45960273.7 |
  49560802.6 |
  53161331.5 |
  56761860.4 |
  60362389.3 |
  63962918.2 |#############
  67563447.1 |
  71163975.9 |#############
  74764504.8 |
  78365033.7 |
  81965562.6 |
  85566091.5 |
  89166620.4 |
  92767149.3 |
  (0 below, 1 above range)

kernel_nway (n=6, range 12865724.2-16939845.4 ns)
  12865724.2 |########################################
  13069430.3 |#############
  13273136.3 |
  13476842.4 |
  13680548.4 |
  13884254.5 |
  14087960.6 |
  14291666.6 |
  14495372.7 |
  14699078.8 |
  14902784.8 |
  15106490.9 |
  15310196.9 |
  15513903.0 |
  15717609.1 |#############
  15921315.1 |
  16125021.2 |
  16328727.3 |
  16532433.3 |
  16736139.4 |
  (0 below, 1 above range)

kernel_seq (n=6, range 27895918.1-30906717.9 ns)
  27895918.1 |####################
  28046458.1 |
  28196998.1 |
  28347538.1 |
  28498078.1 |
  28648618.0 |
  28799158.0 |
  28949698.0 |
  29100238.0 |
  29250778.0 |
  29401318.0 |
  29551858.0 |
  29702398.0 |
  29852937.9 |
  30003477.9 |
  30154017.9 |########################################
  30304557.9 |
  30455097.9 |####################
  30605637.9 |####################
  30756177.9 |
  (0 below, 1 above range)

nway (n=6, range 12806800.3-18771795.9 ns)
  12806800.3 |####################
  13105050.1 |
  13403299.9 |
  13701549.6 |
  13999799.4 |
  14298049.2 |
  14596299.0 |
  14894548.8 |
  15192798.5 |####################
  15491048.3 |########################################
  15789298.1 |
  16087547.9 |
  16385797.7 |
  16684047.4 |
  16982297.2 |
  17280547.0 |
  17578796.8 |####################
  17877046.6 |
  18175296.3 |
  18473546.1 |
  (0 below, 1 above range)

sequential (n=6, range 26169373.4-27246583.1 ns)
  26169373.4 |########################################
  26223233.9 |########################################
  26277094.4 |
  26330954.9 |########################################
  26384815.3 |
  26438675.8 |
  26492536.3 |########################################
  26546396.8 |
  26600257.3 |
  26654117.8 |
  26707978.3 |
  26761838.8 |
  26815699.2 |
  26869559.7 |
  26923420.2 |########################################
  26977280.7 |
  27031141.2 |
  27085001.7 |
  27138862.2 |
  27192722.7 |
  (0 below, 1 above range)

std_parallel (n=6, range 13083123.9-15519172.4 ns)
  13083123.9 |########################################
  13204926.3 |
  13326728.7 |
  13448531.2 |
  13570333.6 |
  13692136.0 |
  13813938.4 |
  13935740.9 |
  14057543.3 |
  14179345.7 |
  14301148.1 |
  14422950.5 |
  14544753.0 |
  14666555.4 |
  14788357.8 |
  14910160.2 |
  15031962.7 |
  15153765.1 |
  15275567.5 |#############
  15397369.9 |#############
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=106.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=62.3% (high variance, measurements may be unstable)
- **head_tail_dyn**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **head_tail_dyn**: bridge=76.3% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=118.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.7% of algo (FFI overhead may distort results)
- **nway**: bridge=98.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=91.9% of algo (FFI overhead may distort results)

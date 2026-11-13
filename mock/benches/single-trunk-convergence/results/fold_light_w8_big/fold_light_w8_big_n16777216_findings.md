# fold_light, far above cache, W = 8

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway beats baseline by 47% (significant)

kernel_nway is -1.25 ms (47%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### kernel_seq is an outlier: 4.0x slower than the field

kernel_seq (5.21 ms) is 4.0x the fastest (1.30 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.66)

kernel_seq's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.0x the fastest

Fastest nway (1.30 ms) to slowest kernel_seq (5.21 ms): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader nway vs stability leader kernel_nway (+9% speed for 2.3x steadier)

nway is fastest (1.30 ms, CV 25.3%); kernel_nway gives up 9.1% median for 2.3x lower variance (CV 11.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### sequential is inconsistent: worst-20% is 3.6x its best-20%

sequential's best 20% of batches run at 2.86 ms but its worst 20% at 10.17 ms (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: nway** at 1297751.0 ns median (-51.4% vs baseline)
- 3 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 4.01x (fastest 1297751.0 ns, slowest 5207789.0 ns)

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
| Counter frequency | 2100242360 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 2668555ns | 2671796ns | 1937067ns | 2471500ns | 3329882ns | base |
| head_tail_dyn | 5096020ns | 5024271ns | 3912106ns | 4821251ns | 6100132ns | +90.97% |
| kernel_nway | 1410376ns | 1418880ns | 1211216ns | 1361441ns | 1583358ns | -47.15% |
| kernel_seq | 5802806ns | 5211311ns | 3000772ns | 4571074ns | 9051422ns | +117.45% |
| nway | 1340130ns | 1299613ns | 951734ns | 1223858ns | 1708737ns | -49.78% |
| sequential | 6111507ns | 5186931ns | 2858230ns | 4486653ns | 10175425ns | +129.02% |
| std_parallel | 1683269ns | 1526454ns | 1010434ns | 1476034ns | 2330541ns | -36.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2611585ns | 1936207ns | 3164435ns | base | 6.424 |
| head_tail_dyn | 5091770ns | 3909353ns | 6094453ns | +94.97% | 3.295 |
| kernel_nway | 1407536ns | 1209471ns | 1580409ns | -46.10% | 11.920 |
| kernel_seq | 5798347ns | 2998820ns | 9044993ns | +122.02% | 2.893 |
| nway | 1338274ns | 950412ns | 1706366ns | -48.76% | 12.536 |
| sequential | 6108667ns | 2857037ns | 10171602ns | +133.91% | 2.746 |
| std_parallel | 1680422ns | 1009058ns | 2326061ns | -35.66% | 9.984 |

## Performance model

- Peak throughput: **17.653 Gops/s** (nway; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 6.288 | 35.6% |
| head_tail_dyn | 3.342 | 18.9% |
| kernel_nway | 11.849 | 67.1% |
| kernel_seq | 3.222 | 18.2% |
| nway | 12.928 | 73.2% |
| sequential | 3.236 | 18.3% |
| std_parallel | 11.009 | 62.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 2668555ns | 2668555ns | base |
| head_tail_dyn | 5096020ns | 5096020ns | +90.97% |
| kernel_nway | 1410376ns | 1410376ns | -47.15% |
| kernel_seq | 5802806ns | 5802806ns | +117.45% |
| nway | 1340130ns | 1340130ns | -49.78% |
| sequential | 6111507ns | 6111507ns | +129.02% |
| std_parallel | 1683269ns | 1683269ns | -36.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2667944ns | base | --- | [2002376, 3164435] | --- | --- | --- | --- |
| head_tail_dyn | 5019844ns | +2435783.1ns (+91.3%) | [+1759109, +3245663]ns | [4161012, 6094453] | YES | 0.0375 | 0.0313 | 0 |
| kernel_nway | 1415929ns | -1252014.8ns (-46.9%) | [-1747249, -612884]ns | [1226269, 1580409] | YES | 0.0375 | 0.0313 | 0 |
| kernel_seq | 5207789ns | +2797865.0ns (+104.9%) | [+881865, +5880558]ns | [3142260, 9044993] | YES | 0.0375 | 0.0313 | 0 |
| nway | 1297751ns | -1061525.1ns (-39.8%) | [-1962557, -795849]ns | [1010706, 1706366] | YES | 0.0375 | 0.0313 | 0 |
| sequential | 5183760ns | +2372842.4ns (+88.9%) | [+456675, +7661730]ns | [2970640, 10171602] | YES | 0.0375 | 0.0313 | 0 |
| std_parallel | 1523925ns | -746396.2ns (-28.0%) | [-1783828, -263263]ns | [1191281, 2326061] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 3064487ns | +95.6% | -52.5% | +122.6% | -64.0% | +67.8% | -58.2% |
| 1 | 2 | 2260395ns | +107.6% | -31.8% | +39.0% | -44.6% | +33.2% | -11.9% |
| 1 | 3 | 2509872ns | +82.8% | -51.1% | +196.1% | -33.8% | +305.3% | -29.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.364 | moderate- |
| head_tail_dyn | -0.129 | ok |
| kernel_nway | -0.340 | moderate- |
| kernel_seq | -0.655 | HIGH- (thermal bounce) |
| nway | -0.044 | ok |
| sequential | -0.355 | moderate- |
| std_parallel | -0.367 | moderate- |

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
| head_tail | 2735713.5ns | 2611584.7ns | 104.8% | HIGH |
| head_tail_dyn | 5083140.0ns | 5091769.6ns | 99.8% | HIGH |
| kernel_nway | 1554136.2ns | 1407535.6ns | 110.4% | HIGH |
| kernel_seq | 6235788.1ns | 5798347.3ns | 107.5% | HIGH |
| nway | 1493530.6ns | 1338274.4ns | 111.6% | HIGH |
| sequential | 6361089.4ns | 6108667.3ns | 104.1% | HIGH |
| std_parallel | 1855613.8ns | 1680422.3ns | 110.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 1936207.4-3164434.9 ns)
  1936207.4 |########################################
  1997618.8 |
  2059030.1 |########################################
  2120441.5 |
  2181852.9 |
  2243264.3 |
  2304675.6 |
  2366087.0 |
  2427498.4 |########################################
  2488909.8 |
  2550321.1 |
  2611732.5 |
  2673143.9 |
  2734555.3 |
  2795966.6 |
  2857378.0 |########################################
  2918789.4 |
  2980200.8 |
  3041612.1 |########################################
  3103023.5 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 3909352.7-6094452.7 ns)
  3909352.7 |########################################
  4018607.7 |
  4127862.7 |
  4237117.7 |
  4346372.7 |########################################
  4455627.7 |
  4564882.7 |
  4674137.7 |########################################
  4783392.7 |
  4892647.7 |
  5001902.7 |
  5111157.7 |
  5220412.7 |########################################
  5329667.7 |
  5438922.7 |########################################
  5548177.7 |
  5657432.7 |
  5766687.7 |
  5875942.7 |
  5985197.7 |
  (0 below, 1 above range)

kernel_nway (n=6, range 1209471.0-1580408.9 ns)
  1209471.0 |########################################
  1228017.9 |########################################
  1246564.8 |
  1265111.7 |
  1283658.6 |
  1302205.5 |########################################
  1320752.4 |
  1339299.3 |
  1357846.2 |
  1376393.1 |
  1394939.9 |
  1413486.8 |
  1432033.7 |
  1450580.6 |
  1469127.5 |
  1487674.4 |
  1506221.3 |########################################
  1524768.2 |
  1543315.1 |
  1561862.0 |########################################
  (0 below, 1 above range)

kernel_seq (n=6, range 2998819.9-9044992.6 ns)
  2998819.9 |########################################
  3301128.5 |
  3603437.2 |
  3905745.8 |
  4208054.4 |
  4510363.1 |####################
  4812671.7 |
  5114980.3 |
  5417289.0 |####################
  5719597.6 |
  6021906.2 |
  6324214.9 |
  6626523.5 |
  6928832.2 |
  7231140.8 |
  7533449.4 |
  7835758.1 |
  8138066.7 |
  8440375.3 |
  8742684.0 |####################
  (0 below, 1 above range)

nway (n=6, range 950412.5-1706366.4 ns)
  950412.5 |########################################
  988210.2 |
  1026007.9 |
  1063805.6 |########################################
  1101603.3 |
  1139401.0 |
  1177198.7 |
  1214996.3 |
  1252794.0 |########################################
  1290591.7 |
  1328389.4 |########################################
  1366187.1 |
  1403984.8 |########################################
  1441782.5 |
  1479580.2 |
  1517377.9 |
  1555175.6 |
  1592973.3 |
  1630771.0 |
  1668568.7 |
  (0 below, 1 above range)

sequential (n=6, range 2857036.8-10171601.9 ns)
  2857036.8 |########################################
  3222765.1 |
  3588493.3 |
  3954221.6 |
  4319949.8 |
  4685678.1 |
  5051406.3 |
  5417134.6 |
  5782862.8 |
  6148591.1 |
  6514319.3 |
  6880047.6 |#############
  7245775.8 |
  7611504.1 |
  7977232.3 |
  8342960.6 |
  8708688.8 |
  9074417.1 |
  9440145.3 |#############
  9805873.6 |
  (0 below, 1 above range)

std_parallel (n=6, range 1009058.2-2326061.2 ns)
  1009058.2 |########################################
  1074908.3 |
  1140758.5 |
  1206608.6 |
  1272458.8 |
  1338308.9 |########################################
  1404159.1 |
  1470009.2 |########################################
  1535859.4 |########################################
  1601709.6 |
  1667559.7 |
  1733409.9 |
  1799260.0 |
  1865110.1 |
  1930960.3 |
  1996810.5 |
  2062660.6 |
  2128510.8 |########################################
  2194360.9 |
  2260211.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=103.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=99.6% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=109.7% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=42.4% (high variance, measurements may be unstable)
- **kernel_seq**: worst_20/best_20 = 3.0x (possible bimodal distribution)
- **kernel_seq**: bridge=107.9% of algo (FFI overhead may distort results)
- **nway**: CV=24.6% (high variance, measurements may be unstable)
- **nway**: bridge=109.5% of algo (FFI overhead may distort results)
- **sequential**: CV=53.3% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **sequential**: bridge=114.2% of algo (FFI overhead may distort results)
- **std_parallel**: CV=29.6% (high variance, measurements may be unstable)
- **std_parallel**: bridge=106.9% of algo (FFI overhead may distort results)

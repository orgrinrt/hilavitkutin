# fold_light, far above cache, W = 1

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (kernel_seq, nway) are a dead heat (<1%)

kernel_seq (11.55 ms) and nway (11.61 ms) differ by 0.45%, inside the noise, even though the wider field spreads 5.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.66)

kernel_nway's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader kernel_seq vs stability leader head_tail_dyn (+1% speed for 1.4x steadier)

kernel_seq is fastest (11.55 ms, CV 4.5%); head_tail_dyn gives up 1.0% median for 1.4x lower variance (CV 3.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: kernel_seq** at 11553526.1 ns median (-4.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.05x (fastest 11553526.1 ns, slowest 12182854.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 1 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 12242616ns | 12094786ns | 11608333ns | 11972240ns | 12965323ns | base |
| head_tail_dyn | 11826696ns | 11675099ns | 11556615ns | 11638618ns | 12243854ns | -3.40% |
| kernel_nway | 12097736ns | 12100010ns | 11337198ns | 11981524ns | 12652323ns | -1.18% |
| kernel_seq | 11754964ns | 11555099ns | 11356167ns | 11491618ns | 12349380ns | -3.98% |
| nway | 11932707ns | 11607865ns | 11567896ns | 11594767ns | 12622021ns | -2.53% |
| sequential | 12194099ns | 12186797ns | 11329312ns | 11919701ns | 13038088ns | -0.40% |
| std_parallel | 11959547ns | 11880672ns | 11407427ns | 11739330ns | 12565932ns | -2.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 12240516ns | 11606448ns | 12962688ns | base | 10.965 |
| head_tail_dyn | 11824811ns | 11554250ns | 12241630ns | -3.40% | 11.351 |
| kernel_nway | 12095592ns | 11335542ns | 12649641ns | -1.18% | 11.096 |
| kernel_seq | 11752885ns | 11354062ns | 12346531ns | -3.98% | 11.420 |
| nway | 11930139ns | 11565198ns | 12619802ns | -2.54% | 11.250 |
| sequential | 12191587ns | 11327479ns | 13036240ns | -0.40% | 11.009 |
| std_parallel | 11957736ns | 11406177ns | 12563760ns | -2.31% | 11.224 |

## Performance model

- Peak throughput: **11.849 Gops/s** (sequential; best 20% batches)
- Ops per call: 134217728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 11.099 | 93.7% |
| head_tail_dyn | 11.498 | 97.0% |
| kernel_nway | 11.094 | 93.6% |
| kernel_seq | 11.617 | 98.0% |
| nway | 11.565 | 97.6% |
| sequential | 11.017 | 93.0% |
| std_parallel | 11.299 | 95.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 12242616ns | 12242616ns | base |
| head_tail_dyn | 11826696ns | 11826696ns | -3.40% |
| kernel_nway | 12097736ns | 12097736ns | -1.18% |
| kernel_seq | 11754964ns | 11754964ns | -3.98% |
| nway | 11932707ns | 11932707ns | -2.53% |
| sequential | 12194099ns | 12194099ns | -0.40% |
| std_parallel | 11959547ns | 11959547ns | -2.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 12092917ns | base | --- | [11665943, 12962688] | --- | --- | --- | --- |
| head_tail_dyn | 11673620ns | -389291.6ns (-3.2%) | [-779495, -78328]ns | [11559182, 12241630] | YES (adj: no) | 0.1875 | 0.0313 | 0 |
| kernel_nway | 12097938ns | no significant difference | [-1073865, +645469]ns | [11539198, 12649641] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 11553526ns | no significant difference | [-1268505, +163125]ns | [11358599, 12346531] | no | 0.4375 | 0.2188 | 0 |
| nway | 11605359ns | no significant difference | [-1185536, +515417]ns | [11565255, 12619802] | no | 0.4375 | 0.2188 | 0 |
| sequential | 12182854ns | no significant difference | [-779833, +943323]ns | [11355667, 13036240] | no | 0.8250 | 0.6875 | 0 |
| std_parallel | 11879057ns | no significant difference | [-1005464, +261203]ns | [11430391, 12563760] | no | 0.8250 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 11665943ns | -0.7% | +2.7% | -2.6% | +4.4% | -2.7% | -2.0% |
| 1 | 2 | 12464088ns | -5.4% | -0.9% | -0.9% | -3.4% | +0.7% | +0.8% |
| 1 | 3 | 12591516ns | -4.0% | -5.1% | -8.2% | -8.1% | +0.6% | -5.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.099 | ok |
| head_tail_dyn | -0.007 | ok |
| kernel_nway | -0.664 | HIGH- (thermal bounce) |
| kernel_seq | -0.644 | HIGH- (thermal bounce) |
| nway | -0.054 | ok |
| sequential | -0.125 | ok |
| std_parallel | -0.564 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 2/3, lost 1/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 2/3, lost 1/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 12333635.4ns | 12240515.6ns | 100.8% | HIGH |
| head_tail_dyn | 11932069.4ns | 11824810.8ns | 100.9% | HIGH |
| kernel_nway | 12143819.4ns | 12095592.0ns | 100.4% | HIGH |
| kernel_seq | 11862375.0ns | 11752885.4ns | 100.9% | HIGH |
| nway | 11927505.2ns | 11930138.9ns | 100.0% | HIGH |
| sequential | 12119019.1ns | 12191586.8ns | 99.4% | HIGH |
| std_parallel | 12089168.4ns | 11957736.1ns | 101.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 11606447.9-12962687.5 ns)
  11606447.9 |########################################
  11674259.9 |########################################
  11742071.9 |
  11809883.8 |
  11877695.8 |########################################
  11945507.8 |
  12013319.8 |
  12081131.8 |
  12148943.7 |
  12216755.7 |########################################
  12284567.7 |
  12352379.7 |
  12420191.7 |
  12488003.6 |
  12555815.6 |
  12623627.6 |########################################
  12691439.6 |
  12759251.6 |
  12827063.5 |
  12894875.5 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 11554250.0-12241630.2 ns)
  11554250.0 |########################################
  11588619.0 |####################
  11622988.0 |
  11657357.0 |
  11691726.0 |
  11726095.1 |####################
  11760464.1 |
  11794833.1 |
  11829202.1 |####################
  11863571.1 |
  11897940.1 |
  11932309.1 |
  11966678.1 |
  12001047.1 |
  12035416.1 |
  12069785.2 |
  12104154.2 |
  12138523.2 |
  12172892.2 |
  12207261.2 |
  (0 below, 1 above range)

kernel_nway (n=6, range 11335541.7-12649640.6 ns)
  11335541.7 |########################################
  11401246.6 |
  11466951.6 |
  11532656.5 |
  11598361.5 |
  11664066.4 |
  11729771.4 |########################################
  11795476.3 |
  11861181.3 |
  11926886.2 |
  11992591.1 |########################################
  12058296.1 |
  12124001.0 |########################################
  12189706.0 |
  12255410.9 |
  12321115.9 |
  12386820.8 |
  12452525.8 |
  12518230.7 |
  12583935.7 |########################################
  (0 below, 1 above range)

kernel_seq (n=6, range 11354062.5-12346531.2 ns)
  11354062.5 |########################################
  11403685.9 |
  11453309.4 |
  11502932.8 |####################
  11552556.2 |####################
  11602179.7 |
  11651803.1 |
  11701426.6 |
  11751050.0 |
  11800673.4 |####################
  11850296.9 |
  11899920.3 |
  11949543.8 |
  11999167.2 |
  12048790.6 |
  12098414.1 |
  12148037.5 |
  12197660.9 |
  12247284.4 |
  12296907.8 |
  (0 below, 1 above range)

nway (n=6, range 11565197.9-12619802.1 ns)
  11565197.9 |########################################
  11617928.1 |#############
  11670658.3 |
  11723388.5 |
  11776118.7 |
  11828848.9 |
  11881579.1 |
  11934309.4 |
  11987039.6 |
  12039769.8 |
  12092500.0 |
  12145230.2 |
  12197960.4 |
  12250690.6 |
  12303420.8 |
  12356151.0 |
  12408881.2 |#############
  12461611.4 |
  12514341.6 |
  12567071.8 |
  (0 below, 1 above range)

sequential (n=6, range 11327479.2-13036239.6 ns)
  11327479.2 |########################################
  11412917.2 |
  11498355.2 |
  11583793.3 |
  11669231.3 |
  11754669.3 |
  11840107.3 |
  11925545.3 |
  12010983.4 |####################
  12096421.4 |
  12181859.4 |
  12267297.4 |####################
  12352735.4 |
  12438173.5 |
  12523611.5 |
  12609049.5 |
  12694487.5 |
  12779925.5 |
  12865363.6 |
  12950801.6 |####################
  (0 below, 1 above range)

std_parallel (n=6, range 11406177.1-12563760.4 ns)
  11406177.1 |########################################
  11464056.3 |
  11521935.4 |
  11579814.6 |####################
  11637693.8 |
  11695572.9 |
  11753452.1 |
  11811331.3 |
  11869210.4 |
  11927089.6 |
  11984968.8 |
  12042847.9 |
  12100727.1 |
  12158606.3 |####################
  12216485.4 |####################
  12274364.6 |
  12332243.8 |
  12390122.9 |
  12448002.1 |
  12505881.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=101.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=99.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=99.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.9% of algo (FFI overhead may distort results)
- **nway**: bridge=100.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=100.7% of algo (FFI overhead may distort results)

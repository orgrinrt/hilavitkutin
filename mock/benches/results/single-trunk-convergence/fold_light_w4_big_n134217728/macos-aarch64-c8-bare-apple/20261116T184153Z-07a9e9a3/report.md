# fold_light, far above cache, W = 4

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential shows alternating (throttle bounce) (autocorr -0.65)

sequential's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: nway** at 12166744.8 ns median (-3.0% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.51x (fastest 12166744.8 ns, slowest 18318192.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 4 |
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
| head_tail | 12870455ns | 12553078ns | 10996875ns | 12297024ns | 14667391ns | base |
| head_tail_dyn | 18580377ns | 18322406ns | 17500927ns | 18105212ns | 19832849ns | +44.36% |
| kernel_nway | 12622816ns | 12716401ns | 11692667ns | 12703354ns | 12967083ns | -1.92% |
| kernel_seq | 15098957ns | 14970505ns | 14530167ns | 14831479ns | 15784568ns | +17.31% |
| nway | 12135748ns | 12169870ns | 11827750ns | 12118486ns | 12315641ns | -5.71% |
| sequential | 15262368ns | 15332380ns | 14201062ns | 15261451ns | 15794396ns | +18.58% |
| std_parallel | 12523531ns | 12540771ns | 12164094ns | 12446451ns | 12818870ns | -2.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 12866514ns | 10993385ns | 14663370ns | base | 10.432 |
| head_tail_dyn | 18576128ns | 17497052ns | 19828458ns | +44.38% | 7.225 |
| kernel_nway | 12618856ns | 11688969ns | 12962979ns | -1.92% | 10.636 |
| kernel_seq | 15095844ns | 14526531ns | 15781625ns | +17.33% | 8.891 |
| nway | 12131576ns | 11822573ns | 12311349ns | -5.71% | 11.064 |
| sequential | 15259847ns | 14198219ns | 15791885ns | +18.60% | 8.795 |
| std_parallel | 12520693ns | 12161521ns | 12815708ns | -2.69% | 10.720 |

## Performance model

- Peak throughput: **12.209 Gops/s** (head_tail; best 20% batches)
- Ops per call: 134217728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 10.696 | 87.6% |
| head_tail_dyn | 7.327 | 60.0% |
| kernel_nway | 10.558 | 86.5% |
| kernel_seq | 8.967 | 73.4% |
| nway | 11.032 | 90.4% |
| sequential | 8.755 | 71.7% |
| std_parallel | 10.704 | 87.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 12870455ns | 12870455ns | base |
| head_tail_dyn | 18580377ns | 18580377ns | +44.36% |
| kernel_nway | 12622816ns | 12622816ns | -1.92% |
| kernel_seq | 15098957ns | 15098957ns | +17.31% |
| nway | 12135748ns | 12135748ns | -5.71% |
| sequential | 15262368ns | 15262368ns | +18.58% |
| std_parallel | 12523531ns | 12523531ns | -2.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 12548635ns | base | --- | [11387536, 14663370] | --- | --- | --- | --- |
| head_tail_dyn | 18318193ns | +5892177.0ns (+47.0%) | [+3707411, +7529255]ns | [17581734, 19828458] | YES (adj: no) | 0.1875 | 0.0313 | 0 |
| kernel_nway | 12712875ns | no significant difference | [-1872948, +1319969]ns | [12180714, 12962979] | no | 0.8250 | 0.6875 | 0 |
| kernel_seq | 14967208ns | +2541328.0ns (+20.3%) | [+303839, +3842823]ns | [14538698, 15781625] | YES (adj: no) | 0.4375 | 0.2188 | 0 |
| nway | 12166745ns | no significant difference | [-2496625, +658984]ns | [11916635, 12311349] | no | 0.8250 | 0.6875 | 0 |
| sequential | 15330141ns | +2616937.5ns (+20.9%) | [+415047, +4148016]ns | [14657516, 15791885] | YES (adj: no) | 0.4375 | 0.2188 | 0 |
| std_parallel | 12538474ns | no significant difference | [-2291479, +1268432]ns | [12207896, 12815708] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 14663370ns | +25.3% | -12.8% | +2.1% | -17.0% | +6.3% | -15.6% |
| 1 | 2 | 12031865ns | +51.5% | +6.8% | +24.5% | +0.5% | +22.7% | +2.8% |
| 1 | 3 | 11904307ns | +60.7% | +2.6% | +28.9% | +2.0% | +29.7% | +7.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.144 | ok |
| head_tail_dyn | -0.258 | moderate- |
| kernel_nway | -0.218 | moderate- |
| kernel_seq | -0.150 | ok |
| nway | -0.597 | HIGH- (thermal bounce) |
| sequential | -0.647 | HIGH- (thermal bounce) |
| std_parallel | -0.164 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 12221227.4ns | 12866513.9ns | 95.0% | HIGH |
| head_tail_dyn | 18579666.7ns | 18576128.5ns | 100.0% | HIGH |
| kernel_nway | 12426868.1ns | 12618855.9ns | 98.5% | HIGH |
| kernel_seq | 15183979.2ns | 15095843.8ns | 100.6% | HIGH |
| nway | 12404892.4ns | 12131576.4ns | 102.3% | HIGH |
| sequential | 15094145.9ns | 15259847.2ns | 98.9% | HIGH |
| std_parallel | 12482793.4ns | 12520692.7ns | 99.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 10993385.4-14663369.8 ns)
  10993385.4 |########################################
  11176884.6 |
  11360383.8 |
  11543883.1 |
  11727382.3 |########################################
  11910881.5 |########################################
  12094380.7 |
  12277879.9 |
  12461379.2 |
  12644878.4 |
  12828377.6 |
  13011876.8 |########################################
  13195376.0 |
  13378875.3 |
  13562374.5 |########################################
  13745873.7 |
  13929372.9 |
  14112872.1 |
  14296371.4 |
  14479870.6 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 17497052.1-19828458.3 ns)
  17497052.1 |########################################
  17613622.4 |########################################
  17730192.7 |########################################
  17846763.0 |
  17963333.3 |
  18079903.6 |
  18196474.0 |
  18313044.3 |
  18429614.6 |
  18546184.9 |
  18662755.2 |
  18779325.5 |########################################
  18895895.8 |
  19012466.1 |
  19129036.4 |########################################
  19245606.8 |
  19362177.1 |
  19478747.4 |
  19595317.7 |
  19711888.0 |
  (0 below, 1 above range)

kernel_nway (n=6, range 11688968.8-12962979.2 ns)
  11688968.8 |####################
  11752669.3 |
  11816369.8 |
  11880070.4 |
  11943770.9 |
  12007471.4 |
  12071171.9 |
  12134872.4 |
  12198572.9 |
  12262273.5 |
  12325974.0 |
  12389674.5 |
  12453375.0 |
  12517075.5 |
  12580776.0 |
  12644476.6 |########################################
  12708177.1 |####################
  12771877.6 |
  12835578.1 |####################
  12899278.6 |
  (0 below, 1 above range)

kernel_seq (n=6, range 14526531.2-15781625.0 ns)
  14526531.2 |########################################
  14589285.9 |
  14652040.6 |
  14714795.3 |
  14777550.0 |
  14840304.6 |
  14903059.3 |####################
  14965814.0 |####################
  15028568.7 |
  15091323.4 |
  15154078.1 |
  15216832.8 |
  15279587.5 |
  15342342.2 |
  15405096.9 |####################
  15467851.6 |
  15530606.2 |
  15593360.9 |
  15656115.6 |
  15718870.3 |
  (0 below, 1 above range)

nway (n=6, range 11822572.9-12311349.0 ns)
  11822572.9 |####################
  11847011.7 |
  11871450.5 |
  11895889.3 |
  11920328.1 |
  11944766.9 |
  11969205.7 |
  11993644.5 |####################
  12018083.3 |
  12042522.1 |####################
  12066960.9 |
  12091399.8 |
  12115838.6 |
  12140277.4 |
  12164716.2 |
  12189155.0 |
  12213593.8 |
  12238032.6 |
  12262471.4 |########################################
  12286910.2 |
  (0 below, 1 above range)

sequential (n=6, range 14198218.8-15791885.4 ns)
  14198218.8 |####################
  14277902.1 |
  14357585.5 |
  14437268.8 |
  14516952.1 |
  14596635.5 |
  14676318.8 |
  14756002.1 |
  14835685.4 |
  14915368.8 |
  14995052.1 |
  15074735.4 |####################
  15154418.8 |
  15234102.1 |
  15313785.4 |########################################
  15393468.8 |
  15473152.1 |
  15552835.4 |
  15632518.7 |
  15712202.1 |####################
  (0 below, 1 above range)

std_parallel (n=6, range 12161520.8-12815708.3 ns)
  12161520.8 |########################################
  12194230.2 |
  12226939.6 |########################################
  12259648.9 |
  12292358.3 |
  12325067.7 |
  12357777.1 |
  12390486.4 |
  12423195.8 |
  12455905.2 |
  12488614.6 |########################################
  12521324.0 |
  12554033.3 |
  12586742.7 |########################################
  12619452.1 |
  12652161.5 |
  12684870.8 |
  12717580.2 |########################################
  12750289.6 |
  12782999.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=97.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=98.1% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=95.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=102.8% of algo (FFI overhead may distort results)
- **nway**: bridge=102.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=98.9% of algo (FFI overhead may distort results)

# fold_light, far above cache, W = 2

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (kernel_nway, head_tail) are a dead heat (<1%)

kernel_nway (19.64 ms) and head_tail (19.70 ms) differ by 0.33%, inside the noise, even though the wider field spreads 57.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### sequential shows alternating (throttle bounce) (autocorr -0.67)

sequential's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader kernel_nway vs stability leader head_tail (+0% speed for 83.2x steadier)

kernel_nway is fastest (19.64 ms, CV 43.8%); head_tail gives up 0.3% median for 83.2x lower variance (CV 0.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### nway is inconsistent: worst-20% is 2.7x its best-20%

nway's best 20% of batches run at 19.25 ms but its worst 20% at 52.90 ms (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 19638509.2 ns median (-0.3% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 1.57x (fastest 19638509.2 ns, slowest 30893908.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 2 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112743060 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 19687556ns | 19708851ns | 19482205ns | 19691585ns | 19784186ns | base |
| head_tail_dyn | 28967196ns | 26069768ns | 25668223ns | 25976283ns | 35103052ns | +47.13% |
| kernel_nway | 24735445ns | 19645037ns | 19200270ns | 19531615ns | 35308777ns | +25.64% |
| kernel_seq | 30162107ns | 30900483ns | 26789140ns | 30511063ns | 31325157ns | +53.20% |
| nway | 32719945ns | 25907542ns | 19252373ns | 23754566ns | 52901799ns | +66.20% |
| sequential | 27601045ns | 26101534ns | 26004450ns | 26074195ns | 30689617ns | +40.20% |
| std_parallel | 21441156ns | 20410418ns | 20137204ns | 20338649ns | 23746892ns | +8.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 19680137ns | 19471777ns | 19776233ns | base | 6.820 |
| head_tail_dyn | 28958957ns | 25665823ns | 35095896ns | +47.15% | 4.635 |
| kernel_nway | 24727605ns | 19197960ns | 35298539ns | +25.65% | 5.428 |
| kernel_seq | 30157406ns | 26787147ns | 31321715ns | +53.24% | 4.451 |
| nway | 32717602ns | 19250475ns | 52898447ns | +66.25% | 4.102 |
| sequential | 27599656ns | 26003220ns | 30688587ns | +40.24% | 4.863 |
| std_parallel | 21437643ns | 20135085ns | 23743949ns | +8.93% | 6.261 |

## Performance model

- Peak throughput: **6.991 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 134217728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 6.812 | 97.4% |
| head_tail_dyn | 5.150 | 73.7% |
| kernel_nway | 6.834 | 97.8% |
| kernel_seq | 4.344 | 62.1% |
| nway | 5.181 | 74.1% |
| sequential | 5.142 | 73.6% |
| std_parallel | 6.578 | 94.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 19687556ns | 19687556ns | base |
| head_tail_dyn | 28967196ns | 28967196ns | +47.13% |
| kernel_nway | 24735445ns | 24735445ns | +25.64% |
| kernel_seq | 30162107ns | 30162107ns | +53.20% |
| nway | 32719945ns | 32719945ns | +66.20% |
| sequential | 27601045ns | 27601045ns | +40.20% |
| std_parallel | 21441156ns | 21441156ns | +8.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 19703096ns | base | --- | [19561082, 19776233] | --- | --- | --- | --- |
| head_tail_dyn | 26060997ns | +6364413.1ns (+32.3%) | [+6059612, +15412436]ns | [25719978, 35095896] | YES | 0.0469 | 0.0313 | 0 |
| kernel_nway | 19638509ns | no significant difference | [-334952, +15522307]ns | [19245766, 35298539] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 30893909ns | +11173627.9ns (+56.7%) | [+8530620, +11727561]ns | [28256595, 31321715] | YES | 0.0469 | 0.0313 | 0 |
| nway | 25905619ns | no significant difference | [-390673, +33317729]ns | [19348739, 52898447] | no | 0.8250 | 0.6875 | 0 |
| sequential | 26099944ns | +6393605.6ns (+32.4%) | [+6257083, +11107869]ns | [26010436, 30688587] | YES | 0.0469 | 0.0313 | 0 |
| std_parallel | 20404782ns | +839098.0ns (+4.3%) | [+424788, +4008634]ns | [20164199, 23743949] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 19683460ns | +78.3% | -0.2% | +57.1% | -0.7% | +32.2% | +2.9% |
| 1 | 2 | 19580718ns | +32.0% | -1.7% | +49.3% | +170.2% | +56.7% | +4.9% |
| 1 | 3 | 19776233ns | +31.2% | +78.5% | +53.3% | +30.0% | +31.9% | +18.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.517 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.172 | ok |
| kernel_nway | -0.179 | ok |
| kernel_seq | -0.581 | HIGH- (thermal bounce) |
| nway | -0.646 | HIGH- (thermal bounce) |
| sequential | -0.667 | HIGH- (thermal bounce) |
| std_parallel | -0.125 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 20884826.0ns | 19680136.8ns | 106.1% | HIGH |
| head_tail_dyn | 27744230.6ns | 28958957.2ns | 95.8% | HIGH |
| kernel_nway | 30533685.6ns | 24727604.7ns | 123.5% | HIGH |
| kernel_seq | 30476204.0ns | 30157406.3ns | 101.1% | HIGH |
| nway | 31059503.9ns | 32717601.5ns | 94.9% | HIGH |
| sequential | 27766838.8ns | 27599655.9ns | 100.6% | HIGH |
| std_parallel | 20353412.9ns | 21437643.4ns | 94.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 19471776.6-19776232.8 ns)
  19471776.6 |########################################
  19486999.4 |
  19502222.2 |
  19517445.0 |
  19532667.8 |
  19547890.6 |
  19563113.4 |
  19578336.3 |
  19593559.1 |
  19608781.9 |
  19624004.7 |
  19639227.5 |########################################
  19654450.3 |
  19669673.1 |
  19684895.9 |########################################
  19700118.7 |
  19715341.5 |########################################
  19730564.3 |
  19745787.1 |
  19761009.9 |########################################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 25665823.1-35095895.9 ns)
  25665823.1 |########################################
  26137326.7 |#############
  26608830.4 |
  27080334.0 |
  27551837.7 |
  28023341.3 |
  28494844.9 |#############
  28966348.6 |
  29437852.2 |
  29909355.8 |
  30380859.5 |
  30852363.1 |
  31323866.8 |
  31795370.4 |
  32266874.0 |
  32738377.7 |
  33209881.3 |
  33681384.9 |
  34152888.6 |
  34624392.2 |
  (0 below, 1 above range)

kernel_nway (n=6, range 19197960.5-35298539.3 ns)
  19197960.5 |########################################
  20002989.4 |
  20808018.4 |
  21613047.3 |
  22418076.3 |
  23223105.2 |
  24028134.1 |
  24833163.1 |
  25638192.0 |
  26443221.0 |
  27248249.9 |##########
  28053278.8 |
  28858307.8 |
  29663336.7 |
  30468365.7 |
  31273394.6 |
  32078423.5 |
  32883452.5 |
  33688481.4 |
  34493510.4 |
  (0 below, 1 above range)

kernel_seq (n=6, range 26787147.3-31321715.1 ns)
  26787147.3 |#############
  27013875.7 |
  27240604.1 |
  27467332.5 |
  27694060.9 |
  27920789.2 |
  28147517.6 |
  28374246.0 |
  28600974.4 |
  28827702.8 |
  29054431.2 |
  29281159.6 |
  29507888.0 |#############
  29734616.4 |
  29961344.8 |
  30188073.2 |
  30414801.5 |
  30641529.9 |
  30868258.3 |########################################
  31094986.7 |
  (0 below, 1 above range)

nway (n=6, range 19250475.4-52898446.9 ns)
  19250475.4 |########################################
  20932874.0 |
  22615272.5 |
  24297671.1 |
  25980069.7 |
  27662468.3 |
  29344866.8 |
  31027265.4 |#############
  32709664.0 |
  34392062.6 |
  36074461.1 |
  37756859.7 |
  39439258.3 |
  41121656.8 |
  42804055.4 |
  44486454.0 |
  46168852.6 |
  47851251.1 |#############
  49533649.7 |
  51216048.3 |
  (0 below, 1 above range)

sequential (n=6, range 26003220.2-30688586.8 ns)
  26003220.2 |########################################
  26237488.5 |
  26471756.9 |
  26706025.2 |
  26940293.5 |
  27174561.8 |
  27408830.2 |
  27643098.5 |
  27877366.8 |
  28111635.2 |
  28345903.5 |
  28580171.8 |
  28814440.2 |
  29048708.5 |
  29282976.8 |
  29517245.1 |
  29751513.5 |
  29985781.8 |
  30220050.1 |
  30454318.5 |##########
  (0 below, 1 above range)

std_parallel (n=6, range 20135085.0-23743949.1 ns)
  20135085.0 |########################################
  20315528.2 |########################################
  20495971.4 |####################
  20676414.6 |
  20856857.8 |
  21037301.0 |
  21217744.2 |
  21398187.4 |
  21578630.6 |
  21759073.8 |
  21939517.0 |
  22119960.2 |
  22300403.4 |
  22480846.6 |
  22661289.8 |
  22841733.0 |
  23022176.2 |
  23202619.4 |
  23383062.6 |
  23563505.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=100.0% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=99.6% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=34.8% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=139.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.1% of algo (FFI overhead may distort results)
- **nway**: CV=46.1% (high variance, measurements may be unstable)
- **nway**: bridge=120.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=99.9% of algo (FFI overhead may distort results)

# append, far above cache, W = 8

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel beats baseline by 36% (significant)

std_parallel is -3.55 ms (36%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 2.2x slower than the field

sequential (14.00 ms) is 2.2x the fastest (6.38 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.67)

nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### nway is inconsistent: worst-20% is 1.8x its best-20%

nway's best 20% of batches run at 5.89 ms but its worst 20% at 10.78 ms (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_parallel** at 6379649.5 ns median (-36.1% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.19x (fastest 6379649.5 ns, slowest 14001387.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, far above cache, W = 8 |
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
| head_tail | 10418078ns | 9984606ns | 9890757ns | 9962470ns | 11365151ns | base |
| head_tail_dyn | 9680873ns | 9942462ns | 9084996ns | 9666398ns | 10000524ns | -7.08% |
| kernel_nway | 8282719ns | 8176425ns | 7285437ns | 7912115ns | 9337265ns | -20.50% |
| kernel_seq | 9583391ns | 9682934ns | 9137592ns | 9505377ns | 9923312ns | -8.01% |
| nway | 7896680ns | 6804822ns | 5892433ns | 6642342ns | 10780312ns | -24.20% |
| sequential | 14528716ns | 14003224ns | 13916695ns | 13976403ns | 15663194ns | +39.46% |
| std_parallel | 6802020ns | 6384468ns | 6261855ns | 6368854ns | 7721851ns | -34.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 10414119ns | 9887546ns | 11361108ns | base | 1.611 |
| head_tail_dyn | 9676841ns | 9080125ns | 9995281ns | -7.08% | 1.734 |
| kernel_nway | 8278310ns | 7280004ns | 9332809ns | -20.51% | 2.027 |
| kernel_seq | 9579531ns | 9134866ns | 9919407ns | -8.01% | 1.751 |
| nway | 7893839ns | 5889780ns | 10777306ns | -24.20% | 2.125 |
| sequential | 14526922ns | 13914968ns | 15661315ns | +39.49% | 1.155 |
| std_parallel | 6798172ns | 6257042ns | 7718941ns | -34.72% | 2.468 |

## Performance model

- Peak throughput: **2.849 Gops/s** (nway; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.681 | 59.0% |
| head_tail_dyn | 1.688 | 59.3% |
| kernel_nway | 2.053 | 72.1% |
| kernel_seq | 1.733 | 60.9% |
| nway | 2.466 | 86.6% |
| sequential | 1.198 | 42.1% |
| std_parallel | 2.630 | 92.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 10418078ns | 10418078ns | base |
| head_tail_dyn | 9680873ns | 9680873ns | -7.08% |
| kernel_nway | 8282719ns | 8282719ns | -20.50% |
| kernel_seq | 9583391ns | 9583391ns | -8.01% |
| nway | 7896680ns | 7896680ns | -24.20% |
| sequential | 14528716ns | 14528716ns | +39.46% |
| std_parallel | 6802020ns | 6802020ns | -34.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 9980656ns | base | --- | [9900594, 11361108] | --- | --- | --- | --- |
| head_tail_dyn | 9939451ns | no significant difference | [-1777288, +7120]ns | [9095792, 9995281] | no | 0.2188 | 0.2188 | 0 |
| kernel_nway | 8173100ns | -2453362.3ns (-24.6%) | [-2944817, -1009248]ns | [7329022, 9332809] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| kernel_seq | 9679013ns | no significant difference | [-1881022, +18813]ns | [9140173, 9919407] | no | 0.2188 | 0.2188 | 0 |
| nway | 6802088ns | no significant difference | [-5171418, +734508]ns | [6102124, 10777306] | no | 0.2188 | 0.2188 | 0 |
| sequential | 14001387ns | +4013226.2ns (+40.2%) | [+2576837, +5748345]ns | [13918064, 15661315] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| std_parallel | 6379649ns | -3546368.6ns (-35.5%) | [-5065181, -2236291]ns | [6295927, 7718941] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 9912970ns | -8.2% | -10.2% | -1.8% | -35.1% | +58.0% | -32.7% |
| 1 | 2 | 10042799ns | -0.8% | -24.7% | -9.0% | +7.3% | +39.0% | -26.4% |
| 1 | 3 | 11286589ns | -11.7% | -25.9% | -12.6% | -42.7% | +23.7% | -43.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.120 | ok |
| head_tail_dyn | -0.163 | ok |
| kernel_nway | -0.561 | HIGH- (thermal bounce) |
| kernel_seq | -0.649 | HIGH- (thermal bounce) |
| nway | -0.667 | HIGH- (thermal bounce) |
| sequential | -0.168 | ok |
| std_parallel | -0.602 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 2/3, lost 1/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 72774616.4ns | 10414119.3ns | 698.8% | HIGH |
| head_tail_dyn | 72564729.1ns | 9676841.0ns | 749.9% | HIGH |
| kernel_nway | 72981690.2ns | 8278310.0ns | 881.6% | HIGH |
| kernel_seq | 72138681.8ns | 9579531.0ns | 753.1% | HIGH |
| nway | 75526981.8ns | 7893839.4ns | 956.8% | HIGH |
| sequential | 76391933.6ns | 14526922.2ns | 525.9% | HIGH |
| std_parallel | 74278792.3ns | 6798172.3ns | 1092.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 9887545.9-11361108.2 ns)
  9887545.9 |########################################
  9961224.0 |#############
  10034902.1 |#############
  10108580.3 |
  10182258.4 |
  10255936.5 |
  10329614.6 |
  10403292.7 |
  10476970.8 |
  10550649.0 |
  10624327.1 |
  10698005.2 |
  10771683.3 |
  10845361.4 |
  10919039.5 |
  10992717.7 |
  11066395.8 |
  11140073.9 |
  11213752.0 |
  11287430.1 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 9080125.0-9995280.6 ns)
  9080125.0 |########################################
  9125882.8 |
  9171640.6 |
  9217398.3 |
  9263156.1 |
  9308913.9 |
  9354671.7 |
  9400429.4 |
  9446187.2 |
  9491945.0 |
  9537702.8 |
  9583460.6 |
  9629218.3 |
  9674976.1 |
  9720733.9 |
  9766491.7 |
  9812249.4 |
  9858007.2 |
  9903765.0 |####################
  9949522.8 |########################################
  (0 below, 1 above range)

kernel_nway (n=6, range 7280004.0-9332808.8 ns)
  7280004.0 |########################################
  7382644.2 |
  7485284.5 |
  7587924.7 |
  7690565.0 |####################
  7793205.2 |
  7895845.4 |
  7998485.7 |
  8101125.9 |
  8203766.1 |
  8306406.4 |
  8409046.6 |
  8511686.8 |####################
  8614327.1 |
  8716967.3 |
  8819607.6 |
  8922247.8 |
  9024888.0 |
  9127528.3 |####################
  9230168.5 |
  (0 below, 1 above range)

kernel_seq (n=6, range 9134865.8-9919407.1 ns)
  9134865.8 |########################################
  9174092.9 |
  9213319.9 |
  9252547.0 |
  9291774.1 |
  9331001.1 |
  9370228.2 |
  9409455.3 |
  9448682.3 |
  9487909.4 |
  9527136.5 |####################
  9566363.5 |
  9605590.6 |
  9644817.7 |
  9684044.7 |
  9723271.8 |
  9762498.9 |
  9801725.9 |####################
  9840953.0 |
  9880180.1 |####################
  (0 below, 1 above range)

nway (n=6, range 5889779.8-10777306.2 ns)
  5889779.8 |########################################
  6134156.1 |########################################
  6378532.4 |########################################
  6622908.8 |
  6867285.1 |########################################
  7111661.4 |
  7356037.7 |
  7600414.0 |
  7844790.4 |
  8089166.7 |
  8333543.0 |
  8577919.3 |
  8822295.6 |
  9066672.0 |
  9311048.3 |########################################
  9555424.6 |
  9799800.9 |
  10044177.2 |
  10288553.6 |
  10532929.9 |
  (0 below, 1 above range)

sequential (n=6, range 13914967.6-15661315.4 ns)
  13914967.6 |########################################
  14002285.0 |
  14089602.4 |
  14176919.8 |
  14264237.2 |
  14351554.6 |
  14438872.0 |
  14526189.3 |
  14613506.7 |
  14700824.1 |
  14788141.5 |
  14875458.9 |
  14962776.3 |
  15050093.7 |
  15137411.1 |
  15224728.5 |
  15312045.9 |
  15399363.3 |
  15486680.7 |
  15573998.1 |##########
  (0 below, 1 above range)

std_parallel (n=6, range 6257042.5-7718940.7 ns)
  6257042.5 |####################
  6330137.4 |########################################
  6403232.3 |####################
  6476327.2 |
  6549422.1 |
  6622517.0 |
  6695611.9 |
  6768706.9 |
  6841801.8 |
  6914896.7 |####################
  6987991.6 |
  7061086.5 |
  7134181.4 |
  7207276.3 |
  7280371.2 |
  7353466.1 |
  7426561.0 |
  7499655.9 |
  7572750.8 |
  7645845.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=729.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=731.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=890.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=740.5% of algo (FFI overhead may distort results)
- **nway**: CV=27.8% (high variance, measurements may be unstable)
- **nway**: bridge=1098.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=538.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=1160.5% of algo (FFI overhead may distort results)

# append, far above cache, W = 8

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 21% faster than the next best (std_parallel)

kernel_nway (4.95 ms) leads std_parallel (5.99 ms) by 21%, a clear separation rather than a photo finish. CV 12.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_nway beats baseline by 67% (significant)

kernel_nway is -10.69 ms (67%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 5.1x slower than the field

sequential (25.47 ms) is 5.1x the fastest (4.95 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.65)

sequential's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.1x the fastest

Fastest kernel_nway (4.95 ms) to slowest sequential (25.47 ms): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_nway** at 4954596.4 ns median (-68.9% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.14x (fastest 4954596.4 ns, slowest 25469776.6 ns)

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
| Counter frequency | 2100242360 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 15821323ns | 15924091ns | 14242521ns | 15777455ns | 16676526ns | base |
| head_tail_dyn | 15569049ns | 15398304ns | 14459545ns | 15278298ns | 16559928ns | -1.59% |
| kernel_nway | 5128429ns | 4960504ns | 4444174ns | 4830070ns | 5918096ns | -67.59% |
| kernel_seq | 15022512ns | 14942150ns | 14035499ns | 14856212ns | 15765470ns | -5.05% |
| nway | 8438601ns | 8466912ns | 7691604ns | 8258725ns | 9081913ns | -46.66% |
| sequential | 26696361ns | 25474611ns | 23400347ns | 25129821ns | 30694177ns | +68.74% |
| std_parallel | 6322306ns | 5995950ns | 5750528ns | 5987944ns | 7109738ns | -60.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 15815020ns | 14235051ns | 16669825ns | base | 1.061 |
| head_tail_dyn | 15562881ns | 14455191ns | 16552908ns | -1.59% | 1.078 |
| kernel_nway | 5122476ns | 4436950ns | 5913459ns | -67.61% | 3.275 |
| kernel_seq | 15015323ns | 14026840ns | 15756998ns | -5.06% | 1.117 |
| nway | 8432732ns | 7686583ns | 9074525ns | -46.68% | 1.990 |
| sequential | 26691528ns | 23395715ns | 30689339ns | +68.77% | 0.629 |
| std_parallel | 6316463ns | 5743500ns | 7103938ns | -60.06% | 2.656 |

## Performance model

- Peak throughput: **3.781 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.054 | 27.9% |
| head_tail_dyn | 1.090 | 28.8% |
| kernel_nway | 3.386 | 89.6% |
| kernel_seq | 1.123 | 29.7% |
| nway | 1.983 | 52.4% |
| sequential | 0.659 | 17.4% |
| std_parallel | 2.801 | 74.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 15821323ns | 15821323ns | base |
| head_tail_dyn | 15569049ns | 15569049ns | -1.59% |
| kernel_nway | 5128429ns | 5128429ns | -67.59% |
| kernel_seq | 15022512ns | 15022512ns | -5.05% |
| nway | 8438601ns | 8438601ns | -46.66% |
| sequential | 26696361ns | 26696361ns | +68.74% |
| std_parallel | 6322306ns | 6322306ns | -60.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 15918015ns | base | --- | [14857221, 16669825] | --- | --- | --- | --- |
| head_tail_dyn | 15392657ns | no significant difference | [-1926747, +1280073]ns | [14743078, 16552908] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 4954596ns | -10689910.6ns (-67.2%) | [-11890786, -9496936]ns | [4499372, 5913459] | YES | 0.0469 | 0.0313 | 0 |
| kernel_seq | 14937394ns | no significant difference | [-2088705, +366766]ns | [14351578, 15756998] | no | 0.8250 | 0.6875 | 0 |
| nway | 8461667ns | -7364119.7ns (-46.3%) | [-8852645, -5930098]ns | [7762005, 9074525] | YES | 0.0469 | 0.0313 | 0 |
| sequential | 25469777ns | +10040777.9ns (+63.1%) | [+8569230, +14019514]ns | [23915467, 30689339] | YES | 0.0469 | 0.0313 | 0 |
| std_parallel | 5989914ns | -9360443.9ns (-58.8%) | [-10800063, -8335165]ns | [5855536, 7103938] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 15122249ns | +1.8% | -66.2% | -5.1% | -45.7% | +66.4% | -55.1% |
| 1 | 2 | 16669825ns | -11.6% | -71.3% | -8.8% | -49.4% | +84.1% | -64.8% |
| 1 | 3 | 15652986ns | +5.7% | -65.0% | -1.1% | -44.8% | +54.7% | -59.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.591 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.400 | moderate- |
| kernel_nway | -0.489 | moderate- |
| kernel_seq | -0.054 | ok |
| nway | -0.000 | ok |
| sequential | -0.655 | HIGH- (thermal bounce) |
| std_parallel | -0.473 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 102773346.6ns | 15815020.2ns | 649.8% | HIGH |
| head_tail_dyn | 101734312.8ns | 15562881.0ns | 653.7% | HIGH |
| kernel_nway | 92057384.6ns | 5122476.1ns | 1797.1% | HIGH |
| kernel_seq | 101812473.6ns | 15015323.0ns | 678.1% | HIGH |
| nway | 93381791.3ns | 8432732.4ns | 1107.4% | HIGH |
| sequential | 112099688.9ns | 26691527.5ns | 420.0% | HIGH |
| std_parallel | 90963479.0ns | 6316462.9ns | 1440.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 14235050.8-16669824.6 ns)
  14235050.8 |########################################
  14356789.5 |
  14478528.2 |
  14600266.9 |
  14722005.6 |
  14843744.2 |
  14965482.9 |
  15087221.6 |
  15208960.3 |
  15330699.0 |
  15452437.7 |########################################
  15574176.4 |
  15695915.1 |
  15817653.8 |########################################
  15939392.5 |########################################
  16061131.2 |########################################
  16182869.8 |
  16304608.5 |
  16426347.2 |
  16548085.9 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 14455190.8-16552907.9 ns)
  14455190.8 |########################################
  14560076.7 |
  14664962.5 |
  14769848.4 |
  14874734.2 |
  14979620.1 |########################################
  15084505.9 |########################################
  15189391.8 |
  15294277.6 |
  15399163.5 |
  15504049.4 |########################################
  15608935.2 |
  15713821.1 |
  15818706.9 |
  15923592.8 |
  16028478.6 |########################################
  16133364.5 |
  16238250.3 |
  16343136.2 |
  16448022.0 |
  (0 below, 1 above range)

kernel_nway (n=6, range 4436949.5-5913459.4 ns)
  4436949.5 |########################################
  4510775.0 |########################################
  4584600.5 |
  4658426.0 |
  4732251.5 |########################################
  4806077.0 |
  4879902.5 |
  4953728.0 |
  5027553.5 |
  5101379.0 |########################################
  5175204.5 |
  5249029.9 |
  5322855.4 |
  5396680.9 |
  5470506.4 |
  5544331.9 |
  5618157.4 |########################################
  5691982.9 |
  5765808.4 |
  5839633.9 |
  (0 below, 1 above range)

kernel_seq (n=6, range 14026839.7-15756997.5 ns)
  14026839.7 |########################################
  14113347.6 |
  14199855.5 |
  14286363.4 |
  14372871.3 |
  14459379.1 |
  14545887.0 |
  14632394.9 |########################################
  14718902.8 |
  14805410.7 |########################################
  14891918.6 |
  14978426.5 |########################################
  15064934.4 |
  15151442.3 |
  15237950.2 |
  15324458.1 |########################################
  15410965.9 |
  15497473.8 |
  15583981.7 |
  15670489.6 |
  (0 below, 1 above range)

nway (n=6, range 7686583.4-9074525.2 ns)
  7686583.4 |########################################
  7755980.5 |
  7825377.6 |########################################
  7894774.7 |
  7964171.8 |
  8033568.8 |
  8102965.9 |
  8172363.0 |########################################
  8241760.1 |
  8311157.2 |
  8380554.3 |
  8449951.4 |
  8519348.5 |
  8588745.6 |
  8658142.7 |
  8727539.8 |########################################
  8796936.8 |
  8866333.9 |
  8935731.0 |
  9005128.1 |########################################
  (0 below, 1 above range)

sequential (n=6, range 23395714.9-30689338.9 ns)
  23395714.9 |########################################
  23760396.1 |
  24125077.3 |########################################
  24489758.5 |
  24854439.7 |########################################
  25219120.9 |
  25583802.1 |########################################
  25948483.3 |
  26313164.5 |
  26677845.7 |
  27042526.9 |
  27407208.1 |########################################
  27771889.3 |
  28136570.5 |
  28501251.7 |
  28865932.9 |
  29230614.1 |
  29595295.3 |
  29959976.5 |
  30324657.7 |
  (0 below, 1 above range)

std_parallel (n=6, range 5743500.3-7103937.7 ns)
  5743500.3 |#############
  5811522.2 |
  5879544.0 |
  5947565.9 |########################################
  6015587.8 |
  6083609.7 |
  6151631.5 |
  6219653.4 |
  6287675.3 |
  6355697.1 |
  6423719.0 |
  6491740.9 |
  6559762.7 |#############
  6627784.6 |
  6695806.5 |
  6763828.4 |
  6831850.2 |
  6899872.1 |
  6967894.0 |
  7035915.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=646.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=657.4% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1844.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=684.6% of algo (FFI overhead may distort results)
- **nway**: bridge=1102.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=438.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=1502.2% of algo (FFI overhead may distort results)

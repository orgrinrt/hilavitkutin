# fold_light, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 16% faster than the next best (head_tail)

sequential (174.60 us) leads head_tail (202.17 us) by 16%, a clear separation rather than a photo finish. CV 11.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.67)

kernel_seq's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### head_tail_dyn is inconsistent: worst-20% is 1.8x its best-20%

head_tail_dyn's best 20% of batches run at 198.03 us but its worst 20% at 347.92 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 174598.7 ns median (-13.6% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.81x (fastest 174598.7 ns, slowest 316595.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 1 |
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
| head_tail | 229514ns | 202584ns | 175634ns | 196954ns | 305294ns | base |
| head_tail_dyn | 294785ns | 317445ns | 199012ns | 290523ns | 349065ns | +28.44% |
| kernel_nway | 276239ns | 302822ns | 200807ns | 265715ns | 329740ns | +20.36% |
| kernel_seq | 244468ns | 205245ns | 194773ns | 205770ns | 327362ns | +6.52% |
| nway | 257983ns | 240244ns | 209841ns | 235880ns | 315208ns | +12.40% |
| sequential | 184178ns | 175440ns | 165734ns | 174551ns | 207840ns | -19.75% |
| std_parallel | 286101ns | 280779ns | 244875ns | 272670ns | 326862ns | +24.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 228586ns | 175135ns | 303658ns | base | 4.587 |
| head_tail_dyn | 293709ns | 198031ns | 347922ns | +28.49% | 3.570 |
| kernel_nway | 275384ns | 200415ns | 328584ns | +20.47% | 3.808 |
| kernel_seq | 243016ns | 194172ns | 325076ns | +6.31% | 4.315 |
| nway | 257269ns | 209493ns | 314302ns | +12.55% | 4.076 |
| sequential | 183161ns | 165026ns | 206369ns | -19.87% | 5.725 |
| std_parallel | 285310ns | 243974ns | 326119ns | +24.82% | 3.675 |

## Performance model

- Peak throughput: **6.354 Gops/s** (sequential; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 5.187 | 81.6% |
| head_tail_dyn | 3.312 | 52.1% |
| kernel_nway | 3.470 | 54.6% |
| kernel_seq | 5.130 | 80.7% |
| nway | 4.380 | 68.9% |
| sequential | 6.006 | 94.5% |
| std_parallel | 3.748 | 59.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 229514ns | 229514ns | base |
| head_tail_dyn | 294785ns | 294785ns | +28.44% |
| kernel_nway | 276239ns | 276239ns | +20.36% |
| kernel_seq | 244468ns | 244468ns | +6.52% |
| nway | 257983ns | 257983ns | +12.40% |
| sequential | 184178ns | 184178ns | -19.75% |
| std_parallel | 286101ns | 286101ns | +24.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 202172ns | base | --- | [186969, 298333] | --- | --- | --- | --- |
| head_tail_dyn | 316596ns | +30756.8ns (+15.2%) | [+20824, +119876]ns | [252421, 334867] | YES | 0.0039 | 0.0013 | 0 |
| kernel_nway | 302165ns | +24045.1ns (+11.9%) | [+9803, +63373]ns | [210168, 311521] | YES | 0.0039 | 0.0013 | 0 |
| kernel_seq | 204409ns | +13611.2ns (+6.7%) | [+3542, +25503]ns | [200196, 294207] | YES | 0.0151 | 0.0075 | 0 |
| nway | 239416ns | no significant difference | [-36165, +99633]ns | [220298, 288574] | no | 0.2379 | 0.2379 | 0 |
| sequential | 174599ns | -30920.1ns (-15.3%) | [-63581, -5881]ns | [170485, 188984] | YES | 0.0463 | 0.0309 | 0 |
| std_parallel | 279770ns | no significant difference | [-4917, +91366]ns | [258884, 293007] | no | 0.2379 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 182978ns | +14.4% | +11.2% | +7.1% | +66.3% | -1.3% | +41.5% |
| 1 | 2 | 300074ns | +5.5% | +2.4% | +10.4% | -29.2% | -39.4% | -5.3% |
| 1 | 3 | 202172ns | +72.5% | +58.6% | +0.5% | +14.3% | -17.0% | +44.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.651 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.059 | ok |
| kernel_nway | -0.111 | ok |
| kernel_seq | -0.665 | HIGH- (thermal bounce) |
| nway | -0.289 | moderate- |
| sequential | -0.215 | moderate- |
| std_parallel | -0.057 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 227425.6ns | 228586.4ns | 99.5% | HIGH |
| head_tail_dyn | 298412.2ns | 293708.9ns | 101.6% | HIGH |
| kernel_nway | 277372.3ns | 275384.0ns | 100.7% | HIGH |
| kernel_seq | 234393.9ns | 243016.0ns | 96.5% | HIGH |
| nway | 263817.7ns | 257268.9ns | 102.5% | HIGH |
| sequential | 184434.3ns | 183161.1ns | 100.7% | HIGH |
| std_parallel | 278851.7ns | 285310.1ns | 97.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 175135.1-303657.5 ns)
  175135.1 |
  181561.2 |##############################
  187987.3 |####################
  194413.4 |##########
  200839.6 |##############################
  207265.7 |##########
  213691.8 |
  220117.9 |
  226544.1 |
  232970.2 |
  239396.3 |
  245822.4 |
  252248.6 |
  258674.7 |
  265100.8 |
  271526.9 |
  277953.1 |
  284379.2 |
  290805.3 |
  297231.4 |########################################
  (2 below, 2 above range)

head_tail_dyn (n=18, range 198030.8-347922.4 ns)
  198030.8 |#############
  205525.4 |
  213019.9 |#############
  220514.5 |
  228009.1 |
  235503.7 |
  242998.3 |#############
  250492.9 |#############
  257987.4 |
  265482.0 |
  272976.6 |
  280471.2 |
  287965.8 |
  295460.4 |#############
  302954.9 |##########################
  310449.5 |
  317944.1 |#############
  325438.7 |########################################
  332933.3 |
  340427.8 |##########################
  (2 below, 3 above range)

kernel_nway (n=18, range 200414.8-328583.6 ns)
  200414.8 |########################################
  206823.3 |
  213231.7 |##########
  219640.2 |
  226048.6 |
  232457.0 |
  238865.5 |
  245273.9 |##########
  251682.4 |
  258090.8 |
  264499.2 |
  270907.7 |
  277316.1 |
  283724.6 |
  290133.0 |##########
  296541.4 |####################
  302949.9 |##########
  309358.3 |########################################
  315766.8 |
  322175.2 |
  (1 below, 3 above range)

kernel_seq (n=18, range 194172.4-325075.8 ns)
  194172.4 |########################################
  200717.6 |################################
  207262.8 |########
  213807.9 |########
  220353.1 |
  226898.3 |
  233443.4 |
  239988.6 |
  246533.8 |
  253079.0 |
  259624.1 |########
  266169.3 |
  272714.5 |
  279259.6 |
  285804.8 |
  292350.0 |
  298895.1 |
  305440.3 |
  311985.5 |
  318530.6 |########
  (1 below, 4 above range)

nway (n=18, range 209493.3-314301.5 ns)
  209493.3 |########################################
  214733.7 |#############
  219974.2 |##########################
  225214.6 |#############
  230455.0 |#############
  235695.4 |
  240935.8 |#############
  246176.2 |
  251416.6 |
  256657.0 |
  261897.4 |#############
  267137.8 |#############
  272378.2 |
  277618.7 |
  282859.1 |
  288099.5 |##########################
  293339.9 |#############
  298580.3 |
  303820.7 |
  309061.1 |#############
  (1 below, 2 above range)

sequential (n=18, range 165026.0-206369.4 ns)
  165026.0 |########################################
  167093.2 |
  169160.4 |#############
  171227.5 |##########################
  173294.7 |########################################
  175361.9 |#############
  177429.0 |
  179496.2 |
  181563.4 |
  183630.5 |
  185697.7 |#############
  187764.9 |##########################
  189832.1 |
  191899.2 |
  193966.4 |
  196033.6 |##########################
  198100.7 |
  200167.9 |
  202235.1 |
  204302.2 |
  (1 below, 2 above range)

std_parallel (n=18, range 243973.6-326119.0 ns)
  243973.6 |#############
  248080.8 |#############
  252188.1 |#############
  256295.4 |##########################
  260402.6 |
  264509.9 |#############
  268617.2 |
  272724.5 |#############
  276831.7 |#############
  280939.0 |#############
  285046.3 |##########################
  289153.5 |#############
  293260.8 |########################################
  297368.1 |
  301475.3 |
  305582.6 |
  309689.9 |
  313797.2 |
  317904.4 |
  322011.7 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=23.7% (high variance, measurements may be unstable)
- **head_tail**: bridge=98.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=102.0% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=24.9% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=101.1% of algo (FFI overhead may distort results)
- **nway**: bridge=105.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=98.8% of algo (FFI overhead may distort results)

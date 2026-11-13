# fold_light, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail dominates: 21% faster than the next best (kernel_nway)

head_tail (133.10 us) leads kernel_nway (160.51 us) by 21%, a clear separation rather than a photo finish. CV 27.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### head_tail_dyn is an outlier: 2.4x slower than the field

head_tail_dyn (325.47 us) is 2.4x the fastest (133.10 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_parallel shows alternating (throttle bounce) (autocorr -0.63)

std_parallel's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (133.10 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### kernel_nway is inconsistent: worst-20% is 1.9x its best-20%

kernel_nway's best 20% of batches run at 113.54 us but its worst 20% at 216.93 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 133104.9 ns median
- 3 variants significantly slower than baseline
- Spread: 2.45x (fastest 133104.9 ns, slowest 325474.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 2 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100342298 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 156159ns | 133881ns | 125558ns | 132155ns | 207466ns | base |
| head_tail_dyn | 348012ns | 326822ns | 307424ns | 324344ns | 403807ns | +122.86% |
| kernel_nway | 164245ns | 161219ns | 114151ns | 145077ns | 218044ns | +5.18% |
| kernel_seq | 329465ns | 293314ns | 243456ns | 290129ns | 431474ns | +110.98% |
| nway | 167932ns | 166425ns | 125692ns | 154850ns | 208675ns | +7.54% |
| sequential | 227426ns | 251045ns | 162496ns | 223127ns | 266338ns | +45.64% |
| std_parallel | 203335ns | 208786ns | 145992ns | 191219ns | 250179ns | +30.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 155590ns | 125139ns | 206871ns | base | 6.739 |
| head_tail_dyn | 346698ns | 305950ns | 402250ns | +122.83% | 3.024 |
| kernel_nway | 163477ns | 113539ns | 216933ns | +5.07% | 6.414 |
| kernel_seq | 328088ns | 242444ns | 429727ns | +110.87% | 3.196 |
| nway | 167540ns | 125338ns | 208275ns | +7.68% | 6.259 |
| sequential | 225955ns | 161487ns | 264910ns | +45.22% | 4.641 |
| std_parallel | 202582ns | 145541ns | 249431ns | +30.20% | 5.176 |

## Performance model

- Peak throughput: **9.235 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 7.878 | 85.3% |
| head_tail_dyn | 3.222 | 34.9% |
| kernel_nway | 6.533 | 70.7% |
| kernel_seq | 3.593 | 38.9% |
| nway | 6.316 | 68.4% |
| sequential | 4.199 | 45.5% |
| std_parallel | 5.048 | 54.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 156159ns | 156159ns | base |
| head_tail_dyn | 348012ns | 348012ns | +122.86% |
| kernel_nway | 164245ns | 164245ns | +5.18% |
| kernel_seq | 329465ns | 329465ns | +110.98% |
| nway | 167932ns | 167932ns | +7.54% |
| sequential | 227426ns | 227426ns | +45.64% |
| std_parallel | 203335ns | 203335ns | +30.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 133105ns | base | --- | [128546, 202264] | --- | --- | --- | --- |
| head_tail_dyn | 325474ns | +190418.5ns (+143.1%) | [+160143, +206333]ns | [317869, 340798] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 160506ns | no significant difference | [-79894, +45733]ns | [124214, 177761] | no | 0.2855 | 0.2379 | 0 |
| kernel_seq | 291877ns | +153485.7ns (+115.3%) | [+117042, +161844]ns | [279756, 325074] | YES | 0.0000 | 0.0000 | 0 |
| nway | 166017ns | no significant difference | [-15477, +22749]ns | [140751, 175720] | no | 0.8145 | 0.8145 | 0 |
| sequential | 249712ns | no significant difference | [-33678, +130815]ns | [170540, 258537] | no | 0.2855 | 0.2379 | 0 |
| std_parallel | 207708ns | +30921.2ns (+23.2%) | [+16670, +85432]ns | [157776, 231685] | YES | 0.0026 | 0.0013 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 128152ns | +146.7% | +25.2% | +122.0% | +9.8% | +106.0% | +80.8% |
| 1 | 2 | 130338ns | +156.6% | +45.9% | +109.1% | +21.9% | +91.6% | +15.8% |
| 1 | 3 | 205937ns | +56.2% | -43.8% | +58.9% | -14.0% | -20.1% | +3.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.153 | ok |
| head_tail_dyn | -0.611 | HIGH- (thermal bounce) |
| kernel_nway | -0.429 | moderate- |
| kernel_seq | -0.299 | moderate- |
| nway | -0.000 | ok |
| sequential | -0.097 | ok |
| std_parallel | -0.634 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 156441.3ns | 155590.0ns | 100.5% | HIGH |
| head_tail_dyn | 337706.9ns | 346698.1ns | 97.4% | HIGH |
| kernel_nway | 164698.2ns | 163476.6ns | 100.7% | HIGH |
| kernel_seq | 320974.9ns | 328087.7ns | 97.8% | HIGH |
| nway | 156406.4ns | 167540.1ns | 93.4% | HIGH |
| sequential | 231215.7ns | 225954.9ns | 102.3% | HIGH |
| std_parallel | 197347.3ns | 202581.7ns | 97.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 125139.4-206871.1 ns)
  125139.4 |########################################
  129226.0 |########################
  133312.5 |########################
  137399.1 |
  141485.7 |
  145572.3 |
  149658.9 |
  153745.5 |
  157832.1 |
  161918.7 |
  166005.2 |
  170091.8 |
  174178.4 |
  178265.0 |
  182351.6 |
  186438.2 |
  190524.8 |
  194611.4 |
  198697.9 |################
  202784.5 |########
  (1 below, 3 above range)

head_tail_dyn (n=18, range 305949.9-402250.2 ns)
  305949.9 |########################################
  310764.9 |
  315579.9 |##########################
  320394.9 |########################################
  325209.9 |#############
  330024.9 |#############
  334840.0 |#############
  339655.0 |########################################
  344470.0 |##########################
  349285.0 |
  354100.0 |
  358915.0 |
  363730.0 |
  368545.1 |
  373360.1 |
  378175.1 |
  382990.1 |
  387805.1 |
  392620.1 |
  397435.1 |
  (1 below, 1 above range)

kernel_nway (n=18, range 113539.1-216933.3 ns)
  113539.1 |########################################
  118708.8 |####################
  123878.5 |####################
  129048.2 |####################
  134217.9 |####################
  139387.6 |
  144557.3 |
  149727.0 |
  154896.8 |####################
  160066.5 |####################
  165236.2 |
  170405.9 |########################################
  175575.6 |########################################
  180745.3 |####################
  185915.0 |
  191084.7 |####################
  196254.5 |
  201424.2 |
  206593.9 |
  211763.6 |
  (2 below, 2 above range)

kernel_seq (n=18, range 242443.5-429727.3 ns)
  242443.5 |####################
  251807.7 |
  261171.9 |##########
  270536.1 |##########
  279900.3 |########################################
  289264.5 |####################
  298628.7 |
  307992.9 |
  317357.0 |####################
  326721.2 |####################
  336085.4 |##########
  345449.6 |
  354813.8 |
  364178.0 |##########
  373542.2 |
  382906.4 |
  392270.6 |
  401634.7 |
  410998.9 |
  420363.1 |
  (1 below, 1 above range)

nway (n=18, range 125338.4-208274.8 ns)
  125338.4 |##########################
  129485.3 |#############
  133632.1 |
  137778.9 |#############
  141925.7 |#############
  146072.5 |#############
  150219.4 |#############
  154366.2 |
  158513.0 |
  162659.8 |#############
  166806.6 |##########################
  170953.5 |#############
  175100.3 |########################################
  179247.1 |
  183393.9 |#############
  187540.7 |#############
  191687.6 |
  195834.4 |
  199981.2 |
  204128.0 |
  (1 below, 1 above range)

sequential (n=18, range 161487.0-264910.5 ns)
  161487.0 |####################
  166658.2 |##########
  171829.4 |##########
  177000.5 |
  182171.7 |
  187342.9 |
  192514.1 |
  197685.2 |
  202856.4 |
  208027.6 |
  213198.7 |
  218369.9 |
  223541.1 |
  228712.3 |
  233883.4 |####################
  239054.6 |
  244225.8 |##########
  249397.0 |##########
  254568.1 |########################################
  259739.3 |##########
  (2 below, 3 above range)

std_parallel (n=18, range 145541.4-249430.8 ns)
  145541.4 |####################
  150735.9 |####################
  155930.3 |########################################
  161124.8 |
  166319.3 |
  171513.8 |
  176708.2 |
  181902.7 |
  187097.2 |
  192291.6 |####################
  197486.1 |####################
  202680.6 |####################
  207875.0 |####################
  213069.5 |####################
  218264.0 |####################
  223458.5 |
  228652.9 |####################
  233847.4 |####################
  239041.9 |########################################
  244236.3 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=23.5% (high variance, measurements may be unstable)
- **head_tail**: bridge=98.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=24.7% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=100.3% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=30.7% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=97.6% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=43.0% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=102.1% of algo (FFI overhead may distort results)
- **nway**: CV=28.6% (high variance, measurements may be unstable)
- **nway**: bridge=96.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.6% of algo (FFI overhead may distort results)
- **std_parallel**: CV=21.1% (high variance, measurements may be unstable)
- **std_parallel**: bridge=96.9% of algo (FFI overhead may distort results)

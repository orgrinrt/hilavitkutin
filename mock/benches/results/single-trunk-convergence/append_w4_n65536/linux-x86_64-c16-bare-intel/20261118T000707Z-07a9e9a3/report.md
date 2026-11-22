# append, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (kernel_seq, head_tail_dyn) are a dead heat (<1%)

kernel_seq (34.56 us) and head_tail_dyn (34.90 us) differ by 0.98%, inside the noise, even though the wider field spreads 99.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### nway shows alternating (throttle bounce) (autocorr -0.67)

nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### kernel_nway is inconsistent: worst-20% is 11.0x its best-20%

kernel_nway's best 20% of batches run at 14.65 us but its worst 20% at 161.11 us (11.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 34557.8 ns median (-12.3% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.99x (fastest 34557.8 ns, slowest 68942.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 4 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 50880ns | 39661ns | 28808ns | 34477ns | 86521ns | base |
| head_tail_dyn | 42720ns | 35068ns | 23776ns | 32297ns | 67827ns | -16.04% |
| kernel_nway | 81271ns | 44323ns | 14897ns | 48357ns | 163829ns | +59.73% |
| kernel_seq | 45559ns | 34784ns | 23786ns | 32325ns | 76296ns | -10.46% |
| nway | 131794ns | 69232ns | 42075ns | 69983ns | 269369ns | +159.03% |
| sequential | 57596ns | 57149ns | 57004ns | 57225ns | 58449ns | +13.20% |
| std_parallel | 85213ns | 55813ns | 38646ns | 54834ns | 154066ns | +67.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 50284ns | 28605ns | 85154ns | base | 1.303 |
| head_tail_dyn | 42219ns | 23576ns | 66736ns | -16.04% | 1.552 |
| kernel_nway | 80009ns | 14648ns | 161113ns | +59.12% | 0.819 |
| kernel_seq | 45002ns | 23623ns | 75016ns | -10.50% | 1.456 |
| nway | 130919ns | 41897ns | 267379ns | +160.36% | 0.501 |
| sequential | 57405ns | 56803ns | 58252ns | +14.16% | 1.142 |
| std_parallel | 84490ns | 38428ns | 152735ns | +68.03% | 0.776 |

## Performance model

- Peak throughput: **4.474 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.663 | 37.2% |
| head_tail_dyn | 1.878 | 42.0% |
| kernel_nway | 1.488 | 33.3% |
| kernel_seq | 1.896 | 42.4% |
| nway | 0.951 | 21.2% |
| sequential | 1.151 | 25.7% |
| std_parallel | 1.187 | 26.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 50880ns | 50880ns | base |
| head_tail_dyn | 42720ns | 42720ns | -16.04% |
| kernel_nway | 81271ns | 81271ns | +59.73% |
| kernel_seq | 45559ns | 45559ns | -10.46% |
| nway | 131794ns | 131794ns | +159.03% |
| sequential | 57596ns | 57596ns | +13.20% |
| std_parallel | 85213ns | 85213ns | +67.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 39419ns | base | --- | [29326, 45508] | --- | --- | --- | --- |
| head_tail_dyn | 34895ns | no significant difference | [-11796, +8444]ns | [27828, 41663] | no | 0.2855 | 0.2379 | 0 |
| kernel_nway | 44053ns | no significant difference | [-11724, +39066]ns | [25675, 122198] | no | 0.8145 | 0.8145 | 0 |
| kernel_seq | 34558ns | -1136.3ns (-2.9%) | [-5196, -329]ns | [28459, 44533] | YES | 0.0463 | 0.0309 | 0 |
| nway | 68943ns | +15970.8ns (+40.5%) | [+7503, +117340]ns | [43243, 188230] | YES | 0.0009 | 0.0001 | 0 |
| sequential | 56963ns | +17477.7ns (+44.3%) | [+12356, +28336]ns | [56861, 58152] | YES | 0.0463 | 0.0309 | 0 |
| std_parallel | 55196ns | +17192.8ns (+43.6%) | [+4807, +39293]ns | [52175, 72746] | YES | 0.0463 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 42036ns | -30.7% | +104.0% | -4.1% | +3.7% | +35.3% | +148.6% |
| 1 | 2 | 29326ns | +41.2% | +0.8% | -16.5% | +773.4% | +98.5% | +106.2% |
| 1 | 3 | 29892ns | -20.4% | +140.1% | -3.7% | +40.7% | +90.4% | +33.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.190 | ok |
| head_tail_dyn | -0.609 | HIGH- (thermal bounce) |
| kernel_nway | -0.629 | HIGH- (thermal bounce) |
| kernel_seq | -0.336 | moderate- |
| nway | -0.667 | HIGH- (thermal bounce) |
| sequential | -0.666 | HIGH- (thermal bounce) |
| std_parallel | -0.028 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 423664.6ns | 50283.7ns | 842.5% | HIGH |
| head_tail_dyn | 356885.2ns | 42218.9ns | 845.3% | HIGH |
| kernel_nway | 759540.1ns | 80009.0ns | 949.3% | HIGH |
| kernel_seq | 376128.2ns | 45002.2ns | 835.8% | HIGH |
| nway | 705790.3ns | 130918.6ns | 539.1% | HIGH |
| sequential | 296760.9ns | 57405.1ns | 517.0% | HIGH |
| std_parallel | 651156.7ns | 84489.8ns | 770.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 28604.9-85154.2 ns)
  28604.9 |########################################
  31432.4 |
  34259.9 |
  37087.3 |#####
  39914.8 |#################
  42742.3 |#####
  45569.7 |#####
  48397.2 |
  51224.7 |
  54052.1 |
  56879.6 |
  59707.0 |
  62534.5 |
  65362.0 |
  68189.4 |
  71016.9 |
  73844.4 |#####
  76671.8 |
  79499.3 |
  82326.8 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 23576.1-66736.1 ns)
  23576.1 |####################
  25734.1 |##########
  27892.1 |########################################
  30050.1 |
  32208.1 |
  34366.1 |
  36524.1 |
  38682.1 |####################
  40840.1 |##############################
  42998.1 |##########
  45156.1 |
  47314.1 |
  49472.1 |
  51630.1 |##########
  53788.1 |
  55946.1 |
  58104.1 |
  60262.1 |
  62420.1 |
  64578.1 |
  (2 below, 2 above range)

kernel_nway (n=18, range 14648.1-161112.7 ns)
  14648.1 |########################################
  21971.4 |
  29294.6 |####################
  36617.8 |####################
  43941.1 |##########
  51264.3 |
  58587.5 |
  65910.7 |
  73234.0 |
  80557.2 |
  87880.4 |
  95203.7 |####################
  102526.9 |
  109850.1 |##########
  117173.4 |
  124496.6 |
  131819.8 |##########
  139143.0 |##########
  146466.3 |##########
  153789.5 |
  (1 below, 2 above range)

kernel_seq (n=18, range 23623.2-75016.4 ns)
  23623.2 |##############################
  26192.9 |##############################
  28762.5 |####################
  31332.2 |
  33901.8 |
  36471.5 |
  39041.1 |########################################
  41610.8 |
  44180.5 |
  46750.1 |##########
  49319.8 |##########
  51889.4 |
  54459.1 |
  57028.8 |
  59598.4 |
  62168.1 |
  64737.7 |
  67307.4 |##########
  69877.0 |
  72446.7 |
  (1 below, 2 above range)

nway (n=18, range 41897.1-267378.7 ns)
  41897.1 |########################################
  53171.1 |
  64445.2 |###########
  75719.3 |
  86993.4 |
  98267.5 |#####
  109541.6 |
  120815.6 |
  132089.7 |
  143363.8 |
  154637.9 |
  165912.0 |#####
  177186.0 |#####
  188460.1 |#####
  199734.2 |
  211008.3 |
  222282.4 |
  233556.5 |
  244830.5 |#####
  256104.6 |#####
  (1 below, 2 above range)

sequential (n=18, range 56802.9-58251.8 ns)
  56802.9 |########################################
  56875.3 |########################################
  56947.8 |########################################
  57020.2 |
  57092.7 |
  57165.1 |
  57237.6 |
  57310.0 |
  57382.5 |
  57454.9 |
  57527.4 |
  57599.8 |
  57672.2 |
  57744.7 |
  57817.1 |
  57889.6 |
  57962.0 |
  58034.5 |#############
  58106.9 |#############
  58179.4 |########################################
  (2 below, 2 above range)

std_parallel (n=18, range 38427.7-152734.8 ns)
  38427.7 |########################
  44143.1 |
  49858.4 |########################################
  55573.8 |################
  61289.2 |########
  67004.5 |########
  72719.9 |########
  78435.2 |
  84150.6 |
  89865.9 |########
  95581.3 |
  101296.6 |
  107012.0 |
  112727.3 |########
  118442.7 |
  124158.1 |
  129873.4 |
  135588.8 |
  141304.1 |
  147019.5 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=65.8% (high variance, measurements may be unstable)
- **head_tail**: bridge=926.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=62.5% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=849.4% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=83.5% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 11.0x (possible bimodal distribution)
- **kernel_nway**: bridge=1345.7% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=64.8% (high variance, measurements may be unstable)
- **kernel_seq**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **kernel_seq**: bridge=1040.4% of algo (FFI overhead may distort results)
- **nway**: CV=83.2% (high variance, measurements may be unstable)
- **nway**: worst_20/best_20 = 6.4x (possible bimodal distribution)
- **nway**: bridge=586.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=520.5% of algo (FFI overhead may distort results)
- **std_parallel**: CV=94.8% (high variance, measurements may be unstable)
- **std_parallel**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **std_parallel**: bridge=748.7% of algo (FFI overhead may distort results)

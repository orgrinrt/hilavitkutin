# fold_light, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 62% faster than the next best (head_tail)

sequential (20.98 us) leads head_tail (33.93 us) by 62%, a clear separation rather than a photo finish. CV 20.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 38% (significant)

sequential is -12.95 us (38%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 8.5x slower than the field

std_parallel (178.45 us) is 8.5x the fastest (20.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_parallel shows alternating (throttle bounce) (autocorr -0.66)

std_parallel's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 8.5x the fastest

Fastest sequential (20.98 us) to slowest std_parallel (178.45 us): 8.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### head_tail is inconsistent: worst-20% is 3.6x its best-20%

head_tail's best 20% of batches run at 28.81 us but its worst 20% at 103.70 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 20984.3 ns median (-38.1% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 8.50x (fastest 20984.3 ns, slowest 178454.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 8 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100242360 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 56150ns | 34383ns | 29209ns | 32870ns | 104540ns | base |
| head_tail_dyn | 51253ns | 43357ns | 34565ns | 41506ns | 74219ns | -8.72% |
| kernel_nway | 70068ns | 75401ns | 36023ns | 68937ns | 88787ns | +24.79% |
| kernel_seq | 40363ns | 42286ns | 29628ns | 38587ns | 48395ns | -28.11% |
| nway | 87856ns | 86984ns | 74943ns | 84316ns | 99623ns | +56.47% |
| sequential | 21095ns | 21322ns | 15011ns | 19991ns | 25794ns | -62.43% |
| std_parallel | 225273ns | 179298ns | 145879ns | 170262ns | 347486ns | +301.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 55534ns | 28808ns | 103702ns | base | 1.180 |
| head_tail_dyn | 50675ns | 33971ns | 73574ns | -8.75% | 1.293 |
| kernel_nway | 69196ns | 35421ns | 87654ns | +24.60% | 0.947 |
| kernel_seq | 39852ns | 29053ns | 47948ns | -28.24% | 1.644 |
| nway | 87332ns | 74495ns | 99055ns | +57.26% | 0.750 |
| sequential | 20753ns | 14739ns | 25378ns | -62.63% | 3.158 |
| std_parallel | 224584ns | 145461ns | 346611ns | +304.41% | 0.292 |

## Performance model

- Peak throughput: **4.446 Gops/s** (sequential; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.932 | 43.4% |
| head_tail_dyn | 1.526 | 34.3% |
| kernel_nway | 0.882 | 19.8% |
| kernel_seq | 1.565 | 35.2% |
| nway | 0.758 | 17.0% |
| sequential | 3.123 | 70.2% |
| std_parallel | 0.367 | 8.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 56150ns | 56150ns | base |
| head_tail_dyn | 51253ns | 51253ns | -8.72% |
| kernel_nway | 70068ns | 70068ns | +24.79% |
| kernel_seq | 40363ns | 40363ns | -28.11% |
| nway | 87856ns | 87856ns | +56.47% |
| sequential | 21095ns | 21095ns | -62.43% |
| std_parallel | 225273ns | 225273ns | +301.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 33926ns | base | --- | [30355, 35708] | --- | --- | --- | --- |
| head_tail_dyn | 42932ns | +10361.8ns (+30.5%) | [+5154, +13088]ns | [39082, 45524] | YES | 0.0371 | 0.0309 | 0 |
| kernel_nway | 74277ns | +39051.6ns (+115.1%) | [+18130, +48861]ns | [58895, 82275] | YES | 0.0020 | 0.0013 | 0 |
| kernel_seq | 41869ns | no significant difference | [-2362, +12364]ns | [29938, 45168] | no | 0.8145 | 0.8145 | 0 |
| nway | 86494ns | +51204.8ns (+150.9%) | [+46375, +56546]ns | [78662, 92008] | YES | 0.0003 | 0.0001 | 0 |
| sequential | 20984ns | -12954.6ns (-38.2%) | [-16454, -9583]ns | [16520, 24038] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 178454ns | +134155.7ns (+395.4%) | [+121600, +170608]ns | [155721, 220799] | YES | 0.0003 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 35708ns | -1.6% | +121.3% | +23.9% | +129.1% | -41.2% | +399.8% |
| 1 | 2 | 33926ns | +58.4% | +26.0% | +32.4% | +135.9% | -55.8% | +1014.4% |
| 1 | 3 | 31642ns | +37.4% | +180.3% | -6.8% | +190.8% | -20.6% | +386.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.003 | ok |
| head_tail_dyn | -0.533 | HIGH- (thermal bounce) |
| kernel_nway | -0.640 | HIGH- (thermal bounce) |
| kernel_seq | -0.189 | ok |
| nway | -0.250 | moderate- |
| sequential | -0.556 | HIGH- (thermal bounce) |
| std_parallel | -0.660 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 35510.7ns | 55533.6ns | 63.9% | HIGH |
| head_tail_dyn | 38929.8ns | 50675.5ns | 76.8% | HIGH |
| kernel_nway | 70656.9ns | 69196.4ns | 102.1% | HIGH |
| kernel_seq | 40341.7ns | 39851.9ns | 101.2% | HIGH |
| nway | 89979.2ns | 87332.1ns | 103.0% | HIGH |
| sequential | 21349.7ns | 20752.8ns | 102.9% | HIGH |
| std_parallel | 246705.8ns | 224584.2ns | 109.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 28808.0-103702.4 ns)
  28808.0 |##################################
  32552.8 |########################################
  36297.5 |#####
  40042.2 |
  43786.9 |
  47531.6 |
  51276.3 |#####
  55021.1 |
  58765.8 |
  62510.5 |
  66255.2 |
  69999.9 |
  73744.7 |
  77489.4 |
  81234.1 |
  84978.8 |
  88723.5 |
  92468.2 |
  96213.0 |
  99957.7 |
  (2 below, 1 above range)

head_tail_dyn (n=18, range 33970.5-73573.6 ns)
  33970.5 |##############################
  35950.7 |
  37930.8 |####################
  39911.0 |####################
  41891.1 |########################################
  43871.3 |####################
  45851.5 |
  47831.6 |
  49811.8 |##########
  51791.9 |
  53772.1 |
  55752.2 |
  57732.4 |
  59712.5 |
  61692.7 |##########
  63672.8 |
  65653.0 |
  67633.1 |
  69613.3 |
  71593.4 |
  (1 below, 2 above range)

kernel_nway (n=18, range 35420.6-87653.6 ns)
  35420.6 |##########################
  38032.2 |
  40643.9 |
  43255.5 |
  45867.2 |
  48478.8 |#############
  51090.5 |
  53702.1 |#############
  56313.8 |
  58925.4 |
  61537.1 |#############
  64148.7 |#############
  66760.4 |
  69372.0 |
  71983.7 |########################################
  74595.3 |
  77207.0 |#############
  79818.6 |##########################
  82430.3 |#############
  85041.9 |#############
  (1 below, 3 above range)

kernel_seq (n=18, range 29053.2-47948.1 ns)
  29053.2 |########################################
  29998.0 |#############
  30942.7 |
  31887.5 |
  32832.2 |
  33777.0 |
  34721.7 |
  35666.5 |
  36611.2 |
  37555.9 |
  38500.7 |
  39445.4 |
  40390.2 |##########################
  41334.9 |##########################
  42279.7 |
  43224.4 |##########################
  44169.2 |#############
  45113.9 |##########################
  46058.7 |
  47003.4 |#############
  (2 below, 2 above range)

nway (n=18, range 74494.8-99055.1 ns)
  74494.8 |####################
  75722.8 |####################
  76950.9 |########################################
  78178.9 |
  79406.9 |####################
  80634.9 |####################
  81862.9 |
  83090.9 |
  84318.9 |
  85547.0 |########################################
  86775.0 |####################
  88003.0 |####################
  89231.0 |
  90459.0 |########################################
  91687.0 |####################
  92915.0 |
  94143.1 |####################
  95371.1 |####################
  96599.1 |
  97827.1 |
  (1 below, 2 above range)

sequential (n=18, range 14738.8-25377.6 ns)
  14738.8 |##########################
  15270.8 |
  15802.7 |
  16334.7 |##########################
  16866.6 |
  17398.5 |
  17930.5 |
  18462.4 |
  18994.4 |
  19526.3 |
  20058.2 |#############
  20590.2 |########################################
  21122.1 |
  21654.1 |
  22186.0 |#############
  22717.9 |#############
  23249.9 |#############
  23781.8 |
  24313.8 |
  24845.7 |#############
  (2 below, 4 above range)

std_parallel (n=18, range 145460.9-346610.9 ns)
  145460.9 |########################################
  155518.4 |##############################
  165575.9 |
  175633.4 |####################
  185690.9 |##########
  195748.4 |####################
  205805.9 |
  215863.4 |
  225920.9 |
  235978.4 |##########
  246035.9 |
  256093.4 |
  266150.9 |
  276208.4 |
  286265.9 |
  296323.4 |
  306380.9 |
  316438.4 |
  326495.9 |
  336553.4 |
  (1 below, 4 above range)

```

## Diagnostics

- **head_tail**: CV=161.7% (high variance, measurements may be unstable)
- **head_tail**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **head_tail**: bridge=97.0% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=54.6% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=88.9% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=27.4% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=106.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.1% of algo (FFI overhead may distort results)
- **nway**: bridge=103.4% of algo (FFI overhead may distort results)
- **sequential**: CV=20.4% (high variance, measurements may be unstable)
- **sequential**: bridge=100.7% of algo (FFI overhead may distort results)
- **std_parallel**: CV=46.2% (high variance, measurements may be unstable)
- **std_parallel**: bridge=90.4% of algo (FFI overhead may distort results)

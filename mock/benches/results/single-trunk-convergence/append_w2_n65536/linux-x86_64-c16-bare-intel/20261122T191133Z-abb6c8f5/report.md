# append, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 33% faster than the next best (kernel_seq)

kernel_nway (26.86 us) leads kernel_seq (35.80 us) by 33%, a clear separation rather than a photo finish. CV 136.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### nway is an outlier: 2.5x slower than the field

nway (68.47 us) is 2.5x the fastest (26.86 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway is fastest but the noisiest (CV 136.8%)

kernel_nway wins on median (26.86 us) yet has the highest variance (CV 136.8%), while sequential is the steadiest (CV 0.4%, 56.75 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.53)

kernel_nway's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### head_tail_dyn is inconsistent: worst-20% is 4.3x its best-20%

head_tail_dyn's best 20% of batches run at 23.51 us but its worst 20% at 101.61 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 26859.2 ns median (-29.2% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.55x (fastest 26859.2 ns, slowest 68473.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 2 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112781387 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 50781ns | 38126ns | 23676ns | 32740ns | 91393ns | base |
| head_tail_dyn | 58874ns | 41917ns | 23673ns | 40954ns | 103355ns | +15.94% |
| kernel_nway | 47668ns | 27113ns | 23005ns | 26325ns | 92015ns | -6.13% |
| kernel_seq | 49406ns | 36045ns | 23632ns | 32976ns | 86937ns | -2.71% |
| nway | 92311ns | 68645ns | 47548ns | 61607ns | 160749ns | +81.78% |
| sequential | 56983ns | 56918ns | 56711ns | 56892ns | 57255ns | +12.21% |
| std_parallel | 65182ns | 56674ns | 37825ns | 51825ns | 98898ns | +28.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 50168ns | 23462ns | 89944ns | base | 1.306 |
| head_tail_dyn | 58118ns | 23509ns | 101611ns | +15.85% | 1.128 |
| kernel_nway | 46970ns | 22730ns | 90442ns | -6.37% | 1.395 |
| kernel_seq | 48825ns | 23468ns | 85595ns | -2.68% | 1.342 |
| nway | 91899ns | 47342ns | 160026ns | +83.18% | 0.713 |
| sequential | 56803ns | 56529ns | 57067ns | +13.23% | 1.154 |
| std_parallel | 64496ns | 37607ns | 97469ns | +28.56% | 1.016 |

## Performance model

- Peak throughput: **2.883 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.728 | 59.9% |
| head_tail_dyn | 1.573 | 54.6% |
| kernel_nway | 2.440 | 84.6% |
| kernel_seq | 1.831 | 63.5% |
| nway | 0.957 | 33.2% |
| sequential | 1.155 | 40.1% |
| std_parallel | 1.165 | 40.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 50781ns | 50781ns | base |
| head_tail_dyn | 58874ns | 58874ns | +15.94% |
| kernel_nway | 47668ns | 47668ns | -6.13% |
| kernel_seq | 49406ns | 49406ns | -2.71% |
| nway | 92311ns | 92311ns | +81.78% |
| sequential | 56983ns | 56983ns | +12.21% |
| std_parallel | 65182ns | 65182ns | +28.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 37933ns | base | --- | [27986, 46423] | --- | --- | --- | --- |
| head_tail_dyn | 41655ns | no significant difference | [-3387, +25982]ns | [35679, 65476] | no | 0.2855 | 0.2379 | 0 |
| kernel_nway | 26859ns | -5725.5ns (-15.1%) | [-12808, -222]ns | [23121, 54362] | YES | 0.0463 | 0.0309 | 0 |
| kernel_seq | 35799ns | no significant difference | [-16374, +17702]ns | [28432, 46102] | no | 0.4807 | 0.4807 | 0 |
| nway | 68473ns | +28380.7ns (+74.8%) | [+23996, +40175]ns | [50047, 86153] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 56750ns | +18960.4ns (+50.0%) | [+10918, +28736]ns | [56644, 56907] | YES | 0.0463 | 0.0309 | 0 |
| std_parallel | 56233ns | +16325.2ns (+43.0%) | [+12387, +27570]ns | [41872, 69985] | YES | 0.0039 | 0.0013 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 23598ns | +151.7% | -1.7% | +82.2% | +101.5% | +140.5% | +68.5% |
| 1 | 2 | 32422ns | -22.6% | -0.9% | -8.9% | +113.1% | +74.9% | +105.9% |
| 1 | 3 | 40808ns | +1.4% | -33.3% | -42.3% | +69.0% | +39.2% | +47.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.000 | ok |
| head_tail_dyn | -0.483 | moderate- |
| kernel_nway | -0.530 | HIGH- (thermal bounce) |
| kernel_seq | -0.031 | ok |
| nway | -0.170 | ok |
| sequential | -0.483 | moderate- |
| std_parallel | -0.315 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 413518.0ns | 50167.8ns | 824.3% | HIGH |
| head_tail_dyn | 455879.8ns | 58117.5ns | 784.4% | HIGH |
| kernel_nway | 470048.6ns | 46970.4ns | 1000.7% | HIGH |
| kernel_seq | 395259.8ns | 48824.5ns | 809.6% | HIGH |
| nway | 450376.9ns | 91898.6ns | 490.1% | HIGH |
| sequential | 296078.2ns | 56802.6ns | 521.2% | HIGH |
| std_parallel | 506243.5ns | 64495.7ns | 784.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 23462.4-89944.2 ns)
  23462.4 |##############################
  26786.5 |########################################
  30110.6 |
  33434.6 |##########
  36758.7 |####################
  40082.8 |####################
  43406.9 |
  46731.0 |
  50055.1 |##########
  53379.2 |
  56703.3 |
  60027.4 |
  63351.5 |
  66675.6 |##########
  69999.7 |
  73323.8 |
  76647.9 |
  79972.0 |
  83296.0 |
  86620.1 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 23509.1-101610.8 ns)
  23509.1 |################
  27414.2 |########
  31319.2 |
  35224.3 |
  39129.4 |########################################
  43034.5 |
  46939.6 |########
  50844.7 |########
  54749.7 |
  58654.8 |
  62559.9 |################
  66465.0 |########
  70370.1 |
  74275.2 |
  78180.2 |
  82085.3 |
  85990.4 |
  89895.5 |
  93800.6 |
  97705.7 |
  (2 below, 3 above range)

kernel_nway (n=18, range 22730.3-90442.5 ns)
  22730.3 |########################################
  26115.9 |#################
  29501.5 |
  32887.1 |
  36272.7 |#####
  39658.3 |
  43043.9 |
  46429.5 |
  49815.1 |
  53200.8 |###########
  56586.4 |
  59972.0 |
  63357.6 |#####
  66743.2 |
  70128.8 |
  73514.4 |
  76900.0 |
  80285.6 |
  83671.2 |
  87056.9 |
  (1 below, 3 above range)

kernel_seq (n=18, range 23468.2-85595.1 ns)
  23468.2 |########################################
  26574.5 |########################################
  29680.9 |##########################
  32787.2 |
  35893.5 |
  38999.9 |##########################
  42106.2 |##########################
  45212.6 |
  48318.9 |#############
  51425.3 |#############
  54531.6 |
  57638.0 |
  60744.3 |
  63850.6 |
  66957.0 |
  70063.3 |
  73169.7 |
  76276.0 |
  79382.4 |
  82488.7 |
  (1 below, 3 above range)

nway (n=18, range 47341.5-160025.5 ns)
  47341.5 |##########################
  52975.7 |
  58609.9 |
  64244.1 |########################################
  69878.3 |######
  75512.5 |
  81146.7 |
  86780.9 |
  92415.1 |
  98049.3 |######
  103683.5 |######
  109317.7 |
  114951.9 |
  120586.1 |
  126220.3 |
  131854.5 |
  137488.7 |
  143122.9 |
  148757.1 |
  154391.3 |
  (2 below, 3 above range)

sequential (n=18, range 56529.3-57067.2 ns)
  56529.3 |
  56556.2 |
  56583.1 |####################
  56610.0 |##########
  56636.9 |
  56663.8 |##########
  56690.7 |##########
  56717.6 |
  56744.5 |########################################
  56771.3 |
  56798.2 |
  56825.1 |
  56852.0 |##########
  56878.9 |##########
  56905.8 |
  56932.7 |####################
  56959.6 |
  56986.5 |
  57013.4 |
  57040.3 |
  (2 below, 3 above range)

std_parallel (n=18, range 37607.2-97469.2 ns)
  37607.2 |########################################
  40600.3 |##########################
  43593.4 |#############
  46586.5 |
  49579.6 |
  52572.7 |#############
  55565.8 |##########################
  58558.9 |
  61552.0 |
  64545.1 |##########################
  67538.2 |#############
  70531.3 |#############
  73524.4 |#############
  76517.5 |
  79510.6 |
  82503.7 |
  85496.8 |
  88489.9 |
  91483.0 |
  94476.1 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=74.6% (high variance, measurements may be unstable)
- **head_tail**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **head_tail**: bridge=927.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=66.8% (high variance, measurements may be unstable)
- **head_tail_dyn**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **head_tail_dyn**: bridge=948.7% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=78.2% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **kernel_nway**: bridge=1409.0% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=71.5% (high variance, measurements may be unstable)
- **kernel_seq**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **kernel_seq**: bridge=1044.3% of algo (FFI overhead may distort results)
- **nway**: CV=68.1% (high variance, measurements may be unstable)
- **nway**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **nway**: bridge=546.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=521.7% of algo (FFI overhead may distort results)
- **std_parallel**: CV=47.1% (high variance, measurements may be unstable)
- **std_parallel**: bridge=857.6% of algo (FFI overhead may distort results)

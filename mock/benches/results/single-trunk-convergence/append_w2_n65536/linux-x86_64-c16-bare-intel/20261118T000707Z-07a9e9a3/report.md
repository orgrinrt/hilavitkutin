# append, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway is an outlier: 2.5x slower than the field

nway (68.22 us) is 2.5x the fastest (26.76 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway is fastest but the noisiest (CV 143.4%)

kernel_nway wins on median (26.76 us) yet has the highest variance (CV 143.4%), while sequential is the steadiest (CV 2.1%, 56.96 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.67)

kernel_seq's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### kernel_nway is inconsistent: worst-20% is 5.6x its best-20%

kernel_nway's best 20% of batches run at 15.46 us but its worst 20% at 86.99 us (5.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 26758.8 ns median (-8.5% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 2.55x (fastest 26758.8 ns, slowest 68220.1 ns)

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
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 45455ns | 29457ns | 28529ns | 29163ns | 78355ns | base |
| head_tail_dyn | 52440ns | 40351ns | 29118ns | 34746ns | 90642ns | +15.37% |
| kernel_nway | 44112ns | 26986ns | 15658ns | 24087ns | 88375ns | -2.95% |
| kernel_seq | 43743ns | 29123ns | 23593ns | 26700ns | 79384ns | -3.76% |
| nway | 84140ns | 68396ns | 42128ns | 58213ns | 144037ns | +85.11% |
| sequential | 57878ns | 57124ns | 56890ns | 57228ns | 59345ns | +27.33% |
| std_parallel | 60842ns | 49855ns | 35361ns | 45384ns | 96769ns | +33.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 44913ns | 28351ns | 77095ns | base | 1.459 |
| head_tail_dyn | 51822ns | 28903ns | 89199ns | +15.38% | 1.265 |
| kernel_nway | 43487ns | 15461ns | 86990ns | -3.18% | 1.507 |
| kernel_seq | 43187ns | 23433ns | 78105ns | -3.84% | 1.518 |
| nway | 83766ns | 41902ns | 143338ns | +86.51% | 0.782 |
| sequential | 57701ns | 56720ns | 59159ns | +28.47% | 1.136 |
| std_parallel | 60278ns | 35179ns | 95555ns | +34.21% | 1.087 |

## Performance model

- Peak throughput: **4.239 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 2.240 | 52.9% |
| head_tail_dyn | 1.632 | 38.5% |
| kernel_nway | 2.449 | 57.8% |
| kernel_seq | 2.269 | 53.5% |
| nway | 0.961 | 22.7% |
| sequential | 1.151 | 27.1% |
| std_parallel | 1.322 | 31.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 45455ns | 45455ns | base |
| head_tail_dyn | 52440ns | 52440ns | +15.37% |
| kernel_nway | 44112ns | 44112ns | -2.95% |
| kernel_seq | 43743ns | 43743ns | -3.76% |
| nway | 84140ns | 84140ns | +85.11% |
| sequential | 57878ns | 57878ns | +27.33% |
| std_parallel | 60842ns | 60842ns | +33.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 29252ns | base | --- | [28586, 35392] | --- | --- | --- | --- |
| head_tail_dyn | 40164ns | +1291.9ns (+4.4%) | [+503, +11498]ns | [29801, 55866] | YES | 0.0151 | 0.0075 | 0 |
| kernel_nway | 26759ns | no significant difference | [-12902, +1969]ns | [19841, 38132] | no | 0.2379 | 0.2379 | 0 |
| kernel_seq | 28877ns | no significant difference | [-5201, +775]ns | [23632, 32180] | no | 0.2379 | 0.2379 | 0 |
| nway | 68220ns | +26454.2ns (+90.4%) | [+16953, +39787]ns | [48139, 84152] | YES | 0.0009 | 0.0001 | 0 |
| sequential | 56957ns | +28197.5ns (+96.4%) | [+22253, +28581]ns | [56826, 58571] | YES | 0.0463 | 0.0309 | 0 |
| std_parallel | 49560ns | +12054.4ns (+41.2%) | [+7350, +23792]ns | [37787, 64437] | YES | 0.0151 | 0.0075 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 29252ns | +4.0% | -3.9% | -18.1% | +64.8% | +100.9% | +50.8% |
| 1 | 2 | 28987ns | +2.8% | +6.5% | +11.0% | +136.5% | +96.1% | +86.0% |
| 1 | 3 | 29134ns | +39.2% | -46.7% | -17.1% | +104.1% | +95.0% | +65.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.536 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.197 | ok |
| kernel_nway | -0.272 | moderate- |
| kernel_seq | -0.666 | HIGH- (thermal bounce) |
| nway | -0.463 | moderate- |
| sequential | -0.164 | ok |
| std_parallel | -0.545 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 388149.8ns | 44912.9ns | 864.2% | HIGH |
| head_tail_dyn | 427228.0ns | 51821.8ns | 824.4% | HIGH |
| kernel_nway | 403295.0ns | 43486.5ns | 927.4% | HIGH |
| kernel_seq | 402667.7ns | 43186.8ns | 932.4% | HIGH |
| nway | 431938.5ns | 83766.2ns | 515.6% | HIGH |
| sequential | 297501.9ns | 57700.9ns | 515.6% | HIGH |
| std_parallel | 443965.8ns | 60278.4ns | 736.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 28351.2-77094.9 ns)
  28351.2 |########################################
  30788.4 |
  33225.5 |###
  35662.7 |###
  38099.9 |
  40537.1 |
  42974.3 |
  45411.5 |
  47848.7 |
  50285.8 |
  52723.0 |
  55160.2 |
  57597.4 |###
  60034.6 |###
  62471.8 |
  64909.0 |
  67346.2 |
  69783.3 |
  72220.5 |
  74657.7 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 28902.7-89198.6 ns)
  28902.7 |########################################
  31917.5 |
  34932.3 |
  37947.1 |######################
  40961.9 |
  43976.7 |#####
  46991.5 |
  50006.2 |
  53021.0 |
  56035.8 |
  59050.6 |
  62065.4 |
  65080.2 |#####
  68095.0 |#####
  71109.8 |
  74124.6 |
  77139.4 |
  80154.2 |
  83169.0 |
  86183.8 |
  (1 below, 3 above range)

kernel_nway (n=18, range 15460.9-86990.0 ns)
  15460.9 |########################################
  19037.3 |##############################
  22613.8 |##########
  26190.2 |##########
  29766.7 |####################
  33343.2 |##########
  36919.6 |
  40496.1 |##########
  44072.5 |
  47649.0 |
  51225.4 |
  54801.9 |
  58378.3 |
  61954.8 |
  65531.3 |
  69107.7 |
  72684.2 |
  76260.6 |
  79837.1 |
  83413.5 |##########
  (1 below, 3 above range)

kernel_seq (n=18, range 23433.0-78105.4 ns)
  23433.0 |########################################
  26166.7 |#####
  28900.3 |######################
  31633.9 |#####
  34367.5 |
  37101.1 |
  39834.7 |
  42568.4 |
  45302.0 |
  48035.6 |
  50769.2 |
  53502.8 |
  56236.5 |
  58970.1 |
  61703.7 |
  64437.3 |
  67170.9 |
  69904.5 |
  72638.2 |
  75371.8 |
  (1 below, 4 above range)

nway (n=18, range 41902.4-143338.1 ns)
  41902.4 |####################
  46974.2 |########################################
  52046.0 |##########
  57117.8 |
  62189.5 |
  67261.3 |########################################
  72333.1 |##########
  77404.9 |
  82476.7 |
  87548.5 |
  92620.3 |##########
  97692.1 |
  102763.8 |
  107835.6 |
  112907.4 |
  117979.2 |
  123051.0 |
  128122.8 |##########
  133194.6 |##########
  138266.3 |
  (1 below, 2 above range)

sequential (n=18, range 56719.9-59158.9 ns)
  56719.9 |########################################
  56841.8 |#####
  56963.8 |#####
  57085.7 |#####
  57207.7 |
  57329.6 |
  57451.6 |
  57573.5 |
  57695.5 |
  57817.4 |
  57939.4 |
  58061.4 |
  58183.3 |
  58305.3 |#####
  58427.2 |
  58549.2 |###########
  58671.1 |
  58793.1 |###########
  58915.0 |#####
  59037.0 |
  (1 below, 1 above range)

std_parallel (n=18, range 35178.9-95555.2 ns)
  35178.9 |########################################
  38197.7 |##########
  41216.5 |
  44235.3 |
  47254.2 |####################
  50273.0 |####################
  53291.8 |##########
  56310.6 |
  59329.4 |##########
  62348.2 |
  65367.0 |####################
  68385.9 |
  71404.7 |
  74423.5 |
  77442.3 |
  80461.1 |##########
  83479.9 |##########
  86498.7 |
  89517.6 |
  92536.4 |
  (2 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=75.3% (high variance, measurements may be unstable)
- **head_tail**: bridge=1232.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=63.4% (high variance, measurements may be unstable)
- **head_tail_dyn**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **head_tail_dyn**: bridge=917.7% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=88.2% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 5.6x (possible bimodal distribution)
- **kernel_nway**: bridge=1261.4% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=73.5% (high variance, measurements may be unstable)
- **kernel_seq**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **kernel_seq**: bridge=1230.2% of algo (FFI overhead may distort results)
- **nway**: CV=64.0% (high variance, measurements may be unstable)
- **nway**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **nway**: bridge=551.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=520.0% of algo (FFI overhead may distort results)
- **std_parallel**: CV=65.4% (high variance, measurements may be unstable)
- **std_parallel**: bridge=685.6% of algo (FFI overhead may distort results)

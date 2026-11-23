# append, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 4.3x slower than the field

std_parallel (127.69 us) is 4.3x the fastest (29.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway is fastest but the noisiest (CV 160.0%)

kernel_nway wins on median (29.56 us) yet has the highest variance (CV 160.0%), while sequential is the steadiest (CV 0.3%, 56.84 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (kernel_nway, kernel_seq) are a dead heat (<1%)

kernel_nway (29.56 us) and kernel_seq (29.61 us) differ by 0.16%, inside the noise, even though the wider field spreads 332.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.66)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.3x the fastest

Fastest kernel_nway (29.56 us) to slowest std_parallel (127.69 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 4.8x its best-20%

kernel_nway's best 20% of batches run at 23.85 us but its worst 20% at 114.25 us (4.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 29558.7 ns median (-12.1% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 4.32x (fastest 29558.7 ns, slowest 127691.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 1 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112772868 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 52213ns | 33855ns | 28324ns | 33301ns | 92526ns | base |
| head_tail_dyn | 53629ns | 30478ns | 25361ns | 30848ns | 101936ns | +2.71% |
| kernel_nway | 57677ns | 29753ns | 24075ns | 30075ns | 115880ns | +10.46% |
| kernel_seq | 48814ns | 29846ns | 23333ns | 29276ns | 90863ns | -6.51% |
| nway | 78073ns | 57139ns | 55994ns | 57186ns | 120442ns | +49.53% |
| sequential | 57083ns | 57015ns | 56870ns | 56991ns | 57327ns | +9.33% |
| std_parallel | 144960ns | 128427ns | 74060ns | 116671ns | 222844ns | +177.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 51561ns | 28116ns | 91018ns | base | 1.271 |
| head_tail_dyn | 52931ns | 25168ns | 100272ns | +2.66% | 1.238 |
| kernel_nway | 56986ns | 23852ns | 114253ns | +10.52% | 1.150 |
| kernel_seq | 48184ns | 23164ns | 89385ns | -6.55% | 1.360 |
| nway | 77738ns | 55824ns | 119786ns | +50.77% | 0.843 |
| sequential | 56906ns | 56706ns | 57144ns | +10.37% | 1.152 |
| std_parallel | 143961ns | 73736ns | 220692ns | +179.20% | 0.455 |

## Performance model

- Peak throughput: **2.829 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.949 | 68.9% |
| head_tail_dyn | 2.166 | 76.6% |
| kernel_nway | 2.217 | 78.4% |
| kernel_seq | 2.214 | 78.2% |
| nway | 1.151 | 40.7% |
| sequential | 1.153 | 40.8% |
| std_parallel | 0.513 | 18.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 52213ns | 52213ns | base |
| head_tail_dyn | 53629ns | 53629ns | +2.71% |
| kernel_nway | 57677ns | 57677ns | +10.46% |
| kernel_seq | 48814ns | 48814ns | -6.51% |
| nway | 78073ns | 78073ns | +49.53% |
| sequential | 57083ns | 57083ns | +9.33% |
| std_parallel | 144960ns | 144960ns | +177.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 33623ns | base | --- | [29451, 52459] | --- | --- | --- | --- |
| head_tail_dyn | 30256ns | no significant difference | [-6370, +9444]ns | [28166, 58159] | no | 0.4807 | 0.4807 | 0 |
| kernel_nway | 29559ns | no significant difference | [-4705, +1420]ns | [27946, 65605] | no | 0.1444 | 0.0963 | 0 |
| kernel_seq | 29607ns | no significant difference | [-15921, +1446]ns | [28477, 33628] | no | 0.4807 | 0.4807 | 0 |
| nway | 56961ns | +25308.6ns (+75.3%) | [+19243, +28064]ns | [56271, 65394] | YES | 0.0004 | 0.0001 | 0 |
| sequential | 56844ns | +23160.8ns (+68.9%) | [+4540, +26802]ns | [56748, 57065] | YES (adj: no) | 0.1444 | 0.0963 | 0 |
| std_parallel | 127692ns | +76570.3ns (+227.7%) | [+55300, +122706]ns | [85235, 195733] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 39240ns | -22.9% | -26.1% | -35.9% | +45.2% | +45.0% | +283.2% |
| 1 | 2 | 28346ns | +62.1% | -12.5% | +8.6% | +98.2% | +100.2% | +192.1% |
| 1 | 3 | 37661ns | -25.2% | +74.2% | -23.7% | +55.6% | +51.2% | +357.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.655 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.659 | HIGH- (thermal bounce) |
| kernel_nway | -0.223 | moderate- |
| kernel_seq | -0.403 | moderate- |
| nway | -0.372 | moderate- |
| sequential | -0.596 | HIGH- (thermal bounce) |
| std_parallel | -0.630 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 442651.9ns | 51561.3ns | 858.5% | HIGH |
| head_tail_dyn | 417100.6ns | 52931.0ns | 788.0% | HIGH |
| kernel_nway | 476535.1ns | 56986.0ns | 836.2% | HIGH |
| kernel_seq | 434718.8ns | 48183.7ns | 902.2% | HIGH |
| nway | 383124.9ns | 77738.3ns | 492.8% | HIGH |
| sequential | 296132.6ns | 56906.4ns | 520.4% | HIGH |
| std_parallel | 609391.2ns | 143960.6ns | 423.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 28116.4-91018.4 ns)
  28116.4 |########################################
  31261.5 |################################
  34406.6 |########
  37551.7 |
  40696.8 |################
  43841.9 |
  46987.0 |
  50132.1 |
  53277.2 |
  56422.3 |
  59567.4 |########
  62712.5 |
  65857.6 |
  69002.7 |
  72147.8 |
  75292.9 |################
  78438.0 |
  81583.1 |
  84728.2 |
  87873.3 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 25167.7-100271.7 ns)
  25167.7 |########################################
  28922.9 |################################
  32678.1 |
  36433.3 |
  40188.5 |########
  43943.7 |
  47698.9 |########
  51454.1 |
  55209.3 |
  58964.5 |
  62719.7 |########
  66474.9 |########
  70230.1 |
  73985.3 |
  77740.5 |
  81495.7 |
  85250.9 |
  89006.1 |
  92761.3 |
  96516.5 |
  (2 below, 3 above range)

kernel_nway (n=18, range 23852.2-114253.1 ns)
  23852.2 |########################################
  28372.3 |########################################
  32892.3 |####################
  37412.4 |##########
  41932.4 |
  46452.5 |
  50972.5 |
  55492.5 |
  60012.6 |
  64532.6 |
  69052.7 |
  73572.7 |
  78092.8 |
  82612.8 |
  87132.9 |##########
  91652.9 |
  96173.0 |
  100693.0 |
  105213.0 |
  109733.1 |
  (2 below, 4 above range)

kernel_seq (n=18, range 23164.3-89384.8 ns)
  23164.3 |#############
  26475.4 |########################################
  29786.4 |####################
  33097.4 |######
  36408.4 |
  39719.5 |
  43030.5 |
  46341.5 |
  49652.5 |
  52963.6 |
  56274.6 |
  59585.6 |
  62896.6 |
  66207.6 |
  69518.7 |
  72829.7 |
  76140.7 |
  79451.7 |
  82762.8 |######
  86073.8 |
  (2 below, 3 above range)

nway (n=18, range 55823.7-119786.2 ns)
  55823.7 |########################################
  59021.9 |####
  62220.0 |####
  65418.1 |
  68616.2 |########
  71814.4 |
  75012.5 |
  78210.6 |
  81408.7 |
  84606.8 |
  87805.0 |
  91003.1 |
  94201.2 |
  97399.3 |
  100597.5 |
  103795.6 |
  106993.7 |
  110191.8 |
  113390.0 |
  116588.1 |
  (1 below, 3 above range)

sequential (n=18, range 56706.2-57143.9 ns)
  56706.2 |##########################
  56728.1 |#############
  56750.0 |#############
  56771.9 |#############
  56793.8 |#############
  56815.6 |
  56837.5 |##########################
  56859.4 |
  56881.3 |
  56903.2 |#############
  56925.1 |#############
  56946.9 |
  56968.8 |
  56990.7 |
  57012.6 |
  57034.5 |
  57056.3 |########################################
  57078.2 |#############
  57100.1 |
  57122.0 |
  (2 below, 2 above range)

std_parallel (n=18, range 73735.9-220691.5 ns)
  73735.9 |########################################
  81083.7 |########################################
  88431.4 |####################
  95779.2 |########################################
  103127.0 |
  110474.8 |
  117822.6 |
  125170.4 |
  132518.1 |
  139865.9 |
  147213.7 |####################
  154561.5 |####################
  161909.3 |
  169257.1 |
  176604.8 |
  183952.6 |####################
  191300.4 |########################################
  198648.2 |####################
  205996.0 |
  213343.8 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=69.8% (high variance, measurements may be unstable)
- **head_tail**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **head_tail**: bridge=1161.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=76.7% (high variance, measurements may be unstable)
- **head_tail_dyn**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **head_tail_dyn**: bridge=1162.5% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=83.0% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 4.8x (possible bimodal distribution)
- **kernel_nway**: bridge=1284.2% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=81.5% (high variance, measurements may be unstable)
- **kernel_seq**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **kernel_seq**: bridge=1256.3% of algo (FFI overhead may distort results)
- **nway**: CV=54.1% (high variance, measurements may be unstable)
- **nway**: bridge=526.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=520.9% of algo (FFI overhead may distort results)
- **std_parallel**: CV=44.4% (high variance, measurements may be unstable)
- **std_parallel**: bridge=392.5% of algo (FFI overhead may distort results)

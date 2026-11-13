# append, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 117% faster than the next best (head_tail)

sequential (11.38 us) leads head_tail (24.66 us) by 117%, a clear separation rather than a photo finish. CV 11.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 55% (significant)

sequential is -13.55 us (55%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 5.4x slower than the field

std_parallel (61.61 us) is 5.4x the fastest (11.38 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_parallel shows alternating (throttle bounce) (autocorr -0.59)

std_parallel's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential} vs {head_tail, kernel_seq, kernel_nway, head_tail_dyn, nway, std_parallel} (117% apart)

The field splits into a fast tier {sequential} and a slow tier {head_tail, kernel_seq, kernel_nway, head_tail_dyn, nway, std_parallel} with a 117% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.4x the fastest

Fastest sequential (11.38 us) to slowest std_parallel (61.61 us): 5.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### head_tail_dyn is inconsistent: worst-20% is 2.8x its best-20%

head_tail_dyn's best 20% of batches run at 20.66 us but its worst 20% at 57.97 us (2.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 11375.0 ns median (-53.9% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 5.42x (fastest 11375.0 ns, slowest 61613.1 ns)

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
| Counter frequency | 2100342298 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 30930ns | 25087ns | 21086ns | 24558ns | 45409ns | base |
| head_tail_dyn | 38616ns | 34773ns | 21044ns | 30874ns | 59014ns | +24.85% |
| kernel_nway | 32522ns | 31369ns | 27781ns | 30611ns | 37760ns | +5.15% |
| kernel_seq | 27272ns | 26849ns | 21821ns | 25475ns | 32693ns | -11.83% |
| nway | 41366ns | 42718ns | 36160ns | 41006ns | 44510ns | +33.74% |
| sequential | 12158ns | 11656ns | 11268ns | 11642ns | 13376ns | -60.69% |
| std_parallel | 73232ns | 62124ns | 46809ns | 58099ns | 109144ns | +136.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 30304ns | 20697ns | 44586ns | base | 0.135 |
| head_tail_dyn | 37817ns | 20658ns | 57974ns | +24.79% | 0.108 |
| kernel_nway | 31975ns | 27062ns | 37133ns | +5.51% | 0.128 |
| kernel_seq | 26566ns | 21441ns | 31647ns | -12.33% | 0.154 |
| nway | 40950ns | 35785ns | 44061ns | +35.13% | 0.100 |
| sequential | 11879ns | 11010ns | 13079ns | -60.80% | 0.345 |
| std_parallel | 72529ns | 46284ns | 108151ns | +139.34% | 0.056 |

## Performance model

- Peak throughput: **0.372 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.166 | 44.7% |
| head_tail_dyn | 0.120 | 32.4% |
| kernel_nway | 0.132 | 35.6% |
| kernel_seq | 0.157 | 42.3% |
| nway | 0.097 | 26.0% |
| sequential | 0.360 | 96.8% |
| std_parallel | 0.066 | 17.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 30930ns | 30930ns | base |
| head_tail_dyn | 38616ns | 38616ns | +24.85% |
| kernel_nway | 32522ns | 32522ns | +5.15% |
| kernel_seq | 27272ns | 27272ns | -11.83% |
| nway | 41366ns | 41366ns | +33.74% |
| sequential | 12158ns | 12158ns | -60.69% |
| std_parallel | 73232ns | 73232ns | +136.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 24656ns | base | --- | [22015, 29103] | --- | --- | --- | --- |
| head_tail_dyn | 34029ns | +5002.0ns (+20.3%) | [+1476, +10185]ns | [23357, 34807] | YES | 0.0113 | 0.0075 | 0 |
| kernel_nway | 30956ns | +7636.9ns (+31.0%) | [+3581, +9685]ns | [28310, 34289] | YES | 0.0371 | 0.0309 | 0 |
| kernel_seq | 26036ns | no significant difference | [-3176, +4336]ns | [23074, 29125] | no | 0.8145 | 0.8145 | 0 |
| nway | 42298ns | +12945.2ns (+52.5%) | [+10487, +19127]ns | [38484, 43353] | YES | 0.0003 | 0.0001 | 0 |
| sequential | 11375ns | -13547.2ns (-54.9%) | [-17200, -10467]ns | [11088, 12087] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 61613ns | +32976.4ns (+133.7%) | [+28349, +37472]ns | [52584, 64717] | YES | 0.0003 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 24656ns | +38.8% | +42.4% | -10.8% | +49.0% | -54.5% | +151.8% |
| 1 | 2 | 20956ns | +0.3% | +38.3% | +24.2% | +107.4% | -44.2% | +135.0% |
| 1 | 3 | 29854ns | +44.4% | -0.4% | +4.9% | +43.8% | -61.2% | +130.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.441 | moderate- |
| head_tail_dyn | -0.560 | HIGH- (thermal bounce) |
| kernel_nway | -0.235 | moderate- |
| kernel_seq | -0.004 | ok |
| nway | -0.209 | moderate- |
| sequential | -0.297 | moderate- |
| std_parallel | -0.588 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 49881.1ns | 30303.9ns | 164.6% | HIGH |
| head_tail_dyn | 57662.1ns | 37816.8ns | 152.5% | HIGH |
| kernel_nway | 52820.6ns | 31974.7ns | 165.2% | HIGH |
| kernel_seq | 51201.5ns | 26566.5ns | 192.7% | HIGH |
| nway | 63898.6ns | 40950.5ns | 156.0% | HIGH |
| sequential | 32419.7ns | 11878.6ns | 272.9% | HIGH |
| std_parallel | 81280.0ns | 72528.7ns | 112.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 20697.4-44586.2 ns)
  20697.4 |########################################
  21891.9 |##########################
  23086.3 |#############
  24280.7 |##########################
  25475.2 |
  26669.6 |##########################
  27864.0 |#############
  29058.5 |##########################
  30252.9 |#############
  31447.4 |
  32641.8 |#############
  33836.2 |
  35030.7 |
  36225.1 |
  37419.5 |
  38614.0 |
  39808.4 |
  41002.8 |
  42197.3 |
  43391.7 |
  (2 below, 1 above range)

head_tail_dyn (n=18, range 20657.8-57973.8 ns)
  20657.8 |####################
  22523.6 |#############
  24389.4 |
  26255.2 |
  28121.0 |
  29986.8 |
  31852.6 |#############
  33718.4 |########################################
  35584.2 |
  37450.0 |
  39315.8 |
  41181.6 |######
  43047.4 |#############
  44913.2 |
  46779.0 |
  48644.8 |
  50510.6 |
  52376.4 |
  54242.2 |
  56108.0 |
  (1 below, 1 above range)

kernel_nway (n=18, range 27062.0-37132.8 ns)
  27062.0 |####################
  27565.6 |########################################
  28069.1 |########################################
  28572.6 |
  29076.2 |####################
  29579.7 |
  30083.3 |####################
  30586.8 |####################
  31090.3 |####################
  31593.9 |####################
  32097.4 |
  32601.0 |
  33104.5 |####################
  33608.0 |####################
  34111.6 |
  34615.1 |####################
  35118.6 |####################
  35622.2 |####################
  36125.7 |
  36629.3 |
  (1 below, 2 above range)

kernel_seq (n=18, range 21441.5-31646.8 ns)
  21441.5 |########################################
  21951.7 |####################
  22462.0 |####################
  22972.3 |
  23482.5 |########################################
  23992.8 |
  24503.1 |####################
  25013.3 |
  25523.6 |####################
  26033.9 |####################
  26544.1 |
  27054.4 |####################
  27564.7 |####################
  28074.9 |
  28585.2 |####################
  29095.5 |####################
  29605.7 |
  30116.0 |
  30626.3 |
  31136.5 |########################################
  (1 below, 2 above range)

nway (n=18, range 35785.3-44060.6 ns)
  35785.3 |
  36199.0 |#############
  36612.8 |
  37026.6 |#############
  37440.3 |#############
  37854.1 |
  38267.9 |
  38681.6 |
  39095.4 |
  39509.2 |##########################
  39922.9 |
  40336.7 |
  40750.5 |#############
  41164.2 |
  41578.0 |
  41991.8 |#############
  42405.5 |##########################
  42819.3 |#############
  43233.1 |########################################
  43646.8 |
  (2 below, 3 above range)

sequential (n=18, range 11009.5-13078.8 ns)
  11009.5 |########################################
  11113.0 |#############
  11216.4 |##########################
  11319.9 |##########################
  11423.4 |
  11526.8 |
  11630.3 |
  11733.7 |#############
  11837.2 |
  11940.7 |#############
  12044.1 |##########################
  12147.6 |#############
  12251.1 |#############
  12354.5 |
  12458.0 |
  12561.5 |
  12664.9 |
  12768.4 |
  12871.8 |
  12975.3 |
  (2 below, 2 above range)

std_parallel (n=18, range 46283.9-108151.0 ns)
  46283.9 |##########
  49377.3 |####################
  52470.6 |####################
  55564.0 |##########
  58657.3 |##########
  61750.7 |########################################
  64844.0 |##########
  67937.4 |####################
  71030.7 |
  74124.1 |
  77217.5 |
  80310.8 |
  83404.2 |
  86497.5 |
  89590.9 |
  92684.2 |
  95777.6 |
  98870.9 |
  101964.3 |
  105057.7 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=69.6% (high variance, measurements may be unstable)
- **head_tail**: bridge=196.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=73.2% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=172.4% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=166.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=184.4% of algo (FFI overhead may distort results)
- **nway**: bridge=151.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=278.4% of algo (FFI overhead may distort results)
- **std_parallel**: CV=67.2% (high variance, measurements may be unstable)
- **std_parallel**: bridge=133.3% of algo (FFI overhead may distort results)

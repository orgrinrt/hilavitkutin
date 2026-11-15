# fold_light, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 134% faster than the next best (kernel_seq)

sequential (15.90 us) leads kernel_seq (37.28 us) by 134%, a clear separation rather than a photo finish. CV 38.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 50% (significant)

sequential is -19.85 us (50%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 5.6x slower than the field

std_parallel (89.19 us) is 5.6x the fastest (15.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 38.3%)

sequential wins on median (15.90 us) yet has the highest variance (CV 38.3%), while kernel_nway is the steadiest (CV 9.7%, 43.13 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### head_tail shows alternating (throttle bounce) (autocorr -0.66)

head_tail's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential} vs {kernel_seq, head_tail_dyn, head_tail, kernel_nway, nway, std_parallel} (134% apart)

The field splits into a fast tier {sequential} and a slow tier {kernel_seq, head_tail_dyn, head_tail, kernel_nway, nway, std_parallel} with a 134% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.6x the fastest

Fastest sequential (15.90 us) to slowest std_parallel (89.19 us): 5.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 1.9x its best-20%

sequential's best 20% of batches run at 14.75 us but its worst 20% at 28.17 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 15900.9 ns median (-59.6% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 5.61x (fastest 15900.9 ns, slowest 89185.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 4 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100312120 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 39374ns | 40177ns | 31815ns | 39056ns | 43630ns | base |
| head_tail_dyn | 40934ns | 39274ns | 35176ns | 39203ns | 46409ns | +3.96% |
| kernel_nway | 44994ns | 43495ns | 39492ns | 43558ns | 49898ns | +14.27% |
| kernel_seq | 37874ns | 37689ns | 29231ns | 35775ns | 45344ns | -3.81% |
| nway | 64244ns | 62129ns | 53102ns | 61827ns | 73440ns | +63.16% |
| sequential | 19964ns | 16167ns | 15015ns | 15856ns | 28601ns | -49.30% |
| std_parallel | 94273ns | 89875ns | 80489ns | 88170ns | 110321ns | +139.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 38827ns | 31198ns | 43205ns | base | 1.688 |
| head_tail_dyn | 40256ns | 34763ns | 45267ns | +3.68% | 1.628 |
| kernel_nway | 44394ns | 38980ns | 49248ns | +14.34% | 1.476 |
| kernel_seq | 37231ns | 28854ns | 44434ns | -4.11% | 1.760 |
| nway | 63805ns | 52760ns | 72995ns | +64.33% | 1.027 |
| sequential | 19635ns | 14749ns | 28167ns | -49.43% | 3.338 |
| std_parallel | 93749ns | 79952ns | 109851ns | +141.45% | 0.699 |

## Performance model

- Peak throughput: **4.443 Gops/s** (sequential; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.664 | 37.4% |
| head_tail_dyn | 1.689 | 38.0% |
| kernel_nway | 1.520 | 34.2% |
| kernel_seq | 1.758 | 39.6% |
| nway | 1.062 | 23.9% |
| sequential | 4.122 | 92.8% |
| std_parallel | 0.735 | 16.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 39374ns | 39374ns | base |
| head_tail_dyn | 40934ns | 40934ns | +3.96% |
| kernel_nway | 44994ns | 44994ns | +14.27% |
| kernel_seq | 37874ns | 37874ns | -3.81% |
| nway | 64244ns | 64244ns | +63.16% |
| sequential | 19964ns | 19964ns | -49.30% |
| std_parallel | 94273ns | 94273ns | +139.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 39391ns | base | --- | [37547, 40678] | --- | --- | --- | --- |
| head_tail_dyn | 38812ns | no significant difference | [-3684, +6960]ns | [37838, 42291] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 43125ns | +6105.9ns (+15.5%) | [+1590, +10180]ns | [40608, 48236] | YES | 0.0113 | 0.0075 | 0 |
| kernel_seq | 37281ns | no significant difference | [-9591, +4637]ns | [31348, 41157] | no | 0.9774 | 0.8145 | 0 |
| nway | 61722ns | +23537.3ns (+59.8%) | [+19167, +29753]ns | [58791, 69763] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 15901ns | -19853.4ns (-50.4%) | [-24113, -13600]ns | [15061, 26632] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 89185ns | +50121.6ns (+127.2%) | [+42923, +60649]ns | [84135, 99345] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 40015ns | -6.1% | +20.5% | -25.8% | +52.9% | -62.1% | +105.0% |
| 1 | 2 | 35942ns | +11.1% | +26.0% | +4.5% | +95.8% | -57.7% | +204.1% |
| 1 | 3 | 39514ns | +0.6% | +1.7% | +12.6% | +45.3% | -28.6% | +130.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.658 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.203 | moderate- |
| kernel_nway | -0.015 | ok |
| kernel_seq | -0.001 | ok |
| nway | -0.614 | HIGH- (thermal bounce) |
| sequential | -0.165 | ok |
| std_parallel | -0.596 | HIGH- (thermal bounce) |

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
| head_tail | 39776.0ns | 38827.5ns | 102.4% | HIGH |
| head_tail_dyn | 40947.7ns | 40256.1ns | 101.7% | HIGH |
| kernel_nway | 45031.0ns | 44393.7ns | 101.4% | HIGH |
| kernel_seq | 38689.1ns | 37230.8ns | 103.9% | HIGH |
| nway | 65687.0ns | 63804.8ns | 102.9% | HIGH |
| sequential | 19702.2ns | 19634.8ns | 100.3% | HIGH |
| std_parallel | 90415.1ns | 93749.2ns | 96.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 31198.1-43204.6 ns)
  31198.1 |
  31798.4 |
  32398.8 |
  32999.1 |
  33599.4 |####################
  34199.7 |####################
  34800.1 |
  35400.4 |
  36000.7 |
  36601.0 |
  37201.4 |####################
  37801.7 |####################
  38402.0 |########################################
  39002.3 |########################################
  39602.7 |########################################
  40203.0 |####################
  40803.3 |####################
  41403.6 |########################################
  42004.0 |
  42604.3 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 34763.2-45267.0 ns)
  34763.2 |
  35288.4 |##########
  35813.5 |##########
  36338.7 |
  36863.9 |
  37389.1 |##########
  37914.3 |##########
  38439.5 |########################################
  38964.7 |
  39489.9 |
  40015.1 |
  40540.3 |##########
  41065.5 |##########
  41590.7 |##########
  42115.8 |
  42641.0 |####################
  43166.2 |
  43691.4 |
  44216.6 |##########
  44741.8 |
  (2 below, 2 above range)

kernel_nway (n=18, range 38979.6-49248.5 ns)
  38979.6 |
  39493.0 |########################################
  40006.5 |####################
  40519.9 |####################
  41033.4 |
  41546.8 |
  42060.3 |########################################
  42573.7 |####################
  43087.1 |####################
  43600.6 |
  44114.0 |
  44627.5 |
  45140.9 |
  45654.4 |
  46167.8 |
  46681.2 |####################
  47194.7 |
  47708.1 |########################################
  48221.6 |####################
  48735.0 |####################
  (2 below, 3 above range)

kernel_seq (n=18, range 28854.3-44434.3 ns)
  28854.3 |##########################
  29633.3 |#############
  30412.3 |#############
  31191.3 |
  31970.3 |#############
  32749.3 |
  33528.3 |
  34307.3 |
  35086.3 |
  35865.3 |
  36644.3 |########################################
  37423.3 |########################################
  38202.3 |#############
  38981.3 |
  39760.3 |
  40539.3 |
  41318.3 |
  42097.3 |
  42876.3 |
  43655.3 |##########################
  (1 below, 3 above range)

nway (n=18, range 52760.1-72995.1 ns)
  52760.1 |
  53771.8 |
  54783.6 |
  55795.3 |####################
  56807.1 |
  57818.8 |########################################
  58830.6 |########################################
  59842.3 |
  60854.1 |########################################
  61865.8 |####################
  62877.6 |
  63889.3 |
  64901.1 |####################
  65912.8 |####################
  66924.6 |
  67936.3 |
  68948.1 |####################
  69959.8 |########################################
  70971.6 |####################
  71983.3 |
  (2 below, 2 above range)

sequential (n=18, range 14749.1-28167.5 ns)
  14749.1 |########################################
  15420.0 |################################
  16091.0 |
  16761.9 |########
  17432.8 |
  18103.7 |
  18774.6 |
  19445.5 |
  20116.5 |
  20787.4 |
  21458.3 |
  22129.2 |
  22800.1 |
  23471.0 |
  24142.0 |
  24812.9 |
  25483.8 |
  26154.7 |########
  26825.6 |########
  27496.5 |########
  (2 below, 3 above range)

std_parallel (n=18, range 79951.9-109850.9 ns)
  79951.9 |########################################
  81446.8 |########################################
  82941.8 |
  84436.7 |
  85931.7 |########################################
  87426.6 |########################################
  88921.6 |
  90416.5 |########################################
  91911.5 |
  93406.5 |####################
  94901.4 |####################
  96396.4 |
  97891.3 |
  99386.3 |
  100881.2 |
  102376.2 |####################
  103871.1 |
  105366.1 |
  106861.0 |
  108356.0 |####################
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=100.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=104.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=103.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=103.1% of algo (FFI overhead may distort results)
- **nway**: bridge=105.6% of algo (FFI overhead may distort results)
- **sequential**: CV=31.0% (high variance, measurements may be unstable)
- **sequential**: bridge=102.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=96.5% of algo (FFI overhead may distort results)

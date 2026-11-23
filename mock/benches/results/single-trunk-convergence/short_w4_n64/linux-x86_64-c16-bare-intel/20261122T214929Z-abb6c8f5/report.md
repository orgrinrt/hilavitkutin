# fold_light, short fiber (1 to 4 morsels), W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 28% faster than the next best (sequential)

kernel_seq (8.40 us) leads sequential (10.77 us) by 28%, a clear separation rather than a photo finish. CV 31.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 29% (significant)

kernel_seq is -3.45 us (29%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 6.9x slower than the field

std_parallel (57.69 us) is 6.9x the fastest (8.40 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.65)

nway's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_seq, sequential, head_tail_dyn, head_tail, kernel_nway} vs {nway, std_parallel} (148% apart)

The field splits into a fast tier {kernel_seq, sequential, head_tail_dyn, head_tail, kernel_nway} and a slow tier {nway, std_parallel} with a 148% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.9x the fastest

Fastest kernel_seq (8.40 us) to slowest std_parallel (57.69 us): 6.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.6x its best-20%

sequential's best 20% of batches run at 6.67 us but its worst 20% at 24.19 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 8403.1 ns median (-28.9% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 6.87x (fastest 8403.1 ns, slowest 57691.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 4 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112732635 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 12838ns | 12396ns | 11126ns | 12161ns | 14709ns | base |
| head_tail_dyn | 12208ns | 11720ns | 10688ns | 11526ns | 13991ns | -4.90% |
| kernel_nway | 15682ns | 15374ns | 12844ns | 15006ns | 18114ns | +22.15% |
| kernel_seq | 8592ns | 8917ns | 4972ns | 7726ns | 11701ns | -33.07% |
| nway | 33427ns | 36784ns | 20081ns | 31041ns | 43679ns | +160.38% |
| sequential | 14878ns | 10984ns | 6805ns | 11013ns | 24711ns | +15.89% |
| std_parallel | 64594ns | 58270ns | 44121ns | 57686ns | 85194ns | +403.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 12151ns | 10621ns | 13862ns | base | 0.005 |
| head_tail_dyn | 11502ns | 10173ns | 13193ns | -5.34% | 0.006 |
| kernel_nway | 14841ns | 12227ns | 17023ns | +22.14% | 0.004 |
| kernel_seq | 8010ns | 4684ns | 10840ns | -34.08% | 0.008 |
| nway | 32974ns | 19748ns | 43125ns | +171.37% | 0.002 |
| sequential | 14575ns | 6667ns | 24194ns | +19.94% | 0.004 |
| std_parallel | 64049ns | 43835ns | 84393ns | +427.10% | 0.001 |

## Performance model

- Peak throughput: **0.014 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.005 | 39.6% |
| head_tail_dyn | 0.006 | 42.5% |
| kernel_nway | 0.004 | 32.0% |
| kernel_seq | 0.008 | 55.7% |
| nway | 0.002 | 12.9% |
| sequential | 0.006 | 43.5% |
| std_parallel | 0.001 | 8.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 12838ns | 12838ns | base |
| head_tail_dyn | 12208ns | 12208ns | -4.90% |
| kernel_nway | 15682ns | 15682ns | +22.15% |
| kernel_seq | 8592ns | 8592ns | -33.07% |
| nway | 33427ns | 33427ns | +160.38% |
| sequential | 14878ns | 14878ns | +15.89% |
| std_parallel | 64594ns | 64594ns | +403.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 11822ns | base | --- | [11114, 12699] | --- | --- | --- | --- |
| head_tail_dyn | 11031ns | no significant difference | [-1846, +1062]ns | [10374, 12215] | no | 0.9774 | 0.8145 | 0 |
| kernel_nway | 14656ns | +2648.9ns (+22.4%) | [+1191, +3805]ns | [13565, 15829] | YES | 0.0020 | 0.0013 | 0 |
| kernel_seq | 8403ns | -3453.5ns (-29.2%) | [-6485, -1657]ns | [4942, 9729] | YES | 0.0003 | 0.0001 | 0 |
| nway | 36323ns | +23775.8ns (+201.1%) | [+9413, +25865]ns | [19927, 39183] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10771ns | no significant difference | [-4045, +8788]ns | [6937, 21689] | no | 1.0000 | 1.0000 | 0 |
| std_parallel | 57692ns | +45242.8ns (+382.7%) | [+43106, +57907]ns | [54644, 71681] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 11988ns | -8.0% | +34.2% | -60.3% | +231.8% | +36.9% | +623.6% |
| 1 | 2 | 11822ns | +10.7% | +9.2% | -11.4% | +67.4% | -33.7% | +368.9% |
| 1 | 3 | 11672ns | -11.5% | +28.2% | -26.5% | +211.3% | +19.9% | +366.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.001 | ok |
| head_tail_dyn | -0.627 | HIGH- (thermal bounce) |
| kernel_nway | -0.584 | HIGH- (thermal bounce) |
| kernel_seq | -0.378 | moderate- |
| nway | -0.649 | HIGH- (thermal bounce) |
| sequential | -0.617 | HIGH- (thermal bounce) |
| std_parallel | -0.152 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 12614.8ns | 12151.1ns | 103.8% | HIGH |
| head_tail_dyn | 11868.1ns | 11502.3ns | 103.2% | HIGH |
| kernel_nway | 15325.9ns | 14841.5ns | 103.3% | HIGH |
| kernel_seq | 8333.2ns | 8009.9ns | 104.0% | HIGH |
| nway | 34453.7ns | 32974.3ns | 104.5% | HIGH |
| sequential | 15054.2ns | 14574.6ns | 103.3% | HIGH |
| std_parallel | 66402.0ns | 64049.2ns | 103.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 10620.6-13862.2 ns)
  10620.6 |########################################
  10782.7 |
  10944.8 |########################################
  11106.8 |########################################
  11268.9 |####################
  11431.0 |
  11593.1 |####################
  11755.1 |
  11917.2 |####################
  12079.3 |########################################
  12241.4 |
  12403.5 |
  12565.5 |########################################
  12727.6 |
  12889.7 |
  13051.8 |
  13213.9 |
  13375.9 |####################
  13538.0 |####################
  13700.1 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 10172.5-13193.2 ns)
  10172.5 |########################################
  10323.6 |########################################
  10474.6 |####################
  10625.6 |####################
  10776.7 |####################
  10927.7 |
  11078.7 |########################################
  11229.8 |
  11380.8 |
  11531.8 |
  11682.9 |####################
  11833.9 |
  11985.0 |
  12136.0 |########################################
  12287.0 |
  12438.1 |
  12589.1 |
  12740.1 |
  12891.2 |
  13042.2 |########################################
  (2 below, 2 above range)

kernel_nway (n=18, range 12227.1-17023.3 ns)
  12227.1 |
  12466.9 |####################
  12706.7 |####################
  12946.5 |####################
  13186.3 |
  13426.2 |####################
  13666.0 |####################
  13905.8 |####################
  14145.6 |####################
  14385.4 |####################
  14625.2 |####################
  14865.0 |
  15104.8 |########################################
  15344.7 |####################
  15584.5 |
  15824.3 |
  16064.1 |####################
  16303.9 |
  16543.7 |
  16783.5 |####################
  (1 below, 3 above range)

kernel_seq (n=18, range 4683.6-10839.8 ns)
   4683.6 |########################################
   4991.4 |##########
   5299.2 |
   5607.0 |
   5914.8 |
   6222.7 |
   6530.5 |
   6838.3 |
   7146.1 |##########
   7453.9 |##########
   7761.7 |##########
   8069.5 |
   8377.3 |
   8685.1 |##########
   8993.0 |##########
   9300.8 |####################
   9608.6 |
   9916.4 |##########
  10224.2 |####################
  10532.0 |
  (1 below, 2 above range)

nway (n=18, range 19748.1-43125.3 ns)
  19748.1 |########################################
  20917.0 |
  22085.8 |
  23254.7 |
  24423.5 |
  25592.4 |
  26761.3 |
  27930.1 |
  29099.0 |
  30267.8 |
  31436.7 |
  32605.6 |
  33774.4 |################
  34943.3 |
  36112.1 |########################
  37281.0 |########
  38449.8 |################
  39618.7 |########
  40787.6 |
  41956.4 |
  (1 below, 3 above range)

sequential (n=18, range 6667.2-24194.0 ns)
   6667.2 |########################################
   7543.5 |
   8419.9 |######
   9296.2 |
  10172.5 |
  11048.9 |
  11925.2 |
  12801.6 |######
  13677.9 |
  14554.3 |
  15430.6 |
  16306.9 |
  17183.3 |
  18059.6 |
  18936.0 |
  19812.3 |######
  20688.7 |#############
  21565.0 |
  22441.3 |#############
  23317.7 |
  (2 below, 3 above range)

std_parallel (n=18, range 43834.7-84392.5 ns)
  43834.7 |
  45862.6 |
  47890.5 |
  49918.4 |
  51946.3 |
  53974.2 |########################################
  56002.1 |#############
  58030.0 |######
  60057.9 |
  62085.7 |
  64113.6 |######
  66141.5 |######
  68169.4 |
  70197.3 |
  72225.2 |
  74253.1 |######
  76281.0 |
  78308.9 |
  80336.8 |
  82364.6 |######
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=104.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=105.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=103.5% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=32.6% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=103.7% of algo (FFI overhead may distort results)
- **nway**: CV=30.9% (high variance, measurements may be unstable)
- **nway**: bridge=102.5% of algo (FFI overhead may distort results)
- **sequential**: CV=55.5% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **sequential**: bridge=103.9% of algo (FFI overhead may distort results)
- **std_parallel**: CV=26.0% (high variance, measurements may be unstable)
- **std_parallel**: bridge=104.8% of algo (FFI overhead may distort results)

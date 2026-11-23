# fold_light, short fiber (1 to 4 morsels), W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 67% faster than the next best (kernel_seq)

sequential (7.00 us) leads kernel_seq (11.70 us) by 67%, a clear separation rather than a photo finish. CV 128.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 27% (significant)

kernel_seq is -3.95 us (27%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 7.3x slower than the field

std_parallel (50.80 us) is 7.3x the fastest (7.00 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 128.0%)

sequential wins on median (7.00 us) yet has the highest variance (CV 128.0%), while kernel_nway is the steadiest (CV 13.2%, 13.93 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.57)

kernel_nway's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 7.3x the fastest

Fastest sequential (7.00 us) to slowest std_parallel (50.80 us): 7.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.8x its best-20%

sequential's best 20% of batches run at 6.75 us but its worst 20% at 25.60 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 7001.3 ns median (-52.5% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 7.26x (fastest 7001.3 ns, slowest 50799.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 2 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112817100 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 16069ns | 15490ns | 13123ns | 15085ns | 19019ns | base |
| head_tail_dyn | 13596ns | 13382ns | 11407ns | 12722ns | 16001ns | -15.39% |
| kernel_nway | 14490ns | 14646ns | 11591ns | 14016ns | 16650ns | -9.83% |
| kernel_seq | 12945ns | 12572ns | 10026ns | 12223ns | 15488ns | -19.44% |
| nway | 37016ns | 34573ns | 31350ns | 34084ns | 44247ns | +130.35% |
| sequential | 14021ns | 7137ns | 6886ns | 8322ns | 26138ns | -12.74% |
| std_parallel | 52578ns | 51477ns | 41630ns | 50100ns | 61769ns | +227.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 15237ns | 12548ns | 17906ns | base | 0.004 |
| head_tail_dyn | 12783ns | 10804ns | 14956ns | -16.11% | 0.005 |
| kernel_nway | 13646ns | 10977ns | 15619ns | -10.44% | 0.005 |
| kernel_seq | 12044ns | 9374ns | 14412ns | -20.96% | 0.005 |
| nway | 36497ns | 30937ns | 43579ns | +139.54% | 0.002 |
| sequential | 13737ns | 6747ns | 25599ns | -9.84% | 0.005 |
| std_parallel | 51988ns | 41263ns | 60916ns | +241.20% | 0.001 |

## Performance model

- Peak throughput: **0.009 Gops/s** (sequential; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.004 | 45.8% |
| head_tail_dyn | 0.005 | 54.0% |
| kernel_nway | 0.005 | 48.4% |
| kernel_seq | 0.005 | 57.7% |
| nway | 0.002 | 19.8% |
| sequential | 0.009 | 96.4% |
| std_parallel | 0.001 | 13.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 16069ns | 16069ns | base |
| head_tail_dyn | 13596ns | 13596ns | -15.39% |
| kernel_nway | 14490ns | 14490ns | -9.83% |
| kernel_seq | 12945ns | 12945ns | -19.44% |
| nway | 37016ns | 37016ns | +130.35% |
| sequential | 14021ns | 14021ns | -12.74% |
| std_parallel | 52578ns | 52578ns | +227.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 14745ns | base | --- | [13760, 16173] | --- | --- | --- | --- |
| head_tail_dyn | 12484ns | no significant difference | [-4204, +541]ns | [11265, 13345] | no | 0.2855 | 0.2379 | 0 |
| kernel_nway | 13926ns | no significant difference | [-3246, +964]ns | [12644, 14495] | no | 0.2855 | 0.2379 | 0 |
| kernel_seq | 11700ns | -3951.4ns (-26.8%) | [-5454, -631]ns | [10763, 13349] | YES | 0.0151 | 0.0075 | 0 |
| nway | 34112ns | +18848.6ns (+127.8%) | [+18349, +23635]ns | [32088, 38798] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 7001ns | no significant difference | [-7468, +4090]ns | [6813, 20703] | no | 0.4807 | 0.4807 | 0 |
| std_parallel | 50800ns | +34451.5ns (+233.7%) | [+31693, +41650]ns | [47789, 55580] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 13599ns | +6.7% | +14.8% | -0.9% | +140.0% | -48.5% | +352.4% |
| 1 | 2 | 15707ns | -24.9% | -27.9% | -25.8% | +112.8% | -56.0% | +217.6% |
| 1 | 3 | 16160ns | -29.5% | -13.8% | -36.8% | +143.6% | -22.4% | +200.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.082 | ok |
| head_tail_dyn | -0.103 | ok |
| kernel_nway | -0.565 | HIGH- (thermal bounce) |
| kernel_seq | -0.003 | ok |
| nway | -0.109 | ok |
| sequential | -0.175 | ok |
| std_parallel | -0.115 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 15814.0ns | 15236.7ns | 103.8% | HIGH |
| head_tail_dyn | 13173.9ns | 12782.6ns | 103.1% | HIGH |
| kernel_nway | 14358.6ns | 13646.3ns | 105.2% | HIGH |
| kernel_seq | 12698.4ns | 12043.8ns | 105.4% | HIGH |
| nway | 37190.4ns | 36497.5ns | 101.9% | HIGH |
| sequential | 14268.2ns | 13736.7ns | 103.9% | HIGH |
| std_parallel | 48193.1ns | 51987.8ns | 92.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 12548.0-17905.7 ns)
  12548.0 |
  12815.9 |
  13083.7 |
  13351.6 |##########
  13619.5 |########################################
  13887.4 |
  14155.3 |##########
  14423.2 |##########
  14691.1 |
  14958.9 |####################
  15226.8 |##########
  15494.7 |
  15762.6 |
  16030.5 |####################
  16298.4 |
  16566.3 |
  16834.2 |
  17102.0 |####################
  17369.9 |
  17637.8 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 10804.0-14956.5 ns)
  10804.0 |########################################
  11011.7 |########################################
  11219.3 |########################################
  11426.9 |
  11634.5 |
  11842.1 |
  12049.8 |####################
  12257.4 |####################
  12465.0 |####################
  12672.6 |####################
  12880.3 |########################################
  13087.9 |
  13295.5 |
  13503.1 |####################
  13710.7 |
  13918.4 |
  14126.0 |
  14333.6 |####################
  14541.2 |####################
  14748.9 |
  (1 below, 2 above range)

kernel_nway (n=18, range 10976.8-15619.2 ns)
  10976.8 |##########################
  11208.9 |
  11441.0 |#############
  11673.1 |
  11905.3 |
  12137.4 |
  12369.5 |#############
  12601.6 |##########################
  12833.7 |#############
  13065.9 |
  13298.0 |#############
  13530.1 |
  13762.2 |
  13994.4 |
  14226.5 |########################################
  14458.6 |##########################
  14690.7 |
  14922.8 |#############
  15155.0 |
  15387.1 |
  (1 below, 3 above range)

kernel_seq (n=18, range 9373.7-14412.1 ns)
   9373.7 |
   9625.6 |####################
   9877.5 |
  10129.4 |####################
  10381.3 |
  10633.3 |########################################
  10885.2 |
  11137.1 |
  11389.0 |########################################
  11640.9 |########################################
  11892.9 |########################################
  12144.8 |
  12396.7 |
  12648.6 |
  12900.5 |
  13152.5 |####################
  13404.4 |########################################
  13656.3 |####################
  13908.2 |
  14160.1 |
  (2 below, 2 above range)

nway (n=18, range 30937.0-43578.5 ns)
  30937.0 |
  31569.1 |########################################
  32201.2 |##########
  32833.3 |##########
  33465.3 |##########
  34097.4 |
  34729.5 |####################
  35361.6 |
  35993.6 |
  36625.7 |
  37257.8 |##########
  37889.8 |##########
  38521.9 |
  39154.0 |##########
  39786.1 |
  40418.1 |##########
  41050.2 |
  41682.3 |
  42314.4 |
  42946.4 |
  (2 below, 3 above range)

sequential (n=18, range 6746.7-25599.0 ns)
   6746.7 |########################################
   7689.3 |
   8631.9 |
   9574.5 |
  10517.1 |
  11459.8 |
  12402.4 |
  13345.0 |
  14287.6 |
  15230.2 |
  16172.8 |
  17115.5 |
  18058.1 |####
  19000.7 |
  19943.3 |####
  20885.9 |
  21828.6 |
  22771.2 |####
  23713.8 |
  24656.4 |####
  (1 below, 3 above range)

std_parallel (n=18, range 41263.0-60915.7 ns)
  41263.0 |
  42245.6 |#############
  43228.3 |
  44210.9 |
  45193.5 |##########################
  46176.2 |#############
  47158.8 |
  48141.4 |#############
  49124.1 |#############
  50106.7 |########################################
  51089.3 |#############
  52072.0 |#############
  53054.6 |#############
  54037.3 |
  55019.9 |
  56002.5 |
  56985.2 |#############
  57967.8 |
  58950.4 |
  59933.1 |#############
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=103.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=102.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=103.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=103.8% of algo (FFI overhead may distort results)
- **nway**: bridge=99.6% of algo (FFI overhead may distort results)
- **sequential**: CV=65.2% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **sequential**: bridge=104.0% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=88.5% of algo (FFI overhead may distort results)

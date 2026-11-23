# fold_light, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 4.3x slower than the field

std_parallel (88.89 us) is 4.3x the fastest (20.77 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 74.2%)

sequential wins on median (20.77 us) yet has the highest variance (CV 74.2%), while nway is the steadiest (CV 12.8%, 52.07 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.59)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.3x the fastest

Fastest sequential (20.77 us) to slowest std_parallel (88.89 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.8x its best-20%

sequential's best 20% of batches run at 12.05 us but its worst 20% at 45.31 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 20769.9 ns median (-24.8% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 4.28x (fastest 20769.9 ns, slowest 88889.6 ns)

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
| Counter frequency | 2112793617 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 29647ns | 28311ns | 23946ns | 27373ns | 35910ns | base |
| head_tail_dyn | 44216ns | 38935ns | 34879ns | 38099ns | 58058ns | +49.14% |
| kernel_nway | 26217ns | 23434ns | 20065ns | 23263ns | 33725ns | -11.57% |
| kernel_seq | 37956ns | 39170ns | 21204ns | 37860ns | 46476ns | +28.02% |
| nway | 55076ns | 52679ns | 50017ns | 51954ns | 62288ns | +85.77% |
| sequential | 27034ns | 21050ns | 12205ns | 19376ns | 45934ns | -8.82% |
| std_parallel | 101712ns | 89145ns | 70116ns | 85195ns | 142285ns | +243.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 28831ns | 23468ns | 34800ns | base | 2.273 |
| head_tail_dyn | 43400ns | 34351ns | 56632ns | +50.53% | 1.510 |
| kernel_nway | 25382ns | 19508ns | 32778ns | -11.96% | 2.582 |
| kernel_seq | 36869ns | 20847ns | 44707ns | +27.88% | 1.778 |
| nway | 54522ns | 49474ns | 61744ns | +89.11% | 1.202 |
| sequential | 26676ns | 12049ns | 45309ns | -7.47% | 2.457 |
| std_parallel | 101175ns | 69876ns | 141174ns | +250.93% | 0.648 |

## Performance model

- Peak throughput: **5.439 Gops/s** (sequential; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 2.371 | 43.6% |
| head_tail_dyn | 1.709 | 31.4% |
| kernel_nway | 2.877 | 52.9% |
| kernel_seq | 1.702 | 31.3% |
| nway | 1.259 | 23.1% |
| sequential | 3.155 | 58.0% |
| std_parallel | 0.737 | 13.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 29647ns | 29647ns | base |
| head_tail_dyn | 44216ns | 44216ns | +49.14% |
| kernel_nway | 26217ns | 26217ns | -11.57% |
| kernel_seq | 37956ns | 37956ns | +28.02% |
| nway | 55076ns | 55076ns | +85.77% |
| sequential | 27034ns | 27034ns | -8.82% |
| std_parallel | 101712ns | 101712ns | +243.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 27637ns | base | --- | [25595, 29205] | --- | --- | --- | --- |
| head_tail_dyn | 38352ns | +12825.1ns (+46.4%) | [+9564, +15538]ns | [36291, 42259] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 22777ns | no significant difference | [-7796, +4265]ns | [20995, 29836] | no | 0.2855 | 0.2379 | 0 |
| kernel_seq | 38502ns | +9737.7ns (+35.2%) | [+6809, +12357]ns | [35798, 40667] | YES | 0.0113 | 0.0075 | 0 |
| nway | 52066ns | +23518.9ns (+85.1%) | [+20894, +30791]ns | [49858, 54541] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 20770ns | no significant difference | [-11822, +4943]ns | [12240, 39802] | no | 0.4807 | 0.4807 | 0 |
| std_parallel | 88890ns | +63569.4ns (+230.0%) | [+50152, +72797]ns | [77090, 104069] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 24883ns | +53.3% | +23.4% | +43.9% | +128.6% | -51.1% | +215.4% |
| 1 | 2 | 27652ns | +49.5% | -26.0% | +48.0% | +80.1% | +6.8% | +316.0% |
| 1 | 3 | 31000ns | +17.5% | -26.5% | +24.3% | +63.1% | +11.3% | +197.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.002 | ok |
| head_tail_dyn | -0.587 | HIGH- (thermal bounce) |
| kernel_nway | -0.303 | moderate- |
| kernel_seq | -0.478 | moderate- |
| nway | -0.224 | moderate- |
| sequential | -0.062 | ok |
| std_parallel | -0.576 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 32318.8ns | 28830.6ns | 112.1% | HIGH |
| head_tail_dyn | 43323.9ns | 43400.0ns | 99.8% | HIGH |
| kernel_nway | 26982.5ns | 25381.8ns | 106.3% | HIGH |
| kernel_seq | 37356.3ns | 36869.2ns | 101.3% | HIGH |
| nway | 55587.1ns | 54521.9ns | 102.0% | HIGH |
| sequential | 27337.1ns | 26676.4ns | 102.5% | HIGH |
| std_parallel | 99166.7ns | 101175.5ns | 98.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 23468.0-34800.3 ns)
  23468.0 |#############
  24034.6 |##########################
  24601.2 |
  25167.8 |##########################
  25734.5 |#############
  26301.1 |#############
  26867.7 |#############
  27434.3 |
  28000.9 |########################################
  28567.5 |#############
  29134.1 |##########################
  29700.8 |
  30267.4 |
  30834.0 |
  31400.6 |
  31967.2 |
  32533.8 |#############
  33100.4 |
  33667.1 |
  34233.7 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 34351.1-56632.2 ns)
  34351.1 |
  35465.2 |########################################
  36579.2 |####################
  37693.3 |##########
  38807.3 |####################
  39921.4 |##########
  41035.4 |##########
  42149.5 |##########
  43263.5 |
  44377.6 |
  45491.6 |
  46605.7 |##########
  47719.7 |
  48833.8 |
  49947.9 |
  51061.9 |
  52176.0 |
  53290.0 |##########
  54404.1 |
  55518.1 |
  (2 below, 2 above range)

kernel_nway (n=18, range 19508.5-32778.3 ns)
  19508.5 |
  20172.0 |####################
  20835.5 |########################################
  21499.0 |
  22162.5 |##########
  22825.9 |##########
  23489.4 |##########
  24152.9 |
  24816.4 |
  25479.9 |
  26143.4 |
  26806.9 |##########
  27470.4 |
  28133.8 |
  28797.3 |
  29460.8 |##########
  30124.3 |####################
  30787.8 |
  31451.3 |##########
  32114.8 |
  (2 below, 2 above range)

kernel_seq (n=18, range 20847.2-44707.3 ns)
  20847.2 |
  22040.2 |
  23233.2 |
  24426.2 |
  25619.2 |
  26812.2 |
  28005.2 |
  29198.2 |
  30391.2 |####################
  31584.2 |
  32777.2 |
  33970.2 |##########
  35163.2 |
  36356.2 |####################
  37549.2 |########################################
  38742.2 |##########
  39935.2 |##########
  41128.3 |##########
  42321.3 |##########
  43514.3 |
  (2 below, 3 above range)

nway (n=18, range 49473.7-61743.5 ns)
  49473.7 |########################################
  50087.2 |################
  50700.7 |########
  51314.2 |
  51927.7 |
  52541.2 |########
  53154.7 |
  53768.2 |################
  54381.7 |################
  54995.1 |
  55608.6 |
  56222.1 |
  56835.6 |
  57449.1 |
  58062.6 |
  58676.1 |########
  59289.6 |
  59903.1 |
  60516.6 |
  61130.0 |
  (1 below, 3 above range)

sequential (n=18, range 12049.0-45308.8 ns)
  12049.0 |########################################
  13712.0 |
  15375.0 |
  17038.0 |
  18701.0 |#####
  20364.0 |
  22027.0 |#####
  23689.9 |
  25352.9 |
  27015.9 |
  28678.9 |
  30341.9 |#####
  32004.9 |
  33667.9 |
  35330.9 |
  36993.8 |#####
  38656.8 |#####
  40319.8 |#####
  41982.8 |###########
  43645.8 |#####
  (1 below, 1 above range)

std_parallel (n=18, range 69875.5-141173.8 ns)
  69875.5 |########################################
  73440.4 |####################
  77005.4 |####################
  80570.3 |####################
  84135.2 |########################################
  87700.1 |####################
  91265.0 |####################
  94829.9 |
  98394.8 |####################
  101959.7 |########################################
  105524.7 |
  109089.6 |
  112654.5 |
  116219.4 |
  119784.3 |
  123349.2 |
  126914.1 |
  130479.0 |
  134044.0 |####################
  137608.9 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=22.6% (high variance, measurements may be unstable)
- **head_tail**: bridge=100.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=29.7% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=101.1% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=23.2% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=105.0% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=24.3% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=101.3% of algo (FFI overhead may distort results)
- **nway**: bridge=102.6% of algo (FFI overhead may distort results)
- **sequential**: CV=57.7% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **sequential**: bridge=113.8% of algo (FFI overhead may distort results)
- **std_parallel**: CV=33.6% (high variance, measurements may be unstable)
- **std_parallel**: bridge=88.9% of algo (FFI overhead may distort results)

# fold_light, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 7.0x slower than the field

std_parallel (93.28 us) is 7.0x the fastest (13.30 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (head_tail_dyn, kernel_seq) are a dead heat (<1%)

head_tail_dyn (13.30 us) and kernel_seq (13.41 us) differ by 0.78%, inside the noise, even though the wider field spreads 601.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### sequential shows alternating (throttle bounce) (autocorr -0.64)

sequential's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 7.0x the fastest

Fastest head_tail_dyn (13.30 us) to slowest std_parallel (93.28 us): 7.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.1x its best-20%

sequential's best 20% of batches run at 11.76 us but its worst 20% at 36.62 us (3.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 13303.5 ns median (-21.1% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 7.01x (fastest 13303.5 ns, slowest 93283.4 ns)

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
| head_tail | 18697ns | 17664ns | 16110ns | 17283ns | 22111ns | base |
| head_tail_dyn | 16953ns | 13860ns | 11962ns | 14296ns | 23435ns | -9.33% |
| kernel_nway | 22983ns | 22096ns | 19051ns | 21812ns | 26706ns | +22.93% |
| kernel_seq | 17122ns | 14289ns | 12734ns | 14251ns | 23622ns | -8.43% |
| nway | 51926ns | 49902ns | 42821ns | 49046ns | 60798ns | +177.73% |
| sequential | 26771ns | 26339ns | 11999ns | 24634ns | 37362ns | +43.18% |
| std_parallel | 105389ns | 93588ns | 72927ns | 87417ns | 148579ns | +463.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 17616ns | 15350ns | 20653ns | base | 0.233 |
| head_tail_dyn | 16031ns | 11488ns | 22084ns | -8.99% | 0.256 |
| kernel_nway | 22012ns | 18456ns | 25505ns | +24.96% | 0.186 |
| kernel_seq | 15934ns | 12073ns | 21940ns | -9.55% | 0.257 |
| nway | 51272ns | 42324ns | 60029ns | +191.06% | 0.080 |
| sequential | 26241ns | 11761ns | 36621ns | +48.96% | 0.156 |
| std_parallel | 104808ns | 72662ns | 147438ns | +494.96% | 0.039 |

## Performance model

- Peak throughput: **0.357 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.243 | 68.1% |
| head_tail_dyn | 0.308 | 86.4% |
| kernel_nway | 0.194 | 54.3% |
| kernel_seq | 0.306 | 85.7% |
| nway | 0.083 | 23.3% |
| sequential | 0.159 | 44.5% |
| std_parallel | 0.044 | 12.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 18697ns | 18697ns | base |
| head_tail_dyn | 16953ns | 16953ns | -9.33% |
| kernel_nway | 22983ns | 22983ns | +22.93% |
| kernel_seq | 17122ns | 17122ns | -8.43% |
| nway | 51926ns | 51926ns | +177.73% |
| sequential | 26771ns | 26771ns | +43.18% |
| std_parallel | 105389ns | 105389ns | +463.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 16867ns | base | --- | [15567, 18997] | --- | --- | --- | --- |
| head_tail_dyn | 13304ns | no significant difference | [-4014, +2610]ns | [12772, 20599] | no | 0.4807 | 0.4807 | 0 |
| kernel_nway | 21160ns | +3965.7ns (+23.5%) | [+3121, +5802]ns | [20233, 23142] | YES | 0.0026 | 0.0013 | 0 |
| kernel_seq | 13407ns | no significant difference | [-3441, +1508]ns | [12808, 21324] | no | 0.2855 | 0.2379 | 0 |
| nway | 49328ns | +32768.9ns (+194.3%) | [+27881, +36187]ns | [47163, 54448] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 25791ns | +9584.7ns (+56.8%) | [+5307, +12845]ns | [20379, 32662] | YES | 0.0113 | 0.0075 | 0 |
| std_parallel | 93283ns | +76439.6ns (+453.2%) | [+59318, +107721]ns | [75098, 126590] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 17606ns | -25.6% | +17.5% | -24.9% | +186.3% | +85.5% | +586.5% |
| 1 | 2 | 15546ns | -23.6% | +30.8% | -22.0% | +233.9% | +37.7% | +383.1% |
| 1 | 3 | 18421ns | +15.0% | +30.0% | +16.9% | +132.2% | +64.6% | +380.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.616 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.241 | moderate- |
| kernel_nway | -0.219 | moderate- |
| kernel_seq | -0.232 | moderate- |
| nway | -0.262 | moderate- |
| sequential | -0.641 | HIGH- (thermal bounce) |
| std_parallel | -0.352 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 19005.1ns | 17615.8ns | 107.9% | HIGH |
| head_tail_dyn | 17177.6ns | 16031.3ns | 107.2% | HIGH |
| kernel_nway | 22876.3ns | 22012.3ns | 103.9% | HIGH |
| kernel_seq | 17339.2ns | 15933.9ns | 108.8% | HIGH |
| nway | 51892.5ns | 51272.2ns | 101.2% | HIGH |
| sequential | 26851.3ns | 26241.0ns | 102.3% | HIGH |
| std_parallel | 105089.5ns | 104807.6ns | 100.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 15350.1-20652.7 ns)
  15350.1 |########################################
  15615.3 |########
  15880.4 |
  16145.5 |
  16410.7 |
  16675.8 |################################
  16940.9 |########
  17206.0 |
  17471.2 |
  17736.3 |
  18001.4 |########
  18266.6 |
  18531.7 |
  18796.8 |
  19062.0 |
  19327.1 |
  19592.2 |
  19857.3 |################
  20122.5 |########
  20387.6 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 11488.3-22083.8 ns)
  11488.3 |####################
  12018.1 |##########
  12547.9 |########################################
  13077.7 |####################
  13607.4 |
  14137.2 |##########
  14667.0 |
  15196.8 |
  15726.5 |
  16256.3 |
  16786.1 |##########
  17315.8 |
  17845.6 |
  18375.4 |
  18905.2 |
  19434.9 |
  19964.7 |
  20494.5 |####################
  21024.3 |####################
  21554.0 |
  (1 below, 2 above range)

kernel_nway (n=18, range 18455.7-25505.1 ns)
  18455.7 |#############
  18808.2 |
  19160.6 |#############
  19513.1 |
  19865.6 |#############
  20218.1 |
  20570.5 |##########################
  20923.0 |########################################
  21275.5 |
  21628.0 |#############
  21980.4 |#############
  22332.9 |
  22685.4 |#############
  23037.8 |
  23390.3 |#############
  23742.8 |
  24095.3 |#############
  24447.7 |
  24800.2 |
  25152.7 |
  (2 below, 3 above range)

kernel_seq (n=18, range 12073.1-21939.6 ns)
  12073.1 |################################
  12566.4 |
  13059.7 |########################################
  13553.1 |########
  14046.4 |########
  14539.7 |
  15033.0 |
  15526.4 |
  16019.7 |
  16513.0 |
  17006.3 |
  17499.7 |
  17993.0 |
  18486.3 |
  18979.7 |
  19473.0 |
  19966.3 |
  20459.6 |
  20953.0 |################
  21446.3 |################
  (1 below, 2 above range)

nway (n=18, range 42324.2-60029.2 ns)
  42324.2 |########################################
  43209.5 |
  44094.7 |
  44980.0 |
  45865.2 |
  46750.5 |########################################
  47635.7 |
  48521.0 |##########################
  49406.2 |#############
  50291.5 |#############
  51176.7 |
  52062.0 |
  52947.2 |##########################
  53832.5 |
  54717.7 |#############
  55603.0 |
  56488.2 |
  57373.5 |
  58258.7 |##########################
  59144.0 |
  (1 below, 2 above range)

sequential (n=18, range 11760.6-36620.7 ns)
  11760.6 |
  13003.6 |
  14246.6 |
  15489.6 |
  16732.6 |####################
  17975.6 |
  19218.6 |########################################
  20461.6 |####################
  21704.6 |########################################
  22947.6 |####################
  24190.6 |
  25433.7 |
  26676.7 |####################
  27919.7 |####################
  29162.7 |
  30405.7 |########################################
  31648.7 |
  32891.7 |####################
  34134.7 |####################
  35377.7 |####################
  (2 below, 2 above range)

std_parallel (n=18, range 72661.7-147438.3 ns)
  72661.7 |########################################
  76400.6 |########
  80139.4 |
  83878.2 |########
  87617.1 |
  91355.9 |################
  95094.7 |
  98833.5 |########
  102572.4 |
  106311.2 |
  110050.0 |########
  113788.9 |
  117527.7 |
  121266.5 |########
  125005.3 |
  128744.2 |########
  132483.0 |
  136221.8 |
  139960.7 |
  143699.5 |########
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=105.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=28.4% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=114.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=104.0% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=27.0% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=110.5% of algo (FFI overhead may distort results)
- **nway**: bridge=101.6% of algo (FFI overhead may distort results)
- **sequential**: CV=35.8% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **sequential**: bridge=102.9% of algo (FFI overhead may distort results)
- **std_parallel**: CV=31.9% (high variance, measurements may be unstable)
- **std_parallel**: bridge=91.8% of algo (FFI overhead may distort results)

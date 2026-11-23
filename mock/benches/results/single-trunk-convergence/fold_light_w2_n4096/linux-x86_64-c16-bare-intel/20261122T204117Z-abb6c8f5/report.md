# fold_light, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 4.1x slower than the field

std_parallel (54.15 us) is 4.1x the fastest (13.26 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.66)

kernel_seq's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.1x the fastest

Fastest head_tail_dyn (13.26 us) to slowest std_parallel (54.15 us): 4.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 2.8x its best-20%

sequential's best 20% of batches run at 13.21 us but its worst 20% at 37.36 us (2.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 13263.8 ns median (-4.6% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 4.08x (fastest 13263.8 ns, slowest 54152.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 2 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112779539 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 16257ns | 14630ns | 13047ns | 14377ns | 20682ns | base |
| head_tail_dyn | 15125ns | 13806ns | 12560ns | 13924ns | 18210ns | -6.96% |
| kernel_nway | 16276ns | 14502ns | 13203ns | 14337ns | 20722ns | +0.12% |
| kernel_seq | 15477ns | 14690ns | 13498ns | 14637ns | 17728ns | -4.80% |
| nway | 51564ns | 49062ns | 40976ns | 47243ns | 63340ns | +217.18% |
| sequential | 28579ns | 28003ns | 13469ns | 27237ns | 38147ns | +75.80% |
| std_parallel | 55604ns | 55351ns | 49414ns | 53759ns | 61468ns | +242.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 15319ns | 12415ns | 19476ns | base | 0.267 |
| head_tail_dyn | 14281ns | 12029ns | 17016ns | -6.77% | 0.287 |
| kernel_nway | 15197ns | 12620ns | 19019ns | -0.79% | 0.270 |
| kernel_seq | 14429ns | 12922ns | 16457ns | -5.80% | 0.284 |
| nway | 50746ns | 40423ns | 62159ns | +231.27% | 0.081 |
| sequential | 27984ns | 13211ns | 37356ns | +82.68% | 0.146 |
| std_parallel | 54792ns | 48900ns | 60359ns | +257.68% | 0.075 |

## Performance model

- Peak throughput: **0.341 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.295 | 86.5% |
| head_tail_dyn | 0.309 | 90.7% |
| kernel_nway | 0.305 | 89.7% |
| kernel_seq | 0.300 | 88.0% |
| nway | 0.085 | 24.9% |
| sequential | 0.150 | 44.1% |
| std_parallel | 0.076 | 22.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 16257ns | 16257ns | base |
| head_tail_dyn | 15125ns | 15125ns | -6.96% |
| kernel_nway | 16276ns | 16276ns | +0.12% |
| kernel_seq | 15477ns | 15477ns | -4.80% |
| nway | 51564ns | 51564ns | +217.18% |
| sequential | 28579ns | 28579ns | +75.80% |
| std_parallel | 55604ns | 55604ns | +242.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 13906ns | base | --- | [13134, 14709] | --- | --- | --- | --- |
| head_tail_dyn | 13264ns | no significant difference | [-1120, +445]ns | [12738, 15205] | no | 0.7210 | 0.4807 | 0 |
| kernel_nway | 13408ns | no significant difference | [-719, +1131]ns | [13088, 15617] | no | 0.8145 | 0.8145 | 0 |
| kernel_seq | 13662ns | no significant difference | [-1446, +1430]ns | [13384, 15662] | no | 0.8145 | 0.8145 | 0 |
| nway | 48308ns | +33316.7ns (+239.6%) | [+29527, +38191]ns | [43319, 53096] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 27286ns | +12409.4ns (+89.2%) | [+10670, +15125]ns | [25046, 33262] | YES | 0.0151 | 0.0075 | 0 |
| std_parallel | 54153ns | +38511.9ns (+276.9%) | [+36184, +43637]ns | [51362, 58171] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 13524ns | -4.9% | -5.4% | -0.8% | +248.8% | +94.5% | +306.8% |
| 1 | 2 | 12568ns | -2.0% | +4.3% | +24.9% | +261.3% | +176.1% | +342.5% |
| 1 | 3 | 15407ns | +0.2% | +5.0% | -14.7% | +255.0% | +77.1% | +232.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.384 | moderate- |
| head_tail_dyn | -0.269 | moderate- |
| kernel_nway | -0.121 | ok |
| kernel_seq | -0.660 | HIGH- (thermal bounce) |
| nway | -0.278 | moderate- |
| sequential | -0.659 | HIGH- (thermal bounce) |
| std_parallel | -0.242 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 14802.8ns | 15318.7ns | 96.6% | HIGH |
| head_tail_dyn | 14505.7ns | 14280.9ns | 101.6% | HIGH |
| kernel_nway | 15389.3ns | 15197.1ns | 101.3% | HIGH |
| kernel_seq | 14938.7ns | 14429.5ns | 103.5% | HIGH |
| nway | 50955.9ns | 50745.6ns | 100.4% | HIGH |
| sequential | 29124.7ns | 27984.4ns | 104.1% | HIGH |
| std_parallel | 54975.3ns | 54792.5ns | 100.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 12414.6-19476.5 ns)
  12414.6 |##############################
  12767.7 |
  13120.8 |##############################
  13473.9 |##########
  13827.0 |########################################
  14180.1 |
  14533.2 |####################
  14886.2 |
  15239.3 |
  15592.4 |
  15945.5 |##########
  16298.6 |
  16651.7 |##########
  17004.8 |
  17357.9 |
  17711.0 |
  18064.1 |
  18417.2 |
  18770.3 |
  19123.4 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 12028.6-17016.2 ns)
  12028.6 |########################################
  12277.9 |####################
  12527.3 |########################################
  12776.7 |########################################
  13026.1 |####################
  13275.5 |####################
  13524.9 |####################
  13774.2 |
  14023.6 |
  14273.0 |
  14522.4 |
  14771.8 |####################
  15021.1 |####################
  15270.5 |####################
  15519.9 |########################################
  15769.3 |
  16018.7 |
  16268.1 |
  16517.4 |
  16766.8 |
  (1 below, 2 above range)

kernel_nway (n=18, range 12620.0-19018.9 ns)
  12620.0 |##############################
  12939.9 |########################################
  13259.9 |####################
  13579.8 |
  13899.8 |##########
  14219.7 |
  14539.7 |
  14859.6 |
  15179.6 |##########
  15499.5 |####################
  15819.4 |
  16139.4 |
  16459.3 |##########
  16779.3 |
  17099.2 |
  17419.2 |
  17739.1 |##########
  18059.1 |##########
  18379.0 |
  18698.9 |
  (1 below, 1 above range)

kernel_seq (n=18, range 12921.5-16457.3 ns)
  12921.5 |
  13098.3 |####################
  13275.1 |########################################
  13451.9 |
  13628.7 |####################
  13805.4 |##########
  13982.2 |
  14159.0 |##########
  14335.8 |
  14512.6 |
  14689.4 |
  14866.2 |
  15043.0 |
  15219.8 |
  15396.5 |
  15573.3 |########################################
  15750.1 |
  15926.9 |
  16103.7 |
  16280.5 |
  (2 below, 2 above range)

nway (n=18, range 40423.4-62159.4 ns)
  40423.4 |########################################
  41510.2 |####################
  42597.0 |####################
  43683.8 |########################################
  44770.6 |
  45857.4 |
  46944.2 |########################################
  48031.0 |####################
  49117.8 |
  50204.6 |####################
  51291.4 |####################
  52378.2 |########################################
  53465.0 |
  54551.8 |
  55638.6 |####################
  56725.4 |
  57812.2 |
  58899.0 |
  59985.8 |
  61072.6 |
  (1 below, 3 above range)

sequential (n=18, range 13210.7-37355.6 ns)
  13210.7 |
  14417.9 |
  15625.2 |
  16832.4 |
  18039.7 |
  19246.9 |
  20454.2 |
  21661.4 |
  22868.6 |##########################
  24075.9 |#############
  25283.1 |##########################
  26490.4 |########################################
  27697.6 |
  28904.9 |##########################
  30112.1 |
  31319.4 |
  32526.6 |########################################
  33733.8 |
  34941.1 |#############
  36148.3 |
  (2 below, 2 above range)

std_parallel (n=18, range 48900.2-60358.9 ns)
  48900.2 |
  49473.2 |
  50046.1 |#############
  50619.0 |#############
  51192.0 |########################################
  51764.9 |
  52337.8 |#############
  52910.8 |
  53483.7 |#############
  54056.6 |#############
  54629.6 |
  55202.5 |#############
  55775.4 |#############
  56348.4 |
  56921.3 |
  57494.2 |#############
  58067.2 |##########################
  58640.1 |
  59213.0 |#############
  59786.0 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=32.6% (high variance, measurements may be unstable)
- **head_tail**: bridge=102.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.3% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=26.5% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=102.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=102.6% of algo (FFI overhead may distort results)
- **nway**: bridge=103.3% of algo (FFI overhead may distort results)
- **sequential**: CV=33.1% (high variance, measurements may be unstable)
- **sequential**: bridge=105.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=97.8% of algo (FFI overhead may distort results)

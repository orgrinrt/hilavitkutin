# fold_light, short fiber (1 to 4 morsels), W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 64% faster than the next best (head_tail)

kernel_seq (2.52 us) leads head_tail (4.13 us) by 64%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 39% (significant)

kernel_seq is -1.59 us (39%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 33.2x slower than the field

std_parallel (83.55 us) is 33.2x the fastest (2.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.64)

kernel_nway's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 33.2x the fastest

Fastest kernel_seq (2.52 us) to slowest std_parallel (83.55 us): 33.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2519.2 ns median (-39.0% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 33.16x (fastest 2519.2 ns, slowest 83545.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 8 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 4726ns | 4768ns | 4542ns | 4719ns | 4828ns | base |
| head_tail_dyn | 4981ns | 4835ns | 4571ns | 4795ns | 5465ns | +5.41% |
| kernel_nway | 16660ns | 16356ns | 14691ns | 16020ns | 18606ns | +252.55% |
| kernel_seq | 2976ns | 2989ns | 2786ns | 2940ns | 3125ns | -37.03% |
| nway | 50941ns | 50902ns | 48263ns | 50631ns | 52746ns | +977.98% |
| sequential | 10612ns | 10373ns | 10306ns | 10374ns | 11123ns | +124.56% |
| std_parallel | 83260ns | 83886ns | 75224ns | 82043ns | 89104ns | +1661.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4118ns | 3939ns | 4230ns | base | 0.004 |
| head_tail_dyn | 4356ns | 3964ns | 4856ns | +5.78% | 0.004 |
| kernel_nway | 16034ns | 14105ns | 17966ns | +289.35% | 0.001 |
| kernel_seq | 2547ns | 2451ns | 2645ns | -38.16% | 0.006 |
| nway | 50382ns | 47716ns | 52171ns | +1123.40% | 0.000 |
| sequential | 10470ns | 10163ns | 10977ns | +154.24% | 0.002 |
| std_parallel | 82881ns | 74841ns | 88736ns | +1912.53% | 0.000 |

## Performance model

- Peak throughput: **0.007 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.004 | 59.3% |
| head_tail_dyn | 0.004 | 58.6% |
| kernel_nway | 0.001 | 15.6% |
| kernel_seq | 0.006 | 97.3% |
| nway | 0.000 | 4.9% |
| sequential | 0.002 | 24.0% |
| std_parallel | 0.000 | 2.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4726ns | 4726ns | base |
| head_tail_dyn | 4981ns | 4981ns | +5.41% |
| kernel_nway | 16660ns | 16660ns | +252.55% |
| kernel_seq | 2976ns | 2976ns | -37.03% |
| nway | 50941ns | 50941ns | +977.98% |
| sequential | 10612ns | 10612ns | +124.56% |
| std_parallel | 83260ns | 83260ns | +1661.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4132ns | base | --- | [4048, 4212] | --- | --- | --- | --- |
| head_tail_dyn | 4181ns | no significant difference | [-96, +525]ns | [4062, 4688] | no | 0.4807 | 0.4807 | 0 |
| kernel_nway | 15716ns | +11687.5ns (+282.8%) | [+10857, +12469]ns | [14922, 16668] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2519ns | -1594.2ns (-38.6%) | [-1649, -1522]ns | [2488, 2602] | YES | 0.0000 | 0.0000 | 0 |
| nway | 50358ns | +46264.2ns (+1119.5%) | [+45573, +47106]ns | [49587, 51314] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10227ns | +6146.7ns (+148.7%) | [+6050, +6477]ns | [10192, 10538] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 83545ns | +79554.1ns (+1925.1%) | [+75016, +81347]ns | [79062, 85278] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4207ns | +16.0% | +278.3% | -37.6% | +1091.0% | +146.7% | +1914.7% |
| 1 | 2 | 4102ns | -1.5% | +280.1% | -38.8% | +1121.4% | +148.1% | +1896.0% |
| 1 | 3 | 4129ns | -0.4% | +287.3% | -39.5% | +1133.1% | +155.2% | +1923.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.329 | moderate- |
| head_tail_dyn | -0.212 | moderate- |
| kernel_nway | -0.644 | HIGH- (thermal bounce) |
| kernel_seq | -0.121 | ok |
| nway | -0.170 | ok |
| sequential | -0.533 | HIGH- (thermal bounce) |
| std_parallel | -0.551 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 97691.1ns | 4118.2ns | 2372.2% | HIGH |
| head_tail_dyn | 95379.6ns | 4356.2ns | 2189.5% | HIGH |
| kernel_nway | 110304.4ns | 16034.5ns | 687.9% | HIGH |
| kernel_seq | 92208.5ns | 2546.9ns | 3620.5% | HIGH |
| nway | 111407.5ns | 50382.4ns | 221.1% | HIGH |
| sequential | 91738.0ns | 10470.3ns | 876.2% | HIGH |
| std_parallel | 132844.3ns | 82881.1ns | 160.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 3938.9-4229.7 ns)
   3938.9 |
   3953.4 |####################
   3968.0 |
   3982.5 |
   3997.1 |
   4011.6 |########################################
   4026.1 |####################
   4040.7 |
   4055.2 |
   4069.8 |####################
   4084.3 |####################
   4098.8 |####################
   4113.4 |####################
   4127.9 |
   4142.5 |########################################
   4157.0 |
   4171.6 |
   4186.1 |####################
   4200.6 |########################################
   4215.2 |########################################
  (1 below, 2 above range)

head_tail_dyn (n=18, range 3963.9-4855.6 ns)
   3963.9 |
   4008.5 |########################################
   4053.1 |#############
   4097.6 |########################################
   4142.2 |
   4186.8 |
   4231.4 |#############
   4276.0 |#############
   4320.6 |
   4365.1 |
   4409.7 |#############
   4454.3 |
   4498.9 |
   4543.5 |
   4588.1 |#############
   4632.6 |
   4677.2 |
   4721.8 |#############
   4766.4 |
   4811.0 |
  (2 below, 4 above range)

kernel_nway (n=18, range 14105.0-17965.8 ns)
  14105.0 |##########################
  14298.0 |
  14491.1 |
  14684.1 |##########################
  14877.2 |#############
  15070.2 |#############
  15263.3 |
  15456.3 |#############
  15649.3 |########################################
  15842.4 |
  16035.4 |#############
  16228.5 |
  16421.5 |#############
  16614.6 |
  16807.6 |#############
  17000.6 |
  17193.7 |
  17386.7 |#############
  17579.8 |
  17772.8 |
  (1 below, 3 above range)

kernel_seq (n=18, range 2451.1-2645.3 ns)
   2451.1 |
   2460.8 |####################
   2470.5 |####################
   2480.3 |########################################
   2490.0 |
   2499.7 |####################
   2509.4 |########################################
   2519.1 |####################
   2528.8 |####################
   2538.5 |
   2548.2 |
   2557.9 |
   2567.6 |
   2577.3 |####################
   2587.0 |####################
   2596.7 |
   2606.5 |####################
   2616.2 |####################
   2625.9 |####################
   2635.6 |####################
  (2 below, 1 above range)

nway (n=18, range 47715.6-52170.6 ns)
  47715.6 |
  47938.3 |
  48161.1 |
  48383.8 |
  48606.6 |##########################
  48829.3 |
  49052.1 |
  49274.8 |#############
  49497.6 |##########################
  49720.3 |
  49943.1 |########################################
  50165.8 |
  50388.6 |#############
  50611.3 |##########################
  50834.1 |
  51056.8 |#############
  51279.6 |
  51502.3 |
  51725.1 |##########################
  51947.8 |
  (1 below, 3 above range)

sequential (n=18, range 10162.8-10976.9 ns)
  10162.8 |################################
  10203.5 |########################################
  10244.2 |
  10284.9 |
  10325.6 |
  10366.3 |
  10407.0 |########
  10447.7 |
  10488.4 |########
  10529.2 |########
  10569.9 |
  10610.6 |########
  10651.3 |
  10692.0 |
  10732.7 |
  10773.4 |
  10814.1 |
  10854.8 |
  10895.5 |
  10936.2 |
  (2 below, 3 above range)

std_parallel (n=18, range 74841.1-88735.5 ns)
  74841.1 |
  75535.8 |#############
  76230.5 |#############
  76925.3 |#############
  77620.0 |#############
  78314.7 |
  79009.4 |
  79704.2 |#############
  80398.9 |#############
  81093.6 |
  81788.3 |
  82483.0 |
  83177.8 |########################################
  83872.5 |#############
  84567.2 |##########################
  85261.9 |#############
  85956.6 |
  86651.4 |
  87346.1 |
  88040.8 |
  (1 below, 4 above range)

```

## Diagnostics

- **head_tail**: bridge=2361.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2293.3% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=702.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3634.2% of algo (FFI overhead may distort results)
- **nway**: bridge=222.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=897.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=156.9% of algo (FFI overhead may distort results)

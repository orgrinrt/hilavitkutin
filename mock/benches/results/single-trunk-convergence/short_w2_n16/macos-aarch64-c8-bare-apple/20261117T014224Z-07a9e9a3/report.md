# fold_light, short fiber (1 to 4 morsels), W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 51% faster than the next best (head_tail_dyn)

kernel_seq (2.72 us) leads head_tail_dyn (4.11 us) by 51%, a clear separation rather than a photo finish. CV 8.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 36% (significant)

kernel_seq is -1.50 us (36%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 11.1x slower than the field

std_parallel (30.36 us) is 11.1x the fastest (2.72 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.55)

nway's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 11.1x the fastest

Fastest kernel_seq (2.72 us) to slowest std_parallel (30.36 us): 11.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2723.3 ns median (-35.6% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 11.15x (fastest 2723.3 ns, slowest 30364.2 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 4889ns | 4839ns | 4617ns | 4785ns | 5180ns | base |
| head_tail_dyn | 4883ns | 4712ns | 4598ns | 4689ns | 5317ns | -0.11% |
| kernel_nway | 5134ns | 5066ns | 4636ns | 5001ns | 5581ns | +5.01% |
| kernel_seq | 3259ns | 3252ns | 2964ns | 3192ns | 3505ns | -33.35% |
| nway | 18675ns | 18572ns | 18025ns | 18489ns | 19280ns | +282.00% |
| sequential | 10483ns | 10383ns | 10001ns | 10341ns | 10938ns | +114.43% |
| std_parallel | 31631ns | 30746ns | 28804ns | 30565ns | 34644ns | +547.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4275ns | 4009ns | 4563ns | base | 0.004 |
| head_tail_dyn | 4215ns | 3988ns | 4531ns | -1.40% | 0.004 |
| kernel_nway | 4532ns | 4039ns | 4998ns | +6.02% | 0.004 |
| kernel_seq | 2766ns | 2544ns | 3016ns | -35.29% | 0.006 |
| nway | 18177ns | 17527ns | 18766ns | +325.22% | 0.001 |
| sequential | 10339ns | 9862ns | 10794ns | +141.86% | 0.002 |
| std_parallel | 31246ns | 28412ns | 34264ns | +630.95% | 0.001 |

## Performance model

- Peak throughput: **0.006 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.004 | 60.2% |
| head_tail_dyn | 0.004 | 61.9% |
| kernel_nway | 0.004 | 56.7% |
| kernel_seq | 0.006 | 93.4% |
| nway | 0.001 | 14.1% |
| sequential | 0.002 | 24.9% |
| std_parallel | 0.001 | 8.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4889ns | 4889ns | base |
| head_tail_dyn | 4883ns | 4883ns | -0.11% |
| kernel_nway | 5134ns | 5134ns | +5.01% |
| kernel_seq | 3259ns | 3259ns | -33.35% |
| nway | 18675ns | 18675ns | +282.00% |
| sequential | 10483ns | 10483ns | +114.43% |
| std_parallel | 31631ns | 31631ns | +547.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4229ns | base | --- | [4073, 4377] | --- | --- | --- | --- |
| head_tail_dyn | 4108ns | no significant difference | [-151, +39]ns | [4043, 4296] | no | 0.4807 | 0.4807 | 0 |
| kernel_nway | 4488ns | no significant difference | [-252, +586]ns | [4192, 4633] | no | 0.4807 | 0.4807 | 0 |
| kernel_seq | 2723ns | -1505.0ns (-35.6%) | [-1613, -1338]ns | [2608, 2812] | YES | 0.0000 | 0.0000 | 0 |
| nway | 18085ns | +13728.4ns (+324.6%) | [+13548, +14227]ns | [17728, 18640] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10233ns | +5976.7ns (+141.3%) | [+5840, +6189]ns | [10109, 10440] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 30364ns | +25977.6ns (+614.2%) | [+25612, +26936]ns | [29820, 31408] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4018ns | +0.6% | +18.2% | -35.1% | +337.5% | +153.8% | +734.2% |
| 1 | 2 | 4257ns | -3.9% | +5.4% | -34.2% | +338.6% | +134.2% | +602.6% |
| 1 | 3 | 4436ns | -1.3% | -7.4% | -38.3% | +306.5% | +135.0% | +584.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.004 | ok |
| head_tail_dyn | -0.097 | ok |
| kernel_nway | -0.008 | ok |
| kernel_seq | -0.384 | moderate- |
| nway | -0.553 | HIGH- (thermal bounce) |
| sequential | -0.506 | HIGH- (thermal bounce) |
| std_parallel | -0.236 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 97412.0ns | 4274.8ns | 2278.8% | HIGH |
| head_tail_dyn | 99435.7ns | 4215.1ns | 2359.0% | HIGH |
| kernel_nway | 98361.5ns | 4532.0ns | 2170.4% | HIGH |
| kernel_seq | 92573.4ns | 2766.1ns | 3346.7% | HIGH |
| nway | 94325.2ns | 18177.1ns | 518.9% | HIGH |
| sequential | 90690.4ns | 10339.1ns | 877.2% | HIGH |
| std_parallel | 104663.9ns | 31246.5ns | 335.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 4009.4-4563.3 ns)
   4009.4 |########################################
   4037.1 |
   4064.8 |
   4092.5 |##########
   4120.2 |##########
   4147.9 |##########
   4175.6 |##########
   4203.3 |
   4231.0 |
   4258.7 |##########
   4286.4 |##########
   4314.1 |##########
   4341.8 |##########
   4369.5 |
   4397.2 |##########
   4424.9 |
   4452.6 |##########
   4480.2 |##########
   4507.9 |
   4535.6 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 3987.8-4530.8 ns)
   3987.8 |#############
   4014.9 |########################################
   4042.1 |##########################
   4069.2 |#############
   4096.4 |########################################
   4123.5 |#############
   4150.7 |
   4177.8 |
   4205.0 |
   4232.1 |
   4259.3 |#############
   4286.5 |
   4313.6 |#############
   4340.8 |#############
   4367.9 |
   4395.1 |#############
   4422.2 |
   4449.4 |
   4476.5 |
   4503.7 |
  (1 below, 2 above range)

kernel_nway (n=18, range 4039.4-4997.8 ns)
   4039.4 |
   4087.4 |########################################
   4135.3 |####################
   4183.2 |####################
   4231.1 |
   4279.0 |
   4326.9 |
   4374.9 |####################
   4422.8 |########################################
   4470.7 |####################
   4518.6 |####################
   4566.5 |####################
   4614.5 |########################################
   4662.4 |
   4710.3 |####################
   4758.2 |####################
   4806.1 |
   4854.0 |
   4902.0 |
   4949.9 |
  (2 below, 2 above range)

kernel_seq (n=18, range 2543.9-3016.1 ns)
   2543.9 |
   2567.5 |
   2591.1 |########################################
   2614.7 |########################################
   2638.3 |
   2662.0 |
   2685.6 |
   2709.2 |##########################
   2732.8 |
   2756.4 |#############
   2780.0 |#############
   2803.6 |##########################
   2827.2 |
   2850.8 |
   2874.5 |
   2898.1 |##########################
   2921.7 |
   2945.3 |
   2968.9 |
   2992.5 |
  (2 below, 2 above range)

nway (n=18, range 17527.2-18765.8 ns)
  17527.2 |########################################
  17589.2 |########################################
  17651.1 |
  17713.0 |
  17775.0 |####################
  17836.9 |
  17898.8 |####################
  17960.7 |####################
  18022.7 |####################
  18084.6 |####################
  18146.5 |
  18208.5 |
  18270.4 |####################
  18332.3 |
  18394.3 |
  18456.2 |
  18518.1 |####################
  18580.0 |####################
  18642.0 |####################
  18703.9 |####################
  (1 below, 3 above range)

sequential (n=18, range 9861.7-10793.6 ns)
   9861.7 |#############
   9908.3 |
   9954.9 |
  10001.5 |
  10048.1 |##########################
  10094.6 |
  10141.2 |########################################
  10187.8 |#############
  10234.4 |##########################
  10281.0 |
  10327.6 |
  10374.2 |#############
  10420.8 |##########################
  10467.4 |#############
  10514.0 |
  10560.6 |
  10607.2 |
  10653.8 |#############
  10700.4 |
  10747.0 |
  (2 below, 2 above range)

std_parallel (n=18, range 28411.6-34264.4 ns)
  28411.6 |
  28704.3 |
  28996.9 |##########
  29289.6 |
  29582.2 |########################################
  29874.8 |##########
  30167.5 |##############################
  30460.1 |####################
  30752.8 |
  31045.4 |##########
  31338.0 |
  31630.7 |##########
  31923.3 |
  32216.0 |
  32508.6 |
  32801.2 |
  33093.9 |
  33386.5 |####################
  33679.2 |
  33971.8 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=2295.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2416.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=2184.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3377.4% of algo (FFI overhead may distort results)
- **nway**: bridge=519.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=887.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=341.6% of algo (FFI overhead may distort results)

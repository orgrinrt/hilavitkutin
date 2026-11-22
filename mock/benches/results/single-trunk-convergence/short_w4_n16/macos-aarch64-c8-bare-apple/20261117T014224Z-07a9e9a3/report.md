# fold_light, short fiber (1 to 4 morsels), W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 58% faster than the next best (head_tail_dyn)

kernel_seq (2.60 us) leads head_tail_dyn (4.13 us) by 58%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 38% (significant)

kernel_seq is -1.61 us (38%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 21.2x slower than the field

std_parallel (55.09 us) is 21.2x the fastest (2.60 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.65)

kernel_seq's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 21.2x the fastest

Fastest kernel_seq (2.60 us) to slowest std_parallel (55.09 us): 21.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2603.3 ns median (-38.1% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 21.16x (fastest 2603.3 ns, slowest 55093.3 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 4919ns | 4829ns | 4629ns | 4785ns | 5265ns | base |
| head_tail_dyn | 4916ns | 4749ns | 4598ns | 4702ns | 5398ns | -0.05% |
| kernel_nway | 10512ns | 10490ns | 8573ns | 9971ns | 12293ns | +113.71% |
| kernel_seq | 2934ns | 2924ns | 2753ns | 2881ns | 3103ns | -40.36% |
| nway | 33869ns | 34094ns | 31390ns | 33678ns | 35395ns | +588.52% |
| sequential | 10443ns | 10403ns | 9795ns | 10199ns | 11133ns | +112.30% |
| std_parallel | 55713ns | 55521ns | 50103ns | 55131ns | 59390ns | +1032.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4314ns | 4036ns | 4654ns | base | 0.004 |
| head_tail_dyn | 4316ns | 4018ns | 4802ns | +0.06% | 0.004 |
| kernel_nway | 9896ns | 7963ns | 11657ns | +129.42% | 0.002 |
| kernel_seq | 2596ns | 2461ns | 2694ns | -39.81% | 0.006 |
| nway | 33321ns | 30834ns | 34859ns | +672.47% | 0.000 |
| sequential | 10296ns | 9667ns | 10970ns | +138.69% | 0.002 |
| std_parallel | 55299ns | 49693ns | 58973ns | +1181.96% | 0.000 |

## Performance model

- Peak throughput: **0.007 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.004 | 58.5% |
| head_tail_dyn | 0.004 | 59.7% |
| kernel_nway | 0.002 | 24.9% |
| kernel_seq | 0.006 | 94.5% |
| nway | 0.000 | 7.3% |
| sequential | 0.002 | 24.0% |
| std_parallel | 0.000 | 4.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4919ns | 4919ns | base |
| head_tail_dyn | 4916ns | 4916ns | -0.05% |
| kernel_nway | 10512ns | 10512ns | +113.71% |
| kernel_seq | 2934ns | 2934ns | -40.36% |
| nway | 33869ns | 33869ns | +588.52% |
| sequential | 10443ns | 10443ns | +112.30% |
| std_parallel | 55713ns | 55713ns | +1032.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4207ns | base | --- | [4119, 4308] | --- | --- | --- | --- |
| head_tail_dyn | 4126ns | no significant difference | [-190, +354]ns | [4046, 4636] | no | 0.4807 | 0.4807 | 0 |
| kernel_nway | 9902ns | +5632.5ns (+133.9%) | [+4040, +7053]ns | [8462, 11266] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2603ns | -1614.2ns (-38.4%) | [-1783, -1509]ns | [2538, 2661] | YES | 0.0000 | 0.0000 | 0 |
| nway | 33550ns | +29079.2ns (+691.3%) | [+27953, +30150]ns | [32322, 34363] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10256ns | +6085.9ns (+144.7%) | [+5653, +6272]ns | [9764, 10544] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 55093ns | +50819.2ns (+1208.1%) | [+49443, +53652]ns | [53522, 57770] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4122ns | -1.9% | +97.0% | -36.1% | +730.7% | +134.6% | +1198.6% |
| 1 | 2 | 4321ns | -4.5% | +109.3% | -42.4% | +671.0% | +152.7% | +1163.4% |
| 1 | 3 | 4163ns | +14.5% | +170.6% | -36.1% | +683.1% | +147.0% | +1316.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.640 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.111 | ok |
| kernel_nway | -0.036 | ok |
| kernel_seq | -0.651 | HIGH- (thermal bounce) |
| nway | -0.004 | ok |
| sequential | -0.507 | HIGH- (thermal bounce) |
| std_parallel | -0.073 | ok |

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
| head_tail | 96275.8ns | 4313.6ns | 2231.9% | HIGH |
| head_tail_dyn | 96570.3ns | 4316.1ns | 2237.4% | HIGH |
| kernel_nway | 121634.6ns | 9896.3ns | 1229.1% | HIGH |
| kernel_seq | 89710.8ns | 2596.2ns | 3455.5% | HIGH |
| nway | 116678.4ns | 33321.1ns | 350.2% | HIGH |
| sequential | 90273.1ns | 10296.2ns | 876.8% | HIGH |
| std_parallel | 120543.0ns | 55298.8ns | 218.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 4036.1-4653.6 ns)
   4036.1 |#############
   4067.0 |#############
   4097.9 |########################################
   4128.7 |
   4159.6 |#############
   4190.5 |########################################
   4221.4 |#############
   4252.2 |#############
   4283.1 |#############
   4314.0 |##########################
   4344.9 |
   4375.7 |
   4406.6 |
   4437.5 |
   4468.4 |
   4499.2 |
   4530.1 |
   4561.0 |
   4591.9 |
   4622.7 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 4017.8-4802.2 ns)
   4017.8 |########################################
   4057.0 |
   4096.2 |##########################
   4135.5 |######
   4174.7 |
   4213.9 |
   4253.1 |
   4292.3 |
   4331.6 |
   4370.8 |
   4410.0 |
   4449.2 |
   4488.4 |
   4527.7 |
   4566.9 |
   4606.1 |######
   4645.3 |######
   4684.6 |
   4723.8 |######
   4763.0 |######
  (1 below, 2 above range)

kernel_nway (n=18, range 7962.8-11656.7 ns)
   7962.8 |########################################
   8147.5 |#############
   8332.2 |
   8516.9 |#############
   8701.5 |##########################
   8886.2 |
   9070.9 |
   9255.6 |#############
   9440.3 |
   9625.0 |
   9809.7 |
   9994.4 |
  10179.1 |
  10363.8 |#############
  10548.5 |
  10733.2 |#############
  10917.9 |
  11102.6 |##########################
  11287.3 |##########################
  11472.0 |#############
  (1 below, 2 above range)

kernel_seq (n=18, range 2461.1-2694.2 ns)
   2461.1 |
   2472.8 |####################
   2484.4 |####################
   2496.1 |
   2507.7 |
   2519.4 |
   2531.0 |########################################
   2542.7 |
   2554.3 |####################
   2566.0 |
   2577.6 |####################
   2589.3 |####################
   2600.9 |
   2612.6 |####################
   2624.2 |####################
   2635.9 |####################
   2647.5 |####################
   2659.2 |####################
   2670.8 |########################################
   2682.5 |
  (2 below, 2 above range)

nway (n=18, range 30834.4-34859.2 ns)
  30834.4 |
  31035.7 |#############
  31236.9 |
  31438.1 |#############
  31639.4 |
  31840.6 |
  32041.9 |##########################
  32243.1 |#############
  32444.3 |
  32645.6 |#############
  32846.8 |
  33048.0 |#############
  33249.3 |
  33450.5 |##########################
  33651.7 |
  33853.0 |
  34054.2 |#############
  34255.5 |########################################
  34456.7 |#############
  34657.9 |#############
  (1 below, 2 above range)

sequential (n=18, range 9667.2-10970.3 ns)
   9667.2 |########################################
   9732.4 |
   9797.5 |#############
   9862.7 |
   9927.8 |#############
   9993.0 |
  10058.1 |
  10123.3 |
  10188.5 |##########################
  10253.6 |########################################
  10318.8 |
  10383.9 |#############
  10449.1 |
  10514.2 |
  10579.4 |#############
  10644.5 |
  10709.7 |
  10774.8 |
  10840.0 |
  10905.1 |##########################
  (2 below, 2 above range)

std_parallel (n=18, range 49693.4-58973.3 ns)
  49693.4 |
  50157.4 |
  50621.4 |
  51085.4 |
  51549.4 |########################################
  52013.4 |
  52477.4 |
  52941.4 |#############
  53405.4 |
  53869.4 |##########################
  54333.3 |#############
  54797.3 |#############
  55261.3 |#############
  55725.3 |
  56189.3 |
  56653.3 |
  57117.3 |##########################
  57581.3 |##########################
  58045.3 |
  58509.3 |#############
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=2264.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2351.4% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1216.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3437.6% of algo (FFI overhead may distort results)
- **nway**: bridge=344.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=879.7% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=214.3% of algo (FFI overhead may distort results)

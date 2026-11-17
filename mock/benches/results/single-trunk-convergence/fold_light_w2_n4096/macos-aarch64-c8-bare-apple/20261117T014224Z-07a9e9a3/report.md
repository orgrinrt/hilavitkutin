# fold_light, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 57% faster than the next best (kernel_nway)

kernel_seq (2.80 us) leads kernel_nway (4.40 us) by 57%, a clear separation rather than a photo finish. CV 11.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 34% (significant)

kernel_seq is -1.50 us (34%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 12.6x slower than the field

std_parallel (35.25 us) is 12.6x the fastest (2.80 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.67)

kernel_nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 12.6x the fastest

Fastest kernel_seq (2.80 us) to slowest std_parallel (35.25 us): 12.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2800.0 ns median (-37.5% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 12.59x (fastest 2800.0 ns, slowest 35254.2 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 5105ns | 5062ns | 4835ns | 5023ns | 5363ns | base |
| head_tail_dyn | 5048ns | 5062ns | 4751ns | 4993ns | 5279ns | -1.11% |
| kernel_nway | 5057ns | 4985ns | 4700ns | 4968ns | 5369ns | -0.94% |
| kernel_seq | 3486ns | 3329ns | 2986ns | 3239ns | 4107ns | -31.71% |
| nway | 18795ns | 18710ns | 17517ns | 18552ns | 19798ns | +268.18% |
| sequential | 10367ns | 10138ns | 10117ns | 10137ns | 10837ns | +103.08% |
| std_parallel | 38545ns | 35667ns | 32178ns | 35620ns | 46115ns | +655.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4462ns | 4208ns | 4656ns | base | 0.918 |
| head_tail_dyn | 4450ns | 4186ns | 4672ns | -0.27% | 0.920 |
| kernel_nway | 4440ns | 4114ns | 4719ns | -0.51% | 0.923 |
| kernel_seq | 2990ns | 2726ns | 3414ns | -32.99% | 1.370 |
| nway | 18266ns | 17065ns | 19272ns | +309.35% | 0.224 |
| sequential | 10215ns | 9974ns | 10681ns | +128.92% | 0.401 |
| std_parallel | 38152ns | 31775ns | 45725ns | +754.99% | 0.107 |

## Performance model

- Peak throughput: **1.502 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.914 | 60.9% |
| head_tail_dyn | 0.925 | 61.6% |
| kernel_nway | 0.931 | 62.0% |
| kernel_seq | 1.463 | 97.4% |
| nway | 0.226 | 15.0% |
| sequential | 0.410 | 27.3% |
| std_parallel | 0.116 | 7.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 5105ns | 5105ns | base |
| head_tail_dyn | 5048ns | 5048ns | -1.11% |
| kernel_nway | 5057ns | 5057ns | -0.94% |
| kernel_seq | 3486ns | 3486ns | -31.71% |
| nway | 18795ns | 18795ns | +268.18% |
| sequential | 10367ns | 10367ns | +103.08% |
| std_parallel | 38545ns | 38545ns | +655.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4479ns | base | --- | [4333, 4521] | --- | --- | --- | --- |
| head_tail_dyn | 4427ns | no significant difference | [-223, +188]ns | [4321, 4583] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 4398ns | no significant difference | [-206, +117]ns | [4312, 4488] | no | 0.8145 | 0.8145 | 0 |
| kernel_seq | 2800ns | -1504.2ns (-33.6%) | [-1727, -1317]ns | [2775, 3144] | YES | 0.0000 | 0.0000 | 0 |
| nway | 18158ns | +13710.4ns (+306.1%) | [+13273, +13927]ns | [17794, 18392] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 9985ns | +5650.1ns (+126.1%) | [+5485, +6067]ns | [9981, 10583] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 35254ns | +30462.5ns (+680.1%) | [+29229, +38567]ns | [33606, 43025] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4454ns | -1.8% | -2.5% | -23.8% | +307.9% | +124.2% | +646.4% |
| 1 | 2 | 4554ns | -5.7% | +2.0% | -38.9% | +312.1% | +119.1% | +652.2% |
| 1 | 3 | 4435ns | +4.0% | -2.1% | -37.3% | +293.5% | +141.1% | +870.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.652 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.318 | moderate- |
| kernel_nway | -0.667 | HIGH- (thermal bounce) |
| kernel_seq | -0.165 | ok |
| nway | -0.471 | moderate- |
| sequential | -0.173 | ok |
| std_parallel | -0.116 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 93188.2ns | 4462.3ns | 2088.4% | HIGH |
| head_tail_dyn | 94660.6ns | 4450.2ns | 2127.1% | HIGH |
| kernel_nway | 96502.5ns | 4439.6ns | 2173.7% | HIGH |
| kernel_seq | 92694.0ns | 2990.3ns | 3099.8% | HIGH |
| nway | 97695.6ns | 18266.4ns | 534.8% | HIGH |
| sequential | 90486.6ns | 10215.0ns | 885.8% | HIGH |
| std_parallel | 112218.1ns | 38151.8ns | 294.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 4208.3-4656.2 ns)
   4208.3 |
   4230.7 |#############
   4253.1 |
   4275.5 |##########################
   4297.9 |
   4320.3 |#############
   4342.7 |#############
   4365.1 |
   4387.5 |#############
   4409.9 |#############
   4432.3 |
   4454.7 |#############
   4477.1 |#############
   4499.5 |########################################
   4521.9 |#############
   4544.3 |
   4566.7 |
   4589.1 |##########################
   4611.5 |
   4633.9 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 4186.1-4671.5 ns)
   4186.1 |#############
   4210.4 |#############
   4234.7 |
   4258.9 |
   4283.2 |##########################
   4307.5 |
   4331.7 |##########################
   4356.0 |
   4380.3 |##########################
   4404.6 |
   4428.8 |
   4453.1 |#############
   4477.4 |##########################
   4501.6 |
   4525.9 |
   4550.2 |
   4574.4 |########################################
   4598.7 |
   4623.0 |#############
   4647.2 |
  (1 below, 2 above range)

kernel_nway (n=18, range 4113.9-4719.4 ns)
   4113.9 |
   4144.2 |
   4174.5 |
   4204.7 |
   4235.0 |####################
   4265.3 |
   4295.6 |####################
   4325.8 |
   4356.1 |##########
   4386.4 |########################################
   4416.7 |####################
   4446.9 |
   4477.2 |
   4507.5 |##########
   4537.8 |
   4568.1 |
   4598.3 |
   4628.6 |
   4658.9 |##########
   4689.2 |
  (2 below, 3 above range)

kernel_seq (n=18, range 2726.4-3413.9 ns)
   2726.4 |##############################
   2760.8 |##############################
   2795.2 |########################################
   2829.5 |##########
   2863.9 |
   2898.3 |
   2932.7 |
   2967.0 |
   3001.4 |
   3035.8 |
   3070.2 |##########
   3104.5 |
   3138.9 |
   3173.3 |##########
   3207.7 |
   3242.0 |
   3276.4 |
   3310.8 |
   3345.2 |##########
   3379.5 |
  (1 below, 3 above range)

nway (n=18, range 17065.3-19272.2 ns)
  17065.3 |#############
  17175.6 |
  17286.0 |
  17396.3 |##########################
  17506.7 |#############
  17617.0 |
  17727.4 |
  17837.7 |
  17948.0 |##########################
  18058.4 |##########################
  18168.7 |########################################
  18279.1 |#############
  18389.4 |#############
  18499.8 |#############
  18610.1 |
  18720.5 |
  18830.8 |
  18941.2 |#############
  19051.5 |
  19161.9 |
  (1 below, 2 above range)

sequential (n=18, range 9973.6-10681.2 ns)
   9973.6 |########################################
  10009.0 |
  10044.4 |
  10079.7 |
  10115.1 |
  10150.5 |
  10185.9 |
  10221.3 |
  10256.7 |
  10292.0 |
  10327.4 |
  10362.8 |
  10398.2 |
  10433.6 |
  10468.9 |
  10504.3 |
  10539.7 |
  10575.1 |###
  10610.5 |###
  10645.9 |###
  (1 below, 3 above range)

std_parallel (n=18, range 31775.0-45725.0 ns)
  31775.0 |
  32472.5 |########################################
  33170.0 |########################################
  33867.5 |########################################
  34565.0 |####################
  35262.5 |####################
  35960.0 |
  36657.5 |
  37355.0 |
  38052.5 |####################
  38750.0 |
  39447.5 |####################
  40145.0 |
  40842.5 |####################
  41540.0 |
  42237.5 |
  42935.0 |
  43632.5 |
  44330.0 |########################################
  45027.5 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=2056.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2164.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=2199.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3262.9% of algo (FFI overhead may distort results)
- **nway**: bridge=531.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=903.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=308.9% of algo (FFI overhead may distort results)

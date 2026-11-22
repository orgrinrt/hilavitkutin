# fold_light, short fiber (1 to 4 morsels), W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 60% faster than the next best (head_tail)

kernel_seq (2.59 us) leads head_tail (4.15 us) by 60%, a clear separation rather than a photo finish. CV 5.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 37% (significant)

kernel_seq is -1.55 us (37%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 21.4x slower than the field

std_parallel (55.36 us) is 21.4x the fastest (2.59 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_parallel shows alternating (throttle bounce) (autocorr -0.53)

std_parallel's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 21.4x the fastest

Fastest kernel_seq (2.59 us) to slowest std_parallel (55.36 us): 21.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2592.5 ns median (-37.5% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 21.35x (fastest 2592.5 ns, slowest 55357.5 ns)

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
| head_tail | 4878ns | 4764ns | 4622ns | 4768ns | 5171ns | base |
| head_tail_dyn | 4830ns | 4802ns | 4635ns | 4765ns | 5026ns | -0.97% |
| kernel_nway | 10743ns | 10612ns | 9229ns | 10344ns | 12099ns | +120.24% |
| kernel_seq | 3070ns | 2999ns | 2786ns | 2978ns | 3351ns | -37.05% |
| nway | 34558ns | 34602ns | 30902ns | 34074ns | 37112ns | +608.47% |
| sequential | 10413ns | 10391ns | 9987ns | 10349ns | 10723ns | +113.48% |
| std_parallel | 55386ns | 55751ns | 51690ns | 55040ns | 57752ns | +1035.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4261ns | 4009ns | 4570ns | base | 0.015 |
| head_tail_dyn | 4218ns | 4037ns | 4395ns | -1.00% | 0.015 |
| kernel_nway | 10098ns | 8597ns | 11419ns | +137.01% | 0.006 |
| kernel_seq | 2635ns | 2490ns | 2795ns | -38.16% | 0.024 |
| nway | 34004ns | 30356ns | 36545ns | +698.07% | 0.002 |
| sequential | 10259ns | 9847ns | 10547ns | +140.79% | 0.006 |
| std_parallel | 54976ns | 51260ns | 57348ns | +1190.29% | 0.001 |

## Performance model

- Peak throughput: **0.026 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.015 | 60.0% |
| head_tail_dyn | 0.015 | 59.8% |
| kernel_nway | 0.006 | 25.0% |
| kernel_seq | 0.025 | 96.0% |
| nway | 0.002 | 7.3% |
| sequential | 0.006 | 24.3% |
| std_parallel | 0.001 | 4.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4878ns | 4878ns | base |
| head_tail_dyn | 4830ns | 4830ns | -0.97% |
| kernel_nway | 10743ns | 10743ns | +120.24% |
| kernel_seq | 3070ns | 3070ns | -37.05% |
| nway | 34558ns | 34558ns | +608.47% |
| sequential | 10413ns | 10413ns | +113.48% |
| std_parallel | 55386ns | 55386ns | +1035.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4148ns | base | --- | [4087, 4405] | --- | --- | --- | --- |
| head_tail_dyn | 4163ns | no significant difference | [-62, +95]ns | [4118, 4282] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 9978ns | +5726.6ns (+138.1%) | [+5303, +6337]ns | [9405, 10600] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2592ns | -1554.2ns (-37.5%) | [-1673, -1527]ns | [2549, 2702] | YES | 0.0000 | 0.0000 | 0 |
| nway | 34070ns | +29858.3ns (+719.9%) | [+28678, +30532]ns | [32684, 34745] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10249ns | +6106.6ns (+147.2%) | [+5735, +6271]ns | [10166, 10332] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 55358ns | +51168.3ns (+1233.7%) | [+49558, +51797]ns | [53765, 56088] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4161ns | -1.3% | +149.5% | -39.2% | +711.2% | +146.6% | +1230.4% |
| 1 | 2 | 4080ns | +1.8% | +142.9% | -36.3% | +733.3% | +149.7% | +1236.5% |
| 1 | 3 | 4348ns | -0.4% | +116.7% | -37.9% | +687.6% | +138.1% | +1188.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.357 | moderate- |
| head_tail_dyn | -0.067 | ok |
| kernel_nway | -0.000 | ok |
| kernel_seq | -0.007 | ok |
| nway | -0.000 | ok |
| sequential | -0.452 | moderate- |
| std_parallel | -0.532 | HIGH- (thermal bounce) |

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
| head_tail | 98725.5ns | 4260.7ns | 2317.1% | HIGH |
| head_tail_dyn | 98448.1ns | 4218.2ns | 2333.9% | HIGH |
| kernel_nway | 119335.7ns | 10098.3ns | 1181.7% | HIGH |
| kernel_seq | 91889.0ns | 2635.0ns | 3487.2% | HIGH |
| nway | 111514.7ns | 34003.9ns | 327.9% | HIGH |
| sequential | 90911.1ns | 10259.3ns | 886.1% | HIGH |
| std_parallel | 117719.9ns | 54975.8ns | 214.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 4008.9-4570.3 ns)
   4008.9 |########################################
   4037.0 |
   4065.0 |########################################
   4093.1 |####################
   4121.2 |########################################
   4149.2 |####################
   4177.3 |
   4205.4 |####################
   4233.4 |
   4261.5 |
   4289.6 |####################
   4317.7 |
   4345.7 |
   4373.8 |####################
   4401.9 |########################################
   4429.9 |
   4458.0 |
   4486.1 |
   4514.1 |
   4542.2 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 4036.7-4395.0 ns)
   4036.7 |##########################
   4054.6 |
   4072.5 |#############
   4090.4 |#############
   4108.3 |
   4126.2 |##########################
   4144.2 |##########################
   4162.1 |#############
   4180.0 |
   4197.9 |
   4215.8 |
   4233.8 |#############
   4251.7 |
   4269.6 |########################################
   4287.5 |
   4305.4 |
   4323.3 |
   4341.2 |
   4359.2 |
   4377.1 |#############
  (1 below, 3 above range)

kernel_nway (n=18, range 8596.6-11419.2 ns)
   8596.6 |
   8737.8 |
   8878.9 |
   9020.0 |########################################
   9161.1 |####################
   9302.3 |####################
   9443.4 |########################################
   9584.5 |####################
   9725.6 |####################
   9866.8 |
  10007.9 |####################
  10149.0 |########################################
  10290.2 |
  10431.3 |
  10572.4 |####################
  10713.5 |
  10854.7 |########################################
  10995.8 |
  11136.9 |
  11278.0 |
  (1 below, 3 above range)

kernel_seq (n=18, range 2490.0-2795.3 ns)
   2490.0 |####################
   2505.3 |####################
   2520.5 |####################
   2535.8 |########################################
   2551.1 |
   2566.3 |########################################
   2581.6 |########################################
   2596.8 |####################
   2612.1 |####################
   2627.4 |####################
   2642.6 |
   2657.9 |
   2673.2 |
   2688.4 |
   2703.7 |
   2719.0 |
   2734.2 |
   2749.5 |
   2764.8 |########################################
   2780.0 |####################
  (1 below, 2 above range)

nway (n=18, range 30355.6-36545.3 ns)
  30355.6 |##########################
  30665.1 |
  30974.5 |
  31284.0 |
  31593.5 |
  31903.0 |#############
  32212.5 |#############
  32522.0 |
  32831.4 |#############
  33140.9 |#############
  33450.4 |
  33759.9 |##########################
  34069.4 |#############
  34378.9 |########################################
  34688.4 |#############
  34997.8 |#############
  35307.3 |#############
  35616.8 |
  35926.3 |
  36235.8 |
  (1 below, 2 above range)

sequential (n=18, range 9846.7-10546.7 ns)
   9846.7 |
   9881.7 |#############
   9916.7 |
   9951.7 |
   9986.7 |
  10021.7 |
  10056.7 |
  10091.7 |#############
  10126.7 |#############
  10161.7 |########################################
  10196.7 |
  10231.7 |########################################
  10266.7 |##########################
  10301.7 |
  10336.7 |
  10371.7 |#############
  10406.7 |#############
  10441.7 |
  10476.7 |
  10511.7 |
  (2 below, 3 above range)

std_parallel (n=18, range 51260.0-57347.8 ns)
  51260.0 |
  51564.4 |
  51868.8 |#############
  52173.2 |
  52477.6 |
  52781.9 |
  53086.3 |#############
  53390.7 |##########################
  53695.1 |#############
  53999.5 |#############
  54303.9 |
  54608.3 |#############
  54912.7 |
  55217.0 |########################################
  55521.4 |##########################
  55825.8 |
  56130.2 |#############
  56434.6 |
  56739.0 |
  57043.4 |
  (1 below, 4 above range)

```

## Diagnostics

- **head_tail**: bridge=2372.0% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2360.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1202.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3532.2% of algo (FFI overhead may distort results)
- **nway**: bridge=325.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=891.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=213.5% of algo (FFI overhead may distort results)

# fold_light, far above cache, W = 4

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway shows alternating (throttle bounce) (autocorr -0.63)

nway's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {std_parallel} vs {head_tail, head_tail_dyn, kernel_nway, sequential, kernel_seq, nway} (66% apart)

The field splits into a fast tier {std_parallel} and a slow tier {head_tail, head_tail_dyn, kernel_nway, sequential, kernel_seq, nway} with a 66% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### nway is inconsistent: worst-20% is 2.3x its best-20%

nway's best 20% of batches run at 3.15 ms but its worst 20% at 7.24 ms (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_parallel** at 2175698.0 ns median (-39.8% vs baseline)
- Spread: 1.98x (fastest 2175698.0 ns, slowest 4302265.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 4 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 4162696ns | 3626596ns | 2791817ns | 3379484ns | 6022954ns | base |
| head_tail_dyn | 4666748ns | 3798480ns | 3140094ns | 3642983ns | 6965724ns | +12.11% |
| kernel_nway | 4230928ns | 3903010ns | 3229224ns | 3793060ns | 5388582ns | +1.64% |
| kernel_seq | 4179857ns | 4171699ns | 4121489ns | 4155773ns | 4245166ns | +0.41% |
| nway | 4973609ns | 4306193ns | 3154040ns | 4064586ns | 7246929ns | +19.48% |
| sequential | 3965429ns | 4142068ns | 3400859ns | 3967774ns | 4244196ns | -4.74% |
| std_parallel | 2643993ns | 2179562ns | 2125518ns | 2162126ns | 3626032ns | -36.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4154391ns | 2789526ns | 6016836ns | base | 4.038 |
| head_tail_dyn | 4660088ns | 3134352ns | 6957357ns | +12.17% | 3.600 |
| kernel_nway | 4221467ns | 3225475ns | 5378299ns | +1.61% | 3.974 |
| kernel_seq | 4173700ns | 4119658ns | 4234309ns | +0.46% | 4.020 |
| nway | 4968683ns | 3150536ns | 7239647ns | +19.60% | 3.377 |
| sequential | 3964250ns | 3399869ns | 4242927ns | -4.58% | 4.232 |
| std_parallel | 2639291ns | 2123062ns | 3618115ns | -36.47% | 6.357 |

## Performance model

- Peak throughput: **7.902 Gops/s** (std_parallel; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 4.640 | 58.7% |
| head_tail_dyn | 4.424 | 56.0% |
| kernel_nway | 4.309 | 54.5% |
| kernel_seq | 4.026 | 50.9% |
| nway | 3.900 | 49.3% |
| sequential | 4.051 | 51.3% |
| std_parallel | 7.711 | 97.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4162696ns | 4162696ns | base |
| head_tail_dyn | 4666748ns | 4666748ns | +12.11% |
| kernel_nway | 4230928ns | 4230928ns | +1.64% |
| kernel_seq | 4179857ns | 4179857ns | +0.41% |
| nway | 4973609ns | 4973609ns | +19.48% |
| sequential | 3965429ns | 3965429ns | -4.74% |
| std_parallel | 2643993ns | 2643993ns | -36.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 3615757ns | base | --- | [2830579, 6016836] | --- | --- | --- | --- |
| head_tail_dyn | 3792283ns | no significant difference | [-2509167, +3510929]ns | [3230624, 6957357] | no | 1.0000 | 0.6875 | 0 |
| kernel_nway | 3893726ns | no significant difference | [-2299172, +2547720]ns | [3392377, 5378299] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 4167010ns | no significant difference | [-1877475, +1317467]ns | [4119780, 4234309] | no | 1.0000 | 0.6875 | 0 |
| nway | 4302265ns | no significant difference | [-1076566, +2785569]ns | [3364138, 7239647] | no | 0.6563 | 0.2188 | 0 |
| sequential | 4141065ns | no significant difference | [-2241945, +1329055]ns | [3508757, 4242927] | no | 1.0000 | 1.0000 | 0 |
| std_parallel | 2175698ns | no significant difference | [-3886952, +687772]ns | [2124060, 3618115] | no | 0.6563 | 0.2188 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4323878ns | -9.9% | +6.0% | -4.1% | +23.6% | -3.6% | -18.7% |
| 1 | 2 | 2999908ns | +17.2% | +56.3% | +38.3% | +21.0% | +40.5% | -25.8% |
| 1 | 3 | 5139386ns | +27.8% | -34.0% | -17.8% | +15.4% | -31.7% | -57.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.572 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.236 | moderate- |
| kernel_nway | -0.212 | moderate- |
| kernel_seq | -0.170 | ok |
| nway | -0.627 | HIGH- (thermal bounce) |
| sequential | -0.202 | moderate- |
| std_parallel | -0.148 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 6395646.8ns | 4154390.7ns | 153.9% | HIGH |
| head_tail_dyn | 7220747.8ns | 4660087.9ns | 154.9% | HIGH |
| kernel_nway | 4468198.0ns | 4221467.2ns | 105.8% | HIGH |
| kernel_seq | 4173270.4ns | 4173699.8ns | 100.0% | HIGH |
| nway | 5013960.4ns | 4968683.2ns | 100.9% | HIGH |
| sequential | 3991504.5ns | 3964250.0ns | 100.7% | HIGH |
| std_parallel | 2804924.1ns | 2639291.0ns | 106.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 2789525.7-6016836.4 ns)
  2789525.7 |########################################
  2950891.2 |
  3112256.8 |####################
  3273622.3 |
  3434987.8 |
  3596353.4 |
  3757718.9 |
  3919084.4 |####################
  4080450.0 |
  4241815.5 |
  4403181.1 |
  4564546.6 |
  4725912.1 |
  4887277.7 |
  5048643.2 |
  5210008.7 |
  5371374.3 |
  5532739.8 |
  5694105.3 |####################
  5855470.9 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 3134352.5-6957357.0 ns)
  3134352.5 |########################################
  3325502.7 |########################################
  3516653.0 |########################################
  3707803.2 |########################################
  3898953.4 |########################################
  4090103.6 |
  4281253.8 |
  4472404.1 |
  4663554.3 |
  4854704.5 |
  5045854.8 |
  5237005.0 |
  5428155.2 |
  5619305.4 |
  5810455.7 |
  6001605.9 |
  6192756.1 |
  6383906.3 |
  6575056.6 |
  6766206.8 |
  (0 below, 1 above range)

kernel_nway (n=6, range 3225475.4-5378299.3 ns)
  3225475.4 |####################
  3333116.6 |
  3440757.8 |
  3548399.0 |########################################
  3656040.2 |
  3763681.4 |
  3871322.6 |
  3978963.8 |
  4086605.0 |
  4194246.2 |####################
  4301887.4 |
  4409528.6 |
  4517169.8 |
  4624811.0 |
  4732452.2 |
  4840093.4 |
  4947734.6 |####################
  5055375.8 |
  5163017.0 |
  5270658.2 |
  (0 below, 1 above range)

kernel_seq (n=6, range 4119658.2-4234309.5 ns)
  4119658.2 |########################################
  4125390.8 |
  4131123.3 |
  4136855.9 |
  4142588.5 |
  4148321.0 |
  4154053.6 |####################
  4159786.1 |
  4165518.7 |
  4171251.3 |########################################
  4176983.8 |
  4182716.4 |
  4188449.0 |
  4194181.5 |
  4199914.1 |
  4205646.6 |
  4211379.2 |
  4217111.8 |
  4222844.3 |
  4228576.9 |
  (0 below, 1 above range)

nway (n=6, range 3150536.2-7239646.7 ns)
  3150536.2 |####################
  3354991.7 |
  3559447.2 |########################################
  3763902.8 |
  3968358.3 |
  4172813.8 |
  4377269.3 |
  4581724.9 |
  4786180.4 |####################
  4990635.9 |
  5195091.4 |
  5399546.9 |
  5604002.5 |
  5808458.0 |
  6012913.5 |
  6217369.0 |
  6421824.6 |
  6626280.1 |
  6830735.6 |####################
  7035191.1 |
  (0 below, 1 above range)

sequential (n=6, range 3399868.6-4242927.4 ns)
  3399868.6 |####################
  3442021.5 |
  3484174.5 |
  3526327.4 |
  3568480.4 |
  3610633.3 |####################
  3652786.2 |
  3694939.2 |
  3737092.1 |
  3779245.1 |
  3821398.0 |
  3863550.9 |
  3905703.9 |
  3947856.8 |
  3990009.8 |
  4032162.7 |
  4074315.6 |
  4116468.6 |########################################
  4158621.5 |####################
  4200774.5 |
  (0 below, 1 above range)

std_parallel (n=6, range 2123061.9-3618114.7 ns)
  2123061.9 |########################################
  2197814.5 |#############
  2272567.2 |#############
  2347319.8 |
  2422072.5 |
  2496825.1 |
  2571577.7 |
  2646330.4 |
  2721083.0 |
  2795835.7 |
  2870588.3 |
  2945340.9 |
  3020093.6 |
  3094846.2 |
  3169598.9 |
  3244351.5 |
  3319104.1 |
  3393856.8 |
  3468609.4 |
  3543362.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=33.3% (high variance, measurements may be unstable)
- **head_tail**: bridge=166.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=51.7% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=183.8% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=21.4% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=103.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.0% of algo (FFI overhead may distort results)
- **nway**: CV=34.3% (high variance, measurements may be unstable)
- **nway**: bridge=107.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.6% of algo (FFI overhead may distort results)
- **std_parallel**: CV=38.6% (high variance, measurements may be unstable)
- **std_parallel**: bridge=102.1% of algo (FFI overhead may distort results)

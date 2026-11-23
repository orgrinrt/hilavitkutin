# fold_light, far above cache, W = 2

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (head_tail) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline head_tail has the worst median (5.34 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest std_parallel at 2.84 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sequential shows alternating (throttle bounce) (autocorr -0.67)

sequential's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {std_parallel} vs {sequential, kernel_seq, nway, kernel_nway, head_tail_dyn, head_tail} (47% apart)

The field splits into a fast tier {std_parallel} and a slow tier {sequential, kernel_seq, nway, kernel_nway, head_tail_dyn, head_tail} with a 47% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### head_tail_dyn is inconsistent: worst-20% is 2.8x its best-20%

head_tail_dyn's best 20% of batches run at 3.25 ms but its worst 20% at 9.16 ms (2.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_parallel** at 2838990.8 ns median (-46.8% vs baseline)
- Spread: 1.88x (fastest 2838990.8 ns, slowest 5335693.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 2 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112743060 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 4730786ns | 5342637ns | 2594908ns | 4432955ns | 6245472ns | base |
| head_tail_dyn | 5812291ns | 5008593ns | 3254136ns | 4424184ns | 9173528ns | +22.86% |
| kernel_nway | 4652754ns | 4728195ns | 2456178ns | 4250607ns | 6354263ns | -1.65% |
| kernel_seq | 4168131ns | 4184831ns | 4110227ns | 4164556ns | 4202445ns | -11.89% |
| nway | 4886866ns | 4327555ns | 2758918ns | 3836654ns | 7526159ns | +3.30% |
| sequential | 3883105ns | 4166843ns | 3256819ns | 3875856ns | 4207121ns | -17.92% |
| std_parallel | 2868238ns | 2842117ns | 2802286ns | 2832960ns | 2954131ns | -39.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4722976ns | 2584796ns | 6237276ns | base | 3.552 |
| head_tail_dyn | 5804545ns | 3249609ns | 9158800ns | +22.90% | 2.890 |
| kernel_nway | 4645248ns | 2451990ns | 6343863ns | -1.65% | 3.612 |
| kernel_seq | 4161851ns | 4108584ns | 4195134ns | -11.88% | 4.031 |
| nway | 4883799ns | 2756837ns | 7522425ns | +3.41% | 3.435 |
| sequential | 3881937ns | 3255789ns | 4205916ns | -17.81% | 4.322 |
| std_parallel | 2863833ns | 2800195ns | 2946143ns | -39.36% | 5.858 |

## Performance model

- Peak throughput: **6.842 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 3.144 | 46.0% |
| head_tail_dyn | 3.353 | 49.0% |
| kernel_nway | 3.554 | 51.9% |
| kernel_seq | 4.018 | 58.7% |
| nway | 3.880 | 56.7% |
| sequential | 4.027 | 58.9% |
| std_parallel | 5.910 | 86.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4730786ns | 4730786ns | base |
| head_tail_dyn | 5812291ns | 5812291ns | +22.86% |
| kernel_nway | 4652754ns | 4652754ns | -1.65% |
| kernel_seq | 4168131ns | 4168131ns | -11.89% |
| nway | 4886866ns | 4886866ns | +3.30% |
| sequential | 3883105ns | 3883105ns | -17.92% |
| std_parallel | 2868238ns | 2868238ns | -39.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 5335693ns | base | --- | [2595958, 6237276] | --- | --- | --- | --- |
| head_tail_dyn | 5003446ns | no significant difference | [-2923184, +5552896]ns | [3251389, 9158800] | no | 1.0000 | 0.6875 | 0 |
| kernel_nway | 4721135ns | no significant difference | [-1352884, +995040]ns | [2870747, 6343863] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 4175380ns | no significant difference | [-2089742, +1595081]ns | [4115039, 4195134] | no | 1.0000 | 0.6875 | 0 |
| nway | 4324446ns | no significant difference | [-2071649, +2650796]ns | [2804525, 7522425] | no | 1.0000 | 1.0000 | 0 |
| sequential | 4165878ns | no significant difference | [-2479684, +1132718]ns | [3274017, 4205916] | no | 1.0000 | 0.6875 | 0 |
| std_parallel | 2838991ns | no significant difference | [-3389368, +308642]ns | [2806364, 2946143] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4489005ns | -26.9% | +2.8% | -6.5% | +59.1% | -6.8% | -36.4% |
| 1 | 2 | 4314585ns | +36.3% | +4.2% | -3.8% | -35.0% | -24.1% | -32.5% |
| 1 | 3 | 5365337ns | +53.8% | -10.0% | -22.8% | -12.3% | -21.9% | -47.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.263 | moderate- |
| head_tail_dyn | -0.000 | ok |
| kernel_nway | -0.399 | moderate- |
| kernel_seq | -0.084 | ok |
| nway | -0.458 | moderate- |
| sequential | -0.667 | HIGH- (thermal bounce) |
| std_parallel | -0.595 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 2/3, lost 1/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 5739569.7ns | 4722975.8ns | 121.5% | HIGH |
| head_tail_dyn | 5927347.9ns | 5804545.0ns | 102.1% | HIGH |
| kernel_nway | 4872492.1ns | 4645248.2ns | 104.9% | HIGH |
| kernel_seq | 4170147.5ns | 4161851.1ns | 100.2% | HIGH |
| nway | 5288983.5ns | 4883798.9ns | 108.3% | HIGH |
| sequential | 3890067.4ns | 3881936.9ns | 100.2% | HIGH |
| std_parallel | 2976323.3ns | 2863832.7ns | 103.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 2584795.9-6237275.8 ns)
  2584795.9 |########################################
  2767419.9 |
  2950043.9 |
  3132667.9 |
  3315291.9 |
  3497915.9 |
  3680539.9 |
  3863163.9 |
  4045787.9 |
  4228411.9 |
  4411035.9 |
  4593659.8 |####################
  4776283.8 |
  4958907.8 |
  5141531.8 |
  5324155.8 |
  5506779.8 |
  5689403.8 |
  5872027.8 |####################
  6054651.8 |####################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 3249609.1-9158800.2 ns)
  3249609.1 |########################################
  3545068.7 |
  3840528.2 |
  4135987.8 |
  4431447.3 |
  4726906.9 |
  5022366.4 |
  5317826.0 |
  5613285.5 |
  5908745.1 |
  6204204.6 |
  6499664.2 |#############
  6795123.7 |
  7090583.3 |
  7386042.8 |
  7681502.4 |
  7976961.9 |
  8272421.5 |#############
  8567881.0 |
  8863340.6 |
  (0 below, 1 above range)

kernel_nway (n=6, range 2451990.2-6343863.2 ns)
  2451990.2 |########################################
  2646583.9 |
  2841177.5 |
  3035771.2 |
  3230364.8 |########################################
  3424958.5 |
  3619552.1 |########################################
  3814145.8 |
  4008739.4 |
  4203333.0 |
  4397926.7 |
  4592520.3 |
  4787114.0 |
  4981707.7 |
  5176301.3 |
  5370894.9 |
  5565488.6 |########################################
  5760082.2 |########################################
  5954675.9 |
  6149269.5 |
  (0 below, 1 above range)

kernel_seq (n=6, range 4108584.0-4195134.2 ns)
  4108584.0 |########################################
  4112911.5 |
  4117239.0 |########################################
  4121566.5 |
  4125894.0 |
  4130221.6 |
  4134549.1 |
  4138876.6 |
  4143204.1 |
  4147531.6 |
  4151859.1 |
  4156186.6 |
  4160514.1 |
  4164841.7 |
  4169169.2 |########################################
  4173496.7 |
  4177824.2 |########################################
  4182151.7 |
  4186479.2 |########################################
  4190806.7 |
  (0 below, 1 above range)

nway (n=6, range 2756836.9-7522425.2 ns)
  2756836.9 |########################################
  2995116.3 |
  3233395.7 |
  3471675.2 |
  3709954.6 |####################
  3948234.0 |
  4186513.4 |
  4424792.8 |
  4663072.2 |####################
  4901351.7 |
  5139631.1 |
  5377910.5 |
  5616189.9 |####################
  5854469.3 |
  6092748.7 |
  6331028.2 |
  6569307.6 |
  6807587.0 |
  7045866.4 |
  7284145.8 |
  (0 below, 1 above range)

sequential (n=6, range 3255789.1-4205915.6 ns)
  3255789.1 |##########################
  3303295.4 |
  3350801.7 |
  3398308.1 |
  3445814.4 |
  3493320.7 |
  3540827.0 |
  3588333.4 |
  3635839.7 |
  3683346.0 |
  3730852.3 |
  3778358.6 |
  3825865.0 |
  3873371.3 |
  3920877.6 |
  3968383.9 |
  4015890.3 |
  4063396.6 |
  4110902.9 |
  4158409.2 |########################################
  (0 below, 1 above range)

std_parallel (n=6, range 2800195.3-2946143.1 ns)
  2800195.3 |########################################
  2807492.7 |########################################
  2814790.1 |
  2822087.5 |########################################
  2829384.9 |
  2836682.2 |
  2843979.6 |########################################
  2851277.0 |
  2858574.4 |
  2865871.8 |
  2873169.2 |
  2880466.6 |
  2887764.0 |
  2895061.4 |########################################
  2902358.8 |
  2909656.1 |
  2916953.5 |
  2924250.9 |
  2931548.3 |
  2938845.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=33.9% (high variance, measurements may be unstable)
- **head_tail**: bridge=111.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=46.3% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=66.1% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=33.7% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=108.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.9% of algo (FFI overhead may distort results)
- **nway**: CV=46.5% (high variance, measurements may be unstable)
- **nway**: bridge=104.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=102.4% of algo (FFI overhead may distort results)

# fold_light, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 3.3x slower than the field

std_parallel (27.15 us) is 3.3x the fastest (8.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.67)

sequential's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (8.35 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.3x the fastest

Fastest head_tail (8.35 us) to slowest std_parallel (27.15 us): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (head_tail) is the fastest** at 8347.9 ns median
- 5 variants significantly slower than baseline
- Spread: 3.25x (fastest 8347.9 ns, slowest 27150.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 1 |
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
| head_tail | 8857ns | 8821ns | 8579ns | 8765ns | 9133ns | base |
| head_tail_dyn | 9229ns | 8994ns | 8576ns | 8980ns | 9928ns | +4.20% |
| kernel_nway | 9068ns | 9065ns | 8667ns | 8976ns | 9405ns | +2.38% |
| kernel_seq | 9053ns | 8977ns | 8775ns | 8937ns | 9365ns | +2.21% |
| nway | 18608ns | 18546ns | 18358ns | 18530ns | 18849ns | +110.10% |
| sequential | 16078ns | 16171ns | 15460ns | 15992ns | 16518ns | +81.54% |
| std_parallel | 27409ns | 27548ns | 25392ns | 27409ns | 28418ns | +209.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 8379ns | 8162ns | 8596ns | base | 7.822 |
| head_tail_dyn | 8771ns | 8114ns | 9462ns | +4.68% | 7.472 |
| kernel_nway | 8623ns | 8315ns | 8933ns | +2.92% | 7.600 |
| kernel_seq | 8589ns | 8378ns | 8901ns | +2.51% | 7.631 |
| nway | 18252ns | 18050ns | 18466ns | +117.84% | 3.591 |
| sequential | 15873ns | 15282ns | 16303ns | +89.45% | 4.129 |
| std_parallel | 27026ns | 25067ns | 28008ns | +222.55% | 2.425 |

## Performance model

- Peak throughput: **8.077 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 7.851 | 97.2% |
| head_tail_dyn | 7.597 | 94.1% |
| kernel_nway | 7.643 | 94.6% |
| kernel_seq | 7.739 | 95.8% |
| nway | 3.600 | 44.6% |
| sequential | 4.108 | 50.9% |
| std_parallel | 2.414 | 29.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 8857ns | 8857ns | base |
| head_tail_dyn | 9229ns | 9229ns | +4.20% |
| kernel_nway | 9068ns | 9068ns | +2.38% |
| kernel_seq | 9053ns | 9053ns | +2.21% |
| nway | 18608ns | 18608ns | +110.10% |
| sequential | 16078ns | 16078ns | +81.54% |
| std_parallel | 27409ns | 27409ns | +209.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 8348ns | base | --- | [8246, 8458] | --- | --- | --- | --- |
| head_tail_dyn | 8627ns | no significant difference | [-60, +702]ns | [8312, 9162] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 8575ns | +202.1ns (+2.4%) | [+25, +515]ns | [8435, 8719] | YES | 0.0371 | 0.0309 | 0 |
| kernel_seq | 8469ns | +147.9ns (+1.8%) | [+21, +433]ns | [8408, 8683] | YES | 0.0371 | 0.0309 | 0 |
| nway | 18204ns | +9906.2ns (+118.7%) | [+9735, +9969]ns | [18100, 18290] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 15954ns | +7731.2ns (+92.6%) | [+7021, +7858]ns | [15475, 16127] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 27150ns | +18904.2ns (+226.5%) | [+18338, +19265]ns | [26856, 27683] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 8227ns | +11.3% | +4.5% | +8.2% | +121.7% | +95.2% | +232.0% |
| 1 | 2 | 8527ns | +2.1% | -1.1% | -1.4% | +112.9% | +79.7% | +218.4% |
| 1 | 3 | 8302ns | -1.1% | +5.3% | +1.4% | +119.7% | +93.6% | +221.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.628 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.000 | ok |
| kernel_nway | -0.507 | HIGH- (thermal bounce) |
| kernel_seq | -0.178 | ok |
| nway | -0.666 | HIGH- (thermal bounce) |
| sequential | -0.667 | HIGH- (thermal bounce) |
| std_parallel | -0.050 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 89806.0ns | 8378.7ns | 1071.8% | HIGH |
| head_tail_dyn | 91741.0ns | 8771.1ns | 1046.0% | HIGH |
| kernel_nway | 91015.3ns | 8623.4ns | 1055.4% | HIGH |
| kernel_seq | 91766.7ns | 8588.7ns | 1068.5% | HIGH |
| nway | 91308.8ns | 18252.3ns | 500.3% | HIGH |
| sequential | 94151.4ns | 15873.2ns | 593.1% | HIGH |
| std_parallel | 100244.7ns | 27025.7ns | 370.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 8162.5-8595.8 ns)
   8162.5 |
   8184.2 |########################################
   8205.8 |
   8227.5 |#############
   8249.2 |#############
   8270.8 |#############
   8292.5 |
   8314.2 |##########################
   8335.8 |
   8357.5 |#############
   8379.2 |##########################
   8400.8 |#############
   8422.5 |
   8444.2 |
   8465.8 |
   8487.5 |#############
   8509.2 |##########################
   8530.8 |
   8552.5 |
   8574.2 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 8113.9-9461.8 ns)
   8113.9 |########################################
   8181.3 |####################
   8248.7 |########################################
   8316.1 |
   8383.5 |########################################
   8450.9 |####################
   8518.2 |
   8585.6 |
   8653.0 |
   8720.4 |####################
   8787.8 |
   8855.2 |########################################
   8922.6 |####################
   8990.0 |
   9057.4 |
   9124.8 |
   9192.2 |
   9259.6 |
   9327.0 |####################
   9394.4 |####################
  (1 below, 3 above range)

kernel_nway (n=18, range 8315.3-8933.3 ns)
   8315.3 |
   8346.2 |####################
   8377.1 |########################################
   8408.0 |
   8438.9 |
   8469.8 |####################
   8500.7 |####################
   8531.6 |####################
   8562.5 |########################################
   8593.4 |########################################
   8624.3 |
   8655.2 |
   8686.1 |####################
   8717.0 |
   8747.9 |
   8778.8 |####################
   8809.7 |####################
   8840.6 |
   8871.5 |
   8902.4 |
  (2 below, 3 above range)

kernel_seq (n=18, range 8377.8-8901.4 ns)
   8377.8 |########################
   8403.9 |########################################
   8430.1 |
   8456.3 |
   8482.5 |
   8508.7 |################
   8534.9 |########
   8561.0 |
   8587.2 |
   8613.4 |
   8639.6 |
   8665.8 |########
   8691.9 |########
   8718.1 |
   8744.3 |
   8770.5 |
   8796.7 |
   8822.8 |
   8849.0 |########
   8875.2 |
  (1 below, 3 above range)

nway (n=18, range 18050.0-18466.0 ns)
  18050.0 |#############
  18070.8 |#############
  18091.6 |########################################
  18112.4 |
  18133.2 |
  18154.0 |#############
  18174.8 |
  18195.6 |########################################
  18216.4 |
  18237.2 |
  18258.0 |##########################
  18278.8 |##########################
  18299.6 |
  18320.4 |
  18341.2 |
  18362.0 |
  18382.8 |#############
  18403.6 |
  18424.4 |
  18445.2 |
  (1 below, 3 above range)

sequential (n=18, range 15281.9-16302.8 ns)
  15281.9 |##########################
  15333.0 |#############
  15384.0 |#############
  15435.1 |
  15486.1 |
  15537.1 |#############
  15588.2 |
  15639.2 |
  15690.3 |
  15741.3 |
  15792.4 |
  15843.4 |
  15894.4 |########################################
  15945.5 |##########################
  15996.5 |#############
  16047.6 |
  16098.6 |##########################
  16149.7 |
  16200.7 |
  16251.7 |
  (1 below, 4 above range)

std_parallel (n=18, range 25066.7-28007.6 ns)
  25066.7 |
  25213.7 |
  25360.8 |
  25507.8 |
  25654.9 |
  25801.9 |
  25949.0 |#############
  26096.0 |
  26243.1 |
  26390.1 |#############
  26537.2 |
  26684.2 |
  26831.3 |##########################
  26978.3 |########################################
  27125.4 |##########################
  27272.4 |##########################
  27419.4 |
  27566.5 |
  27713.5 |
  27860.6 |#############
  (2 below, 4 above range)

```

## Diagnostics

- **head_tail**: bridge=1070.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=1056.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1066.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=1077.1% of algo (FFI overhead may distort results)
- **nway**: bridge=500.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=598.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=372.8% of algo (FFI overhead may distort results)

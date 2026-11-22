# fold_light, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 9.3x slower than the field

std_parallel (82.32 us) is 9.3x the fastest (8.87 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.67)

kernel_seq's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (8.87 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 9.3x the fastest

Fastest head_tail (8.87 us) to slowest std_parallel (82.32 us): 9.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (head_tail) is the fastest** at 8872.9 ns median
- 5 variants significantly slower than baseline
- Spread: 9.28x (fastest 8872.9 ns, slowest 82316.6 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 9697ns | 9550ns | 8933ns | 9507ns | 10365ns | base |
| head_tail_dyn | 13278ns | 12975ns | 12646ns | 12957ns | 14074ns | +36.92% |
| kernel_nway | 24757ns | 24846ns | 19749ns | 24146ns | 28178ns | +155.30% |
| kernel_seq | 9625ns | 9508ns | 9024ns | 9455ns | 10182ns | -0.74% |
| nway | 52669ns | 52267ns | 49164ns | 52188ns | 55144ns | +443.13% |
| sequential | 16340ns | 15965ns | 15492ns | 15882ns | 17451ns | +68.50% |
| std_parallel | 83100ns | 82627ns | 77265ns | 81288ns | 88736ns | +756.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 8981ns | 8469ns | 9526ns | base | 7.297 |
| head_tail_dyn | 12710ns | 12110ns | 13438ns | +41.52% | 5.156 |
| kernel_nway | 24075ns | 19096ns | 27505ns | +168.05% | 2.722 |
| kernel_seq | 9161ns | 8590ns | 9708ns | +2.00% | 7.154 |
| nway | 52081ns | 48569ns | 54569ns | +479.87% | 1.258 |
| sequential | 16122ns | 15307ns | 17173ns | +79.50% | 4.065 |
| std_parallel | 82741ns | 76938ns | 88357ns | +821.25% | 0.792 |

## Performance model

- Peak throughput: **7.738 Gops/s** (head_tail; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 7.386 | 95.5% |
| head_tail_dyn | 5.274 | 68.2% |
| kernel_nway | 2.713 | 35.1% |
| kernel_seq | 7.232 | 93.5% |
| nway | 1.268 | 16.4% |
| sequential | 4.155 | 53.7% |
| std_parallel | 0.796 | 10.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 9697ns | 9697ns | base |
| head_tail_dyn | 13278ns | 13278ns | +36.92% |
| kernel_nway | 24757ns | 24757ns | +155.30% |
| kernel_seq | 9625ns | 9625ns | -0.74% |
| nway | 52669ns | 52669ns | +443.13% |
| sequential | 16340ns | 16340ns | +68.50% |
| std_parallel | 83100ns | 83100ns | +756.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 8873ns | base | --- | [8615, 9308] | --- | --- | --- | --- |
| head_tail_dyn | 12427ns | +3685.5ns (+41.5%) | [+3344, +3931]ns | [12294, 13000] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 24154ns | +15145.9ns (+170.7%) | [+13371, +17069]ns | [22379, 26204] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 9062ns | no significant difference | [-158, +700]ns | [8817, 9477] | no | 0.2379 | 0.2379 | 0 |
| nway | 51681ns | +42822.9ns (+482.6%) | [+42262, +44421]ns | [51100, 53762] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 15773ns | +6852.0ns (+77.2%) | [+6583, +7090]ns | [15483, 16012] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 82317ns | +73566.6ns (+829.1%) | [+69988, +75783]ns | [79085, 84867] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 9208ns | +40.6% | +174.6% | +1.1% | +478.4% | +72.9% | +807.5% |
| 1 | 2 | 8640ns | +42.4% | +145.6% | +2.2% | +492.3% | +86.9% | +853.0% |
| 1 | 3 | 8719ns | +45.6% | +176.4% | +7.1% | +491.1% | +76.5% | +816.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.246 | moderate- |
| head_tail_dyn | -0.565 | HIGH- (thermal bounce) |
| kernel_nway | -0.613 | HIGH- (thermal bounce) |
| kernel_seq | -0.665 | HIGH- (thermal bounce) |
| nway | -0.268 | moderate- |
| sequential | -0.352 | moderate- |
| std_parallel | -0.023 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 88554.6ns | 8981.5ns | 986.0% | HIGH |
| head_tail_dyn | 94080.3ns | 12710.4ns | 740.2% | HIGH |
| kernel_nway | 106092.1ns | 24074.6ns | 440.7% | HIGH |
| kernel_seq | 91647.7ns | 9161.1ns | 1000.4% | HIGH |
| nway | 110363.0ns | 52080.6ns | 211.9% | HIGH |
| sequential | 95147.9ns | 16122.0ns | 590.2% | HIGH |
| std_parallel | 135220.4ns | 82741.4ns | 163.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 8469.4-9526.4 ns)
   8469.4 |########################################
   8522.3 |
   8575.1 |
   8628.0 |
   8680.8 |####################
   8733.7 |##########
   8786.5 |
   8839.4 |####################
   8892.2 |
   8945.1 |
   8997.9 |##########
   9050.8 |
   9103.6 |##########
   9156.5 |
   9209.3 |
   9262.1 |##########
   9315.0 |##########
   9367.8 |
   9420.7 |##########
   9473.5 |##########
  (1 below, 2 above range)

head_tail_dyn (n=18, range 12109.7-13438.2 ns)
  12109.7 |
  12176.1 |##########################
  12242.5 |#############
  12309.0 |########################################
  12375.4 |#############
  12441.8 |#############
  12508.2 |#############
  12574.7 |
  12641.1 |
  12707.5 |
  12773.9 |
  12840.4 |
  12906.8 |##########################
  12973.2 |#############
  13039.6 |
  13106.1 |
  13172.5 |
  13238.9 |
  13305.3 |
  13371.8 |#############
  (2 below, 3 above range)

kernel_nway (n=18, range 19095.9-27504.9 ns)
  19095.9 |####################
  19516.3 |
  19936.8 |
  20357.2 |
  20777.7 |####################
  21198.1 |
  21618.6 |
  22039.0 |########################################
  22459.5 |
  22879.9 |####################
  23300.4 |####################
  23720.8 |####################
  24141.3 |####################
  24561.7 |####################
  24982.2 |
  25402.6 |####################
  25823.1 |####################
  26243.5 |####################
  26664.0 |####################
  27084.4 |####################
  (2 below, 2 above range)

kernel_seq (n=18, range 8590.3-9707.6 ns)
   8590.3 |####################
   8646.1 |
   8702.0 |########################################
   8757.9 |####################
   8813.7 |########################################
   8869.6 |
   8925.5 |
   8981.3 |####################
   9037.2 |########################################
   9093.1 |
   9148.9 |
   9204.8 |
   9260.7 |########################################
   9316.5 |
   9372.4 |####################
   9428.3 |
   9484.1 |
   9540.0 |########################################
   9595.9 |####################
   9651.7 |
  (1 below, 2 above range)

nway (n=18, range 48569.5-54569.5 ns)
  48569.5 |
  48869.5 |
  49169.5 |##########
  49469.5 |
  49769.5 |
  50069.5 |
  50369.5 |##########
  50669.5 |
  50969.5 |########################################
  51269.5 |##########
  51569.5 |##########
  51869.5 |
  52169.5 |
  52469.5 |##########
  52769.5 |
  53069.5 |##########
  53369.5 |
  53669.5 |##############################
  53969.5 |
  54269.5 |##########
  (2 below, 2 above range)

sequential (n=18, range 15306.9-17172.9 ns)
  15306.9 |#############
  15400.2 |########################################
  15493.5 |
  15586.8 |
  15680.1 |########################################
  15773.4 |#############
  15866.7 |##########################
  15960.0 |##########################
  16053.3 |
  16146.6 |
  16239.9 |#############
  16333.2 |
  16426.5 |
  16519.8 |
  16613.1 |
  16706.4 |##########################
  16799.7 |
  16893.0 |
  16986.3 |
  17079.6 |
  (2 below, 1 above range)

std_parallel (n=18, range 76937.5-88356.9 ns)
  76937.5 |##########################
  77508.5 |
  78079.4 |#############
  78650.4 |#############
  79221.4 |#############
  79792.4 |#############
  80363.3 |
  80934.3 |#############
  81505.3 |
  82076.2 |########################################
  82647.2 |
  83218.2 |#############
  83789.2 |
  84360.1 |##########################
  84931.1 |
  85502.1 |
  86073.0 |
  86644.0 |
  87215.0 |#############
  87786.0 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=1004.0% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=759.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=437.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=1014.5% of algo (FFI overhead may distort results)
- **nway**: bridge=213.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=601.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=164.6% of algo (FFI overhead may distort results)

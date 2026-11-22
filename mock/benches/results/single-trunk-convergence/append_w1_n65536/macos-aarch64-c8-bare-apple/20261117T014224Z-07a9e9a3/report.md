# append, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway is an outlier: 6.2x slower than the field

nway (59.05 us) is 6.2x the fastest (9.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (kernel_nway, head_tail_dyn) are a dead heat (<1%)

kernel_nway (9.55 us) and head_tail_dyn (9.62 us) differ by 0.76%, inside the noise, even though the wider field spreads 518.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.65)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 6.2x the fastest

Fastest kernel_nway (9.55 us) to slowest nway (59.05 us): 6.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_nway** at 9547.9 ns median (-1.0% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 6.18x (fastest 9547.9 ns, slowest 59045.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 1 |
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
| head_tail | 10454ns | 10208ns | 10021ns | 10216ns | 11026ns | base |
| head_tail_dyn | 10428ns | 10412ns | 10026ns | 10349ns | 10748ns | -0.25% |
| kernel_nway | 10217ns | 10240ns | 9833ns | 10166ns | 10485ns | -2.26% |
| kernel_seq | 10529ns | 10442ns | 10000ns | 10350ns | 11061ns | +0.72% |
| nway | 59496ns | 59527ns | 58769ns | 59320ns | 60123ns | +469.14% |
| sequential | 55786ns | 55285ns | 54746ns | 55265ns | 57086ns | +433.64% |
| std_parallel | 26371ns | 26125ns | 25378ns | 26035ns | 27372ns | +152.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 9874ns | 9497ns | 10433ns | base | 6.637 |
| head_tail_dyn | 9692ns | 9494ns | 9910ns | -1.85% | 6.762 |
| kernel_nway | 9589ns | 9467ns | 9715ns | -2.89% | 6.835 |
| kernel_seq | 9920ns | 9497ns | 10346ns | +0.47% | 6.606 |
| nway | 59084ns | 58326ns | 59808ns | +498.37% | 1.109 |
| sequential | 55421ns | 54371ns | 56697ns | +461.28% | 1.183 |
| std_parallel | 25089ns | 23953ns | 26583ns | +154.09% | 2.612 |

## Performance model

- Peak throughput: **6.923 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 6.799 | 98.2% |
| head_tail_dyn | 6.812 | 98.4% |
| kernel_nway | 6.864 | 99.1% |
| kernel_seq | 6.625 | 95.7% |
| nway | 1.110 | 16.0% |
| sequential | 1.191 | 17.2% |
| std_parallel | 2.681 | 38.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 10454ns | 10454ns | base |
| head_tail_dyn | 10428ns | 10428ns | -0.25% |
| kernel_nway | 10217ns | 10217ns | -2.26% |
| kernel_seq | 10529ns | 10529ns | +0.72% |
| nway | 59496ns | 59496ns | +469.14% |
| sequential | 55786ns | 55786ns | +433.64% |
| std_parallel | 26371ns | 26371ns | +152.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 9640ns | base | --- | [9540, 10069] | --- | --- | --- | --- |
| head_tail_dyn | 9621ns | no significant difference | [-406, +81]ns | [9567, 9738] | no | 0.9774 | 0.8145 | 0 |
| kernel_nway | 9548ns | no significant difference | [-373, +19]ns | [9523, 9615] | no | 0.3568 | 0.2379 | 0 |
| kernel_seq | 9892ns | no significant difference | [-279, +402]ns | [9600, 10131] | no | 1.0000 | 1.0000 | 0 |
| nway | 59046ns | +49014.6ns (+508.5%) | [+48821, +49542]ns | [58535, 59406] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 55015ns | +45150.0ns (+468.4%) | [+44794, +46188]ns | [54744, 55712] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 24448ns | +14739.6ns (+152.9%) | [+14512, +15681]ns | [24190, 25431] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 9810ns | -1.7% | -2.9% | +4.1% | +503.9% | +454.8% | +169.6% |
| 1 | 2 | 9973ns | -3.5% | -3.3% | -2.5% | +490.0% | +450.6% | +143.6% |
| 1 | 3 | 9540ns | +1.1% | +0.1% | +0.6% | +516.6% | +490.7% | +154.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.412 | moderate- |
| head_tail_dyn | -0.651 | HIGH- (thermal bounce) |
| kernel_nway | -0.651 | HIGH- (thermal bounce) |
| kernel_seq | -0.073 | ok |
| nway | -0.138 | ok |
| sequential | -0.052 | ok |
| std_parallel | -0.167 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 0/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 443047.7ns | 9874.1ns | 4487.0% | HIGH |
| head_tail_dyn | 441617.8ns | 9691.7ns | 4556.7% | HIGH |
| kernel_nway | 442700.0ns | 9588.9ns | 4616.8% | HIGH |
| kernel_seq | 443807.2ns | 9920.1ns | 4473.8% | HIGH |
| nway | 468206.9ns | 59083.8ns | 792.4% | HIGH |
| sequential | 463452.1ns | 55420.8ns | 836.2% | HIGH |
| std_parallel | 454247.7ns | 25089.4ns | 1810.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 9497.2-10432.6 ns)
   9497.2 |########################################
   9544.0 |####################
   9590.8 |##########
   9637.5 |##########
   9684.3 |
   9731.1 |##########
   9777.9 |
   9824.6 |
   9871.4 |##########
   9918.2 |
   9964.9 |
  10011.7 |
  10058.5 |####################
  10105.2 |
  10152.0 |##########
  10198.8 |
  10245.6 |
  10292.3 |##########
  10339.1 |
  10385.9 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 9494.4-9910.4 ns)
   9494.4 |
   9515.2 |####################
   9536.0 |########################################
   9556.8 |
   9577.6 |########################################
   9598.4 |########################################
   9619.2 |####################
   9640.0 |
   9660.8 |
   9681.6 |####################
   9702.4 |####################
   9723.2 |########################################
   9744.0 |
   9764.8 |####################
   9785.6 |####################
   9806.4 |
   9827.2 |
   9848.0 |
   9868.8 |
   9889.6 |
  (2 below, 2 above range)

kernel_nway (n=18, range 9466.7-9715.3 ns)
   9466.7 |
   9479.1 |##########
   9491.5 |
   9504.0 |##########
   9516.4 |########################################
   9528.8 |####################
   9541.2 |
   9553.7 |##########
   9566.1 |
   9578.5 |##########
   9591.0 |
   9603.4 |####################
   9615.8 |##########
   9628.3 |
   9640.7 |
   9653.1 |
   9665.5 |##########
   9678.0 |
   9690.4 |
   9702.8 |
  (1 below, 3 above range)

kernel_seq (n=18, range 9497.2-10345.8 ns)
   9497.2 |#############
   9539.6 |
   9582.1 |########################################
   9624.5 |
   9666.9 |#############
   9709.4 |#############
   9751.8 |
   9794.2 |
   9836.7 |#############
   9879.1 |
   9921.5 |##########################
   9964.0 |
  10006.4 |#############
  10048.8 |#############
  10091.3 |
  10133.7 |
  10176.1 |#############
  10218.6 |#############
  10261.0 |
  10303.4 |
  (2 below, 3 above range)

nway (n=18, range 58326.4-59808.3 ns)
  58326.4 |####################
  58400.5 |########################################
  58474.6 |####################
  58548.7 |####################
  58622.8 |####################
  58696.9 |
  58771.0 |
  58845.0 |
  58919.1 |####################
  58993.2 |####################
  59067.3 |########################################
  59141.4 |
  59215.5 |
  59289.6 |########################################
  59363.7 |
  59437.8 |####################
  59511.9 |
  59586.0 |
  59660.1 |####################
  59734.2 |####################
  (1 below, 2 above range)

sequential (n=18, range 54370.8-56697.2 ns)
  54370.8 |########################################
  54487.2 |
  54603.5 |#############
  54719.8 |#############
  54836.1 |########################################
  54952.4 |
  55068.8 |#############
  55185.1 |#############
  55301.4 |#############
  55417.7 |
  55534.0 |#############
  55650.4 |
  55766.7 |#############
  55883.0 |
  55999.3 |#############
  56115.6 |
  56232.0 |
  56348.3 |
  56464.6 |
  56580.9 |
  (1 below, 3 above range)

std_parallel (n=18, range 23952.8-26582.6 ns)
  23952.8 |##########################
  24084.3 |##########################
  24215.8 |########################################
  24347.3 |##########################
  24478.8 |
  24610.3 |
  24741.8 |
  24873.2 |
  25004.7 |#############
  25136.2 |#############
  25267.7 |#############
  25399.2 |#############
  25530.7 |#############
  25662.2 |
  25793.7 |
  25925.2 |
  26056.7 |
  26188.2 |
  26319.6 |
  26451.1 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=4599.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=4588.6% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=4632.6% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=4470.1% of algo (FFI overhead may distort results)
- **nway**: bridge=792.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=833.0% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=1841.2% of algo (FFI overhead may distort results)

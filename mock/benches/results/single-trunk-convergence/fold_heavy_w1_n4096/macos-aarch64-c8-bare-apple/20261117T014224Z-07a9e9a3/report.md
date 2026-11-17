# fold_heavy, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 4.3x slower than the field

std_parallel (23.70 us) is 4.3x the fastest (5.45 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (kernel_seq, kernel_nway) are a dead heat (<1%)

kernel_seq (5.45 us) and kernel_nway (5.51 us) differ by 0.99%, inside the noise, even though the wider field spreads 334.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail shows alternating (throttle bounce) (autocorr -0.66)

head_tail's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_seq, kernel_nway, head_tail_dyn, head_tail} vs {sequential, nway, std_parallel} (130% apart)

The field splits into a fast tier {kernel_seq, kernel_nway, head_tail_dyn, head_tail} and a slow tier {sequential, nway, std_parallel} with a 130% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.3x the fastest

Fastest kernel_seq (5.45 us) to slowest std_parallel (23.70 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 5454.1 ns median (-2.5% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 4.35x (fastest 5454.1 ns, slowest 23702.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 1 |
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
| head_tail | 6335ns | 6117ns | 5718ns | 6015ns | 7122ns | base |
| head_tail_dyn | 6167ns | 6000ns | 5774ns | 5959ns | 6676ns | -2.65% |
| kernel_nway | 6070ns | 6004ns | 5775ns | 5969ns | 6370ns | -4.18% |
| kernel_seq | 5946ns | 5931ns | 5789ns | 5900ns | 6092ns | -6.14% |
| nway | 15666ns | 15633ns | 15403ns | 15613ns | 15878ns | +147.31% |
| sequential | 13260ns | 12990ns | 12653ns | 12943ns | 14039ns | +109.32% |
| std_parallel | 24583ns | 24098ns | 23579ns | 24026ns | 25920ns | +288.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 5850ns | 5399ns | 6552ns | base | 0.700 |
| head_tail_dyn | 5705ns | 5386ns | 6177ns | -2.48% | 0.718 |
| kernel_nway | 5611ns | 5400ns | 5909ns | -4.09% | 0.730 |
| kernel_seq | 5500ns | 5403ns | 5631ns | -5.98% | 0.745 |
| nway | 15328ns | 15103ns | 15513ns | +162.01% | 0.267 |
| sequential | 13096ns | 12510ns | 13844ns | +123.85% | 0.313 |
| std_parallel | 24182ns | 23168ns | 25514ns | +313.35% | 0.169 |

## Performance model

- Peak throughput: **0.760 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.732 | 96.3% |
| head_tail_dyn | 0.744 | 97.8% |
| kernel_nway | 0.744 | 97.8% |
| kernel_seq | 0.751 | 98.8% |
| nway | 0.268 | 35.2% |
| sequential | 0.319 | 41.9% |
| std_parallel | 0.173 | 22.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 6335ns | 6335ns | base |
| head_tail_dyn | 6167ns | 6167ns | -2.65% |
| kernel_nway | 6070ns | 6070ns | -4.18% |
| kernel_seq | 5946ns | 5946ns | -6.14% |
| nway | 15666ns | 15666ns | +147.31% |
| sequential | 13260ns | 13260ns | +109.32% |
| std_parallel | 24583ns | 24583ns | +288.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 5596ns | base | --- | [5425, 6392] | --- | --- | --- | --- |
| head_tail_dyn | 5508ns | no significant difference | [-831, +123]ns | [5460, 5606] | no | 0.9774 | 0.8145 | 0 |
| kernel_nway | 5508ns | no significant difference | [-783, +102]ns | [5444, 5598] | no | 0.9774 | 0.8145 | 0 |
| kernel_seq | 5454ns | no significant difference | [-977, +44]ns | [5421, 5548] | no | 1.0000 | 1.0000 | 0 |
| nway | 15308ns | +9681.2ns (+173.0%) | [+9133, +9767]ns | [15240, 15375] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 12846ns | +7100.0ns (+126.9%) | [+6525, +7788]ns | [12696, 13375] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 23702ns | +18014.6ns (+321.9%) | [+17550, +18931]ns | [23425, 24352] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 5425ns | +7.8% | +1.8% | +1.7% | +180.6% | +130.7% | +364.7% |
| 1 | 2 | 6585ns | -16.5% | -16.7% | -17.8% | +133.7% | +94.7% | +253.9% |
| 1 | 3 | 5527ns | -1.2% | +1.2% | -0.4% | +176.7% | +151.8% | +326.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.662 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.116 | ok |
| kernel_nway | -0.390 | moderate- |
| kernel_seq | -0.656 | HIGH- (thermal bounce) |
| nway | -0.557 | HIGH- (thermal bounce) |
| sequential | -0.064 | ok |
| std_parallel | -0.247 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 89836.3ns | 5850.2ns | 1535.6% | HIGH |
| head_tail_dyn | 88663.2ns | 5704.9ns | 1554.2% | HIGH |
| kernel_nway | 87856.7ns | 5611.1ns | 1565.8% | HIGH |
| kernel_seq | 86786.6ns | 5500.2ns | 1577.9% | HIGH |
| nway | 91295.4ns | 15328.0ns | 595.6% | HIGH |
| sequential | 91080.3ns | 13095.8ns | 695.5% | HIGH |
| std_parallel | 98393.8ns | 24182.2ns | 406.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 5398.6-6552.1 ns)
   5398.6 |########################################
   5456.3 |################
   5513.9 |
   5571.6 |################
   5629.3 |########
   5687.0 |########
   5744.6 |
   5802.3 |
   5860.0 |
   5917.7 |
   5975.3 |
   6033.0 |
   6090.7 |
   6148.4 |
   6206.0 |
   6263.7 |########
   6321.4 |
   6379.1 |
   6436.7 |########
   6494.4 |
  (1 below, 4 above range)

head_tail_dyn (n=18, range 5386.1-6177.1 ns)
   5386.1 |##############################
   5425.7 |##########
   5465.2 |########################################
   5504.8 |##########
   5544.3 |##############################
   5583.9 |
   5623.4 |##########
   5663.0 |##########
   5702.5 |
   5742.1 |
   5781.6 |
   5821.1 |
   5860.7 |
   5900.2 |
   5939.8 |
   5979.3 |
   6018.9 |
   6058.4 |
   6098.0 |##########
   6137.5 |
  (1 below, 2 above range)

kernel_nway (n=18, range 5400.0-5909.0 ns)
   5400.0 |########################################
   5425.5 |
   5450.9 |####################
   5476.4 |
   5501.8 |########################################
   5527.3 |##########
   5552.7 |
   5578.2 |##########
   5603.6 |
   5629.1 |
   5654.5 |####################
   5680.0 |##########
   5705.4 |
   5730.9 |
   5756.3 |
   5781.8 |
   5807.2 |
   5832.7 |
   5858.1 |
   5883.6 |
  (1 below, 2 above range)

kernel_seq (n=18, range 5402.8-5630.6 ns)
   5402.8 |########################################
   5414.2 |##########################
   5425.5 |##########################
   5436.9 |#############
   5448.3 |
   5459.7 |##########################
   5471.1 |
   5482.5 |
   5493.9 |
   5505.3 |
   5516.7 |#############
   5528.0 |
   5539.4 |##########################
   5550.8 |
   5562.2 |
   5573.6 |#############
   5585.0 |
   5596.4 |#############
   5607.8 |
   5619.2 |
  (1 below, 2 above range)

nway (n=18, range 15102.8-15513.2 ns)
  15102.8 |
  15123.3 |
  15143.8 |
  15164.4 |#############
  15184.9 |
  15205.4 |#############
  15225.9 |#############
  15246.4 |########################################
  15267.0 |
  15287.5 |#############
  15308.0 |##########################
  15328.5 |
  15349.0 |#############
  15369.6 |##########################
  15390.1 |#############
  15410.6 |
  15431.1 |
  15451.6 |
  15472.2 |
  15492.7 |
  (2 below, 3 above range)

sequential (n=18, range 12509.7-13844.4 ns)
  12509.7 |####################
  12576.4 |##########
  12643.2 |
  12709.9 |
  12776.6 |########################################
  12843.4 |####################
  12910.1 |
  12976.9 |####################
  13043.6 |
  13110.3 |
  13177.1 |
  13243.8 |
  13310.5 |
  13377.3 |
  13444.0 |
  13510.8 |
  13577.5 |
  13644.2 |
  13711.0 |##########
  13777.7 |##########
  (2 below, 3 above range)

std_parallel (n=18, range 23168.1-25513.9 ns)
  23168.1 |#############
  23285.4 |#############
  23402.6 |########################################
  23519.9 |
  23637.2 |########################################
  23754.5 |#############
  23871.8 |
  23989.1 |#############
  24106.4 |#############
  24223.7 |
  24341.0 |
  24458.3 |
  24575.6 |#############
  24692.8 |
  24810.1 |
  24927.4 |
  25044.7 |#############
  25162.0 |#############
  25279.3 |
  25396.6 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=1574.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=1586.3% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1595.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=1593.5% of algo (FFI overhead may distort results)
- **nway**: bridge=597.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=701.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=412.5% of algo (FFI overhead may distort results)

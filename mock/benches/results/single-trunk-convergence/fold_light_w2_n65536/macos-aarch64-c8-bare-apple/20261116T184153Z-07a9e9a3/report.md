# fold_light, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 4.2x slower than the field

std_parallel (37.21 us) is 4.2x the fastest (8.78 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (head_tail, kernel_seq) are a dead heat (<1%)

head_tail (8.78 us) and kernel_seq (8.81 us) differ by 0.38%, inside the noise, even though the wider field spreads 323.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### std_parallel shows alternating (throttle bounce) (autocorr -0.67)

std_parallel's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (8.78 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.2x the fastest

Fastest head_tail (8.78 us) to slowest std_parallel (37.21 us): 4.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (head_tail) is the fastest** at 8777.0 ns median
- 4 variants significantly slower than baseline
- Spread: 4.24x (fastest 8777.0 ns, slowest 37206.2 ns)

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
| head_tail | 9374ns | 9348ns | 8735ns | 9304ns | 9799ns | base |
| head_tail_dyn | 13274ns | 13110ns | 12735ns | 13063ns | 13860ns | +41.61% |
| kernel_nway | 9979ns | 9823ns | 8632ns | 9707ns | 11062ns | +6.46% |
| kernel_seq | 9609ns | 9269ns | 8792ns | 9169ns | 10679ns | +2.51% |
| nway | 23456ns | 23227ns | 22479ns | 23090ns | 24493ns | +150.22% |
| sequential | 16393ns | 16448ns | 15890ns | 16254ns | 16853ns | +74.88% |
| std_parallel | 39044ns | 37581ns | 34019ns | 36811ns | 44904ns | +316.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 8791ns | 8235ns | 9194ns | base | 7.455 |
| head_tail_dyn | 12665ns | 12190ns | 13121ns | +44.06% | 5.174 |
| kernel_nway | 9388ns | 8182ns | 10427ns | +6.78% | 6.981 |
| kernel_seq | 9185ns | 8468ns | 10219ns | +4.48% | 7.135 |
| nway | 22881ns | 21907ns | 23918ns | +160.27% | 2.864 |
| sequential | 16200ns | 15714ns | 16649ns | +84.28% | 4.045 |
| std_parallel | 38660ns | 33629ns | 44507ns | +339.75% | 1.695 |

## Performance model

- Peak throughput: **8.010 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 7.467 | 93.2% |
| head_tail_dyn | 5.199 | 64.9% |
| kernel_nway | 7.104 | 88.7% |
| kernel_seq | 7.438 | 92.9% |
| nway | 2.892 | 36.1% |
| sequential | 4.030 | 50.3% |
| std_parallel | 1.761 | 22.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 9374ns | 9374ns | base |
| head_tail_dyn | 13274ns | 13274ns | +41.61% |
| kernel_nway | 9979ns | 9979ns | +6.46% |
| kernel_seq | 9609ns | 9609ns | +2.51% |
| nway | 23456ns | 23456ns | +150.22% |
| sequential | 16393ns | 16393ns | +74.88% |
| std_parallel | 39044ns | 39044ns | +316.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 8777ns | base | --- | [8579, 9050] | --- | --- | --- | --- |
| head_tail_dyn | 12606ns | +3877.1ns (+44.2%) | [+3594, +4098]ns | [12358, 12854] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 9225ns | no significant difference | [-212, +954]ns | [8898, 9417] | no | 0.2855 | 0.2379 | 0 |
| kernel_seq | 8810ns | no significant difference | [-262, +325]ns | [8623, 9031] | no | 0.8145 | 0.8145 | 0 |
| nway | 22662ns | +14089.6ns (+160.5%) | [+13688, +14433]ns | [22348, 23225] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 16262ns | +7266.7ns (+82.8%) | [+7054, +7717]ns | [15767, 16502] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 37206ns | +28629.2ns (+326.2%) | [+26765, +31933]ns | [34969, 40604] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 9050ns | +36.9% | +1.9% | -4.7% | +154.6% | +81.3% | +336.0% |
| 1 | 2 | 8690ns | +43.3% | +7.2% | +4.8% | +160.7% | +81.3% | +320.5% |
| 1 | 3 | 8758ns | +47.7% | +0.1% | +0.7% | +157.8% | +86.3% | +349.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.279 | moderate- |
| head_tail_dyn | -0.109 | ok |
| kernel_nway | -0.267 | moderate- |
| kernel_seq | -0.554 | HIGH- (thermal bounce) |
| nway | -0.083 | ok |
| sequential | -0.657 | HIGH- (thermal bounce) |
| std_parallel | -0.666 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 88724.5ns | 8791.4ns | 1009.2% | HIGH |
| head_tail_dyn | 92144.2ns | 12665.3ns | 727.5% | HIGH |
| kernel_nway | 89063.4ns | 9387.7ns | 948.7% | HIGH |
| kernel_seq | 92093.1ns | 9185.0ns | 1002.7% | HIGH |
| nway | 104533.6ns | 22881.2ns | 456.9% | HIGH |
| sequential | 95433.6ns | 16200.5ns | 589.1% | HIGH |
| std_parallel | 110259.7ns | 38660.2ns | 285.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 8234.7-9194.4 ns)
   8234.7 |
   8282.7 |
   8330.7 |####################
   8378.7 |
   8426.7 |####################
   8474.7 |####################
   8522.6 |
   8570.6 |
   8618.6 |
   8666.6 |########################################
   8714.6 |########################################
   8762.6 |########################################
   8810.6 |
   8858.5 |
   8906.5 |####################
   8954.5 |
   9002.5 |####################
   9050.5 |########################################
   9098.5 |
   9146.4 |####################
  (2 below, 2 above range)

head_tail_dyn (n=18, range 12190.3-13120.9 ns)
  12190.3 |########################################
  12236.8 |
  12283.3 |########################################
  12329.9 |####################
  12376.4 |
  12422.9 |
  12469.4 |
  12516.0 |########################################
  12562.5 |####################
  12609.0 |########################################
  12655.6 |
  12702.1 |####################
  12748.6 |####################
  12795.1 |
  12841.7 |
  12888.2 |####################
  12934.7 |####################
  12981.3 |####################
  13027.8 |
  13074.3 |
  (1 below, 2 above range)

kernel_nway (n=18, range 8181.9-10427.1 ns)
   8181.9 |
   8294.2 |#############
   8406.5 |
   8518.7 |
   8631.0 |
   8743.2 |##########################
   8855.5 |#############
   8967.7 |#############
   9080.0 |#############
   9192.3 |##########################
   9304.5 |########################################
   9416.8 |#############
   9529.0 |
   9641.3 |
   9753.6 |
   9865.8 |
   9978.1 |
  10090.3 |#############
  10202.6 |
  10314.8 |
  (2 below, 3 above range)

kernel_seq (n=18, range 8468.1-10218.7 ns)
   8468.1 |##########################
   8555.6 |##########################
   8643.1 |##########################
   8730.7 |########################################
   8818.2 |##########################
   8905.7 |
   8993.3 |##########################
   9080.8 |
   9168.3 |
   9255.9 |#############
   9343.4 |
   9430.9 |
   9518.5 |##########################
   9606.0 |
   9693.5 |
   9781.1 |
   9868.6 |
   9956.1 |
  10043.7 |
  10131.2 |
  (1 below, 1 above range)

nway (n=18, range 21906.9-23918.0 ns)
  21906.9 |#############
  22007.5 |
  22108.0 |#############
  22208.6 |#############
  22309.2 |
  22409.7 |##########################
  22510.3 |#############
  22610.8 |########################################
  22711.4 |#############
  22811.9 |
  22912.5 |
  23013.0 |#############
  23113.6 |
  23214.2 |
  23314.7 |
  23415.3 |#############
  23515.8 |
  23616.4 |
  23716.9 |#############
  23817.5 |
  (2 below, 3 above range)

sequential (n=18, range 15713.9-16649.3 ns)
  15713.9 |########################################
  15760.7 |########################################
  15807.4 |
  15854.2 |
  15901.0 |
  15947.8 |########################################
  15994.5 |
  16041.3 |
  16088.1 |
  16134.8 |
  16181.6 |####################
  16228.4 |
  16275.1 |
  16321.9 |####################
  16368.7 |####################
  16415.5 |########################################
  16462.2 |
  16509.0 |
  16555.8 |########################################
  16602.5 |
  (2 below, 3 above range)

std_parallel (n=18, range 33629.2-44507.0 ns)
  33629.2 |####################
  34173.1 |########################################
  34716.9 |########################################
  35260.8 |
  35804.7 |####################
  36348.6 |########################################
  36892.5 |
  37436.4 |########################################
  37980.3 |####################
  38524.2 |
  39068.1 |
  39611.9 |
  40155.8 |####################
  40699.7 |####################
  41243.6 |
  41787.5 |
  42331.4 |
  42875.3 |
  43419.2 |
  43963.1 |####################
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=997.0% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=740.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=955.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=1051.6% of algo (FFI overhead may distort results)
- **nway**: bridge=459.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=583.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=293.1% of algo (FFI overhead may distort results)

# fold_light, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 6.8x slower than the field

std_parallel (87.33 us) is 6.8x the fastest (12.89 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.66)

head_tail's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (12.89 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 6.8x the fastest

Fastest head_tail (12.89 us) to slowest std_parallel (87.33 us): 6.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 4.3x its best-20%

sequential's best 20% of batches run at 7.11 us but its worst 20% at 30.78 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 12892.0 ns median
- 3 variants significantly slower than baseline
- Spread: 6.77x (fastest 12892.0 ns, slowest 87330.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 4 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 16316ns | 14133ns | 12605ns | 14232ns | 21297ns | base |
| head_tail_dyn | 18511ns | 19196ns | 13823ns | 17616ns | 22197ns | +13.45% |
| kernel_nway | 16773ns | 16811ns | 13148ns | 16376ns | 19179ns | +2.80% |
| kernel_seq | 15173ns | 14186ns | 12377ns | 13930ns | 18435ns | -7.01% |
| nway | 50154ns | 50172ns | 42027ns | 49096ns | 55805ns | +207.39% |
| sequential | 20548ns | 24194ns | 7259ns | 17752ns | 31387ns | +25.94% |
| std_parallel | 87355ns | 88465ns | 34716ns | 81887ns | 121877ns | +435.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 15394ns | 12041ns | 20223ns | base | 0.266 |
| head_tail_dyn | 17533ns | 13186ns | 20823ns | +13.90% | 0.234 |
| kernel_nway | 15678ns | 12587ns | 17798ns | +1.85% | 0.261 |
| kernel_seq | 14049ns | 11549ns | 16868ns | -8.74% | 0.292 |
| nway | 49528ns | 41491ns | 55115ns | +221.74% | 0.083 |
| sequential | 20155ns | 7114ns | 30785ns | +30.93% | 0.203 |
| std_parallel | 86493ns | 34541ns | 120475ns | +461.88% | 0.047 |

## Performance model

- Peak throughput: **0.576 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.318 | 55.2% |
| head_tail_dyn | 0.221 | 38.5% |
| kernel_nway | 0.256 | 44.5% |
| kernel_seq | 0.308 | 53.5% |
| nway | 0.083 | 14.4% |
| sequential | 0.173 | 30.0% |
| std_parallel | 0.047 | 8.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 16316ns | 16316ns | base |
| head_tail_dyn | 18511ns | 18511ns | +13.45% |
| kernel_nway | 16773ns | 16773ns | +2.80% |
| kernel_seq | 15173ns | 15173ns | -7.01% |
| nway | 50154ns | 50154ns | +207.39% |
| sequential | 20548ns | 20548ns | +25.94% |
| std_parallel | 87355ns | 87355ns | +435.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 12892ns | base | --- | [12459, 17240] | --- | --- | --- | --- |
| head_tail_dyn | 18495ns | +1246.6ns (+9.7%) | [+671, +4458]ns | [13388, 18970] | YES | 0.0151 | 0.0075 | 0 |
| kernel_nway | 15970ns | no significant difference | [-1081, +2233]ns | [13542, 17603] | no | 0.8145 | 0.8145 | 0 |
| kernel_seq | 13300ns | no significant difference | [-5688, +2500]ns | [12317, 15980] | no | 0.8145 | 0.8145 | 0 |
| nway | 49544ns | +36065.1ns (+279.7%) | [+29978, +39339]ns | [47428, 52862] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 23744ns | no significant difference | [-8965, +14207]ns | [7689, 28162] | no | 0.7210 | 0.4807 | 0 |
| std_parallel | 87330ns | +72522.2ns (+562.5%) | [+57307, +95841]ns | [72113, 108923] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 12459ns | +6.4% | +7.8% | +5.5% | +325.8% | +101.0% | +798.1% |
| 1 | 2 | 17392ns | +8.7% | -8.2% | -30.5% | +145.7% | -58.4% | +393.0% |
| 1 | 3 | 12892ns | +45.5% | +38.4% | +26.8% | +281.6% | +134.4% | +177.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.662 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.181 | ok |
| kernel_nway | -0.005 | ok |
| kernel_seq | -0.319 | moderate- |
| nway | -0.576 | HIGH- (thermal bounce) |
| sequential | -0.636 | HIGH- (thermal bounce) |
| std_parallel | -0.021 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 15897.7ns | 15393.5ns | 103.3% | HIGH |
| head_tail_dyn | 18196.0ns | 17533.0ns | 103.8% | HIGH |
| kernel_nway | 16523.3ns | 15677.7ns | 105.4% | HIGH |
| kernel_seq | 14289.7ns | 14048.6ns | 101.7% | HIGH |
| nway | 49462.6ns | 49527.8ns | 99.9% | HIGH |
| sequential | 21016.6ns | 20155.3ns | 104.3% | HIGH |
| std_parallel | 86700.3ns | 86493.4ns | 100.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 12040.9-20223.3 ns)
  12040.9 |########################################
  12450.1 |########################################
  12859.2 |##########
  13268.3 |
  13677.4 |
  14086.5 |
  14495.6 |
  14904.8 |##########
  15313.9 |
  15723.0 |##########
  16132.1 |
  16541.2 |
  16950.4 |####################
  17359.5 |##########
  17768.6 |
  18177.7 |
  18586.8 |
  18995.9 |
  19405.1 |
  19814.2 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 13186.4-20823.4 ns)
  13186.4 |########################################
  13568.2 |
  13950.1 |
  14331.9 |
  14713.8 |
  15095.6 |
  15477.5 |
  15859.3 |
  16241.2 |
  16623.0 |
  17004.9 |
  17386.7 |
  17768.6 |
  18150.4 |################################
  18532.3 |################
  18914.1 |########################
  19296.0 |
  19677.8 |
  20059.7 |
  20441.5 |
  (1 below, 3 above range)

kernel_nway (n=18, range 12587.3-17798.4 ns)
  12587.3 |
  12847.8 |
  13108.4 |
  13368.9 |########################################
  13629.5 |
  13890.0 |
  14150.6 |
  14411.1 |
  14671.7 |
  14932.3 |
  15192.8 |
  15453.4 |##########
  15713.9 |####################
  15974.5 |####################
  16235.0 |
  16495.6 |
  16756.1 |
  17016.7 |
  17277.2 |####################
  17537.8 |##########
  (2 below, 4 above range)

kernel_seq (n=18, range 11548.6-16867.7 ns)
  11548.6 |####################
  11814.6 |
  12080.5 |########################################
  12346.5 |########################################
  12612.4 |########################################
  12878.4 |
  13144.3 |
  13410.3 |
  13676.2 |####################
  13942.2 |####################
  14208.2 |
  14474.1 |####################
  14740.1 |
  15006.0 |
  15272.0 |
  15537.9 |
  15803.9 |########################################
  16069.8 |####################
  16335.8 |####################
  16601.8 |
  (2 below, 2 above range)

nway (n=18, range 41491.4-55115.4 ns)
  41491.4 |#############
  42172.6 |#############
  42853.8 |#############
  43535.0 |
  44216.2 |
  44897.4 |
  45578.6 |
  46259.8 |
  46941.0 |#############
  47622.2 |########################################
  48303.4 |#############
  48984.6 |
  49665.8 |#############
  50347.0 |
  51028.2 |##########################
  51709.4 |
  52390.6 |#############
  53071.8 |#############
  53753.0 |##########################
  54434.2 |
  (1 below, 2 above range)

sequential (n=18, range 7114.2-30784.8 ns)
   7114.2 |########################################
   8297.7 |
   9481.3 |
  10664.8 |########
  11848.3 |
  13031.8 |
  14215.4 |
  15398.9 |
  16582.4 |
  17766.0 |
  18949.5 |
  20133.0 |
  21316.5 |
  22500.1 |################
  23683.6 |########
  24867.1 |
  26050.7 |########################
  27234.2 |
  28417.7 |########
  29601.3 |########
  (1 below, 3 above range)

std_parallel (n=18, range 34540.9-120475.0 ns)
  34540.9 |########################################
  38837.6 |
  43134.3 |
  47431.0 |
  51727.7 |
  56024.4 |
  60321.1 |
  64617.8 |####################
  68914.5 |
  73211.2 |########################################
  77507.9 |
  81804.6 |####################
  86101.3 |########################################
  90398.0 |
  94694.7 |
  98991.4 |
  103288.1 |########################################
  107584.9 |########################################
  111881.6 |####################
  116178.3 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=26.3% (high variance, measurements may be unstable)
- **head_tail**: bridge=120.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=104.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=103.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=103.1% of algo (FFI overhead may distort results)
- **nway**: bridge=99.3% of algo (FFI overhead may distort results)
- **sequential**: CV=51.1% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **sequential**: bridge=102.9% of algo (FFI overhead may distort results)
- **std_parallel**: CV=38.4% (high variance, measurements may be unstable)
- **std_parallel**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **std_parallel**: bridge=101.2% of algo (FFI overhead may distort results)

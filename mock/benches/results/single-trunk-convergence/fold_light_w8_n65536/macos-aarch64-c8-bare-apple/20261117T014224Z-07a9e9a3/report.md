# fold_light, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 10.5x slower than the field

std_parallel (90.41 us) is 10.5x the fastest (8.65 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (head_tail, kernel_seq) are a dead heat (<1%)

head_tail (8.65 us) and kernel_seq (8.65 us) differ by 0.00%, inside the noise, even though the wider field spreads 945.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.61)

kernel_seq's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (8.65 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 10.5x the fastest

Fastest head_tail (8.65 us) to slowest std_parallel (90.41 us): 10.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (head_tail) is the fastest** at 8650.0 ns median
- 5 variants significantly slower than baseline
- Spread: 10.45x (fastest 8650.0 ns, slowest 90412.5 ns)

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
| head_tail | 9614ns | 9290ns | 8753ns | 9236ns | 10613ns | base |
| head_tail_dyn | 12824ns | 12871ns | 12454ns | 12748ns | 13122ns | +33.38% |
| kernel_nway | 27159ns | 25456ns | 22104ns | 25181ns | 32651ns | +182.48% |
| kernel_seq | 9105ns | 9065ns | 8706ns | 9042ns | 9400ns | -5.30% |
| nway | 55023ns | 54619ns | 50682ns | 54225ns | 58391ns | +472.30% |
| sequential | 16553ns | 16496ns | 16265ns | 16451ns | 16849ns | +72.17% |
| std_parallel | 91033ns | 90777ns | 80292ns | 88168ns | 100701ns | +846.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 9004ns | 8233ns | 9969ns | base | 7.279 |
| head_tail_dyn | 12300ns | 12054ns | 12528ns | +36.62% | 5.328 |
| kernel_nway | 26510ns | 21439ns | 32033ns | +194.44% | 2.472 |
| kernel_seq | 8691ns | 8385ns | 8960ns | -3.47% | 7.541 |
| nway | 54454ns | 50124ns | 57804ns | +504.80% | 1.204 |
| sequential | 16352ns | 16069ns | 16645ns | +81.61% | 4.008 |
| std_parallel | 90643ns | 79928ns | 100281ns | +906.73% | 0.723 |

## Performance model

- Peak throughput: **7.960 Gops/s** (head_tail; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 7.576 | 95.2% |
| head_tail_dyn | 5.353 | 67.2% |
| kernel_nway | 2.649 | 33.3% |
| kernel_seq | 7.576 | 95.2% |
| nway | 1.214 | 15.3% |
| sequential | 4.023 | 50.5% |
| std_parallel | 0.725 | 9.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 9614ns | 9614ns | base |
| head_tail_dyn | 12824ns | 12824ns | +33.38% |
| kernel_nway | 27159ns | 27159ns | +182.48% |
| kernel_seq | 9105ns | 9105ns | -5.30% |
| nway | 55023ns | 55023ns | +472.30% |
| sequential | 16553ns | 16553ns | +72.17% |
| std_parallel | 91033ns | 91033ns | +846.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 8650ns | base | --- | [8433, 9512] | --- | --- | --- | --- |
| head_tail_dyn | 12244ns | +3537.5ns (+40.9%) | [+2950, +3700]ns | [12150, 12419] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 24744ns | +15677.1ns (+181.2%) | [+14215, +19206]ns | [22708, 28242] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 8650ns | no significant difference | [-658, +192]ns | [8533, 8862] | no | 0.2379 | 0.2379 | 0 |
| nway | 53983ns | +45283.3ns (+523.5%) | [+43408, +47604]ns | [52744, 56804] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 16290ns | +7520.8ns (+86.9%) | [+7096, +7842]ns | [16175, 16433] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 90412ns | +81960.4ns (+947.5%) | [+75171, +85419]ns | [82779, 94698] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 8496ns | +43.3% | +162.9% | +3.0% | +502.7% | +91.4% | +964.2% |
| 1 | 2 | 8508ns | +43.9% | +206.0% | -0.3% | +553.4% | +90.1% | +858.9% |
| 1 | 3 | 9950ns | +25.1% | +183.8% | -10.9% | +476.0% | +65.2% | +889.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.162 | ok |
| head_tail_dyn | -0.047 | ok |
| kernel_nway | -0.014 | ok |
| kernel_seq | -0.610 | HIGH- (thermal bounce) |
| nway | -0.040 | ok |
| sequential | -0.373 | moderate- |
| std_parallel | -0.515 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 89696.5ns | 9003.7ns | 996.2% | HIGH |
| head_tail_dyn | 93677.1ns | 12300.5ns | 761.6% | HIGH |
| kernel_nway | 113583.6ns | 26510.4ns | 428.4% | HIGH |
| kernel_seq | 91592.8ns | 8691.0ns | 1053.9% | HIGH |
| nway | 111135.9ns | 54453.9ns | 204.1% | HIGH |
| sequential | 94469.4ns | 16351.8ns | 577.7% | HIGH |
| std_parallel | 128052.8ns | 90643.0ns | 141.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 8233.3-9968.8 ns)
   8233.3 |##########################
   8320.1 |##########################
   8406.9 |#############
   8493.6 |#############
   8580.4 |########################################
   8667.2 |
   8754.0 |
   8840.7 |#############
   8927.5 |#############
   9014.3 |
   9101.0 |
   9187.8 |
   9274.6 |
   9361.4 |#############
   9448.1 |
   9534.9 |#############
   9621.7 |
   9708.4 |
   9795.2 |
   9882.0 |#############
  (1 below, 3 above range)

head_tail_dyn (n=18, range 12054.2-12528.5 ns)
  12054.2 |####################
  12077.9 |
  12101.6 |########################################
  12125.3 |####################
  12149.0 |####################
  12172.7 |####################
  12196.5 |####################
  12220.2 |####################
  12243.9 |####################
  12267.6 |
  12291.3 |
  12315.0 |
  12338.8 |
  12362.5 |
  12386.2 |########################################
  12409.9 |########################################
  12433.6 |
  12457.3 |########################################
  12481.1 |####################
  12504.8 |
  (1 below, 1 above range)

kernel_nway (n=18, range 21438.9-32033.3 ns)
  21438.9 |#############
  21968.6 |##########################
  22498.3 |##########################
  23028.1 |
  23557.8 |#############
  24087.5 |##########################
  24617.2 |#############
  25146.9 |
  25676.7 |
  26206.4 |
  26736.1 |
  27265.8 |#############
  27795.5 |########################################
  28325.3 |
  28855.0 |#############
  29384.7 |
  29914.4 |
  30444.2 |
  30973.9 |#############
  31503.6 |
  (1 below, 2 above range)

kernel_seq (n=18, range 8384.7-8960.4 ns)
   8384.7 |#############
   8413.5 |
   8442.3 |
   8471.1 |
   8499.9 |##########################
   8528.7 |##########################
   8557.4 |#############
   8586.2 |
   8615.0 |#############
   8643.8 |#############
   8672.6 |#############
   8701.4 |
   8730.1 |
   8758.9 |
   8787.7 |
   8816.5 |#############
   8845.3 |########################################
   8874.1 |
   8902.8 |
   8931.6 |
  (2 below, 3 above range)

nway (n=18, range 50123.6-57804.2 ns)
  50123.6 |####################
  50507.6 |####################
  50891.7 |
  51275.7 |
  51659.7 |####################
  52043.7 |####################
  52427.8 |
  52811.8 |####################
  53195.8 |########################################
  53579.9 |####################
  53963.9 |
  54347.9 |####################
  54731.9 |
  55116.0 |####################
  55500.0 |####################
  55884.0 |
  56268.1 |
  56652.1 |########################################
  57036.1 |
  57420.2 |####################
  (1 below, 3 above range)

sequential (n=18, range 16069.4-16645.1 ns)
  16069.4 |
  16098.2 |########################################
  16127.0 |####################
  16155.8 |
  16184.6 |########################################
  16213.4 |####################
  16242.1 |
  16270.9 |########################################
  16299.7 |
  16328.5 |
  16357.3 |####################
  16386.1 |########################################
  16414.9 |
  16443.6 |####################
  16472.4 |
  16501.2 |####################
  16530.0 |
  16558.8 |
  16587.6 |####################
  16616.3 |
  (2 below, 2 above range)

std_parallel (n=18, range 79927.8-100281.3 ns)
  79927.8 |
  80945.4 |########################################
  81963.1 |####################
  82980.8 |####################
  83998.5 |
  85016.1 |
  86033.8 |####################
  87051.5 |
  88069.2 |
  89086.8 |########################################
  90104.5 |####################
  91122.2 |
  92139.9 |########################################
  93157.5 |
  94175.2 |########################################
  95192.9 |
  96210.6 |
  97228.2 |
  98245.9 |########################################
  99263.6 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=1032.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=769.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=457.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=1063.6% of algo (FFI overhead may distort results)
- **nway**: bridge=205.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=581.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=145.5% of algo (FFI overhead may distort results)

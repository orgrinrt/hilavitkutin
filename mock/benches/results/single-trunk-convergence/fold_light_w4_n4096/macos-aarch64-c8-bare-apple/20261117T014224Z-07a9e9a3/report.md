# fold_light, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 46% faster than the next best (head_tail_dyn)

kernel_seq (2.96 us) leads head_tail_dyn (4.33 us) by 46%, a clear separation rather than a photo finish. CV 7.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 31% (significant)

kernel_seq is -1.39 us (31%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 18.9x slower than the field

std_parallel (56.08 us) is 18.9x the fastest (2.96 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_parallel shows alternating (throttle bounce) (autocorr -0.66)

std_parallel's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 18.9x the fastest

Fastest kernel_seq (2.96 us) to slowest std_parallel (56.08 us): 18.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2962.5 ns median (-33.1% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 18.93x (fastest 2962.5 ns, slowest 56075.0 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 5160ns | 5129ns | 4575ns | 4975ns | 5730ns | base |
| head_tail_dyn | 4914ns | 4896ns | 4742ns | 4878ns | 5053ns | -4.77% |
| kernel_nway | 11837ns | 11883ns | 9718ns | 11767ns | 13001ns | +129.40% |
| kernel_seq | 3449ns | 3475ns | 3021ns | 3342ns | 3824ns | -33.15% |
| nway | 34936ns | 34658ns | 32667ns | 34358ns | 36937ns | +577.09% |
| sequential | 10490ns | 10306ns | 10022ns | 10231ns | 11112ns | +103.31% |
| std_parallel | 56701ns | 56510ns | 51704ns | 55249ns | 61377ns | +998.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4540ns | 3931ns | 5144ns | base | 0.902 |
| head_tail_dyn | 4337ns | 4194ns | 4458ns | -4.46% | 0.944 |
| kernel_nway | 11208ns | 9168ns | 12350ns | +146.88% | 0.365 |
| kernel_seq | 3004ns | 2758ns | 3262ns | -33.83% | 1.364 |
| nway | 34377ns | 32064ns | 36383ns | +657.23% | 0.119 |
| sequential | 10337ns | 9885ns | 10950ns | +127.69% | 0.396 |
| std_parallel | 56282ns | 51296ns | 60944ns | +1139.74% | 0.073 |

## Performance model

- Peak throughput: **1.485 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.926 | 62.3% |
| head_tail_dyn | 0.945 | 63.7% |
| kernel_nway | 0.363 | 24.4% |
| kernel_seq | 1.383 | 93.1% |
| nway | 0.120 | 8.1% |
| sequential | 0.403 | 27.2% |
| std_parallel | 0.073 | 4.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 5160ns | 5160ns | base |
| head_tail_dyn | 4914ns | 4914ns | -4.77% |
| kernel_nway | 11837ns | 11837ns | +129.40% |
| kernel_seq | 3449ns | 3449ns | -33.15% |
| nway | 34936ns | 34936ns | +577.09% |
| sequential | 10490ns | 10490ns | +103.31% |
| std_parallel | 56701ns | 56701ns | +998.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4425ns | base | --- | [4135, 4771] | --- | --- | --- | --- |
| head_tail_dyn | 4333ns | no significant difference | [-488, +169]ns | [4269, 4371] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 11298ns | +6914.5ns (+156.3%) | [+6408, +7306]ns | [10865, 11783] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2962ns | -1393.8ns (-31.5%) | [-1788, -1188]ns | [2819, 3150] | YES | 0.0000 | 0.0000 | 0 |
| nway | 34090ns | +29672.9ns (+670.6%) | [+28421, +31058]ns | [33146, 35217] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10158ns | +5600.0ns (+126.6%) | [+5377, +6438]ns | [9929, 10642] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 56075ns | +51306.2ns (+1159.5%) | [+48888, +54090]ns | [53196, 58598] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4588ns | -6.7% | +144.5% | -33.2% | +639.2% | +115.6% | +1085.1% |
| 1 | 2 | 4771ns | -9.0% | +150.6% | -33.8% | +581.6% | +112.9% | +1117.1% |
| 1 | 3 | 4073ns | +6.4% | +168.0% | -31.4% | +761.2% | +166.6% | +1240.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.330 | moderate- |
| head_tail_dyn | -0.222 | moderate- |
| kernel_nway | -0.614 | HIGH- (thermal bounce) |
| kernel_seq | -0.323 | moderate- |
| nway | -0.529 | HIGH- (thermal bounce) |
| sequential | -0.042 | ok |
| std_parallel | -0.665 | HIGH- (thermal bounce) |

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
| head_tail | 92472.7ns | 4539.8ns | 2036.9% | HIGH |
| head_tail_dyn | 96572.4ns | 4337.3ns | 2226.6% | HIGH |
| kernel_nway | 124322.2ns | 11208.1ns | 1109.2% | HIGH |
| kernel_seq | 92469.2ns | 3003.9ns | 3078.3% | HIGH |
| nway | 116803.0ns | 34376.9ns | 339.8% | HIGH |
| sequential | 89591.9ns | 10336.8ns | 866.7% | HIGH |
| std_parallel | 121700.5ns | 56282.2ns | 216.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 3930.6-5143.8 ns)
   3930.6 |##########################
   3991.2 |
   4051.9 |##########################
   4112.5 |
   4173.2 |########################################
   4233.9 |
   4294.5 |
   4355.2 |#############
   4415.8 |#############
   4476.5 |
   4537.2 |
   4597.8 |
   4658.5 |#############
   4719.1 |########################################
   4779.8 |#############
   4840.5 |
   4901.1 |
   4961.8 |
   5022.4 |
   5083.1 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 4194.5-4458.4 ns)
   4194.5 |#############
   4207.7 |
   4220.9 |##########################
   4234.0 |#############
   4247.2 |
   4260.4 |
   4273.6 |
   4286.8 |##########################
   4300.0 |
   4313.2 |#############
   4326.4 |########################################
   4339.6 |#############
   4352.8 |
   4366.0 |##########################
   4379.2 |#############
   4392.4 |
   4405.6 |#############
   4418.8 |
   4432.0 |
   4445.2 |
  (1 below, 2 above range)

kernel_nway (n=18, range 9168.1-12350.0 ns)
   9168.1 |
   9327.2 |
   9486.3 |
   9645.4 |
   9804.5 |
   9963.6 |####################
  10122.6 |
  10281.7 |
  10440.8 |####################
  10599.9 |
  10759.0 |########################################
  10918.1 |####################
  11077.2 |####################
  11236.3 |########################################
  11395.4 |####################
  11554.5 |####################
  11713.6 |########################################
  11872.7 |
  12031.8 |####################
  12190.9 |####################
  (2 below, 2 above range)

kernel_seq (n=18, range 2758.3-3261.8 ns)
   2758.3 |########################################
   2783.5 |####################
   2808.7 |####################
   2833.9 |
   2859.0 |####################
   2884.2 |
   2909.4 |####################
   2934.5 |####################
   2959.7 |
   2984.9 |####################
   3010.1 |####################
   3035.2 |
   3060.4 |####################
   3085.6 |
   3110.8 |
   3135.9 |########################################
   3161.1 |
   3186.3 |
   3211.5 |####################
   3236.6 |
  (2 below, 3 above range)

nway (n=18, range 32063.9-36383.3 ns)
  32063.9 |
  32279.8 |########################################
  32495.8 |####################
  32711.8 |
  32927.8 |####################
  33143.7 |####################
  33359.7 |####################
  33575.7 |
  33791.7 |########################################
  34007.6 |####################
  34223.6 |
  34439.6 |
  34655.5 |
  34871.5 |########################################
  35087.5 |########################################
  35303.5 |
  35519.4 |
  35735.4 |####################
  35951.4 |
  36167.4 |
  (1 below, 3 above range)

sequential (n=18, range 9884.7-10950.0 ns)
   9884.7 |####################
   9938.0 |######
   9991.2 |
  10044.5 |
  10097.8 |
  10151.0 |########################################
  10204.3 |
  10257.6 |
  10310.8 |
  10364.1 |
  10417.3 |
  10470.6 |
  10523.9 |
  10577.1 |
  10630.4 |######
  10683.7 |
  10736.9 |######
  10790.2 |######
  10843.5 |
  10896.7 |######
  (2 below, 2 above range)

std_parallel (n=18, range 51295.8-60944.5 ns)
  51295.8 |
  51778.2 |####################
  52260.7 |####################
  52743.1 |####################
  53225.5 |########################################
  53708.0 |
  54190.4 |####################
  54672.8 |
  55155.3 |
  55637.7 |####################
  56120.1 |########################################
  56602.6 |
  57085.0 |########################################
  57567.4 |
  58049.9 |
  58532.3 |
  59014.7 |
  59497.2 |####################
  59979.6 |########################################
  60462.0 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=2104.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2210.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1107.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3092.1% of algo (FFI overhead may distort results)
- **nway**: bridge=340.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=881.0% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=213.9% of algo (FFI overhead may distort results)

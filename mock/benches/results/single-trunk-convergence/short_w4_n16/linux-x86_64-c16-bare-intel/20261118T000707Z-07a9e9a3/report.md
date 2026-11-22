# fold_light, short fiber (1 to 4 morsels), W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 25% faster than the next best (sequential)

kernel_seq (10.09 us) leads sequential (12.60 us) by 25%, a clear separation rather than a photo finish. CV 17.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 4.2x slower than the field

std_parallel (42.32 us) is 4.2x the fastest (10.09 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.55)

nway's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_seq, sequential, head_tail, head_tail_dyn, kernel_nway} vs {nway, std_parallel} (188% apart)

The field splits into a fast tier {kernel_seq, sequential, head_tail, head_tail_dyn, kernel_nway} and a slow tier {nway, std_parallel} with a 188% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.2x the fastest

Fastest kernel_seq (10.09 us) to slowest std_parallel (42.32 us): 4.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.5x its best-20%

sequential's best 20% of batches run at 6.77 us but its worst 20% at 23.43 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 10088.3 ns median (-20.2% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 4.19x (fastest 10088.3 ns, slowest 42317.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 4 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 13451ns | 13424ns | 11242ns | 13037ns | 15176ns | base |
| head_tail_dyn | 13970ns | 13908ns | 11875ns | 13628ns | 15530ns | +3.86% |
| kernel_nway | 15572ns | 15576ns | 14086ns | 15263ns | 16778ns | +15.77% |
| kernel_seq | 10708ns | 10790ns | 8050ns | 10083ns | 12974ns | -20.39% |
| nway | 44533ns | 42587ns | 35667ns | 41153ns | 54035ns | +231.08% |
| sequential | 14609ns | 12859ns | 6904ns | 10969ns | 23922ns | +8.61% |
| std_parallel | 47835ns | 42550ns | 36148ns | 41174ns | 63671ns | +255.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 12638ns | 10647ns | 14207ns | base | 0.001 |
| head_tail_dyn | 13189ns | 11270ns | 14578ns | +4.36% | 0.001 |
| kernel_nway | 14709ns | 13358ns | 15861ns | +16.38% | 0.001 |
| kernel_seq | 10020ns | 7605ns | 12011ns | -20.72% | 0.002 |
| nway | 43914ns | 35210ns | 53244ns | +247.48% | 0.000 |
| sequential | 14314ns | 6768ns | 23428ns | +13.26% | 0.001 |
| std_parallel | 47486ns | 35961ns | 63059ns | +275.74% | 0.000 |

## Performance model

- Peak throughput: **0.002 Gops/s** (sequential; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.001 | 53.6% |
| head_tail_dyn | 0.001 | 51.0% |
| kernel_nway | 0.001 | 46.4% |
| kernel_seq | 0.002 | 67.1% |
| nway | 0.000 | 16.1% |
| sequential | 0.001 | 53.7% |
| std_parallel | 0.000 | 16.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 13451ns | 13451ns | base |
| head_tail_dyn | 13970ns | 13970ns | +3.86% |
| kernel_nway | 15572ns | 15572ns | +15.77% |
| kernel_seq | 10708ns | 10708ns | -20.39% |
| nway | 44533ns | 44533ns | +231.08% |
| sequential | 14609ns | 14609ns | +8.61% |
| std_parallel | 47835ns | 47835ns | +255.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 12638ns | base | --- | [11722, 13100] | --- | --- | --- | --- |
| head_tail_dyn | 13262ns | no significant difference | [-606, +2432]ns | [12562, 13891] | no | 0.2855 | 0.2379 | 0 |
| kernel_nway | 14574ns | +2079.8ns (+16.5%) | [+721, +2853]ns | [14122, 15336] | YES | 0.0002 | 0.0001 | 0 |
| kernel_seq | 10088ns | -1856.1ns (-14.7%) | [-4778, -1113]ns | [8638, 11122] | YES | 0.0002 | 0.0001 | 0 |
| nway | 42003ns | +28576.3ns (+226.1%) | [+25995, +31821]ns | [38658, 45577] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 12596ns | no significant difference | [-4155, +8299]ns | [6986, 21106] | no | 0.8145 | 0.8145 | 0 |
| std_parallel | 42317ns | +30833.5ns (+244.0%) | [+26793, +36195]ns | [38724, 49914] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 14439ns | -7.4% | +0.1% | -40.2% | +189.6% | -26.1% | +163.0% |
| 1 | 2 | 10966ns | +29.9% | +30.5% | -13.7% | +235.3% | +8.2% | +367.3% |
| 1 | 3 | 12638ns | -4.5% | +19.2% | -6.2% | +260.6% | +21.3% | +246.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.487 | moderate- |
| head_tail_dyn | -0.431 | moderate- |
| kernel_nway | -0.283 | moderate- |
| kernel_seq | -0.049 | ok |
| nway | -0.546 | HIGH- (thermal bounce) |
| sequential | -0.050 | ok |
| std_parallel | -0.540 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 13242.6ns | 12638.0ns | 104.8% | HIGH |
| head_tail_dyn | 13688.4ns | 13189.1ns | 103.8% | HIGH |
| kernel_nway | 15206.4ns | 14708.5ns | 103.4% | HIGH |
| kernel_seq | 10402.3ns | 10019.5ns | 103.8% | HIGH |
| nway | 44419.0ns | 43914.4ns | 101.1% | HIGH |
| sequential | 14763.5ns | 14313.9ns | 103.1% | HIGH |
| std_parallel | 47294.4ns | 47485.6ns | 99.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 10647.0-14207.3 ns)
  10647.0 |
  10825.0 |##########################
  11003.0 |#############
  11181.1 |#############
  11359.1 |
  11537.1 |
  11715.1 |
  11893.1 |
  12071.1 |#############
  12249.2 |
  12427.2 |########################################
  12605.2 |########################################
  12783.2 |
  12961.2 |##########################
  13139.2 |
  13317.3 |
  13495.3 |
  13673.3 |
  13851.3 |
  14029.3 |
  (1 below, 4 above range)

head_tail_dyn (n=18, range 11269.8-14578.1 ns)
  11269.8 |
  11435.2 |
  11600.7 |####################
  11766.1 |
  11931.5 |
  12096.9 |####################
  12262.3 |
  12427.7 |########################################
  12593.1 |####################
  12758.5 |####################
  12924.0 |
  13089.4 |####################
  13254.8 |########################################
  13420.2 |########################################
  13585.6 |
  13751.0 |
  13916.4 |
  14081.8 |####################
  14247.2 |####################
  14412.7 |
  (2 below, 3 above range)

kernel_nway (n=18, range 13358.3-15860.8 ns)
  13358.3 |
  13483.4 |
  13608.5 |##########################
  13733.6 |
  13858.8 |#############
  13983.9 |
  14109.0 |
  14234.2 |########################################
  14359.3 |
  14484.4 |#############
  14609.5 |#############
  14734.7 |#############
  14859.8 |#############
  14984.9 |
  15110.1 |
  15235.2 |#############
  15360.3 |#############
  15485.4 |#############
  15610.6 |#############
  15735.7 |
  (2 below, 2 above range)

kernel_seq (n=18, range 7605.1-12011.4 ns)
   7605.1 |####################
   7825.4 |########################################
   8045.7 |####################
   8266.0 |
   8486.4 |
   8706.7 |
   8927.0 |########################################
   9147.3 |
   9367.6 |
   9587.9 |
   9808.2 |########################################
  10028.6 |####################
  10248.9 |####################
  10469.2 |########################################
  10689.5 |
  10909.8 |
  11130.1 |
  11350.4 |####################
  11570.8 |
  11791.1 |########################################
  (1 below, 2 above range)

nway (n=18, range 35209.6-53243.5 ns)
  35209.6 |
  36111.3 |##########
  37013.0 |##########
  37914.7 |####################
  38816.4 |
  39718.1 |##########
  40619.8 |
  41521.5 |########################################
  42423.2 |
  43324.9 |##########
  44226.6 |
  45128.3 |##########
  46030.0 |
  46931.6 |####################
  47833.3 |
  48735.0 |
  49636.7 |
  50538.4 |
  51440.1 |##########
  52341.8 |
  (2 below, 2 above range)

sequential (n=18, range 6768.2-23428.4 ns)
   6768.2 |########################################
   7601.2 |
   8434.3 |
   9267.3 |
  10100.3 |######
  10933.3 |
  11766.3 |
  12599.3 |
  13432.3 |
  14265.3 |######
  15098.3 |
  15931.3 |######
  16764.4 |
  17597.4 |
  18430.4 |
  19263.4 |######
  20096.4 |
  20929.4 |######
  21762.4 |######
  22595.4 |######
  (2 below, 3 above range)

std_parallel (n=18, range 35960.7-63058.7 ns)
  35960.7 |##########################
  37315.6 |#############
  38670.5 |##########################
  40025.4 |
  41380.3 |########################################
  42735.2 |#############
  44090.1 |#############
  45445.0 |
  46799.9 |#############
  48154.8 |
  49509.7 |
  50864.6 |
  52219.5 |#############
  53574.4 |
  54929.3 |
  56284.2 |
  57639.1 |
  58994.0 |##########################
  60348.9 |
  61703.8 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=105.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=104.3% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=103.6% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=103.6% of algo (FFI overhead may distort results)
- **nway**: CV=20.2% (high variance, measurements may be unstable)
- **nway**: bridge=101.8% of algo (FFI overhead may distort results)
- **sequential**: CV=52.0% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **sequential**: bridge=106.1% of algo (FFI overhead may distort results)
- **std_parallel**: CV=28.2% (high variance, measurements may be unstable)
- **std_parallel**: bridge=96.9% of algo (FFI overhead may distort results)

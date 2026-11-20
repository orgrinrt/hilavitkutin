# fold_light, short fiber (1 to 4 morsels), W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail_dyn dominates: 14% faster than the next best (head_tail)

head_tail_dyn (10.38 us) leads head_tail (11.85 us) by 14%, a clear separation rather than a photo finish. CV 24.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 6.9x slower than the field

std_parallel (71.20 us) is 6.9x the fastest (10.38 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.61)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn, head_tail, kernel_seq, kernel_nway, sequential} vs {nway, std_parallel} (166% apart)

The field splits into a fast tier {head_tail_dyn, head_tail, kernel_seq, kernel_nway, sequential} and a slow tier {nway, std_parallel} with a 166% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.9x the fastest

Fastest head_tail_dyn (10.38 us) to slowest std_parallel (71.20 us): 6.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_seq is inconsistent: worst-20% is 6.0x its best-20%

kernel_seq's best 20% of batches run at 3.12 us but its worst 20% at 18.78 us (6.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 10383.1 ns median (-12.4% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 6.86x (fastest 10383.1 ns, slowest 71195.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 8 |
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
| head_tail | 12861ns | 12444ns | 11051ns | 12034ns | 15007ns | base |
| head_tail_dyn | 11966ns | 11005ns | 9058ns | 10587ns | 15489ns | -6.96% |
| kernel_nway | 19497ns | 18596ns | 15793ns | 18008ns | 23582ns | +51.59% |
| kernel_seq | 12870ns | 14588ns | 3295ns | 11179ns | 20193ns | +0.06% |
| nway | 51911ns | 49290ns | 43110ns | 48554ns | 61348ns | +303.62% |
| sequential | 15946ns | 18692ns | 6917ns | 14242ns | 23016ns | +23.98% |
| std_parallel | 101774ns | 71476ns | 66873ns | 70580ns | 166016ns | +691.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 12169ns | 10537ns | 14063ns | base | 0.021 |
| head_tail_dyn | 11252ns | 8616ns | 14561ns | -7.54% | 0.023 |
| kernel_nway | 18525ns | 15207ns | 22169ns | +52.23% | 0.014 |
| kernel_seq | 12105ns | 3125ns | 18785ns | -0.53% | 0.021 |
| nway | 51336ns | 42615ns | 60669ns | +321.86% | 0.005 |
| sequential | 15629ns | 6782ns | 22547ns | +28.43% | 0.016 |
| std_parallel | 101315ns | 66632ns | 165192ns | +732.57% | 0.003 |

## Performance model

- Peak throughput: **0.082 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.022 | 26.4% |
| head_tail_dyn | 0.025 | 30.1% |
| kernel_nway | 0.015 | 17.7% |
| kernel_seq | 0.018 | 22.4% |
| nway | 0.005 | 6.4% |
| sequential | 0.014 | 17.1% |
| std_parallel | 0.004 | 4.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 12861ns | 12861ns | base |
| head_tail_dyn | 11966ns | 11966ns | -6.96% |
| kernel_nway | 19497ns | 19497ns | +51.59% |
| kernel_seq | 12870ns | 12870ns | +0.06% |
| nway | 51911ns | 51911ns | +303.62% |
| sequential | 15946ns | 15946ns | +23.98% |
| std_parallel | 101774ns | 101774ns | +691.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 11851ns | base | --- | [10954, 12575] | --- | --- | --- | --- |
| head_tail_dyn | 10383ns | no significant difference | [-2216, +304]ns | [9402, 13376] | no | 0.1444 | 0.0963 | 0 |
| kernel_nway | 17611ns | +5403.1ns (+45.6%) | [+4603, +7014]ns | [16664, 19796] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 13940ns | no significant difference | [-7685, +4272]ns | [3939, 16697] | no | 0.2855 | 0.2379 | 0 |
| nway | 48763ns | +37147.8ns (+313.5%) | [+32555, +44156]ns | [43436, 57048] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 18328ns | no significant difference | [-3657, +7797]ns | [6894, 19931] | no | 0.4807 | 0.4807 | 0 |
| std_parallel | 71195ns | +59320.6ns (+500.5%) | [+56768, +63172]ns | [68602, 75778] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 11018ns | -20.0% | +41.5% | +26.5% | +294.2% | +77.9% | +518.1% |
| 1 | 2 | 12164ns | +17.2% | +76.3% | +58.9% | +358.6% | +25.2% | +532.9% |
| 1 | 3 | 12083ns | -14.1% | +47.6% | -71.8% | +338.9% | -42.9% | +496.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.205 | moderate- |
| head_tail_dyn | -0.615 | HIGH- (thermal bounce) |
| kernel_nway | -0.570 | HIGH- (thermal bounce) |
| kernel_seq | -0.385 | moderate- |
| nway | -0.302 | moderate- |
| sequential | -0.021 | ok |
| std_parallel | -0.533 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 12360.0ns | 12168.9ns | 101.6% | HIGH |
| head_tail_dyn | 12245.8ns | 11251.8ns | 108.8% | HIGH |
| kernel_nway | 19964.4ns | 18525.2ns | 107.8% | HIGH |
| kernel_seq | 12569.7ns | 12104.5ns | 103.8% | HIGH |
| nway | 51602.1ns | 51336.3ns | 100.5% | HIGH |
| sequential | 16076.9ns | 15629.0ns | 102.9% | HIGH |
| std_parallel | 100186.1ns | 101314.7ns | 98.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 10537.4-14062.8 ns)
  10537.4 |####################
  10713.7 |########################################
  10890.0 |
  11066.2 |########################################
  11242.5 |####################
  11418.8 |
  11595.1 |####################
  11771.3 |
  11947.6 |########################################
  12123.9 |####################
  12300.1 |
  12476.4 |####################
  12652.7 |####################
  12828.9 |####################
  13005.2 |####################
  13181.5 |
  13357.8 |
  13534.0 |
  13710.3 |
  13886.6 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 8616.4-14560.8 ns)
   8616.4 |########################################
   8913.6 |
   9210.8 |#############
   9508.1 |########################################
   9805.3 |
  10102.5 |#############
  10399.7 |##########################
  10696.9 |
  10994.2 |#############
  11291.4 |
  11588.6 |
  11885.8 |
  12183.0 |
  12480.3 |
  12777.5 |
  13074.7 |#############
  13371.9 |#############
  13669.1 |
  13966.4 |#############
  14263.6 |#############
  (1 below, 2 above range)

kernel_nway (n=18, range 15207.3-22169.0 ns)
  15207.3 |####################
  15555.4 |
  15903.5 |##########
  16251.5 |
  16599.6 |########################################
  16947.7 |
  17295.8 |##########
  17643.9 |##########
  17992.0 |##########
  18340.0 |##########
  18688.1 |
  19036.2 |
  19384.3 |
  19732.4 |##############################
  20080.5 |
  20428.5 |
  20776.6 |
  21124.7 |
  21472.8 |
  21820.9 |
  (1 below, 3 above range)

kernel_seq (n=18, range 3125.0-18785.0 ns)
   3125.0 |########################################
   3908.0 |#############
   4691.0 |
   5474.0 |
   6257.0 |
   7040.0 |
   7823.0 |
   8606.0 |
   9389.0 |
  10172.0 |
  10955.0 |
  11738.0 |#############
  12521.0 |#############
  13304.0 |#############
  14087.0 |##########################
  14870.0 |
  15653.0 |#############
  16436.0 |##########################
  17219.0 |
  18002.0 |
  (2 below, 4 above range)

nway (n=18, range 42615.2-60669.2 ns)
  42615.2 |########################################
  43517.9 |#############
  44420.6 |#############
  45323.3 |
  46226.0 |
  47128.7 |
  48031.4 |##########################
  48934.1 |#############
  49836.8 |
  50739.5 |
  51642.2 |
  52544.9 |
  53447.6 |
  54350.3 |#############
  55253.0 |
  56155.7 |##########################
  57058.4 |#############
  57961.1 |
  58863.8 |
  59766.5 |#############
  (2 below, 3 above range)

sequential (n=18, range 6782.4-22547.1 ns)
   6782.4 |########################################
   7570.6 |
   8358.9 |
   9147.1 |
   9935.3 |
  10723.6 |
  11511.8 |##########
  12300.0 |
  13088.3 |
  13876.5 |
  14664.7 |
  15453.0 |
  16241.2 |
  17029.4 |##########
  17817.7 |##########
  18605.9 |##########
  19394.1 |##############################
  20182.4 |####################
  20970.6 |
  21758.8 |####################
  (2 below, 1 above range)

std_parallel (n=18, range 66632.2-165192.1 ns)
  66632.2 |########################################
  71560.2 |####################
  76488.2 |###############
  81416.2 |
  86344.2 |
  91272.2 |
  96200.2 |
  101128.2 |
  106056.2 |
  110984.2 |
  115912.2 |
  120840.2 |
  125768.2 |
  130696.2 |
  135624.2 |
  140552.2 |
  145480.2 |
  150408.2 |
  155336.2 |
  160264.2 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=102.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=22.6% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=105.4% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=107.0% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=53.8% (high variance, measurements may be unstable)
- **kernel_seq**: worst_20/best_20 = 6.0x (possible bimodal distribution)
- **kernel_seq**: bridge=102.5% of algo (FFI overhead may distort results)
- **nway**: bridge=103.4% of algo (FFI overhead may distort results)
- **sequential**: CV=45.4% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **sequential**: bridge=104.0% of algo (FFI overhead may distort results)
- **std_parallel**: CV=83.8% (high variance, measurements may be unstable)
- **std_parallel**: bridge=98.4% of algo (FFI overhead may distort results)

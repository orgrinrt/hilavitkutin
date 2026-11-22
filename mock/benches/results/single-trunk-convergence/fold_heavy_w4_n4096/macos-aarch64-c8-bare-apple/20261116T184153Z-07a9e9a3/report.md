# fold_heavy, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 8.7x slower than the field

std_parallel (54.27 us) is 8.7x the fastest (6.23 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.61)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_seq, head_tail, head_tail_dyn, sequential, kernel_nway} vs {nway, std_parallel} (180% apart)

The field splits into a fast tier {kernel_seq, head_tail, head_tail_dyn, sequential, kernel_nway} and a slow tier {nway, std_parallel} with a 180% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 8.7x the fastest

Fastest kernel_seq (6.23 us) to slowest std_parallel (54.27 us): 8.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 1.6x its best-20%

kernel_nway's best 20% of batches run at 12.22 us but its worst 20% at 19.36 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 6233.3 ns median (-7.5% vs baseline)
- 5 variants significantly slower than baseline
- Spread: 8.71x (fastest 6233.3 ns, slowest 54275.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 4 |
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
| head_tail | 7366ns | 7335ns | 6742ns | 7247ns | 7857ns | base |
| head_tail_dyn | 7917ns | 7735ns | 7246ns | 7627ns | 8687ns | +7.48% |
| kernel_nway | 16157ns | 14790ns | 12860ns | 14687ns | 20012ns | +119.35% |
| kernel_seq | 6851ns | 6760ns | 6311ns | 6629ns | 7453ns | -7.00% |
| nway | 41090ns | 40042ns | 35943ns | 39667ns | 45797ns | +457.83% |
| sequential | 13618ns | 13531ns | 13239ns | 13473ns | 14026ns | +84.88% |
| std_parallel | 56399ns | 54715ns | 51265ns | 54257ns | 62178ns | +665.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 6769ns | 6271ns | 7205ns | base | 0.605 |
| head_tail_dyn | 7280ns | 6625ns | 8033ns | +7.54% | 0.563 |
| kernel_nway | 15487ns | 12218ns | 19362ns | +128.78% | 0.264 |
| kernel_seq | 6305ns | 5806ns | 6869ns | -6.87% | 0.650 |
| nway | 40547ns | 35351ns | 45285ns | +498.97% | 0.101 |
| sequential | 13462ns | 13100ns | 13858ns | +98.87% | 0.304 |
| std_parallel | 55987ns | 50836ns | 61758ns | +727.06% | 0.073 |

## Performance model

- Peak throughput: **0.706 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.608 | 86.1% |
| head_tail_dyn | 0.576 | 81.7% |
| kernel_nway | 0.290 | 41.1% |
| kernel_seq | 0.657 | 93.1% |
| nway | 0.104 | 14.7% |
| sequential | 0.306 | 43.4% |
| std_parallel | 0.075 | 10.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 7366ns | 7366ns | base |
| head_tail_dyn | 7917ns | 7917ns | +7.48% |
| kernel_nway | 16157ns | 16157ns | +119.35% |
| kernel_seq | 6851ns | 6851ns | -7.00% |
| nway | 41090ns | 41090ns | +457.83% |
| sequential | 13618ns | 13618ns | +84.88% |
| std_parallel | 56399ns | 56399ns | +665.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 6740ns | base | --- | [6504, 7081] | --- | --- | --- | --- |
| head_tail_dyn | 7106ns | +202.1ns (+3.0%) | [+21, +613]ns | [6788, 7344] | YES | 0.0371 | 0.0309 | 0 |
| kernel_nway | 14119ns | +7493.8ns (+111.2%) | [+6648, +9635]ns | [13412, 16492] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 6233ns | no significant difference | [-923, +96]ns | [5873, 6525] | no | 0.2379 | 0.2379 | 0 |
| nway | 39521ns | +32985.4ns (+489.4%) | [+30821, +37338]ns | [37429, 43971] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 13377ns | +6504.2ns (+96.5%) | [+6262, +7060]ns | [13165, 13662] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 54275ns | +47806.2ns (+709.3%) | [+45796, +53796]ns | [52273, 60508] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 6758ns | +8.9% | +107.8% | -5.1% | +541.0% | +96.1% | +795.9% |
| 1 | 2 | 6802ns | -1.6% | +121.7% | -10.5% | +450.3% | +96.8% | +688.9% |
| 1 | 3 | 6635ns | +7.9% | +103.8% | -8.8% | +486.5% | +103.9% | +696.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.329 | moderate- |
| head_tail_dyn | -0.612 | HIGH- (thermal bounce) |
| kernel_nway | -0.594 | HIGH- (thermal bounce) |
| kernel_seq | -0.119 | ok |
| nway | -0.322 | moderate- |
| sequential | -0.000 | ok |
| std_parallel | -0.114 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 87583.8ns | 6769.4ns | 1293.8% | HIGH |
| head_tail_dyn | 89899.8ns | 7280.1ns | 1234.9% | HIGH |
| kernel_nway | 110842.8ns | 15487.3ns | 715.7% | HIGH |
| kernel_seq | 92401.2ns | 6304.6ns | 1465.6% | HIGH |
| nway | 112439.3ns | 40547.2ns | 277.3% | HIGH |
| sequential | 91260.6ns | 13462.3ns | 677.9% | HIGH |
| std_parallel | 116720.4ns | 55987.5ns | 208.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 6270.8-7204.9 ns)
   6270.8 |
   6317.5 |####################
   6364.2 |########################################
   6410.9 |####################
   6457.6 |
   6504.3 |
   6551.0 |########################################
   6597.7 |
   6644.4 |####################
   6691.1 |####################
   6737.8 |####################
   6784.6 |####################
   6831.3 |####################
   6878.0 |
   6924.7 |
   6971.4 |
   7018.1 |####################
   7064.8 |
   7111.5 |########################################
   7158.2 |####################
  (1 below, 2 above range)

head_tail_dyn (n=18, range 6625.0-8032.6 ns)
   6625.0 |##########################
   6695.4 |#############
   6765.8 |#############
   6836.2 |#############
   6906.6 |
   6976.9 |#############
   7047.3 |#############
   7117.7 |#############
   7188.1 |
   7258.5 |##########################
   7328.8 |########################################
   7399.2 |
   7469.6 |
   7540.0 |
   7610.4 |
   7680.7 |#############
   7751.1 |
   7821.5 |
   7891.9 |
   7962.3 |
  (2 below, 2 above range)

kernel_nway (n=18, range 12218.1-19362.5 ns)
  12218.1 |
  12575.3 |##########################
  12932.5 |##########################
  13289.7 |#############
  13646.9 |########################################
  14004.2 |
  14361.4 |##########################
  14718.6 |
  15075.8 |
  15433.1 |#############
  15790.3 |
  16147.5 |#############
  16504.7 |#############
  16861.9 |
  17219.2 |#############
  17576.4 |
  17933.6 |
  18290.8 |
  18648.0 |
  19005.3 |
  (1 below, 3 above range)

kernel_seq (n=18, range 5805.6-6868.7 ns)
   5805.6 |########################################
   5858.7 |####################
   5911.9 |
   5965.0 |
   6018.2 |
   6071.4 |
   6124.5 |
   6177.7 |####################
   6230.8 |####################
   6284.0 |
   6337.1 |
   6390.3 |##########
   6443.5 |##########
   6496.6 |
   6549.8 |####################
   6602.9 |
   6656.1 |
   6709.3 |
   6762.4 |##########
   6815.6 |
  (1 below, 2 above range)

nway (n=18, range 35351.4-45284.7 ns)
  35351.4 |
  35848.0 |#############
  36344.7 |
  36841.4 |#############
  37338.0 |##########################
  37834.7 |##########################
  38331.4 |
  38828.0 |
  39324.7 |##########################
  39821.4 |
  40318.0 |
  40814.7 |
  41311.4 |
  41808.0 |#############
  42304.7 |#############
  42801.4 |
  43298.0 |
  43794.7 |########################################
  44291.4 |
  44788.0 |#############
  (2 below, 2 above range)

sequential (n=18, range 13100.0-13857.6 ns)
  13100.0 |########################################
  13137.9 |########################################
  13175.8 |########################################
  13213.6 |
  13251.5 |####################
  13289.4 |
  13327.3 |####################
  13365.2 |####################
  13403.1 |
  13440.9 |
  13478.8 |
  13516.7 |
  13554.6 |########################################
  13592.5 |
  13630.4 |####################
  13668.2 |####################
  13706.1 |####################
  13744.0 |####################
  13781.9 |
  13819.8 |
  (1 below, 2 above range)

std_parallel (n=18, range 50836.1-61758.3 ns)
  50836.1 |
  51382.2 |##########################
  51928.3 |##########################
  52474.4 |
  53020.5 |##########################
  53566.7 |#############
  54112.8 |
  54658.9 |#############
  55205.0 |
  55751.1 |#############
  56297.2 |#############
  56843.3 |
  57389.4 |
  57935.6 |
  58481.7 |
  59027.8 |
  59573.9 |
  60120.0 |########################################
  60666.1 |##########################
  61212.2 |
  (2 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=1288.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=1259.7% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=20.9% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=782.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=1487.7% of algo (FFI overhead may distort results)
- **nway**: bridge=283.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=686.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=213.2% of algo (FFI overhead may distort results)

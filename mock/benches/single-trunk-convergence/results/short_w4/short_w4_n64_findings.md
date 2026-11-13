# fold_light, short fiber (1 to 4 morsels), W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 234% faster than the next best (kernel_seq)

sequential (7.40 us) leads kernel_seq (24.73 us) by 234%, a clear separation rather than a photo finish. CV 16.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 79% (significant)

sequential is -26.67 us (79%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 13.5x slower than the field

std_parallel (100.23 us) is 13.5x the fastest (7.40 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_parallel shows alternating (throttle bounce) (autocorr -0.66)

std_parallel's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential} vs {kernel_seq, head_tail_dyn, head_tail, kernel_nway, nway, std_parallel} (234% apart)

The field splits into a fast tier {sequential} and a slow tier {kernel_seq, head_tail_dyn, head_tail, kernel_nway, nway, std_parallel} with a 234% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 13.5x the fastest

Fastest sequential (7.40 us) to slowest std_parallel (100.23 us): 13.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### head_tail is inconsistent: worst-20% is 1.8x its best-20%

head_tail's best 20% of batches run at 25.77 us but its worst 20% at 45.30 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 7399.8 ns median (-78.1% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 13.55x (fastest 7399.8 ns, slowest 100230.0 ns)

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
| Counter frequency | 2100312120 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 35886ns | 34497ns | 26130ns | 32533ns | 45795ns | base |
| head_tail_dyn | 30608ns | 29346ns | 24802ns | 28344ns | 36908ns | -14.71% |
| kernel_nway | 46234ns | 45870ns | 41460ns | 45303ns | 50019ns | +28.84% |
| kernel_seq | 25074ns | 25201ns | 19436ns | 23399ns | 30406ns | -30.13% |
| nway | 52695ns | 50366ns | 46830ns | 49680ns | 60148ns | +46.84% |
| sequential | 7903ns | 7629ns | 7225ns | 7499ns | 8848ns | -77.98% |
| std_parallel | 101006ns | 100691ns | 89721ns | 97456ns | 111974ns | +181.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 35377ns | 25772ns | 45301ns | base | 0.002 |
| head_tail_dyn | 30147ns | 24441ns | 36406ns | -14.78% | 0.002 |
| kernel_nway | 45669ns | 41081ns | 49388ns | +29.09% | 0.001 |
| kernel_seq | 24617ns | 19036ns | 29879ns | -30.42% | 0.003 |
| nway | 52329ns | 46488ns | 59742ns | +47.92% | 0.001 |
| sequential | 7668ns | 7002ns | 8598ns | -78.33% | 0.008 |
| std_parallel | 100478ns | 89279ns | 111419ns | +184.02% | 0.001 |

## Performance model

- Peak throughput: **0.009 Gops/s** (sequential; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.002 | 20.7% |
| head_tail_dyn | 0.002 | 24.2% |
| kernel_nway | 0.001 | 15.4% |
| kernel_seq | 0.003 | 28.3% |
| nway | 0.001 | 14.0% |
| sequential | 0.009 | 94.6% |
| std_parallel | 0.001 | 7.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 35886ns | 35886ns | base |
| head_tail_dyn | 30608ns | 30608ns | -14.71% |
| kernel_nway | 46234ns | 46234ns | +28.84% |
| kernel_seq | 25074ns | 25074ns | -30.13% |
| nway | 52695ns | 52695ns | +46.84% |
| sequential | 7903ns | 7903ns | -77.98% |
| std_parallel | 101006ns | 101006ns | +181.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 33835ns | base | --- | [29403, 35053] | --- | --- | --- | --- |
| head_tail_dyn | 28930ns | no significant difference | [-7424, +866]ns | [25403, 34065] | no | 0.2379 | 0.2379 | 0 |
| kernel_nway | 45399ns | +13219.5ns (+39.1%) | [+11381, +14755]ns | [43904, 47139] | YES | 0.0002 | 0.0001 | 0 |
| kernel_seq | 24728ns | -5654.4ns (-16.7%) | [-15491, -2228]ns | [19582, 28516] | YES | 0.0016 | 0.0013 | 0 |
| nway | 49977ns | +13569.7ns (+40.1%) | [+11490, +26002]ns | [47236, 55884] | YES | 0.0002 | 0.0001 | 0 |
| sequential | 7400ns | -26668.3ns (-78.8%) | [-27609, -22114]ns | [7070, 7671] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 100230ns | +66080.3ns (+195.3%) | [+58946, +73821]ns | [93527, 104724] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 35827ns | -3.6% | +32.0% | -46.5% | +31.1% | -78.7% | +174.4% |
| 1 | 2 | 26563ns | +8.6% | +57.2% | -6.9% | +111.4% | -72.1% | +299.1% |
| 1 | 3 | 33458ns | -25.1% | +37.0% | -11.2% | +54.6% | -78.7% | +191.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.626 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.009 | ok |
| kernel_nway | -0.625 | HIGH- (thermal bounce) |
| kernel_seq | -0.001 | ok |
| nway | -0.488 | moderate- |
| sequential | -0.002 | ok |
| std_parallel | -0.661 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 32296.9ns | 35376.9ns | 91.3% | HIGH |
| head_tail_dyn | 30120.0ns | 30147.1ns | 99.9% | HIGH |
| kernel_nway | 46455.9ns | 45669.4ns | 101.7% | HIGH |
| kernel_seq | 24786.4ns | 24616.5ns | 100.7% | HIGH |
| nway | 50969.6ns | 52329.1ns | 97.4% | HIGH |
| sequential | 7690.2ns | 7667.6ns | 100.3% | HIGH |
| std_parallel | 115920.8ns | 100477.9ns | 115.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 25771.8-45300.7 ns)
  25771.8 |##########################
  26748.3 |
  27724.7 |#############
  28701.2 |
  29677.6 |#############
  30654.0 |
  31630.5 |#############
  32606.9 |#############
  33583.4 |########################################
  34559.8 |########################################
  35536.2 |
  36512.7 |#############
  37489.1 |
  38465.6 |#############
  39442.0 |
  40418.4 |#############
  41394.9 |
  42371.3 |
  43347.8 |
  44324.2 |
  (2 below, 1 above range)

head_tail_dyn (n=18, range 24441.3-36405.8 ns)
  24441.3 |##########################
  25039.5 |##########################
  25637.8 |#############
  26236.0 |
  26834.2 |
  27432.4 |
  28030.7 |
  28628.9 |########################################
  29227.1 |#############
  29825.3 |
  30423.6 |
  31021.8 |
  31620.0 |
  32218.3 |
  32816.5 |
  33414.7 |##########################
  34012.9 |#############
  34611.2 |#############
  35209.4 |#############
  35807.6 |
  (2 below, 2 above range)

kernel_nway (n=18, range 41081.1-49387.7 ns)
  41081.1 |#############
  41496.4 |##########################
  41911.8 |
  42327.1 |
  42742.4 |#############
  43157.7 |
  43573.1 |
  43988.4 |
  44403.7 |##########################
  44819.0 |#############
  45234.4 |########################################
  45649.7 |
  46065.0 |
  46480.4 |##########################
  46895.7 |
  47311.0 |#############
  47726.3 |#############
  48141.7 |#############
  48557.0 |
  48972.3 |
  (1 below, 2 above range)

kernel_seq (n=18, range 19035.5-29878.7 ns)
  19035.5 |########################################
  19577.7 |##########
  20119.8 |
  20662.0 |
  21204.1 |
  21746.3 |####################
  22288.5 |
  22830.6 |
  23372.8 |
  23914.9 |##########
  24457.1 |
  24999.3 |##########
  25541.4 |
  26083.6 |
  26625.7 |##########
  27167.9 |
  27710.0 |####################
  28252.2 |
  28794.4 |####################
  29336.5 |
  (1 below, 3 above range)

nway (n=18, range 46487.8-59741.9 ns)
  46487.8 |########################################
  47150.5 |########################################
  47813.2 |########################################
  48475.9 |
  49138.6 |####################
  49801.3 |####################
  50464.0 |####################
  51126.7 |
  51789.4 |
  52452.1 |
  53114.9 |
  53777.6 |
  54440.3 |
  55103.0 |####################
  55765.7 |########################################
  56428.4 |####################
  57091.1 |
  57753.8 |
  58416.5 |
  59079.2 |
  (2 below, 3 above range)

sequential (n=18, range 7001.8-8598.4 ns)
   7001.8 |########################################
   7081.7 |####################
   7161.5 |
   7241.3 |##########
   7321.1 |##########
   7401.0 |##########
   7480.8 |####################
   7560.6 |
   7640.5 |####################
   7720.3 |##########
   7800.1 |##########
   7879.9 |
   7959.8 |
   8039.6 |
   8119.4 |##########
   8199.3 |
   8279.1 |
   8358.9 |
   8438.7 |
   8518.6 |
  (1 below, 1 above range)

std_parallel (n=18, range 89278.8-111419.3 ns)
  89278.8 |########################################
  90385.8 |
  91492.8 |
  92599.9 |##########################
  93706.9 |
  94813.9 |#############
  95920.9 |#############
  97028.0 |
  98135.0 |
  99242.0 |#############
  100349.0 |##########################
  101456.1 |
  102563.1 |##########################
  103670.1 |
  104777.2 |
  105884.2 |#############
  106991.2 |
  108098.2 |#############
  109205.3 |
  110312.3 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=36.8% (high variance, measurements may be unstable)
- **head_tail**: bridge=98.0% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=98.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=103.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.0% of algo (FFI overhead may distort results)
- **nway**: bridge=100.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.7% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=97.3% of algo (FFI overhead may distort results)

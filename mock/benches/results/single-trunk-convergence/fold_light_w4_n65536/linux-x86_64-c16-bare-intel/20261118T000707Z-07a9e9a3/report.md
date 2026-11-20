# fold_light, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 41% faster than the next best (kernel_nway)

sequential (17.16 us) leads kernel_nway (24.16 us) by 41%, a clear separation rather than a photo finish. CV 88.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 4.9x slower than the field

std_parallel (84.80 us) is 4.9x the fastest (17.16 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 88.6%)

sequential wins on median (17.16 us) yet has the highest variance (CV 88.6%), while nway is the steadiest (CV 13.8%, 58.73 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.67)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.9x the fastest

Fastest sequential (17.16 us) to slowest std_parallel (84.80 us): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.7x its best-20%

sequential's best 20% of batches run at 12.11 us but its worst 20% at 44.87 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 17157.7 ns median (-38.9% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 4.94x (fastest 17157.7 ns, slowest 84801.7 ns)

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
| head_tail | 32721ns | 29119ns | 24768ns | 28294ns | 43337ns | base |
| head_tail_dyn | 42985ns | 44388ns | 17384ns | 43035ns | 55711ns | +31.37% |
| kernel_nway | 26744ns | 24851ns | 20490ns | 24333ns | 33488ns | -18.27% |
| kernel_seq | 35961ns | 35217ns | 26422ns | 33238ns | 44816ns | +9.90% |
| nway | 60581ns | 59471ns | 48842ns | 58404ns | 69715ns | +85.14% |
| sequential | 26421ns | 17434ns | 12268ns | 18433ns | 45480ns | -19.25% |
| std_parallel | 85046ns | 85832ns | 42489ns | 81828ns | 111151ns | +159.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 31704ns | 24171ns | 41858ns | base | 2.067 |
| head_tail_dyn | 42255ns | 17181ns | 54423ns | +33.28% | 1.551 |
| kernel_nway | 25703ns | 19909ns | 31864ns | -18.93% | 2.550 |
| kernel_seq | 34909ns | 25847ns | 43331ns | +10.11% | 1.877 |
| nway | 59883ns | 48301ns | 68916ns | +88.88% | 1.094 |
| sequential | 26069ns | 12113ns | 44872ns | -17.77% | 2.514 |
| std_parallel | 84343ns | 42243ns | 110081ns | +166.04% | 0.777 |

## Performance model

- Peak throughput: **5.410 Gops/s** (sequential; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 2.332 | 43.1% |
| head_tail_dyn | 1.495 | 27.6% |
| kernel_nway | 2.712 | 50.1% |
| kernel_seq | 1.924 | 35.6% |
| nway | 1.116 | 20.6% |
| sequential | 3.820 | 70.6% |
| std_parallel | 0.773 | 14.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 32721ns | 32721ns | base |
| head_tail_dyn | 42985ns | 42985ns | +31.37% |
| kernel_nway | 26744ns | 26744ns | -18.27% |
| kernel_seq | 35961ns | 35961ns | +9.90% |
| nway | 60581ns | 60581ns | +85.14% |
| sequential | 26421ns | 26421ns | -19.25% |
| std_parallel | 85046ns | 85046ns | +159.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 28098ns | base | --- | [26560, 30208] | --- | --- | --- | --- |
| head_tail_dyn | 43826ns | +13006.2ns (+46.3%) | [+4770, +16851]ns | [41850, 49791] | YES | 0.0113 | 0.0075 | 0 |
| kernel_nway | 24164ns | -4381.2ns (-15.6%) | [-6958, -2384]ns | [20640, 28799] | YES | 0.0113 | 0.0075 | 0 |
| kernel_seq | 34058ns | no significant difference | [-1087, +7467]ns | [30385, 37781] | no | 0.4807 | 0.4807 | 0 |
| nway | 58733ns | +28209.3ns (+100.4%) | [+22241, +38406]ns | [55446, 64632] | YES | 0.0004 | 0.0001 | 0 |
| sequential | 17158ns | no significant difference | [-14414, +5068]ns | [12384, 38652] | no | 0.2855 | 0.2379 | 0 |
| std_parallel | 84802ns | +56291.5ns (+200.3%) | [+38326, +64547]ns | [75020, 97931] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 31438ns | +36.5% | +0.4% | +27.2% | +79.1% | -59.7% | +66.9% |
| 1 | 2 | 25470ns | +108.1% | -20.8% | +34.8% | +158.9% | -15.5% | +321.8% |
| 1 | 3 | 28757ns | +49.5% | -16.0% | -5.3% | +89.5% | +0.7% | +194.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.533 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.667 | HIGH- (thermal bounce) |
| kernel_nway | -0.394 | moderate- |
| kernel_seq | -0.003 | ok |
| nway | -0.652 | HIGH- (thermal bounce) |
| sequential | -0.002 | ok |
| std_parallel | -0.438 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 31544.9ns | 31703.7ns | 99.5% | HIGH |
| head_tail_dyn | 42928.5ns | 42254.7ns | 101.6% | HIGH |
| kernel_nway | 26208.2ns | 25703.1ns | 102.0% | HIGH |
| kernel_seq | 37499.8ns | 34909.4ns | 107.4% | HIGH |
| nway | 62331.3ns | 59883.4ns | 104.1% | HIGH |
| sequential | 27357.7ns | 26069.1ns | 104.9% | HIGH |
| std_parallel | 84878.9ns | 84343.5ns | 100.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 24171.3-41858.3 ns)
  24171.3 |
  25055.6 |##########################
  25940.0 |#############
  26824.3 |########################################
  27708.7 |##########################
  28593.0 |########################################
  29477.4 |
  30361.7 |
  31246.1 |#############
  32130.4 |
  33014.8 |#############
  33899.1 |
  34783.5 |
  35667.8 |
  36552.2 |
  37436.5 |
  38320.9 |#############
  39205.2 |
  40089.6 |
  40973.9 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 17181.4-54422.8 ns)
  17181.4 |
  19043.5 |
  20905.5 |
  22767.6 |##########
  24629.7 |
  26491.8 |
  28353.8 |
  30215.9 |##########
  32078.0 |
  33940.0 |
  35802.1 |
  37664.2 |
  39526.3 |##########
  41388.3 |########################################
  43250.4 |##########
  45112.5 |##########
  46974.5 |##########
  48836.6 |####################
  50698.7 |
  52560.8 |
  (2 below, 4 above range)

kernel_nway (n=18, range 19909.0-31863.9 ns)
  19909.0 |########################################
  20506.8 |#############
  21104.5 |
  21702.3 |
  22300.0 |
  22897.8 |
  23495.5 |##########################
  24093.2 |##########################
  24691.0 |
  25288.7 |
  25886.5 |
  26484.2 |#############
  27082.0 |#############
  27679.7 |#############
  28277.4 |
  28875.2 |
  29472.9 |#############
  30070.7 |
  30668.4 |
  31266.2 |##########################
  (2 below, 2 above range)

kernel_seq (n=18, range 25847.4-43331.1 ns)
  25847.4 |
  26721.6 |##########################
  27595.8 |
  28470.0 |
  29344.2 |
  30218.4 |##########################
  31092.5 |#############
  31966.7 |
  32840.9 |#############
  33715.1 |########################################
  34589.3 |#############
  35463.5 |
  36337.7 |#############
  37211.8 |
  38086.0 |#############
  38960.2 |
  39834.4 |
  40708.6 |#############
  41582.8 |
  42456.9 |
  (2 below, 3 above range)

nway (n=18, range 48301.1-68915.9 ns)
  48301.1 |
  49331.8 |
  50362.6 |
  51393.3 |#############
  52424.1 |
  53454.8 |
  54485.6 |##########################
  55516.3 |#############
  56547.0 |########################################
  57577.8 |
  58608.5 |
  59639.3 |##########################
  60670.0 |#############
  61700.8 |
  62731.5 |
  63762.2 |#############
  64793.0 |#############
  65823.7 |##########################
  66854.5 |
  67885.2 |
  (2 below, 2 above range)

sequential (n=18, range 12113.2-44872.2 ns)
  12113.2 |########################################
  13751.1 |
  15389.1 |
  17027.0 |
  18665.0 |
  20302.9 |#####
  21940.9 |
  23578.8 |
  25216.8 |
  26854.7 |
  28492.7 |
  30130.6 |#####
  31768.6 |
  33406.5 |
  35044.5 |##########
  36682.4 |
  38320.4 |
  39958.3 |#####
  41596.3 |#####
  43234.2 |
  (1 below, 3 above range)

std_parallel (n=18, range 42242.6-110081.1 ns)
  42242.6 |########
  45634.5 |
  49026.4 |
  52418.4 |
  55810.3 |
  59202.2 |########
  62594.1 |
  65986.1 |########
  69378.0 |
  72769.9 |
  76161.9 |
  79553.8 |########
  82945.7 |########################################
  86337.6 |
  89729.6 |
  93121.5 |
  96513.4 |################################
  99905.3 |
  103297.3 |
  106689.2 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=35.1% (high variance, measurements may be unstable)
- **head_tail**: bridge=101.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=31.2% (high variance, measurements may be unstable)
- **head_tail_dyn**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **head_tail_dyn**: bridge=100.5% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=20.2% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=102.9% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=21.2% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=104.5% of algo (FFI overhead may distort results)
- **nway**: bridge=101.0% of algo (FFI overhead may distort results)
- **sequential**: CV=58.3% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **sequential**: bridge=108.3% of algo (FFI overhead may distort results)
- **std_parallel**: CV=29.5% (high variance, measurements may be unstable)
- **std_parallel**: bridge=100.7% of algo (FFI overhead may distort results)

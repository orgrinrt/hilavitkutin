# fold_light, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 52% faster than the next best (head_tail_dyn)

kernel_seq (2.98 us) leads head_tail_dyn (4.51 us) by 52%, a clear separation rather than a photo finish. CV 5.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 37% (significant)

kernel_seq is -1.71 us (37%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 18.4x slower than the field

std_parallel (54.74 us) is 18.4x the fastest (2.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.64)

head_tail's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 18.4x the fastest

Fastest kernel_seq (2.98 us) to slowest std_parallel (54.74 us): 18.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 1.7x its best-20%

kernel_nway's best 20% of batches run at 8.55 us but its worst 20% at 14.15 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 2977.1 ns median (-36.1% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 18.39x (fastest 2977.1 ns, slowest 54735.4 ns)

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
| head_tail | 5456ns | 5294ns | 4821ns | 5177ns | 6190ns | base |
| head_tail_dyn | 5206ns | 5092ns | 4960ns | 5062ns | 5547ns | -4.57% |
| kernel_nway | 12009ns | 11371ns | 9204ns | 11072ns | 14816ns | +120.12% |
| kernel_seq | 3577ns | 3550ns | 3389ns | 3523ns | 3753ns | -34.43% |
| nway | 36136ns | 35046ns | 31085ns | 34731ns | 40769ns | +562.37% |
| sequential | 10974ns | 10848ns | 10531ns | 10816ns | 11431ns | +101.15% |
| std_parallel | 55007ns | 55119ns | 49175ns | 53992ns | 59447ns | +908.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4825ns | 4204ns | 5551ns | base | 0.849 |
| head_tail_dyn | 4596ns | 4326ns | 4931ns | -4.74% | 0.891 |
| kernel_nway | 11367ns | 8546ns | 14152ns | +135.58% | 0.360 |
| kernel_seq | 3000ns | 2839ns | 3172ns | -37.81% | 1.365 |
| nway | 35579ns | 30528ns | 40207ns | +637.40% | 0.115 |
| sequential | 10808ns | 10367ns | 11251ns | +124.00% | 0.379 |
| std_parallel | 54598ns | 48765ns | 59026ns | +1031.57% | 0.075 |

## Performance model

- Peak throughput: **1.443 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.879 | 60.9% |
| head_tail_dyn | 0.907 | 62.9% |
| kernel_nway | 0.382 | 26.4% |
| kernel_seq | 1.376 | 95.4% |
| nway | 0.119 | 8.2% |
| sequential | 0.383 | 26.5% |
| std_parallel | 0.075 | 5.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 5456ns | 5456ns | base |
| head_tail_dyn | 5206ns | 5206ns | -4.57% |
| kernel_nway | 12009ns | 12009ns | +120.12% |
| kernel_seq | 3577ns | 3577ns | -34.43% |
| nway | 36136ns | 36136ns | +562.37% |
| sequential | 10974ns | 10974ns | +101.15% |
| std_parallel | 55007ns | 55007ns | +908.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4658ns | base | --- | [4310, 5129] | --- | --- | --- | --- |
| head_tail_dyn | 4515ns | no significant difference | [-688, +192]ns | [4396, 4581] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 10733ns | +6202.0ns (+133.1%) | [+5010, +7427]ns | [9873, 12142] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2977ns | -1708.3ns (-36.7%) | [-2181, -1342]ns | [2894, 3035] | YES | 0.0000 | 0.0000 | 0 |
| nway | 34521ns | +29829.2ns (+640.3%) | [+28646, +31465]ns | [33796, 36831] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10696ns | +6018.8ns (+129.2%) | [+5421, +6398]ns | [10531, 11000] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 54735ns | +49625.0ns (+1065.3%) | [+47058, +52238]ns | [51921, 56952] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4496ns | -0.6% | +177.6% | -32.8% | +748.0% | +145.0% | +1054.4% |
| 1 | 2 | 5306ns | -16.2% | +96.3% | -44.2% | +541.3% | +101.5% | +911.6% |
| 1 | 3 | 4292ns | +10.1% | +140.8% | -31.2% | +694.0% | +148.2% | +1254.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.643 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.203 | moderate- |
| kernel_nway | -0.148 | ok |
| kernel_seq | -0.095 | ok |
| nway | -0.172 | ok |
| sequential | -0.111 | ok |
| std_parallel | -0.039 | ok |

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
| head_tail | 94327.8ns | 4825.0ns | 1955.0% | HIGH |
| head_tail_dyn | 95695.1ns | 4596.3ns | 2082.0% | HIGH |
| kernel_nway | 117774.8ns | 11366.7ns | 1036.1% | HIGH |
| kernel_seq | 90327.3ns | 3000.5ns | 3010.4% | HIGH |
| nway | 113694.2ns | 35579.4ns | 319.6% | HIGH |
| sequential | 92307.4ns | 10808.1ns | 854.1% | HIGH |
| std_parallel | 113929.9ns | 54598.4ns | 208.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 4204.2-5551.4 ns)
   4204.2 |########################################
   4271.5 |########################################
   4338.9 |####################
   4406.2 |####################
   4473.6 |
   4541.0 |
   4608.3 |########################################
   4675.7 |
   4743.1 |
   4810.4 |####################
   4877.8 |
   4945.1 |
   5012.5 |####################
   5079.9 |########################################
   5147.2 |
   5214.6 |
   5281.9 |
   5349.3 |
   5416.7 |
   5484.0 |####################
  (2 below, 3 above range)

head_tail_dyn (n=18, range 4326.4-4930.6 ns)
   4326.4 |####################
   4356.6 |########################################
   4386.8 |########################################
   4417.0 |
   4447.2 |
   4477.4 |########################################
   4507.6 |########################################
   4537.8 |####################
   4568.0 |########################################
   4598.2 |
   4628.5 |
   4658.7 |
   4688.9 |####################
   4719.1 |
   4749.3 |
   4779.5 |
   4809.7 |
   4839.9 |
   4870.1 |####################
   4900.3 |
  (2 below, 2 above range)

kernel_nway (n=18, range 8545.8-14152.1 ns)
   8545.8 |
   8826.1 |####################
   9106.5 |
   9386.8 |
   9667.1 |##########
   9947.4 |##########
  10227.7 |
  10508.0 |########################################
  10788.3 |##########
  11068.7 |
  11349.0 |
  11629.3 |####################
  11909.6 |
  12189.9 |##########
  12470.2 |
  12750.5 |
  13030.8 |##########
  13311.2 |
  13591.5 |
  13871.8 |
  (2 below, 3 above range)

kernel_seq (n=18, range 2838.9-3172.2 ns)
   2838.9 |
   2855.6 |#############
   2872.2 |#############
   2888.9 |##########################
   2905.6 |
   2922.2 |##########################
   2938.9 |
   2955.6 |
   2972.2 |########################################
   2988.9 |##########################
   3005.6 |
   3022.2 |
   3038.9 |
   3055.5 |#############
   3072.2 |
   3088.9 |#############
   3105.5 |
   3122.2 |
   3138.9 |
   3155.5 |
  (2 below, 3 above range)

nway (n=18, range 30527.8-40206.9 ns)
  30527.8 |
  31011.7 |
  31495.7 |#############
  31979.6 |#############
  32463.6 |#############
  32947.6 |
  33431.5 |#############
  33915.5 |########################################
  34399.4 |########################################
  34883.4 |#############
  35367.3 |
  35851.3 |
  36335.3 |#############
  36819.2 |#############
  37303.2 |
  37787.1 |
  38271.1 |
  38755.1 |#############
  39239.0 |#############
  39723.0 |
  (1 below, 2 above range)

sequential (n=18, range 10366.7-11250.7 ns)
  10366.7 |
  10410.9 |
  10455.1 |########################################
  10499.3 |########################################
  10543.5 |####################
  10587.7 |####################
  10631.9 |####################
  10676.1 |########################################
  10720.3 |
  10764.5 |
  10808.7 |####################
  10852.9 |####################
  10897.1 |
  10941.3 |
  10985.5 |########################################
  11029.7 |####################
  11073.9 |
  11118.1 |
  11162.3 |
  11206.5 |
  (1 below, 3 above range)

std_parallel (n=18, range 48765.3-59026.4 ns)
  48765.3 |##########################
  49278.4 |
  49791.4 |
  50304.5 |
  50817.5 |
  51330.6 |
  51843.6 |########################################
  52356.7 |#############
  52869.7 |#############
  53382.8 |
  53895.8 |#############
  54408.9 |
  54922.0 |#############
  55435.0 |##########################
  55948.1 |
  56461.1 |#############
  56974.2 |#############
  57487.2 |#############
  58000.3 |#############
  58513.3 |#############
  (1 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=2039.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2113.5% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=21.1% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=1087.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3043.4% of algo (FFI overhead may distort results)
- **nway**: bridge=321.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=870.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=209.5% of algo (FFI overhead may distort results)

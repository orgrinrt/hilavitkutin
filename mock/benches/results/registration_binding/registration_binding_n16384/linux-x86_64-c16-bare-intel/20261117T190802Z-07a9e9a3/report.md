# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 8.7x slower than the field

reg_bound_record (22.14 us) is 8.7x the fastest (2.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {reg_typed_twin, reg_typed, reg_bound_units, reg_bound_columns, reg_bound_strided} vs {reg_bound_record} (328% apart)

The field splits into a fast tier {reg_typed_twin, reg_typed, reg_bound_units, reg_bound_columns, reg_bound_strided} and a slow tier {reg_bound_record} with a 328% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 8.7x the fastest

Fastest reg_typed_twin (2.54 us) to slowest reg_bound_record (22.14 us): 8.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### reg_bound_units's edge over baseline is significant but tiny (39 ns, 1.53%)

reg_bound_units differs from baseline reg_typed by 39 ns (1.53%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: reg_typed_twin** at 2538.3 ns median (-1.1% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 8.72x (fastest 2538.3 ns, slowest 22141.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Registration: typed binding vs a run-time binding table over a column workload |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x71c40e2b95a3d86f |
| Counter frequency | 2112819991 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| reg_bound_columns | 4992ns | 5007ns | 4440ns | 5002ns | 5516ns | +12.00% |
| reg_bound_record | 23497ns | 24076ns | 19757ns | 22902ns | 29023ns | +427.11% |
| reg_bound_strided | 6961ns | 6924ns | 6081ns | 6958ns | 7852ns | +56.17% |
| reg_bound_units | 4466ns | 4148ns | 4069ns | 4403ns | 5052ns | +0.18% |
| reg_typed | 4458ns | 4138ns | 4089ns | 4385ns | 5044ns | base |
| reg_typed_twin | 4211ns | 4107ns | 4029ns | 4110ns | 4694ns | -5.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 3088ns | 2747ns | 3424ns | +12.05% | 5.305 |
| reg_bound_record | 21605ns | 18172ns | 26667ns | +683.89% | 0.758 |
| reg_bound_strided | 5213ns | 4510ns | 5927ns | +89.13% | 3.143 |
| reg_bound_units | 2767ns | 2504ns | 3134ns | +0.41% | 5.920 |
| reg_typed | 2756ns | 2521ns | 3121ns | base | 5.945 |
| reg_typed_twin | 2600ns | 2466ns | 2907ns | -5.66% | 6.302 |

## Performance model

- Peak throughput: **6.644 Gops/s** (reg_typed_twin; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 5.293 | 79.7% |
| reg_bound_record | 0.740 | 11.1% |
| reg_bound_strided | 3.170 | 47.7% |
| reg_bound_units | 6.345 | 95.5% |
| reg_typed | 6.383 | 96.1% |
| reg_typed_twin | 6.455 | 97.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 4992ns | 4992ns | +12.00% |
| reg_bound_record | 23497ns | 23497ns | +427.11% |
| reg_bound_strided | 6961ns | 6961ns | +56.17% |
| reg_bound_units | 4466ns | 4466ns | +0.18% |
| reg_typed | 4458ns | 4458ns | base |
| reg_typed_twin | 4211ns | 4211ns | -5.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 2567ns | base | --- | [2562, 2576] | --- | --- | --- | --- |
| reg_bound_columns | 3095ns | +470.8ns (+18.3%) | [+28, +485]ns | [3092, 3101] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_record | 22141ns | +19567.3ns (+762.3%) | [+19005, +19588]ns | [22083, 22151] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 5169ns | +2512.4ns (+97.9%) | [+2271, +2790]ns | [4836, 5508] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 2582ns | +39.2ns (+1.5%) | [+29, +47]ns | [2580, 2585] | YES | 0.0001 | 0.0000 | 0 |
| reg_typed_twin | 2538ns | no significant difference | [-53, +18]ns | [2530, 2574] | no | 0.1788 | 0.1788 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2521ns | +22.7% | +624.8% | +80.0% | +2.4% | +2.2% |
| 1 | 2 | 2560ns | +21.3% | +766.3% | +76.1% | +22.0% | +1.0% |
| 1 | 3 | 2568ns | +0.9% | +769.3% | +114.9% | +21.7% | +20.5% |
| 1 | 4 | 2561ns | +22.8% | +764.8% | +88.8% | -0.1% | +0.6% |
| 1 | 5 | 3082ns | +0.5% | +487.2% | +56.4% | +1.5% | -19.9% |
| 1 | 6 | 2524ns | +19.0% | +778.0% | +119.4% | +1.3% | +0.3% |
| 1 | 7 | 3135ns | -1.3% | +483.4% | +88.9% | -17.6% | -21.3% |
| 1 | 8 | 3079ns | +0.8% | +617.3% | +92.4% | -19.6% | -17.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | -0.304 | moderate- |
| reg_bound_record | -0.450 | moderate- |
| reg_bound_strided | 0.365 | moderate+ |
| reg_bound_units | -0.116 | ok |
| reg_typed | 0.026 | ok |
| reg_typed_twin | 0.039 | ok |

**Consistency summary:**

- **reg_bound_columns**: won 1/8, lost 7/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 3/8, lost 5/8
- **reg_typed_twin**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 38939.5ns | 3088.2ns | 1260.9% | HIGH |
| reg_bound_record | 38091.6ns | 21604.7ns | 176.3% | HIGH |
| reg_bound_strided | 35556.0ns | 5212.6ns | 682.1% | HIGH |
| reg_bound_units | 34650.4ns | 2767.4ns | 1252.1% | HIGH |
| reg_typed | 34633.6ns | 2756.1ns | 1256.6% | HIGH |
| reg_typed_twin | 32849.2ns | 2600.0ns | 1263.4% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 2746.8-3424.3 ns)
   2746.8 |
   2780.7 |
   2814.5 |
   2848.4 |
   2882.3 |
   2916.2 |
   2950.1 |
   2983.9 |########
   3017.8 |#
   3051.7 |###
   3085.6 |########################################
   3119.4 |#########
   3153.3 |#
   3187.2 |
   3221.1 |
   3254.9 |
   3288.8 |
   3322.7 |
   3356.6 |
   3390.4 |
  (20 below, 1 above range)

reg_bound_record (n=160, range 18172.0-26666.7 ns)
  18172.0 |#################
  18596.7 |
  19021.5 |
  19446.2 |
  19870.9 |
  20295.7 |
  20720.4 |
  21145.1 |
  21569.9 |
  21994.6 |########################################
  22419.3 |
  22844.1 |
  23268.8 |
  23693.5 |
  24118.3 |
  24543.0 |
  24967.7 |
  25392.5 |
  25817.2 |
  26241.9 |
  (16 below, 3 above range)

reg_bound_strided (n=160, range 4510.1-5926.9 ns)
   4510.1 |##########################
   4580.9 |#
   4651.7 |
   4722.6 |
   4793.4 |########################################
   4864.3 |#
   4935.1 |
   5006.0 |
   5076.8 |
   5147.6 |
   5218.5 |
   5289.3 |
   5360.2 |
   5431.0 |#######
   5501.8 |########################
   5572.7 |###
   5643.5 |
   5714.4 |#
   5785.2 |
   5856.0 |##############################
  (13 below, 15 above range)

reg_bound_units (n=160, range 2504.0-3133.8 ns)
   2504.0 |
   2535.5 |################################
   2566.9 |########################################
   2598.4 |
   2629.9 |
   2661.4 |
   2692.9 |
   2724.4 |
   2755.9 |
   2787.4 |
   2818.9 |
   2850.4 |
   2881.9 |
   2913.4 |
   2944.8 |
   2976.3 |
   3007.8 |
   3039.3 |
   3070.8 |
   3102.3 |#######################################
  (20 below, 17 above range)

reg_typed (n=160, range 2521.1-3121.4 ns)
   2521.1 |###############
   2551.1 |########################################
   2581.1 |#
   2611.1 |
   2641.2 |
   2671.2 |
   2701.2 |
   2731.2 |
   2761.2 |
   2791.2 |
   2821.2 |
   2851.3 |
   2881.3 |
   2911.3 |
   2941.3 |
   2971.3 |
   3001.3 |
   3031.3 |
   3061.4 |#######################
   3091.4 |####
  (18 below, 20 above range)

reg_typed_twin (n=160, range 2466.1-2907.0 ns)
   2466.1 |#####################################
   2488.1 |
   2510.2 |###########################
   2532.2 |##############
   2554.3 |##############
   2576.3 |########################################
   2598.3 |
   2620.4 |
   2642.4 |
   2664.5 |
   2686.5 |
   2708.6 |
   2730.6 |
   2752.6 |
   2774.7 |
   2796.7 |
   2818.8 |
   2840.8 |
   2862.9 |
   2884.9 |
  (12 below, 20 above range)

```

## Diagnostics

- **reg_bound_columns**: CV=24.0% (high variance, measurements may be unstable)
- **reg_bound_columns**: bridge=1262.6% of algo (FFI overhead may distort results)
- **reg_bound_record**: CV=31.5% (high variance, measurements may be unstable)
- **reg_bound_record**: bridge=176.5% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=698.2% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=1238.5% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=1245.1% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=1259.0% of algo (FFI overhead may distort results)

# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 7.2x slower than the field

reg_bound_record (18.11 us) is 7.2x the fastest (2.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### reg_bound_units shows warm-up / thermal drift (autocorr +0.66)

reg_bound_units's per-pass series has lag-1 autocorrelation +0.66, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {reg_typed_twin, reg_bound_units, reg_bound_columns, reg_typed, reg_bound_strided} vs {reg_bound_record} (222% apart)

The field splits into a fast tier {reg_typed_twin, reg_bound_units, reg_bound_columns, reg_typed, reg_bound_strided} and a slow tier {reg_bound_record} with a 222% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.2x the fastest

Fastest reg_typed_twin (2.53 us) to slowest reg_bound_record (18.11 us): 7.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### reg_bound_strided is inconsistent: worst-20% is 1.6x its best-20%

reg_bound_strided's best 20% of batches run at 4.84 us but its worst 20% at 7.59 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: reg_typed_twin** at 2528.5 ns median (-16.3% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 7.16x (fastest 2528.5 ns, slowest 18112.9 ns)

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
| Counter frequency | 2112866659 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| reg_bound_columns | 4542ns | 4541ns | 4014ns | 4542ns | 5070ns | -2.20% |
| reg_bound_record | 21140ns | 19693ns | 19654ns | 20156ns | 25579ns | +355.21% |
| reg_bound_strided | 7883ns | 7536ns | 6404ns | 7543ns | 10382ns | +69.75% |
| reg_bound_units | 4521ns | 4559ns | 4036ns | 4517ns | 5017ns | -2.66% |
| reg_typed | 4644ns | 4931ns | 4053ns | 4715ns | 5022ns | base |
| reg_typed_twin | 4391ns | 4091ns | 4020ns | 4318ns | 4982ns | -5.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 2806ns | 2457ns | 3154ns | -1.97% | 5.839 |
| reg_bound_record | 19444ns | 18073ns | 23531ns | +579.29% | 0.843 |
| reg_bound_strided | 5880ns | 4837ns | 7590ns | +105.41% | 2.787 |
| reg_bound_units | 2783ns | 2475ns | 3102ns | -2.79% | 5.888 |
| reg_typed | 2862ns | 2490ns | 3106ns | base | 5.724 |
| reg_typed_twin | 2696ns | 2459ns | 3066ns | -5.80% | 6.076 |

## Performance model

- Peak throughput: **6.668 Gops/s** (reg_bound_columns; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 5.865 | 88.0% |
| reg_bound_record | 0.905 | 13.6% |
| reg_bound_strided | 2.916 | 43.7% |
| reg_bound_units | 5.907 | 88.6% |
| reg_typed | 5.425 | 81.4% |
| reg_typed_twin | 6.480 | 97.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 4542ns | 4542ns | -2.20% |
| reg_bound_record | 21140ns | 21140ns | +355.21% |
| reg_bound_strided | 7883ns | 7883ns | +69.75% |
| reg_bound_units | 4521ns | 4521ns | -2.66% |
| reg_typed | 4644ns | 4644ns | base |
| reg_typed_twin | 4391ns | 4391ns | -5.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 3020ns | base | --- | [3010, 3087] | --- | --- | --- | --- |
| reg_bound_columns | 2794ns | no significant difference | [-6, +42]ns | [2571, 3014] | no | 0.4769 | 0.4769 | 0 |
| reg_bound_record | 18113ns | +15545.4ns (+514.7%) | [+15088, +15563]ns | [18109, 18118] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 5619ns | +2811.4ns (+93.1%) | [+2506, +2823]ns | [5516, 5907] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 2774ns | no significant difference | [-10, +3]ns | [2536, 3004] | no | 0.4769 | 0.4769 | 0 |
| reg_typed_twin | 2528ns | -284.3ns (-9.4%) | [-548, -74]ns | [2527, 2534] | YES | 0.0002 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3093ns | +1.8% | +485.5% | +78.8% | +0.1% | -18.2% |
| 1 | 2 | 3091ns | +1.8% | +486.1% | +92.3% | +0.2% | -2.5% |
| 1 | 3 | 2529ns | +19.1% | +615.1% | +131.6% | +18.8% | +22.3% |
| 1 | 4 | 3094ns | -16.9% | +616.0% | +91.2% | -2.9% | -20.5% |
| 1 | 5 | 2530ns | -2.6% | +773.3% | +91.3% | +0.1% | +18.7% |
| 1 | 6 | 3010ns | -18.4% | +500.3% | +82.8% | -15.8% | -18.3% |
| 1 | 7 | 2467ns | -0.1% | +633.2% | +211.7% | -0.1% | +0.1% |
| 1 | 8 | 3088ns | +2.0% | +486.7% | +56.7% | -18.1% | -18.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | 0.419 | moderate+ |
| reg_bound_record | 0.286 | moderate+ |
| reg_bound_strided | -0.392 | moderate- |
| reg_bound_units | 0.656 | HIGH+ (drift/warm-up) |
| reg_typed | -0.614 | HIGH- (thermal bounce) |
| reg_typed_twin | -0.130 | ok |

**Consistency summary:**

- **reg_bound_columns**: won 4/8, lost 4/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 3/8, lost 4/8
- **reg_typed_twin**: won 5/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 35602.6ns | 2806.1ns | 1268.7% | HIGH |
| reg_bound_record | 34319.8ns | 19444.3ns | 176.5% | HIGH |
| reg_bound_strided | 40621.8ns | 5879.7ns | 690.9% | HIGH |
| reg_bound_units | 35532.6ns | 2782.6ns | 1277.0% | HIGH |
| reg_typed | 36404.1ns | 2862.4ns | 1271.8% | HIGH |
| reg_typed_twin | 34626.7ns | 2696.3ns | 1284.2% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 2457.1-3154.2 ns)
   2457.1 |##################################
   2492.0 |
   2526.8 |
   2561.7 |##############
   2596.5 |
   2631.4 |
   2666.2 |
   2701.1 |
   2735.9 |
   2770.8 |
   2805.6 |
   2840.5 |
   2875.3 |
   2910.2 |
   2945.0 |
   2979.9 |##########
   3014.7 |
   3049.6 |
   3084.4 |
   3119.3 |########################################
  (15 below, 13 above range)

reg_bound_record (n=160, range 18072.6-23530.7 ns)
  18072.6 |########################################
  18345.5 |
  18618.4 |
  18891.3 |
  19164.2 |
  19437.1 |
  19710.0 |
  19982.9 |
  20255.9 |
  20528.8 |
  20801.7 |
  21074.6 |
  21347.5 |
  21620.4 |
  21893.3 |##############
  22166.2 |#
  22439.1 |
  22712.0 |
  22984.9 |
  23257.8 |
  (16 below, 1 above range)

reg_bound_strided (n=160, range 4836.6-7589.8 ns)
   4836.6 |####################
   4974.3 |
   5111.9 |
   5249.6 |
   5387.3 |################################
   5524.9 |###
   5662.6 |
   5800.2 |########################################
   5937.9 |###########
   6075.5 |
   6213.2 |
   6350.9 |
   6488.5 |
   6626.2 |
   6763.8 |
   6901.5 |
   7039.1 |
   7176.8 |
   7314.5 |
   7452.1 |
  (17 below, 21 above range)

reg_bound_units (n=160, range 2474.7-3101.9 ns)
   2474.7 |##
   2506.1 |########################################
   2537.4 |##########
   2568.8 |
   2600.1 |
   2631.5 |
   2662.9 |
   2694.2 |
   2725.6 |
   2756.9 |
   2788.3 |
   2819.6 |
   2851.0 |
   2882.4 |
   2913.7 |
   2945.1 |
   2976.4 |############################
   3007.8 |##########
   3039.1 |
   3070.5 |########################
  (25 below, 15 above range)

reg_typed (n=160, range 2490.0-3106.0 ns)
   2490.0 |
   2520.8 |########################
   2551.6 |
   2582.4 |
   2613.2 |
   2644.0 |
   2674.8 |
   2705.6 |
   2736.4 |
   2767.2 |
   2798.0 |
   2828.8 |
   2859.6 |
   2890.4 |
   2921.2 |
   2952.0 |
   2982.8 |#########
   3013.6 |########
   3044.4 |
   3075.2 |########################################
  (20 below, 8 above range)

reg_typed_twin (n=160, range 2459.2-3065.6 ns)
   2459.2 |########################################
   2489.5 |
   2519.8 |###################################
   2550.1 |
   2580.5 |
   2610.8 |
   2641.1 |
   2671.4 |
   2701.7 |
   2732.1 |
   2762.4 |
   2792.7 |
   2823.0 |
   2853.4 |
   2883.7 |
   2914.0 |
   2944.3 |
   2974.7 |#########
   3005.0 |#########################
   3035.3 |
  (15 below, 20 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=1376.7% of algo (FFI overhead may distort results)
- **reg_bound_record**: CV=21.2% (high variance, measurements may be unstable)
- **reg_bound_record**: bridge=176.6% of algo (FFI overhead may distort results)
- **reg_bound_strided**: CV=27.9% (high variance, measurements may be unstable)
- **reg_bound_strided**: bridge=695.6% of algo (FFI overhead may distort results)
- **reg_bound_units**: autocorrelation=0.66 (measurement drift or warm-up artifact)
- **reg_bound_units**: bridge=1282.8% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=1293.3% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=1264.3% of algo (FFI overhead may distort results)

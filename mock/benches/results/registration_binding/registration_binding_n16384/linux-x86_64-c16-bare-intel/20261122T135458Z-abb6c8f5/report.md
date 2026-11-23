# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 8.7x slower than the field

reg_bound_record (22.29 us) is 8.7x the fastest (2.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (reg_bound_units, reg_typed_twin) are a dead heat (<1%)

reg_bound_units (2.56 us) and reg_typed_twin (2.56 us) differ by 0.26%, inside the noise, even though the wider field spreads 772.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### reg_bound_strided shows alternating (throttle bounce) (autocorr -0.68)

reg_bound_strided's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {reg_bound_units, reg_typed_twin, reg_typed, reg_bound_columns, reg_bound_strided} vs {reg_bound_record} (362% apart)

The field splits into a fast tier {reg_bound_units, reg_typed_twin, reg_typed, reg_bound_columns, reg_bound_strided} and a slow tier {reg_bound_record} with a 362% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 8.7x the fastest

Fastest reg_bound_units (2.56 us) to slowest reg_bound_record (22.29 us): 8.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: reg_bound_units** at 2555.2 ns median (-0.4% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 8.72x (fastest 2555.2 ns, slowest 22292.6 ns)

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
| Counter frequency | 2112791800 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| reg_bound_columns | 4316ns | 4138ns | 4023ns | 4188ns | 4994ns | -2.46% |
| reg_bound_record | 23546ns | 24235ns | 19742ns | 23880ns | 26350ns | +432.15% |
| reg_bound_strided | 6831ns | 6401ns | 6021ns | 6754ns | 7872ns | +54.38% |
| reg_bound_units | 4223ns | 4118ns | 4028ns | 4107ns | 4762ns | -4.57% |
| reg_typed | 4425ns | 4136ns | 4024ns | 4352ns | 5045ns | base |
| reg_typed_twin | 4442ns | 4139ns | 4024ns | 4368ns | 5080ns | +0.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 2664ns | 2463ns | 3078ns | -2.22% | 6.151 |
| reg_bound_record | 21659ns | 18152ns | 24265ns | +695.11% | 0.756 |
| reg_bound_strided | 5124ns | 4447ns | 5941ns | +88.11% | 3.197 |
| reg_bound_units | 2612ns | 2467ns | 2975ns | -4.10% | 6.272 |
| reg_typed | 2724ns | 2456ns | 3124ns | base | 6.015 |
| reg_typed_twin | 2742ns | 2459ns | 3160ns | +0.66% | 5.975 |

## Performance model

- Peak throughput: **6.670 Gops/s** (reg_typed; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 6.371 | 95.5% |
| reg_bound_record | 0.735 | 11.0% |
| reg_bound_strided | 3.396 | 50.9% |
| reg_bound_units | 6.412 | 96.1% |
| reg_typed | 6.384 | 95.7% |
| reg_typed_twin | 6.396 | 95.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 4316ns | 4316ns | -2.46% |
| reg_bound_record | 23546ns | 23546ns | +432.15% |
| reg_bound_strided | 6831ns | 6831ns | +54.38% |
| reg_bound_units | 4223ns | 4223ns | -4.57% |
| reg_typed | 4425ns | 4425ns | base |
| reg_typed_twin | 4442ns | 4442ns | +0.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 2566ns | base | --- | [2562, 2573] | --- | --- | --- | --- |
| reg_bound_columns | 2572ns | no significant difference | [-13, +13]ns | [2548, 2579] | no | 0.4807 | 0.3846 | 0 |
| reg_bound_record | 22293ns | +19626.2ns (+764.7%) | [+19466, +19646]ns | [22143, 22326] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 4824ns | +2181.0ns (+85.0%) | [+1983, +2273]ns | [4811, 4838] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 2555ns | no significant difference | [-30, +33]ns | [2526, 2560] | no | 0.8126 | 0.8126 | 0 |
| reg_typed_twin | 2562ns | no significant difference | [-2, +59]ns | [2538, 2592] | no | 0.1609 | 0.0966 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2461ns | +0.4% | +807.8% | +138.9% | +5.0% | -0.1% |
| 1 | 2 | 3132ns | -17.8% | +478.6% | +42.8% | -18.3% | -3.5% |
| 1 | 3 | 2567ns | +21.0% | +609.9% | +129.2% | -3.8% | +23.0% |
| 1 | 4 | 2454ns | +3.5% | +800.4% | +81.6% | +4.2% | +3.4% |
| 1 | 5 | 2520ns | -2.0% | +785.8% | +133.8% | -1.0% | +2.8% |
| 1 | 6 | 2999ns | -17.7% | +649.3% | +60.2% | -17.6% | -18.0% |
| 1 | 7 | 3020ns | -0.1% | +633.3% | +56.5% | +5.5% | +4.2% |
| 1 | 8 | 2562ns | +1.0% | +783.8% | +88.6% | +1.8% | -1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | -0.237 | moderate- |
| reg_bound_record | 0.311 | moderate+ |
| reg_bound_strided | -0.682 | HIGH- (thermal bounce) |
| reg_bound_units | -0.110 | ok |
| reg_typed | -0.166 | ok |
| reg_typed_twin | -0.252 | moderate- |

**Consistency summary:**

- **reg_bound_columns**: won 4/8, lost 4/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 4/8, lost 4/8
- **reg_typed_twin**: won 3/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 33770.4ns | 2663.6ns | 1267.8% | HIGH |
| reg_bound_record | 37872.9ns | 21658.7ns | 174.9% | HIGH |
| reg_bound_strided | 34690.8ns | 5124.0ns | 677.0% | HIGH |
| reg_bound_units | 32884.3ns | 2612.3ns | 1258.8% | HIGH |
| reg_typed | 34622.9ns | 2724.0ns | 1271.0% | HIGH |
| reg_typed_twin | 34636.6ns | 2742.1ns | 1263.2% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 2463.1-3077.7 ns)
   2463.1 |#####################################
   2493.8 |#
   2524.6 |####################################
   2555.3 |########################################
   2586.0 |##################
   2616.7 |
   2647.5 |
   2678.2 |
   2708.9 |
   2739.7 |
   2770.4 |
   2801.1 |
   2831.9 |
   2862.6 |
   2893.3 |
   2924.0 |
   2954.8 |
   2985.5 |############
   3016.2 |##########
   3047.0 |##
  (13 below, 20 above range)

reg_bound_record (n=160, range 18151.8-24264.6 ns)
  18151.8 |##############
  18457.5 |
  18763.1 |
  19068.7 |
  19374.4 |
  19680.0 |
  19985.7 |
  20291.3 |
  20596.9 |
  20902.6 |
  21208.2 |
  21513.9 |
  21819.5 |###################
  22125.1 |########################################
  22430.8 |############################
  22736.4 |
  23042.1 |
  23347.7 |
  23653.3 |
  23959.0 |
  (20 below, 2 above range)

reg_bound_strided (n=160, range 4447.2-5941.2 ns)
   4447.2 |#################
   4521.9 |########
   4596.6 |#
   4671.3 |
   4746.0 |####################
   4820.7 |##################
   4895.4 |
   4970.1 |
   5044.8 |
   5119.5 |
   5194.2 |
   5268.9 |
   5343.6 |
   5418.3 |
   5493.0 |
   5567.7 |
   5642.4 |
   5717.1 |
   5791.8 |####
   5866.5 |########################################
  (15 below, 3 above range)

reg_bound_units (n=160, range 2466.8-2974.7 ns)
   2466.8 |########################################
   2492.2 |########
   2517.6 |#
   2543.0 |#################################
   2568.4 |##############
   2593.8 |#################
   2619.2 |##
   2644.6 |
   2670.0 |
   2695.4 |
   2720.8 |
   2746.1 |
   2771.5 |
   2796.9 |
   2822.3 |
   2847.7 |
   2873.1 |
   2898.5 |
   2923.9 |
   2949.3 |
  (16 below, 20 above range)

reg_typed (n=160, range 2456.4-3124.3 ns)
   2456.4 |###################
   2489.8 |############
   2523.2 |########
   2556.6 |########################################
   2590.0 |
   2623.4 |
   2656.8 |
   2690.2 |
   2723.5 |
   2756.9 |
   2790.3 |
   2823.7 |
   2857.1 |
   2890.5 |
   2923.9 |
   2957.3 |
   2990.7 |##############################
   3024.1 |
   3057.5 |
   3090.9 |
  (17 below, 28 above range)

reg_typed_twin (n=160, range 2458.6-3159.6 ns)
   2458.6 |#############################
   2493.6 |########
   2528.7 |########################################
   2563.7 |#######################
   2598.8 |
   2633.8 |#
   2668.9 |
   2703.9 |
   2739.0 |
   2774.0 |
   2809.1 |
   2844.1 |
   2879.2 |
   2914.2 |
   2949.3 |
   2984.3 |#########
   3019.4 |#########
   3054.5 |
   3089.5 |#
   3124.6 |######################################
  (16 below, 11 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=1243.7% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=175.3% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=663.6% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=1251.3% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=1245.2% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=1247.7% of algo (FFI overhead may distort results)

# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 9.8x slower than the field

reg_bound_record (8.38 us) is 9.8x the fastest (854 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {reg_bound_columns, reg_bound_units, reg_typed_twin, reg_typed, reg_bound_strided} vs {reg_bound_record} (317% apart)

The field splits into a fast tier {reg_bound_columns, reg_bound_units, reg_typed_twin, reg_typed, reg_bound_strided} and a slow tier {reg_bound_record} with a 317% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 9.8x the fastest

Fastest reg_bound_columns (854 ns) to slowest reg_bound_record (8.38 us): 9.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### reg_bound_strided is inconsistent: worst-20% is 3.3x its best-20%

reg_bound_strided's best 20% of batches run at 1.33 us but its worst 20% at 4.37 us (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: reg_bound_columns** at 853.8 ns median (-7.7% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 9.82x (fastest 853.8 ns, slowest 8384.1 ns)

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
| Counter frequency | 2100480538 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| reg_bound_columns | 3160ns | 3085ns | 2853ns | 3097ns | 3654ns | -10.12% |
| reg_bound_record | 11307ns | 10622ns | 8622ns | 10755ns | 15646ns | +221.63% |
| reg_bound_strided | 4765ns | 4426ns | 3217ns | 4248ns | 7864ns | +35.54% |
| reg_bound_units | 3439ns | 3076ns | 2751ns | 3110ns | 5118ns | -2.16% |
| reg_typed | 3515ns | 3155ns | 2770ns | 3228ns | 5122ns | base |
| reg_typed_twin | 3247ns | 3167ns | 2971ns | 3192ns | 3689ns | -7.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 894ns | 805ns | 1058ns | -27.04% | 4.582 |
| reg_bound_record | 8946ns | 6443ns | 12953ns | +630.09% | 0.458 |
| reg_bound_strided | 2296ns | 1329ns | 4375ns | +87.41% | 1.784 |
| reg_bound_units | 1107ns | 780ns | 2060ns | -9.65% | 3.700 |
| reg_typed | 1225ns | 785ns | 2468ns | base | 3.343 |
| reg_typed_twin | 956ns | 839ns | 1177ns | -21.99% | 4.285 |

## Performance model

- Peak throughput: **5.250 Gops/s** (reg_bound_units; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 4.797 | 91.4% |
| reg_bound_record | 0.489 | 9.3% |
| reg_bound_strided | 2.037 | 38.8% |
| reg_bound_units | 4.696 | 89.4% |
| reg_typed | 4.429 | 84.4% |
| reg_typed_twin | 4.534 | 86.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 3160ns | 3160ns | -10.12% |
| reg_bound_record | 11307ns | 11307ns | +221.63% |
| reg_bound_strided | 4765ns | 4765ns | +35.54% |
| reg_bound_units | 3439ns | 3439ns | -2.16% |
| reg_typed | 3515ns | 3515ns | base |
| reg_typed_twin | 3247ns | 3247ns | -7.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 925ns | base | --- | [856, 1002] | --- | --- | --- | --- |
| reg_bound_columns | 854ns | -33.6ns (-3.6%) | [-102, -2]ns | [848, 861] | YES (adj: no) | 0.0796 | 0.0478 | 0 |
| reg_bound_record | 8384ns | +7106.2ns (+768.5%) | [+6359, +7783]ns | [7609, 8820] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 2011ns | +705.2ns (+76.3%) | [+633, +869]ns | [1738, 2130] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 872ns | no significant difference | [-54, +8]ns | [862, 883] | no | 0.0859 | 0.0687 | 0 |
| reg_typed_twin | 903ns | no significant difference | [-89, +3]ns | [870, 962] | no | 0.1788 | 0.1788 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1317ns | -35.6% | +397.2% | +77.1% | -32.7% | -25.2% |
| 1 | 2 | 1135ns | -24.5% | +638.7% | +109.6% | -28.2% | -24.4% |
| 1 | 3 | 837ns | +1.9% | +1041.7% | +650.1% | -6.0% | +38.0% |
| 1 | 4 | 799ns | -5.0% | +727.9% | +84.0% | +9.9% | +7.3% |
| 1 | 5 | 1405ns | -39.4% | +489.2% | +41.9% | -38.3% | -38.9% |
| 1 | 6 | 1022ns | -16.9% | +1181.6% | +44.0% | +9.8% | -16.1% |
| 1 | 7 | 851ns | -0.2% | +1302.7% | +61.1% | +208.4% | -1.1% |
| 1 | 8 | 781ns | +30.5% | +751.9% | +79.8% | +0.0% | +26.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | 0.006 | ok |
| reg_bound_record | 0.096 | ok |
| reg_bound_strided | -0.049 | ok |
| reg_bound_units | -0.067 | ok |
| reg_typed | 0.021 | ok |
| reg_typed_twin | -0.300 | moderate- |

**Consistency summary:**

- **reg_bound_columns**: won 6/8, lost 2/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 4/8, lost 3/8
- **reg_typed_twin**: won 5/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 11520.2ns | 893.9ns | 1288.7% | HIGH |
| reg_bound_record | 11762.1ns | 8946.0ns | 131.5% | HIGH |
| reg_bound_strided | 12302.3ns | 2296.4ns | 535.7% | HIGH |
| reg_bound_units | 11629.9ns | 1107.1ns | 1050.5% | HIGH |
| reg_typed | 11987.3ns | 1225.3ns | 978.3% | HIGH |
| reg_typed_twin | 11535.0ns | 955.9ns | 1206.8% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 804.7-1057.5 ns)
    804.7 |
    817.4 |
    830.0 |
    842.6 |########################################
    855.3 |########
    867.9 |##
    880.6 |####
    893.2 |####
    905.8 |#
    918.5 |
    931.1 |
    943.8 |
    956.4 |##
    969.0 |##
    981.7 |##
    994.3 |##
   1007.0 |##
   1019.6 |
   1032.2 |
   1044.9 |#
  (15 below, 13 above range)

reg_bound_record (n=160, range 6443.3-12952.8 ns)
   6443.3 |########################################
   6768.8 |##########
   7094.3 |###
   7419.8 |##
   7745.2 |#
   8070.7 |##########
   8396.2 |##########
   8721.7 |######
   9047.1 |###
   9372.6 |######
   9698.1 |#####
  10023.5 |#####
  10349.0 |###
  10674.5 |##
  11000.0 |#
  11325.4 |#
  11650.9 |#########
  11976.4 |##
  12301.9 |###
  12627.3 |###
  (14 below, 21 above range)

reg_bound_strided (n=160, range 1329.1-4374.9 ns)
   1329.1 |########################################
   1481.4 |#########
   1633.7 |######
   1786.0 |#####
   1938.2 |########
   2090.5 |##############
   2242.8 |###########
   2395.1 |############
   2547.4 |#####
   2699.7 |###
   2852.0 |#
   3004.3 |
   3156.6 |
   3308.8 |
   3461.1 |
   3613.4 |
   3765.7 |
   3918.0 |#
   4070.3 |
   4222.6 |#
  (8 below, 12 above range)

reg_bound_units (n=160, range 780.1-2060.5 ns)
    780.1 |###################################
    844.1 |########################################
    908.2 |#####
    972.2 |#########
   1036.2 |######
   1100.2 |#####
   1164.2 |###
   1228.2 |#
   1292.3 |
   1356.3 |
   1420.3 |
   1484.3 |
   1548.3 |
   1612.4 |
   1676.4 |
   1740.4 |
   1804.4 |
   1868.4 |
   1932.4 |
   1996.5 |
  (12 below, 18 above range)

reg_typed (n=160, range 784.5-2467.8 ns)
    784.5 |########################################
    868.7 |########
    952.9 |##########
   1037.0 |########
   1121.2 |############
   1205.4 |#
   1289.5 |##
   1373.7 |######
   1457.8 |##
   1542.0 |###
   1626.2 |#
   1710.3 |
   1794.5 |
   1878.7 |
   1962.8 |#
   2047.0 |
   2131.1 |
   2215.3 |
   2299.5 |
   2383.6 |
  (17 below, 1 above range)

reg_typed_twin (n=160, range 839.0-1176.9 ns)
    839.0 |########################################
    855.9 |######
    872.8 |####
    889.7 |#####
    906.6 |#
    923.5 |
    940.4 |###
    957.2 |#####
    974.1 |############
    991.0 |####
   1007.9 |
   1024.8 |##
   1041.7 |###
   1058.6 |##
   1075.5 |#
   1092.4 |##
   1109.3 |##
   1126.2 |##
   1143.1 |
   1160.0 |
  (5 below, 15 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=1309.0% of algo (FFI overhead may distort results)
- **reg_bound_record**: CV=27.8% (high variance, measurements may be unstable)
- **reg_bound_record**: bridge=134.2% of algo (FFI overhead may distort results)
- **reg_bound_strided**: CV=59.3% (high variance, measurements may be unstable)
- **reg_bound_strided**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **reg_bound_strided**: bridge=596.8% of algo (FFI overhead may distort results)
- **reg_bound_units**: CV=53.4% (high variance, measurements may be unstable)
- **reg_bound_units**: bridge=1268.0% of algo (FFI overhead may distort results)
- **reg_typed**: CV=200.5% (high variance, measurements may be unstable)
- **reg_typed**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **reg_typed**: bridge=1212.6% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=1255.0% of algo (FFI overhead may distort results)

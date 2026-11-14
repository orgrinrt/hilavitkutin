# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 8.0x slower than the field

reg_bound_record (108.21 us) is 8.0x the fastest (13.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### reg_typed_twin is fastest but the noisiest (CV 42.4%)

reg_typed_twin wins on median (13.50 us) yet has the highest variance (CV 42.4%), while reg_bound_strided is the steadiest (CV 13.4%, 25.23 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {reg_typed_twin, reg_typed, reg_bound_units, reg_bound_columns, reg_bound_strided} vs {reg_bound_record} (329% apart)

The field splits into a fast tier {reg_typed_twin, reg_typed, reg_bound_units, reg_bound_columns, reg_bound_strided} and a slow tier {reg_bound_record} with a 329% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 8.0x the fastest

Fastest reg_typed_twin (13.50 us) to slowest reg_bound_record (108.21 us): 8.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### reg_typed_twin is inconsistent: worst-20% is 1.7x its best-20%

reg_typed_twin's best 20% of batches run at 11.98 us but its worst 20% at 20.74 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: reg_typed_twin** at 13498.8 ns median (-1.0% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 8.02x (fastest 13498.8 ns, slowest 108211.9 ns)

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
| reg_bound_columns | 18337ns | 16233ns | 15585ns | 16914ns | 25356ns | +9.79% |
| reg_bound_record | 123508ns | 110856ns | 100321ns | 115677ns | 170188ns | +639.49% |
| reg_bound_strided | 28836ns | 27676ns | 25409ns | 28113ns | 34431ns | +72.65% |
| reg_bound_units | 16028ns | 16039ns | 12849ns | 16036ns | 19181ns | -4.04% |
| reg_typed | 16702ns | 15994ns | 13291ns | 16107ns | 21899ns | base |
| reg_typed_twin | 17139ns | 15836ns | 14071ns | 15864ns | 24031ns | +2.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 15861ns | 13310ns | 22494ns | +10.22% | 4.132 |
| reg_bound_record | 120640ns | 97891ns | 166775ns | +738.33% | 0.543 |
| reg_bound_strided | 26357ns | 23211ns | 31673ns | +83.16% | 2.486 |
| reg_bound_units | 13717ns | 10955ns | 16503ns | -4.68% | 4.778 |
| reg_typed | 14390ns | 11344ns | 19268ns | base | 4.554 |
| reg_typed_twin | 14664ns | 11981ns | 20736ns | +1.90% | 4.469 |

## Performance model

- Peak throughput: **5.982 Gops/s** (reg_bound_units; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 4.737 | 79.2% |
| reg_bound_record | 0.606 | 10.1% |
| reg_bound_strided | 2.597 | 43.4% |
| reg_bound_units | 4.785 | 80.0% |
| reg_typed | 4.804 | 80.3% |
| reg_typed_twin | 4.855 | 81.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 18337ns | 18337ns | +9.79% |
| reg_bound_record | 123508ns | 123508ns | +639.49% |
| reg_bound_strided | 28836ns | 28836ns | +72.65% |
| reg_bound_units | 16028ns | 16028ns | -4.04% |
| reg_typed | 16702ns | 16702ns | base |
| reg_typed_twin | 17139ns | 17139ns | +2.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 13641ns | base | --- | [13451, 13896] | --- | --- | --- | --- |
| reg_bound_columns | 13834ns | +541.5ns (+4.0%) | [+48, +1018]ns | [13757, 14129] | YES | 0.0360 | 0.0216 | 0 |
| reg_bound_record | 108212ns | +93384.6ns (+684.6%) | [+91821, +95717]ns | [106426, 110236] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 25231ns | +11902.5ns (+87.3%) | [+11095, +12385]ns | [25048, 25681] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 13697ns | no significant difference | [-38, +297]ns | [13589, 13805] | no | 0.3041 | 0.3041 | 0 |
| reg_typed_twin | 13499ns | no significant difference | [-1004, +216]ns | [13308, 13663] | no | 0.3041 | 0.3041 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 13824ns | -1.7% | +1158.8% | +86.5% | +1.4% | -11.4% |
| 1 | 2 | 17855ns | -24.0% | +523.1% | +40.8% | -23.7% | -24.3% |
| 1 | 3 | 13975ns | -2.7% | +716.3% | +112.0% | +9.4% | -2.2% |
| 1 | 4 | 12857ns | +59.9% | +1033.8% | +129.7% | +6.4% | +39.9% |
| 1 | 5 | 15302ns | -5.2% | +588.0% | +65.8% | -11.0% | -16.4% |
| 1 | 6 | 10711ns | +30.9% | +897.1% | +133.4% | -0.1% | +28.5% |
| 1 | 7 | 13160ns | +7.2% | +703.6% | +86.9% | +4.6% | +3.7% |
| 1 | 8 | 13366ns | +47.3% | +642.4% | +84.7% | -0.3% | -5.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | -0.165 | ok |
| reg_bound_record | -0.056 | ok |
| reg_bound_strided | 0.345 | moderate+ |
| reg_bound_units | -0.042 | ok |
| reg_typed | -0.109 | ok |
| reg_typed_twin | -0.180 | ok |

**Consistency summary:**

- **reg_bound_columns**: won 4/8, lost 4/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 3/8, lost 4/8
- **reg_typed_twin**: won 5/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 184752.1ns | 15860.9ns | 1164.8% | HIGH |
| reg_bound_record | 182330.6ns | 120639.6ns | 151.1% | HIGH |
| reg_bound_strided | 179559.7ns | 26356.9ns | 681.3% | HIGH |
| reg_bound_units | 170732.5ns | 13717.4ns | 1244.6% | HIGH |
| reg_typed | 171772.1ns | 14390.4ns | 1193.7% | HIGH |
| reg_typed_twin | 177899.0ns | 14664.4ns | 1213.1% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 13309.8-22493.7 ns)
  13309.8 |########################################
  13769.0 |###############
  14228.2 |#####
  14687.4 |#
  15146.6 |#
  15605.8 |#
  16065.0 |##
  16524.2 |#####
  16983.4 |
  17442.6 |#
  17901.7 |###
  18360.9 |#
  18820.1 |
  19279.3 |
  19738.5 |
  20197.7 |#
  20656.9 |#
  21116.1 |###
  21575.3 |##
  22034.5 |##
  (9 below, 6 above range)

reg_bound_record (n=160, range 97890.6-166774.6 ns)
  97890.6 |##########################
  101334.8 |###################
  104779.0 |########################################
  108223.2 |####################
  111667.4 |########
  115111.6 |####
  118555.8 |##
  122000.0 |###
  125444.2 |#
  128888.4 |####
  132332.6 |#
  135776.8 |##
  139221.0 |###
  142665.2 |####
  146109.4 |#####
  149553.6 |###
  152997.8 |###
  156442.0 |#
  159886.2 |#
  163330.4 |#
  (5 below, 17 above range)

reg_bound_strided (n=160, range 23211.2-31673.3 ns)
  23211.2 |########
  23634.3 |##
  24057.4 |############
  24480.5 |########################################
  24903.6 |##################################
  25326.7 |################
  25749.8 |################
  26173.0 |######
  26596.1 |#####
  27019.2 |#####
  27442.3 |####
  27865.4 |##
  28288.5 |##
  28711.6 |#
  29134.7 |######
  29557.8 |####
  29980.9 |######
  30404.0 |#########
  30827.1 |#####
  31250.2 |
  (10 below, 8 above range)

reg_bound_units (n=160, range 10954.7-16502.5 ns)
  10954.7 |##
  11232.1 |#
  11509.5 |
  11786.9 |
  12064.3 |#
  12341.7 |#
  12619.1 |#####
  12896.4 |##
  13173.8 |##################
  13451.2 |########################################
  13728.6 |###############################
  14006.0 |#####################
  14283.4 |###########
  14560.8 |########
  14838.2 |#
  15115.6 |
  15392.9 |#
  15670.3 |#
  15947.7 |#
  16225.1 |#
  (25 below, 14 above range)

reg_typed (n=160, range 11344.3-19268.2 ns)
  11344.3 |#
  11740.5 |#
  12136.7 |########
  12532.9 |########################################
  12929.1 |###############
  13325.3 |####################################
  13721.5 |#################################
  14117.7 |##################
  14513.8 |###
  14910.0 |###
  15306.2 |#############
  15702.4 |########
  16098.6 |
  16494.8 |#
  16891.0 |###
  17287.2 |##########
  17683.4 |######
  18079.6 |###########
  18475.8 |
  18872.0 |#
  (20 below, 9 above range)

reg_typed_twin (n=160, range 11980.5-20736.3 ns)
  11980.5 |############
  12418.3 |##################################
  12856.1 |#########################
  13293.9 |########################################
  13731.7 |#############################
  14169.5 |######
  14607.3 |
  15045.1 |#
  15482.9 |####
  15920.7 |####
  16358.4 |#
  16796.2 |####
  17234.0 |#########
  17671.8 |######
  18109.6 |##
  18547.4 |##
  18985.2 |#
  19423.0 |
  19860.8 |#
  20298.6 |#
  (13 below, 6 above range)

```

## Diagnostics

- **reg_bound_columns**: CV=26.4% (high variance, measurements may be unstable)
- **reg_bound_columns**: bridge=1279.8% of algo (FFI overhead may distort results)
- **reg_bound_record**: CV=21.8% (high variance, measurements may be unstable)
- **reg_bound_record**: bridge=167.4% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=704.7% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=1280.0% of algo (FFI overhead may distort results)
- **reg_typed**: CV=21.1% (high variance, measurements may be unstable)
- **reg_typed**: bridge=1253.7% of algo (FFI overhead may distort results)
- **reg_typed_twin**: CV=39.0% (high variance, measurements may be unstable)
- **reg_typed_twin**: bridge=1302.2% of algo (FFI overhead may distort results)

# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 7.3x slower than the field

reg_bound_record (4.54 us) is 7.3x the fastest (618 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (reg_typed, reg_typed_twin) are a dead heat (<1%)

reg_typed (618 ns) and reg_typed_twin (622 ns) differ by 0.65%, inside the noise, even though the wider field spreads 633.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (reg_typed)

The baseline reg_typed is the fastest (618 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {reg_typed, reg_typed_twin, reg_bound_units, reg_bound_columns, reg_bound_strided} vs {reg_bound_record} (247% apart)

The field splits into a fast tier {reg_typed, reg_typed_twin, reg_bound_units, reg_bound_columns, reg_bound_strided} and a slow tier {reg_bound_record} with a 247% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.3x the fastest

Fastest reg_typed (618 ns) to slowest reg_bound_record (4.54 us): 7.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### reg_bound_columns's edge over baseline is significant but tiny (9 ns, 1.53%)

reg_bound_columns differs from baseline reg_typed by 9 ns (1.53%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (reg_typed) is the fastest** at 618.2 ns median
- 5 variants significantly slower than baseline
- Spread: 7.34x (fastest 618.2 ns, slowest 4536.8 ns)

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
| reg_bound_columns | 2412ns | 2428ns | 2161ns | 2409ns | 2670ns | +8.43% |
| reg_bound_record | 6613ns | 6106ns | 6086ns | 6504ns | 7467ns | +197.31% |
| reg_bound_strided | 2967ns | 3026ns | 2646ns | 2963ns | 3298ns | +33.39% |
| reg_bound_units | 2356ns | 2178ns | 2167ns | 2316ns | 2665ns | +5.93% |
| reg_typed | 2224ns | 2161ns | 2152ns | 2162ns | 2485ns | base |
| reg_typed_twin | 2347ns | 2166ns | 2156ns | 2306ns | 2660ns | +5.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 695ns | 623ns | 769ns | +9.41% | 5.889 |
| reg_bound_record | 4912ns | 4521ns | 5548ns | +672.72% | 0.834 |
| reg_bound_strided | 1246ns | 1100ns | 1400ns | +95.99% | 3.288 |
| reg_bound_units | 680ns | 626ns | 770ns | +6.97% | 6.023 |
| reg_typed | 636ns | 616ns | 707ns | base | 6.443 |
| reg_typed_twin | 674ns | 620ns | 763ns | +5.99% | 6.079 |

## Performance model

- Peak throughput: **6.646 Gops/s** (reg_typed; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 5.853 | 88.1% |
| reg_bound_record | 0.903 | 13.6% |
| reg_bound_strided | 3.131 | 47.1% |
| reg_bound_units | 6.522 | 98.1% |
| reg_typed | 6.626 | 99.7% |
| reg_typed_twin | 6.583 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 2412ns | 2412ns | +8.43% |
| reg_bound_record | 6613ns | 6613ns | +197.31% |
| reg_bound_strided | 2967ns | 2967ns | +33.39% |
| reg_bound_units | 2356ns | 2356ns | +5.93% |
| reg_typed | 2224ns | 2224ns | base |
| reg_typed_twin | 2347ns | 2347ns | +5.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 618ns | base | --- | [618, 618] | --- | --- | --- | --- |
| reg_bound_columns | 700ns | +9.4ns (+1.5%) | [+8, +12]ns | [627, 762] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_record | 4537ns | +3918.4ns (+633.8%) | [+3915, +3925]ns | [4532, 4543] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 1308ns | +678.6ns (+109.8%) | [+514, +726]ns | [1136, 1344] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 628ns | +9.4ns (+1.5%) | [+9, +12]ns | [627, 630] | YES | 0.0000 | 0.0000 | 0 |
| reg_typed_twin | 622ns | +3.9ns (+0.6%) | [+3, +5]ns | [622, 623] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 618ns | +1.2% | +632.7% | +122.5% | +1.4% | +0.4% |
| 1 | 2 | 618ns | +23.5% | +794.8% | +78.8% | +1.4% | +0.5% |
| 1 | 3 | 755ns | +1.2% | +633.0% | +48.3% | -16.8% | -17.6% |
| 1 | 4 | 619ns | +0.8% | +632.1% | +119.3% | +1.3% | +22.6% |
| 1 | 5 | 618ns | +1.1% | +796.3% | +119.1% | +23.9% | +22.9% |
| 1 | 6 | 618ns | +23.6% | +632.2% | +80.5% | +24.2% | +0.6% |
| 1 | 7 | 618ns | +1.2% | +632.8% | +80.6% | +1.4% | +0.5% |
| 1 | 8 | 617ns | +24.0% | +634.4% | +121.9% | +24.3% | +23.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | -0.374 | moderate- |
| reg_bound_record | -0.143 | ok |
| reg_bound_strided | -0.141 | ok |
| reg_bound_units | 0.054 | ok |
| reg_typed | -0.151 | ok |
| reg_typed_twin | 0.058 | ok |

**Consistency summary:**

- **reg_bound_columns**: won 0/8, lost 8/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 1/8, lost 7/8
- **reg_typed_twin**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 8938.5ns | 695.5ns | 1285.2% | HIGH |
| reg_bound_record | 8710.1ns | 4912.1ns | 177.3% | HIGH |
| reg_bound_strided | 8930.1ns | 1245.9ns | 716.8% | HIGH |
| reg_bound_units | 8701.3ns | 680.0ns | 1279.6% | HIGH |
| reg_typed | 8254.1ns | 635.7ns | 1298.5% | HIGH |
| reg_typed_twin | 8693.2ns | 673.8ns | 1290.2% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 623.4-768.9 ns)
    623.4 |###################################
    630.7 |##
    638.0 |
    645.3 |
    652.5 |
    659.8 |
    667.1 |
    674.4 |
    681.6 |
    688.9 |
    696.2 |
    703.5 |
    710.7 |
    718.0 |
    725.3 |
    732.6 |
    739.8 |
    747.1 |
    754.4 |##
    761.7 |########################################
  (14 below, 7 above range)

reg_bound_record (n=160, range 4521.1-5548.0 ns)
   4521.1 |########################################
   4572.4 |
   4623.8 |
   4675.1 |
   4726.5 |
   4777.8 |
   4829.2 |
   4880.5 |
   4931.8 |
   4983.2 |
   5034.5 |
   5085.9 |
   5137.2 |
   5188.6 |
   5239.9 |
   5291.3 |
   5342.6 |
   5394.0 |
   5445.3 |
   5496.7 |#####################
  (12 below, 14 above range)

reg_bound_strided (n=160, range 1100.5-1400.0 ns)
   1100.5 |######################################
   1115.4 |###########################
   1130.4 |###############
   1145.4 |#####
   1160.4 |##
   1175.4 |#
   1190.3 |
   1205.3 |
   1220.3 |
   1235.3 |
   1250.2 |
   1265.2 |#
   1280.2 |
   1295.2 |
   1310.2 |
   1325.1 |
   1340.1 |########################################
   1355.1 |############################
   1370.1 |############
   1385.1 |##############
  (15 below, 13 above range)

reg_bound_units (n=160, range 625.6-770.4 ns)
    625.6 |########################################
    632.9 |#
    640.1 |
    647.3 |
    654.6 |
    661.8 |
    669.1 |
    676.3 |
    683.5 |
    690.8 |
    698.0 |
    705.3 |
    712.5 |
    719.7 |
    727.0 |
    734.2 |
    741.5 |
    748.7 |
    755.9 |
    763.2 |#########################
  (16 below, 10 above range)

reg_typed (n=160, range 616.3-707.0 ns)
    616.3 |########################################
    620.8 |##
    625.4 |
    629.9 |
    634.5 |
    639.0 |
    643.5 |
    648.1 |
    652.6 |
    657.1 |
    661.7 |
    666.2 |
    670.7 |
    675.3 |
    679.8 |
    684.3 |
    688.9 |
    693.4 |
    698.0 |
    702.5 |
  (14 below, 20 above range)

reg_typed_twin (n=160, range 620.0-762.6 ns)
    620.0 |########################################
    627.1 |##
    634.2 |
    641.3 |
    648.5 |
    655.6 |
    662.7 |
    669.9 |
    677.0 |
    684.1 |
    691.3 |
    698.4 |
    705.5 |
    712.7 |
    719.8 |
    726.9 |
    734.1 |
    741.2 |
    748.3 |
    755.4 |###########################
  (14 below, 6 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=1276.6% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=177.0% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=682.7% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=1278.7% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=1296.6% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=1289.7% of algo (FFI overhead may distort results)

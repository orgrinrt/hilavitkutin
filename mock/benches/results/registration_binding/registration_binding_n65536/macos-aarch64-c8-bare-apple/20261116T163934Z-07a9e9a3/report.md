# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 20.4x slower than the field

reg_bound_record (88.71 us) is 20.4x the fastest (4.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (reg_typed, reg_typed_twin) are a dead heat (<1%)

reg_typed (4.35 us) and reg_typed_twin (4.35 us) differ by 0.02%, inside the noise, even though the wider field spreads 1937.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (reg_typed)

The baseline reg_typed is the fastest (4.35 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 20.4x the fastest

Fastest reg_typed (4.35 us) to slowest reg_bound_record (88.71 us): 20.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (reg_typed) is the fastest** at 4353.7 ns median
- 4 variants significantly slower than baseline
- Spread: 20.38x (fastest 4353.7 ns, slowest 88706.6 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| reg_bound_columns | 6928ns | 6865ns | 6678ns | 6892ns | 7286ns | +2.75% |
| reg_bound_record | 91592ns | 91202ns | 90925ns | 91305ns | 93119ns | +1258.46% |
| reg_bound_strided | 22474ns | 22429ns | 22248ns | 22430ns | 22832ns | +233.33% |
| reg_bound_units | 7004ns | 6964ns | 6836ns | 6977ns | 7252ns | +3.88% |
| reg_typed | 6742ns | 6713ns | 6538ns | 6728ns | 6991ns | base |
| reg_typed_twin | 6773ns | 6736ns | 6577ns | 6756ns | 7020ns | +0.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 4520ns | 4473ns | 4654ns | +3.64% | 14.500 |
| reg_bound_record | 89105ns | 88526ns | 90560ns | +1943.30% | 0.735 |
| reg_bound_strided | 20029ns | 19940ns | 20157ns | +359.30% | 3.272 |
| reg_bound_units | 4590ns | 4575ns | 4614ns | +5.25% | 14.279 |
| reg_typed | 4361ns | 4344ns | 4396ns | base | 15.028 |
| reg_typed_twin | 4362ns | 4343ns | 4401ns | +0.04% | 15.023 |

## Performance model

- Peak throughput: **15.088 Gops/s** (reg_typed_twin; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 14.611 | 96.8% |
| reg_bound_record | 0.739 | 4.9% |
| reg_bound_strided | 3.274 | 21.7% |
| reg_bound_units | 14.287 | 94.7% |
| reg_typed | 15.053 | 99.8% |
| reg_typed_twin | 15.050 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 6928ns | 6928ns | +2.75% |
| reg_bound_record | 91592ns | 91592ns | +1258.46% |
| reg_bound_strided | 22474ns | 22474ns | +233.33% |
| reg_bound_units | 7004ns | 7004ns | +3.88% |
| reg_typed | 6742ns | 6742ns | base |
| reg_typed_twin | 6773ns | 6773ns | +0.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 4354ns | base | --- | [4353, 4355] | --- | --- | --- | --- |
| reg_bound_columns | 4485ns | +132.5ns (+3.0%) | [+128, +135]ns | [4483, 4490] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_record | 88707ns | +84353.1ns (+1937.5%) | [+84313, +84386]ns | [88684, 88748] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 20016ns | +15661.3ns (+359.7%) | [+15647, +15674]ns | [20007, 20028] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 4587ns | +230.6ns (+5.3%) | [+229, +233]ns | [4585, 4588] | YES | 0.0000 | 0.0000 | 0 |
| reg_typed_twin | 4355ns | no significant difference | [-3, +2]ns | [4353, 4358] | no | 0.9362 | 0.9362 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4362ns | +2.8% | +1932.7% | +358.8% | +5.2% | -0.1% |
| 1 | 2 | 4352ns | +2.9% | +1942.2% | +359.6% | +5.3% | -0.1% |
| 1 | 3 | 4358ns | +4.0% | +1933.8% | +359.3% | +5.4% | -0.0% |
| 1 | 4 | 4360ns | +2.8% | +1932.5% | +358.9% | +5.2% | -0.0% |
| 1 | 5 | 4348ns | +3.2% | +1975.6% | +360.6% | +5.4% | +0.3% |
| 1 | 6 | 4365ns | +3.1% | +1934.1% | +358.3% | +5.1% | +0.0% |
| 1 | 7 | 4350ns | +2.9% | +1938.6% | +360.9% | +5.3% | -0.0% |
| 1 | 8 | 4349ns | +3.5% | +1939.6% | +360.5% | +5.3% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | -0.482 | moderate- |
| reg_bound_record | -0.172 | ok |
| reg_bound_strided | -0.110 | ok |
| reg_bound_units | -0.190 | ok |
| reg_typed | -0.465 | moderate- |
| reg_typed_twin | -0.043 | ok |

**Consistency summary:**

- **reg_bound_columns**: won 0/8, lost 8/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 0/8, lost 8/8
- **reg_typed_twin**: won 1/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 129052.1ns | 4519.6ns | 2855.4% | HIGH |
| reg_bound_record | 129274.2ns | 89105.2ns | 145.1% | HIGH |
| reg_bound_strided | 128677.0ns | 20029.2ns | 642.4% | HIGH |
| reg_bound_units | 128663.7ns | 4589.7ns | 2803.3% | HIGH |
| reg_typed | 128692.6ns | 4360.8ns | 2951.1% | HIGH |
| reg_typed_twin | 128684.5ns | 4362.5ns | 2949.8% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 4473.0-4654.4 ns)
   4473.0 |########################################
   4482.1 |########################
   4491.2 |#########
   4500.2 |#####
   4509.3 |#####
   4518.4 |##
   4527.4 |####
   4536.5 |###
   4545.6 |#
   4554.6 |
   4563.7 |
   4572.8 |
   4581.8 |
   4590.9 |
   4600.0 |##
   4609.0 |
   4618.1 |
   4627.2 |
   4636.2 |
   4645.3 |
  (9 below, 12 above range)

reg_bound_record (n=160, range 88526.4-90559.9 ns)
  88526.4 |################################
  88628.1 |########################################
  88729.8 |##################
  88831.5 |########
  88933.1 |
  89034.8 |
  89136.5 |
  89238.1 |#
  89339.8 |
  89441.5 |
  89543.1 |##
  89644.8 |
  89746.5 |
  89848.2 |
  89949.8 |
  90051.5 |#####
  90153.2 |########
  90254.8 |###
  90356.5 |#####
  90458.2 |#
  (15 below, 8 above range)

reg_bound_strided (n=160, range 19939.6-20157.3 ns)
  19939.6 |##########
  19950.5 |#####
  19961.4 |#####################################
  19972.3 |##########################
  19983.1 |##################
  19994.0 |#####################################
  20004.9 |################################
  20015.8 |################################
  20026.7 |########################################
  20037.6 |########################
  20048.5 |########################
  20059.3 |##########################
  20070.2 |#####
  20081.1 |################
  20092.0 |################
  20102.9 |
  20113.8 |########
  20124.6 |
  20135.5 |##
  20146.4 |
  (16 below, 8 above range)

reg_bound_units (n=160, range 4574.8-4614.3 ns)
   4574.8 |########################
   4576.7 |#################
   4578.7 |##########################
   4580.7 |###################################
   4582.7 |######################
   4584.6 |###############
   4586.6 |########################################
   4588.6 |#################################
   4590.6 |###############################
   4592.5 |#############
   4594.5 |######################
   4596.5 |#############
   4598.5 |##
   4600.4 |####
   4602.4 |##
   4604.4 |##
   4606.4 |
   4608.4 |
   4610.3 |
   4612.3 |
  (12 below, 10 above range)

reg_typed (n=160, range 4343.6-4396.3 ns)
   4343.6 |###################
   4346.2 |#################################
   4348.9 |###########################
   4351.5 |########################################
   4354.1 |###############################
   4356.8 |#############
   4359.4 |#############
   4362.0 |####################
   4364.7 |##########################
   4367.3 |#
   4369.9 |###
   4372.6 |#
   4375.2 |###
   4377.8 |
   4380.5 |
   4383.1 |
   4385.8 |#
   4388.4 |#
   4391.0 |#
   4393.7 |
  (12 below, 9 above range)

reg_typed_twin (n=160, range 4343.5-4401.2 ns)
   4343.5 |################
   4346.4 |################################
   4349.2 |########################################
   4352.1 |######################################
   4355.0 |####################
   4357.9 |#######################
   4360.8 |#########################
   4363.7 |#####################
   4366.6 |##############
   4369.4 |#######
   4372.3 |#
   4375.2 |#
   4378.1 |#
   4381.0 |
   4383.9 |
   4386.7 |
   4389.6 |
   4392.5 |#
   4395.4 |#
   4398.3 |
  (12 below, 11 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=2868.0% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=145.1% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=642.5% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=2804.6% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=2953.3% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=2952.7% of algo (FFI overhead may distort results)

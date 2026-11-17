# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 20.3x slower than the field

reg_bound_record (88.47 us) is 20.3x the fastest (4.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (reg_typed, reg_typed_twin) are a dead heat (<1%)

reg_typed (4.35 us) and reg_typed_twin (4.36 us) differ by 0.27%, inside the noise, even though the wider field spreads 1934.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (reg_typed)

The baseline reg_typed is the fastest (4.35 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 20.3x the fastest

Fastest reg_typed (4.35 us) to slowest reg_bound_record (88.47 us): 20.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### reg_typed_twin's edge over baseline is significant but tiny (4 ns, 0.09%)

reg_typed_twin differs from baseline reg_typed by 4 ns (0.09%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (reg_typed) is the fastest** at 4349.6 ns median
- 5 variants significantly slower than baseline
- Spread: 20.34x (fastest 4349.6 ns, slowest 88469.4 ns)

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
| reg_bound_columns | 6902ns | 6835ns | 6680ns | 6868ns | 7225ns | +2.45% |
| reg_bound_record | 91317ns | 90926ns | 90639ns | 90987ns | 92986ns | +1255.52% |
| reg_bound_strided | 22538ns | 22451ns | 22124ns | 22474ns | 23145ns | +234.56% |
| reg_bound_units | 6981ns | 6910ns | 6718ns | 6952ns | 7333ns | +3.63% |
| reg_typed | 6737ns | 6665ns | 6510ns | 6708ns | 7050ns | base |
| reg_typed_twin | 6801ns | 6784ns | 6533ns | 6783ns | 7124ns | +0.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 4507ns | 4467ns | 4592ns | +3.15% | 14.540 |
| reg_bound_record | 88920ns | 88390ns | 90600ns | +1934.98% | 0.737 |
| reg_bound_strided | 20126ns | 19912ns | 20723ns | +360.60% | 3.256 |
| reg_bound_units | 4636ns | 4573ns | 4792ns | +6.11% | 14.135 |
| reg_typed | 4370ns | 4339ns | 4457ns | base | 14.998 |
| reg_typed_twin | 4400ns | 4342ns | 4573ns | +0.69% | 14.896 |

## Performance model

- Peak throughput: **15.105 Gops/s** (reg_typed; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 14.565 | 96.4% |
| reg_bound_record | 0.741 | 4.9% |
| reg_bound_strided | 3.278 | 21.7% |
| reg_bound_units | 14.192 | 94.0% |
| reg_typed | 15.067 | 99.7% |
| reg_typed_twin | 15.027 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 6902ns | 6902ns | +2.45% |
| reg_bound_record | 91317ns | 91317ns | +1255.52% |
| reg_bound_strided | 22538ns | 22538ns | +234.56% |
| reg_bound_units | 6981ns | 6981ns | +3.63% |
| reg_typed | 6737ns | 6737ns | base |
| reg_typed_twin | 6801ns | 6801ns | +0.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 4350ns | base | --- | [4348, 4351] | --- | --- | --- | --- |
| reg_bound_columns | 4499ns | +152.1ns (+3.5%) | [+132, +155]ns | [4488, 4502] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_record | 88469ns | +84106.6ns (+1933.7%) | [+84093, +84129]ns | [88451, 88503] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 19994ns | +15634.0ns (+359.4%) | [+15619, +15652]ns | [19978, 20003] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 4618ns | +239.1ns (+5.5%) | [+237, +257]ns | [4584, 4621] | YES | 0.0000 | 0.0000 | 0 |
| reg_typed_twin | 4361ns | +3.8ns (+0.1%) | [+2, +13]ns | [4352, 4367] | YES | 0.0001 | 0.0001 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4371ns | +3.8% | +1923.0% | +357.3% | +6.2% | +0.2% |
| 1 | 2 | 4342ns | +3.6% | +1936.7% | +359.7% | +5.3% | +0.5% |
| 1 | 3 | 4350ns | +3.3% | +1934.0% | +359.2% | +5.7% | +0.0% |
| 1 | 4 | 4351ns | +2.7% | +1932.5% | +359.7% | +6.3% | +0.5% |
| 1 | 5 | 4391ns | +1.8% | +1916.4% | +355.5% | +4.3% | -1.1% |
| 1 | 6 | 4350ns | +3.6% | +1934.8% | +359.7% | +5.3% | -0.0% |
| 1 | 7 | 4341ns | +3.0% | +1937.4% | +362.3% | +5.5% | +0.0% |
| 1 | 8 | 4341ns | +3.7% | +1976.9% | +360.3% | +6.5% | +4.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | 0.022 | ok |
| reg_bound_record | -0.037 | ok |
| reg_bound_strided | -0.002 | ok |
| reg_bound_units | -0.272 | moderate- |
| reg_typed | -0.080 | ok |
| reg_typed_twin | -0.092 | ok |

**Consistency summary:**

- **reg_bound_columns**: won 0/8, lost 8/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 0/8, lost 8/8
- **reg_typed_twin**: won 1/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 128844.9ns | 4507.3ns | 2858.6% | HIGH |
| reg_bound_record | 129015.3ns | 88919.9ns | 145.1% | HIGH |
| reg_bound_strided | 129189.6ns | 20126.4ns | 641.9% | HIGH |
| reg_bound_units | 129037.8ns | 4636.4ns | 2783.1% | HIGH |
| reg_typed | 128857.7ns | 4369.6ns | 2949.0% | HIGH |
| reg_typed_twin | 129302.6ns | 4399.7ns | 2938.9% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 4466.9-4592.4 ns)
   4466.9 |########################################
   4473.1 |#################
   4479.4 |###
   4485.7 |#
   4492.0 |##########
   4498.2 |############################
   4504.5 |############################
   4510.8 |###
   4517.1 |##
   4523.4 |####
   4529.6 |##
   4535.9 |#####
   4542.2 |#
   4548.5 |
   4554.8 |#
   4561.0 |
   4567.3 |#
   4573.6 |
   4579.9 |#
   4586.1 |
  (14 below, 13 above range)

reg_bound_record (n=160, range 88389.8-90599.6 ns)
  88389.8 |########################################
  88500.3 |#########
  88610.8 |###
  88721.3 |###
  88831.7 |#
  88942.2 |
  89052.7 |#
  89163.2 |
  89273.7 |
  89384.2 |#
  89494.7 |
  89605.2 |
  89715.7 |
  89826.2 |#
  89936.7 |#
  90047.2 |
  90157.7 |
  90268.2 |#
  90378.7 |#
  90489.2 |
  (12 below, 17 above range)

reg_bound_strided (n=160, range 19912.2-20722.9 ns)
  19912.2 |######################
  19952.8 |########################################
  19993.3 |############################
  20033.8 |#########
  20074.4 |#####
  20114.9 |
  20155.4 |#
  20196.0 |
  20236.5 |
  20277.1 |#
  20317.6 |
  20358.1 |
  20398.7 |
  20439.2 |
  20479.7 |
  20520.3 |##
  20560.8 |#
  20601.3 |#
  20641.9 |
  20682.4 |####
  (13 below, 11 above range)

reg_bound_units (n=160, range 4572.6-4791.7 ns)
   4572.6 |########################################
   4583.6 |#######
   4594.5 |##
   4605.5 |
   4616.4 |########################
   4627.4 |#####
   4638.3 |######
   4649.3 |###
   4660.2 |
   4671.2 |###
   4682.2 |#
   4693.1 |
   4704.1 |
   4715.0 |
   4726.0 |
   4736.9 |
   4747.9 |
   4758.8 |
   4769.8 |
   4780.8 |
  (16 below, 18 above range)

reg_typed (n=160, range 4338.7-4456.9 ns)
   4338.7 |######################################
   4344.6 |########################################
   4350.5 |########################
   4356.4 |###
   4362.3 |#####
   4368.3 |#####
   4374.2 |#####
   4380.1 |#
   4386.0 |#####
   4391.9 |##
   4397.8 |
   4403.7 |#
   4409.6 |###
   4415.5 |#
   4421.4 |#
   4427.3 |#
   4433.3 |
   4439.2 |
   4445.1 |
   4451.0 |
  (13 below, 12 above range)

reg_typed_twin (n=160, range 4341.6-4573.5 ns)
   4341.6 |########################################
   4353.2 |##########
   4364.7 |#####################
   4376.3 |#######
   4387.9 |####
   4399.5 |#
   4411.1 |
   4422.7 |
   4434.3 |#
   4445.9 |#
   4457.5 |
   4469.1 |
   4480.7 |#
   4492.3 |#
   4503.9 |
   4515.5 |##
   4527.1 |
   4538.7 |
   4550.3 |
   4561.9 |##
  (14 below, 8 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=2852.3% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=145.1% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=641.7% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=2779.1% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=2949.6% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=2944.0% of algo (FFI overhead may distort results)

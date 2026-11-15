# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 20.3x slower than the field

reg_bound_record (88.51 us) is 20.3x the fastest (4.36 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (reg_typed_twin, reg_typed) are a dead heat (<1%)

reg_typed_twin (4.36 us) and reg_typed (4.37 us) differ by 0.25%, inside the noise, even though the wider field spreads 1929.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Wide spread: slowest is 20.3x the fastest

Fastest reg_typed_twin (4.36 us) to slowest reg_bound_record (88.51 us): 20.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: reg_typed_twin** at 4361.0 ns median (-0.2% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 20.30x (fastest 4361.0 ns, slowest 88514.1 ns)

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
| reg_bound_columns | 6891ns | 6841ns | 6641ns | 6861ns | 7231ns | +1.61% |
| reg_bound_record | 92339ns | 91030ns | 90624ns | 91044ns | 97938ns | +1261.65% |
| reg_bound_strided | 22432ns | 22356ns | 22042ns | 22389ns | 22952ns | +230.79% |
| reg_bound_units | 6949ns | 6917ns | 6725ns | 6928ns | 7238ns | +2.48% |
| reg_typed | 6781ns | 6749ns | 6492ns | 6747ns | 7173ns | base |
| reg_typed_twin | 6769ns | 6737ns | 6534ns | 6748ns | 7065ns | -0.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 4531ns | 4471ns | 4662ns | +2.77% | 14.464 |
| reg_bound_record | 89789ns | 88369ns | 94848ns | +1936.54% | 0.730 |
| reg_bound_strided | 20001ns | 19821ns | 20509ns | +353.64% | 3.277 |
| reg_bound_units | 4638ns | 4578ns | 4763ns | +5.19% | 14.132 |
| reg_typed | 4409ns | 4345ns | 4582ns | base | 14.864 |
| reg_typed_twin | 4386ns | 4345ns | 4491ns | -0.51% | 14.941 |

## Performance model

- Peak throughput: **15.084 Gops/s** (reg_typed_twin; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 14.543 | 96.4% |
| reg_bound_record | 0.740 | 4.9% |
| reg_bound_strided | 3.300 | 21.9% |
| reg_bound_units | 14.193 | 94.1% |
| reg_typed | 14.991 | 99.4% |
| reg_typed_twin | 15.028 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 6891ns | 6891ns | +1.61% |
| reg_bound_record | 92339ns | 92339ns | +1261.65% |
| reg_bound_strided | 22432ns | 22432ns | +230.79% |
| reg_bound_units | 6949ns | 6949ns | +2.48% |
| reg_typed | 6781ns | 6781ns | base |
| reg_typed_twin | 6769ns | 6769ns | -0.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 4372ns | base | --- | [4370, 4374] | --- | --- | --- | --- |
| reg_bound_columns | 4506ns | +130.8ns (+3.0%) | [+126, +141]ns | [4502, 4510] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_record | 88514ns | +84141.6ns (+1924.7%) | [+84110, +84175]ns | [88491, 88559] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 19857ns | +15488.5ns (+354.3%) | [+15477, +15511]ns | [19846, 19880] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 4618ns | +237.5ns (+5.4%) | [+221, +254]ns | [4599, 4626] | YES | 0.0000 | 0.0000 | 0 |
| reg_typed_twin | 4361ns | no significant difference | [-15, +1]ns | [4355, 4372] | no | 0.0966 | 0.0966 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4373ns | +2.4% | +1923.1% | +353.9% | +5.9% | -0.4% |
| 1 | 2 | 4368ns | +2.9% | +1925.2% | +354.4% | +5.2% | +0.5% |
| 1 | 3 | 4426ns | +5.3% | +1909.4% | +363.9% | +4.8% | -0.6% |
| 1 | 4 | 4523ns | -0.6% | +1855.7% | +338.8% | +1.7% | -3.2% |
| 1 | 5 | 4349ns | +3.6% | +1936.8% | +356.4% | +6.2% | -0.0% |
| 1 | 6 | 4370ns | +3.2% | +1925.5% | +354.8% | +5.0% | +0.1% |
| 1 | 7 | 4376ns | +3.1% | +1922.9% | +353.3% | +4.6% | -0.7% |
| 1 | 8 | 4348ns | +2.9% | +1934.9% | +356.5% | +8.9% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | -0.187 | ok |
| reg_bound_record | -0.358 | moderate- |
| reg_bound_strided | -0.171 | ok |
| reg_bound_units | -0.246 | moderate- |
| reg_typed | 0.019 | ok |
| reg_typed_twin | 0.234 | moderate+ |

**Consistency summary:**

- **reg_bound_columns**: won 1/8, lost 7/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 0/8, lost 8/8
- **reg_typed_twin**: won 4/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 129317.7ns | 4531.0ns | 2854.1% | HIGH |
| reg_bound_record | 129034.0ns | 89789.4ns | 143.7% | HIGH |
| reg_bound_strided | 129254.0ns | 20000.6ns | 646.3% | HIGH |
| reg_bound_units | 129140.2ns | 4637.5ns | 2784.7% | HIGH |
| reg_typed | 136859.3ns | 4408.9ns | 3104.1% | HIGH |
| reg_typed_twin | 129021.3ns | 4386.4ns | 2941.4% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 4470.6-4662.3 ns)
   4470.6 |########################################
   4480.2 |########
   4489.8 |##########
   4499.3 |###################################
   4508.9 |#####################
   4518.5 |###########
   4528.1 |###
   4537.7 |#####
   4547.3 |##
   4556.9 |
   4566.4 |#
   4576.0 |##
   4585.6 |#
   4595.2 |#
   4604.8 |#
   4614.4 |###
   4624.0 |#
   4633.5 |
   4643.1 |#
   4652.7 |#############
  (15 below, 13 above range)

reg_bound_record (n=160, range 88369.3-94848.0 ns)
  88369.3 |########################################
  88693.3 |###########
  89017.2 |#####
  89341.1 |
  89665.1 |
  89989.0 |
  90312.9 |
  90636.9 |
  90960.8 |
  91284.7 |
  91608.7 |
  91932.6 |
  92256.5 |
  92580.4 |
  92904.4 |
  93228.3 |
  93552.2 |
  93876.2 |
  94200.1 |
  94524.0 |
  (13 below, 4 above range)

reg_bound_strided (n=160, range 19820.7-20508.7 ns)
  19820.7 |########################################
  19855.1 |##########
  19889.5 |#####
  19923.9 |#####
  19958.3 |##
  19992.7 |#
  20027.1 |#
  20061.5 |###
  20095.9 |#
  20130.3 |#
  20164.7 |
  20199.1 |
  20233.5 |
  20267.9 |
  20302.3 |
  20336.7 |
  20371.1 |
  20405.5 |
  20439.9 |
  20474.3 |
  (15 below, 24 above range)

reg_bound_units (n=160, range 4578.3-4763.5 ns)
   4578.3 |######################################
   4587.5 |########################################
   4596.8 |################
   4606.1 |########
   4615.3 |###############
   4624.6 |###############################
   4633.8 |#####################
   4643.1 |#
   4652.4 |###
   4661.6 |#############
   4670.9 |###
   4680.1 |#####
   4689.4 |
   4698.7 |#
   4707.9 |
   4717.2 |#
   4726.4 |#####
   4735.7 |###
   4745.0 |######
   4754.2 |#
  (17 below, 12 above range)

reg_typed (n=160, range 4345.2-4582.1 ns)
   4345.2 |##########################
   4357.0 |###################
   4368.9 |########################################
   4380.7 |##########
   4392.6 |###
   4404.4 |####
   4416.2 |#
   4428.1 |##
   4439.9 |
   4451.8 |
   4463.6 |
   4475.5 |#
   4487.3 |#
   4499.2 |
   4511.0 |
   4522.9 |#######
   4534.7 |
   4546.6 |#
   4558.4 |
   4570.2 |
  (15 below, 7 above range)

reg_typed_twin (n=160, range 4344.6-4491.5 ns)
   4344.6 |########################################
   4352.0 |########################
   4359.3 |#######
   4366.6 |############
   4374.0 |######################
   4381.3 |######
   4388.7 |########
   4396.0 |####
   4403.4 |#
   4410.7 |#
   4418.0 |##
   4425.4 |##
   4432.7 |##
   4440.1 |##
   4447.4 |
   4454.8 |
   4462.1 |#
   4469.5 |
   4476.8 |
   4484.1 |###
  (15 below, 12 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=2849.8% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=145.1% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=646.6% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=2780.9% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=2936.0% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=2943.0% of algo (FFI overhead may distort results)

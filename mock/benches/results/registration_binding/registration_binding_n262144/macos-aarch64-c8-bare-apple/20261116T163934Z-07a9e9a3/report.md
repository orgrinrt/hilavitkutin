# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 21.3x slower than the field

reg_bound_record (352.90 us) is 21.3x the fastest (16.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (reg_typed, reg_typed_twin) are a dead heat (<1%)

reg_typed (16.53 us) and reg_typed_twin (16.62 us) differ by 0.52%, inside the noise, even though the wider field spreads 2034.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### reg_bound_units shows alternating (throttle bounce) (autocorr -0.58)

reg_bound_units's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (reg_typed)

The baseline reg_typed is the fastest (16.53 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 21.3x the fastest

Fastest reg_typed (16.53 us) to slowest reg_bound_record (352.90 us): 21.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (reg_typed) is the fastest** at 16533.8 ns median
- 5 variants significantly slower than baseline
- Spread: 21.34x (fastest 16533.8 ns, slowest 352902.9 ns)

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
| reg_bound_columns | 20179ns | 20190ns | 19796ns | 20176ns | 20572ns | +3.65% |
| reg_bound_record | 356697ns | 356116ns | 355430ns | 356136ns | 359649ns | +1732.12% |
| reg_bound_strided | 82705ns | 82640ns | 82246ns | 82642ns | 83350ns | +324.80% |
| reg_bound_units | 20613ns | 20575ns | 20320ns | 20578ns | 21009ns | +5.87% |
| reg_typed | 19469ns | 19461ns | 19031ns | 19467ns | 19915ns | base |
| reg_typed_twin | 19626ns | 19604ns | 19262ns | 19609ns | 20039ns | +0.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 17228ns | 17142ns | 17417ns | +3.81% | 15.216 |
| reg_bound_record | 353525ns | 352236ns | 356628ns | +2030.28% | 0.742 |
| reg_bound_strided | 79680ns | 79416ns | 80193ns | +380.14% | 3.290 |
| reg_bound_units | 17658ns | 17557ns | 17834ns | +6.40% | 14.846 |
| reg_typed | 16595ns | 16490ns | 16857ns | base | 15.796 |
| reg_typed_twin | 16674ns | 16500ns | 16973ns | +0.48% | 15.722 |

## Performance model

- Peak throughput: **15.897 Gops/s** (reg_typed; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 15.254 | 96.0% |
| reg_bound_record | 0.743 | 4.7% |
| reg_bound_strided | 3.294 | 20.7% |
| reg_bound_units | 14.880 | 93.6% |
| reg_typed | 15.855 | 99.7% |
| reg_typed_twin | 15.773 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 20179ns | 20179ns | +3.65% |
| reg_bound_record | 356697ns | 356697ns | +1732.12% |
| reg_bound_strided | 82705ns | 82705ns | +324.80% |
| reg_bound_units | 20613ns | 20613ns | +5.87% |
| reg_typed | 19469ns | 19469ns | base |
| reg_typed_twin | 19626ns | 19626ns | +0.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 16534ns | base | --- | [16523, 16547] | --- | --- | --- | --- |
| reg_bound_columns | 17185ns | +653.6ns (+4.0%) | [+642, +665]ns | [17176, 17201] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_record | 352903ns | +336333.8ns (+2034.2%) | [+336268, +336403]ns | [352805, 352974] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 79582ns | +63022.5ns (+381.2%) | [+62978, +63050]ns | [79559, 79611] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 17618ns | +1076.6ns (+6.5%) | [+1062, +1091]ns | [17606, 17645] | YES | 0.0000 | 0.0000 | 0 |
| reg_typed_twin | 16620ns | +65.2ns (+0.4%) | [+20, +112]ns | [16583, 16680] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 16593ns | +3.5% | +2027.4% | +379.6% | +6.1% | -0.3% |
| 1 | 2 | 16507ns | +4.0% | +2034.1% | +382.3% | +6.5% | +0.4% |
| 1 | 3 | 16531ns | +4.0% | +2034.9% | +381.0% | +6.7% | +1.1% |
| 1 | 4 | 16559ns | +4.0% | +2031.4% | +380.5% | +6.5% | -0.1% |
| 1 | 5 | 16543ns | +3.9% | +2035.3% | +381.0% | +6.2% | +1.7% |
| 1 | 6 | 16536ns | +4.2% | +2034.3% | +381.1% | +7.0% | +1.0% |
| 1 | 7 | 16499ns | +4.0% | +2039.6% | +382.5% | +6.5% | +0.4% |
| 1 | 8 | 16555ns | +3.6% | +2028.5% | +380.9% | +7.0% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | -0.025 | ok |
| reg_bound_record | -0.147 | ok |
| reg_bound_strided | -0.065 | ok |
| reg_bound_units | -0.577 | HIGH- (thermal bounce) |
| reg_typed | -0.323 | moderate- |
| reg_typed_twin | -0.075 | ok |

**Consistency summary:**

- **reg_bound_columns**: won 0/8, lost 8/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 0/8, lost 8/8
- **reg_typed_twin**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 513929.8ns | 17228.3ns | 2983.1% | HIGH |
| reg_bound_record | 514719.1ns | 353525.3ns | 145.6% | HIGH |
| reg_bound_strided | 514025.4ns | 79679.7ns | 645.1% | HIGH |
| reg_bound_units | 514127.7ns | 17657.6ns | 2911.7% | HIGH |
| reg_typed | 514813.0ns | 16595.2ns | 3102.2% | HIGH |
| reg_typed_twin | 514620.9ns | 16674.2ns | 3086.3% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 17141.5-17416.7 ns)
  17141.5 |###########################
  17155.3 |###################################
  17169.0 |########################################
  17182.8 |#################
  17196.5 |############
  17210.3 |################
  17224.1 |###################
  17237.8 |###########
  17251.6 |######
  17265.4 |####
  17279.1 |####
  17292.9 |#
  17306.6 |######
  17320.4 |####
  17334.2 |#
  17347.9 |###
  17361.7 |
  17375.4 |#
  17389.2 |###
  17403.0 |
  (14 below, 10 above range)

reg_bound_record (n=160, range 352235.8-356627.6 ns)
  352235.8 |##################
  352455.4 |####################
  352675.0 |########################################
  352894.6 |################################
  353114.2 |#################
  353333.8 |###
  353553.3 |########
  353772.9 |#
  353992.5 |###
  354212.1 |#####
  354431.7 |#
  354651.3 |###
  354870.9 |
  355090.5 |
  355310.0 |
  355529.6 |#
  355749.2 |
  355968.8 |
  356188.4 |
  356408.0 |
  (13 below, 13 above range)

reg_bound_strided (n=160, range 79416.5-80192.7 ns)
  79416.5 |######################
  79455.3 |######################################
  79494.1 |####################
  79532.9 |########################################
  79571.7 |################################
  79610.5 |############################
  79649.4 |##############
  79688.2 |##############
  79727.0 |######
  79765.8 |########
  79804.6 |######
  79843.4 |################
  79882.2 |########
  79921.0 |####
  79959.8 |####
  79998.7 |##
  80037.5 |####
  80076.3 |######
  80115.1 |####
  80153.9 |####
  (15 below, 5 above range)

reg_bound_units (n=160, range 17557.5-17834.4 ns)
  17557.5 |########################################
  17571.3 |################################
  17585.2 |#######################
  17599.0 |#####################################
  17612.9 |############################
  17626.7 |##############
  17640.5 |###########
  17654.4 |##############
  17668.2 |##############
  17682.1 |################
  17695.9 |##############################
  17709.8 |##################
  17723.6 |###########
  17737.5 |####
  17751.3 |####
  17765.2 |#######
  17779.0 |##
  17792.9 |##
  17806.7 |
  17820.6 |
  (16 below, 10 above range)

reg_typed (n=160, range 16489.8-16856.8 ns)
  16489.8 |######################################
  16508.2 |########################################
  16526.5 |###########################
  16544.9 |#############
  16563.2 |################
  16581.6 |#########
  16599.9 |######
  16618.3 |#####
  16636.6 |#
  16655.0 |####
  16673.3 |####
  16691.7 |####
  16710.0 |#
  16728.4 |##
  16746.7 |
  16765.1 |##
  16783.4 |##
  16801.8 |
  16820.1 |##
  16838.5 |
  (14 below, 12 above range)

reg_typed_twin (n=160, range 16500.2-16972.5 ns)
  16500.2 |#############################
  16523.8 |###################################
  16547.4 |###############################
  16571.1 |#############################
  16594.7 |########
  16618.3 |########
  16641.9 |####
  16665.5 |##################
  16689.1 |########################################
  16712.8 |################
  16736.4 |######
  16760.0 |##########
  16783.6 |############
  16807.2 |############
  16830.8 |######
  16854.4 |##
  16878.1 |######
  16901.7 |
  16925.3 |########
  16948.9 |
  (15 below, 8 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=2987.7% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=145.6% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=645.3% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=2915.2% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=3106.2% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=3090.1% of algo (FFI overhead may distort results)

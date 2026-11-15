# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 21.3x slower than the field

reg_bound_record (352.85 us) is 21.3x the fastest (16.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (reg_typed_twin, reg_typed) are a dead heat (<1%)

reg_typed_twin (16.55 us) and reg_typed (16.56 us) differ by 0.04%, inside the noise, even though the wider field spreads 2032.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Wide spread: slowest is 21.3x the fastest

Fastest reg_typed_twin (16.55 us) to slowest reg_bound_record (352.85 us): 21.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: reg_typed_twin** at 16549.0 ns median (-0.0% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 21.32x (fastest 16549.0 ns, slowest 352847.9 ns)

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
| reg_bound_columns | 20074ns | 20047ns | 19622ns | 20056ns | 20579ns | +3.10% |
| reg_bound_record | 356856ns | 355841ns | 355036ns | 355879ns | 361604ns | +1732.85% |
| reg_bound_strided | 82594ns | 82496ns | 81953ns | 82494ns | 83535ns | +324.21% |
| reg_bound_units | 20508ns | 20522ns | 19897ns | 20477ns | 21215ns | +5.33% |
| reg_typed | 19470ns | 19486ns | 18977ns | 19461ns | 19991ns | base |
| reg_typed_twin | 19340ns | 19309ns | 18831ns | 19318ns | 19918ns | -0.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 17297ns | 17154ns | 17507ns | +4.28% | 15.156 |
| reg_bound_record | 353859ns | 352039ns | 358728ns | +2033.29% | 0.741 |
| reg_bound_strided | 79760ns | 79324ns | 80735ns | +380.85% | 3.287 |
| reg_bound_units | 17753ns | 17562ns | 18101ns | +7.03% | 14.766 |
| reg_typed | 16587ns | 16488ns | 16762ns | base | 15.804 |
| reg_typed_twin | 16583ns | 16476ns | 16794ns | -0.02% | 15.808 |

## Performance model

- Peak throughput: **15.911 Gops/s** (reg_typed_twin; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 15.188 | 95.5% |
| reg_bound_record | 0.743 | 4.7% |
| reg_bound_strided | 3.294 | 20.7% |
| reg_bound_units | 14.815 | 93.1% |
| reg_typed | 15.834 | 99.5% |
| reg_typed_twin | 15.840 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 20074ns | 20074ns | +3.10% |
| reg_bound_record | 356856ns | 356856ns | +1732.85% |
| reg_bound_strided | 82594ns | 82594ns | +324.21% |
| reg_bound_units | 20508ns | 20508ns | +5.33% |
| reg_typed | 19470ns | 19470ns | base |
| reg_typed_twin | 19340ns | 19340ns | -0.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 16556ns | base | --- | [16545, 16572] | --- | --- | --- | --- |
| reg_bound_columns | 17260ns | +711.2ns (+4.3%) | [+685, +727]ns | [17235, 17301] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_record | 352848ns | +336294.6ns (+2031.3%) | [+336110, +336381]ns | [352711, 352933] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 79579ns | +63011.7ns (+380.6%) | [+62958, +63052]ns | [79542, 79612] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 17694ns | +1131.7ns (+6.8%) | [+1110, +1151]ns | [17678, 17722] | YES | 0.0000 | 0.0000 | 0 |
| reg_typed_twin | 16549ns | no significant difference | [-37, +3]ns | [16537, 16564] | no | 0.0966 | 0.0966 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 16621ns | +4.6% | +2024.0% | +378.8% | +6.3% | -0.2% |
| 1 | 2 | 16579ns | +3.7% | +2028.8% | +379.9% | +6.9% | -0.5% |
| 1 | 3 | 16548ns | +4.3% | +2032.5% | +381.0% | +6.8% | +0.1% |
| 1 | 4 | 16540ns | +5.1% | +2033.5% | +381.4% | +6.9% | -0.0% |
| 1 | 5 | 16548ns | +4.6% | +2032.6% | +381.3% | +6.7% | -0.3% |
| 1 | 6 | 16553ns | +4.1% | +2032.4% | +381.1% | +7.1% | -0.0% |
| 1 | 7 | 16536ns | +4.0% | +2031.7% | +380.7% | +7.6% | +0.0% |
| 1 | 8 | 16560ns | +4.0% | +2028.5% | +379.8% | +6.1% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | -0.019 | ok |
| reg_bound_record | 0.344 | moderate+ |
| reg_bound_strided | 0.398 | moderate+ |
| reg_bound_units | -0.346 | moderate- |
| reg_typed | 0.315 | moderate+ |
| reg_typed_twin | -0.428 | moderate- |

**Consistency summary:**

- **reg_bound_columns**: won 0/8, lost 8/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 0/8, lost 8/8
- **reg_typed_twin**: won 3/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 514286.7ns | 17296.9ns | 2973.3% | HIGH |
| reg_bound_record | 514891.1ns | 353858.8ns | 145.5% | HIGH |
| reg_bound_strided | 514548.6ns | 79760.3ns | 645.1% | HIGH |
| reg_bound_units | 515459.8ns | 17753.4ns | 2903.4% | HIGH |
| reg_typed | 513862.4ns | 16587.5ns | 3097.9% | HIGH |
| reg_typed_twin | 514091.9ns | 16583.4ns | 3100.0% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 17153.6-17506.6 ns)
  17153.6 |#############
  17171.3 |########################
  17188.9 |########################################
  17206.6 |###############################
  17224.2 |####################
  17241.9 |#################
  17259.5 |###############
  17277.2 |########
  17294.8 |########
  17312.5 |#############
  17330.1 |###########
  17347.8 |#################
  17365.4 |#################################
  17383.1 |#############
  17400.7 |#############
  17418.4 |######
  17436.0 |##
  17453.7 |####
  17471.3 |##
  17489.0 |######
  (14 below, 9 above range)

reg_bound_record (n=160, range 352038.7-358728.3 ns)
  352038.7 |############################
  352373.2 |########################################
  352707.7 |########################################
  353042.2 |##############################
  353376.6 |##################
  353711.1 |#
  354045.6 |#####
  354380.1 |
  354714.5 |#
  355049.0 |#
  355383.5 |#
  355718.0 |
  356052.5 |
  356386.9 |
  356721.4 |
  357055.9 |#
  357390.4 |
  357724.8 |#
  358059.3 |##
  358393.8 |
  (15 below, 11 above range)

reg_bound_strided (n=160, range 79324.5-80735.0 ns)
  79324.5 |#################################
  79395.0 |#########################
  79465.5 |###################
  79536.0 |########################################
  79606.6 |#########################
  79677.1 |###################
  79747.6 |###########
  79818.1 |###########
  79888.7 |###
  79959.2 |#
  80029.7 |######
  80100.2 |###
  80170.8 |###
  80241.3 |####
  80311.8 |
  80382.3 |
  80452.9 |
  80523.4 |#
  80593.9 |#
  80664.4 |#
  (16 below, 11 above range)

reg_bound_units (n=160, range 17562.0-18101.3 ns)
  17562.0 |########################
  17589.0 |##################
  17616.0 |######################
  17642.9 |##############################
  17669.9 |########################################
  17696.8 |##########################
  17723.8 |####################################
  17750.8 |############
  17777.7 |##########################
  17804.7 |################
  17831.6 |####
  17858.6 |##
  17885.6 |##
  17912.5 |
  17939.5 |
  17966.5 |##
  17993.4 |##
  18020.4 |
  18047.3 |######
  18074.3 |
  (14 below, 12 above range)

reg_typed (n=160, range 16488.1-16762.1 ns)
  16488.1 |######################
  16501.8 |####################
  16515.5 |###############################
  16529.2 |########################################
  16542.9 |#################################
  16556.6 |##########################
  16570.3 |#################
  16584.0 |##########################
  16597.7 |#################
  16611.4 |####################
  16625.1 |####################
  16638.8 |######
  16652.5 |########
  16666.2 |
  16679.9 |####
  16693.6 |####
  16707.3 |####
  16721.0 |##
  16734.7 |##
  16748.4 |
  (14 below, 7 above range)

reg_typed_twin (n=160, range 16475.9-16794.0 ns)
  16475.9 |###################################
  16491.8 |###################################
  16507.7 |############################
  16523.6 |###################################
  16539.5 |#####################################
  16555.4 |############################
  16571.3 |########################################
  16587.2 |#########
  16603.1 |##################
  16619.0 |#########
  16634.9 |#########
  16650.8 |#######
  16666.7 |#########
  16682.6 |
  16698.5 |##
  16714.5 |
  16730.4 |
  16746.3 |
  16762.2 |##
  16778.1 |##
  (15 below, 13 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=2973.6% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=145.6% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=645.1% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=2902.0% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=3099.9% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=3101.1% of algo (FFI overhead may distort results)

# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 25.9x slower than the field

reg_bound_record (21.88 us) is 25.9x the fastest (845 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (reg_typed, reg_typed_twin) are a dead heat (<1%)

reg_typed (845 ns) and reg_typed_twin (845 ns) differ by 0.05%, inside the noise, even though the wider field spreads 2489.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (reg_typed)

The baseline reg_typed is the fastest (845 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 25.9x the fastest

Fastest reg_typed (845 ns) to slowest reg_bound_record (21.88 us): 25.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### reg_bound_columns's edge over baseline is significant but tiny (2 ns, 0.29%)

reg_bound_columns differs from baseline reg_typed by 2 ns (0.29%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (reg_typed) is the fastest** at 845.0 ns median
- 4 variants significantly slower than baseline
- Spread: 25.89x (fastest 845.0 ns, slowest 21881.1 ns)

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
| reg_bound_columns | 2958ns | 2943ns | 2922ns | 2946ns | 3026ns | +0.01% |
| reg_bound_record | 24073ns | 24025ns | 23956ns | 24028ns | 24326ns | +714.02% |
| reg_bound_strided | 7110ns | 7090ns | 7046ns | 7092ns | 7228ns | +140.41% |
| reg_bound_units | 2980ns | 2960ns | 2938ns | 2964ns | 3069ns | +0.77% |
| reg_typed | 2957ns | 2943ns | 2918ns | 2946ns | 3031ns | base |
| reg_typed_twin | 2971ns | 2949ns | 2921ns | 2953ns | 3072ns | +0.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 851ns | 845ns | 866ns | +0.21% | 19.249 |
| reg_bound_record | 21931ns | 21850ns | 22139ns | +2481.89% | 0.747 |
| reg_bound_strided | 4992ns | 4962ns | 5078ns | +487.73% | 3.282 |
| reg_bound_units | 869ns | 859ns | 895ns | +2.28% | 18.858 |
| reg_typed | 849ns | 842ns | 869ns | base | 19.289 |
| reg_typed_twin | 854ns | 842ns | 885ns | +0.50% | 19.193 |

## Performance model

- Peak throughput: **19.470 Gops/s** (reg_typed; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 19.314 | 99.2% |
| reg_bound_record | 0.749 | 3.8% |
| reg_bound_strided | 3.296 | 16.9% |
| reg_bound_units | 18.987 | 97.5% |
| reg_typed | 19.389 | 99.6% |
| reg_typed_twin | 19.380 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 2958ns | 2958ns | +0.01% |
| reg_bound_record | 24073ns | 24073ns | +714.02% |
| reg_bound_strided | 7110ns | 7110ns | +140.41% |
| reg_bound_units | 2980ns | 2980ns | +0.77% |
| reg_typed | 2957ns | 2957ns | base |
| reg_typed_twin | 2971ns | 2971ns | +0.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 845ns | base | --- | [844, 846] | --- | --- | --- | --- |
| reg_bound_columns | 848ns | +2.5ns (+0.3%) | [+1, +4]ns | [848, 849] | YES | 0.0000 | 0.0000 | 6 |
| reg_bound_record | 21881ns | +21033.3ns (+2489.2%) | [+21029, +21047]ns | [21876, 21894] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 4971ns | +4124.2ns (+488.1%) | [+4122, +4128]ns | [4969, 4972] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 863ns | +17.5ns (+2.1%) | [+17, +19]ns | [862, 863] | YES | 0.0000 | 0.0000 | 0 |
| reg_typed_twin | 845ns | no significant difference | [-1, +1]ns | [845, 846] | no | 0.4684 | 0.4684 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 845ns | +0.4% | +2492.1% | +488.5% | +2.0% | +0.0% |
| 1 | 2 | 846ns | +0.0% | +2485.5% | +487.9% | +2.0% | +1.2% |
| 1 | 3 | 845ns | +0.2% | +2488.4% | +488.6% | +2.0% | +2.5% |
| 1 | 4 | 844ns | +0.8% | +2494.7% | +488.4% | +2.4% | +0.0% |
| 1 | 5 | 843ns | +0.4% | +2495.9% | +489.1% | +2.7% | +0.2% |
| 1 | 6 | 856ns | -0.9% | +2459.3% | +480.8% | +0.9% | -1.3% |
| 1 | 7 | 845ns | +0.6% | +2487.9% | +488.1% | +2.0% | +0.0% |
| 1 | 8 | 844ns | +0.5% | +2494.0% | +489.7% | +2.3% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | -0.282 | moderate- |
| reg_bound_record | -0.189 | ok |
| reg_bound_strided | 0.201 | moderate+ |
| reg_bound_units | 0.115 | ok |
| reg_typed | -0.208 | moderate- |
| reg_typed_twin | 0.174 | ok |

**Consistency summary:**

- **reg_bound_columns**: won 1/8, lost 6/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 0/8, lost 8/8
- **reg_typed_twin**: won 1/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 32262.6ns | 851.2ns | 3790.4% | HIGH |
| reg_bound_record | 32214.9ns | 21930.8ns | 146.9% | HIGH |
| reg_bound_strided | 32258.3ns | 4992.3ns | 646.2% | HIGH |
| reg_bound_units | 32367.8ns | 868.8ns | 3725.6% | HIGH |
| reg_typed | 32283.2ns | 849.4ns | 3800.7% | HIGH |
| reg_typed_twin | 32393.3ns | 853.7ns | 3794.7% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 844.6-866.2 ns)
    844.6 |#############
    845.7 |########################################
    846.8 |######
    847.9 |##############################
    848.9 |####################
    850.0 |#######
    851.1 |#######
    852.2 |###
    853.3 |#####
    854.3 |#
    855.4 |#
    856.5 |##
    857.6 |
    858.7 |#
    859.7 |#
    860.8 |##
    861.9 |#
    863.0 |#####
    864.1 |
    865.1 |
  (14 below, 11 above range)

reg_bound_record (n=160, range 21850.0-22139.4 ns)
  21850.0 |######################################
  21864.5 |########################################
  21879.0 |########################
  21893.4 |#####################
  21907.9 |############
  21922.4 |############
  21936.9 |#####
  21951.3 |######
  21965.8 |##
  21980.3 |####
  21994.7 |#
  22009.2 |
  22023.7 |#
  22038.1 |#
  22052.6 |#
  22067.1 |##
  22081.5 |
  22096.0 |#
  22110.5 |
  22125.0 |
  (15 below, 13 above range)

reg_bound_strided (n=160, range 4962.2-5077.8 ns)
   4962.2 |########################################
   4968.0 |########################
   4973.8 |###############
   4979.6 |###
   4985.3 |#
   4991.1 |
   4996.9 |
   5002.7 |
   5008.5 |###
   5014.2 |##
   5020.0 |##
   5025.8 |#
   5031.6 |#
   5037.4 |
   5043.1 |
   5048.9 |
   5054.7 |##
   5060.5 |##
   5066.2 |
   5072.0 |#
  (10 below, 13 above range)

reg_bound_units (n=160, range 859.5-895.2 ns)
    859.5 |#############################
    861.3 |########################################
    863.0 |####################
    864.8 |############
    866.6 |###
    868.4 |#
    870.2 |
    872.0 |###
    873.8 |
    875.6 |
    877.4 |####
    879.1 |
    880.9 |
    882.7 |
    884.5 |
    886.3 |
    888.1 |
    889.9 |#
    891.7 |###
    893.4 |
  (12 below, 8 above range)

reg_typed (n=160, range 841.5-868.9 ns)
    841.5 |#######################
    842.9 |########################################
    844.3 |##################
    845.6 |########################
    847.0 |#########
    848.4 |######
    849.7 |#
    851.1 |####
    852.5 |###
    853.8 |
    855.2 |
    856.6 |#
    857.9 |###
    859.3 |#
    860.7 |####
    862.1 |#
    863.4 |#
    864.8 |#
    866.2 |#
    867.5 |
  (9 below, 16 above range)

reg_typed_twin (n=160, range 841.7-885.4 ns)
    841.7 |######################################
    843.8 |########################################
    846.0 |###################
    848.2 |#######
    850.4 |#
    852.6 |##
    854.8 |##
    857.0 |####
    859.2 |####
    861.3 |#
    863.5 |###
    865.7 |#
    867.9 |#
    870.1 |##
    872.3 |###
    874.5 |##
    876.7 |##
    878.9 |###
    881.0 |#
    883.2 |#
  (12 below, 12 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=3787.9% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=146.9% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=647.0% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=3726.3% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=3800.3% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=3801.8% of algo (FFI overhead may distort results)

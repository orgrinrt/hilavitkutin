# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 25.9x slower than the field

reg_bound_record (21.87 us) is 25.9x the fastest (844 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (reg_typed_twin, reg_typed) are a dead heat (<1%)

reg_typed_twin (844 ns) and reg_typed (845 ns) differ by 0.02%, inside the noise, even though the wider field spreads 2489.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Wide spread: slowest is 25.9x the fastest

Fastest reg_typed_twin (844 ns) to slowest reg_bound_record (21.87 us): 25.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### reg_bound_columns's edge over baseline is significant but tiny (4 ns, 0.44%)

reg_bound_columns differs from baseline reg_typed by 4 ns (0.44%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: reg_typed_twin** at 844.4 ns median (-0.0% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 25.90x (fastest 844.4 ns, slowest 21868.8 ns)

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
| reg_bound_columns | 2969ns | 2955ns | 2931ns | 2956ns | 3047ns | +0.06% |
| reg_bound_record | 24114ns | 24001ns | 23944ns | 24005ns | 24611ns | +712.64% |
| reg_bound_strided | 7106ns | 7070ns | 7048ns | 7073ns | 7263ns | +139.48% |
| reg_bound_units | 2976ns | 2956ns | 2944ns | 2959ns | 3061ns | +0.30% |
| reg_typed | 2967ns | 2946ns | 2927ns | 2947ns | 3069ns | base |
| reg_typed_twin | 2967ns | 2947ns | 2930ns | 2949ns | 3055ns | -0.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 853ns | 845ns | 872ns | +0.30% | 19.212 |
| reg_bound_record | 21982ns | 21847ns | 22436ns | +2485.33% | 0.745 |
| reg_bound_strided | 4993ns | 4960ns | 5102ns | +487.21% | 3.282 |
| reg_bound_units | 866ns | 859ns | 883ns | +1.82% | 18.924 |
| reg_typed | 850ns | 842ns | 875ns | base | 19.270 |
| reg_typed_twin | 847ns | 842ns | 860ns | -0.37% | 19.341 |

## Performance model

- Peak throughput: **19.463 Gops/s** (reg_typed; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 19.303 | 99.2% |
| reg_bound_record | 0.749 | 3.8% |
| reg_bound_strided | 3.299 | 17.0% |
| reg_bound_units | 19.005 | 97.6% |
| reg_typed | 19.399 | 99.7% |
| reg_typed_twin | 19.403 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 2969ns | 2969ns | +0.06% |
| reg_bound_record | 24114ns | 24114ns | +712.64% |
| reg_bound_strided | 7106ns | 7106ns | +139.48% |
| reg_bound_units | 2976ns | 2976ns | +0.30% |
| reg_typed | 2967ns | 2967ns | base |
| reg_typed_twin | 2967ns | 2967ns | -0.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 845ns | base | --- | [844, 845] | --- | --- | --- | --- |
| reg_bound_columns | 849ns | +3.8ns (+0.4%) | [+3, +5]ns | [848, 849] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_record | 21869ns | +21022.3ns (+2489.0%) | [+21015, +21033]ns | [21863, 21878] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 4966ns | +4120.9ns (+487.9%) | [+4120, +4122]ns | [4965, 4968] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 862ns | +17.5ns (+2.1%) | [+17, +18]ns | [862, 863] | YES | 0.0000 | 0.0000 | 1 |
| reg_typed_twin | 844ns | no significant difference | [-1, +0]ns | [844, 845] | no | 0.0750 | 0.0750 | 7 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 844ns | +0.3% | +2498.4% | +488.1% | +2.0% | +0.0% |
| 1 | 2 | 845ns | +0.6% | +2489.7% | +488.3% | +2.2% | -0.1% |
| 1 | 3 | 844ns | +0.4% | +2492.1% | +488.3% | +2.6% | +0.1% |
| 1 | 4 | 850ns | +0.1% | +2473.6% | +484.2% | +1.5% | -0.6% |
| 1 | 5 | 844ns | +0.4% | +2490.3% | +488.3% | +2.2% | +0.1% |
| 1 | 6 | 846ns | +0.3% | +2485.0% | +487.3% | +2.0% | -0.2% |
| 1 | 7 | 843ns | +4.3% | +2499.7% | +509.4% | +2.3% | +0.2% |
| 1 | 8 | 844ns | +0.6% | +2491.7% | +488.2% | +2.2% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | -0.173 | ok |
| reg_bound_record | -0.048 | ok |
| reg_bound_strided | -0.154 | ok |
| reg_bound_units | -0.130 | ok |
| reg_typed | -0.341 | moderate- |
| reg_typed_twin | -0.106 | ok |

**Consistency summary:**

- **reg_bound_columns**: won 0/8, lost 7/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 0/8, lost 8/8
- **reg_typed_twin**: won 2/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 32290.5ns | 852.8ns | 3786.5% | HIGH |
| reg_bound_record | 32297.8ns | 21981.8ns | 146.9% | HIGH |
| reg_bound_strided | 32272.6ns | 4992.8ns | 646.4% | HIGH |
| reg_bound_units | 32239.6ns | 865.8ns | 3723.9% | HIGH |
| reg_typed | 32287.3ns | 850.3ns | 3797.4% | HIGH |
| reg_typed_twin | 32257.3ns | 847.1ns | 3807.8% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 845.3-871.5 ns)
    845.3 |#######################
    846.6 |########################################
    847.9 |#####################################
    849.2 |####################
    850.5 |###############
    851.8 |#############
    853.2 |###
    854.5 |
    855.8 |
    857.1 |
    858.4 |
    859.7 |
    861.0 |
    862.3 |
    863.6 |
    865.0 |
    866.3 |#
    867.6 |
    868.9 |
    870.2 |
  (11 below, 21 above range)

reg_bound_record (n=160, range 21847.2-22436.1 ns)
  21847.2 |########################################
  21876.6 |##############
  21906.1 |#####
  21935.5 |#
  21965.0 |#
  21994.4 |
  22023.9 |#
  22053.3 |
  22082.8 |
  22112.2 |#
  22141.6 |
  22171.1 |
  22200.5 |
  22230.0 |
  22259.4 |
  22288.9 |
  22318.3 |#
  22347.7 |
  22377.2 |
  22406.6 |
  (16 below, 18 above range)

reg_bound_strided (n=160, range 4960.4-5102.2 ns)
   4960.4 |########################################
   4967.4 |#################
   4974.5 |####
   4981.6 |
   4988.7 |
   4995.8 |#
   5002.9 |#
   5010.0 |
   5017.1 |
   5024.2 |
   5031.3 |#
   5038.4 |
   5045.4 |
   5052.5 |
   5059.6 |
   5066.7 |
   5073.8 |
   5080.9 |
   5088.0 |#
   5095.1 |
  (11 below, 17 above range)

reg_bound_units (n=160, range 859.0-882.8 ns)
    859.0 |##############################
    860.2 |####################################
    861.4 |########################################
    862.6 |#################
    863.8 |#################################
    865.0 |#################
    866.2 |########
    867.3 |##
    868.5 |
    869.7 |#
    870.9 |
    872.1 |
    873.3 |
    874.5 |#
    875.7 |
    876.9 |
    878.1 |
    879.3 |
    880.5 |
    881.6 |
  (11 below, 8 above range)

reg_typed (n=160, range 841.8-875.5 ns)
    841.8 |####################################
    843.5 |########################################
    845.2 |#################################
    846.9 |#######
    848.5 |####
    850.2 |
    851.9 |
    853.6 |
    855.3 |
    857.0 |#
    858.6 |
    860.3 |
    862.0 |
    863.7 |
    865.4 |
    867.1 |
    868.7 |
    870.4 |
    872.1 |#
    873.8 |
  (14 below, 11 above range)

reg_typed_twin (n=160, range 841.9-860.5 ns)
    841.9 |#######################
    842.8 |################################
    843.7 |########################################
    844.7 |####################################
    845.6 |############
    846.5 |############
    847.5 |#####
    848.4 |##
    849.3 |
    850.2 |#
    851.2 |#
    852.1 |#
    853.0 |
    854.0 |#
    854.9 |
    855.8 |
    856.7 |
    857.7 |#
    858.6 |
    859.5 |
  (11 below, 12 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=3786.2% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=147.0% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=647.2% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=3727.4% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=3801.7% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=3800.9% of algo (FFI overhead may distort results)

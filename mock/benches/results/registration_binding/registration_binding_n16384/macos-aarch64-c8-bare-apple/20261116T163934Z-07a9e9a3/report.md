# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 25.9x slower than the field

reg_bound_record (21.90 us) is 25.9x the fastest (845 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (reg_typed, reg_typed_twin) are a dead heat (<1%)

reg_typed (845 ns) and reg_typed_twin (845 ns) differ by 0.00%, inside the noise, even though the wider field spreads 2490.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (reg_typed)

The baseline reg_typed is the fastest (845 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 25.9x the fastest

Fastest reg_typed (845 ns) to slowest reg_bound_record (21.90 us): 25.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### reg_bound_columns's edge over baseline is significant but tiny (5 ns, 0.54%)

reg_bound_columns differs from baseline reg_typed by 5 ns (0.54%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (reg_typed) is the fastest** at 845.4 ns median
- 4 variants significantly slower than baseline
- Spread: 25.90x (fastest 845.4 ns, slowest 21899.4 ns)

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
| reg_bound_columns | 2962ns | 2959ns | 2934ns | 2958ns | 3002ns | +0.09% |
| reg_bound_record | 24087ns | 24043ns | 23980ns | 24052ns | 24300ns | +713.93% |
| reg_bound_strided | 7101ns | 7095ns | 7054ns | 7096ns | 7159ns | +139.93% |
| reg_bound_units | 2980ns | 2976ns | 2949ns | 2976ns | 3025ns | +0.71% |
| reg_typed | 2959ns | 2955ns | 2928ns | 2954ns | 3008ns | base |
| reg_typed_twin | 2961ns | 2957ns | 2930ns | 2956ns | 3007ns | +0.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 851ns | 846ns | 858ns | +0.56% | 19.258 |
| reg_bound_record | 21939ns | 21850ns | 22119ns | +2493.17% | 0.747 |
| reg_bound_strided | 4981ns | 4960ns | 5024ns | +488.77% | 3.289 |
| reg_bound_units | 865ns | 859ns | 874ns | +2.20% | 18.949 |
| reg_typed | 846ns | 842ns | 852ns | base | 19.366 |
| reg_typed_twin | 849ns | 842ns | 866ns | +0.34% | 19.301 |

## Performance model

- Peak throughput: **19.466 Gops/s** (reg_typed; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 19.280 | 99.0% |
| reg_bound_record | 0.748 | 3.8% |
| reg_bound_strided | 3.297 | 16.9% |
| reg_bound_units | 18.987 | 97.5% |
| reg_typed | 19.380 | 99.6% |
| reg_typed_twin | 19.380 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 2962ns | 2962ns | +0.09% |
| reg_bound_record | 24087ns | 24087ns | +713.93% |
| reg_bound_strided | 7101ns | 7101ns | +139.93% |
| reg_bound_units | 2980ns | 2980ns | +0.71% |
| reg_typed | 2959ns | 2959ns | base |
| reg_typed_twin | 2961ns | 2961ns | +0.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 845ns | base | --- | [845, 846] | --- | --- | --- | --- |
| reg_bound_columns | 850ns | +4.6ns (+0.5%) | [+4, +5]ns | [849, 850] | YES | 0.0000 | 0.0000 | 2 |
| reg_bound_record | 21899ns | +21054.0ns (+2490.4%) | [+21037, +21069]ns | [21884, 21914] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 4969ns | +4123.5ns (+487.8%) | [+4121, +4127]ns | [4967, 4972] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 863ns | +17.9ns (+2.1%) | [+17, +19]ns | [862, 864] | YES | 0.0000 | 0.0000 | 0 |
| reg_typed_twin | 845ns | no significant difference | [-0, +1]ns | [845, 846] | no | 0.6255 | 0.6255 | 9 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 846ns | +0.5% | +2487.9% | +488.2% | +2.1% | +0.0% |
| 1 | 2 | 845ns | +0.6% | +2493.4% | +488.4% | +2.3% | +0.0% |
| 1 | 3 | 845ns | +0.7% | +2489.7% | +487.6% | +2.1% | -0.0% |
| 1 | 4 | 845ns | +0.4% | +2489.9% | +488.1% | +2.1% | -0.1% |
| 1 | 5 | 846ns | +0.6% | +2491.8% | +487.9% | +2.0% | +0.2% |
| 1 | 6 | 846ns | +0.3% | +2486.8% | +487.4% | +2.1% | -0.1% |
| 1 | 7 | 844ns | +0.5% | +2496.2% | +488.8% | +2.3% | +0.2% |
| 1 | 8 | 845ns | +0.6% | +2496.6% | +488.3% | +2.1% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | -0.391 | moderate- |
| reg_bound_record | -0.171 | ok |
| reg_bound_strided | 0.070 | ok |
| reg_bound_units | -0.080 | ok |
| reg_typed | -0.167 | ok |
| reg_typed_twin | -0.165 | ok |

**Consistency summary:**

- **reg_bound_columns**: won 0/8, lost 8/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 0/8, lost 8/8
- **reg_typed_twin**: won 0/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 32220.5ns | 850.7ns | 3787.3% | HIGH |
| reg_bound_record | 32230.4ns | 21939.3ns | 146.9% | HIGH |
| reg_bound_strided | 32215.7ns | 4981.2ns | 646.7% | HIGH |
| reg_bound_units | 32214.3ns | 864.6ns | 3725.7% | HIGH |
| reg_typed | 32185.9ns | 846.0ns | 3804.3% | HIGH |
| reg_typed_twin | 32189.9ns | 848.9ns | 3792.0% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 846.3-857.8 ns)
    846.3 |##############
    846.8 |############
    847.4 |##############################
    848.0 |################
    848.6 |##########
    849.2 |################################
    849.7 |#########
    850.3 |########################################
    850.9 |##########
    851.5 |#######
    852.0 |############
    852.6 |###
    853.2 |############
    853.8 |###
    854.4 |#
    854.9 |#########
    855.5 |###
    856.1 |###
    856.7 |###
    857.2 |#####
  (15 below, 10 above range)

reg_bound_record (n=160, range 21850.4-22119.4 ns)
  21850.4 |########################################
  21863.9 |#################################
  21877.3 |###############
  21890.8 |###################
  21904.2 |####################
  21917.7 |#############
  21931.1 |###
  21944.6 |###############
  21958.0 |#################
  21971.5 |######
  21984.9 |#############
  21998.4 |#############
  22011.8 |###
  22025.3 |#####
  22038.7 |######
  22052.2 |#
  22065.6 |#
  22079.1 |#
  22092.5 |#
  22106.0 |
  (19 below, 5 above range)

reg_bound_strided (n=160, range 4960.1-5024.0 ns)
   4960.1 |########################################
   4963.3 |################################
   4966.5 |#############################
   4969.7 |###################
   4972.9 |########
   4976.1 |#####
   4979.3 |##
   4982.5 |####
   4985.7 |#####
   4988.9 |##
   4992.1 |
   4995.3 |########
   4998.5 |###########
   5001.6 |########
   5004.8 |##########
   5008.0 |#
   5011.2 |##
   5014.4 |
   5017.6 |#
   5020.8 |#
  (13 below, 12 above range)

reg_bound_units (n=160, range 859.4-873.9 ns)
    859.4 |########################
    860.1 |######################
    860.8 |###############
    861.5 |##########################
    862.3 |########################################
    863.0 |#################
    863.7 |######################
    864.5 |###################
    865.2 |###############
    865.9 |#
    866.6 |#########
    867.4 |###################
    868.1 |#########
    868.8 |#
    869.6 |#####
    870.3 |###########
    871.0 |###
    871.8 |
    872.5 |#
    873.2 |#
  (14 below, 4 above range)

reg_typed (n=160, range 841.7-852.5 ns)
    841.7 |##############################
    842.2 |###
    842.8 |########################################
    843.3 |####################
    843.9 |###########
    844.4 |#############
    844.9 |##############################
    845.5 |#############
    846.0 |###########
    846.6 |#########
    847.1 |###############
    847.6 |#############
    848.2 |#####
    848.7 |####################
    849.3 |#####
    849.8 |#########
    850.3 |#######
    850.9 |#
    851.4 |###
    852.0 |###
  (10 below, 7 above range)

reg_typed_twin (n=160, range 842.2-865.6 ns)
    842.2 |#########################
    843.3 |########################
    844.5 |########################################
    845.7 |################################
    846.8 |##################
    848.0 |#######
    849.2 |#########
    850.4 |#####
    851.5 |#########
    852.7 |#
    853.9 |#
    855.0 |##
    856.2 |
    857.4 |
    858.6 |
    859.7 |
    860.9 |
    862.1 |
    863.2 |
    864.4 |
  (16 below, 7 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=3784.1% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=146.9% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=647.3% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=3728.0% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=3801.4% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=3800.6% of algo (FFI overhead may distort results)

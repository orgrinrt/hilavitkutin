# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 25.4x slower than the field

reg_bound_record (5.47 us) is 25.4x the fastest (215 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (reg_bound_units, reg_bound_columns) are a dead heat (<1%)

reg_bound_units (215 ns) and reg_bound_columns (218 ns) differ by 0.97%, inside the noise, even though the wider field spreads 2438.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Wide spread: slowest is 25.4x the fastest

Fastest reg_bound_units (215 ns) to slowest reg_bound_record (5.47 us): 25.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: reg_bound_units** at 215.4 ns median (-6.5% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 25.38x (fastest 215.4 ns, slowest 5467.9 ns)

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
| reg_bound_columns | 2308ns | 2302ns | 2293ns | 2303ns | 2341ns | -1.09% |
| reg_bound_record | 7581ns | 7559ns | 7542ns | 7563ns | 7672ns | +224.81% |
| reg_bound_strided | 3383ns | 3343ns | 3327ns | 3363ns | 3499ns | +44.95% |
| reg_bound_units | 2317ns | 2302ns | 2291ns | 2306ns | 2377ns | -0.72% |
| reg_typed | 2334ns | 2317ns | 2308ns | 2319ns | 2406ns | base |
| reg_typed_twin | 2323ns | 2315ns | 2306ns | 2315ns | 2364ns | -0.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 218ns | 215ns | 222ns | -6.22% | 18.795 |
| reg_bound_record | 5489ns | 5464ns | 5560ns | +2261.82% | 0.746 |
| reg_bound_strided | 1270ns | 1248ns | 1319ns | +446.29% | 3.226 |
| reg_bound_units | 216ns | 213ns | 221ns | -6.95% | 18.943 |
| reg_typed | 232ns | 228ns | 242ns | base | 17.626 |
| reg_typed_twin | 231ns | 228ns | 235ns | -0.72% | 17.753 |

## Performance model

- Peak throughput: **19.221 Gops/s** (reg_bound_units; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 18.832 | 98.0% |
| reg_bound_record | 0.749 | 3.9% |
| reg_bound_strided | 3.271 | 17.0% |
| reg_bound_units | 19.016 | 98.9% |
| reg_typed | 17.778 | 92.5% |
| reg_typed_twin | 17.778 | 92.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 2308ns | 2308ns | -1.09% |
| reg_bound_record | 7581ns | 7581ns | +224.81% |
| reg_bound_strided | 3383ns | 3383ns | +44.95% |
| reg_bound_units | 2317ns | 2317ns | -0.72% |
| reg_typed | 2334ns | 2334ns | base |
| reg_typed_twin | 2323ns | 2323ns | -0.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 230ns | base | --- | [230, 231] | --- | --- | --- | --- |
| reg_bound_columns | 218ns | -13.4ns (-5.8%) | [-14, -13]ns | [217, 218] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_record | 5468ns | +5237.9ns (+2273.4%) | [+5237, +5240]ns | [5467, 5471] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 1252ns | +1022.9ns (+444.0%) | [+1021, +1025]ns | [1252, 1255] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 215ns | -15.2ns (-6.6%) | [-16, -14]ns | [215, 216] | YES | 0.0000 | 0.0000 | 0 |
| reg_typed_twin | 230ns | no significant difference | [-1, +0]ns | [230, 231] | no | 0.6300 | 0.6300 | 5 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 230ns | -4.9% | +2272.9% | +443.1% | -6.8% | +0.0% |
| 1 | 2 | 231ns | -5.9% | +2270.6% | +463.1% | -5.8% | -0.4% |
| 1 | 3 | 230ns | -5.5% | +2275.0% | +443.4% | -6.5% | -0.1% |
| 1 | 4 | 230ns | -6.2% | +2273.0% | +443.8% | -6.5% | +0.4% |
| 1 | 5 | 240ns | -9.3% | +2175.3% | +447.8% | -10.3% | -4.5% |
| 1 | 6 | 230ns | -5.6% | +2299.8% | +444.5% | -6.4% | +0.5% |
| 1 | 7 | 230ns | -5.4% | +2279.5% | +444.5% | -6.4% | +0.4% |
| 1 | 8 | 230ns | -5.4% | +2277.1% | +443.5% | -6.2% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | -0.283 | moderate- |
| reg_bound_record | -0.214 | moderate- |
| reg_bound_strided | -0.350 | moderate- |
| reg_bound_units | -0.374 | moderate- |
| reg_typed | -0.178 | ok |
| reg_typed_twin | -0.439 | moderate- |

**Consistency summary:**

- **reg_bound_columns**: won 8/8, lost 0/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 8/8, lost 0/8
- **reg_typed_twin**: won 2/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 8152.3ns | 217.9ns | 3740.7% | HIGH |
| reg_bound_record | 8173.1ns | 5488.6ns | 148.9% | HIGH |
| reg_bound_strided | 8249.5ns | 1269.5ns | 649.8% | HIGH |
| reg_bound_units | 8204.4ns | 216.2ns | 3794.3% | HIGH |
| reg_typed | 8164.9ns | 232.4ns | 3513.5% | HIGH |
| reg_typed_twin | 8138.8ns | 230.7ns | 3527.5% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 215.1-222.2 ns)
    215.1 |#################
    215.4 |
    215.8 |#################
    216.1 |########################################
    216.5 |########################################
    216.8 |###########################
    217.2 |#####################################
    217.6 |###########################
    217.9 |
    218.3 |##############################
    218.6 |####################
    219.0 |######################
    219.3 |#################
    219.7 |#####
    220.0 |
    220.4 |###############
    220.7 |############
    221.1 |##
    221.5 |#####
    221.8 |#####
  (18 below, 5 above range)

reg_bound_record (n=160, range 5464.4-5560.2 ns)
   5464.4 |########################################
   5469.2 |####
   5474.0 |####
   5478.8 |###
   5483.6 |######
   5488.4 |
   5493.2 |
   5498.0 |
   5502.8 |#
   5507.6 |#
   5512.3 |#
   5517.1 |#
   5521.9 |#
   5526.7 |##
   5531.5 |#
   5536.3 |
   5541.1 |
   5545.9 |
   5550.7 |
   5555.4 |
  (12 below, 16 above range)

reg_bound_strided (n=160, range 1248.5-1318.5 ns)
   1248.5 |########################################
   1252.0 |##############
   1255.5 |#####
   1259.0 |#
   1262.5 |
   1266.0 |
   1269.5 |
   1273.0 |#
   1276.5 |
   1280.0 |#
   1283.5 |#
   1287.0 |
   1290.5 |#
   1294.0 |#####
   1297.5 |####
   1301.0 |####
   1304.5 |#
   1308.0 |#
   1311.5 |
   1315.0 |#
  (12 below, 12 above range)

reg_bound_units (n=160, range 213.1-221.3 ns)
    213.1 |###########################
    213.5 |####################
    213.9 |##############################
    214.3 |##############################
    214.7 |########################################
    215.2 |########################################
    215.6 |#########################
    216.0 |#########################
    216.4 |####################
    216.8 |###########################
    217.2 |#################
    217.6 |#######
    218.0 |#####
    218.5 |##
    218.9 |############
    219.3 |##
    219.7 |#######
    220.1 |#####
    220.5 |#######
    220.9 |##
  (12 below, 6 above range)

reg_typed (n=160, range 227.6-242.0 ns)
    227.6 |###############
    228.4 |####################
    229.1 |########################################
    229.8 |######################################
    230.5 |####################################
    231.2 |############
    231.9 |##########
    232.7 |########################
    233.4 |#
    234.1 |#####
    234.8 |######
    235.5 |#
    236.3 |
    237.0 |######
    237.7 |###
    238.4 |###
    239.1 |###
    239.9 |
    240.6 |#####
    241.3 |
  (15 below, 9 above range)

reg_typed_twin (n=160, range 227.6-234.6 ns)
    227.6 |##################
    228.0 |
    228.3 |##################
    228.6 |##################################
    229.0 |#############################
    229.3 |##########################
    229.7 |########################################
    230.0 |
    230.4 |##########################
    230.7 |################################
    231.1 |################
    231.4 |#############################
    231.8 |##################
    232.1 |
    232.5 |##########################
    232.8 |########
    233.2 |########
    233.5 |################
    233.9 |##########
    234.2 |
  (13 below, 12 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=3738.4% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=149.0% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=651.3% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=3783.6% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=3516.4% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=3517.2% of algo (FFI overhead may distort results)

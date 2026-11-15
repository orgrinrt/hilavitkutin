# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 25.4x slower than the field

reg_bound_record (5.51 us) is 25.4x the fastest (217 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 25.4x the fastest

Fastest reg_bound_units (217 ns) to slowest reg_bound_record (5.51 us): 25.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### reg_typed_twin's edge over baseline is significant but tiny (-3 ns, 1.40%)

reg_typed_twin differs from baseline reg_typed by -3 ns (1.40%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: reg_bound_units** at 217.1 ns median (-8.1% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 25.38x (fastest 217.1 ns, slowest 5509.1 ns)

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
| reg_bound_columns | 2357ns | 2346ns | 2300ns | 2349ns | 2440ns | -1.28% |
| reg_bound_record | 7677ns | 7610ns | 7548ns | 7627ns | 7957ns | +221.47% |
| reg_bound_strided | 3405ns | 3390ns | 3332ns | 3386ns | 3536ns | +42.58% |
| reg_bound_units | 2341ns | 2329ns | 2291ns | 2330ns | 2423ns | -1.99% |
| reg_typed | 2388ns | 2374ns | 2321ns | 2377ns | 2488ns | base |
| reg_typed_twin | 2355ns | 2350ns | 2306ns | 2346ns | 2430ns | -1.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 224ns | 216ns | 236ns | -5.69% | 18.315 |
| reg_bound_record | 5561ns | 5465ns | 5775ns | +2244.93% | 0.737 |
| reg_bound_strided | 1279ns | 1250ns | 1337ns | +439.41% | 3.202 |
| reg_bound_units | 219ns | 213ns | 228ns | -7.77% | 18.728 |
| reg_typed | 237ns | 230ns | 247ns | base | 17.272 |
| reg_typed_twin | 234ns | 228ns | 241ns | -1.39% | 17.516 |

## Performance model

- Peak throughput: **19.215 Gops/s** (reg_bound_units; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 18.442 | 96.0% |
| reg_bound_record | 0.743 | 3.9% |
| reg_bound_strided | 3.220 | 16.8% |
| reg_bound_units | 18.867 | 98.2% |
| reg_typed | 17.341 | 90.3% |
| reg_typed_twin | 17.572 | 91.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 2357ns | 2357ns | -1.28% |
| reg_bound_record | 7677ns | 7677ns | +221.47% |
| reg_bound_strided | 3405ns | 3405ns | +42.58% |
| reg_bound_units | 2341ns | 2341ns | -1.99% |
| reg_typed | 2388ns | 2388ns | base |
| reg_typed_twin | 2355ns | 2355ns | -1.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 236ns | base | --- | [235, 237] | --- | --- | --- | --- |
| reg_bound_columns | 222ns | -14.6ns (-6.2%) | [-15, -13]ns | [221, 223] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_record | 5509ns | +5274.0ns (+2232.9%) | [+5250, +5311]ns | [5483, 5546] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 1272ns | +1035.4ns (+438.4%) | [+1022, +1043]ns | [1256, 1276] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 217ns | -18.8ns (-7.9%) | [-20, -17]ns | [217, 218] | YES | 0.0000 | 0.0000 | 1 |
| reg_typed_twin | 233ns | -3.3ns (-1.4%) | [-4, -2]ns | [232, 234] | YES | 0.0000 | 0.0000 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 237ns | -5.8% | +2222.8% | +428.7% | -6.1% | -0.6% |
| 1 | 2 | 235ns | -4.3% | +2231.9% | +444.9% | -6.7% | -0.7% |
| 1 | 3 | 238ns | -6.7% | +2210.3% | +426.7% | -9.5% | -0.7% |
| 1 | 4 | 236ns | -4.0% | +2212.1% | +460.3% | -5.6% | -2.4% |
| 1 | 5 | 238ns | -8.2% | +2232.3% | +436.6% | -8.7% | -2.8% |
| 1 | 6 | 231ns | -4.1% | +2277.4% | +452.7% | -4.6% | +2.5% |
| 1 | 7 | 235ns | -6.7% | +2338.8% | +444.0% | -8.6% | -1.6% |
| 1 | 8 | 238ns | -6.1% | +2245.7% | +428.4% | -9.6% | -1.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | -0.295 | moderate- |
| reg_bound_record | 0.083 | ok |
| reg_bound_strided | -0.363 | moderate- |
| reg_bound_units | -0.216 | moderate- |
| reg_typed | -0.170 | ok |
| reg_typed_twin | -0.417 | moderate- |

**Consistency summary:**

- **reg_bound_columns**: won 8/8, lost 0/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 8/8, lost 0/8
- **reg_typed_twin**: won 7/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 8339.2ns | 223.6ns | 3728.8% | HIGH |
| reg_bound_record | 8264.5ns | 5560.8ns | 148.6% | HIGH |
| reg_bound_strided | 8309.6ns | 1279.2ns | 649.6% | HIGH |
| reg_bound_units | 8293.6ns | 218.7ns | 3792.0% | HIGH |
| reg_typed | 8352.0ns | 237.1ns | 3522.0% | HIGH |
| reg_typed_twin | 8243.4ns | 233.8ns | 3525.1% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 216.4-235.9 ns)
    216.4 |################
    217.4 |########################################
    218.4 |############
    219.3 |################
    220.3 |###################################
    221.3 |##################
    222.3 |#####################
    223.2 |#############################
    224.2 |###########################
    225.2 |################
    226.1 |##############
    227.1 |########
    228.1 |##
    229.1 |##########
    230.0 |####
    231.0 |####
    232.0 |############
    233.0 |##
    233.9 |
    234.9 |##
  (15 below, 4 above range)

reg_bound_record (n=160, range 5464.8-5774.7 ns)
   5464.8 |########################################
   5480.3 |###########
   5495.8 |####
   5511.2 |#
   5526.7 |######
   5542.2 |#####
   5557.7 |##############
   5573.2 |####
   5588.7 |###
   5604.2 |####
   5619.7 |
   5635.2 |
   5650.7 |##
   5666.2 |
   5681.7 |####
   5697.2 |####
   5712.7 |
   5728.2 |
   5743.7 |
   5759.2 |##
  (19 below, 14 above range)

reg_bound_strided (n=160, range 1249.7-1336.7 ns)
   1249.7 |########################################
   1254.1 |################
   1258.4 |#
   1262.8 |
   1267.1 |#
   1271.5 |############
   1275.8 |###########
   1280.2 |##
   1284.5 |
   1288.9 |######
   1293.2 |##########
   1297.6 |#####
   1301.9 |#####
   1306.3 |##
   1310.6 |
   1315.0 |
   1319.3 |
   1323.7 |
   1328.0 |
   1332.4 |###
  (15 below, 11 above range)

reg_bound_units (n=160, range 213.2-227.7 ns)
    213.2 |#########################
    213.9 |########################################
    214.6 |#########################
    215.3 |##############################
    216.1 |########################################
    216.8 |###################################
    217.5 |##########
    218.2 |############
    219.0 |###############
    219.7 |#################
    220.4 |##########
    221.1 |####################
    221.9 |#################
    222.6 |############
    223.3 |#####
    224.0 |#######
    224.8 |#####
    225.5 |##
    226.2 |
    226.9 |#######
  (13 below, 12 above range)

reg_typed (n=160, range 230.1-247.4 ns)
    230.1 |#####################
    231.0 |##############
    231.8 |#####################
    232.7 |################
    233.6 |###################################
    234.4 |###########
    235.3 |#######################
    236.1 |########################################
    237.0 |#########################
    237.9 |############################
    238.7 |#####################
    239.6 |
    240.5 |####
    241.3 |##
    242.2 |##
    243.1 |
    243.9 |####
    244.8 |##############
    245.7 |################
    246.5 |##############
  (15 below, 10 above range)

reg_typed_twin (n=160, range 228.4-241.3 ns)
    228.4 |##########
    229.1 |########################################
    229.7 |##########################
    230.4 |################################
    231.0 |##########
    231.7 |#############################
    232.3 |#############################
    232.9 |#############
    233.6 |########################
    234.2 |##################
    234.9 |##########################
    235.5 |##################
    236.2 |##########################
    236.8 |#####
    237.5 |#############
    238.1 |##################
    238.7 |#############
    239.4 |#####
    240.0 |##
    240.7 |
  (13 below, 10 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=3738.2% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=148.4% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=651.5% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=3791.4% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=3507.7% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=3536.9% of algo (FFI overhead may distort results)

# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 7.5x slower than the field

reg_bound_record (89.13 us) is 7.5x the fastest (11.95 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (reg_typed_twin, reg_typed) are a dead heat (<1%)

reg_typed_twin (11.95 us) and reg_typed (11.96 us) differ by 0.10%, inside the noise, even though the wider field spreads 645.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### reg_bound_columns shows warm-up / thermal drift (autocorr +0.64)

reg_bound_columns's per-pass series has lag-1 autocorrelation +0.64, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {reg_typed_twin, reg_typed, reg_bound_units, reg_bound_columns, reg_bound_strided} vs {reg_bound_record} (303% apart)

The field splits into a fast tier {reg_typed_twin, reg_typed, reg_bound_units, reg_bound_columns, reg_bound_strided} and a slow tier {reg_bound_record} with a 303% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.5x the fastest

Fastest reg_typed_twin (11.95 us) to slowest reg_bound_record (89.13 us): 7.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: reg_typed_twin** at 11950.3 ns median (-0.1% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 7.46x (fastest 11950.3 ns, slowest 89130.2 ns)

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
| Counter frequency | 2112866659 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| reg_bound_columns | 14194ns | 14277ns | 13935ns | 14170ns | 14522ns | +1.00% |
| reg_bound_record | 91551ns | 91078ns | 90661ns | 91088ns | 93830ns | +551.49% |
| reg_bound_strided | 23602ns | 24044ns | 20224ns | 24027ns | 25704ns | +67.95% |
| reg_bound_units | 13995ns | 14194ns | 12641ns | 14268ns | 14533ns | -0.41% |
| reg_typed | 14053ns | 13894ns | 13823ns | 14033ns | 14339ns | base |
| reg_typed_twin | 13991ns | 13886ns | 13819ns | 13935ns | 14332ns | -0.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 12252ns | 12005ns | 12550ns | +1.11% | 5.349 |
| reg_bound_record | 89589ns | 88725ns | 91803ns | +639.34% | 0.732 |
| reg_bound_strided | 21736ns | 18628ns | 23736ns | +79.38% | 3.015 |
| reg_bound_units | 12094ns | 10922ns | 12584ns | -0.19% | 5.419 |
| reg_typed | 12117ns | 11890ns | 12398ns | base | 5.408 |
| reg_typed_twin | 12056ns | 11889ns | 12392ns | -0.51% | 5.436 |

## Performance model

- Peak throughput: **6.001 Gops/s** (reg_bound_units; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 5.307 | 88.4% |
| reg_bound_record | 0.735 | 12.3% |
| reg_bound_strided | 2.966 | 49.4% |
| reg_bound_units | 5.367 | 89.4% |
| reg_typed | 5.479 | 91.3% |
| reg_typed_twin | 5.484 | 91.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 14194ns | 14194ns | +1.00% |
| reg_bound_record | 91551ns | 91551ns | +551.49% |
| reg_bound_strided | 23602ns | 23602ns | +67.95% |
| reg_bound_units | 13995ns | 13995ns | -0.41% |
| reg_typed | 14053ns | 14053ns | base |
| reg_typed_twin | 13991ns | 13991ns | -0.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 11962ns | base | --- | [11948, 12063] | --- | --- | --- | --- |
| reg_bound_columns | 12350ns | +70.4ns (+0.6%) | [+46, +87]ns | [12204, 12354] | YES | 0.0008 | 0.0006 | 0 |
| reg_bound_record | 89130ns | +77074.9ns (+644.3%) | [+77002, +77135]ns | [89095, 89197] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 22095ns | +10119.7ns (+84.6%) | [+10099, +10154]ns | [22060, 22127] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 12212ns | +153.5ns (+1.3%) | [+138, +168]ns | [12194, 12537] | YES | 0.0000 | 0.0000 | 0 |
| reg_typed_twin | 11950ns | no significant difference | [-14, +3]ns | [11944, 11958] | no | 0.2356 | 0.2356 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 11961ns | +3.3% | +643.0% | +97.8% | +4.8% | -0.4% |
| 1 | 2 | 12082ns | +2.3% | +642.1% | +83.8% | -0.1% | -1.1% |
| 1 | 3 | 12387ns | -0.2% | +622.3% | +45.6% | +1.3% | -0.0% |
| 1 | 4 | 12377ns | -0.1% | +620.4% | +56.3% | -1.5% | -3.6% |
| 1 | 5 | 11891ns | +1.1% | +650.6% | +85.5% | -13.7% | +0.4% |
| 1 | 6 | 11942ns | +0.5% | +646.1% | +98.1% | +0.8% | +0.2% |
| 1 | 7 | 11934ns | +0.7% | +644.8% | +84.7% | +5.2% | +0.0% |
| 1 | 8 | 12388ns | -2.7% | +615.6% | +78.6% | +1.4% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | 0.637 | HIGH+ (drift/warm-up) |
| reg_bound_record | 0.225 | moderate+ |
| reg_bound_strided | 0.303 | moderate+ |
| reg_bound_units | 0.031 | ok |
| reg_typed | 0.085 | ok |
| reg_typed_twin | -0.218 | moderate- |

**Consistency summary:**

- **reg_bound_columns**: won 3/8, lost 5/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 3/8, lost 5/8
- **reg_typed_twin**: won 3/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 156195.3ns | 12252.1ns | 1274.8% | HIGH |
| reg_bound_record | 156866.1ns | 89588.6ns | 175.1% | HIGH |
| reg_bound_strided | 149185.4ns | 21736.1ns | 686.3% | HIGH |
| reg_bound_units | 152740.2ns | 12094.2ns | 1262.9% | HIGH |
| reg_typed | 156218.4ns | 12117.3ns | 1289.2% | HIGH |
| reg_typed_twin | 156192.4ns | 12055.7ns | 1295.6% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 12004.7-12550.3 ns)
  12004.7 |####################################
  12032.0 |#######
  12059.3 |###
  12086.6 |
  12113.9 |
  12141.1 |
  12168.4 |
  12195.7 |
  12223.0 |
  12250.2 |
  12277.5 |
  12304.8 |#
  12332.1 |########################################
  12359.3 |####################################
  12386.6 |######
  12413.9 |
  12441.2 |
  12468.5 |
  12495.7 |
  12523.0 |
  (15 below, 1 above range)

reg_bound_record (n=160, range 88724.8-91802.9 ns)
  88724.8 |##################
  88878.7 |##############################
  89032.6 |########################################
  89186.5 |############################
  89340.4 |####################
  89494.3 |#################
  89648.2 |##########
  89802.1 |##
  89956.0 |##
  90110.0 |
  90263.9 |#
  90417.8 |
  90571.7 |
  90725.6 |
  90879.5 |
  91033.4 |
  91187.3 |
  91341.2 |
  91495.1 |
  91649.0 |
  (20 below, 3 above range)

reg_bound_strided (n=160, range 18628.5-23735.8 ns)
  18628.5 |
  18883.8 |
  19139.2 |############
  19394.5 |
  19649.9 |
  19905.3 |
  20160.6 |
  20416.0 |
  20671.4 |
  20926.7 |
  21182.1 |
  21437.5 |
  21692.8 |
  21948.2 |########################################
  22203.6 |##
  22458.9 |
  22714.3 |
  22969.7 |
  23225.0 |
  23480.4 |#####################
  (17 below, 10 above range)

reg_bound_units (n=160, range 10921.6-12584.2 ns)
  10921.6 |
  11004.7 |
  11087.9 |
  11171.0 |
  11254.1 |
  11337.3 |
  11420.4 |
  11503.5 |
  11586.6 |
  11669.8 |
  11752.9 |
  11836.0 |
  11919.2 |#
  12002.3 |###################
  12085.4 |#
  12168.5 |##########
  12251.7 |
  12334.8 |
  12417.9 |
  12501.1 |########################################
  (20 below, 1 above range)

reg_typed (n=160, range 11890.3-12398.4 ns)
  11890.3 |################
  11915.7 |#################
  11941.1 |#######################
  11966.5 |########
  11991.9 |
  12017.3 |
  12042.7 |
  12068.2 |
  12093.6 |
  12119.0 |
  12144.4 |
  12169.8 |
  12195.2 |
  12220.6 |
  12246.0 |
  12271.4 |
  12296.8 |
  12322.2 |
  12347.6 |#######
  12373.0 |########################################
  (16 below, 10 above range)

reg_typed_twin (n=160, range 11888.6-12392.3 ns)
  11888.6 |###################
  11913.8 |##################
  11938.9 |########################################
  11964.1 |###########
  11989.3 |
  12014.5 |
  12039.7 |
  12064.9 |
  12090.1 |
  12115.3 |
  12140.4 |
  12165.6 |
  12190.8 |
  12216.0 |
  12241.2 |
  12266.4 |
  12291.6 |
  12316.8 |
  12341.9 |###
  12367.1 |#########################
  (17 below, 11 above range)

```

## Diagnostics

- **reg_bound_columns**: autocorrelation=0.64 (measurement drift or warm-up artifact)
- **reg_bound_columns**: bridge=1264.5% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=175.3% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=706.8% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=1279.3% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=1305.4% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=1306.7% of algo (FFI overhead may distort results)

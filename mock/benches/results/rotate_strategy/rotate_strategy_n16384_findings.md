# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_variable_amount is an outlier: 3.1x slower than the field

rot_variable_amount (3.96 us) is 3.1x the fastest (1.28 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (rot_manual_shifts, rot_intrinsic_const) are a dead heat (<1%)

rot_manual_shifts (1.28 us) and rot_intrinsic_const (1.28 us) differ by 0.13%, inside the noise, even though the wider field spreads 209.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Wide spread: slowest is 3.1x the fastest

Fastest rot_manual_shifts (1.28 us) to slowest rot_variable_amount (3.96 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: rot_manual_shifts** at 1280.8 ns median (-0.1% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 3.09x (fastest 1280.8 ns, slowest 3959.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Rotate strategy: intrinsic const vs manual shifts vs variable amount |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x71f692ab4e08d325 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rot_intrinsic_const | 3426ns | 3405ns | 3331ns | 3406ns | 3583ns | base |
| rot_manual_shifts | 3423ns | 3401ns | 3335ns | 3405ns | 3565ns | -0.09% |
| rot_variable_amount | 6123ns | 6114ns | 5900ns | 6116ns | 6365ns | +78.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 1288ns | 1250ns | 1346ns | base | 12.721 |
| rot_manual_shifts | 1286ns | 1251ns | 1340ns | -0.12% | 12.737 |
| rot_variable_amount | 3968ns | 3823ns | 4125ns | +208.09% | 4.129 |

## Performance model

- Peak throughput: **13.109 Gops/s** (rot_intrinsic_const; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 12.775 | 97.5% |
| rot_manual_shifts | 12.792 | 97.6% |
| rot_variable_amount | 4.138 | 31.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 3426ns | 3426ns | base |
| rot_manual_shifts | 3423ns | 3423ns | -0.09% |
| rot_variable_amount | 6123ns | 6123ns | +78.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 1282ns | base | --- | [1265, 1293] | --- | --- | --- | --- |
| rot_manual_shifts | 1281ns | no significant difference | [-2, +11]ns | [1277, 1285] | no | 0.1124 | 0.1124 | 1 |
| rot_variable_amount | 3960ns | +2688.6ns (+209.6%) | [+2594, +2719]ns | [3914, 4017] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 1254ns | +1.8% | +220.3% |
| 1 | 2 | 1330ns | -3.5% | +205.0% |
| 1 | 3 | 1340ns | -5.8% | +206.3% |
| 1 | 4 | 1295ns | -0.1% | +216.7% |
| 1 | 5 | 1267ns | +2.2% | +202.0% |
| 1 | 6 | 1296ns | +3.6% | +195.2% |
| 1 | 7 | 1264ns | -0.7% | +203.2% |
| 1 | 8 | 1254ns | -0.0% | +227.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | 0.183 | ok |
| rot_manual_shifts | -0.029 | ok |
| rot_variable_amount | 0.309 | moderate+ |

**Consistency summary:**

- **rot_manual_shifts**: won 3/8, lost 3/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 3.4ns | 1288.0ns | 0.3% |  |
| rot_manual_shifts | 3.1ns | 1286.4ns | 0.2% |  |
| rot_variable_amount | 3.2ns | 3968.1ns | 0.1% |  |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 1249.9-1345.8 ns)
   1249.9 |########################################
   1254.7 |##############################
   1259.5 |#########
   1264.3 |###
   1269.1 |###
   1273.9 |###
   1278.7 |############
   1283.4 |####
   1288.2 |##############
   1293.0 |########################
   1297.8 |############
   1302.6 |####
   1307.4 |####
   1312.2 |
   1317.0 |#
   1321.8 |#
   1326.6 |
   1331.4 |
   1336.2 |#################
   1341.0 |######################
  (17 below, 11 above range)

rot_manual_shifts (n=160, range 1251.1-1339.6 ns)
   1251.1 |########################################
   1255.5 |#########
   1259.9 |###########
   1264.3 |#
   1268.8 |########
   1273.2 |######################
   1277.6 |####################
   1282.0 |################
   1286.5 |###
   1290.9 |##############
   1295.3 |####################
   1299.7 |##############
   1304.2 |#
   1308.6 |####
   1313.0 |#
   1317.4 |
   1321.9 |
   1326.3 |
   1330.7 |
   1335.1 |######
  (14 below, 23 above range)

rot_variable_amount (n=160, range 3823.2-4124.9 ns)
   3823.2 |####################################
   3838.3 |#####
   3853.4 |
   3868.5 |
   3883.6 |
   3898.7 |
   3913.7 |###
   3928.8 |
   3943.9 |#####
   3959.0 |########
   3974.1 |
   3989.2 |
   4004.2 |####
   4019.3 |
   4034.4 |
   4049.5 |
   4064.6 |##
   4079.7 |
   4094.8 |########################################
   4109.8 |#######
  (13 below, 2 above range)

```

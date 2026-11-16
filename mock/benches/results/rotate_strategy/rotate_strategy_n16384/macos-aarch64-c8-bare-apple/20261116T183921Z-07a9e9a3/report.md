# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_variable_amount is an outlier: 3.1x slower than the field

rot_variable_amount (3.90 us) is 3.1x the fastest (1.28 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (rot_intrinsic_const, rot_manual_shifts) are a dead heat (<1%)

rot_intrinsic_const (1.28 us) and rot_manual_shifts (1.28 us) differ by 0.08%, inside the noise, even though the wider field spreads 205.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (rot_intrinsic_const)

The baseline rot_intrinsic_const is the fastest (1.28 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.1x the fastest

Fastest rot_intrinsic_const (1.28 us) to slowest rot_variable_amount (3.90 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (rot_intrinsic_const) is the fastest** at 1276.7 ns median
- 1 variant significantly slower than baseline
- Spread: 3.05x (fastest 1276.7 ns, slowest 3900.0 ns)

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
| rot_intrinsic_const | 3410ns | 3405ns | 3401ns | 3406ns | 3431ns | base |
| rot_manual_shifts | 3422ns | 3408ns | 3401ns | 3411ns | 3473ns | +0.35% |
| rot_variable_amount | 6038ns | 6025ns | 6017ns | 6026ns | 6095ns | +77.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 1279ns | 1272ns | 1291ns | base | 12.813 |
| rot_manual_shifts | 1283ns | 1272ns | 1310ns | +0.37% | 12.765 |
| rot_variable_amount | 3907ns | 3895ns | 3940ns | +205.55% | 4.193 |

## Performance model

- Peak throughput: **12.880 Gops/s** (rot_manual_shifts; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 12.833 | 99.6% |
| rot_manual_shifts | 12.823 | 99.6% |
| rot_variable_amount | 4.201 | 32.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 3410ns | 3410ns | base |
| rot_manual_shifts | 3422ns | 3422ns | +0.35% |
| rot_variable_amount | 6038ns | 6038ns | +77.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 1277ns | base | --- | [1276, 1278] | --- | --- | --- | --- |
| rot_manual_shifts | 1278ns | no significant difference | [+0, +2]ns | [1276, 1280] | no | 0.1427 | 0.1427 | 9 |
| rot_variable_amount | 3900ns | +2622.9ns (+205.4%) | [+2622, +2623]ns | [3899, 3901] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 1279ns | +1.1% | +204.9% |
| 1 | 2 | 1277ns | +0.0% | +205.5% |
| 1 | 3 | 1276ns | -0.0% | +205.5% |
| 1 | 4 | 1278ns | -0.1% | +205.3% |
| 1 | 5 | 1276ns | +0.0% | +205.8% |
| 1 | 6 | 1277ns | +0.3% | +205.3% |
| 1 | 7 | 1274ns | +0.4% | +206.0% |
| 1 | 8 | 1276ns | +0.1% | +205.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | -0.112 | ok |
| rot_manual_shifts | -0.028 | ok |
| rot_variable_amount | -0.296 | moderate- |

**Consistency summary:**

- **rot_manual_shifts**: won 1/8, lost 4/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 3.3ns | 1278.7ns | 0.3% |  |
| rot_manual_shifts | 3.4ns | 1283.5ns | 0.3% |  |
| rot_variable_amount | 3.4ns | 3907.1ns | 0.1% |  |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 1272.3-1291.0 ns)
   1272.3 |#############
   1273.2 |##############################
   1274.2 |########################################
   1275.1 |###########
   1276.0 |##########################
   1277.0 |##########################
   1277.9 |#####################
   1278.8 |#####
   1279.8 |###########
   1280.7 |#####
   1281.6 |###########
   1282.6 |###########
   1283.5 |###
   1284.4 |##########
   1285.4 |###
   1286.3 |
   1287.3 |
   1288.2 |
   1289.1 |#
   1290.1 |
  (11 below, 9 above range)

rot_manual_shifts (n=160, range 1272.1-1309.5 ns)
   1272.1 |##########################
   1274.0 |########################################
   1275.8 |##########################
   1277.7 |###########
   1279.6 |#######################
   1281.4 |######################
   1283.3 |#####
   1285.2 |###########
   1287.1 |#
   1288.9 |####
   1290.8 |#
   1292.7 |#
   1294.5 |######
   1296.4 |
   1298.3 |##
   1300.2 |#
   1302.0 |####
   1303.9 |#
   1305.8 |#
   1307.6 |
  (13 below, 8 above range)

rot_variable_amount (n=160, range 3894.7-3939.8 ns)
   3894.7 |######################################
   3896.9 |######################################
   3899.2 |########################################
   3901.4 |#################
   3903.7 |############
   3906.0 |############
   3908.2 |####
   3910.5 |####
   3912.7 |####
   3915.0 |#
   3917.2 |#
   3919.5 |
   3921.8 |
   3924.0 |#
   3926.3 |
   3928.5 |##
   3930.8 |#
   3933.0 |
   3935.3 |##
   3937.5 |
  (17 below, 14 above range)

```

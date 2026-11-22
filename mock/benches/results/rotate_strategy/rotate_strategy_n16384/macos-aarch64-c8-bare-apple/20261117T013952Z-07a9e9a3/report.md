# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_variable_amount is an outlier: 3.1x slower than the field

rot_variable_amount (3.83 us) is 3.1x the fastest (1.25 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (rot_manual_shifts, rot_intrinsic_const) are a dead heat (<1%)

rot_manual_shifts (1.25 us) and rot_intrinsic_const (1.26 us) differ by 0.20%, inside the noise, even though the wider field spreads 205.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### rot_variable_amount shows alternating (throttle bounce) (autocorr -0.61)

rot_variable_amount's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.1x the fastest

Fastest rot_manual_shifts (1.25 us) to slowest rot_variable_amount (3.83 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: rot_manual_shifts** at 1254.6 ns median (-0.2% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 3.05x (fastest 1254.6 ns, slowest 3832.5 ns)

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
| rot_intrinsic_const | 3402ns | 3350ns | 3337ns | 3374ns | 3550ns | base |
| rot_manual_shifts | 3373ns | 3345ns | 3338ns | 3355ns | 3462ns | -0.84% |
| rot_variable_amount | 6037ns | 5923ns | 5906ns | 5997ns | 6290ns | +77.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 1278ns | 1249ns | 1355ns | base | 12.817 |
| rot_manual_shifts | 1263ns | 1248ns | 1297ns | -1.16% | 12.968 |
| rot_variable_amount | 3907ns | 3822ns | 4101ns | +205.63% | 4.194 |

## Performance model

- Peak throughput: **13.129 Gops/s** (rot_manual_shifts; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 13.033 | 99.3% |
| rot_manual_shifts | 13.059 | 99.5% |
| rot_variable_amount | 4.275 | 32.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 3402ns | 3402ns | base |
| rot_manual_shifts | 3373ns | 3373ns | -0.84% |
| rot_variable_amount | 6037ns | 6037ns | +77.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 1257ns | base | --- | [1255, 1261] | --- | --- | --- | --- |
| rot_manual_shifts | 1255ns | no significant difference | [-3, +0]ns | [1253, 1257] | no (adj: YES) | 0.0224 | 0.0224 | 9 |
| rot_variable_amount | 3832ns | +2575.2ns (+204.9%) | [+2571, +2611]ns | [3830, 3870] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 1263ns | -0.6% | +207.3% |
| 1 | 2 | 1267ns | -1.2% | +201.9% |
| 1 | 3 | 1252ns | -0.0% | +222.9% |
| 1 | 4 | 1254ns | +3.4% | +205.2% |
| 1 | 5 | 1251ns | +0.2% | +220.0% |
| 1 | 6 | 1325ns | -3.8% | +188.7% |
| 1 | 7 | 1252ns | -0.1% | +210.2% |
| 1 | 8 | 1296ns | -3.3% | +195.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | -0.371 | moderate- |
| rot_manual_shifts | -0.331 | moderate- |
| rot_variable_amount | -0.614 | HIGH- (thermal bounce) |

**Consistency summary:**

- **rot_manual_shifts**: won 4/8, lost 2/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 3.3ns | 1278.3ns | 0.3% |  |
| rot_manual_shifts | 3.4ns | 1263.4ns | 0.3% |  |
| rot_variable_amount | 3.3ns | 3906.8ns | 0.1% |  |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 1248.7-1355.3 ns)
   1248.7 |########################################
   1254.0 |######################
   1259.3 |###########
   1264.6 |#####
   1270.0 |#####
   1275.3 |#####
   1280.6 |####
   1286.0 |#
   1291.3 |
   1296.6 |
   1302.0 |
   1307.3 |#
   1312.6 |##
   1317.9 |##
   1323.3 |######
   1328.6 |###
   1333.9 |
   1339.3 |
   1344.6 |
   1349.9 |
  (15 below, 13 above range)

rot_manual_shifts (n=160, range 1247.9-1296.9 ns)
   1247.9 |######################################
   1250.3 |########################################
   1252.8 |#################################
   1255.2 |###############
   1257.7 |#############
   1260.2 |#########
   1262.6 |#######
   1265.1 |#
   1267.5 |#
   1270.0 |####
   1272.4 |#########
   1274.9 |#######
   1277.3 |######
   1279.8 |###
   1282.2 |#
   1284.7 |
   1287.1 |#
   1289.6 |###
   1292.0 |####
   1294.5 |#######
  (11 below, 12 above range)

rot_variable_amount (n=160, range 3821.6-4100.8 ns)
   3821.6 |########################################
   3835.6 |
   3849.5 |##
   3863.5 |####
   3877.5 |###
   3891.4 |##
   3905.4 |
   3919.3 |
   3933.3 |#
   3947.3 |#
   3961.2 |#
   3975.2 |#
   3989.1 |##
   4003.1 |#
   4017.1 |
   4031.0 |###
   4045.0 |##
   4058.9 |
   4072.9 |#
   4086.9 |##
  (11 below, 11 above range)

```

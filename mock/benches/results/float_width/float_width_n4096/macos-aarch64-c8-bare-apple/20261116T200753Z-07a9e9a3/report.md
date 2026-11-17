# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (float_f32)

The baseline float_f32 is the fastest (1.21 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (float_f32) is the fastest** at 1208.8 ns median
- 2 variants significantly slower than baseline
- Spread: 1.34x (fastest 1208.8 ns, slowest 1622.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot) |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x2e519d3bc768a4f0 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| float_f32 | 3333ns | 3322ns | 3315ns | 3322ns | 3382ns | base |
| float_f64 | 3369ns | 3349ns | 3344ns | 3351ns | 3449ns | +1.08% |
| float_i64_scaled | 3726ns | 3705ns | 3698ns | 3708ns | 3808ns | +11.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 1212ns | 1204ns | 1230ns | base | 3.379 |
| float_f64 | 1279ns | 1266ns | 1311ns | +5.47% | 3.204 |
| float_i64_scaled | 1631ns | 1617ns | 1668ns | +34.55% | 2.511 |

## Performance model

- Peak throughput: **3.401 Gops/s** (float_f32; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 3.388 | 99.6% |
| float_f64 | 3.221 | 94.7% |
| float_i64_scaled | 2.524 | 74.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 3333ns | 3333ns | base |
| float_f64 | 3369ns | 3369ns | +1.08% |
| float_i64_scaled | 3726ns | 3726ns | +11.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 1209ns | base | --- | [1208, 1209] | --- | --- | --- | --- |
| float_f64 | 1272ns | +62.9ns (+5.2%) | [+62, +63]ns | [1271, 1272] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 1622ns | +413.8ns (+34.2%) | [+412, +415]ns | [1622, 1624] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 1208ns | +5.2% | +34.2% |
| 1 | 2 | 1211ns | +4.9% | +38.3% |
| 1 | 3 | 1209ns | +5.1% | +34.2% |
| 1 | 4 | 1209ns | +5.2% | +34.3% |
| 1 | 5 | 1208ns | +5.2% | +36.0% |
| 1 | 6 | 1209ns | +5.1% | +34.1% |
| 1 | 7 | 1208ns | +5.2% | +34.3% |
| 1 | 8 | 1209ns | +8.8% | +34.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | -0.326 | moderate- |
| float_f64 | -0.024 | ok |
| float_i64_scaled | -0.302 | moderate- |

**Consistency summary:**

- **float_f64**: won 0/8, lost 8/8
- **float_i64_scaled**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 2.5ns | 1212.2ns | 0.2% |  |
| float_f64 | 2.3ns | 1278.5ns | 0.2% |  |
| float_i64_scaled | 2.2ns | 1631.1ns | 0.1% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 1204.4-1230.5 ns)
   1204.4 |##############
   1205.7 |#####################
   1207.0 |#############################
   1208.3 |########################################
   1209.6 |#########################
   1210.9 |##################
   1212.2 |######
   1213.5 |#
   1214.8 |
   1216.1 |
   1217.4 |
   1218.7 |##
   1220.0 |
   1221.3 |
   1222.6 |
   1223.9 |
   1225.2 |
   1226.6 |#
   1227.9 |#
   1229.2 |##
  (14 below, 11 above range)

float_f64 (n=160, range 1266.5-1310.8 ns)
   1266.5 |######################
   1268.7 |########################################
   1270.9 |##########################
   1273.1 |#####################
   1275.3 |######
   1277.5 |#
   1279.8 |#
   1282.0 |
   1284.2 |#
   1286.4 |###
   1288.6 |#
   1290.8 |
   1293.1 |##
   1295.3 |
   1297.5 |#
   1299.7 |
   1301.9 |
   1304.1 |
   1306.3 |#
   1308.6 |#
  (13 below, 20 above range)

float_i64_scaled (n=160, range 1617.2-1668.4 ns)
   1617.2 |#############################
   1619.7 |########################################
   1622.3 |##################################
   1624.8 |############
   1627.4 |###
   1630.0 |###
   1632.5 |##
   1635.1 |##
   1637.6 |#
   1640.2 |
   1642.8 |#
   1645.3 |####
   1647.9 |###
   1650.4 |###
   1653.0 |####
   1655.6 |#
   1658.1 |#
   1660.7 |
   1663.2 |
   1665.8 |
  (17 below, 13 above range)

```

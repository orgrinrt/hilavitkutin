# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (float_f32)

The baseline float_f32 is the fastest (1.28 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (float_f32) is the fastest** at 1278.5 ns median
- 2 variants significantly slower than baseline
- Spread: 1.36x (fastest 1278.5 ns, slowest 1736.9 ns)

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
| float_f32 | 3476ns | 3501ns | 3309ns | 3472ns | 3656ns | base |
| float_f64 | 3521ns | 3571ns | 3338ns | 3509ns | 3740ns | +1.29% |
| float_i64_scaled | 3942ns | 3956ns | 3717ns | 3929ns | 4203ns | +13.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 1268ns | 1206ns | 1333ns | base | 3.230 |
| float_f64 | 1341ns | 1269ns | 1430ns | +5.74% | 3.055 |
| float_i64_scaled | 1730ns | 1629ns | 1847ns | +36.42% | 2.368 |

## Performance model

- Peak throughput: **3.395 Gops/s** (float_f32; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 3.204 | 94.4% |
| float_f64 | 3.014 | 88.8% |
| float_i64_scaled | 2.358 | 69.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 3476ns | 3476ns | base |
| float_f64 | 3521ns | 3521ns | +1.29% |
| float_i64_scaled | 3942ns | 3942ns | +13.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 1279ns | base | --- | [1258, 1293] | --- | --- | --- | --- |
| float_f64 | 1359ns | +65.0ns (+5.1%) | [+63, +69]ns | [1328, 1362] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 1737ns | +445.7ns (+34.9%) | [+442, +467]ns | [1735, 1738] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 1212ns | +5.0% | +43.5% |
| 1 | 2 | 1209ns | +5.7% | +43.8% |
| 1 | 3 | 1252ns | +1.9% | +38.8% |
| 1 | 4 | 1298ns | +4.9% | +33.8% |
| 1 | 5 | 1296ns | +5.5% | +34.0% |
| 1 | 6 | 1295ns | -1.7% | +34.4% |
| 1 | 7 | 1258ns | +8.7% | +38.1% |
| 1 | 8 | 1211ns | +12.6% | +38.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | 0.496 | moderate+ |
| float_f64 | 0.103 | ok |
| float_i64_scaled | 0.003 | ok |

**Consistency summary:**

- **float_f64**: won 1/8, lost 7/8
- **float_i64_scaled**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 2.4ns | 1267.9ns | 0.2% |  |
| float_f64 | 2.4ns | 1340.7ns | 0.2% |  |
| float_i64_scaled | 2.4ns | 1729.7ns | 0.1% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 1206.4-1333.4 ns)
   1206.4 |#################################
   1212.7 |###
   1219.1 |#
   1225.4 |###
   1231.8 |####
   1238.1 |##
   1244.5 |##
   1250.8 |#########
   1257.2 |#######
   1263.5 |######
   1269.9 |
   1276.3 |###
   1282.6 |#
   1289.0 |################################
   1295.3 |########################################
   1301.7 |###
   1308.0 |##
   1314.4 |#
   1320.7 |
   1327.1 |
  (16 below, 12 above range)

float_f64 (n=160, range 1269.4-1430.3 ns)
   1269.4 |################################
   1277.4 |####
   1285.5 |
   1293.5 |#
   1301.6 |##
   1309.6 |####
   1317.7 |###
   1325.7 |
   1333.7 |
   1341.8 |
   1349.8 |#####
   1357.9 |########################################
   1365.9 |################
   1374.0 |#
   1382.0 |
   1390.0 |#
   1398.1 |
   1406.1 |
   1414.2 |
   1422.2 |
  (12 below, 16 above range)

float_i64_scaled (n=160, range 1629.5-1847.2 ns)
   1629.5 |###
   1640.4 |#####
   1651.3 |######
   1662.1 |##
   1673.0 |###########
   1683.9 |####
   1694.8 |###
   1705.7 |
   1716.6 |
   1727.5 |########################################
   1738.3 |###################################
   1749.2 |##
   1760.1 |##
   1771.0 |
   1781.9 |#
   1792.8 |#
   1803.6 |
   1814.5 |#
   1825.4 |#
   1836.3 |#
  (21 below, 22 above range)

```

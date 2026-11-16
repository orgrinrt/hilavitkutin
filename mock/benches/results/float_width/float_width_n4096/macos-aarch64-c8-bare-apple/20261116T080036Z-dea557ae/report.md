# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (float_f32)

The baseline float_f32 is the fastest (1.23 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (float_f32) is the fastest** at 1233.8 ns median
- 2 variants significantly slower than baseline
- Spread: 1.34x (fastest 1233.8 ns, slowest 1654.8 ns)

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
| float_f32 | 3397ns | 3375ns | 3366ns | 3379ns | 3484ns | base |
| float_f64 | 3417ns | 3405ns | 3395ns | 3406ns | 3472ns | +0.58% |
| float_i64_scaled | 3795ns | 3771ns | 3756ns | 3777ns | 3888ns | +11.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 1239ns | 1228ns | 1265ns | base | 3.306 |
| float_f64 | 1299ns | 1291ns | 1318ns | +4.88% | 3.152 |
| float_i64_scaled | 1667ns | 1649ns | 1716ns | +34.54% | 2.457 |

## Performance model

- Peak throughput: **3.336 Gops/s** (float_f32; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 3.320 | 99.5% |
| float_f64 | 3.160 | 94.7% |
| float_i64_scaled | 2.475 | 74.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 3397ns | 3397ns | base |
| float_f64 | 3417ns | 3417ns | +0.58% |
| float_i64_scaled | 3795ns | 3795ns | +11.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 1234ns | base | --- | [1232, 1235] | --- | --- | --- | --- |
| float_f64 | 1296ns | +62.5ns (+5.1%) | [+61, +64]ns | [1295, 1297] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 1655ns | +421.3ns (+34.1%) | [+420, +422]ns | [1654, 1656] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 1235ns | +4.9% | +33.9% |
| 1 | 2 | 1231ns | +5.4% | +34.3% |
| 1 | 3 | 1232ns | +5.3% | +34.6% |
| 1 | 4 | 1232ns | +5.1% | +34.4% |
| 1 | 5 | 1238ns | +4.7% | +33.6% |
| 1 | 6 | 1240ns | +4.6% | +33.1% |
| 1 | 7 | 1234ns | +5.0% | +34.2% |
| 1 | 8 | 1234ns | +5.1% | +34.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | 0.305 | moderate+ |
| float_f64 | -0.236 | moderate- |
| float_i64_scaled | 0.051 | ok |

**Consistency summary:**

- **float_f64**: won 0/8, lost 8/8
- **float_i64_scaled**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 2.6ns | 1238.9ns | 0.2% |  |
| float_f64 | 2.6ns | 1299.5ns | 0.2% |  |
| float_i64_scaled | 2.3ns | 1666.9ns | 0.1% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 1227.8-1265.4 ns)
   1227.8 |###################
   1229.7 |########################################
   1231.6 |###################################
   1233.5 |##################################
   1235.4 |#########################
   1237.2 |########
   1239.1 |#####
   1241.0 |####
   1242.9 |####
   1244.8 |
   1246.6 |#
   1248.5 |#
   1250.4 |####
   1252.3 |####
   1254.1 |##
   1256.0 |
   1257.9 |#
   1259.8 |
   1261.7 |##
   1263.5 |
  (14 below, 13 above range)

float_f64 (n=160, range 1290.6-1318.0 ns)
   1290.6 |################
   1292.0 |###############################
   1293.3 |##############################
   1294.7 |#####################
   1296.1 |#################################
   1297.4 |########################################
   1298.8 |###########
   1300.2 |##########
   1301.6 |#############
   1302.9 |#####
   1304.3 |#
   1305.7 |###
   1307.0 |#####
   1308.4 |#
   1309.8 |#
   1311.1 |
   1312.5 |#
   1313.9 |
   1315.2 |
   1316.6 |
  (15 below, 8 above range)

float_i64_scaled (n=160, range 1649.0-1716.2 ns)
   1649.0 |########################################
   1652.4 |###############################
   1655.7 |##############
   1659.1 |########
   1662.4 |########
   1665.8 |##
   1669.2 |###
   1672.5 |####
   1675.9 |####
   1679.2 |#
   1682.6 |###
   1685.9 |
   1689.3 |##
   1692.7 |
   1696.0 |
   1699.4 |
   1702.7 |
   1706.1 |
   1709.4 |
   1712.8 |
  (13 below, 11 above range)

```

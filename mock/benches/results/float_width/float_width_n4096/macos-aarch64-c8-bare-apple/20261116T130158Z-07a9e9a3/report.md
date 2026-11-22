# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### float_i64_scaled shows alternating (throttle bounce) (autocorr -0.55)

float_i64_scaled's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (float_f32)

The baseline float_f32 is the fastest (1.21 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (float_f32) is the fastest** at 1208.3 ns median
- 2 variants significantly slower than baseline
- Spread: 1.34x (fastest 1208.3 ns, slowest 1621.2 ns)

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
| float_f32 | 3322ns | 3320ns | 3314ns | 3320ns | 3338ns | base |
| float_f64 | 3365ns | 3347ns | 3343ns | 3347ns | 3440ns | +1.28% |
| float_i64_scaled | 3708ns | 3703ns | 3697ns | 3703ns | 3736ns | +11.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 1209ns | 1205ns | 1213ns | base | 3.389 |
| float_f64 | 1279ns | 1267ns | 1315ns | +5.79% | 3.203 |
| float_i64_scaled | 1624ns | 1617ns | 1637ns | +34.32% | 2.523 |

## Performance model

- Peak throughput: **3.399 Gops/s** (float_f32; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 3.390 | 99.7% |
| float_f64 | 3.223 | 94.8% |
| float_i64_scaled | 2.527 | 74.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 3322ns | 3322ns | base |
| float_f64 | 3365ns | 3365ns | +1.28% |
| float_i64_scaled | 3708ns | 3708ns | +11.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 1208ns | base | --- | [1208, 1209] | --- | --- | --- | --- |
| float_f64 | 1271ns | +62.1ns (+5.1%) | [+62, +63]ns | [1270, 1271] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 1621ns | +413.1ns (+34.2%) | [+412, +414]ns | [1621, 1622] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 1209ns | +5.1% | +34.2% |
| 1 | 2 | 1209ns | +5.2% | +34.0% |
| 1 | 3 | 1208ns | +5.1% | +34.2% |
| 1 | 4 | 1209ns | +5.1% | +34.3% |
| 1 | 5 | 1209ns | +5.2% | +34.1% |
| 1 | 6 | 1208ns | +5.2% | +34.4% |
| 1 | 7 | 1208ns | +5.2% | +34.2% |
| 1 | 8 | 1208ns | +5.3% | +34.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | 0.300 | moderate+ |
| float_f64 | -0.168 | ok |
| float_i64_scaled | -0.549 | HIGH- (thermal bounce) |

**Consistency summary:**

- **float_f64**: won 0/8, lost 8/8
- **float_i64_scaled**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 2.4ns | 1208.7ns | 0.2% |  |
| float_f64 | 2.4ns | 1278.8ns | 0.2% |  |
| float_i64_scaled | 2.2ns | 1623.6ns | 0.1% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 1205.1-1213.4 ns)
   1205.1 |#############
   1205.5 |#############################
   1205.9 |########################
   1206.4 |##################################
   1206.8 |##################
   1207.2 |#####################
   1207.6 |################
   1208.0 |########################################
   1208.4 |##########################
   1208.9 |########################
   1209.3 |#####################
   1209.7 |################
   1210.1 |########################
   1210.5 |##################
   1210.9 |################
   1211.4 |##########
   1211.8 |
   1212.2 |########
   1212.6 |##
   1213.0 |##
  (11 below, 11 above range)

float_f64 (n=160, range 1266.7-1314.6 ns)
   1266.7 |##################################
   1269.1 |########################################
   1271.4 |###########################
   1273.8 |#######################
   1276.2 |#
   1278.6 |
   1281.0 |
   1283.4 |
   1285.8 |#
   1288.2 |
   1290.6 |
   1293.0 |
   1295.4 |
   1297.8 |
   1300.2 |
   1302.6 |
   1305.0 |
   1307.4 |
   1309.8 |
   1312.2 |
  (10 below, 6 above range)

float_i64_scaled (n=160, range 1616.7-1637.3 ns)
   1616.7 |##############################
   1617.7 |################
   1618.7 |#############################
   1619.8 |######################################
   1620.8 |#########################
   1621.8 |##############
   1622.9 |########################################
   1623.9 |#####################
   1624.9 |################
   1626.0 |#####
   1627.0 |###
   1628.0 |#####
   1629.1 |
   1630.1 |###
   1631.1 |###
   1632.2 |
   1633.2 |
   1634.2 |###
   1635.2 |
   1636.3 |#
  (12 below, 5 above range)

```

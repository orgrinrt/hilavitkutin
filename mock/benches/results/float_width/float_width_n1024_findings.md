# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### float_f32 dominates: 27% faster than the next best (float_f64)

float_f32 (262 ns) leads float_f64 (332 ns) by 27%, a clear separation rather than a photo finish. CV 6.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### float_f64 shows alternating (throttle bounce) (autocorr -0.70)

float_f64's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (float_f32)

The baseline float_f32 is the fastest (262 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (float_f32) is the fastest** at 262.1 ns median
- 2 variants significantly slower than baseline
- Spread: 1.83x (fastest 262.1 ns, slowest 479.4 ns)

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
| float_f32 | 2502ns | 2514ns | 2350ns | 2474ns | 2737ns | base |
| float_f64 | 2569ns | 2547ns | 2377ns | 2568ns | 2761ns | +2.68% |
| float_i64_scaled | 2741ns | 2814ns | 2516ns | 2768ns | 2885ns | +9.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 264ns | 247ns | 290ns | base | 3.883 |
| float_f64 | 336ns | 309ns | 363ns | +27.26% | 3.051 |
| float_i64_scaled | 466ns | 426ns | 493ns | +76.57% | 2.199 |

## Performance model

- Peak throughput: **4.152 Gops/s** (float_f32; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 3.907 | 94.1% |
| float_f64 | 3.085 | 74.3% |
| float_i64_scaled | 2.136 | 51.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 2502ns | 2502ns | base |
| float_f64 | 2569ns | 2569ns | +2.68% |
| float_i64_scaled | 2741ns | 2741ns | +9.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 262ns | base | --- | [253, 265] | --- | --- | --- | --- |
| float_f64 | 332ns | +71.0ns (+27.1%) | [+66, +75]ns | [328, 338] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 479ns | +200.8ns (+76.6%) | [+198, +204]ns | [457, 485] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 286ns | +12.7% | +52.5% |
| 1 | 2 | 265ns | +30.2% | +83.0% |
| 1 | 3 | 269ns | +33.6% | +76.3% |
| 1 | 4 | 286ns | +8.8% | +70.6% |
| 1 | 5 | 250ns | +43.1% | +94.7% |
| 1 | 6 | 250ns | +28.3% | +93.8% |
| 1 | 7 | 251ns | +42.3% | +69.9% |
| 1 | 8 | 249ns | +25.4% | +95.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | 0.217 | moderate+ |
| float_f64 | -0.696 | HIGH- (thermal bounce) |
| float_i64_scaled | -0.296 | moderate- |

**Consistency summary:**

- **float_f64**: won 0/8, lost 8/8
- **float_i64_scaled**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 2.6ns | 263.7ns | 1.0% |  |
| float_f64 | 3.1ns | 335.6ns | 0.9% |  |
| float_i64_scaled | 2.6ns | 465.6ns | 0.6% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 246.6-289.8 ns)
    246.6 |##################################
    248.8 |########################################
    251.0 |#################
    253.1 |######
    255.3 |########
    257.4 |###
    259.6 |#
    261.7 |############
    263.9 |######################
    266.1 |#################
    268.2 |###############
    270.4 |###
    272.5 |
    274.7 |
    276.8 |#
    279.0 |
    281.2 |
    283.3 |########
    285.5 |######################
    287.6 |###########################
  (14 below, 5 above range)

float_f64 (n=160, range 308.9-363.0 ns)
    308.9 |########################
    311.6 |##########################
    314.3 |##########
    317.0 |####
    319.7 |##############
    322.4 |##############
    325.2 |##############
    327.9 |##################
    330.6 |############
    333.3 |############
    336.0 |######
    338.7 |####
    341.4 |####
    344.1 |##
    346.8 |##
    349.5 |####
    352.2 |############
    354.9 |######################
    357.6 |########################################
    360.3 |##########################
  (16 below, 9 above range)

float_i64_scaled (n=160, range 426.5-492.7 ns)
    426.5 |###
    429.8 |###
    433.1 |###
    436.4 |###
    439.7 |####
    443.0 |####
    446.3 |###############
    449.6 |######
    452.9 |############
    456.2 |##########
    459.6 |#
    462.9 |#
    466.2 |
    469.5 |
    472.8 |#
    476.1 |#
    479.4 |###
    482.7 |################
    486.0 |########################################
    489.3 |#########################
  (19 below, 9 above range)

```

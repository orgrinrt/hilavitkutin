# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### float_f32 dominates: 25% faster than the next best (float_f64)

float_f32 (250 ns) leads float_f64 (312 ns) by 25%, a clear separation rather than a photo finish. CV 4.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (float_f32)

The baseline float_f32 is the fastest (250 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (float_f32) is the fastest** at 250.0 ns median
- 2 variants significantly slower than baseline
- Spread: 1.69x (fastest 250.0 ns, slowest 422.9 ns)

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
| float_f32 | 2388ns | 2363ns | 2357ns | 2367ns | 2483ns | base |
| float_f64 | 2406ns | 2391ns | 2383ns | 2399ns | 2451ns | +0.77% |
| float_i64_scaled | 2504ns | 2496ns | 2489ns | 2496ns | 2541ns | +4.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 252ns | 246ns | 264ns | base | 4.062 |
| float_f64 | 312ns | 307ns | 318ns | +23.91% | 3.278 |
| float_i64_scaled | 423ns | 418ns | 429ns | +67.79% | 2.421 |

## Performance model

- Peak throughput: **4.167 Gops/s** (float_f32; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 4.096 | 98.3% |
| float_f64 | 3.281 | 78.7% |
| float_i64_scaled | 2.421 | 58.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 2388ns | 2388ns | base |
| float_f64 | 2406ns | 2406ns | +0.77% |
| float_i64_scaled | 2504ns | 2504ns | +4.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 250ns | base | --- | [249, 250] | --- | --- | --- | --- |
| float_f64 | 312ns | +61.7ns (+24.7%) | [+61, +63]ns | [311, 313] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 423ns | +172.9ns (+69.2%) | [+172, +173]ns | [422, 423] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 250ns | +24.2% | +69.3% |
| 1 | 2 | 249ns | +26.0% | +70.1% |
| 1 | 3 | 249ns | +26.6% | +69.8% |
| 1 | 4 | 249ns | +24.9% | +70.0% |
| 1 | 5 | 250ns | +26.6% | +70.1% |
| 1 | 6 | 262ns | +18.6% | +61.6% |
| 1 | 7 | 249ns | +25.0% | +69.6% |
| 1 | 8 | 249ns | +25.2% | +69.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | -0.116 | ok |
| float_f64 | -0.392 | moderate- |
| float_i64_scaled | -0.038 | ok |

**Consistency summary:**

- **float_f64**: won 0/8, lost 8/8
- **float_i64_scaled**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 2.5ns | 252.1ns | 1.0% |  |
| float_f64 | 2.9ns | 312.4ns | 0.9% |  |
| float_i64_scaled | 2.3ns | 423.0ns | 0.5% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 245.7-264.5 ns)
    245.7 |#################
    246.7 |###################
    247.6 |#################################
    248.5 |##################################
    249.5 |########################################
    250.4 |#############
    251.3 |##########################
    252.3 |##########
    253.2 |######
    254.2 |######
    255.1 |######
    256.0 |###
    257.0 |#
    257.9 |#####
    258.9 |###
    259.8 |#####
    260.7 |#
    261.7 |#
    262.6 |
    263.5 |#
  (12 below, 10 above range)

float_f64 (n=160, range 306.7-318.2 ns)
    306.7 |#########################
    307.3 |#######
    307.8 |#################
    308.4 |#######
    309.0 |#####
    309.6 |########################################
    310.1 |############
    310.7 |#########################
    311.3 |######################
    311.9 |############
    312.4 |####################
    313.0 |#################
    313.6 |####################
    314.2 |##############################
    314.7 |#######
    315.3 |##############################
    315.9 |############
    316.5 |##########
    317.0 |##########
    317.6 |#######
  (12 below, 12 above range)

float_i64_scaled (n=160, range 417.8-429.1 ns)
    417.8 |###########
    418.4 |#######
    418.9 |#######################
    419.5 |#######################
    420.0 |#######
    420.6 |##############
    421.2 |########################################
    421.7 |#######
    422.3 |#########
    422.9 |########################################
    423.4 |############################
    424.0 |##################
    424.6 |#######################
    425.1 |####
    425.7 |################
    426.3 |#######
    426.8 |#########
    427.4 |###########
    428.0 |##
    428.5 |###########
  (15 below, 10 above range)

```

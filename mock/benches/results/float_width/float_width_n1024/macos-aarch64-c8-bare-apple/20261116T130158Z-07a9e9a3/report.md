# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### float_f32 dominates: 25% faster than the next best (float_f64)

float_f32 (249 ns) leads float_f64 (311 ns) by 25%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (float_f32)

The baseline float_f32 is the fastest (249 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (float_f32) is the fastest** at 248.7 ns median
- 2 variants significantly slower than baseline
- Spread: 1.70x (fastest 248.7 ns, slowest 421.7 ns)

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
| float_f32 | 2366ns | 2361ns | 2356ns | 2361ns | 2391ns | base |
| float_f64 | 2408ns | 2389ns | 2383ns | 2389ns | 2488ns | +1.76% |
| float_i64_scaled | 2497ns | 2493ns | 2488ns | 2493ns | 2517ns | +5.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 249ns | 245ns | 254ns | base | 4.111 |
| float_f64 | 313ns | 307ns | 328ns | +25.78% | 3.268 |
| float_i64_scaled | 423ns | 418ns | 431ns | +69.82% | 2.421 |

## Performance model

- Peak throughput: **4.182 Gops/s** (float_f32; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 4.117 | 98.4% |
| float_f64 | 3.295 | 78.8% |
| float_i64_scaled | 2.428 | 58.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 2366ns | 2366ns | base |
| float_f64 | 2408ns | 2408ns | +1.76% |
| float_i64_scaled | 2497ns | 2497ns | +5.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 249ns | base | --- | [248, 249] | --- | --- | --- | --- |
| float_f64 | 311ns | +61.7ns (+24.8%) | [+62, +62]ns | [310, 311] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 422ns | +173.8ns (+69.9%) | [+173, +174]ns | [421, 422] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 248ns | +25.4% | +70.4% |
| 1 | 2 | 248ns | +24.8% | +69.7% |
| 1 | 3 | 250ns | +24.2% | +69.0% |
| 1 | 4 | 249ns | +25.1% | +69.4% |
| 1 | 5 | 249ns | +24.9% | +69.6% |
| 1 | 6 | 250ns | +24.0% | +68.6% |
| 1 | 7 | 249ns | +25.0% | +69.4% |
| 1 | 8 | 248ns | +25.3% | +70.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | -0.032 | ok |
| float_f64 | -0.160 | ok |
| float_i64_scaled | -0.272 | moderate- |

**Consistency summary:**

- **float_f64**: won 0/8, lost 8/8
- **float_i64_scaled**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 2.5ns | 249.1ns | 1.0% |  |
| float_f64 | 2.8ns | 313.3ns | 0.9% |  |
| float_i64_scaled | 2.3ns | 423.0ns | 0.5% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 244.8-254.2 ns)
    244.8 |#######
    245.3 |###########
    245.8 |############################
    246.3 |#####################
    246.7 |###########
    247.2 |#######
    247.7 |#########################
    248.1 |#########################
    248.6 |############################
    249.1 |#########################
    249.5 |########################################
    250.0 |###########
    250.5 |#########################
    250.9 |################
    251.4 |#########
    251.9 |####
    252.4 |#######
    252.8 |#######
    253.3 |##############
    253.8 |####
  (14 below, 4 above range)

float_f64 (n=160, range 306.7-327.8 ns)
    306.7 |############
    307.7 |#########################
    308.8 |################################
    309.8 |########################################
    310.9 |####################
    312.0 |#########################
    313.0 |###########
    314.1 |#####################
    315.1 |####
    316.2 |#####
    317.2 |####
    318.3 |
    319.3 |
    320.4 |
    321.5 |
    322.5 |
    323.6 |#
    324.6 |
    325.7 |
    326.7 |
  (13 below, 3 above range)

float_i64_scaled (n=160, range 417.8-430.7 ns)
    417.8 |########################
    418.5 |###########
    419.1 |####################
    419.8 |#################################
    420.4 |####################
    421.1 |##########################
    421.7 |###################################
    422.3 |####################
    423.0 |######
    423.6 |########################################
    424.3 |###############
    424.9 |#################
    425.5 |######
    426.2 |#################
    426.8 |######
    427.5 |########
    428.1 |####
    428.7 |
    429.4 |######
    430.0 |
  (11 below, 4 above range)

```

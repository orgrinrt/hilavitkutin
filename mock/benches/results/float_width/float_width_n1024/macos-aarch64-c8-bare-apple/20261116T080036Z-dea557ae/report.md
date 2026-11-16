# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### float_f32 dominates: 24% faster than the next best (float_f64)

float_f32 (255 ns) leads float_f64 (318 ns) by 24%, a clear separation rather than a photo finish. CV 4.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### float_f64 shows warm-up / thermal drift (autocorr +0.53)

float_f64's per-pass series has lag-1 autocorrelation +0.53, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (float_f32)

The baseline float_f32 is the fastest (255 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (float_f32) is the fastest** at 255.4 ns median
- 2 variants significantly slower than baseline
- Spread: 1.69x (fastest 255.4 ns, slowest 431.7 ns)

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
| float_f32 | 2417ns | 2402ns | 2387ns | 2406ns | 2478ns | base |
| float_f64 | 2436ns | 2423ns | 2415ns | 2423ns | 2496ns | +0.81% |
| float_i64_scaled | 2544ns | 2532ns | 2523ns | 2533ns | 2599ns | +5.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 256ns | 251ns | 266ns | base | 3.993 |
| float_f64 | 319ns | 313ns | 326ns | +24.21% | 3.215 |
| float_i64_scaled | 434ns | 427ns | 447ns | +69.18% | 2.360 |

## Performance model

- Peak throughput: **4.074 Gops/s** (float_f32; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 4.009 | 98.4% |
| float_f64 | 3.221 | 79.1% |
| float_i64_scaled | 2.372 | 58.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 2417ns | 2417ns | base |
| float_f64 | 2436ns | 2436ns | +0.81% |
| float_i64_scaled | 2544ns | 2544ns | +5.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 255ns | base | --- | [255, 256] | --- | --- | --- | --- |
| float_f64 | 318ns | +62.9ns (+24.6%) | [+62, +63]ns | [318, 319] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 432ns | +176.5ns (+69.1%) | [+175, +178]ns | [431, 432] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 256ns | +24.3% | +69.6% |
| 1 | 2 | 256ns | +24.4% | +68.4% |
| 1 | 3 | 254ns | +25.2% | +69.4% |
| 1 | 4 | 255ns | +25.0% | +68.9% |
| 1 | 5 | 254ns | +25.2% | +69.3% |
| 1 | 6 | 255ns | +24.3% | +69.0% |
| 1 | 7 | 257ns | +23.4% | +68.2% |
| 1 | 8 | 254ns | +24.8% | +70.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | -0.242 | moderate- |
| float_f64 | 0.530 | HIGH+ (drift/warm-up) |
| float_i64_scaled | 0.249 | moderate+ |

**Consistency summary:**

- **float_f64**: won 0/8, lost 8/8
- **float_i64_scaled**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 2.6ns | 256.5ns | 1.0% |  |
| float_f64 | 2.9ns | 318.5ns | 0.9% |  |
| float_i64_scaled | 2.5ns | 433.9ns | 0.6% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 251.3-265.5 ns)
    251.3 |########
    252.0 |##############################
    252.7 |##################################
    253.5 |############
    254.2 |##################################
    254.9 |########################################
    255.6 |####################################
    256.3 |##########
    257.0 |######################
    257.7 |############
    258.4 |
    259.1 |##################
    259.8 |##########
    260.6 |############
    261.3 |##
    262.0 |
    262.7 |
    263.4 |
    264.1 |####
    264.8 |##
  (13 below, 4 above range)

float_f64 (n=160, range 313.1-325.7 ns)
    313.1 |#########
    313.7 |#############
    314.3 |###
    315.0 |###################
    315.6 |########################
    316.2 |#############
    316.9 |#########
    317.5 |########################################
    318.1 |###########
    318.8 |######################
    319.4 |############################
    320.0 |#########
    320.7 |###########
    321.3 |#####
    321.9 |####################
    322.6 |#####
    323.2 |#########
    323.8 |#
    324.5 |#####
    325.1 |
  (16 below, 4 above range)

float_i64_scaled (n=160, range 426.8-446.9 ns)
    426.8 |###########
    427.8 |#######################
    428.8 |###########
    429.8 |########################################
    430.8 |############################
    431.8 |###############
    432.9 |##########################
    433.9 |##############################
    434.9 |###############
    435.9 |######
    436.9 |#####
    437.9 |###
    438.9 |
    439.9 |#####
    440.9 |###
    441.9 |#
    442.9 |
    443.9 |
    444.9 |
    445.9 |
  (16 below, 8 above range)

```

## Diagnostics

- **float_f64**: autocorrelation=0.53 (measurement drift or warm-up artifact)

# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### float_f32 dominates: 61% faster than the next best (float_f64)

float_f32 (47 ns) leads float_f64 (76 ns) by 61%, a clear separation rather than a photo finish. CV 8.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### float_i64_scaled is an outlier: 2.6x slower than the field

float_i64_scaled (122 ns) is 2.6x the fastest (47 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (float_f32)

The baseline float_f32 is the fastest (47 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (float_f32) is the fastest** at 47.1 ns median
- 2 variants significantly slower than baseline
- Spread: 2.60x (fastest 47.1 ns, slowest 122.5 ns)

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
| float_f32 | 2179ns | 2168ns | 2158ns | 2172ns | 2224ns | base |
| float_f64 | 2195ns | 2180ns | 2172ns | 2183ns | 2254ns | +0.73% |
| float_i64_scaled | 2245ns | 2227ns | 2217ns | 2232ns | 2313ns | +3.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 47ns | 42ns | 53ns | base | 5.418 |
| float_f64 | 76ns | 70ns | 84ns | +61.79% | 3.349 |
| float_i64_scaled | 123ns | 118ns | 132ns | +161.35% | 2.073 |

## Performance model

- Peak throughput: **6.103 Gops/s** (float_f32; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 5.435 | 89.1% |
| float_f64 | 3.368 | 55.2% |
| float_i64_scaled | 2.090 | 34.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 2179ns | 2179ns | base |
| float_f64 | 2195ns | 2195ns | +0.73% |
| float_i64_scaled | 2245ns | 2245ns | +3.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 47ns | base | --- | [46, 48] | --- | --- | --- | --- |
| float_f64 | 76ns | +28.8ns (+61.0%) | [+28, +29]ns | [75, 77] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 122ns | +75.4ns (+160.1%) | [+74, +76]ns | [122, 123] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 46ns | +61.1% | +167.0% |
| 1 | 2 | 48ns | +56.6% | +155.2% |
| 1 | 3 | 48ns | +58.1% | +156.8% |
| 1 | 4 | 48ns | +60.9% | +157.5% |
| 1 | 5 | 47ns | +66.8% | +165.5% |
| 1 | 6 | 48ns | +60.4% | +157.9% |
| 1 | 7 | 47ns | +60.8% | +161.6% |
| 1 | 8 | 48ns | +59.5% | +153.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | -0.296 | moderate- |
| float_f64 | 0.266 | moderate+ |
| float_i64_scaled | -0.182 | ok |

**Consistency summary:**

- **float_f64**: won 0/8, lost 8/8
- **float_i64_scaled**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 2.5ns | 47.2ns | 5.2% | HIGH |
| float_f64 | 2.3ns | 76.4ns | 3.0% |  |
| float_i64_scaled | 2.4ns | 123.5ns | 1.9% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 41.9-52.8 ns)
     41.9 |########
     42.5 |##################
     43.0 |##########
     43.6 |##################
     44.1 |#############################
     44.7 |########
     45.2 |##########
     45.7 |########################################
     46.3 |########################
     46.8 |#####################
     47.4 |#############################
     47.9 |##################
     48.5 |#############
     49.0 |#############
     49.5 |##################
     50.1 |##################
     50.6 |##########
     51.2 |##########################
     51.7 |#####
     52.2 |##
  (14 below, 16 above range)

float_f64 (n=160, range 70.4-83.6 ns)
     70.4 |##
     71.1 |###############
     71.8 |
     72.4 |#################
     73.1 |####################
     73.7 |############################
     74.4 |###################################
     75.1 |####
     75.7 |########################################
     76.4 |####
     77.0 |#################################
     77.7 |#####################################
     78.4 |######
     79.0 |####################
     79.7 |######
     80.3 |#################
     81.0 |#############
     81.6 |########
     82.3 |##
     83.0 |##
  (15 below, 2 above range)

float_i64_scaled (n=160, range 117.6-132.3 ns)
    117.6 |###########
    118.3 |#############
    119.0 |########################
    119.8 |######################
    120.5 |########################
    121.2 |###########
    122.0 |#####################################
    122.7 |########################################
    123.4 |###############
    124.2 |#################################
    124.9 |######################
    125.6 |####################
    126.4 |###########
    127.1 |##
    127.9 |########
    128.6 |#############
    129.3 |##
    130.1 |##
    130.8 |
    131.5 |
  (16 below, 2 above range)

```

# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### float_f32 dominates: 61% faster than the next best (float_f64)

float_f32 (45 ns) leads float_f64 (73 ns) by 61%, a clear separation rather than a photo finish. CV 8.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### float_i64_scaled is an outlier: 2.6x slower than the field

float_i64_scaled (119 ns) is 2.6x the fastest (45 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### float_f32 is fastest but the noisiest (CV 8.3%)

float_f32 wins on median (45 ns) yet has the highest variance (CV 8.3%), while float_i64_scaled is the steadiest (CV 2.8%, 119 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (float_f32)

The baseline float_f32 is the fastest (45 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (float_f32) is the fastest** at 45.4 ns median
- 2 variants significantly slower than baseline
- Spread: 2.62x (fastest 45.4 ns, slowest 118.8 ns)

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
| float_f32 | 2136ns | 2132ns | 2127ns | 2132ns | 2158ns | base |
| float_f64 | 2153ns | 2148ns | 2144ns | 2148ns | 2175ns | +0.78% |
| float_i64_scaled | 2195ns | 2191ns | 2188ns | 2192ns | 2213ns | +2.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 45ns | 39ns | 50ns | base | 5.719 |
| float_f64 | 73ns | 68ns | 79ns | +64.08% | 3.485 |
| float_i64_scaled | 119ns | 114ns | 124ns | +165.52% | 2.154 |

## Performance model

- Peak throughput: **6.541 Gops/s** (float_f32; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 5.639 | 86.2% |
| float_f64 | 3.492 | 53.4% |
| float_i64_scaled | 2.155 | 32.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 2136ns | 2136ns | base |
| float_f64 | 2153ns | 2153ns | +0.78% |
| float_i64_scaled | 2195ns | 2195ns | +2.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 45ns | base | --- | [44, 46] | --- | --- | --- | --- |
| float_f64 | 73ns | +28.7ns (+63.2%) | [+28, +29]ns | [73, 74] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 119ns | +74.2ns (+163.4%) | [+74, +75]ns | [118, 119] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 45ns | +64.9% | +163.4% |
| 1 | 2 | 46ns | +61.3% | +157.2% |
| 1 | 3 | 45ns | +61.4% | +165.6% |
| 1 | 4 | 46ns | +60.3% | +161.4% |
| 1 | 5 | 44ns | +67.2% | +170.0% |
| 1 | 6 | 45ns | +61.1% | +161.1% |
| 1 | 7 | 46ns | +62.3% | +160.0% |
| 1 | 8 | 47ns | +57.4% | +154.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | -0.073 | ok |
| float_f64 | 0.130 | ok |
| float_i64_scaled | -0.048 | ok |

**Consistency summary:**

- **float_f64**: won 0/8, lost 8/8
- **float_i64_scaled**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 2.3ns | 44.8ns | 5.2% | HIGH |
| float_f64 | 1.9ns | 73.4ns | 2.6% |  |
| float_i64_scaled | 2.1ns | 118.9ns | 1.7% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 39.1-49.5 ns)
     39.1 |##############
     39.7 |##
     40.2 |####
     40.7 |##############################
     41.2 |#######
     41.7 |################
     42.3 |####
     42.8 |#########
     43.3 |###################################
     43.8 |##############
     44.3 |##
     44.8 |##################
     45.4 |########################################
     45.9 |####
     46.4 |################
     46.9 |#########################
     47.4 |########################################
     48.0 |#########
     48.5 |###########
     49.0 |###########
  (11 below, 13 above range)

float_f64 (n=160, range 68.4-78.5 ns)
     68.4 |########
     68.9 |####################
     69.4 |###########
     69.9 |###############################
     70.4 |#################
     70.9 |###########
     71.4 |##############
     71.9 |##############
     72.4 |########################################
     72.9 |######################
     73.4 |######################
     73.9 |#################
     74.5 |######################
     75.0 |############################
     75.5 |#################
     76.0 |####################
     76.5 |##
     77.0 |###############################
     77.5 |###########
     78.0 |###########
  (14 below, 14 above range)

float_i64_scaled (n=160, range 114.2-123.6 ns)
    114.2 |#########
    114.7 |###########
    115.2 |###########
    115.6 |####
    116.1 |##################
    116.6 |##################
    117.1 |##############################
    117.5 |##################
    118.0 |#####################
    118.5 |############################
    118.9 |#######
    119.4 |###########
    119.9 |#######
    120.3 |########################################
    120.8 |################
    121.3 |###########
    121.7 |####
    122.2 |##############
    122.7 |#######
    123.1 |################
  (15 below, 13 above range)

```

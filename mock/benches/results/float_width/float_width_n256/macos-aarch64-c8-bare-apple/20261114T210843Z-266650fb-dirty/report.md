# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### float_f32 dominates: 50% faster than the next best (float_f64)

float_f32 (50 ns) leads float_f64 (75 ns) by 50%, a clear separation rather than a photo finish. CV 10.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### float_i64_scaled is an outlier: 2.7x slower than the field

float_i64_scaled (134 ns) is 2.7x the fastest (50 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### float_f32 is fastest but the noisiest (CV 10.1%)

float_f32 wins on median (50 ns) yet has the highest variance (CV 10.1%), while float_i64_scaled is the steadiest (CV 6.3%, 134 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (float_f32)

The baseline float_f32 is the fastest (50 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (float_f32) is the fastest** at 50.2 ns median
- 2 variants significantly slower than baseline
- Spread: 2.67x (fastest 50.2 ns, slowest 134.2 ns)

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
| float_f32 | 2321ns | 2354ns | 2139ns | 2335ns | 2461ns | base |
| float_f64 | 2259ns | 2182ns | 2133ns | 2230ns | 2472ns | -2.69% |
| float_i64_scaled | 2413ns | 2508ns | 2220ns | 2438ns | 2530ns | +3.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 50ns | 43ns | 57ns | base | 5.098 |
| float_f64 | 77ns | 70ns | 87ns | +53.49% | 3.321 |
| float_i64_scaled | 132ns | 120ns | 142ns | +163.34% | 1.936 |

## Performance model

- Peak throughput: **5.959 Gops/s** (float_f32; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 5.100 | 85.6% |
| float_f64 | 3.395 | 57.0% |
| float_i64_scaled | 1.908 | 32.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 2321ns | 2321ns | base |
| float_f64 | 2259ns | 2259ns | -2.69% |
| float_i64_scaled | 2413ns | 2413ns | +3.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 50ns | base | --- | [49, 52] | --- | --- | --- | --- |
| float_f64 | 75ns | +25.2ns (+50.2%) | [+24, +27]ns | [75, 78] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 134ns | +84.6ns (+168.5%) | [+82, +86]ns | [131, 136] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 49ns | +47.9% | +154.8% |
| 1 | 2 | 50ns | +54.2% | +178.3% |
| 1 | 3 | 52ns | +44.0% | +169.5% |
| 1 | 4 | 49ns | +46.9% | +164.6% |
| 1 | 5 | 47ns | +82.0% | +193.1% |
| 1 | 6 | 53ns | +37.1% | +160.8% |
| 1 | 7 | 53ns | +57.5% | +128.3% |
| 1 | 8 | 49ns | +59.8% | +170.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | -0.104 | ok |
| float_f64 | -0.466 | moderate- |
| float_i64_scaled | -0.181 | ok |

**Consistency summary:**

- **float_f64**: won 0/8, lost 8/8
- **float_i64_scaled**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 2.9ns | 50.2ns | 5.8% | HIGH |
| float_f64 | 2.2ns | 77.1ns | 2.9% |  |
| float_i64_scaled | 2.4ns | 132.2ns | 1.8% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 43.0-57.2 ns)
     43.0 |##
     43.7 |########
     44.4 |#########################
     45.1 |#########################
     45.8 |#####
     46.5 |############################
     47.2 |###############################
     47.9 |#################
     48.6 |##############
     49.4 |#########################
     50.1 |##############
     50.8 |##############
     51.5 |#########################
     52.2 |############################
     52.9 |####################
     53.6 |###########
     54.3 |###########
     55.0 |###########
     55.8 |#########################
     56.5 |########################################
  (15 below, 9 above range)

float_f64 (n=160, range 69.7-86.6 ns)
     69.7 |##
     70.5 |######################
     71.4 |########################################
     72.2 |#########################
     73.1 |############################
     73.9 |############################
     74.8 |##################################
     75.6 |########
     76.5 |#################
     77.3 |############################
     78.2 |##############
     79.0 |##############
     79.9 |##############
     80.7 |########
     81.6 |########
     82.4 |###########
     83.2 |###########
     84.1 |####################
     84.9 |###########
     85.8 |##############
  (18 below, 14 above range)

float_i64_scaled (n=160, range 119.9-142.4 ns)
    119.9 |###############
    121.0 |##############################
    122.1 |###
    123.3 |########################################
    124.4 |#####################
    125.5 |############
    126.6 |###
    127.8 |##################
    128.9 |##################
    130.0 |######
    131.1 |#####################
    132.3 |############
    133.4 |############
    134.5 |########################
    135.6 |##############################
    136.8 |###############
    137.9 |########################################
    139.0 |#################################
    140.1 |####################################
    141.3 |############
  (11 below, 16 above range)

```

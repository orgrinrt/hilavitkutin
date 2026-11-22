# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### float_f32 dominates: 60% faster than the next best (float_f64)

float_f32 (46 ns) leads float_f64 (73 ns) by 60%, a clear separation rather than a photo finish. CV 12.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### float_i64_scaled is an outlier: 2.6x slower than the field

float_i64_scaled (120 ns) is 2.6x the fastest (46 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### float_f32 is fastest but the noisiest (CV 12.4%)

float_f32 wins on median (46 ns) yet has the highest variance (CV 12.4%), while float_i64_scaled is the steadiest (CV 2.8%, 120 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (float_f32)

The baseline float_f32 is the fastest (46 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (float_f32) is the fastest** at 45.6 ns median
- 2 variants significantly slower than baseline
- Spread: 2.63x (fastest 45.6 ns, slowest 120.0 ns)

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
| float_f32 | 2158ns | 2137ns | 2130ns | 2149ns | 2210ns | base |
| float_f64 | 2160ns | 2151ns | 2146ns | 2152ns | 2198ns | +0.12% |
| float_i64_scaled | 2207ns | 2195ns | 2189ns | 2200ns | 2246ns | +2.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 46ns | 40ns | 52ns | base | 5.598 |
| float_f64 | 73ns | 68ns | 79ns | +60.21% | 3.494 |
| float_i64_scaled | 120ns | 115ns | 124ns | +161.54% | 2.140 |

## Performance model

- Peak throughput: **6.376 Gops/s** (float_f32; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 5.614 | 88.1% |
| float_f64 | 3.502 | 54.9% |
| float_i64_scaled | 2.133 | 33.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 2158ns | 2158ns | base |
| float_f64 | 2160ns | 2160ns | +0.12% |
| float_i64_scaled | 2207ns | 2207ns | +2.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 46ns | base | --- | [45, 46] | --- | --- | --- | --- |
| float_f64 | 73ns | +27.9ns (+61.2%) | [+27, +28]ns | [72, 74] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 120ns | +74.2ns (+162.7%) | [+74, +75]ns | [119, 121] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 45ns | +60.0% | +165.0% |
| 1 | 2 | 44ns | +66.1% | +176.0% |
| 1 | 3 | 45ns | +60.6% | +160.7% |
| 1 | 4 | 45ns | +63.4% | +165.3% |
| 1 | 5 | 46ns | +58.3% | +155.7% |
| 1 | 6 | 45ns | +66.4% | +171.3% |
| 1 | 7 | 45ns | +58.4% | +163.9% |
| 1 | 8 | 47ns | +55.7% | +154.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | -0.146 | ok |
| float_f64 | -0.268 | moderate- |
| float_i64_scaled | -0.352 | moderate- |

**Consistency summary:**

- **float_f64**: won 0/8, lost 8/8
- **float_i64_scaled**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 2.3ns | 45.7ns | 5.1% | HIGH |
| float_f64 | 2.1ns | 73.3ns | 2.8% |  |
| float_i64_scaled | 2.0ns | 119.6ns | 1.7% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 40.2-52.0 ns)
     40.2 |####
     40.7 |#############
     41.3 |########
     41.9 |#############
     42.5 |#############
     43.1 |########
     43.7 |########################################
     44.3 |###########
     44.9 |##########################
     45.5 |####################
     46.1 |###############
     46.7 |##########################
     47.2 |#################
     47.8 |########################
     48.4 |########
     49.0 |########################
     49.6 |########
     50.2 |##
     50.8 |########
     51.4 |####
  (17 below, 7 above range)

float_f64 (n=160, range 68.0-78.5 ns)
     68.0 |######
     68.6 |######
     69.1 |##################
     69.6 |##################
     70.1 |##################
     70.7 |##################
     71.2 |####################################
     71.7 |########################################
     72.2 |########################
     72.7 |############
     73.3 |#####################
     73.8 |#####################
     74.3 |#####################
     74.8 |#####################
     75.4 |########################################
     75.9 |############
     76.4 |##################
     76.9 |##################
     77.5 |#####################
     78.0 |#########
  (15 below, 13 above range)

float_i64_scaled (n=160, range 114.5-124.1 ns)
    114.5 |########
    115.0 |##########
    115.5 |#
    116.0 |#####
    116.5 |#####
    116.9 |############
    117.4 |#############
    117.9 |######################
    118.4 |########
    118.8 |############
    119.3 |########
    119.8 |############
    120.3 |######
    120.8 |########################################
    121.2 |#############
    121.7 |##########
    122.2 |######
    122.7 |#####
    123.1 |############
    123.6 |##########
  (14 below, 15 above range)

```

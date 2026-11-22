# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (float_f32)

The baseline float_f32 is the fastest (5.04 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (float_f32) is the fastest** at 5044.2 ns median
- 2 variants significantly slower than baseline
- Spread: 1.27x (fastest 5044.2 ns, slowest 6417.1 ns)

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
| float_f32 | 7200ns | 7159ns | 7151ns | 7161ns | 7367ns | base |
| float_f64 | 7201ns | 7185ns | 7178ns | 7187ns | 7265ns | +0.00% |
| float_i64_scaled | 8532ns | 8502ns | 8493ns | 8510ns | 8637ns | +18.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 5074ns | 5040ns | 5195ns | base | 3.229 |
| float_f64 | 5116ns | 5101ns | 5160ns | +0.83% | 3.202 |
| float_i64_scaled | 6440ns | 6410ns | 6527ns | +26.93% | 2.544 |

## Performance model

- Peak throughput: **3.251 Gops/s** (float_f32; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 3.248 | 99.9% |
| float_f64 | 3.209 | 98.7% |
| float_i64_scaled | 2.553 | 78.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 7200ns | 7200ns | base |
| float_f64 | 7201ns | 7201ns | +0.00% |
| float_i64_scaled | 8532ns | 8532ns | +18.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 5044ns | base | --- | [5043, 5045] | --- | --- | --- | --- |
| float_f64 | 5106ns | +62.1ns (+1.2%) | [+61, +63]ns | [5106, 5108] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 6417ns | +1372.1ns (+27.2%) | [+1371, +1374]ns | [6416, 6419] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 5043ns | +1.3% | +28.5% |
| 1 | 2 | 5043ns | +1.2% | +27.2% |
| 1 | 3 | 5044ns | +1.3% | +27.2% |
| 1 | 4 | 5043ns | +1.3% | +27.2% |
| 1 | 5 | 5045ns | +1.2% | +27.2% |
| 1 | 6 | 5042ns | +1.3% | +27.3% |
| 1 | 7 | 5161ns | -1.1% | +24.3% |
| 1 | 8 | 5043ns | +1.3% | +27.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | -0.174 | ok |
| float_f64 | -0.455 | moderate- |
| float_i64_scaled | -0.010 | ok |

**Consistency summary:**

- **float_f64**: won 1/8, lost 7/8
- **float_i64_scaled**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 3.3ns | 5073.9ns | 0.1% |  |
| float_f64 | 3.2ns | 5116.3ns | 0.1% |  |
| float_i64_scaled | 3.1ns | 6440.4ns | 0.0% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 5039.8-5195.1 ns)
   5039.8 |########################################
   5047.6 |###
   5055.3 |#
   5063.1 |#
   5070.9 |#
   5078.6 |
   5086.4 |
   5094.2 |#
   5101.9 |
   5109.7 |
   5117.4 |
   5125.2 |
   5133.0 |
   5140.7 |
   5148.5 |
   5156.3 |
   5164.0 |
   5171.8 |
   5179.6 |
   5187.3 |
  (13 below, 13 above range)

float_f64 (n=160, range 5101.4-5159.8 ns)
   5101.4 |###############################
   5104.3 |########################################
   5107.2 |################################
   5110.2 |#######
   5113.1 |#
   5116.0 |#
   5118.9 |
   5121.8 |
   5124.8 |
   5127.7 |#
   5130.6 |
   5133.5 |
   5136.5 |
   5139.4 |
   5142.3 |
   5145.2 |
   5148.2 |
   5151.1 |
   5154.0 |
   5156.9 |#
  (14 below, 14 above range)

float_i64_scaled (n=160, range 6410.4-6527.1 ns)
   6410.4 |########################################
   6416.2 |###########################
   6422.0 |####
   6427.9 |
   6433.7 |#####
   6439.5 |
   6445.4 |
   6451.2 |##
   6457.1 |###
   6462.9 |##
   6468.7 |
   6474.6 |###
   6480.4 |#
   6486.2 |
   6492.1 |##
   6497.9 |
   6503.8 |#
   6509.6 |
   6515.4 |
   6521.3 |
  (12 below, 9 above range)

```

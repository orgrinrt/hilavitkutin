# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### float_i64_scaled shows warm-up / thermal drift (autocorr +0.58)

float_i64_scaled's per-pass series has lag-1 autocorrelation +0.58, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (float_f32)

The baseline float_f32 is the fastest (5.04 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (float_f32) is the fastest** at 5042.9 ns median
- 2 variants significantly slower than baseline
- Spread: 1.27x (fastest 5042.9 ns, slowest 6414.8 ns)

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
| float_f32 | 7163ns | 7156ns | 7149ns | 7156ns | 7200ns | base |
| float_f64 | 7190ns | 7183ns | 7178ns | 7183ns | 7224ns | +0.37% |
| float_i64_scaled | 8505ns | 8498ns | 8492ns | 8498ns | 8539ns | +18.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 5048ns | 5039ns | 5074ns | base | 3.245 |
| float_f64 | 5111ns | 5101ns | 5139ns | +1.24% | 3.206 |
| float_i64_scaled | 6419ns | 6410ns | 6440ns | +27.15% | 2.552 |

## Performance model

- Peak throughput: **3.251 Gops/s** (float_f32; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 3.249 | 99.9% |
| float_f64 | 3.209 | 98.7% |
| float_i64_scaled | 2.554 | 78.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 7163ns | 7163ns | base |
| float_f64 | 7190ns | 7190ns | +0.37% |
| float_i64_scaled | 8505ns | 8505ns | +18.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 5043ns | base | --- | [5042, 5043] | --- | --- | --- | --- |
| float_f64 | 5105ns | +62.5ns (+1.2%) | [+62, +63]ns | [5104, 5106] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 6415ns | +1371.6ns (+27.2%) | [+1371, +1372]ns | [6414, 6416] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 5043ns | +1.2% | +27.2% |
| 1 | 2 | 5044ns | +1.2% | +27.2% |
| 1 | 3 | 5042ns | +1.2% | +27.2% |
| 1 | 4 | 5042ns | +1.2% | +27.2% |
| 1 | 5 | 5042ns | +1.2% | +27.2% |
| 1 | 6 | 5043ns | +1.2% | +27.2% |
| 1 | 7 | 5044ns | +1.2% | +27.2% |
| 1 | 8 | 5043ns | +1.3% | +27.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | -0.032 | ok |
| float_f64 | 0.109 | ok |
| float_i64_scaled | 0.575 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **float_f64**: won 0/8, lost 8/8
- **float_i64_scaled**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 3.4ns | 5048.3ns | 0.1% |  |
| float_f64 | 3.2ns | 5110.9ns | 0.1% |  |
| float_i64_scaled | 3.0ns | 6418.9ns | 0.0% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 5039.1-5073.7 ns)
   5039.1 |###################################
   5040.8 |###################################
   5042.6 |########################################
   5044.3 |#################
   5046.0 |############
   5047.7 |####
   5049.5 |
   5051.2 |
   5052.9 |
   5054.7 |
   5056.4 |
   5058.1 |
   5059.8 |
   5061.6 |##
   5063.3 |#
   5065.0 |#
   5066.8 |
   5068.5 |#
   5070.2 |#
   5071.9 |
  (12 below, 11 above range)

float_f64 (n=160, range 5100.6-5138.5 ns)
   5100.6 |####################################
   5102.5 |####################################
   5104.4 |########################################
   5106.3 |#####################
   5108.2 |######################
   5110.1 |###
   5112.0 |
   5113.9 |
   5115.8 |
   5117.7 |
   5119.6 |#
   5121.5 |##
   5123.4 |#
   5125.2 |
   5127.1 |
   5129.0 |
   5130.9 |
   5132.8 |
   5134.7 |
   5136.6 |
  (14 below, 14 above range)

float_i64_scaled (n=160, range 6409.7-6440.3 ns)
   6409.7 |#############################
   6411.2 |######################################
   6412.8 |########################################
   6414.3 |########################################
   6415.8 |####################
   6417.3 |#############################
   6418.9 |###################
   6420.4 |#####
   6421.9 |#
   6423.5 |###
   6425.0 |
   6426.5 |#
   6428.1 |
   6429.6 |
   6431.1 |
   6432.6 |
   6434.2 |
   6435.7 |###
   6437.2 |
   6438.8 |#
  (13 below, 12 above range)

```

## Diagnostics

- **float_i64_scaled**: autocorrelation=0.58 (measurement drift or warm-up artifact)

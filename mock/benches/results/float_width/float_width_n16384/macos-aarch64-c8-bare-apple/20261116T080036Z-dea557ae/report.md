# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (float_f32)

The baseline float_f32 is the fastest (5.15 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (float_f32) is the fastest** at 5154.8 ns median
- 2 variants significantly slower than baseline
- Spread: 1.27x (fastest 5154.8 ns, slowest 6551.2 ns)

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
| float_f32 | 7364ns | 7307ns | 7279ns | 7326ns | 7562ns | base |
| float_f64 | 7402ns | 7345ns | 7308ns | 7366ns | 7601ns | +0.51% |
| float_i64_scaled | 8736ns | 8679ns | 8645ns | 8688ns | 8972ns | +18.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 5199ns | 5138ns | 5346ns | base | 3.151 |
| float_f64 | 5268ns | 5202ns | 5411ns | +1.32% | 3.110 |
| float_i64_scaled | 6601ns | 6535ns | 6781ns | +26.95% | 2.482 |

## Performance model

- Peak throughput: **3.189 Gops/s** (float_f32; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 3.178 | 99.7% |
| float_f64 | 3.134 | 98.3% |
| float_i64_scaled | 2.501 | 78.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 7364ns | 7364ns | base |
| float_f64 | 7402ns | 7402ns | +0.51% |
| float_i64_scaled | 8736ns | 8736ns | +18.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 5155ns | base | --- | [5146, 5172] | --- | --- | --- | --- |
| float_f64 | 5228ns | +65.8ns (+1.3%) | [+61, +73]ns | [5218, 5243] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 6551ns | +1397.8ns (+27.1%) | [+1395, +1404]ns | [6545, 6560] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 5147ns | +1.5% | +28.2% |
| 1 | 2 | 5152ns | +1.1% | +27.3% |
| 1 | 3 | 5145ns | +1.7% | +27.5% |
| 1 | 4 | 5162ns | +1.1% | +27.0% |
| 1 | 5 | 5150ns | +1.5% | +27.0% |
| 1 | 6 | 5142ns | +1.5% | +27.5% |
| 1 | 7 | 5149ns | +1.1% | +27.1% |
| 1 | 8 | 5339ns | +1.6% | +22.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | -0.026 | ok |
| float_f64 | -0.084 | ok |
| float_i64_scaled | 0.088 | ok |

**Consistency summary:**

- **float_f64**: won 0/8, lost 8/8
- **float_i64_scaled**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 2.9ns | 5199.3ns | 0.1% |  |
| float_f64 | 2.9ns | 5267.7ns | 0.1% |  |
| float_i64_scaled | 2.6ns | 6600.6ns | 0.0% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 5137.7-5346.1 ns)
   5137.7 |########################################
   5148.2 |########
   5158.6 |###
   5169.0 |####
   5179.4 |###
   5189.8 |##
   5200.2 |##
   5210.6 |####
   5221.1 |####
   5231.5 |###
   5241.9 |####
   5252.3 |##
   5262.7 |#
   5273.1 |
   5283.6 |
   5294.0 |
   5304.4 |#
   5314.8 |
   5325.2 |#
   5335.6 |
  (15 below, 14 above range)

float_f64 (n=160, range 5201.6-5410.6 ns)
   5201.6 |########################################
   5212.0 |#########
   5222.5 |##########
   5232.9 |######
   5243.4 |####
   5253.8 |####
   5264.3 |##
   5274.7 |#
   5285.2 |###
   5295.6 |#######
   5306.1 |###
   5316.5 |#####
   5327.0 |#
   5337.4 |
   5347.9 |###
   5358.3 |#
   5368.8 |#
   5379.2 |
   5389.7 |#
   5400.1 |
  (13 below, 13 above range)

float_i64_scaled (n=160, range 6535.0-6781.5 ns)
   6535.0 |########################################
   6547.3 |############
   6559.6 |######
   6572.0 |##
   6584.3 |######
   6596.6 |##
   6608.9 |#####
   6621.3 |#####
   6633.6 |#
   6645.9 |###
   6658.2 |####
   6670.6 |##
   6682.9 |
   6695.2 |
   6707.5 |#
   6719.9 |
   6732.2 |
   6744.5 |
   6756.8 |
   6769.2 |
  (13 below, 5 above range)

```

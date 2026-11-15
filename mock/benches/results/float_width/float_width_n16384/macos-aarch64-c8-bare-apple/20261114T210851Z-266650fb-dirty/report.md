# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (float_f64, float_f32) are a dead heat (<1%)

float_f64 (5.21 us) and float_f32 (5.22 us) differ by 0.21%, inside the noise, even though the wider field spreads 23.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: float_f64** at 5209.0 ns median (-0.2% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.24x (fastest 5209.0 ns, slowest 6452.5 ns)

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
| float_f32 | 7428ns | 7394ns | 7148ns | 7415ns | 7750ns | base |
| float_f64 | 7386ns | 7318ns | 7172ns | 7338ns | 7744ns | -0.57% |
| float_i64_scaled | 8734ns | 8558ns | 8489ns | 8662ns | 9192ns | +17.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 5240ns | 5042ns | 5463ns | base | 3.127 |
| float_f64 | 5255ns | 5104ns | 5508ns | +0.28% | 3.118 |
| float_i64_scaled | 6596ns | 6413ns | 6944ns | +25.88% | 2.484 |

## Performance model

- Peak throughput: **3.250 Gops/s** (float_f32; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 3.139 | 96.6% |
| float_f64 | 3.145 | 96.8% |
| float_i64_scaled | 2.539 | 78.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 7428ns | 7428ns | base |
| float_f64 | 7386ns | 7386ns | -0.57% |
| float_i64_scaled | 8734ns | 8734ns | +17.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 5220ns | base | --- | [5182, 5285] | --- | --- | --- | --- |
| float_f64 | 5209ns | no significant difference | [-3, +68]ns | [5154, 5264] | no | 0.0687 | 0.0687 | 0 |
| float_i64_scaled | 6452ns | +1371.8ns (+26.3%) | [+1306, +1381]ns | [6423, 6509] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 5409ns | -5.5% | +18.6% |
| 1 | 2 | 5222ns | +1.9% | +31.7% |
| 1 | 3 | 5046ns | +1.2% | +27.2% |
| 1 | 4 | 5410ns | -5.6% | +19.3% |
| 1 | 5 | 5076ns | +3.8% | +35.9% |
| 1 | 6 | 5110ns | +3.4% | +34.5% |
| 1 | 7 | 5053ns | +8.3% | +27.0% |
| 1 | 8 | 5408ns | +1.2% | +18.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | -0.296 | moderate- |
| float_f64 | 0.329 | moderate+ |
| float_i64_scaled | -0.131 | ok |

**Consistency summary:**

- **float_f64**: won 2/8, lost 6/8
- **float_i64_scaled**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 3.3ns | 5240.0ns | 0.1% |  |
| float_f64 | 3.0ns | 5254.6ns | 0.1% |  |
| float_i64_scaled | 2.9ns | 6596.0ns | 0.0% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 5041.8-5463.4 ns)
   5041.8 |##############################
   5062.9 |#####
   5084.0 |#
   5105.1 |###
   5126.1 |#
   5147.2 |
   5168.3 |#
   5189.4 |#
   5210.5 |###########
   5231.5 |###
   5252.6 |###
   5273.7 |
   5294.8 |
   5315.9 |
   5336.9 |#
   5358.0 |#
   5379.1 |#
   5400.2 |########################################
   5421.3 |#
   5442.3 |##
  (15 below, 7 above range)

float_f64 (n=160, range 5104.4-5508.5 ns)
   5104.4 |########################################
   5124.6 |#
   5144.8 |##
   5165.0 |####
   5185.2 |##
   5205.4 |#####
   5225.6 |####
   5245.8 |##
   5266.0 |########
   5286.2 |####
   5306.4 |###
   5326.6 |####
   5346.8 |
   5367.0 |#
   5387.3 |
   5407.5 |
   5427.7 |#
   5447.9 |
   5468.1 |########################
   5488.3 |
  (15 below, 5 above range)

float_i64_scaled (n=160, range 6412.5-6943.5 ns)
   6412.5 |########################################
   6439.1 |######
   6465.6 |##
   6492.2 |####
   6518.7 |
   6545.3 |#
   6571.8 |
   6598.4 |##
   6624.9 |###
   6651.5 |##
   6678.0 |#
   6704.6 |
   6731.1 |
   6757.7 |#
   6784.2 |
   6810.8 |
   6837.3 |
   6863.9 |##############
   6890.4 |#######
   6917.0 |####
  (17 below, 5 above range)

```

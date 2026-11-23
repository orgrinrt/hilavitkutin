# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### float_i64_scaled dominates: 309% faster than the next best (float_f32)

float_i64_scaled (9.14 us) leads float_f32 (37.36 us) by 309%, a clear separation rather than a photo finish. CV 7.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### float_i64_scaled beats baseline by 76% (significant)

float_i64_scaled is -28.41 us (76%) faster than baseline float_f32, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### float_f64 is an outlier: 4.4x slower than the field

float_f64 (40.51 us) is 4.4x the fastest (9.14 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### float_f64 shows alternating (throttle bounce) (autocorr -0.67)

float_f64's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.4x the fastest

Fastest float_i64_scaled (9.14 us) to slowest float_f64 (40.51 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### float_f64's edge over baseline is significant but tiny (15 ns, 0.04%)

float_f64 differs from baseline float_f32 by 15 ns (0.04%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: float_i64_scaled** at 9138.2 ns median (-75.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.43x (fastest 9138.2 ns, slowest 40511.3 ns)

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
| Counter frequency | 2112797269 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| float_f32 | 38569ns | 39124ns | 34667ns | 38575ns | 42452ns | base |
| float_f64 | 39560ns | 42408ns | 34690ns | 40167ns | 42613ns | +2.57% |
| float_i64_scaled | 10535ns | 11020ns | 9017ns | 10862ns | 11074ns | -72.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 36838ns | 33114ns | 40541ns | base | 0.445 |
| float_f64 | 37785ns | 33137ns | 40693ns | +2.57% | 0.434 |
| float_i64_scaled | 8730ns | 7475ns | 9166ns | -76.30% | 1.877 |

## Performance model

- Peak throughput: **2.192 Gops/s** (float_i64_scaled; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 0.438 | 20.0% |
| float_f64 | 0.404 | 18.5% |
| float_i64_scaled | 1.793 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 38569ns | 38569ns | base |
| float_f64 | 39560ns | 39560ns | +2.57% |
| float_i64_scaled | 10535ns | 10535ns | -72.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 37364ns | base | --- | [33150, 40495] | --- | --- | --- | --- |
| float_f64 | 40511ns | +15.0ns (+0.0%) | [+3, +32]ns | [40502, 40512] | YES | 0.0033 | 0.0033 | 0 |
| float_i64_scaled | 9138ns | -28406.1ns (-76.0%) | [-31355, -25665]ns | [9138, 9138] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 40511ns | +0.0% | -77.4% |
| 1 | 2 | 40506ns | -18.2% | -81.5% |
| 1 | 3 | 33130ns | +22.3% | -72.4% |
| 1 | 4 | 40510ns | -18.2% | -77.4% |
| 1 | 5 | 33137ns | +22.3% | -72.4% |
| 1 | 6 | 33144ns | +22.2% | -77.4% |
| 1 | 7 | 33140ns | +0.0% | -72.4% |
| 1 | 8 | 40507ns | +0.0% | -77.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | -0.126 | ok |
| float_f64 | -0.675 | HIGH- (thermal bounce) |
| float_i64_scaled | -0.375 | moderate- |

**Consistency summary:**

- **float_f64**: won 2/8, lost 3/8
- **float_i64_scaled**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 22.5ns | 36837.6ns | 0.1% |  |
| float_f64 | 23.1ns | 37784.7ns | 0.1% |  |
| float_i64_scaled | 23.1ns | 8729.9ns | 0.3% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 33114.2-40541.5 ns)
  33114.2 |#####################################
  33485.6 |#
  33856.9 |
  34228.3 |
  34599.7 |
  34971.0 |
  35342.4 |
  35713.7 |
  36085.1 |
  36456.5 |
  36827.8 |
  37199.2 |
  37570.6 |
  37941.9 |
  38313.3 |
  38684.6 |
  39056.0 |
  39427.4 |
  39798.7 |
  40170.1 |########################################
  (5 below, 4 above range)

float_f64 (n=160, range 33137.1-40692.9 ns)
  33137.1 |####################
  33514.9 |
  33892.7 |
  34270.5 |
  34648.3 |
  35026.1 |
  35403.9 |
  35781.6 |
  36159.4 |
  36537.2 |
  36915.0 |
  37292.8 |
  37670.6 |
  38048.4 |
  38426.2 |
  38804.0 |
  39181.8 |
  39559.6 |
  39937.3 |
  40315.1 |########################################
  (10 below, 3 above range)

float_i64_scaled (n=160, range 7475.2-9165.6 ns)
   7475.2 |##########
   7559.7 |
   7644.3 |
   7728.8 |
   7813.3 |
   7897.8 |
   7982.3 |
   8066.9 |
   8151.4 |
   8235.9 |
   8320.4 |
   8404.9 |
   8489.4 |
   8574.0 |
   8658.5 |
   8743.0 |
   8827.5 |
   8912.0 |
   8996.6 |
   9081.1 |########################################
  (8 below, 3 above range)

```

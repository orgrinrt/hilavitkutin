# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### float_i64_scaled dominates: 249% faster than the next best (float_f32)

float_i64_scaled (151 ns) leads float_f32 (527 ns) by 249%, a clear separation rather than a photo finish. CV 86.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### float_i64_scaled beats baseline by 76% (significant)

float_i64_scaled is -403 ns (76%) faster than baseline float_f32, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### float_f64 is an outlier: 3.5x slower than the field

float_f64 (531 ns) is 3.5x the fastest (151 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### float_i64_scaled is fastest but the noisiest (CV 86.5%)

float_i64_scaled wins on median (151 ns) yet has the highest variance (CV 86.5%), while float_f32 is the steadiest (CV 67.0%, 527 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 3.5x the fastest

Fastest float_i64_scaled (151 ns) to slowest float_f64 (531 ns): 3.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### float_i64_scaled is inconsistent: worst-20% is 3.6x its best-20%

float_i64_scaled's best 20% of batches run at 123 ns but its worst 20% at 439 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### float_f64's edge over baseline is significant but tiny (7 ns, 1.24%)

float_f64 differs from baseline float_f32 by 7 ns (1.24%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: float_i64_scaled** at 150.8 ns median (-71.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.52x (fastest 150.8 ns, slowest 531.5 ns)

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
| Counter frequency | 2112770428 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| float_f32 | 2689ns | 2074ns | 2058ns | 2254ns | 4622ns | base |
| float_f64 | 2826ns | 2083ns | 2066ns | 2202ns | 5458ns | +5.11% |
| float_i64_scaled | 2701ns | 2027ns | 1657ns | 1966ns | 5952ns | +0.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 682ns | 524ns | 1161ns | base | 0.376 |
| float_f64 | 722ns | 531ns | 1391ns | +5.97% | 0.354 |
| float_i64_scaled | 200ns | 123ns | 439ns | -70.60% | 1.278 |

## Performance model

- Peak throughput: **2.075 Gops/s** (float_i64_scaled; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 0.486 | 23.4% |
| float_f64 | 0.482 | 23.2% |
| float_i64_scaled | 1.698 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 2689ns | 2689ns | base |
| float_f64 | 2826ns | 2826ns | +5.11% |
| float_i64_scaled | 2701ns | 2701ns | +0.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 527ns | base | --- | [526, 641] | --- | --- | --- | --- |
| float_f64 | 531ns | +6.6ns (+1.2%) | [+6, +7]ns | [531, 532] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 151ns | -402.9ns (-76.5%) | [-414, -401]ns | [151, 151] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 524ns | +1.3% | -71.2% |
| 1 | 2 | 525ns | +1.2% | -76.5% |
| 1 | 3 | 526ns | +0.9% | -76.5% |
| 1 | 4 | 641ns | +1.3% | -76.5% |
| 1 | 5 | 525ns | +1.2% | -76.5% |
| 1 | 6 | 524ns | +23.8% | -71.2% |
| 1 | 7 | 641ns | -17.1% | -76.5% |
| 1 | 8 | 642ns | -17.2% | -76.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | 0.062 | ok |
| float_f64 | -0.375 | moderate- |
| float_i64_scaled | -0.140 | ok |

**Consistency summary:**

- **float_f64**: won 2/8, lost 6/8
- **float_i64_scaled**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 26.2ns | 681.6ns | 3.8% |  |
| float_f64 | 27.2ns | 722.2ns | 3.8% |  |
| float_i64_scaled | 32.3ns | 200.4ns | 16.1% | HIGH |

## Distribution (algo ns)

```
float_f32 (n=160, range 524.2-1161.5 ns)
    524.2 |########################################
    556.0 |
    587.9 |
    619.8 |#############################
    651.6 |
    683.5 |
    715.4 |
    747.2 |
    779.1 |
    811.0 |
    842.8 |
    874.7 |
    906.6 |
    938.4 |
    970.3 |
   1002.1 |
   1034.0 |
   1065.9 |#
   1097.7 |
   1129.6 |
  (14 below, 13 above range)

float_f64 (n=160, range 530.7-1391.3 ns)
    530.7 |########################################
    573.8 |
    616.8 |#############
    659.8 |#
    702.9 |
    745.9 |
    788.9 |
    832.0 |
    875.0 |
    918.0 |
    961.0 |
   1004.1 |
   1047.1 |
   1090.1 |
   1133.2 |
   1176.2 |
   1219.2 |
   1262.3 |
   1305.3 |
   1348.3 |
  (12 below, 20 above range)

float_i64_scaled (n=160, range 123.4-439.2 ns)
    123.4 |######################
    139.2 |########################################
    155.0 |#
    170.8 |#
    186.6 |
    202.3 |
    218.1 |
    233.9 |
    249.7 |
    265.5 |
    281.3 |
    297.1 |
    312.9 |
    328.7 |
    344.5 |
    360.3 |
    376.1 |
    391.8 |
    407.6 |#
    423.4 |#
  (6 below, 21 above range)

```

## Diagnostics

- **float_f32**: CV=51.8% (high variance, measurements may be unstable)
- **float_f64**: CV=57.6% (high variance, measurements may be unstable)
- **float_i64_scaled**: CV=65.1% (high variance, measurements may be unstable)
- **float_i64_scaled**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **float_i64_scaled**: bridge=15.9% of algo (FFI overhead may distort results)

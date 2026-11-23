# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### float_i64_scaled dominates: 419% faster than the next best (float_f32)

float_i64_scaled (124 ns) leads float_f32 (641 ns) by 419%, a clear separation rather than a photo finish. CV 99.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### float_i64_scaled beats baseline by 77% (significant)

float_i64_scaled is -491 ns (77%) faster than baseline float_f32, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### float_f64 is an outlier: 5.3x slower than the field

float_f64 (649 ns) is 5.3x the fastest (124 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### float_i64_scaled is fastest but the noisiest (CV 99.7%)

float_i64_scaled wins on median (124 ns) yet has the highest variance (CV 99.7%), while float_f64 is the steadiest (CV 62.3%, 649 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 5.3x the fastest

Fastest float_i64_scaled (124 ns) to slowest float_f64 (649 ns): 5.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### float_i64_scaled is inconsistent: worst-20% is 3.5x its best-20%

float_i64_scaled's best 20% of batches run at 123 ns but its worst 20% at 426 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### float_f64's edge over baseline is significant but tiny (7 ns, 1.05%)

float_f64 differs from baseline float_f32 by 7 ns (1.05%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: float_i64_scaled** at 123.5 ns median (-80.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.25x (fastest 123.5 ns, slowest 648.6 ns)

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
| float_f32 | 3028ns | 2519ns | 2059ns | 2366ns | 5981ns | base |
| float_f64 | 2931ns | 2526ns | 2066ns | 2373ns | 5469ns | -3.21% |
| float_i64_scaled | 2706ns | 1672ns | 1657ns | 2026ns | 5798ns | -10.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 767ns | 525ns | 1509ns | base | 0.334 |
| float_f64 | 749ns | 531ns | 1389ns | -2.36% | 0.342 |
| float_i64_scaled | 200ns | 123ns | 426ns | -73.93% | 1.279 |

## Performance model

- Peak throughput: **2.075 Gops/s** (float_i64_scaled; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 0.399 | 19.2% |
| float_f64 | 0.395 | 19.0% |
| float_i64_scaled | 2.073 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 3028ns | 3028ns | base |
| float_f64 | 2931ns | 2931ns | -3.21% |
| float_i64_scaled | 2706ns | 2706ns | -10.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 641ns | base | --- | [641, 641] | --- | --- | --- | --- |
| float_f64 | 649ns | +6.8ns (+1.1%) | [+6, +7]ns | [648, 649] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 124ns | -490.6ns (-76.5%) | [-518, -402]ns | [124, 151] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 642ns | -17.2% | -80.8% |
| 1 | 2 | 525ns | +1.2% | -76.5% |
| 1 | 3 | 525ns | +23.6% | -76.5% |
| 1 | 4 | 525ns | +23.7% | -76.5% |
| 1 | 5 | 641ns | +1.3% | -80.7% |
| 1 | 6 | 642ns | -17.2% | -80.7% |
| 1 | 7 | 525ns | +1.2% | -71.2% |
| 1 | 8 | 642ns | +1.1% | -76.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | -0.126 | ok |
| float_f64 | 0.126 | ok |
| float_i64_scaled | 0.459 | moderate+ |

**Consistency summary:**

- **float_f64**: won 2/8, lost 6/8
- **float_i64_scaled**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 29.5ns | 767.4ns | 3.8% |  |
| float_f64 | 28.4ns | 749.3ns | 3.8% |  |
| float_i64_scaled | 32.3ns | 200.1ns | 16.1% | HIGH |

## Distribution (algo ns)

```
float_f32 (n=160, range 524.5-1508.6 ns)
    524.5 |##############################
    573.7 |
    622.9 |########################################
    672.2 |
    721.4 |
    770.6 |
    819.8 |
    869.0 |
    918.2 |
    967.4 |
   1016.6 |
   1065.8 |
   1115.0 |
   1164.2 |
   1213.4 |
   1262.6 |
   1311.8 |
   1361.0 |
   1410.2 |
   1459.4 |
  (13 below, 20 above range)

float_f64 (n=160, range 530.8-1389.1 ns)
    530.8 |############################
    573.7 |
    616.6 |########################################
    659.5 |##
    702.4 |
    745.3 |
    788.2 |
    831.2 |
    874.1 |
    917.0 |
    959.9 |
   1002.8 |
   1045.7 |
   1088.7 |
   1131.6 |
   1174.5 |#
   1217.4 |
   1260.3 |
   1303.2 |
   1346.1 |#
  (16 below, 17 above range)

float_i64_scaled (n=160, range 123.4-426.1 ns)
    123.4 |########################################
    138.5 |###############
    153.6 |#
    168.8 |
    183.9 |
    199.0 |
    214.2 |
    229.3 |
    244.5 |
    259.6 |
    274.7 |
    289.9 |
    305.0 |
    320.1 |#
    335.3 |#
    350.4 |##
    365.5 |
    380.7 |###
    395.8 |##
    410.9 |#
  (6 below, 12 above range)

```

## Diagnostics

- **float_f32**: CV=56.3% (high variance, measurements may be unstable)
- **float_f64**: CV=53.9% (high variance, measurements may be unstable)
- **float_i64_scaled**: CV=61.5% (high variance, measurements may be unstable)
- **float_i64_scaled**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **float_i64_scaled**: bridge=16.0% of algo (FFI overhead may distort results)

# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### float_i64_scaled dominates: 333% faster than the next best (float_f32)

float_i64_scaled (124 ns) leads float_f32 (535 ns) by 333%, a clear separation rather than a photo finish. CV 99.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### float_i64_scaled beats baseline by 75% (significant)

float_i64_scaled is -402 ns (75%) faster than baseline float_f32, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### float_f64 is an outlier: 5.3x slower than the field

float_f64 (649 ns) is 5.3x the fastest (124 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### float_i64_scaled is fastest but the noisiest (CV 99.5%)

float_i64_scaled wins on median (124 ns) yet has the highest variance (CV 99.5%), while float_f64 is the steadiest (CV 62.6%, 649 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 5.3x the fastest

Fastest float_i64_scaled (124 ns) to slowest float_f64 (649 ns): 5.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### float_i64_scaled is inconsistent: worst-20% is 3.5x its best-20%

float_i64_scaled's best 20% of batches run at 123 ns but its worst 20% at 430 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: float_i64_scaled** at 123.5 ns median (-76.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.26x (fastest 123.5 ns, slowest 649.1 ns)

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
| Counter frequency | 2112601374 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| float_f32 | 2969ns | 2079ns | 2065ns | 2316ns | 5833ns | base |
| float_f64 | 3037ns | 2535ns | 2074ns | 2536ns | 5505ns | +2.30% |
| float_i64_scaled | 2636ns | 1676ns | 1663ns | 1889ns | 5847ns | -11.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 729ns | 525ns | 1371ns | base | 0.351 |
| float_f64 | 776ns | 531ns | 1403ns | +6.45% | 0.330 |
| float_i64_scaled | 195ns | 123ns | 430ns | -73.32% | 1.316 |

## Performance model

- Peak throughput: **2.075 Gops/s** (float_i64_scaled; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 0.479 | 23.1% |
| float_f64 | 0.394 | 19.0% |
| float_i64_scaled | 2.073 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 2969ns | 2969ns | base |
| float_f64 | 3037ns | 3037ns | +2.30% |
| float_i64_scaled | 2636ns | 2636ns | -11.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 535ns | base | --- | [525, 641] | --- | --- | --- | --- |
| float_f64 | 649ns | +124.0ns (+23.2%) | [+122, +124]ns | [649, 649] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 124ns | -402.1ns (-75.2%) | [-490, -402]ns | [124, 151] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 525ns | +23.6% | -76.5% |
| 1 | 2 | 525ns | +23.7% | -76.5% |
| 1 | 3 | 760ns | -30.1% | -83.8% |
| 1 | 4 | 525ns | +23.7% | -76.5% |
| 1 | 5 | 642ns | -17.2% | -76.5% |
| 1 | 6 | 642ns | +1.2% | -76.5% |
| 1 | 7 | 525ns | +23.7% | -76.5% |
| 1 | 8 | 525ns | +23.6% | -76.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | -0.313 | moderate- |
| float_f64 | -0.377 | moderate- |
| float_i64_scaled | 0.291 | moderate+ |

**Consistency summary:**

- **float_f64**: won 2/8, lost 6/8
- **float_i64_scaled**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 29.2ns | 729.2ns | 4.0% |  |
| float_f64 | 29.4ns | 776.3ns | 3.8% |  |
| float_i64_scaled | 31.1ns | 194.5ns | 16.0% | HIGH |

## Distribution (algo ns)

```
float_f32 (n=160, range 524.6-1371.2 ns)
    524.6 |########################################
    567.0 |
    609.3 |###################
    651.6 |####
    694.0 |###
    736.3 |#
    778.6 |
    820.9 |
    863.3 |#
    905.6 |
    947.9 |#
    990.3 |
   1032.6 |
   1074.9 |#
   1117.3 |
   1159.6 |
   1201.9 |
   1244.2 |#
   1286.6 |
   1328.9 |
  (18 below, 14 above range)

float_f64 (n=160, range 531.4-1402.6 ns)
    531.4 |#
    575.0 |
    618.6 |########################################
    662.1 |#
    705.7 |
    749.2 |
    792.8 |
    836.4 |
    879.9 |
    923.5 |
    967.0 |
   1010.6 |
   1054.1 |
   1097.7 |
   1141.3 |
   1184.8 |
   1228.4 |
   1271.9 |
   1315.5 |
   1359.0 |
  (27 below, 18 above range)

float_i64_scaled (n=160, range 123.4-429.5 ns)
    123.4 |########################################
    138.7 |################
    154.0 |#
    169.3 |
    184.6 |
    199.9 |
    215.2 |
    230.5 |
    245.8 |
    261.1 |
    276.5 |
    291.8 |
    307.1 |
    322.4 |
    337.7 |
    353.0 |##
    368.3 |#
    383.6 |##
    398.9 |#
    414.2 |##
  (10 below, 14 above range)

```

## Diagnostics

- **float_f32**: CV=54.7% (high variance, measurements may be unstable)
- **float_f64**: CV=52.3% (high variance, measurements may be unstable)
- **float_i64_scaled**: CV=63.2% (high variance, measurements may be unstable)
- **float_i64_scaled**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **float_i64_scaled**: bridge=16.1% of algo (FFI overhead may distort results)

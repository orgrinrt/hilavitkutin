# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (float_f32) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline float_f32 has the worst median (10.41 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest float_i64_scaled at 2.29 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### float_i64_scaled dominates: 278% faster than the next best (float_f64)

float_i64_scaled (2.29 us) leads float_f64 (8.64 us) by 278%, a clear separation rather than a photo finish. CV 34.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### float_i64_scaled beats baseline by 78% (significant)

float_i64_scaled is -8.13 us (78%) faster than baseline float_f32, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### float_f32 is an outlier: 4.6x slower than the field

float_f32 (10.41 us) is 4.6x the fastest (2.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### float_i64_scaled is fastest but the noisiest (CV 34.5%)

float_i64_scaled wins on median (2.29 us) yet has the highest variance (CV 34.5%), while float_f32 is the steadiest (CV 7.9%, 10.41 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 4.6x the fastest

Fastest float_i64_scaled (2.29 us) to slowest float_f32 (10.41 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### float_i64_scaled is inconsistent: worst-20% is 1.8x its best-20%

float_i64_scaled's best 20% of batches run at 1.87 us but its worst 20% at 3.34 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: float_i64_scaled** at 2287.9 ns median (-78.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.55x (fastest 2287.9 ns, slowest 10412.5 ns)

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
| float_f32 | 11755ns | 12304ns | 10064ns | 12120ns | 12349ns | base |
| float_f64 | 11058ns | 10187ns | 10177ns | 10875ns | 12487ns | -5.93% |
| float_i64_scaled | 4193ns | 4162ns | 3407ns | 3818ns | 6105ns | -64.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 9944ns | 8519ns | 10439ns | base | 0.412 |
| float_f64 | 9375ns | 8632ns | 10583ns | -5.72% | 0.437 |
| float_i64_scaled | 2299ns | 1872ns | 3336ns | -76.88% | 1.782 |

## Performance model

- Peak throughput: **2.188 Gops/s** (float_i64_scaled; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 0.393 | 18.0% |
| float_f64 | 0.474 | 21.7% |
| float_i64_scaled | 1.790 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 11755ns | 11755ns | base |
| float_f64 | 11058ns | 11058ns | -5.93% |
| float_i64_scaled | 4193ns | 4193ns | -64.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 10412ns | base | --- | [10412, 10413] | --- | --- | --- | --- |
| float_f64 | 8638ns | +116.3ns (+1.1%) | [+110, +125]ns | [8637, 8704] | YES | 0.0033 | 0.0033 | 0 |
| float_i64_scaled | 2288ns | -8126.1ns (-78.0%) | [-8136, -8125]ns | [1882, 2288] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 10413ns | +1.4% | -82.0% |
| 1 | 2 | 10412ns | -17.1% | -82.0% |
| 1 | 3 | 8520ns | +1.4% | -78.0% |
| 1 | 4 | 10415ns | +1.3% | -78.0% |
| 1 | 5 | 10413ns | -17.1% | -82.0% |
| 1 | 6 | 10413ns | -17.1% | -78.0% |
| 1 | 7 | 10413ns | +1.4% | -78.0% |
| 1 | 8 | 8520ns | +1.4% | -62.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | -0.208 | moderate- |
| float_f64 | -0.475 | moderate- |
| float_i64_scaled | 0.150 | ok |

**Consistency summary:**

- **float_f64**: won 3/8, lost 5/8
- **float_i64_scaled**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 23.5ns | 9943.8ns | 0.2% |  |
| float_f64 | 21.8ns | 9375.0ns | 0.2% |  |
| float_i64_scaled | 24.2ns | 2298.7ns | 1.1% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 8519.1-10438.5 ns)
   8519.1 |###########
   8615.1 |
   8711.1 |
   8807.0 |
   8903.0 |
   8999.0 |
   9095.0 |
   9190.9 |
   9286.9 |
   9382.9 |
   9478.8 |
   9574.8 |
   9670.8 |
   9766.8 |
   9862.7 |
   9958.7 |
  10054.7 |
  10150.6 |
  10246.6 |
  10342.6 |########################################
  (5 below, 2 above range)

float_f64 (n=160, range 8631.8-10583.1 ns)
   8631.8 |########################################
   8729.4 |#
   8826.9 |
   8924.5 |
   9022.1 |
   9119.6 |
   9217.2 |
   9314.8 |
   9412.3 |
   9509.9 |
   9607.5 |
   9705.0 |
   9802.6 |
   9900.2 |
   9997.7 |
  10095.3 |
  10192.9 |
  10290.4 |
  10388.0 |
  10485.6 |##########################
  (11 below, 5 above range)

float_i64_scaled (n=160, range 1872.0-3336.0 ns)
   1872.0 |########################################
   1945.2 |
   2018.4 |
   2091.6 |
   2164.8 |
   2238.0 |########################################
   2311.2 |#
   2384.4 |
   2457.6 |
   2530.8 |
   2604.0 |
   2677.2 |
   2750.4 |
   2823.6 |
   2896.8 |
   2970.0 |
   3043.2 |
   3116.4 |
   3189.6 |
   3262.8 |
  (15 below, 12 above range)

```

## Diagnostics

- **float_i64_scaled**: CV=34.3% (high variance, measurements may be unstable)

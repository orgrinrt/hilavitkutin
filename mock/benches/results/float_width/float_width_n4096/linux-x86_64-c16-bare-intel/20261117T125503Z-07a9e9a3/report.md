# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### float_i64_scaled dominates: 355% faster than the next best (float_f32)

float_i64_scaled (2.29 us) leads float_f32 (10.41 us) by 355%, a clear separation rather than a photo finish. CV 11.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### float_i64_scaled beats baseline by 78% (significant)

float_i64_scaled is -8.12 us (78%) faster than baseline float_f32, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### float_f64 is an outlier: 4.6x slower than the field

float_f64 (10.55 us) is 4.6x the fastest (2.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### float_i64_scaled is fastest but the noisiest (CV 11.6%)

float_i64_scaled wins on median (2.29 us) yet has the highest variance (CV 11.6%), while float_f32 is the steadiest (CV 7.9%, 10.41 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 4.6x the fastest

Fastest float_i64_scaled (2.29 us) to slowest float_f64 (10.55 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: float_i64_scaled** at 2288.1 ns median (-78.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.61x (fastest 2288.1 ns, slowest 10554.0 ns)

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
| float_f32 | 11756ns | 12306ns | 10069ns | 12124ns | 12338ns | base |
| float_f64 | 11609ns | 12444ns | 10183ns | 11792ns | 12488ns | -1.25% |
| float_i64_scaled | 3943ns | 4172ns | 3414ns | 3973ns | 4383ns | -66.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 9941ns | 8517ns | 10422ns | base | 0.412 |
| float_f64 | 9838ns | 8633ns | 10578ns | -1.03% | 0.416 |
| float_i64_scaled | 2158ns | 1872ns | 2390ns | -78.29% | 1.898 |

## Performance model

- Peak throughput: **2.188 Gops/s** (float_i64_scaled; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 0.393 | 18.0% |
| float_f64 | 0.388 | 17.7% |
| float_i64_scaled | 1.790 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 11756ns | 11756ns | base |
| float_f64 | 11609ns | 11609ns | -1.25% |
| float_i64_scaled | 3943ns | 3943ns | -66.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 10413ns | base | --- | [10412, 10413] | --- | --- | --- | --- |
| float_f64 | 10554ns | +140.9ns (+1.4%) | [+130, +142]ns | [10543, 10554] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 2288ns | -8124.7ns (-78.0%) | [-8125, -8124]ns | [2288, 2288] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 10414ns | +1.4% | -78.0% |
| 1 | 2 | 8520ns | +23.9% | -73.1% |
| 1 | 3 | 10413ns | -17.1% | -78.0% |
| 1 | 4 | 10412ns | +1.4% | -82.0% |
| 1 | 5 | 10413ns | -17.1% | -78.0% |
| 1 | 6 | 8520ns | +1.4% | -78.0% |
| 1 | 7 | 10413ns | +1.4% | -82.0% |
| 1 | 8 | 10413ns | +1.4% | -78.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | -0.375 | moderate- |
| float_f64 | -0.142 | ok |
| float_i64_scaled | -0.142 | ok |

**Consistency summary:**

- **float_f64**: won 2/8, lost 6/8
- **float_i64_scaled**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 23.7ns | 9940.7ns | 0.2% |  |
| float_f64 | 23.2ns | 9838.3ns | 0.2% |  |
| float_i64_scaled | 22.6ns | 2158.3ns | 1.0% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 8517.4-10421.8 ns)
   8517.4 |############
   8612.6 |
   8707.8 |
   8803.1 |
   8898.3 |
   8993.5 |
   9088.7 |
   9183.9 |
   9279.2 |
   9374.4 |
   9469.6 |
   9564.8 |
   9660.0 |
   9755.3 |
   9850.5 |
   9945.7 |
  10040.9 |
  10136.2 |
  10231.4 |
  10326.6 |########################################
  (9 below, 22 above range)

float_f64 (n=160, range 8633.0-10577.7 ns)
   8633.0 |####################
   8730.2 |
   8827.4 |
   8924.7 |
   9021.9 |
   9119.1 |
   9216.4 |
   9313.6 |
   9410.9 |
   9508.1 |
   9605.3 |
   9702.6 |
   9799.8 |
   9897.0 |
   9994.3 |
  10091.5 |
  10188.7 |
  10286.0 |
  10383.2 |
  10480.4 |########################################
  (9 below, 2 above range)

float_i64_scaled (n=160, range 1872.1-2390.0 ns)
   1872.1 |###################
   1898.0 |
   1923.9 |
   1949.8 |
   1975.7 |
   2001.6 |
   2027.5 |
   2053.3 |
   2079.2 |
   2105.1 |
   2131.0 |
   2156.9 |
   2182.8 |
   2208.7 |
   2234.6 |
   2260.5 |
   2286.4 |########################################
   2312.3 |
   2338.2 |
   2364.1 |
  (10 below, 3 above range)

```

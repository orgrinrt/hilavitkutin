# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### float_i64_scaled dominates: 355% faster than the next best (float_f32)

float_i64_scaled (2.29 us) leads float_f32 (10.41 us) by 355%, a clear separation rather than a photo finish. CV 14.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### float_i64_scaled beats baseline by 78% (significant)

float_i64_scaled is -8.12 us (78%) faster than baseline float_f32, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### float_f64 is an outlier: 4.6x slower than the field

float_f64 (10.55 us) is 4.6x the fastest (2.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### float_i64_scaled is fastest but the noisiest (CV 14.7%)

float_i64_scaled wins on median (2.29 us) yet has the highest variance (CV 14.7%), while float_f32 is the steadiest (CV 6.0%, 10.41 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 4.6x the fastest

Fastest float_i64_scaled (2.29 us) to slowest float_f64 (10.55 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: float_i64_scaled** at 2288.1 ns median (-78.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.61x (fastest 2288.1 ns, slowest 10554.2 ns)

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
| float_f32 | 12031ns | 12303ns | 10907ns | 12303ns | 12339ns | base |
| float_f64 | 12174ns | 12442ns | 11030ns | 12442ns | 12517ns | +1.19% |
| float_i64_scaled | 3953ns | 4168ns | 3409ns | 3967ns | 4456ns | -67.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 10177ns | 9225ns | 10425ns | base | 0.402 |
| float_f64 | 10323ns | 9350ns | 10605ns | +1.43% | 0.397 |
| float_i64_scaled | 2167ns | 1872ns | 2437ns | -78.70% | 1.890 |

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
| float_f32 | 12031ns | 12031ns | base |
| float_f64 | 12174ns | 12174ns | +1.19% |
| float_i64_scaled | 3953ns | 3953ns | -67.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 10413ns | base | --- | [10413, 10413] | --- | --- | --- | --- |
| float_f64 | 10554ns | +141.4ns (+1.4%) | [+140, +142]ns | [10554, 10555] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 2288ns | -8124.6ns (-78.0%) | [-8125, -8124]ns | [2288, 2288] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 10413ns | +1.3% | -78.0% |
| 1 | 2 | 8520ns | +23.9% | -78.0% |
| 1 | 3 | 10413ns | +1.4% | -78.0% |
| 1 | 4 | 10413ns | +1.4% | -78.0% |
| 1 | 5 | 10415ns | +1.3% | -82.0% |
| 1 | 6 | 10412ns | +1.4% | -78.0% |
| 1 | 7 | 10414ns | +1.4% | -78.0% |
| 1 | 8 | 10414ns | -17.1% | -82.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | -0.160 | ok |
| float_f64 | -0.018 | ok |
| float_i64_scaled | -0.475 | moderate- |

**Consistency summary:**

- **float_f64**: won 1/8, lost 7/8
- **float_i64_scaled**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 24.0ns | 10177.3ns | 0.2% |  |
| float_f64 | 24.0ns | 10323.3ns | 0.2% |  |
| float_i64_scaled | 22.6ns | 2167.4ns | 1.0% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 9225.1-10425.2 ns)
   9225.1 |
   9285.1 |
   9345.1 |
   9405.1 |
   9465.1 |
   9525.1 |
   9585.2 |
   9645.2 |
   9705.2 |
   9765.2 |
   9825.2 |
   9885.2 |
   9945.2 |
  10005.2 |
  10065.2 |
  10125.2 |
  10185.2 |
  10245.2 |
  10305.2 |
  10365.2 |########################################
  (20 below, 8 above range)

float_f64 (n=160, range 9349.7-10605.4 ns)
   9349.7 |
   9412.5 |
   9475.2 |
   9538.0 |
   9600.8 |
   9663.6 |
   9726.4 |
   9789.2 |
   9851.9 |
   9914.7 |
   9977.5 |
  10040.3 |
  10103.1 |
  10165.9 |
  10228.7 |
  10291.4 |
  10354.2 |
  10417.0 |
  10479.8 |##
  10542.6 |########################################
  (20 below, 3 above range)

float_i64_scaled (n=160, range 1872.1-2437.1 ns)
   1872.1 |####################
   1900.4 |
   1928.6 |
   1956.9 |
   1985.1 |
   2013.4 |
   2041.6 |
   2069.9 |
   2098.1 |
   2126.4 |
   2154.6 |
   2182.9 |
   2211.1 |
   2239.4 |
   2267.6 |########################################
   2295.9 |##############
   2324.1 |
   2352.4 |
   2380.6 |
   2408.9 |
  (21 below, 4 above range)

```

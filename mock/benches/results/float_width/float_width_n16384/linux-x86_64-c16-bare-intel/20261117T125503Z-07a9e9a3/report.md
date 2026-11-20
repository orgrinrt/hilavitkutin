# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### float_i64_scaled dominates: 343% faster than the next best (float_f32)

float_i64_scaled (9.14 us) leads float_f32 (40.51 us) by 343%, a clear separation rather than a photo finish. CV 8.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### float_i64_scaled beats baseline by 77% (significant)

float_i64_scaled is -31.37 us (77%) faster than baseline float_f32, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### float_f64 is an outlier: 4.4x slower than the field

float_f64 (40.51 us) is 4.4x the fastest (9.14 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### float_i64_scaled is fastest but the noisiest (CV 8.8%)

float_i64_scaled wins on median (9.14 us) yet has the highest variance (CV 8.8%), while float_f32 is the steadiest (CV 7.9%, 40.51 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 4.4x the fastest

Fastest float_i64_scaled (9.14 us) to slowest float_f64 (40.51 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: float_i64_scaled** at 9138.7 ns median (-77.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.43x (fastest 9138.7 ns, slowest 40512.6 ns)

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
| float_f32 | 40498ns | 42406ns | 34686ns | 41764ns | 42510ns | base |
| float_f64 | 39751ns | 42407ns | 34695ns | 40509ns | 42531ns | -1.84% |
| float_i64_scaled | 10281ns | 11023ns | 9017ns | 10444ns | 11056ns | -74.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 38682ns | 33133ns | 40599ns | base | 0.424 |
| float_f64 | 37969ns | 33141ns | 40618ns | -1.84% | 0.432 |
| float_i64_scaled | 8517ns | 7472ns | 9152ns | -77.98% | 1.924 |

## Performance model

- Peak throughput: **2.193 Gops/s** (float_i64_scaled; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 0.404 | 18.4% |
| float_f64 | 0.404 | 18.4% |
| float_i64_scaled | 1.793 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 40498ns | 40498ns | base |
| float_f64 | 39751ns | 39751ns | -1.84% |
| float_i64_scaled | 10281ns | 10281ns | -74.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 40507ns | base | --- | [40502, 40511] | --- | --- | --- | --- |
| float_f64 | 40513ns | no significant difference | [-8, +9]ns | [40508, 40513] | no | 1.0000 | 1.0000 | 0 |
| float_i64_scaled | 9139ns | -31369.3ns (-77.4%) | [-31378, -31363]ns | [9128, 9139] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 33137ns | +0.0% | -72.4% |
| 1 | 2 | 40523ns | -0.0% | -81.5% |
| 1 | 3 | 40505ns | +0.0% | -77.4% |
| 1 | 4 | 40510ns | +0.0% | -77.4% |
| 1 | 5 | 33141ns | +22.3% | -77.4% |
| 1 | 6 | 40511ns | -18.2% | -77.4% |
| 1 | 7 | 40517ns | -0.0% | -77.4% |
| 1 | 8 | 40515ns | -17.3% | -81.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | -0.209 | moderate- |
| float_f64 | -0.278 | moderate- |
| float_i64_scaled | -0.475 | moderate- |

**Consistency summary:**

- **float_f64**: won 2/8, lost 1/8
- **float_i64_scaled**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 23.8ns | 38681.7ns | 0.1% |  |
| float_f64 | 23.3ns | 37969.0ns | 0.1% |  |
| float_i64_scaled | 22.6ns | 8516.9ns | 0.3% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 33132.6-40599.1 ns)
  33132.6 |########
  33506.0 |
  33879.3 |
  34252.6 |
  34625.9 |
  34999.2 |
  35372.6 |
  35745.9 |
  36119.2 |
  36492.5 |
  36865.8 |
  37239.2 |
  37612.5 |
  37985.8 |
  38359.1 |
  38732.5 |
  39105.8 |
  39479.1 |
  39852.4 |
  40225.7 |########################################
  (15 below, 6 above range)

float_f64 (n=160, range 33140.6-40618.0 ns)
  33140.6 |##############
  33514.4 |#
  33888.3 |
  34262.2 |
  34636.1 |
  35009.9 |
  35383.8 |
  35757.7 |
  36131.6 |
  36505.4 |
  36879.3 |
  37253.2 |
  37627.0 |
  38000.9 |
  38374.8 |
  38748.7 |
  39122.5 |
  39496.4 |
  39870.3 |
  40244.2 |########################################
  (16 below, 8 above range)

float_i64_scaled (n=160, range 7472.4-9152.3 ns)
   7472.4 |##################
   7556.4 |
   7640.4 |
   7724.4 |
   7808.4 |
   7892.4 |
   7976.4 |
   8060.4 |
   8144.4 |
   8228.4 |
   8312.4 |
   8396.4 |
   8480.3 |
   8564.3 |
   8648.3 |
   8732.3 |
   8816.3 |
   8900.3 |
   8984.3 |
   9068.3 |########################################
  (17 below, 5 above range)

```

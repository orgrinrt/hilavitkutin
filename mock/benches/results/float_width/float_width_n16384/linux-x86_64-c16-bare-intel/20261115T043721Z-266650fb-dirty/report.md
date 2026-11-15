# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### float_i64_scaled dominates: 343% faster than the next best (float_f32)

float_i64_scaled (9.14 us) leads float_f32 (40.51 us) by 343%, a clear separation rather than a photo finish. CV 7.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### float_i64_scaled beats baseline by 77% (significant)

float_i64_scaled is -31.37 us (77%) faster than baseline float_f32, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### float_f64 is an outlier: 4.4x slower than the field

float_f64 (40.51 us) is 4.4x the fastest (9.14 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### float_i64_scaled is fastest but the noisiest (CV 7.9%)

float_i64_scaled wins on median (9.14 us) yet has the highest variance (CV 7.9%), while float_f32 is the steadiest (CV 0.4%, 40.51 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 4.4x the fastest

Fastest float_i64_scaled (9.14 us) to slowest float_f64 (40.51 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: float_i64_scaled** at 9138.8 ns median (-77.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.43x (fastest 9138.8 ns, slowest 40512.4 ns)

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
| float_f32 | 42437ns | 42401ns | 42382ns | 42401ns | 42598ns | base |
| float_f64 | 40516ns | 42403ns | 34695ns | 41799ns | 42487ns | -4.53% |
| float_i64_scaled | 10529ns | 11019ns | 9016ns | 10859ns | 11053ns | -75.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 40536ns | 40487ns | 40674ns | base | 0.404 |
| float_f64 | 38697ns | 33141ns | 40552ns | -4.54% | 0.423 |
| float_i64_scaled | 8727ns | 7475ns | 9150ns | -78.47% | 1.877 |

## Performance model

- Peak throughput: **2.192 Gops/s** (float_i64_scaled; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 0.404 | 18.5% |
| float_f64 | 0.404 | 18.5% |
| float_i64_scaled | 1.793 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 42437ns | 42437ns | base |
| float_f64 | 40516ns | 40516ns | -4.53% |
| float_i64_scaled | 10529ns | 10529ns | -75.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 40506ns | base | --- | [40504, 40508] | --- | --- | --- | --- |
| float_f64 | 40512ns | no significant difference | [-3, +8]ns | [40511, 40513] | no | 0.3846 | 0.3846 | 0 |
| float_i64_scaled | 9139ns | -31373.0ns (-77.5%) | [-31378, -31369]ns | [9139, 9139] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 40508ns | -18.2% | -81.5% |
| 1 | 2 | 40515ns | +0.0% | -77.4% |
| 1 | 3 | 40505ns | +0.0% | -77.4% |
| 1 | 4 | 40503ns | +0.0% | -77.4% |
| 1 | 5 | 40502ns | +0.0% | -81.5% |
| 1 | 6 | 40504ns | -18.2% | -77.4% |
| 1 | 7 | 40506ns | +0.0% | -77.4% |
| 1 | 8 | 40506ns | +0.0% | -77.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | 0.231 | moderate+ |
| float_f64 | -0.208 | moderate- |
| float_i64_scaled | -0.208 | moderate- |

**Consistency summary:**

- **float_f64**: won 2/8, lost 0/8
- **float_i64_scaled**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 24.9ns | 40535.7ns | 0.1% |  |
| float_f64 | 23.6ns | 38697.2ns | 0.1% |  |
| float_i64_scaled | 23.1ns | 8726.6ns | 0.3% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 40486.8-40674.3 ns)
  40486.8 |############################
  40496.2 |########################################
  40505.6 |###################################
  40515.0 |###########################
  40524.3 |##########
  40533.7 |#
  40543.1 |
  40552.4 |
  40561.8 |
  40571.2 |#
  40580.6 |
  40589.9 |
  40599.3 |
  40608.7 |
  40618.1 |
  40627.4 |
  40636.8 |
  40646.2 |
  40655.5 |
  40664.9 |
  (13 below, 6 above range)

float_f64 (n=160, range 33140.6-40552.0 ns)
  33140.6 |#######
  33511.2 |
  33881.7 |
  34252.3 |
  34622.9 |
  34993.4 |
  35364.0 |
  35734.6 |
  36105.1 |
  36475.7 |
  36846.3 |
  37216.9 |
  37587.4 |
  37958.0 |
  38328.6 |
  38699.1 |
  39069.7 |
  39440.3 |
  39810.8 |
  40181.4 |########################################
  (18 below, 6 above range)

float_i64_scaled (n=160, range 7474.7-9150.1 ns)
   7474.7 |##########
   7558.4 |
   7642.2 |
   7726.0 |
   7809.7 |
   7893.5 |
   7977.3 |
   8061.1 |
   8144.8 |
   8228.6 |
   8312.4 |
   8396.1 |
   8479.9 |
   8563.7 |
   8647.4 |
   8731.2 |
   8815.0 |
   8898.8 |
   8982.5 |
   9066.3 |########################################
  (9 below, 8 above range)

```

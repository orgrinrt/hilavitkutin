# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (float_f32) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline float_f32 has the worst median (2.57 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest float_i64_scaled at 578 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### float_i64_scaled dominates: 338% faster than the next best (float_f64)

float_i64_scaled (578 ns) leads float_f64 (2.53 us) by 338%, a clear separation rather than a photo finish. CV 66.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### float_i64_scaled beats baseline by 77% (significant)

float_i64_scaled is -1.99 us (77%) faster than baseline float_f32, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### float_f32 is an outlier: 4.4x slower than the field

float_f32 (2.57 us) is 4.4x the fastest (578 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### float_i64_scaled is fastest but the noisiest (CV 66.2%)

float_i64_scaled wins on median (578 ns) yet has the highest variance (CV 66.2%), while float_f32 is the steadiest (CV 3.0%, 2.57 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 4.4x the fastest

Fastest float_i64_scaled (578 ns) to slowest float_f32 (2.57 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### float_i64_scaled is inconsistent: worst-20% is 2.8x its best-20%

float_i64_scaled's best 20% of batches run at 477 ns but its worst 20% at 1.34 us (2.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### float_f64's edge over baseline is significant but tiny (-36 ns, 1.42%)

float_f64 differs from baseline float_f32 by -36 ns (1.42%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: float_i64_scaled** at 577.9 ns median (-77.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.44x (fastest 577.9 ns, slowest 2566.1 ns)

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
| float_f32 | 4476ns | 4460ns | 4450ns | 4461ns | 4548ns | base |
| float_f64 | 4277ns | 4420ns | 3616ns | 4364ns | 4675ns | -4.46% |
| float_i64_scaled | 3022ns | 2454ns | 2019ns | 2455ns | 5725ns | -32.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 2575ns | 2565ns | 2608ns | base | 0.398 |
| float_f64 | 2445ns | 2071ns | 2664ns | -5.04% | 0.419 |
| float_i64_scaled | 710ns | 477ns | 1337ns | -72.43% | 1.443 |

## Performance model

- Peak throughput: **2.145 Gops/s** (float_i64_scaled; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 0.399 | 18.6% |
| float_f64 | 0.405 | 18.9% |
| float_i64_scaled | 1.772 | 82.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 4476ns | 4476ns | base |
| float_f64 | 4277ns | 4277ns | -4.46% |
| float_i64_scaled | 3022ns | 3022ns | -32.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 2566ns | base | --- | [2566, 2566] | --- | --- | --- | --- |
| float_f64 | 2530ns | -36.3ns (-1.4%) | [-40, -36]ns | [2530, 2530] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 578ns | -1988.0ns (-77.5%) | [-1989, -1988]ns | [578, 578] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 2566ns | -1.4% | -77.5% |
| 1 | 2 | 2566ns | -1.4% | -81.6% |
| 1 | 3 | 2566ns | -1.4% | -77.5% |
| 1 | 4 | 2566ns | -1.4% | -77.5% |
| 1 | 5 | 2566ns | -19.3% | -81.6% |
| 1 | 6 | 2566ns | -1.4% | -77.5% |
| 1 | 7 | 2566ns | -1.4% | -77.5% |
| 1 | 8 | 2566ns | -19.3% | -77.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | -0.097 | ok |
| float_f64 | -0.209 | moderate- |
| float_i64_scaled | -0.374 | moderate- |

**Consistency summary:**

- **float_f64**: won 8/8, lost 0/8
- **float_i64_scaled**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 25.1ns | 2575.0ns | 1.0% |  |
| float_f64 | 24.0ns | 2445.3ns | 1.0% |  |
| float_i64_scaled | 29.8ns | 709.9ns | 4.2% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 2565.3-2608.5 ns)
   2565.3 |########################################
   2567.5 |
   2569.6 |
   2571.8 |
   2573.9 |
   2576.1 |#########
   2578.3 |####
   2580.4 |
   2582.6 |
   2584.7 |
   2586.9 |
   2589.1 |
   2591.2 |
   2593.4 |
   2595.5 |
   2597.7 |
   2599.9 |
   2602.0 |
   2604.2 |
   2606.3 |
  (16 below, 1 above range)

float_f64 (n=160, range 2070.7-2664.1 ns)
   2070.7 |#####
   2100.4 |
   2130.1 |
   2159.7 |
   2189.4 |
   2219.1 |
   2248.7 |
   2278.4 |
   2308.1 |
   2337.7 |
   2367.4 |
   2397.1 |
   2426.7 |
   2456.4 |
   2486.1 |
   2515.7 |########################################
   2545.4 |#
   2575.1 |
   2604.7 |
   2634.4 |
  (24 below, 2 above range)

float_i64_scaled (n=160, range 477.4-1337.4 ns)
    477.4 |#
    520.4 |
    563.4 |########################################
    606.4 |
    649.4 |
    692.4 |
    735.4 |
    778.4 |
    821.4 |
    864.4 |
    907.4 |
    950.4 |
    993.4 |
   1036.4 |
   1079.4 |
   1122.4 |
   1165.4 |
   1208.4 |
   1251.4 |
   1294.4 |
  (27 below, 21 above range)

```

## Diagnostics

- **float_i64_scaled**: CV=53.9% (high variance, measurements may be unstable)

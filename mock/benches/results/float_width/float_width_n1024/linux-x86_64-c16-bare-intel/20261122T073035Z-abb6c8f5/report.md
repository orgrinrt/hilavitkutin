# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (float_f32) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline float_f32 has the worst median (2.56 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest float_i64_scaled at 473 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### float_i64_scaled dominates: 435% faster than the next best (float_f64)

float_i64_scaled (473 ns) leads float_f64 (2.53 us) by 435%, a clear separation rather than a photo finish. CV 81.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### float_i64_scaled beats baseline by 63% (significant)

float_i64_scaled is -1.63 us (63%) faster than baseline float_f32, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### float_f32 is an outlier: 5.4x slower than the field

float_f32 (2.56 us) is 5.4x the fastest (473 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### float_i64_scaled is fastest but the noisiest (CV 81.7%)

float_i64_scaled wins on median (473 ns) yet has the highest variance (CV 81.7%), while float_f32 is the steadiest (CV 10.0%, 2.56 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 5.4x the fastest

Fastest float_i64_scaled (473 ns) to slowest float_f32 (2.56 us): 5.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### float_i64_scaled is inconsistent: worst-20% is 2.7x its best-20%

float_i64_scaled's best 20% of batches run at 473 ns but its worst 20% at 1.27 us (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### float_f64's edge over baseline is significant but tiny (-29 ns, 1.14%)

float_f64 differs from baseline float_f32 by -29 ns (1.14%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: float_i64_scaled** at 473.2 ns median (-81.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.42x (fastest 473.2 ns, slowest 2564.8 ns)

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
| float_f32 | 4088ns | 4443ns | 3643ns | 4073ns | 4576ns | base |
| float_f64 | 4364ns | 4423ns | 3615ns | 4245ns | 5472ns | +6.77% |
| float_i64_scaled | 2812ns | 2022ns | 2007ns | 2208ns | 5428ns | -31.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 2351ns | 2099ns | 2624ns | base | 0.436 |
| float_f64 | 2494ns | 2069ns | 3121ns | +6.06% | 0.411 |
| float_i64_scaled | 659ns | 473ns | 1265ns | -71.96% | 1.553 |

## Performance model

- Peak throughput: **2.166 Gops/s** (float_i64_scaled; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 0.399 | 18.4% |
| float_f64 | 0.405 | 18.7% |
| float_i64_scaled | 2.164 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 4088ns | 4088ns | base |
| float_f64 | 4364ns | 4364ns | +6.77% |
| float_i64_scaled | 2812ns | 2812ns | -31.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 2565ns | base | --- | [2109, 2565] | --- | --- | --- | --- |
| float_f64 | 2530ns | -29.2ns (-1.1%) | [-35, -18]ns | [2530, 2530] | YES | 0.0478 | 0.0478 | 0 |
| float_i64_scaled | 473ns | -1627.4ns (-63.5%) | [-1728, -1627]ns | [473, 578] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 2566ns | -1.4% | -81.6% |
| 1 | 2 | 2100ns | -1.4% | -77.5% |
| 1 | 3 | 2566ns | -19.3% | -81.6% |
| 1 | 4 | 2566ns | -19.3% | -77.5% |
| 1 | 5 | 2566ns | -1.4% | -77.5% |
| 1 | 6 | 2100ns | +20.5% | -72.5% |
| 1 | 7 | 2099ns | +20.6% | -77.5% |
| 1 | 8 | 2100ns | +20.6% | -77.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | 0.126 | ok |
| float_f64 | 0.392 | moderate+ |
| float_i64_scaled | 0.392 | moderate+ |

**Consistency summary:**

- **float_f64**: won 5/8, lost 3/8
- **float_i64_scaled**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 22.6ns | 2351.2ns | 1.0% |  |
| float_f64 | 24.3ns | 2493.6ns | 1.0% |  |
| float_i64_scaled | 27.5ns | 659.2ns | 4.2% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 2099.0-2623.6 ns)
   2099.0 |########################################
   2125.3 |
   2151.5 |
   2177.7 |
   2204.0 |
   2230.2 |
   2256.4 |
   2282.7 |
   2308.9 |
   2335.1 |
   2361.3 |
   2387.6 |
   2413.8 |
   2440.0 |
   2466.3 |
   2492.5 |
   2518.7 |
   2545.0 |########################################
   2571.2 |############
   2597.4 |
  (17 below, 3 above range)

float_f64 (n=160, range 2069.3-3121.3 ns)
   2069.3 |####################
   2121.9 |
   2174.5 |
   2227.1 |
   2279.7 |
   2332.3 |
   2384.9 |
   2437.5 |
   2490.1 |########################################
   2542.7 |##
   2595.3 |
   2647.9 |
   2700.5 |
   2753.1 |
   2805.7 |
   2858.3 |
   2910.9 |
   2963.5 |
   3016.1 |
   3068.7 |
  (7 below, 6 above range)

float_i64_scaled (n=160, range 472.7-1265.4 ns)
    472.7 |########################################
    512.4 |
    552.0 |#############################
    591.6 |#
    631.3 |
    670.9 |
    710.5 |
    750.2 |
    789.8 |
    829.4 |
    869.1 |
    908.7 |
    948.3 |
    988.0 |
   1027.6 |
   1067.2 |
   1106.9 |
   1146.5 |
   1186.2 |
   1225.8 |
  (20 below, 18 above range)

```

## Diagnostics

- **float_f64**: CV=28.2% (high variance, measurements may be unstable)
- **float_i64_scaled**: CV=58.7% (high variance, measurements may be unstable)

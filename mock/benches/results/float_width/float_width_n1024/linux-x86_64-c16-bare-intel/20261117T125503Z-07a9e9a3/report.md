# Float width strategy: f32 (FastFloat) vs f64 (StrictFloat) vs i64-fixed-point (UFixed Hot)

3 variants, 160 samples per variant.
Baseline: **float_f32**

## Highlights

Baseline for all deltas below: **float_f32**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (float_f32) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline float_f32 has the worst median (2.57 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest float_i64_scaled at 578 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### float_i64_scaled dominates: 260% faster than the next best (float_f64)

float_i64_scaled (578 ns) leads float_f64 (2.08 us) by 260%, a clear separation rather than a photo finish. CV 65.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### float_i64_scaled beats baseline by 77% (significant)

float_i64_scaled is -1.99 us (77%) faster than baseline float_f32, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### float_f32 is an outlier: 4.4x slower than the field

float_f32 (2.57 us) is 4.4x the fastest (578 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### float_i64_scaled is fastest but the noisiest (CV 65.2%)

float_i64_scaled wins on median (578 ns) yet has the highest variance (CV 65.2%), while float_f32 is the steadiest (CV 7.9%, 2.57 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 4.4x the fastest

Fastest float_i64_scaled (578 ns) to slowest float_f32 (2.57 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### float_i64_scaled is inconsistent: worst-20% is 2.8x its best-20%

float_i64_scaled's best 20% of batches run at 473 ns but its worst 20% at 1.32 us (2.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### float_f64's edge over baseline is significant but tiny (-37 ns, 1.44%)

float_f64 differs from baseline float_f32 by -37 ns (1.44%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: float_i64_scaled** at 578.0 ns median (-77.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.44x (fastest 578.0 ns, slowest 2566.4 ns)

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
| float_f32 | 4269ns | 4462ns | 3654ns | 4399ns | 4496ns | base |
| float_f64 | 3964ns | 3631ns | 3619ns | 3876ns | 4576ns | -7.14% |
| float_i64_scaled | 2955ns | 2463ns | 2016ns | 2392ns | 5584ns | -30.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| float_f32 | 2453ns | 2100ns | 2579ns | base | 0.417 |
| float_f64 | 2263ns | 2069ns | 2605ns | -7.75% | 0.452 |
| float_i64_scaled | 695ns | 473ns | 1317ns | -71.67% | 1.473 |

## Performance model

- Peak throughput: **2.166 Gops/s** (float_i64_scaled; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| float_f32 | 0.399 | 18.4% |
| float_f64 | 0.493 | 22.7% |
| float_i64_scaled | 1.772 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| float_f32 | 4269ns | 4269ns | base |
| float_f64 | 3964ns | 3964ns | -7.14% |
| float_i64_scaled | 2955ns | 2955ns | -30.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| float_f32 | 2566ns | base | --- | [2566, 2567] | --- | --- | --- | --- |
| float_f64 | 2079ns | -36.8ns (-1.4%) | [-47, -36]ns | [2071, 2081] | YES | 0.0000 | 0.0000 | 0 |
| float_i64_scaled | 578ns | -1988.0ns (-77.5%) | [-1989, -1987]ns | [578, 578] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | float_f32 | float_f64 | float_i64_scaled |
|---|---|---|---|---|
| 1 | 1 | 2566ns | -19.3% | -77.5% |
| 1 | 2 | 2568ns | -19.4% | -77.5% |
| 1 | 3 | 2567ns | -1.4% | -77.5% |
| 1 | 4 | 2567ns | -1.4% | -77.5% |
| 1 | 5 | 2100ns | -1.4% | -77.5% |
| 1 | 6 | 2567ns | -1.4% | -81.6% |
| 1 | 7 | 2566ns | -19.3% | -81.6% |
| 1 | 8 | 2100ns | -1.4% | -72.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| float_f32 | -0.207 | moderate- |
| float_f64 | -0.142 | ok |
| float_i64_scaled | 0.391 | moderate+ |

**Consistency summary:**

- **float_f64**: won 8/8, lost 0/8
- **float_i64_scaled**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| float_f32 | 23.8ns | 2453.2ns | 1.0% |  |
| float_f64 | 22.2ns | 2263.1ns | 1.0% |  |
| float_i64_scaled | 28.8ns | 695.1ns | 4.1% |  |

## Distribution (algo ns)

```
float_f32 (n=160, range 2100.5-2579.4 ns)
   2100.5 |####
   2124.4 |
   2148.4 |
   2172.3 |
   2196.3 |
   2220.2 |
   2244.2 |
   2268.1 |
   2292.1 |
   2316.0 |
   2340.0 |
   2363.9 |
   2387.9 |
   2411.8 |
   2435.7 |
   2459.7 |
   2483.6 |
   2507.6 |
   2531.5 |
   2555.5 |########################################
  (27 below, 9 above range)

float_f64 (n=160, range 2069.4-2604.8 ns)
   2069.4 |########################################
   2096.2 |
   2123.0 |
   2149.7 |
   2176.5 |
   2203.3 |
   2230.0 |
   2256.8 |
   2283.6 |
   2310.3 |
   2337.1 |
   2363.9 |
   2390.7 |
   2417.4 |
   2444.2 |
   2471.0 |
   2497.7 |
   2524.5 |########################
   2551.3 |
   2578.0 |
  (4 below, 2 above range)

float_i64_scaled (n=160, range 472.8-1317.3 ns)
    472.8 |#############
    515.1 |
    557.3 |########################################
    599.5 |
    641.7 |
    683.9 |
    726.2 |
    768.4 |
    810.6 |
    852.8 |
    895.1 |
    937.3 |
    979.5 |
   1021.7 |
   1064.0 |
   1106.2 |
   1148.4 |
   1190.6 |
   1232.9 |
   1275.1 |
  (19 below, 20 above range)

```

## Diagnostics

- **float_i64_scaled**: CV=54.2% (high variance, measurements may be unstable)

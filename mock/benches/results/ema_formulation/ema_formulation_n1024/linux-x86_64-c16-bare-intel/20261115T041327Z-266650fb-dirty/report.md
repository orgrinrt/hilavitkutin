# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (2.00 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 863 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 66% faster than the next best (ema_form_q32)

ema_form_intshift (863 ns) leads ema_form_q32 (1.43 us) by 66%, a clear separation rather than a photo finish. CV 60.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 57% (significant)

ema_form_intshift is -1.14 us (57%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_form_float is an outlier: 2.3x slower than the field

ema_form_float (2.00 us) is 2.3x the fastest (863 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ema_form_intshift is fastest but the noisiest (CV 60.9%)

ema_form_intshift wins on median (863 ns) yet has the highest variance (CV 60.9%), while ema_form_float is the steadiest (CV 28.1%, 2.00 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### ema_form_intshift is inconsistent: worst-20% is 2.3x its best-20%

ema_form_intshift's best 20% of batches run at 780 ns but its worst 20% at 1.78 us (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ema_form_intshift** at 863.1 ns median (-56.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.32x (fastest 863.1 ns, slowest 2004.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6c97b3e25a1d8f04 |
| Counter frequency | 2112773217 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ema_form_float | 4024ns | 3884ns | 3483ns | 3886ns | 4978ns | base |
| ema_form_intshift | 3279ns | 2742ns | 2473ns | 2743ns | 5690ns | -18.52% |
| ema_form_q32 | 3623ns | 3312ns | 3304ns | 3313ns | 4872ns | -9.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 2074ns | 1800ns | 2556ns | base | 0.494 |
| ema_form_intshift | 1030ns | 780ns | 1779ns | -50.36% | 0.994 |
| ema_form_q32 | 1566ns | 1432ns | 2101ns | -24.49% | 0.654 |

## Performance model

- Peak throughput: **1.313 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.511 | 38.9% |
| ema_form_intshift | 1.186 | 90.4% |
| ema_form_q32 | 0.715 | 54.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 4024ns | 4024ns | base |
| ema_form_intshift | 3279ns | 3279ns | -18.52% |
| ema_form_q32 | 3623ns | 3623ns | -9.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 2005ns | base | --- | [2005, 2005] | --- | --- | --- | --- |
| ema_form_intshift | 863ns | -1141.6ns (-56.9%) | [-1142, -1142]ns | [863, 863] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 1432ns | -572.7ns (-28.6%) | [-573, -573]ns | [1432, 1432] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 2004ns | -56.9% | -28.6% |
| 1 | 2 | 2005ns | -56.9% | -28.6% |
| 1 | 3 | 2005ns | -64.8% | -28.6% |
| 1 | 4 | 2005ns | -56.9% | -28.6% |
| 1 | 5 | 2005ns | -56.9% | -28.6% |
| 1 | 6 | 2005ns | -56.9% | -28.6% |
| 1 | 7 | 1640ns | -47.4% | -12.7% |
| 1 | 8 | 2005ns | -56.9% | -28.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | -0.161 | ok |
| ema_form_intshift | -0.161 | ok |
| ema_form_q32 | -0.092 | ok |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 24.6ns | 2074.3ns | 1.2% |  |
| ema_form_intshift | 28.8ns | 1029.7ns | 2.8% |  |
| ema_form_q32 | 26.0ns | 1566.3ns | 1.7% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 1800.4-2555.7 ns)
   1800.4 |
   1838.2 |
   1876.0 |
   1913.7 |
   1951.5 |
   1989.3 |########################################
   2027.0 |
   2064.8 |
   2102.5 |
   2140.3 |
   2178.1 |
   2215.8 |
   2253.6 |
   2291.4 |
   2329.1 |
   2366.9 |
   2404.7 |
   2442.4 |
   2480.2 |
   2517.9 |
  (18 below, 7 above range)

ema_form_intshift (n=160, range 779.9-1779.2 ns)
    779.9 |
    829.9 |########################################
    879.9 |
    929.8 |
    979.8 |
   1029.8 |
   1079.7 |
   1129.7 |
   1179.6 |
   1229.6 |
   1279.6 |
   1329.5 |
   1379.5 |
   1429.5 |
   1479.4 |
   1529.4 |
   1579.4 |
   1629.3 |
   1679.3 |
   1729.2 |
  (17 below, 17 above range)

ema_form_q32 (n=160, range 1431.8-2101.4 ns)
   1431.8 |########################################
   1465.3 |
   1498.7 |
   1532.2 |
   1565.7 |
   1599.2 |
   1632.7 |
   1666.1 |
   1699.6 |
   1733.1 |
   1766.6 |
   1800.1 |
   1833.5 |
   1867.0 |
   1900.5 |
   1934.0 |
   1967.4 |
   2000.9 |
   2034.4 |
   2067.9 |
  (7 below, 9 above range)

```

## Diagnostics

- **ema_form_float**: CV=27.1% (high variance, measurements may be unstable)
- **ema_form_intshift**: CV=51.0% (high variance, measurements may be unstable)
- **ema_form_q32**: CV=34.3% (high variance, measurements may be unstable)

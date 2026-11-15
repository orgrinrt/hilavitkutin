# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (509 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 222 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 64% faster than the next best (ema_form_q32)

ema_form_intshift (222 ns) leads ema_form_q32 (363 ns) by 64%, a clear separation rather than a photo finish. CV 83.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 56% (significant)

ema_form_intshift is -287 ns (56%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_form_float is an outlier: 2.3x slower than the field

ema_form_float (509 ns) is 2.3x the fastest (222 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ema_form_intshift is fastest but the noisiest (CV 83.1%)

ema_form_intshift wins on median (222 ns) yet has the highest variance (CV 83.1%), while ema_form_float is the steadiest (CV 71.5%, 509 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### ema_form_intshift is inconsistent: worst-20% is 3.5x its best-20%

ema_form_intshift's best 20% of batches run at 185 ns but its worst 20% at 652 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ema_form_intshift** at 221.9 ns median (-56.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.29x (fastest 221.9 ns, slowest 508.7 ns)

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
| ema_form_float | 2997ns | 2382ns | 1949ns | 2258ns | 6265ns | base |
| ema_form_intshift | 2856ns | 2099ns | 1752ns | 2113ns | 6188ns | -4.72% |
| ema_form_q32 | 2983ns | 2241ns | 1873ns | 2243ns | 6314ns | -0.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 631ns | 416ns | 1296ns | base | 0.406 |
| ema_form_intshift | 301ns | 185ns | 652ns | -52.19% | 0.849 |
| ema_form_q32 | 483ns | 304ns | 1017ns | -23.49% | 0.531 |

## Performance model

- Peak throughput: **1.382 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.503 | 36.4% |
| ema_form_intshift | 1.154 | 83.5% |
| ema_form_q32 | 0.705 | 51.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 2997ns | 2997ns | base |
| ema_form_intshift | 2856ns | 2856ns | -4.72% |
| ema_form_q32 | 2983ns | 2983ns | -0.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 509ns | base | --- | [509, 509] | --- | --- | --- | --- |
| ema_form_intshift | 222ns | -286.8ns (-56.4%) | [-287, -287]ns | [222, 222] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 363ns | -145.3ns (-28.6%) | [-145, -119]ns | [363, 363] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 416ns | -56.4% | -28.5% |
| 1 | 2 | 509ns | -64.3% | -41.6% |
| 1 | 3 | 416ns | -46.7% | -12.7% |
| 1 | 4 | 509ns | -56.4% | -28.6% |
| 1 | 5 | 509ns | -56.4% | -28.6% |
| 1 | 6 | 509ns | -56.4% | -28.6% |
| 1 | 7 | 416ns | -46.7% | -12.7% |
| 1 | 8 | 416ns | -46.7% | -12.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | -0.124 | ok |
| ema_form_intshift | 0.458 | moderate+ |
| ema_form_q32 | 0.458 | moderate+ |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 29.4ns | 630.6ns | 4.7% |  |
| ema_form_intshift | 32.2ns | 301.5ns | 10.7% | HIGH |
| ema_form_q32 | 31.5ns | 482.5ns | 6.5% | HIGH |

## Distribution (algo ns)

```
ema_form_float (n=160, range 416.2-1296.5 ns)
    416.2 |###############################
    460.2 |
    504.2 |########################################
    548.2 |
    592.2 |
    636.2 |
    680.3 |
    724.3 |
    768.3 |
    812.3 |
    856.3 |
    900.3 |
    944.3 |
    988.4 |#
   1032.4 |
   1076.4 |
   1120.4 |
   1164.4 |
   1208.4 |#
   1252.4 |
  (9 below, 21 above range)

ema_form_intshift (n=160, range 185.3-652.3 ns)
    185.3 |
    208.6 |########################################
    232.0 |#
    255.3 |
    278.7 |
    302.0 |
    325.4 |
    348.7 |
    372.1 |
    395.4 |
    418.8 |
    442.1 |
    465.5 |
    488.8 |#
    512.2 |
    535.5 |
    558.9 |
    582.2 |
    605.6 |#
    628.9 |
  (29 below, 18 above range)

ema_form_q32 (n=160, range 304.1-1017.5 ns)
    304.1 |
    339.7 |########################################
    375.4 |
    411.1 |
    446.7 |
    482.4 |
    518.1 |
    553.8 |
    589.4 |
    625.1 |
    660.8 |
    696.4 |
    732.1 |
    767.8 |
    803.4 |#
    839.1 |#
    874.8 |
    910.5 |
    946.1 |
    981.8 |##
  (27 below, 12 above range)

```

## Diagnostics

- **ema_form_float**: CV=57.7% (high variance, measurements may be unstable)
- **ema_form_float**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **ema_form_intshift**: CV=61.2% (high variance, measurements may be unstable)
- **ema_form_intshift**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **ema_form_intshift**: bridge=10.7% of algo (FFI overhead may distort results)
- **ema_form_q32**: CV=58.1% (high variance, measurements may be unstable)
- **ema_form_q32**: worst_20/best_20 = 3.3x (possible bimodal distribution)

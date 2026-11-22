# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (509 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 222 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 64% faster than the next best (ema_form_q32)

ema_form_intshift (222 ns) leads ema_form_q32 (363 ns) by 64%, a clear separation rather than a photo finish. CV 83.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 56% (significant)

ema_form_intshift is -287 ns (56%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_form_float is an outlier: 2.3x slower than the field

ema_form_float (509 ns) is 2.3x the fastest (222 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ema_form_intshift is fastest but the noisiest (CV 83.3%)

ema_form_intshift wins on median (222 ns) yet has the highest variance (CV 83.3%), while ema_form_float is the steadiest (CV 71.8%, 509 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### ema_form_intshift is inconsistent: worst-20% is 3.6x its best-20%

ema_form_intshift's best 20% of batches run at 181 ns but its worst 20% at 649 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ema_form_intshift** at 221.8 ns median (-56.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.29x (fastest 221.8 ns, slowest 508.7 ns)

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
| Counter frequency | 2112771874 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ema_form_float | 2944ns | 2392ns | 1958ns | 2323ns | 5790ns | base |
| ema_form_intshift | 2784ns | 2105ns | 1722ns | 1995ns | 6215ns | -5.41% |
| ema_form_q32 | 2918ns | 2249ns | 1839ns | 2215ns | 6104ns | -0.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 625ns | 416ns | 1224ns | base | 0.410 |
| ema_form_intshift | 292ns | 181ns | 649ns | -53.20% | 0.876 |
| ema_form_q32 | 471ns | 297ns | 983ns | -24.56% | 0.543 |

## Performance model

- Peak throughput: **1.411 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.503 | 35.7% |
| ema_form_intshift | 1.154 | 81.8% |
| ema_form_q32 | 0.705 | 50.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 2944ns | 2944ns | base |
| ema_form_intshift | 2784ns | 2784ns | -5.41% |
| ema_form_q32 | 2918ns | 2918ns | -0.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 509ns | base | --- | [509, 509] | --- | --- | --- | --- |
| ema_form_intshift | 222ns | -286.8ns (-56.4%) | [-287, -287]ns | [222, 222] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 363ns | -145.4ns (-28.6%) | [-146, -145]ns | [363, 363] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 509ns | -56.4% | -41.6% |
| 1 | 2 | 416ns | -56.4% | -12.7% |
| 1 | 3 | 416ns | -56.4% | -28.6% |
| 1 | 4 | 416ns | -56.4% | -12.7% |
| 1 | 5 | 509ns | -56.4% | -35.7% |
| 1 | 6 | 509ns | -64.3% | -28.6% |
| 1 | 7 | 509ns | -56.4% | -28.6% |
| 1 | 8 | 509ns | -56.4% | -28.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | 0.392 | moderate+ |
| ema_form_intshift | -0.124 | ok |
| ema_form_q32 | -0.400 | moderate- |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 29.6ns | 624.5ns | 4.7% |  |
| ema_form_intshift | 31.5ns | 292.3ns | 10.8% | HIGH |
| ema_form_q32 | 30.8ns | 471.1ns | 6.5% | HIGH |

## Distribution (algo ns)

```
ema_form_float (n=160, range 416.2-1224.1 ns)
    416.2 |##########
    456.6 |
    497.0 |########################################
    537.4 |
    577.8 |
    618.2 |
    658.6 |
    699.0 |
    739.4 |
    779.8 |
    820.1 |
    860.5 |
    900.9 |
    941.3 |
    981.7 |
   1022.1 |
   1062.5 |
   1102.9 |
   1143.3 |
   1183.7 |
  (26 below, 22 above range)

ema_form_intshift (n=160, range 181.5-649.4 ns)
    181.5 |#################################
    204.9 |########################################
    228.3 |#
    251.7 |
    275.1 |
    298.5 |
    321.9 |
    345.3 |
    368.7 |
    392.1 |
    415.5 |
    438.9 |
    462.3 |
    485.7 |
    509.1 |#
    532.5 |#
    555.9 |#
    579.2 |#
    602.6 |##
    626.0 |
  (6 below, 16 above range)

ema_form_q32 (n=160, range 297.3-982.6 ns)
    297.3 |##############
    331.6 |########################################
    365.8 |##
    400.1 |
    434.4 |
    468.6 |
    502.9 |
    537.2 |
    571.4 |
    605.7 |
    640.0 |
    674.2 |
    708.5 |
    742.7 |
    777.0 |
    811.3 |#
    845.5 |
    879.8 |
    914.1 |#
    948.3 |
  (9 below, 18 above range)

```

## Diagnostics

- **ema_form_float**: CV=58.5% (high variance, measurements may be unstable)
- **ema_form_intshift**: CV=63.2% (high variance, measurements may be unstable)
- **ema_form_intshift**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **ema_form_intshift**: bridge=10.7% of algo (FFI overhead may distort results)
- **ema_form_q32**: CV=59.3% (high variance, measurements may be unstable)
- **ema_form_q32**: worst_20/best_20 = 3.3x (possible bimodal distribution)

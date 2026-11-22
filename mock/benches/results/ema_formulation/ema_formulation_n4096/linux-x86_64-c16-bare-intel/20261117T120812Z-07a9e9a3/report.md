# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (8.00 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 3.43 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 66% faster than the next best (ema_form_q32)

ema_form_intshift (3.43 us) leads ema_form_q32 (5.71 us) by 66%, a clear separation rather than a photo finish. CV 7.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 57% (significant)

ema_form_intshift is -4.57 us (57%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_form_float is an outlier: 2.3x slower than the field

ema_form_float (8.00 us) is 2.3x the fastest (3.43 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: ema_form_intshift** at 3427.8 ns median (-57.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.33x (fastest 3427.8 ns, slowest 8000.1 ns)

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
| ema_form_float | 9450ns | 9885ns | 8087ns | 9737ns | 9951ns | base |
| ema_form_intshift | 5082ns | 5315ns | 4349ns | 5239ns | 5345ns | -46.22% |
| ema_form_q32 | 7259ns | 7599ns | 6215ns | 7486ns | 7624ns | -23.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 7642ns | 6542ns | 8038ns | base | 0.536 |
| ema_form_intshift | 3276ns | 2806ns | 3440ns | -57.14% | 1.250 |
| ema_form_q32 | 5453ns | 4672ns | 5719ns | -28.65% | 0.751 |

## Performance model

- Peak throughput: **1.460 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.512 | 35.1% |
| ema_form_intshift | 1.195 | 81.9% |
| ema_form_q32 | 0.718 | 49.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 9450ns | 9450ns | base |
| ema_form_intshift | 5082ns | 5082ns | -46.22% |
| ema_form_q32 | 7259ns | 7259ns | -23.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 8000ns | base | --- | [8000, 8000] | --- | --- | --- | --- |
| ema_form_intshift | 3428ns | -4571.5ns (-57.1%) | [-4572, -4561]ns | [3428, 3428] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 5707ns | -2282.8ns (-28.5%) | [-2284, -2282]ns | [5707, 5707] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 8000ns | -57.2% | -28.6% |
| 1 | 2 | 8001ns | -64.9% | -28.7% |
| 1 | 3 | 6545ns | -47.6% | -12.6% |
| 1 | 4 | 8000ns | -57.2% | -28.5% |
| 1 | 5 | 8000ns | -57.2% | -28.5% |
| 1 | 6 | 6546ns | -57.2% | -28.7% |
| 1 | 7 | 8000ns | -57.2% | -28.5% |
| 1 | 8 | 8001ns | -57.2% | -41.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | -0.375 | moderate- |
| ema_form_intshift | -0.375 | moderate- |
| ema_form_q32 | -0.213 | moderate- |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 22.6ns | 7642.2ns | 0.3% |  |
| ema_form_intshift | 22.5ns | 3275.7ns | 0.7% |  |
| ema_form_q32 | 22.5ns | 5452.8ns | 0.4% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 6541.9-8037.9 ns)
   6541.9 |#########
   6616.7 |
   6691.5 |
   6766.3 |
   6841.1 |
   6915.9 |
   6990.7 |
   7065.5 |
   7140.3 |
   7215.1 |
   7289.9 |
   7364.7 |
   7439.5 |
   7514.3 |
   7589.1 |
   7663.9 |
   7738.7 |
   7813.5 |
   7888.3 |
   7963.1 |########################################
  (13 below, 4 above range)

ema_form_intshift (n=160, range 2805.8-3440.2 ns)
   2805.8 |####
   2837.5 |
   2869.2 |
   2901.0 |
   2932.7 |
   2964.4 |
   2996.1 |
   3027.8 |
   3059.6 |
   3091.3 |
   3123.0 |
   3154.7 |
   3186.4 |
   3218.2 |
   3249.9 |
   3281.6 |
   3313.3 |
   3345.0 |
   3376.8 |
   3408.5 |########################################
  (28 below, 8 above range)

ema_form_q32 (n=160, range 4671.9-5718.7 ns)
   4671.9 |######
   4724.3 |
   4776.6 |
   4829.0 |
   4881.3 |
   4933.6 |
   4986.0 |
   5038.3 |
   5090.6 |
   5143.0 |
   5195.3 |
   5247.7 |
   5300.0 |
   5352.3 |
   5404.7 |
   5457.0 |
   5509.4 |
   5561.7 |
   5614.0 |
   5666.4 |########################################
  (22 below, 8 above range)

```

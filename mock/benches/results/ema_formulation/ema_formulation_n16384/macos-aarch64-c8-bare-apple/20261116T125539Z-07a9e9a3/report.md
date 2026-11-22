# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (35.68 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 20.40 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 50% faster than the next best (ema_form_q32)

ema_form_intshift (20.40 us) leads ema_form_q32 (30.64 us) by 50%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 43% (significant)

ema_form_intshift is -15.25 us (43%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_form_q32 shows warm-up / thermal drift (autocorr +0.53)

ema_form_q32's per-pass series has lag-1 autocorrelation +0.53, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: ema_form_intshift** at 20399.8 ns median (-42.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.75x (fastest 20399.8 ns, slowest 35676.9 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ema_form_float | 37910ns | 37891ns | 37830ns | 37890ns | 38051ns | base |
| ema_form_intshift | 22573ns | 22524ns | 22500ns | 22528ns | 22782ns | -40.46% |
| ema_form_q32 | 32819ns | 32775ns | 32729ns | 32776ns | 33041ns | -13.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 35698ns | 35643ns | 35818ns | base | 0.459 |
| ema_form_intshift | 20457ns | 20392ns | 20651ns | -42.70% | 0.801 |
| ema_form_q32 | 30682ns | 30614ns | 30870ns | -14.05% | 0.534 |

## Performance model

- Peak throughput: **0.803 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.459 | 57.2% |
| ema_form_intshift | 0.803 | 100.0% |
| ema_form_q32 | 0.535 | 66.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 37910ns | 37910ns | base |
| ema_form_intshift | 22573ns | 22573ns | -40.46% |
| ema_form_q32 | 32819ns | 32819ns | -13.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 35677ns | base | --- | [35651, 35692] | --- | --- | --- | --- |
| ema_form_intshift | 20400ns | -15251.2ns (-42.7%) | [-15255, -15250]ns | [20399, 20418] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 30636ns | -5028.6ns (-14.1%) | [-5039, -5025]ns | [30623, 30653] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 35657ns | -42.6% | -14.1% |
| 1 | 2 | 35684ns | -42.8% | -14.2% |
| 1 | 3 | 35659ns | -42.8% | -14.1% |
| 1 | 4 | 35694ns | -42.8% | -14.2% |
| 1 | 5 | 35700ns | -42.9% | -14.2% |
| 1 | 6 | 35661ns | -42.7% | -14.1% |
| 1 | 7 | 35693ns | -42.8% | -14.1% |
| 1 | 8 | 35650ns | -42.8% | -14.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | -0.428 | moderate- |
| ema_form_intshift | -0.071 | ok |
| ema_form_q32 | 0.530 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 3.7ns | 35698.1ns | 0.0% |  |
| ema_form_intshift | 3.3ns | 20456.7ns | 0.0% |  |
| ema_form_q32 | 3.4ns | 30681.8ns | 0.0% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 35642.8-35817.9 ns)
  35642.8 |########################################
  35651.6 |##
  35660.4 |###
  35669.1 |####
  35677.9 |##
  35686.6 |###########
  35695.4 |######
  35704.1 |#########
  35712.9 |#
  35721.6 |#
  35730.4 |#####
  35739.1 |##
  35747.9 |
  35756.6 |
  35765.4 |##
  35774.1 |##
  35782.9 |##
  35791.6 |##
  35800.4 |#
  35809.1 |#
  (16 below, 10 above range)

ema_form_intshift (n=160, range 20392.1-20651.5 ns)
  20392.1 |########################################
  20405.1 |#
  20418.0 |####
  20431.0 |#########
  20444.0 |########
  20456.9 |##
  20469.9 |#
  20482.9 |#
  20495.9 |#
  20508.8 |#
  20521.8 |
  20534.8 |
  20547.7 |
  20560.7 |
  20573.7 |
  20586.6 |
  20599.6 |
  20612.6 |
  20625.5 |
  20638.5 |
  (12 below, 6 above range)

ema_form_q32 (n=160, range 30613.8-30870.1 ns)
  30613.8 |########################################
  30626.6 |###
  30639.4 |#####
  30652.2 |########
  30665.0 |################
  30677.9 |######
  30690.7 |
  30703.5 |
  30716.3 |
  30729.1 |
  30741.9 |#
  30754.8 |
  30767.6 |
  30780.4 |
  30793.2 |
  30806.0 |
  30818.8 |
  30831.7 |
  30844.5 |#
  30857.3 |
  (13 below, 5 above range)

```

## Diagnostics

- **ema_form_q32**: autocorrelation=0.53 (measurement drift or warm-up artifact)

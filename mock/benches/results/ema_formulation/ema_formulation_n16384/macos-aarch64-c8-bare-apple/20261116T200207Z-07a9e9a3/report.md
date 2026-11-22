# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (35.70 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 20.40 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 50% faster than the next best (ema_form_q32)

ema_form_intshift (20.40 us) leads ema_form_q32 (30.65 us) by 50%, a clear separation rather than a photo finish. CV 0.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 43% (significant)

ema_form_intshift is -15.29 us (43%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: ema_form_intshift** at 20400.8 ns median (-42.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.75x (fastest 20400.8 ns, slowest 35695.8 ns)

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
| ema_form_float | 38191ns | 37908ns | 37838ns | 37925ns | 39345ns | base |
| ema_form_intshift | 22546ns | 22529ns | 22502ns | 22532ns | 22636ns | -40.96% |
| ema_form_q32 | 32804ns | 32776ns | 32728ns | 32781ns | 32950ns | -14.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 35965ns | 35645ns | 37041ns | base | 0.456 |
| ema_form_intshift | 20430ns | 20393ns | 20518ns | -43.19% | 0.802 |
| ema_form_q32 | 30672ns | 30616ns | 30793ns | -14.72% | 0.534 |

## Performance model

- Peak throughput: **0.803 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.459 | 57.1% |
| ema_form_intshift | 0.803 | 100.0% |
| ema_form_q32 | 0.535 | 66.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 38191ns | 38191ns | base |
| ema_form_intshift | 22546ns | 22546ns | -40.96% |
| ema_form_q32 | 32804ns | 32804ns | -14.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 35696ns | base | --- | [35690, 35712] | --- | --- | --- | --- |
| ema_form_intshift | 20401ns | -15285.2ns (-42.8%) | [-15295, -15264]ns | [20399, 20413] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 30651ns | -5048.7ns (-14.1%) | [-5070, -5031]ns | [30634, 30663] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 35742ns | -42.9% | -14.3% |
| 1 | 2 | 35705ns | -42.8% | -14.2% |
| 1 | 3 | 35674ns | -42.8% | -14.1% |
| 1 | 4 | 35693ns | -42.8% | -14.1% |
| 1 | 5 | 35691ns | -42.8% | -14.1% |
| 1 | 6 | 35688ns | -42.8% | -14.1% |
| 1 | 7 | 37295ns | -45.2% | -17.8% |
| 1 | 8 | 35674ns | -42.8% | -14.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | -0.179 | ok |
| ema_form_intshift | -0.248 | moderate- |
| ema_form_q32 | 0.078 | ok |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 3.6ns | 35964.7ns | 0.0% |  |
| ema_form_intshift | 3.4ns | 20430.2ns | 0.0% |  |
| ema_form_q32 | 3.6ns | 30672.1ns | 0.0% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 35644.9-37040.7 ns)
  35644.9 |########################################
  35714.7 |#########
  35784.5 |####
  35854.3 |##
  35924.1 |#
  35993.9 |
  36063.7 |
  36133.5 |
  36203.2 |
  36273.0 |
  36342.8 |
  36412.6 |
  36482.4 |
  36552.2 |
  36622.0 |
  36691.8 |
  36761.5 |
  36831.3 |
  36901.1 |
  36970.9 |
  (11 below, 22 above range)

ema_form_intshift (n=160, range 20393.0-20518.2 ns)
  20393.0 |########################################
  20399.2 |#########
  20405.5 |##
  20411.8 |##
  20418.0 |##
  20424.3 |###
  20430.5 |###
  20436.8 |####
  20443.1 |#####
  20449.3 |
  20455.6 |##
  20461.9 |##
  20468.1 |#
  20474.4 |####
  20480.6 |
  20486.9 |
  20493.2 |##
  20499.4 |
  20505.7 |##
  20512.0 |
  (16 below, 12 above range)

ema_form_q32 (n=160, range 30616.2-30793.3 ns)
  30616.2 |########################################
  30625.0 |########
  30633.9 |######
  30642.8 |####
  30651.6 |######
  30660.5 |#################
  30669.3 |########
  30678.2 |######
  30687.0 |#
  30695.9 |#########
  30704.8 |#
  30713.6 |
  30722.5 |#
  30731.3 |
  30740.2 |###
  30749.0 |##
  30757.9 |
  30766.7 |
  30775.6 |
  30784.5 |
  (17 below, 12 above range)

```

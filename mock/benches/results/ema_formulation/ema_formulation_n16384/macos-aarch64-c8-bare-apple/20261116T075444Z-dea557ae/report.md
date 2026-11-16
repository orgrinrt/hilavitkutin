# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (35.70 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 20.44 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 50% faster than the next best (ema_form_q32)

ema_form_intshift (20.44 us) leads ema_form_q32 (30.69 us) by 50%, a clear separation rather than a photo finish. CV 0.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 43% (significant)

ema_form_intshift is -15.25 us (43%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: ema_form_intshift** at 20439.2 ns median (-42.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.75x (fastest 20439.2 ns, slowest 35698.6 ns)

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
| ema_form_float | 37949ns | 37899ns | 37830ns | 37903ns | 38206ns | base |
| ema_form_intshift | 22563ns | 22544ns | 22492ns | 22543ns | 22691ns | -40.54% |
| ema_form_q32 | 33019ns | 32812ns | 32731ns | 32829ns | 33877ns | -12.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 35743ns | 35645ns | 35977ns | base | 0.458 |
| ema_form_intshift | 20453ns | 20395ns | 20570ns | -42.78% | 0.801 |
| ema_form_q32 | 30879ns | 30620ns | 31671ns | -13.61% | 0.531 |

## Performance model

- Peak throughput: **0.803 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.459 | 57.1% |
| ema_form_intshift | 0.802 | 99.8% |
| ema_form_q32 | 0.534 | 66.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 37949ns | 37949ns | base |
| ema_form_intshift | 22563ns | 22563ns | -40.54% |
| ema_form_q32 | 33019ns | 33019ns | -12.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 35699ns | base | --- | [35689, 35710] | --- | --- | --- | --- |
| ema_form_intshift | 20439ns | -15254.6ns (-42.7%) | [-15273, -15248]ns | [20416, 20450] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 30689ns | -5006.9ns (-14.0%) | [-5026, -4982]ns | [30671, 30708] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 35722ns | -42.9% | -14.2% |
| 1 | 2 | 35686ns | -42.7% | -13.9% |
| 1 | 3 | 35711ns | -42.9% | -14.0% |
| 1 | 4 | 35711ns | -42.9% | -14.2% |
| 1 | 5 | 35692ns | -42.7% | -14.1% |
| 1 | 6 | 35689ns | -42.7% | -14.1% |
| 1 | 7 | 35702ns | -42.7% | -13.5% |
| 1 | 8 | 35691ns | -42.8% | -14.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | -0.303 | moderate- |
| ema_form_intshift | -0.203 | moderate- |
| ema_form_q32 | -0.295 | moderate- |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 3.1ns | 35743.1ns | 0.0% |  |
| ema_form_intshift | 2.7ns | 20453.5ns | 0.0% |  |
| ema_form_q32 | 3.1ns | 30878.9ns | 0.0% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 35645.0-35976.5 ns)
  35645.0 |########################################
  35661.6 |###########
  35678.2 |###############
  35694.7 |##################
  35711.3 |###############
  35727.9 |#########
  35744.5 |########
  35761.0 |######
  35777.6 |####
  35794.2 |####
  35810.8 |#####
  35827.3 |###
  35843.9 |
  35860.5 |#
  35877.1 |##
  35893.6 |##
  35910.2 |#
  35926.8 |#
  35943.4 |
  35960.0 |
  (15 below, 6 above range)

ema_form_intshift (n=160, range 20395.0-20569.6 ns)
  20395.0 |########################################
  20403.7 |#####
  20412.5 |######
  20421.2 |####
  20429.9 |###
  20438.7 |########
  20447.4 |###########
  20456.1 |##########
  20464.9 |####
  20473.6 |##
  20482.3 |#######
  20491.1 |###
  20499.8 |####
  20508.5 |##
  20517.3 |###
  20526.0 |
  20534.7 |
  20543.4 |
  20552.2 |
  20560.9 |
  (13 below, 10 above range)

ema_form_q32 (n=160, range 30619.5-31671.2 ns)
  30619.5 |########################################
  30672.1 |#########################
  30724.7 |#########
  30777.3 |#####
  30829.9 |##
  30882.5 |##
  30935.0 |
  30987.6 |
  31040.2 |
  31092.8 |
  31145.4 |
  31198.0 |##
  31250.5 |#
  31303.1 |
  31355.7 |
  31408.3 |
  31460.9 |#
  31513.5 |
  31566.0 |
  31618.6 |#
  (15 below, 14 above range)

```

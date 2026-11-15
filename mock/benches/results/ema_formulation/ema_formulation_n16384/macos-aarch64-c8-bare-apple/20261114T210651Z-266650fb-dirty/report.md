# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (35.71 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 20.46 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 50% faster than the next best (ema_form_q32)

ema_form_intshift (20.46 us) leads ema_form_q32 (30.68 us) by 50%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 43% (significant)

ema_form_intshift is -15.23 us (43%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: ema_form_intshift** at 20464.8 ns median (-42.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.74x (fastest 20464.8 ns, slowest 35709.8 ns)

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
| ema_form_float | 38091ns | 37922ns | 37837ns | 37934ns | 38819ns | base |
| ema_form_intshift | 22856ns | 22589ns | 22502ns | 22684ns | 23729ns | -40.00% |
| ema_form_q32 | 32976ns | 32808ns | 32728ns | 32821ns | 33687ns | -13.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 35870ns | 35647ns | 36542ns | base | 0.457 |
| ema_form_intshift | 20708ns | 20395ns | 21486ns | -42.27% | 0.791 |
| ema_form_q32 | 30842ns | 30620ns | 31498ns | -14.02% | 0.531 |

## Performance model

- Peak throughput: **0.803 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.459 | 57.1% |
| ema_form_intshift | 0.801 | 99.7% |
| ema_form_q32 | 0.534 | 66.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 38091ns | 38091ns | base |
| ema_form_intshift | 22856ns | 22856ns | -40.00% |
| ema_form_q32 | 32976ns | 32976ns | -13.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 35710ns | base | --- | [35696, 35728] | --- | --- | --- | --- |
| ema_form_intshift | 20465ns | -15233.5ns (-42.7%) | [-15251, -15208]ns | [20447, 20512] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 30682ns | -5028.4ns (-14.1%) | [-5054, -5009]ns | [30673, 30702] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 35714ns | -42.8% | -14.1% |
| 1 | 2 | 35693ns | -42.3% | -13.9% |
| 1 | 3 | 35717ns | -41.9% | -14.0% |
| 1 | 4 | 35715ns | -42.8% | -14.1% |
| 1 | 5 | 35695ns | -42.5% | -14.1% |
| 1 | 6 | 35723ns | -42.2% | -14.1% |
| 1 | 7 | 35712ns | -42.7% | -14.2% |
| 1 | 8 | 35725ns | -42.6% | -14.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | -0.376 | moderate- |
| ema_form_intshift | -0.217 | moderate- |
| ema_form_q32 | 0.339 | moderate+ |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 3.1ns | 35869.9ns | 0.0% |  |
| ema_form_intshift | 3.6ns | 20708.0ns | 0.0% |  |
| ema_form_q32 | 3.4ns | 30841.9ns | 0.0% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 35647.2-36541.6 ns)
  35647.2 |######################################
  35692.0 |########################################
  35736.7 |#############
  35781.4 |####
  35826.1 |#####
  35870.8 |
  35915.5 |
  35960.3 |
  36005.0 |##
  36049.7 |#
  36094.4 |
  36139.1 |
  36183.8 |
  36228.6 |
  36273.3 |
  36318.0 |##
  36362.7 |#
  36407.4 |
  36452.1 |#
  36496.9 |#
  (18 below, 12 above range)

ema_form_intshift (n=160, range 20394.8-21486.4 ns)
  20394.8 |########################################
  20449.4 |################
  20503.9 |######
  20558.5 |##
  20613.1 |
  20667.7 |###
  20722.3 |
  20776.8 |#######
  20831.4 |###
  20886.0 |
  20940.6 |
  20995.2 |###
  21049.7 |#
  21104.3 |###
  21158.9 |##
  21213.5 |###
  21268.0 |##
  21322.6 |
  21377.2 |#
  21431.8 |
  (16 below, 13 above range)

ema_form_q32 (n=160, range 30619.7-31497.8 ns)
  30619.7 |########################################
  30663.6 |###############################
  30707.5 |##################
  30751.4 |########
  30795.3 |##
  30839.2 |###
  30883.1 |#
  30927.0 |#####
  30970.9 |
  31014.8 |
  31058.7 |
  31102.7 |
  31146.6 |
  31190.5 |
  31234.4 |##
  31278.3 |
  31322.2 |
  31366.1 |
  31410.0 |
  31453.9 |
  (14 below, 16 above range)

```

# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (8.80 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 5.06 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 50% faster than the next best (ema_form_q32)

ema_form_intshift (5.06 us) leads ema_form_q32 (7.61 us) by 50%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 43% (significant)

ema_form_intshift is -3.75 us (43%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: ema_form_intshift** at 5056.7 ns median (-42.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.74x (fastest 5056.7 ns, slowest 8801.2 ns)

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
| ema_form_float | 11016ns | 10984ns | 10975ns | 10991ns | 11135ns | base |
| ema_form_intshift | 7187ns | 7169ns | 7161ns | 7172ns | 7260ns | -34.76% |
| ema_form_q32 | 9759ns | 9719ns | 9710ns | 9725ns | 9911ns | -11.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 8830ns | 8798ns | 8932ns | base | 0.464 |
| ema_form_intshift | 5071ns | 5050ns | 5131ns | -42.57% | 0.808 |
| ema_form_q32 | 7641ns | 7603ns | 7763ns | -13.47% | 0.536 |

## Performance model

- Peak throughput: **0.811 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.465 | 57.4% |
| ema_form_intshift | 0.810 | 99.9% |
| ema_form_q32 | 0.538 | 66.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 11016ns | 11016ns | base |
| ema_form_intshift | 7187ns | 7187ns | -34.76% |
| ema_form_q32 | 9759ns | 9759ns | -11.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 8801ns | base | --- | [8801, 8802] | --- | --- | --- | --- |
| ema_form_intshift | 5057ns | -3745.0ns (-42.6%) | [-3747, -3744]ns | [5056, 5058] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 7609ns | -1193.4ns (-13.6%) | [-1195, -1192]ns | [7608, 7610] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 8802ns | -42.5% | -13.6% |
| 1 | 2 | 8800ns | -42.5% | -13.5% |
| 1 | 3 | 8803ns | -42.6% | -13.6% |
| 1 | 4 | 8835ns | -42.8% | -13.9% |
| 1 | 5 | 8800ns | -42.5% | -13.5% |
| 1 | 6 | 8802ns | -42.6% | -13.6% |
| 1 | 7 | 8801ns | -42.6% | -10.5% |
| 1 | 8 | 8801ns | -42.6% | -13.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | -0.165 | ok |
| ema_form_intshift | 0.169 | ok |
| ema_form_q32 | -0.157 | ok |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 2.9ns | 8829.9ns | 0.0% |  |
| ema_form_intshift | 2.9ns | 5070.7ns | 0.1% |  |
| ema_form_q32 | 2.6ns | 7640.8ns | 0.0% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 8797.5-8932.2 ns)
   8797.5 |########################################
   8804.2 |
   8811.0 |
   8817.7 |##
   8824.4 |
   8831.2 |
   8837.9 |#
   8844.6 |###
   8851.4 |
   8858.1 |##
   8864.8 |
   8871.6 |#
   8878.3 |
   8885.0 |#
   8891.8 |
   8898.5 |
   8905.2 |
   8912.0 |
   8918.7 |
   8925.4 |
  (18 below, 8 above range)

ema_form_intshift (n=160, range 5050.2-5130.8 ns)
   5050.2 |####################
   5054.3 |########################################
   5058.3 |###################
   5062.3 |####
   5066.4 |##
   5070.4 |##
   5074.4 |
   5078.4 |
   5082.5 |#
   5086.5 |
   5090.5 |##
   5094.6 |
   5098.6 |#
   5102.6 |
   5106.7 |
   5110.7 |#
   5114.7 |
   5118.8 |
   5122.8 |
   5126.8 |
  (14 below, 12 above range)

ema_form_q32 (n=160, range 7602.6-7762.8 ns)
   7602.6 |########################################
   7610.6 |########
   7618.6 |###
   7626.6 |
   7634.6 |#
   7642.6 |#
   7650.6 |###
   7658.6 |
   7666.6 |#
   7674.6 |
   7682.7 |#
   7690.7 |
   7698.7 |
   7706.7 |
   7714.7 |
   7722.7 |
   7730.7 |
   7738.7 |
   7746.7 |
   7754.7 |
  (13 below, 13 above range)

```

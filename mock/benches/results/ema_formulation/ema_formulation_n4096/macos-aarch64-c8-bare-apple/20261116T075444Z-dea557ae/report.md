# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (8.80 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 5.06 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 50% faster than the next best (ema_form_q32)

ema_form_intshift (5.06 us) leads ema_form_q32 (7.61 us) by 50%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 43% (significant)

ema_form_intshift is -3.75 us (43%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: ema_form_intshift** at 5057.3 ns median (-42.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.74x (fastest 5057.3 ns, slowest 8803.8 ns)

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
| ema_form_float | 11057ns | 10990ns | 10962ns | 11002ns | 11316ns | base |
| ema_form_intshift | 7215ns | 7165ns | 7147ns | 7176ns | 7397ns | -34.75% |
| ema_form_q32 | 9790ns | 9724ns | 9701ns | 9733ns | 10050ns | -11.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 8870ns | 8798ns | 9079ns | base | 0.462 |
| ema_form_intshift | 5095ns | 5052ns | 5232ns | -42.56% | 0.804 |
| ema_form_q32 | 7670ns | 7603ns | 7869ns | -13.53% | 0.534 |

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
| ema_form_float | 11057ns | 11057ns | base |
| ema_form_intshift | 7215ns | 7215ns | -34.75% |
| ema_form_q32 | 9790ns | 9790ns | -11.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 8804ns | base | --- | [8802, 8829] | --- | --- | --- | --- |
| ema_form_intshift | 5057ns | -3746.3ns (-42.6%) | [-3748, -3745]ns | [5057, 5059] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 7611ns | -1193.3ns (-13.6%) | [-1196, -1191]ns | [7610, 7625] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 8802ns | -42.6% | -13.6% |
| 1 | 2 | 8800ns | -42.5% | -13.5% |
| 1 | 3 | 9056ns | -44.2% | -16.0% |
| 1 | 4 | 8826ns | -42.7% | -13.8% |
| 1 | 5 | 8801ns | -42.5% | -13.5% |
| 1 | 6 | 8858ns | -42.9% | -13.7% |
| 1 | 7 | 8851ns | -40.9% | -11.0% |
| 1 | 8 | 8800ns | -42.5% | -13.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | -0.229 | moderate- |
| ema_form_intshift | -0.136 | ok |
| ema_form_q32 | -0.047 | ok |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 3.0ns | 8870.0ns | 0.0% |  |
| ema_form_intshift | 2.8ns | 5095.1ns | 0.1% |  |
| ema_form_q32 | 2.5ns | 7669.8ns | 0.0% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 8797.6-9078.9 ns)
   8797.6 |########################################
   8811.7 |####
   8825.8 |####
   8839.8 |#####
   8853.9 |###
   8867.9 |####
   8882.0 |###
   8896.1 |
   8910.1 |#
   8924.2 |
   8938.2 |
   8952.3 |#
   8966.4 |#
   8980.4 |#
   8994.5 |
   9008.5 |
   9022.6 |
   9036.7 |
   9050.7 |
   9064.8 |
  (14 below, 17 above range)

ema_form_intshift (n=160, range 5051.6-5231.6 ns)
   5051.6 |########################################
   5060.6 |###
   5069.6 |##
   5078.6 |
   5087.6 |#
   5096.6 |
   5105.6 |
   5114.6 |#
   5123.6 |
   5132.6 |
   5141.6 |
   5150.6 |#
   5159.6 |
   5168.6 |
   5177.6 |
   5186.6 |
   5195.6 |
   5204.6 |
   5213.6 |
   5222.6 |###
  (14 below, 17 above range)

ema_form_q32 (n=160, range 7603.4-7868.8 ns)
   7603.4 |########################################
   7616.7 |#####
   7629.9 |###
   7643.2 |#########
   7656.5 |##
   7669.7 |#
   7683.0 |##
   7696.3 |
   7709.6 |
   7722.8 |
   7736.1 |
   7749.4 |#
   7762.6 |
   7775.9 |
   7789.2 |
   7802.5 |#
   7815.7 |
   7829.0 |
   7842.3 |
   7855.6 |#
  (16 below, 22 above range)

```

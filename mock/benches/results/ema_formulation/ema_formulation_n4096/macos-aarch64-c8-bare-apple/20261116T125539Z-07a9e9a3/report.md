# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (8.80 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 5.06 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 50% faster than the next best (ema_form_q32)

ema_form_intshift (5.06 us) leads ema_form_q32 (7.61 us) by 50%, a clear separation rather than a photo finish. CV 0.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 43% (significant)

ema_form_intshift is -3.75 us (43%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: ema_form_intshift** at 5055.8 ns median (-42.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.74x (fastest 5055.8 ns, slowest 8800.4 ns)

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
| ema_form_float | 10994ns | 10982ns | 10976ns | 10984ns | 11044ns | base |
| ema_form_intshift | 7172ns | 7165ns | 7159ns | 7166ns | 7204ns | -34.77% |
| ema_form_q32 | 9737ns | 9719ns | 9712ns | 9721ns | 9809ns | -11.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 8809ns | 8797ns | 8847ns | base | 0.465 |
| ema_form_intshift | 5060ns | 5049ns | 5081ns | -42.56% | 0.810 |
| ema_form_q32 | 7619ns | 7602ns | 7668ns | -13.51% | 0.538 |

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
| ema_form_float | 10994ns | 10994ns | base |
| ema_form_intshift | 7172ns | 7172ns | -34.77% |
| ema_form_q32 | 9737ns | 9737ns | -11.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 8800ns | base | --- | [8800, 8801] | --- | --- | --- | --- |
| ema_form_intshift | 5056ns | -3745.2ns (-42.6%) | [-3746, -3744]ns | [5055, 5056] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 7609ns | -1192.7ns (-13.6%) | [-1193, -1192]ns | [7608, 7609] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 8801ns | -42.5% | -13.6% |
| 1 | 2 | 8800ns | -42.6% | -13.5% |
| 1 | 3 | 8800ns | -42.6% | -13.5% |
| 1 | 4 | 8801ns | -42.6% | -13.6% |
| 1 | 5 | 8800ns | -42.5% | -13.5% |
| 1 | 6 | 8800ns | -42.6% | -13.5% |
| 1 | 7 | 8800ns | -42.6% | -13.5% |
| 1 | 8 | 8800ns | -42.5% | -13.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | -0.297 | moderate- |
| ema_form_intshift | 0.003 | ok |
| ema_form_q32 | -0.013 | ok |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 3.1ns | 8809.1ns | 0.0% |  |
| ema_form_intshift | 2.8ns | 5059.6ns | 0.1% |  |
| ema_form_q32 | 2.5ns | 7619.2ns | 0.0% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 8796.8-8847.2 ns)
   8796.8 |######################
   8799.4 |########################################
   8801.9 |##########
   8804.4 |
   8806.9 |
   8809.4 |
   8812.0 |
   8814.5 |
   8817.0 |
   8819.5 |
   8822.0 |
   8824.5 |#
   8827.1 |###
   8829.6 |
   8832.1 |
   8834.6 |
   8837.1 |
   8839.7 |
   8842.2 |
   8844.7 |
  (13 below, 13 above range)

ema_form_intshift (n=160, range 5049.5-5081.2 ns)
   5049.5 |##########
   5051.1 |##########################
   5052.7 |###############################
   5054.3 |########################################
   5055.8 |###################################
   5057.4 |#################################
   5059.0 |####################
   5060.6 |#####
   5062.2 |###
   5063.8 |
   5065.4 |
   5066.9 |###
   5068.5 |#
   5070.1 |
   5071.7 |
   5073.3 |#
   5074.9 |#####
   5076.5 |#
   5078.0 |#
   5079.6 |#####
  (18 below, 7 above range)

ema_form_q32 (n=160, range 7602.1-7668.4 ns)
   7602.1 |######################
   7605.4 |######################################
   7608.7 |########################################
   7612.0 |###
   7615.4 |##
   7618.7 |
   7622.0 |##
   7625.3 |
   7628.6 |
   7631.9 |#####
   7635.2 |##
   7638.5 |
   7641.9 |
   7645.2 |
   7648.5 |
   7651.8 |##
   7655.1 |#
   7658.4 |
   7661.7 |
   7665.1 |
  (12 below, 9 above range)

```

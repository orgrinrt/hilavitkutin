# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (26.15 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 13.70 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 67% faster than the next best (ema_form_q32)

ema_form_intshift (13.70 us) leads ema_form_q32 (22.83 us) by 67%, a clear separation rather than a photo finish. CV 8.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 55% (significant)

ema_form_intshift is -14.47 us (55%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: ema_form_intshift** at 13697.8 ns median (-47.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.91x (fastest 13697.8 ns, slowest 26151.3 ns)

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
| ema_form_float | 29297ns | 27697ns | 27685ns | 28254ns | 34036ns | base |
| ema_form_intshift | 14530ns | 15582ns | 12749ns | 14763ns | 15610ns | -50.40% |
| ema_form_q32 | 23045ns | 24714ns | 20218ns | 23412ns | 24771ns | -21.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 27657ns | 26140ns | 32134ns | base | 0.592 |
| ema_form_intshift | 12768ns | 11206ns | 13712ns | -53.83% | 1.283 |
| ema_form_q32 | 21280ns | 18675ns | 22868ns | -23.06% | 0.770 |

## Performance model

- Peak throughput: **1.462 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.627 | 42.9% |
| ema_form_intshift | 1.196 | 81.8% |
| ema_form_q32 | 0.718 | 49.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 29297ns | 29297ns | base |
| ema_form_intshift | 14530ns | 14530ns | -50.40% |
| ema_form_q32 | 23045ns | 23045ns | -21.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 26151ns | base | --- | [26151, 26153] | --- | --- | --- | --- |
| ema_form_intshift | 13698ns | -14466.6ns (-55.3%) | [-14934, -12461]ns | [13687, 13698] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 22826ns | -7471.4ns (-28.6%) | [-7474, -7462]ns | [22817, 22828] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 26150ns | -47.6% | -28.6% |
| 1 | 2 | 26149ns | -47.6% | -28.6% |
| 1 | 3 | 26151ns | -57.1% | -12.7% |
| 1 | 4 | 26150ns | -57.1% | -12.7% |
| 1 | 5 | 31961ns | -64.9% | -28.6% |
| 1 | 6 | 26152ns | -47.6% | -28.6% |
| 1 | 7 | 31974ns | -57.2% | -28.6% |
| 1 | 8 | 26150ns | -47.6% | -12.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | -0.375 | moderate- |
| ema_form_intshift | 0.391 | moderate+ |
| ema_form_q32 | 0.058 | ok |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 20.8ns | 27657.0ns | 0.1% |  |
| ema_form_intshift | 22.3ns | 12768.0ns | 0.2% |  |
| ema_form_q32 | 22.6ns | 21280.1ns | 0.1% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 26140.1-32133.9 ns)
  26140.1 |########################################
  26439.8 |
  26739.5 |
  27039.2 |
  27338.9 |
  27638.6 |
  27938.3 |
  28238.0 |
  28537.6 |
  28837.3 |
  29137.0 |
  29436.7 |
  29736.4 |
  30036.1 |
  30335.8 |
  30635.5 |
  30935.1 |
  31234.8 |
  31534.5 |
  31834.2 |#############
  (10 below, 4 above range)

ema_form_intshift (n=160, range 11205.9-13712.1 ns)
  11205.9 |######################
  11331.2 |
  11456.5 |
  11581.8 |
  11707.2 |
  11832.5 |
  11957.8 |
  12083.1 |
  12208.4 |
  12333.7 |
  12459.0 |
  12584.3 |
  12709.6 |
  12834.9 |
  12960.2 |
  13085.5 |
  13210.8 |
  13336.1 |
  13461.5 |
  13586.8 |########################################
  (6 below, 5 above range)

ema_form_q32 (n=160, range 18674.8-22868.2 ns)
  18674.8 |#####################
  18884.4 |
  19094.1 |
  19303.8 |
  19513.4 |
  19723.1 |
  19932.8 |
  20142.5 |
  20352.1 |
  20561.8 |
  20771.5 |
  20981.1 |
  21190.8 |
  21400.5 |
  21610.2 |
  21819.8 |
  22029.5 |
  22239.2 |
  22448.8 |
  22658.5 |########################################
  (6 below, 2 above range)

```

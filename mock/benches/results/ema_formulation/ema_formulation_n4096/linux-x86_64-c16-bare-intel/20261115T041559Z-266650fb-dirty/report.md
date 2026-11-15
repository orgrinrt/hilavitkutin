# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (8.00 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 3.43 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 67% faster than the next best (ema_form_q32)

ema_form_intshift (3.43 us) leads ema_form_q32 (5.72 us) by 67%, a clear separation rather than a photo finish. CV 0.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 57% (significant)

ema_form_intshift is -4.57 us (57%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_form_float is an outlier: 2.3x slower than the field

ema_form_float (8.00 us) is 2.3x the fastest (3.43 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: ema_form_intshift** at 3427.9 ns median (-57.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.33x (fastest 3427.9 ns, slowest 8000.1 ns)

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
| Counter frequency | 2112773217 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ema_form_float | 9886ns | 9878ns | 9872ns | 9878ns | 9921ns | base |
| ema_form_intshift | 5321ns | 5313ns | 5300ns | 5312ns | 5370ns | -46.17% |
| ema_form_q32 | 7602ns | 7594ns | 6726ns | 7592ns | 8507ns | -23.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 8000ns | 7989ns | 8011ns | base | 0.512 |
| ema_form_intshift | 3432ns | 3428ns | 3443ns | -57.10% | 1.193 |
| ema_form_q32 | 5714ns | 5061ns | 6371ns | -28.57% | 0.717 |

## Performance model

- Peak throughput: **1.195 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.512 | 42.8% |
| ema_form_intshift | 1.195 | 100.0% |
| ema_form_q32 | 0.716 | 60.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 9886ns | 9886ns | base |
| ema_form_intshift | 5321ns | 5321ns | -46.17% |
| ema_form_q32 | 7602ns | 7602ns | -23.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 8000ns | base | --- | [8000, 8000] | --- | --- | --- | --- |
| ema_form_intshift | 3428ns | -4571.7ns (-57.1%) | [-4572, -4562]ns | [3428, 3428] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 5717ns | -2283.0ns (-28.5%) | [-2284, -2283]ns | [5707, 5717] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 8000ns | -57.2% | -28.5% |
| 1 | 2 | 8000ns | -57.2% | -28.7% |
| 1 | 3 | 8000ns | -57.2% | -41.6% |
| 1 | 4 | 8000ns | -57.1% | -28.5% |
| 1 | 5 | 8000ns | -57.2% | -28.5% |
| 1 | 6 | 8000ns | -57.2% | -28.6% |
| 1 | 7 | 8000ns | -57.2% | -28.5% |
| 1 | 8 | 8000ns | -57.2% | -28.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | 0.273 | moderate+ |
| ema_form_intshift | -0.160 | ok |
| ema_form_q32 | -0.153 | ok |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 23.6ns | 7999.8ns | 0.3% |  |
| ema_form_intshift | 23.5ns | 3432.1ns | 0.7% |  |
| ema_form_q32 | 23.4ns | 5713.9ns | 0.4% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 7989.1-8010.7 ns)
   7989.1 |#########
   7990.1 |
   7991.2 |
   7992.3 |
   7993.4 |
   7994.5 |
   7995.6 |
   7996.6 |
   7997.7 |
   7998.8 |##########
   7999.9 |########################################
   8001.0 |############
   8002.1 |##
   8003.1 |
   8004.2 |
   8005.3 |
   8006.4 |
   8007.5 |
   8008.6 |
   8009.6 |
  (17 below, 3 above range)

ema_form_intshift (n=160, range 3427.6-3443.3 ns)
   3427.6 |########################################
   3428.4 |
   3429.2 |
   3430.0 |
   3430.7 |
   3431.5 |
   3432.3 |
   3433.1 |
   3433.9 |
   3434.7 |
   3435.4 |
   3436.2 |
   3437.0 |
   3437.8 |###
   3438.6 |#######
   3439.3 |##
   3440.1 |###
   3440.9 |#
   3441.7 |
   3442.5 |
  (16 below, 4 above range)

ema_form_q32 (n=160, range 5060.7-6371.1 ns)
   5060.7 |
   5126.2 |
   5191.7 |
   5257.2 |
   5322.7 |
   5388.3 |
   5453.8 |
   5519.3 |
   5584.8 |
   5650.3 |###############################
   5715.9 |########################################
   5781.4 |#
   5846.9 |
   5912.4 |#
   5977.9 |
   6043.5 |
   6109.0 |
   6174.5 |
   6240.0 |
   6305.5 |
  (20 below, 5 above range)

```

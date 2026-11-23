# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (8.00 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 3.43 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 66% faster than the next best (ema_form_q32)

ema_form_intshift (3.43 us) leads ema_form_q32 (5.71 us) by 66%, a clear separation rather than a photo finish. CV 17.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 57% (significant)

ema_form_intshift is -4.56 us (57%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_form_float is an outlier: 2.3x slower than the field

ema_form_float (8.00 us) is 2.3x the fastest (3.43 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ema_form_intshift is fastest but the noisiest (CV 17.3%)

ema_form_intshift wins on median (3.43 us) yet has the highest variance (CV 17.3%), while ema_form_float is the steadiest (CV 8.2%, 8.00 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: ema_form_intshift** at 3427.7 ns median (-57.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.33x (fastest 3427.7 ns, slowest 7999.9 ns)

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
| Counter frequency | 2112780456 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ema_form_float | 9455ns | 9880ns | 8081ns | 9733ns | 9996ns | base |
| ema_form_intshift | 4995ns | 5304ns | 4340ns | 5033ns | 5534ns | -47.18% |
| ema_form_q32 | 7084ns | 7590ns | 6208ns | 7194ns | 7629ns | -25.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 7651ns | 6541ns | 8085ns | base | 0.535 |
| ema_form_intshift | 3237ns | 2805ns | 3635ns | -57.69% | 1.266 |
| ema_form_q32 | 5325ns | 4669ns | 5729ns | -30.40% | 0.769 |

## Performance model

- Peak throughput: **1.460 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.512 | 35.1% |
| ema_form_intshift | 1.195 | 81.8% |
| ema_form_q32 | 0.718 | 49.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 9455ns | 9455ns | base |
| ema_form_intshift | 4995ns | 4995ns | -47.18% |
| ema_form_q32 | 7084ns | 7084ns | -25.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 8000ns | base | --- | [7990, 8000] | --- | --- | --- | --- |
| ema_form_intshift | 3428ns | -4562.5ns (-57.0%) | [-4572, -4562]ns | [3428, 3428] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 5706ns | -2282.9ns (-28.5%) | [-2283, -2282]ns | [5706, 5707] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 8000ns | -57.2% | -28.5% |
| 1 | 2 | 8000ns | -57.2% | -28.5% |
| 1 | 3 | 6545ns | -47.6% | -28.7% |
| 1 | 4 | 6546ns | -57.2% | -12.7% |
| 1 | 5 | 8000ns | -64.9% | -28.5% |
| 1 | 6 | 8000ns | -57.2% | -28.6% |
| 1 | 7 | 8001ns | -64.9% | -41.6% |
| 1 | 8 | 8000ns | -57.1% | -41.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | 0.292 | moderate+ |
| ema_form_intshift | -0.142 | ok |
| ema_form_q32 | 0.060 | ok |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 22.6ns | 7650.6ns | 0.3% |  |
| ema_form_intshift | 22.0ns | 3236.6ns | 0.7% |  |
| ema_form_q32 | 22.0ns | 5325.0ns | 0.4% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 6541.3-8084.6 ns)
   6541.3 |########
   6618.5 |
   6695.6 |
   6772.8 |
   6850.0 |
   6927.1 |
   7004.3 |
   7081.4 |
   7158.6 |
   7235.8 |
   7312.9 |
   7390.1 |
   7467.3 |
   7544.4 |
   7621.6 |
   7698.8 |
   7775.9 |
   7853.1 |
   7930.2 |########################################
   8007.4 |
  (16 below, 3 above range)

ema_form_intshift (n=160, range 2804.5-3634.8 ns)
   2804.5 |################
   2846.0 |
   2887.5 |
   2929.1 |
   2970.6 |
   3012.1 |
   3053.6 |
   3095.1 |
   3136.6 |
   3178.1 |
   3219.7 |
   3261.2 |
   3302.7 |
   3344.2 |
   3385.7 |
   3427.2 |########################################
   3468.7 |
   3510.3 |
   3551.8 |
   3593.3 |
  (19 below, 1 above range)

ema_form_q32 (n=160, range 4669.0-5729.2 ns)
   4669.0 |####################
   4722.0 |
   4775.0 |
   4828.0 |
   4881.0 |
   4934.1 |
   4987.1 |
   5040.1 |
   5093.1 |
   5146.1 |
   5199.1 |
   5252.1 |
   5305.1 |
   5358.2 |
   5411.2 |
   5464.2 |
   5517.2 |
   5570.2 |
   5623.2 |
   5676.2 |########################################
  (9 below, 2 above range)

```

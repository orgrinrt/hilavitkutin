# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (31.96 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 11.22 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 67% faster than the next best (ema_form_q32)

ema_form_intshift (11.22 us) leads ema_form_q32 (18.68 us) by 67%, a clear separation rather than a photo finish. CV 10.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 61% (significant)

ema_form_intshift is -19.65 us (61%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_form_float is an outlier: 2.8x slower than the field

ema_form_float (31.96 us) is 2.8x the fastest (11.22 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ema_form_intshift is fastest but the noisiest (CV 10.8%)

ema_form_intshift wins on median (11.22 us) yet has the highest variance (CV 10.8%), while ema_form_float is the steadiest (CV 7.9%, 31.96 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: ema_form_intshift** at 11216.4 ns median (-64.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.85x (fastest 11216.4 ns, slowest 31959.9 ns)

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
| ema_form_float | 32328ns | 33847ns | 27689ns | 33342ns | 33925ns | base |
| ema_form_intshift | 13823ns | 12758ns | 12746ns | 13584ns | 15615ns | -57.24% |
| ema_form_q32 | 21359ns | 20222ns | 20217ns | 20607ns | 24756ns | -33.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 30520ns | 26144ns | 32018ns | base | 0.537 |
| ema_form_intshift | 12147ns | 11204ns | 13720ns | -60.20% | 1.349 |
| ema_form_q32 | 19722ns | 18671ns | 22857ns | -35.38% | 0.831 |

## Performance model

- Peak throughput: **1.462 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.513 | 35.1% |
| ema_form_intshift | 1.461 | 99.9% |
| ema_form_q32 | 0.877 | 60.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 32328ns | 32328ns | base |
| ema_form_intshift | 13823ns | 13823ns | -57.24% |
| ema_form_q32 | 21359ns | 21359ns | -33.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 31960ns | base | --- | [31959, 31960] | --- | --- | --- | --- |
| ema_form_intshift | 11216ns | -19651.3ns (-61.5%) | [-20743, -18264]ns | [11209, 11218] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 18677ns | -13280.4ns (-41.6%) | [-13283, -13270]ns | [18677, 18678] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 31960ns | -57.1% | -41.6% |
| 1 | 2 | 31961ns | -64.9% | -28.6% |
| 1 | 3 | 31966ns | -57.1% | -41.6% |
| 1 | 4 | 31961ns | -64.9% | -41.6% |
| 1 | 5 | 26148ns | -47.6% | -28.6% |
| 1 | 6 | 26149ns | -57.1% | -12.7% |
| 1 | 7 | 31962ns | -64.9% | -41.6% |
| 1 | 8 | 31959ns | -64.9% | -41.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | 0.292 | moderate+ |
| ema_form_intshift | -0.475 | moderate- |
| ema_form_q32 | -0.375 | moderate- |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 23.0ns | 30520.2ns | 0.1% |  |
| ema_form_intshift | 21.3ns | 12146.7ns | 0.2% |  |
| ema_form_q32 | 20.8ns | 19722.4ns | 0.1% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 26144.0-32017.5 ns)
  26144.0 |#########
  26437.7 |
  26731.3 |
  27025.0 |
  27318.7 |
  27612.4 |
  27906.0 |
  28199.7 |
  28493.4 |
  28787.1 |
  29080.8 |
  29374.4 |
  29668.1 |
  29961.8 |
  30255.5 |
  30549.1 |
  30842.8 |
  31136.5 |
  31430.2 |
  31723.8 |########################################
  (14 below, 7 above range)

ema_form_intshift (n=160, range 11204.2-13720.3 ns)
  11204.2 |########################################
  11330.0 |
  11455.8 |
  11581.6 |
  11707.4 |
  11833.2 |
  11959.0 |
  12084.8 |
  12210.6 |
  12336.4 |
  12462.2 |
  12588.0 |
  12713.8 |
  12839.6 |
  12965.5 |
  13091.3 |
  13217.1 |
  13342.9 |
  13468.7 |
  13594.5 |#########################
  (11 below, 3 above range)

ema_form_q32 (n=160, range 18670.7-22856.6 ns)
  18670.7 |########################################
  18880.0 |
  19089.3 |
  19298.6 |
  19507.9 |
  19717.2 |
  19926.5 |
  20135.8 |
  20345.1 |
  20554.4 |
  20763.7 |
  20973.0 |
  21182.3 |
  21391.6 |
  21600.9 |
  21810.2 |
  22019.5 |
  22228.8 |
  22438.0 |
  22647.3 |###############
  (21 below, 2 above range)

```

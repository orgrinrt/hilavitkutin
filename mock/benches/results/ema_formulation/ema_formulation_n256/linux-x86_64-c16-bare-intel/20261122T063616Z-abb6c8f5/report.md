# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (509 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 182 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 100% faster than the next best (ema_form_q32)

ema_form_intshift (182 ns) leads ema_form_q32 (363 ns) by 100%, a clear separation rather than a photo finish. CV 95.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 46% (significant)

ema_form_intshift is -235 ns (46%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_form_float is an outlier: 2.8x slower than the field

ema_form_float (509 ns) is 2.8x the fastest (182 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ema_form_intshift is fastest but the noisiest (CV 95.3%)

ema_form_intshift wins on median (182 ns) yet has the highest variance (CV 95.3%), while ema_form_float is the steadiest (CV 66.0%, 509 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### ema_form_intshift is inconsistent: worst-20% is 3.3x its best-20%

ema_form_intshift's best 20% of batches run at 181 ns but its worst 20% at 596 ns (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ema_form_intshift** at 181.6 ns median (-64.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.80x (fastest 181.6 ns, slowest 508.6 ns)

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
| ema_form_float | 2707ns | 2384ns | 1949ns | 2217ns | 4933ns | base |
| ema_form_intshift | 2604ns | 1729ns | 1715ns | 1868ns | 5698ns | -3.82% |
| ema_form_q32 | 2748ns | 2237ns | 1831ns | 2095ns | 5627ns | +1.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 575ns | 416ns | 1041ns | base | 0.445 |
| ema_form_intshift | 274ns | 181ns | 596ns | -52.37% | 0.935 |
| ema_form_q32 | 444ns | 297ns | 904ns | -22.76% | 0.577 |

## Performance model

- Peak throughput: **1.411 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.503 | 35.7% |
| ema_form_intshift | 1.409 | 99.9% |
| ema_form_q32 | 0.705 | 50.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 2707ns | 2707ns | base |
| ema_form_intshift | 2604ns | 2604ns | -3.82% |
| ema_form_q32 | 2748ns | 2748ns | +1.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 509ns | base | --- | [416, 509] | --- | --- | --- | --- |
| ema_form_intshift | 182ns | -234.8ns (-46.2%) | [-287, -235]ns | [182, 201] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 363ns | -145.2ns (-28.5%) | [-145, -119]ns | [363, 363] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 416ns | -56.4% | -28.6% |
| 1 | 2 | 509ns | -64.3% | -41.6% |
| 1 | 3 | 509ns | -56.4% | -41.6% |
| 1 | 4 | 416ns | -56.4% | -12.7% |
| 1 | 5 | 509ns | -64.3% | -28.6% |
| 1 | 6 | 416ns | -56.4% | -28.6% |
| 1 | 7 | 416ns | -46.7% | -12.7% |
| 1 | 8 | 509ns | -64.3% | -28.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | -0.375 | moderate- |
| ema_form_intshift | -0.375 | moderate- |
| ema_form_q32 | 0.125 | ok |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 27.0ns | 574.8ns | 4.7% |  |
| ema_form_intshift | 29.4ns | 273.8ns | 10.7% | HIGH |
| ema_form_q32 | 29.4ns | 443.9ns | 6.6% | HIGH |

## Distribution (algo ns)

```
ema_form_float (n=160, range 416.2-1041.1 ns)
    416.2 |####################################
    447.4 |
    478.7 |########################################
    509.9 |###
    541.2 |
    572.4 |
    603.7 |
    634.9 |
    666.2 |
    697.4 |
    728.7 |
    759.9 |
    791.2 |
    822.4 |
    853.7 |
    884.9 |
    916.2 |
    947.4 |
    978.6 |
   1009.9 |
  (10 below, 15 above range)

ema_form_intshift (n=160, range 181.5-595.9 ns)
    181.5 |########################################
    202.2 |##############
    222.9 |#
    243.7 |
    264.4 |
    285.1 |
    305.8 |
    326.5 |
    347.3 |
    368.0 |
    388.7 |
    409.4 |
    430.1 |
    450.9 |
    471.6 |
    492.3 |
    513.0 |
    533.7 |
    554.5 |#
    575.2 |
  (3 below, 20 above range)

ema_form_q32 (n=160, range 297.2-903.5 ns)
    297.2 |#####################################
    327.5 |
    357.8 |########################################
    388.1 |
    418.5 |
    448.8 |
    479.1 |
    509.4 |
    539.7 |
    570.0 |
    600.4 |
    630.7 |
    661.0 |
    691.3 |
    721.6 |
    751.9 |
    782.3 |
    812.6 |
    842.9 |#
    873.2 |
  (4 below, 19 above range)

```

## Diagnostics

- **ema_form_float**: CV=58.4% (high variance, measurements may be unstable)
- **ema_form_intshift**: CV=63.2% (high variance, measurements may be unstable)
- **ema_form_intshift**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **ema_form_intshift**: bridge=10.8% of algo (FFI overhead may distort results)
- **ema_form_q32**: CV=61.7% (high variance, measurements may be unstable)
- **ema_form_q32**: worst_20/best_20 = 3.0x (possible bimodal distribution)

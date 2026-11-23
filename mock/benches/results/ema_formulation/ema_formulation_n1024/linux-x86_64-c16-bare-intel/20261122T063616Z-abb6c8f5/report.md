# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (2.00 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 863 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 66% faster than the next best (ema_form_q32)

ema_form_intshift (863 ns) leads ema_form_q32 (1.43 us) by 66%, a clear separation rather than a photo finish. CV 64.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 47% (significant)

ema_form_intshift is -939 ns (47%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_form_float is an outlier: 2.3x slower than the field

ema_form_float (2.00 us) is 2.3x the fastest (863 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ema_form_intshift is fastest but the noisiest (CV 64.3%)

ema_form_intshift wins on median (863 ns) yet has the highest variance (CV 64.3%), while ema_form_float is the steadiest (CV 18.0%, 2.00 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### ema_form_intshift is inconsistent: worst-20% is 2.9x its best-20%

ema_form_intshift's best 20% of batches run at 706 ns but its worst 20% at 2.04 us (2.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ema_form_intshift** at 863.0 ns median (-57.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.32x (fastest 863.0 ns, slowest 2004.8 ns)

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
| ema_form_float | 3759ns | 3883ns | 3176ns | 3705ns | 4507ns | base |
| ema_form_intshift | 3341ns | 2740ns | 2240ns | 2649ns | 6516ns | -11.13% |
| ema_form_q32 | 3390ns | 3307ns | 2706ns | 3077ns | 5010ns | -9.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 1917ns | 1640ns | 2211ns | base | 0.534 |
| ema_form_intshift | 1049ns | 706ns | 2037ns | -45.30% | 0.976 |
| ema_form_q32 | 1463ns | 1172ns | 2150ns | -23.72% | 0.700 |

## Performance model

- Peak throughput: **1.450 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.511 | 35.2% |
| ema_form_intshift | 1.186 | 81.8% |
| ema_form_q32 | 0.715 | 49.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 3759ns | 3759ns | base |
| ema_form_intshift | 3341ns | 3341ns | -11.13% |
| ema_form_q32 | 3390ns | 3390ns | -9.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 2005ns | base | --- | [2005, 2005] | --- | --- | --- | --- |
| ema_form_intshift | 863ns | -939.0ns (-46.8%) | [-1142, -934]ns | [863, 863] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 1432ns | -572.7ns (-28.6%) | [-573, -560]ns | [1189, 1432] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 1640ns | -56.9% | -28.6% |
| 1 | 2 | 2005ns | -64.8% | -28.6% |
| 1 | 3 | 2005ns | -57.0% | -28.6% |
| 1 | 4 | 2005ns | -57.0% | -41.6% |
| 1 | 5 | 2005ns | -64.8% | -28.6% |
| 1 | 6 | 1640ns | -47.4% | -12.7% |
| 1 | 7 | 2005ns | -8.2% | -41.6% |
| 1 | 8 | 1640ns | -47.4% | -28.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | -0.275 | moderate- |
| ema_form_intshift | -0.022 | ok |
| ema_form_q32 | -0.125 | ok |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 22.8ns | 1917.5ns | 1.2% |  |
| ema_form_intshift | 29.0ns | 1048.9ns | 2.8% |  |
| ema_form_q32 | 24.6ns | 1462.7ns | 1.7% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 1640.2-2211.2 ns)
   1640.2 |####################
   1668.8 |
   1697.3 |
   1725.9 |
   1754.4 |
   1783.0 |
   1811.5 |
   1840.1 |
   1868.6 |
   1897.2 |
   1925.7 |
   1954.3 |
   1982.8 |########################################
   2011.4 |##########
   2039.9 |#
   2068.5 |
   2097.0 |
   2125.6 |
   2154.1 |
   2182.7 |
  (18 below, 4 above range)

ema_form_intshift (n=160, range 706.1-2036.5 ns)
    706.1 |##################
    772.6 |
    839.2 |########################################
    905.7 |
    972.2 |#
   1038.7 |
   1105.2 |
   1171.8 |
   1238.3 |#
   1304.8 |
   1371.3 |
   1437.8 |
   1504.4 |#
   1570.9 |
   1637.4 |
   1703.9 |
   1770.4 |
   1837.0 |
   1903.5 |
   1970.0 |
  (20 below, 20 above range)

ema_form_q32 (n=160, range 1171.5-2150.3 ns)
   1171.5 |###########################
   1220.5 |
   1269.4 |
   1318.4 |
   1367.3 |
   1416.2 |########################################
   1465.2 |
   1514.1 |
   1563.0 |
   1612.0 |
   1660.9 |
   1709.9 |
   1758.8 |
   1807.7 |
   1856.7 |
   1905.6 |
   1954.6 |
   2003.5 |
   2052.4 |
   2101.4 |
  (17 below, 11 above range)

```

## Diagnostics

- **ema_form_intshift**: CV=52.9% (high variance, measurements may be unstable)
- **ema_form_q32**: CV=38.5% (high variance, measurements may be unstable)

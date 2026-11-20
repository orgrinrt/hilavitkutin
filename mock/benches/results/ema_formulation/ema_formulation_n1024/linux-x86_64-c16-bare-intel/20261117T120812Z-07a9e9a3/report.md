# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (2.00 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 863 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 66% faster than the next best (ema_form_q32)

ema_form_intshift (863 ns) leads ema_form_q32 (1.43 us) by 66%, a clear separation rather than a photo finish. CV 54.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 57% (significant)

ema_form_intshift is -1.14 us (57%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_form_float is an outlier: 2.3x slower than the field

ema_form_float (2.00 us) is 2.3x the fastest (863 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ema_form_intshift is fastest but the noisiest (CV 54.6%)

ema_form_intshift wins on median (863 ns) yet has the highest variance (CV 54.6%), while ema_form_float is the steadiest (CV 15.6%, 2.00 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### ema_form_intshift is inconsistent: worst-20% is 2.3x its best-20%

ema_form_intshift's best 20% of batches run at 706 ns but its worst 20% at 1.62 us (2.3x) - a bimodal or bursty profile the median hides.

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
| Counter frequency | 2112771874 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ema_form_float | 3799ns | 3889ns | 3186ns | 3850ns | 4258ns | base |
| ema_form_intshift | 3132ns | 2749ns | 2249ns | 2734ns | 5206ns | -17.56% |
| ema_form_q32 | 3458ns | 3318ns | 2716ns | 3295ns | 4691ns | -8.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 1956ns | 1641ns | 2186ns | base | 0.524 |
| ema_form_intshift | 980ns | 706ns | 1620ns | -49.87% | 1.044 |
| ema_form_q32 | 1489ns | 1173ns | 2007ns | -23.87% | 0.688 |

## Performance model

- Peak throughput: **1.450 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.511 | 35.2% |
| ema_form_intshift | 1.187 | 81.8% |
| ema_form_q32 | 0.715 | 49.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 3799ns | 3799ns | base |
| ema_form_intshift | 3132ns | 3132ns | -17.56% |
| ema_form_q32 | 3458ns | 3458ns | -8.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 2005ns | base | --- | [2005, 2005] | --- | --- | --- | --- |
| ema_form_intshift | 863ns | -1141.7ns (-56.9%) | [-1142, -1142]ns | [863, 863] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 1432ns | -572.9ns (-28.6%) | [-573, -573]ns | [1432, 1432] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 2005ns | -57.0% | -41.6% |
| 1 | 2 | 2005ns | -64.8% | -28.6% |
| 1 | 3 | 1640ns | -47.4% | -12.7% |
| 1 | 4 | 2005ns | -64.8% | -28.6% |
| 1 | 5 | 2005ns | -57.0% | -28.6% |
| 1 | 6 | 1640ns | -47.4% | -12.7% |
| 1 | 7 | 2005ns | -57.0% | -41.6% |
| 1 | 8 | 2005ns | -56.9% | -28.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | -0.375 | moderate- |
| ema_form_intshift | -0.375 | moderate- |
| ema_form_q32 | -0.209 | moderate- |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 23.3ns | 1955.9ns | 1.2% |  |
| ema_form_intshift | 27.3ns | 980.4ns | 2.8% |  |
| ema_form_q32 | 24.8ns | 1489.0ns | 1.7% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 1640.9-2186.1 ns)
   1640.9 |##
   1668.1 |
   1695.4 |
   1722.7 |
   1749.9 |
   1777.2 |
   1804.5 |
   1831.7 |
   1859.0 |
   1886.2 |
   1913.5 |
   1940.8 |
   1968.0 |
   1995.3 |########################################
   2022.6 |
   2049.8 |
   2077.1 |
   2104.4 |
   2131.6 |
   2158.9 |
  (30 below, 3 above range)

ema_form_intshift (n=160, range 706.2-1620.4 ns)
    706.2 |#######
    751.9 |
    797.6 |
    843.3 |########################################
    889.0 |
    934.7 |
    980.4 |
   1026.1 |
   1071.9 |
   1117.6 |
   1163.3 |
   1209.0 |
   1254.7 |
   1300.4 |
   1346.1 |
   1391.9 |
   1437.6 |
   1483.3 |
   1529.0 |
   1574.7 |
  (16 below, 14 above range)

ema_form_q32 (n=160, range 1172.6-2007.2 ns)
   1172.6 |##
   1214.3 |
   1256.0 |
   1297.8 |
   1339.5 |
   1381.2 |
   1423.0 |########################################
   1464.7 |
   1506.4 |
   1548.2 |
   1589.9 |
   1631.6 |
   1673.4 |
   1715.1 |
   1756.8 |
   1798.6 |
   1840.3 |
   1882.0 |
   1923.7 |
   1965.5 |
  (29 below, 9 above range)

```

## Diagnostics

- **ema_form_intshift**: CV=48.1% (high variance, measurements may be unstable)
- **ema_form_q32**: CV=34.4% (high variance, measurements may be unstable)
